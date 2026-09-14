// Key-to-command mapping: Vim + Emacs bindings converging on one action set.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Pure function of crossterm key values. Unmapped keys yield `None` — the
//! caller ignores them. No terminal, no I/O, fully table-tested.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// UI-level intent decoded from a key press.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyAction {
    NextChat,
    PrevChat,
    Open,
    Palette,
    Compose,
    Quit,
}

/// Map a key press to its action. Returns `None` for unbound keys.
pub fn map_key(key: KeyEvent) -> Option<KeyAction> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('q'), m) if m.contains(KeyModifiers::CONTROL) => Some(KeyAction::Quit),
        (KeyCode::Char('k'), m) if m.contains(KeyModifiers::CONTROL) => Some(KeyAction::Palette),
        (KeyCode::Char('n'), m) if m.contains(KeyModifiers::CONTROL) => Some(KeyAction::NextChat),
        (KeyCode::Char('p'), m) if m.contains(KeyModifiers::CONTROL) => Some(KeyAction::PrevChat),
        (KeyCode::Char('j'), KeyModifiers::NONE) => Some(KeyAction::NextChat),
        (KeyCode::Char('k'), KeyModifiers::NONE) => Some(KeyAction::PrevChat),
        (KeyCode::Char('i'), KeyModifiers::NONE) => Some(KeyAction::Compose),
        (KeyCode::Char('/'), _) => Some(KeyAction::Palette),
        (KeyCode::Enter, _) => Some(KeyAction::Open),
        _ => None,
    }
}
