// Message composer buffer and command-palette filter.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Pure input models: no terminal, no network. The event loop feeds keystrokes
//! here; `submit()` hands validated text to the `MessageSent` command path.
//! [`handle_submit`] runs the full optimistic cycle against a provider.

use crate::app::{AppState, Command, Event};
use crate::domain::ChatMessage;
use crate::error::AppError;
use crate::provider::ChatProvider;
use chrono::Utc;

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

/// Run the optimistic submit cycle: blank text rejects without I/O; otherwise
/// append a sanitized temp message, send via the provider, and confirm by
/// swapping the temp id for the real one. Provider failure rolls the temp
/// back so state never holds phantom messages.
pub async fn handle_submit<P: ChatProvider>(
    state: &mut AppState,
    provider: &P,
    chat_id: &str,
    text: &str,
) -> Result<Vec<Event>, AppError> {
    if text.trim().is_empty() {
        return Ok(vec![Event::Rejected { reason: "empty body" }]);
    }
    let now = Utc::now();
    let temp_id = format!("temp-{}", uuid::Uuid::new_v4());
    let temp = ChatMessage {
        id: temp_id.clone(),
        chat_id: chat_id.into(),
        created: now,
        modified: now,
        sender: "me".into(),
        body: crate::sanitize::sanitize(text),
        reply_to_id: None,
        reactions: vec![],
        mentions: vec![],
        is_read: true,
        segments: vec![],
    };
    let mut events = state.apply(Command::MessageSent { message: temp });
    match provider.send_message(chat_id, text).await {
        Ok(real) => {
            events.extend(state.apply(Command::MessageConfirmed { temp_id, message: real }));
            Ok(events)
        }
        Err(e) => {
            state.messages.retain(|m| m.id != temp_id);
            Err(e)
        }
    }
}
