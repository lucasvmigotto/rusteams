// Deterministic in-memory fake for all unit/integration tests.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use super::traits::{ChatProvider, PresenceProvider};
use crate::domain::{Chat, ChatMessage};
use crate::error::AppError;
use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Mutex;

/// Fixture-backed fake: deterministic, offline, no network.
pub struct MockTeamsProvider {
    chats: Vec<Chat>,
    messages: Mutex<HashMap<String, Vec<ChatMessage>>>,
    fail_next: Mutex<bool>,
}

impl MockTeamsProvider {
    pub fn new() -> Self {
        Self {
            chats: vec![Chat {
                id: "chat-1".into(),
                topic: Some("Engineering".into()),
                last_message_preview: Some("hello".into()),
                last_message_at: Some(Utc::now()),
                unread: false,
            }],
            messages: Mutex::new(HashMap::new()),
            fail_next: Mutex::new(false),
        }
    }

    /// Make the next call fail with a 429-style throttling error.
    pub fn fail_next_with_throttle(&self) {
        *self.fail_next.lock().unwrap() = true;
    }

    fn take_failure(&self) -> Option<AppError> {
        let mut f = self.fail_next.lock().unwrap();
        if *f {
            *f = false;
            return Some(AppError::Network("HTTP 429 throttled".into()));
        }
        None
    }
}

impl Default for MockTeamsProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ChatProvider for MockTeamsProvider {
    async fn list_chats(&self) -> Result<Vec<Chat>, AppError> {
        if let Some(e) = self.take_failure() {
            return Err(e);
        }
        Ok(self.chats.clone())
    }

    async fn list_messages(&self, chat_id: &str) -> Result<Vec<ChatMessage>, AppError> {
        if let Some(e) = self.take_failure() {
            return Err(e);
        }
        Ok(self.messages.lock().unwrap().get(chat_id).cloned().unwrap_or_default())
    }

    async fn send_message(&self, chat_id: &str, body: &str) -> Result<ChatMessage, AppError> {
        if let Some(e) = self.take_failure() {
            return Err(e);
        }
        let now = Utc::now();
        let m = ChatMessage {
            id: format!("m-{}", now.timestamp_nanos_opt().unwrap_or(0)),
            chat_id: chat_id.into(),
            created: now,
            modified: now,
            sender: "me".into(),
            body: crate::sanitize::sanitize(body),
            reply_to_id: None,
        };
        self.messages.lock().unwrap().entry(chat_id.into()).or_default().push(m.clone());
        Ok(m)
    }
}

#[async_trait]
impl PresenceProvider for MockTeamsProvider {
    async fn my_presence(&self) -> Result<String, AppError> {
        Ok("Available".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_roundtrip_is_deterministic_and_offline() {
        let p = MockTeamsProvider::new();
        assert_eq!(p.list_chats().await.unwrap().len(), 1);
        let sent = p.send_message("chat-1", "hi <b>there</b>").await.unwrap();
        assert!(sent.body.contains("hi"));
        let msgs = p.list_messages("chat-1").await.unwrap();
        assert_eq!(msgs.len(), 1);
    }

    #[tokio::test]
    async fn mock_can_simulate_throttling() {
        let p = MockTeamsProvider::new();
        p.fail_next_with_throttle();
        let err = p.list_chats().await.unwrap_err();
        assert!(err.to_string().contains("429"));
    }
}
