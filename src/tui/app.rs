// Read-only TUI panes: sidebar, conversation, status bar.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Rendering takes plain [`ReadView`] data assembled by the caller (app state
//! and sanitized domain text). It never touches providers or the network, which
//! keeps every pane testable on [`ratatui::backend::TestBackend`].

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

/// One sidebar row: `(chat_id, title, unread)`.
pub type ChatRow = (String, String, bool);

/// One conversation row, already sanitized for display.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageRow {
    pub sender: String,
    pub time: String,
    pub body: String,
}

/// Everything the read panes need. All strings are display-ready.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadView {
    pub title: String,
    pub connection: String,
    pub chats: Vec<ChatRow>,
    pub selected_chat: Option<String>,
    pub messages: Vec<MessageRow>,
    pub status: String,
}

/// Render header (title + connection), sidebar, conversation, and status bar.
/// Degrades gracefully: empty chats/messages render hints, never panic.
pub fn render_read_view(f: &mut Frame, view: &ReadView) {
    let chrome = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1), Constraint::Length(1)])
        .split(f.area());

    let header = Paragraph::new(format!("{}  ● {}", view.title, view.connection));
    f.render_widget(header, chrome[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chrome[1]);

    let chat_items: Vec<ListItem> = if view.chats.is_empty() {
        vec![ListItem::new("(no chats)")]
    } else {
        view.chats
            .iter()
            .map(|(id, title, unread)| {
                let marker = if Some(id) == view.selected_chat.as_ref() { "● " } else { "  " };
                let flag = if *unread { " *" } else { "" };
                ListItem::new(format!("{marker}{title}{flag}"))
            })
            .collect()
    };
    f.render_widget(List::new(chat_items).block(Block::default().borders(Borders::RIGHT)), body[0]);

    let text = if view.messages.is_empty() {
        "(no messages)".to_string()
    } else {
        view.messages
            .iter()
            .map(|m| format!("{} [{}]\n{}", m.sender, m.time, m.body))
            .collect::<Vec<_>>()
            .join("\n\n")
    };
    f.render_widget(Paragraph::new(text).block(Block::default()), body[1]);

    f.render_widget(Paragraph::new(view.status.clone()), chrome[2]);
}
