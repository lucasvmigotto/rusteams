// Event loop: scripted harness (tested) and live terminal feed (manual).
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Key streams become [`KeyAction`]s via [`map_key`], folded into `AppState`
//! with [`fold_actions`]. The scripted harness keeps the whole flow
//! deterministic; [`run_live`] wires the same fold to crossterm events and the
//! renderer. Live behavior needs a real TTY — verified manually, never in CI.

use super::app::{apply_action, build_read_view, render_read_view};
use super::keys::{KeyAction, map_key};
use super::terminal::acquire_terminal;
use crate::app::AppState;
use crate::error::AppError;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
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

/// Live loop: poll crossterm (100ms tick so shutdown/paint stay responsive),
/// fold key presses, re-render. Returns on quit or shutdown. Requires a TTY;
/// callers acquire nothing — acquisition and restore live here so every exit
/// path leaves the terminal usable.
pub async fn run_live(
    state: &mut AppState,
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
                    if let Some(action) = map_key(key) {
                        if fold_actions(state, &[action]) {
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
        let view = build_read_view(state);
        terminal
            .draw(|f| render_read_view(f, &view))
            .map_err(|e| AppError::Terminal(format!("render: {e}")))?;
    }
    super::terminal::restore_terminal();
    Ok(())
}
