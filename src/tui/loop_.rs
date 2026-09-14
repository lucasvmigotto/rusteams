// Event loop: scripted harness (tested) and live terminal feed (manual).
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Key streams become [`KeyAction`]s via [`map_key`], folded into `AppState`
//! with [`fold_actions`]. The scripted harness keeps the whole flow
//! deterministic; [`run_live`] wires the same fold to crossterm events and the
//! renderer. Live behavior needs a real TTY — verified manually, never in CI.

use super::app::{apply_action, build_read_view, render_read_view};
use super::compose::{Composer, handle_submit};
use super::keys::{KeyAction, map_key};
use super::terminal::acquire_terminal;
use crate::app::AppState;
use crate::error::AppError;
use crate::provider::ChatProvider;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;

/// Fold a scripted key stream into actions. Stops after `Quit` — keys past it
/// are never processed. `_selected` names the chat context for future use
/// (open/navigation scoping lands with the live loop).
pub fn run_scripted(_selected: &str, keys: &[KeyEvent]) -> Vec<KeyAction> {
    let mut out = Vec::new();
    for key in keys {
        match map_key(*key) {
            Some(KeyAction::Quit) => {
                out.push(KeyAction::Quit);
                break;
            }
            Some(action) => out.push(action),
            None => {}
        }
    }
    out
}

/// Fold actions into state. Returns true once `Quit` is seen; actions past it
/// are never applied.
pub fn fold_actions(state: &mut AppState, actions: &[KeyAction]) -> bool {
    for action in actions {
        if apply_action(state, action.clone()) {
            return true;
        }
    }
    false
}

/// Live input services: provider, composer buffer, compose mode, and the last
/// send error (surfaced by future status rendering; never swallowed silently
/// into the void — the accessor keeps it observable and tested).
pub struct LiveServices<P> {
    provider: P,
    composer: Composer,
    composing: bool,
    last_error: Option<String>,
}

impl<P> LiveServices<P> {
    pub fn new(provider: P) -> Self {
        Self { provider, composer: Composer::new(), composing: false, last_error: None }
    }

    pub fn composing(&self) -> bool {
        self.composing
    }

    pub fn draft(&self) -> &str {
        self.composer.text()
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }
}

impl<P: ChatProvider> LiveServices<P> {
    /// Process one key press. Returns true when the UI must quit.
    /// Compose mode captures printable keys into the draft; `Enter` submits
    /// against the selected chat (no selection = stay composing); `Esc`
    /// abandons the draft; `Ctrl+Q` quits from any mode.
    pub async fn step(&mut self, state: &mut AppState, key: KeyEvent) -> bool {
        if matches!(map_key(key), Some(KeyAction::Quit)) {
            return true;
        }
        if key.code == KeyCode::Esc {
            self.composing = false;
            self.composer = Composer::new();
            return false;
        }
        if self.composing {
            match key.code {
                KeyCode::Enter => {
                    let Some(chat_id) = state.selected_chat.clone() else {
                        return false;
                    };
                    let Some(text) = self.composer.submit() else {
                        return false;
                    };
                    match handle_submit(state, &self.provider, &chat_id, &text).await {
                        Ok(_) => {
                            self.composing = false;
                            self.last_error = None;
                        }
                        Err(e) => {
                            self.last_error = Some(e.to_string());
                        }
                    }
                    return false;
                }
                KeyCode::Char(c)
                    if key.modifiers == KeyModifiers::NONE
                        || key.modifiers == KeyModifiers::SHIFT =>
                {
                    self.composer.insert(&c.to_string());
                    return false;
                }
                KeyCode::Backspace => {
                    self.composer.backspace(1);
                    return false;
                }
                _ => return false,
            }
        }
        match map_key(key) {
            Some(KeyAction::Compose) => {
                self.composing = true;
                self.composer = Composer::new();
                self.last_error = None;
                false
            }
            Some(action) => fold_actions(state, &[action]),
            None => false,
        }
    }
}

/// Live loop: poll crossterm (100ms tick so shutdown/paint stay responsive),
/// step input services, re-render. Returns on quit or shutdown. Requires a
/// TTY; callers acquire nothing — acquisition and restore live here so every
/// exit path leaves the terminal usable.
pub async fn run_live<P: ChatProvider>(
    state: &mut AppState,
    services: &mut LiveServices<P>,
    shutdown: &crate::app::Shutdown,
) -> Result<(), AppError> {
    let mut terminal = acquire_terminal()?;
    super::terminal::install_panic_hook();
    loop {
        if shutdown.is_triggered() {
            break;
        }
        if event::poll(Duration::from_millis(100))
            .map_err(|e| AppError::Terminal(format!("event poll: {e}")))?
        {
            match event::read().map_err(|e| AppError::Terminal(format!("event read: {e}")))? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    if services.step(state, key).await {
                        break;
                    }
                }
                _ => {}
            }
        }
        let view = build_read_view(state, services.last_error());
        terminal
            .draw(|f| render_read_view(f, &view))
            .map_err(|e| AppError::Terminal(format!("render: {e}")))?;
    }
    super::terminal::restore_terminal();
    Ok(())
}
