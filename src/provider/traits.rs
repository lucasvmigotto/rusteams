// Provider boundary: the application core depends only on these traits.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::domain::{Chat, ChatMessage};
use crate::error::AppError;
use async_trait::async_trait;

/// Minimal MVP provider surface. Extended only with demonstrated need.
#[async_trait]
pub trait ChatProvider: Send + Sync {
    async fn list_chats(&self) -> Result<Vec<Chat>, AppError>;
    async fn list_messages(&self, chat_id: &str) -> Result<Vec<ChatMessage>, AppError>;
    async fn send_message(&self, chat_id: &str, body: &str) -> Result<ChatMessage, AppError>;
}

/// Presence surface (MVP: read own + others).
#[async_trait]
pub trait PresenceProvider: Send + Sync {
    async fn my_presence(&self) -> Result<String, AppError>;
}
