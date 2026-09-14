// Minimal HTML body renderer: Graph message HTML to rich segments.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Hand-rolled scanner, no new dependencies. Handles links, code blocks,
//! lists/paragraph breaks, and Teams `<at>` mentions; everything else degrades
//! to literal text. Callers sanitize segment text before display.

use crate::domain::RichSegment;

/// Render Graph message HTML into segments. Never panics; malformed markup
/// falls back to plain text of whatever survives.
pub fn render_html_body(html: &str) -> Vec<RichSegment> {
    let mut scanner = Scanner { rest: html, out: Vec::new(), text: String::new() };
    scanner.run();
    scanner.finish()
}

/// Convert `<at id="N">Name</at>` segments to `@Name` (shared with the legacy
/// plain path; other markup passes through for the segment scanner).
pub fn render_at_tags(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(start) = rest.find("<at ") {
        out.push_str(&rest[..start]);
        let after_open = &rest[start..];
        match after_open.find('>').map(|i| (i, after_open.find("</at>"))) {
            Some((tag_end, Some(close))) if close > tag_end => {
                out.push('@');
                out.push_str(&after_open[tag_end + 1..close]);
                rest = &after_open[close + "</at>".len()..];
            }
            _ => {
                out.push_str(after_open);
                break;
            }
        }
    }
    out.push_str(rest);
    out
}

struct Scanner<'a> {
    rest: &'a str,
    out: Vec<RichSegment>,
    text: String,
}

impl<'a> Scanner<'a> {
    fn run(&mut self) {
        while !self.rest.is_empty() {
            if let Some(seg) = self.try_link() {
                self.push(seg);
            } else if let Some(lines) = self.try_code_block() {
                self.push(RichSegment::CodeBlock { lines });
            } else if let Some(advance) = self.try_line_break() {
                self.text.push('\n');
                self.rest = advance;
            } else if let Some(tag) = self.try_tag() {
                // Unknown tag: emit literally, keep scanning after `>`.
                self.text.push_str(tag);
            } else {
                // Plain text until the next `<` (or end); lone `<` stays literal.
                match self.rest.find('<') {
                    Some(0) => {
                        self.text.push('<');
                        self.rest = &self.rest[1..];
                    }
                    Some(i) => {
                        self.text.push_str(&self.rest[..i]);
                        self.rest = &self.rest[i..];
                    }
                    None => {
                        self.text.push_str(self.rest);
                        self.rest = "";
                    }
                }
            }
        }
    }

    fn push(&mut self, seg: RichSegment) {
        if !self.text.is_empty() {
            let text = std::mem::take(&mut self.text);
            self.out.push(RichSegment::Text(text));
        }
        self.out.push(seg);
    }

    fn finish(mut self) -> Vec<RichSegment> {
        if !self.text.is_empty() {
            self.out.push(RichSegment::Text(std::mem::take(&mut self.text)));
        }
        self.out
    }

    /// `<a href="URL">text</a>` — both quote styles; unclosed → None.
    fn try_link(&mut self) -> Option<RichSegment> {
        let s = self.rest.strip_prefix("<a ")?;
        let href_start = s.find("href=")? + "href=".len();
        let quote = s[href_start..].chars().next()?;
        if quote != '"' && quote != '\'' {
            return None;
        }
        let url_start = href_start + 1;
        let url_end = s[url_start..].find(quote)? + url_start;
        let url = s[url_start..url_end].to_string();
        let tag_end = s.find('>')?;
        let after = &s[tag_end + 1..];
        let close = after.find("</a>")?;
        let text = after[..close].to_string();
        self.rest = &after[close + "</a>".len()..];
        Some(RichSegment::Link { text, url })
    }

    /// `<pre><code>…</code></pre>` (or bare `<pre>`); unterminated → None.
    fn try_code_block(&mut self) -> Option<Vec<String>> {
        let mut s = self.rest.strip_prefix("<pre>")?;
        s = s.strip_prefix("<code>").unwrap_or(s);
        let end = s.find("</code></pre>").map(|i| (i, "</code></pre>".len())).or_else(|| {
            s.find("</pre>")
                .map(|i| (i, "</pre>".len()))
                .or_else(|| s.find("</code>").map(|i| (i, "</code>".len())))
        })?;
        let lines = s[..end.0].lines().map(str::to_string).collect();
        self.rest = &s[end.0 + end.1..];
        Some(lines)
    }

    /// Block boundaries that force a newline; returns the remaining input.
    fn try_line_break(&mut self) -> Option<&'a str> {
        for tag in ["<br>", "<br/>", "<li>", "<p>", "</p>", "<div>", "</div>", "<ul>", "</ul>"] {
            if let Some(rest) = self.rest.strip_prefix(tag) {
                return Some(rest);
            }
        }
        None
    }

    /// Any other `<…>` tag; returns the tag text and advances past `>`.
    /// Unclosed `<` returns None (caller emits it literally).
    fn try_tag(&mut self) -> Option<&'a str> {
        let s = self.rest.strip_prefix('<')?;
        let end = s.find('>')?;
        let tag_len = end + 2; // '<' + body + '>'
        let tag = &self.rest[..tag_len];
        self.rest = &self.rest[tag_len..];
        Some(tag)
    }
}
