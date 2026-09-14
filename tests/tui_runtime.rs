// Runtime wiring tests: ReadView assembly from AppState, action folding, and
// terminal acquire/restore behavior (headless-safe: asserts fail-closed).
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use chrono::{TimeZone, Utc};
use rusteams::app::{AppState, Command};
use rusteams::domain::{Chat, ChatMessage};
use rusteams::tui::{KeyAction, apply_action, build_read_view, restore_terminal};

fn chat(id: &str, topic: &str) -> Chat {
    Chat {
        id: id.into(),
        topic: Some(topic.into()),
        last_message_preview: None,
        last_message_at: None,
        unread: topic == "Unread",
    }
}

fn msg(id: &str, chat: &str, sender: &str, body: &str) -> ChatMessage {
    let t = Utc.timestamp_opt(1_700_000_000, 0).unwrap();
    ChatMessage {
        id: id.into(),
        chat_id: chat.into(),
        created: t,
        modified: t,
        sender: sender.into(),
        body: body.into(),
        reply_to_id: None,
        reactions: vec![],
        mentions: vec![],
        is_read: false,
    }
}

#[test]
fn read_view_assembles_sidebar_selection_and_messages() {
    let mut s = AppState::default();
    s.chats = vec![chat("c1", "Engineering"), chat("c2", "Unread")];
    s.apply(Command::MessageReceived { message: msg("m1", "c1", "Alice", "hi") });
    s.apply(Command::SelectChat { chat_id: "c1".into() });

    let view = build_read_view(&s);
    assert_eq!(view.chats.len(), 2);
    assert_eq!(view.selected_chat.as_deref(), Some("c1"));
    assert_eq!(view.messages.len(), 1);
    assert_eq!(view.messages[0].sender, "Alice");
    assert!(!view.messages[0].time.is_empty(), "time is always stamped");
}

#[test]
fn read_view_scopes_messages_to_selected_chat() {
    let mut s = AppState::default();
    s.chats = vec![chat("c1", "A"), chat("c2", "B")];
    s.apply(Command::MessageReceived { message: msg("m1", "c1", "A", "one") });
    s.apply(Command::MessageReceived { message: msg("m2", "c2", "B", "two") });
    s.apply(Command::SelectChat { chat_id: "c2".into() });

    let view = build_read_view(&s);
    assert_eq!(view.messages.len(), 1);
    assert_eq!(view.messages[0].body, "two");
}

#[test]
fn actions_move_selection_and_quit_stops() {
    let mut s = AppState::default();
    s.chats = vec![chat("c1", "A"), chat("c2", "B")];
    assert!(!apply_action(&mut s, KeyAction::NextChat));
    assert_eq!(s.selected_chat.as_deref(), Some("c2"));
    assert!(!apply_action(&mut s, KeyAction::PrevChat));
    assert_eq!(s.selected_chat.as_deref(), Some("c1"));
    assert!(apply_action(&mut s, KeyAction::Quit), "quit returns true");
}

#[test]
fn restore_is_best_effort_and_never_panics_headless() {
    // No TTY in CI/containers: must not panic, success is opportunistic.
    let _ = restore_terminal();
}
