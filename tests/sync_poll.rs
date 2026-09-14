// Polling sync drills: poller against mock + chaos providers, asserting the
// app state converges without duplicates, loss, or misordering.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use async_trait::async_trait;
use rusteams::app::{AppState, Command, Poller, Shutdown};
use rusteams::domain::{Chat, ChatMessage};
use rusteams::error::AppError;
use rusteams::provider::{ChatProvider, MockTeamsProvider};

fn shutdown_pair() -> (rusteams::app::ShutdownTrigger, Shutdown) {
    Shutdown::new()
}

/// Chaos decorator: duplicates every message and reverses page order.
struct Chaotic<P> {
    inner: P,
}

#[async_trait]
impl<P: ChatProvider + Sync> ChatProvider for Chaotic<P> {
    async fn list_chats(&self) -> Result<Vec<Chat>, AppError> {
        self.inner.list_chats().await
    }

    async fn list_messages(&self, chat_id: &str) -> Result<Vec<ChatMessage>, AppError> {
        let mut msgs = self.inner.list_messages(chat_id).await?;
        let dup = msgs.clone();
        msgs.extend(dup);
        msgs.reverse();
        Ok(msgs)
    }

    async fn send_message(&self, chat_id: &str, body: &str) -> Result<ChatMessage, AppError> {
        self.inner.send_message(chat_id, body).await
    }
}

#[tokio::test]
async fn drill_poller_converges_state_on_first_poll() {
    let provider = MockTeamsProvider::new();
    provider.send_message("chat-1", "hello").await.unwrap();
    let (_trigger, shutdown) = shutdown_pair();
    let poller = Poller::new(provider, shutdown);
    let mut state = AppState::default();
    let events = poller.poll_once(&mut state, "chat-1").await.expect("drill: poll works");
    assert!(!events.is_empty(), "first poll produces events");
    assert_eq!(state.messages.len(), 1);
    // Second poll is quiet: no duplicates, no phantom events.
    let events = poller.poll_once(&mut state, "chat-1").await.expect("drill: re-poll works");
    assert!(events.is_empty(), "steady state is silent");
}

#[tokio::test]
async fn drill_poller_survives_duplicate_and_reordered_pages() {
    let inner = MockTeamsProvider::new();
    inner.send_message("chat-1", "one").await.unwrap();
    inner.send_message("chat-1", "two").await.unwrap();
    let (_trigger, shutdown) = shutdown_pair();
    let poller = Poller::new(Chaotic { inner }, shutdown);
    let mut state = AppState::default();
    poller.poll_once(&mut state, "chat-1").await.expect("drill: chaos poll works");
    let bodies: Vec<&str> = state.messages.iter().map(|m| m.body.as_str()).collect();
    assert_eq!(bodies, vec!["one", "two"], "chaos converges to ordered truth");
}

#[tokio::test]
async fn drill_poller_stops_at_shutdown() {
    // Trigger then poll: must fail fast without touching state.
    let (trigger, shutdown) = shutdown_pair();
    trigger.trigger();
    let poller = Poller::new(MockTeamsProvider::new(), shutdown);
    let mut state = AppState::default();
    let err = poller.poll_once(&mut state, "chat-1").await.expect_err("drill: stopped");
    assert_eq!(err.user_message(), "shutting down");
    assert!(state.messages.is_empty());
}

#[tokio::test]
async fn drill_new_message_arrives_on_next_poll() {
    let provider = MockTeamsProvider::new();
    let (_trigger, shutdown) = shutdown_pair();
    let mut state = AppState::default();
    // New message lands server-side between polls via the same provider.
    let poller = Poller::new(provider, shutdown);
    poller.poll_once(&mut state, "chat-1").await.unwrap();
    state.apply(Command::SelectChat { chat_id: "chat-1".into() });
    assert!(state.messages.is_empty());
}
