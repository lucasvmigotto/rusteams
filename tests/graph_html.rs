// HTML body rendering fixtures: links, code, lists, nesting, emoji, and
// malformed markup — all must degrade to readable text, never panic.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use rusteams::infra::graph::{RichSegment, render_html_body};

#[test]
fn link_renders_text_with_url() {
    let segs = render_html_body(r#"see <a href="https://example.com/x">docs</a> now"#);
    assert_eq!(
        segs,
        vec![
            RichSegment::Text("see ".into()),
            RichSegment::Link { text: "docs".into(), url: "https://example.com/x".into() },
            RichSegment::Text(" now".into()),
        ]
    );
}

#[test]
fn code_block_preserves_lines() {
    let segs = render_html_body("<pre><code>fn main() {\n    println!();\n}</code></pre>");
    assert_eq!(
        segs,
        vec![RichSegment::CodeBlock {
            lines: vec!["fn main() {".into(), "    println!();".into(), "}".into()]
        }]
    );
}

#[test]
fn lists_break_lines_and_nesting_degrades() {
    let segs = render_html_body("<div><ul><li>one</li><li>two</li></ul></div>");
    let text: String = segs
        .iter()
        .map(|s| match s {
            RichSegment::Text(t) => t.clone(),
            RichSegment::Link { text, .. } => text.clone(),
            RichSegment::CodeBlock { lines } => lines.join("\n"),
        })
        .collect::<Vec<_>>()
        .join("");
    assert!(text.contains("one"), "first item kept: {text:?}");
    assert!(text.contains("two"), "second item kept: {text:?}");
}

#[test]
fn emoji_and_plain_text_pass_through() {
    let segs = render_html_body("hi 💘 done");
    assert_eq!(segs, vec![RichSegment::Text("hi 💘 done".into())]);
}

#[test]
fn malformed_markup_falls_back_to_text() {
    let segs = render_html_body("a <b broken <at>oops");
    let text: String = segs
        .iter()
        .map(|s| match s {
            RichSegment::Text(t) => t.clone(),
            RichSegment::Link { text, .. } => text.clone(),
            RichSegment::CodeBlock { lines } => lines.join("\n"),
        })
        .collect::<Vec<_>>()
        .join("");
    assert!(text.contains('a'), "leading text kept: {text:?}");
    assert!(text.contains("oops"), "trailing text kept: {text:?}");
}
