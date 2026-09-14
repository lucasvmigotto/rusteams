// Key-to-command mapping and scripted event-loop tests. Pure crossterm key
// values in, app commands out — no terminal required.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rusteams::tui::{KeyAction, map_key, run_scripted};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

fn ctrl(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::CONTROL)
}

#[test]
fn vim_and_emacs_navigation_converge() {
    assert_eq!(map_key(key(KeyCode::Char('j'))), Some(KeyAction::NextChat));
    assert_eq!(map_key(key(KeyCode::Char('k'))), Some(KeyAction::PrevChat));
    assert_eq!(map_key(ctrl(KeyCode::Char('n'))), Some(KeyAction::NextChat));
    assert_eq!(map_key(ctrl(KeyCode::Char('p'))), Some(KeyAction::PrevChat));
}

#[test]
fn command_keys_map() {
    assert_eq!(map_key(key(KeyCode::Enter)), Some(KeyAction::Open));
    assert_eq!(map_key(key(KeyCode::Char('/'))), Some(KeyAction::Palette));
    assert_eq!(map_key(ctrl(KeyCode::Char('k'))), Some(KeyAction::Palette));
    assert_eq!(map_key(ctrl(KeyCode::Char('q'))), Some(KeyAction::Quit));
}

#[test]
fn unmapped_keys_are_ignored_never_panic() {
    assert_eq!(map_key(key(KeyCode::F(12))), None);
    assert_eq!(map_key(key(KeyCode::Esc)), None);
    assert_eq!(map_key(ctrl(KeyCode::Char('z'))), None);
}

#[test]
fn scripted_stream_drives_commands_until_quit() {
    let keys = vec![
        key(KeyCode::Char('j')),
        key(KeyCode::Char('j')),
        key(KeyCode::Enter),
        ctrl(KeyCode::Char('q')),
        key(KeyCode::Char('k')), // past quit: never processed
    ];
    let cmds = run_scripted("chat-1", &keys);
    assert_eq!(
        cmds,
        vec![KeyAction::NextChat, KeyAction::NextChat, KeyAction::Open, KeyAction::Quit,]
    );
}
