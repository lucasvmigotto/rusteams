// Key-to-command mapping and scripted event-loop tests. Pure crossterm key
// values in, app commands out — no terminal required.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rusteams::tui::{KeyAction, fold_actions, map_key, run_scripted};

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

#[test]
fn fold_applies_actions_to_state_until_quit() {
    use rusteams::app::AppState;
    use rusteams::domain::Chat;

    let mut state = AppState {
        chats: vec![
            Chat {
                id: "c1".into(),
                topic: Some("A".into()),
                last_message_preview: None,
                last_message_at: None,
                unread: false,
            },
            Chat {
                id: "c2".into(),
                topic: Some("B".into()),
                last_message_preview: None,
                last_message_at: None,
                unread: false,
            },
        ],
        ..Default::default()
    };
    let quit = fold_actions(
        &mut state,
        &[KeyAction::NextChat, KeyAction::Open, KeyAction::Palette, KeyAction::Quit],
    );
    assert!(quit, "quit terminates the fold");
    assert_eq!(state.selected_chat.as_deref(), Some("c1"));
}

#[test]
fn fold_without_quit_returns_false() {
    use rusteams::app::AppState;

    let mut state = AppState::default();
    assert!(!fold_actions(&mut state, &[KeyAction::NextChat, KeyAction::Open]));
}

#[test]
fn insert_key_enters_compose_mode() {
    assert_eq!(map_key(key(KeyCode::Char('i'))), Some(KeyAction::Compose));
}

#[test]
fn live_step_types_submits_and_quits() {
    use rusteams::app::AppState;
    use rusteams::provider::MockTeamsProvider;
    use rusteams::tui::LiveServices;

    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    rt.block_on(async {
        let mut state = AppState::default();
        let mut services = LiveServices::new(MockTeamsProvider::new());
        assert!(!services.step(&mut state, key(KeyCode::Char('i'))).await);
        assert!(services.composing());
        assert!(!services.step(&mut state, key(KeyCode::Char('h'))).await);
        assert!(!services.step(&mut state, key(KeyCode::Char('i'))).await);
        assert_eq!(services.draft(), "hi");
        // Submitting with no chat selected is a no-op, stays composing.
        assert!(!services.step(&mut state, key(KeyCode::Enter)).await);
        assert!(services.composing());
        // Escape abandons the draft.
        assert!(!services.step(&mut state, key(KeyCode::Esc)).await);
        assert!(!services.composing());
        assert_eq!(services.draft(), "");
        // Quit still terminates from any mode.
        assert!(services.step(&mut state, ctrl(KeyCode::Char('q'))).await);
    });
}
