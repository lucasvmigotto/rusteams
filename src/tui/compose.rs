// Message composer buffer and command-palette filter.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Pure input models: no terminal, no network. The event loop feeds keystrokes
//! here; `submit()` hands validated text to the `MessageSent` command path.

/// Editable single-submission message buffer.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Composer {
    buffer: String,
}

impl Composer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Append typed text.
    pub fn insert(&mut self, s: &str) {
        self.buffer.push_str(s);
    }

    /// Delete the last `n` characters (Unicode-safe). Short buffers clear.
    pub fn backspace(&mut self, n: usize) {
        let len = self.buffer.chars().count();
        let keep = len.saturating_sub(n);
        self.buffer = self.buffer.chars().take(keep).collect();
    }

    pub fn text(&self) -> &str {
        &self.buffer
    }

    /// Take the buffer if it holds non-blank text; blank input is rejected and
    /// the buffer is preserved so the user loses nothing.
    pub fn submit(&mut self) -> Option<String> {
        if self.buffer.trim().is_empty() {
            return None;
        }
        Some(std::mem::take(&mut self.buffer))
    }
}

/// Case-insensitive substring filter over command names.
#[derive(Debug, Clone)]
pub struct Palette {
    commands: Vec<String>,
}

impl Palette {
    pub fn new(commands: Vec<String>) -> Self {
        Self { commands }
    }

    pub fn filter(&self, query: &str) -> Vec<String> {
        let q = query.to_lowercase();
        self.commands.iter().filter(|c| c.to_lowercase().contains(&q)).cloned().collect()
    }
}
