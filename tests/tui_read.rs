// Read-pane rendering tests over ratatui's TestBackend: deterministic,
// no terminal required. Rendering takes plain view data — never providers.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use ratatui::{Terminal, backend::TestBackend};
use rusteams::tui::{MessageRow, ReadView, render_read_view};

fn sample_view() -> ReadView {
    ReadView {
        title: "rusteams".into(),
        connection: "Connected".into(),
        chats: vec![("chat-1".into(), "Engineering".into(), false)],
        selected_chat: Some("chat-1".into()),
        messages: vec![MessageRow {
            sender: "Alice".into(),
            time: "10:32".into(),
            body: "Hello, Teams!".into(),
        }],
        status: "Ctrl+Q Quit".into(),
        notice: None,
    }
}

#[test]
fn read_view_shows_chats_messages_and_status() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| render_read_view(f, &sample_view())).unwrap();
    let content: String =
        terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();
    assert!(content.contains("Engineering"), "sidebar chat missing");
    assert!(content.contains("Hello, Teams!"), "message body missing");
    assert!(content.contains("Connected"), "connection state missing");
    assert!(content.contains("Ctrl+Q Quit"), "status bar missing");
}

fn empty_view() -> ReadView {
    ReadView {
        title: "rusteams".into(),
        connection: "Reconnecting".into(),
        chats: vec![],
        selected_chat: None,
        messages: vec![],
        status: "offline".into(),
        notice: None,
    }
}

fn screen_of(view: &ReadView, width: u16, height: u16) -> String {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| render_read_view(f, view)).unwrap();
    terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect()
}

#[test]
fn empty_states_render_hints_never_panic() {
    let content = screen_of(&empty_view(), 80, 24);
    assert!(content.contains("(no chats)"), "empty sidebar hint missing");
    assert!(content.contains("(no messages)"), "empty conversation hint missing");
    assert!(content.contains("Reconnecting"), "degraded state missing");
}

#[test]
fn narrow_terminal_renders_without_panic() {
    let content = screen_of(&sample_view(), 20, 8);
    assert!(content.contains("rusteams"), "title missing on narrow screen");
}
