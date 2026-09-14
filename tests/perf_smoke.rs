// Performance smoke tests: 10k-message conversation render and ordering with
// generous bounds. These guard against pathological blowups (quadratic joins,
// unbounded allocations), not for benchmarking claims — real profiling belongs
// to Stage 6 follow-ups on release builds.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use chrono::{TimeZone, Utc};
use ratatui::{Terminal, backend::TestBackend};
use rusteams::domain::{ChatMessage, order_messages};
use rusteams::tui::{MessageRow, ReadView, render_read_view};
use std::time::{Duration, Instant};

fn big_view(n: usize) -> ReadView {
    ReadView {
        title: "rusteams".into(),
        connection: "Connected".into(),
        chats: vec![("chat-1".into(), "Big".into(), false)],
        selected_chat: Some("chat-1".into()),
        messages: (0..n)
            .map(|i| MessageRow {
                sender: "Alice".into(),
                time: "10:32".into(),
                body: format!("message number {i} with some body text to wrap around"),
            })
            .collect(),
        status: "ok".into(),
        notice: None,
    }
}

#[test]
fn render_10k_messages_completes_within_budget() {
    let view = big_view(10_000);
    let backend = TestBackend::new(100, 40);
    let mut terminal = Terminal::new(backend).unwrap();
    let start = Instant::now();
    terminal.draw(|f| render_read_view(f, &view)).unwrap();
    let elapsed = start.elapsed();
    eprintln!("render 10k messages (debug): {elapsed:?}");
    assert!(elapsed < Duration::from_secs(20), "render blew the smoke budget: {elapsed:?}");
}

#[test]
fn order_10k_messages_completes_within_budget() {
    let mut msgs: Vec<ChatMessage> = (0..10_000)
        .map(|i| {
            let t = Utc.timestamp_opt(i as i64, 0).unwrap();
            ChatMessage {
                id: format!("m{i:05}"),
                chat_id: "c1".into(),
                created: t,
                modified: t,
                sender: "a".into(),
                body: "x".into(),
                reply_to_id: None,
                reactions: vec![],
                mentions: vec![],
                is_read: false,
                segments: vec![],
            }
        })
        .collect();
    msgs.reverse();
    let start = Instant::now();
    order_messages(&mut msgs);
    let elapsed = start.elapsed();
    eprintln!("order 10k messages (debug): {elapsed:?}");
    assert!(elapsed < Duration::from_secs(20), "ordering blew the smoke budget: {elapsed:?}");
    assert_eq!(msgs.first().unwrap().id, "m00000");
    assert_eq!(msgs.last().unwrap().id, "m09999");
}
