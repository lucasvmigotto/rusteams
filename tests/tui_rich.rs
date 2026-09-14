// Rich message rendering tests: segments become styled ratatui lines, with
// plain-text fallback when no segments exist.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use chrono::{TimeZone, Utc};
use ratatui::style::{Modifier, Style};
use rusteams::domain::{ChatMessage, RichSegment};
use rusteams::tui::message_lines;

fn msg_with_segments() -> ChatMessage {
    let t = Utc.timestamp_opt(1_700_000_000, 0).unwrap();
    ChatMessage {
        id: "m1".into(),
        chat_id: "c1".into(),
        created: t,
        modified: t,
        sender: "Alice".into(),
        body: "see docs (https://example.com/x)".into(),
        reply_to_id: None,
        reactions: vec![],
        mentions: vec![],
        is_read: false,
        segments: vec![
            RichSegment::Text("see ".into()),
            RichSegment::Link { text: "docs".into(), url: "https://example.com/x".into() },
            RichSegment::CodeBlock { lines: vec!["fn main() {}".into()] },
        ],
    }
}

#[test]
fn links_render_underlined_with_url_visible() {
    let lines = message_lines(&msg_with_segments());
    let link_line = lines.iter().find(|l| l.width() > 0 && format!("{l:?}").contains("docs")).expect("link line");
    assert!(link_line.spans.iter().any(|s| s.style.add_modifier.contains(Modifier::UNDERLINED)));
    let all: String = lines.iter().flat_map(|l| l.spans.iter().map(|s| s.content.as_ref())).collect();
    assert!(all.contains("https://example.com/x"), "url visible in plain form");
}

#[test]
fn code_lines_render_bold_and_indented() {
    let lines = message_lines(&msg_with_segments());
    let code_line =
        lines.iter().find(|l| l.spans.iter().any(|s| s.content.contains("fn main"))).expect("code line");
    assert!(code_line.spans.iter().any(|s| s.style.add_modifier.contains(Modifier::BOLD)));
}

#[test]
fn empty_segments_fall_back_to_plain_body() {
    let mut msg = msg_with_segments();
    msg.segments.clear();
    msg.body = "plain fallback".into();
    let lines = message_lines(&msg);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].spans[0].content, "plain fallback");
    assert_eq!(lines[0].spans[0].style, Style::default());
}
