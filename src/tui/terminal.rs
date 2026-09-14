// Terminal lifecycle: acquire on startup, restore on every exit path.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! `acquire_terminal` fails closed (`AppError::Terminal`) when no TTY exists.
//! `restore_terminal` is best-effort and infallible by design: it must run on
//! normal exit, shutdown, and panics without ever panicking itself. Wire it
//! into `DropGuard` and the panic hook at the binary root (Stage B runtime).

use crate::error::AppError;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::Stdout;

/// Take over the terminal: raw mode + alternate screen. Fails closed headless.
pub fn acquire_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, AppError> {
    enable_raw_mode().map_err(|e| AppError::Terminal(format!("raw mode: {e}")))?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)
        .map_err(|e| AppError::Terminal(format!("alternate screen: {e}")))?;
    Terminal::new(CrosstermBackend::new(stdout))
        .map_err(|e| AppError::Terminal(format!("terminal init: {e}")))
}

/// Give the terminal back. Ignores all errors — a failing restore must never
/// trap the user in a worse state than a best-effort attempt.
pub fn restore_terminal() {
    let _ = disable_raw_mode();
    let _ = execute!(std::io::stdout(), LeaveAlternateScreen);
}

/// Panic hook that restores the terminal, then delegates to the default hook
/// so crash reports still surface. Install once at binary startup.
pub fn install_panic_hook() {
    let default = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore_terminal();
        default(info);
    }));
}
