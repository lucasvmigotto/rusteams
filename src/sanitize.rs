// Terminal-output sanitization — mandatory security boundary.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! All remote (untrusted) text must pass through [`sanitize`] before rendering.
//!
//! Guarantees:
//! - strips C0 controls (except `\n`, `\t`) and DEL,
//! - strips ANSI CSI (`ESC [` … final byte), OSC (`ESC ]` … `BEL`/`ESC \`),
//!   and other ESC sequences, including lone `ESC`,
//! - replaces Unicode non-characters / surrogates defensively,
//! - never panics on malformed input; output is always safe to hand to ratatui.

/// Sanitize untrusted remote text for terminal presentation.
pub fn sanitize(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        match b {
            // ESC introduces CSI / OSC / other escape sequences.
            0x1B => {
                if i + 1 >= bytes.len() {
                    i += 1; // lone ESC at end: drop
                    continue;
                }
                let next = bytes[i + 1];
                match next {
                    // CSI: ESC [ ... final byte @..~
                    b'[' => {
                        i += 2;
                        while i < bytes.len() {
                            let c = bytes[i];
                            i += 1;
                            if (0x40..=0x7E).contains(&c) {
                                break;
                            }
                        }
                    }
                    // OSC: ESC ] ... terminated by BEL or ESC \
                    b']' => {
                        i += 2;
                        while i < bytes.len() {
                            if bytes[i] == 0x07 {
                                i += 1;
                                break;
                            }
                            if bytes[i] == 0x1B && i + 1 < bytes.len() && bytes[i + 1] == b'\\' {
                                i += 2;
                                break;
                            }
                            i += 1;
                        }
                    }
                    // Other ESC-led sequences: Fe escapes (ESC + ASCII letter,
                    // e.g. ESC c RIS, ESC M RI) are dropped with their final
                    // byte; charset sequences consume an extra byte; anything
                    // else drops ESC alone and keeps the follower (stray ESC).
                    _ => {
                        if matches!(next, b'(' | b')' | b'#' | b'%') {
                            i += 3;
                        } else if next.is_ascii_alphabetic() {
                            i += 2;
                        } else {
                            i += 1;
                        }
                    }
                }
            }
            // C0 controls: keep \n and \t only.
            0x00..=0x08 | 0x0B | 0x0C | 0x0E..=0x1F => {
                i += 1;
            }
            0x7F => {
                i += 1; // DEL
            }
            _ => {
                // Decode one UTF-8 scalar safely; input is &str so slicing is safe.
                let ch = input[i..].chars().next().expect("valid UTF-8");
                // Drop Unicode non-characters / surrogates defensively
                // (surrogates cannot appear in valid &str, but be explicit).
                let v = ch as u32;
                let is_noncharacter = (0xFDD0..=0xFDEF).contains(&v) || (v & 0xFFFE == 0xFFFE);
                if !is_noncharacter {
                    out.push(ch);
                }
                i += ch.len_utf8();
            }
        }
    }
    out
}

/// Sanitize and additionally truncate to `max_chars` characters (Unicode-safe).
pub fn sanitize_with_limit(input: &str, max_chars: usize) -> String {
    let clean = sanitize(input);
    if clean.chars().count() <= max_chars {
        return clean;
    }
    clean.chars().take(max_chars).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_passes_through() {
        assert_eq!(sanitize("Hello, Teams!"), "Hello, Teams!");
    }

    #[test]
    fn strips_ansi_color_sequences() {
        assert_eq!(sanitize("\x1b[31mred\x1b[0m"), "red");
    }

    #[test]
    fn strips_csi_cursor_positioning() {
        assert_eq!(sanitize("a\x1b[2;5Hb"), "ab");
    }

    #[test]
    fn strips_osc_hyperlink_and_title() {
        assert_eq!(sanitize("\x1b]8;;http://evil.example\x07click\x1b]0;owned\x07"), "click");
    }

    #[test]
    fn strips_lone_escape_and_other_esc_sequences() {
        // Fe escape (ESC + letter) is dropped with its final byte.
        assert_eq!(sanitize("a\x1bb"), "a");
        // Charset designation sequence is dropped wholesale.
        assert_eq!(sanitize("a\x1b(0b"), "ab");
        // Stray ESC before a non-letter keeps the follower.
        assert_eq!(sanitize("a\x1b b"), "a b");
    }

    #[test]
    fn strips_c0_controls_but_keeps_newline_and_tab() {
        assert_eq!(sanitize("a\x00b\x07c\nd\te"), "abc\nd\te");
    }

    #[test]
    fn strips_del_character() {
        assert_eq!(sanitize("a\x7fb"), "ab");
    }

    #[test]
    fn strips_esc_terminated_osc() {
        assert_eq!(sanitize("x\x1b]8;;http://e\x1b\\y"), "xy");
    }

    #[test]
    fn handles_emoji_and_unicode() {
        assert_eq!(sanitize("hi 💘 ünïcodé"), "hi 💘 ünïcodé");
    }

    #[test]
    fn truncate_respects_char_boundary() {
        assert_eq!(sanitize_with_limit("abcdef", 3), "abc");
        assert_eq!(sanitize_with_limit("hi 💘bye", 4), "hi 💘");
    }

    #[test]
    fn empty_input_stays_empty() {
        assert_eq!(sanitize(""), "");
    }
}
