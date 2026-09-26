// Render faithful TUI prints for the documentation website.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! `cargo run --example tui_snapshot` regenerates
//! `docs/site/src/content/tui-shots.json` from the real renderer on
//! `TestBackend` — no terminal, no network, no mocks beyond plain view
//! data. `--check` fails when the committed file is stale (CI gate), so
//! the site's screenshots can never drift from what the TUI renders.

use ratatui::{Terminal, backend::TestBackend, style::Color};
use rusteams::tui::{MessageRow, ReadView, render_read_view};
use std::collections::HashMap;

const WIDTH: u16 = 80;
const HEIGHT: u16 = 24;
const OUT: &str = "docs/site/src/content/tui-shots.json";

fn populated_view() -> ReadView {
    ReadView {
        title: "rusteams".into(),
        connection: "Connected".into(),
        chats: vec![
            ("chat-1".into(), "Engineering".into(), false),
            ("chat-2".into(), "Family".into(), true),
        ],
        selected_chat: Some("chat-1".into()),
        messages: vec![
            MessageRow {
                sender: "Alice".into(),
                time: "10:32".into(),
                body: "Hello, Teams!".into(),
            },
            MessageRow {
                sender: "Bob".into(),
                time: "10:33".into(),
                body: "Standup moved to 11:00.".into(),
            },
        ],
        status: "j/k move · Enter open · / palette · Ctrl+Q quit · rusteams 0.2.0 · message sent"
            .into(),
        notice: Some("message sent".into()),
    }
}

fn color_name(c: &Color) -> String {
    match c {
        Color::Reset => "reset".into(),
        Color::Black => "black".into(),
        Color::Red => "red".into(),
        Color::Green => "green".into(),
        Color::Yellow => "yellow".into(),
        Color::Blue => "blue".into(),
        Color::Magenta => "magenta".into(),
        Color::Cyan => "cyan".into(),
        Color::Gray => "gray".into(),
        Color::DarkGray => "darkgray".into(),
        Color::LightRed => "lightred".into(),
        Color::LightGreen => "lightgreen".into(),
        Color::LightYellow => "lightyellow".into(),
        Color::LightBlue => "lightblue".into(),
        Color::LightMagenta => "lightmagenta".into(),
        Color::LightCyan => "lightcyan".into(),
        Color::White => "white".into(),
        Color::Rgb(r, g, b) => format!("rgb({r},{g},{b})"),
        Color::Indexed(i) => format!("indexed({i})"),
    }
}

fn render_shot(view: &ReadView) -> serde_json::Value {
    let backend = TestBackend::new(WIDTH, HEIGHT);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| render_read_view(f, view)).unwrap();
    let buffer = terminal.backend().buffer();
    let area = buffer.area;
    let mut rows = Vec::new();
    for y in 0..area.height {
        let mut runs: Vec<serde_json::Value> = Vec::new();
        let mut text = String::new();
        let mut fg = String::from("reset");
        let mut bg = String::from("reset");
        let flush = |text: &mut String, fg: &mut String, bg: &mut String| {
            if text.is_empty() {
                return None;
            }
            let run =
                serde_json::json!({"t": std::mem::take(text), "fg": fg.clone(), "bg": bg.clone()});
            *fg = String::from("reset");
            *bg = String::from("reset");
            Some(run)
        };
        for x in 0..area.width {
            let cell = &buffer[(x, y)];
            let (cf, cb) = (color_name(&cell.fg), color_name(&cell.bg));
            if cf != fg || cb != bg {
                if let Some(run) = flush(&mut text, &mut fg, &mut bg) {
                    runs.push(run);
                }
                fg = cf;
                bg = cb;
            }
            text.push_str(cell.symbol());
        }
        if let Some(run) = flush(&mut text, &mut fg, &mut bg) {
            runs.push(run);
        }
        // Trim trailing whitespace-only runs: structure without the noise.
        while runs.last().is_some_and(|r| {
            r["fg"] == "reset"
                && r["bg"] == "reset"
                && r["t"].as_str().is_some_and(|t| t.trim().is_empty())
        }) {
            runs.pop();
        }
        rows.push(serde_json::Value::Array(runs));
    }
    serde_json::json!({"width": area.width, "height": area.height, "rows": rows})
}

fn main() {
    let mut shots = HashMap::new();
    shots.insert("read", render_shot(&populated_view()));
    let doc = serde_json::json!({
        "_generated": "cargo run --example tui_snapshot — do not hand-edit",
        "shots": shots,
    });
    let text = serde_json::to_string_pretty(&doc).unwrap() + "\n";
    if std::env::args().any(|a| a == "--check") {
        let committed = std::fs::read_to_string(OUT).unwrap_or_default();
        if committed != text {
            eprintln!("stale {OUT}: regenerate with `cargo run --example tui_snapshot`");
            std::process::exit(1);
        }
        println!("{OUT} is fresh");
    } else {
        std::fs::write(OUT, text).unwrap();
        println!("wrote {OUT}");
    }
}
