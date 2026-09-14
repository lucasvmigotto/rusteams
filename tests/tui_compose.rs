// Composer and palette tests: buffer editing, submit rules, fuzzy filter.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use rusteams::tui::{Composer, Palette};

#[test]
fn composer_edits_and_submits_non_empty_input() {
    let mut c = Composer::new();
    c.insert("hello");
    c.insert(" world");
    assert_eq!(c.text(), "hello world");
    c.backspace(6);
    assert_eq!(c.text(), "hello");
    let submitted = c.submit().expect("non-empty submits");
    assert_eq!(submitted, "hello");
    assert_eq!(c.text(), "", "submit clears the buffer");
}

#[test]
fn composer_rejects_blank_submit() {
    let mut c = Composer::new();
    c.insert("   ");
    assert!(c.submit().is_none(), "blank input never sends");
    assert_eq!(c.text(), "   ", "buffer preserved on reject");
}

#[test]
fn palette_filters_commands_by_substring() {
    let p = Palette::new(vec!["login".into(), "logout".into(), "quit".into()]);
    assert_eq!(p.filter("log"), vec!["login", "logout"]);
    assert_eq!(p.filter("QUIT"), vec!["quit"], "filter is case-insensitive");
    assert!(p.filter("zzz").is_empty());
}
