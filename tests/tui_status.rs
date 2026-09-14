// Status error line and confirmation model tests.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use rusteams::app::AppState;
use rusteams::tui::{Confirm, build_read_view};

#[test]
fn status_shows_last_error_when_present() {
    let view = build_read_view(&AppState::default(), Some("send failed: throttled"));
    assert!(view.status.contains("send failed"), "error visible: {}", view.status);
    assert!(view.status.contains("rusteams"), "version still present");
}

#[test]
fn status_without_error_shows_hints_and_version() {
    let view = build_read_view(&AppState::default(), None);
    assert!(view.status.contains("j/k move"), "hints kept");
    assert!(!view.status.contains("send failed"));
}

#[test]
fn confirm_asks_then_resolves() {
    let mut c = Confirm::ask("Delete this message?");
    assert_eq!(c.prompt(), "Delete this message?");
    assert_eq!(c.answer(), None, "unresolved until answered");
    c.resolve(true);
    assert_eq!(c.answer(), Some(true));
}

#[test]
fn confirm_reject_is_explicit() {
    let mut c = Confirm::ask("Delete this message?");
    c.resolve(false);
    assert_eq!(c.answer(), Some(false));
}
