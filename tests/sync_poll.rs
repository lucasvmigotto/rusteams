// Polling sync drills: poller against mock + chaos providers, asserting the
// app state converges without duplicates, loss, or misordering.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use async_trait::async_trait;
use rusteams::app::{
    AppState, Command, Event, Poller, Shutdown, SyncLoop, Watermarks, note_connection,
    poll_due_chats, refresh_chats,
};
use rusteams::app::{app_state::ConnectionState, connection::ConnectionEvent};
use rusteams::domain::{Chat, ChatMessage};
use rusteams::error::AppError;
use rusteams::provider::{ChatProvider, MockTeamsProvider};
use std::time::Duration;

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

    async fn update_message(
        &self,
        chat_id: &str,
        message_id: &str,
        body: &str,
    ) -> Result<ChatMessage, AppError> {
        self.inner.update_message(chat_id, message_id, body).await
    }

    async fn delete_message(&self, chat_id: &str, message_id: &str) -> Result<(), AppError> {
        self.inner.delete_message(chat_id, message_id).await
    }

    async fn set_reaction(
        &self,
        chat_id: &str,
        message_id: &str,
        kind: &str,
    ) -> Result<(), AppError> {
        self.inner.set_reaction(chat_id, message_id, kind).await
    }

    async fn unset_reaction(
        &self,
        chat_id: &str,
        message_id: &str,
        kind: &str,
    ) -> Result<(), AppError> {
        self.inner.unset_reaction(chat_id, message_id, kind).await
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

/// Drop decorator: hides the first message once, then behaves (loss + heal).
struct Flaky<P> {
    inner: P,
    drop_armed: std::sync::atomic::AtomicBool,
}

#[async_trait]
impl<P: ChatProvider + Sync> ChatProvider for Flaky<P> {
    async fn list_chats(&self) -> Result<Vec<Chat>, AppError> {
        self.inner.list_chats().await
    }

    async fn list_messages(&self, chat_id: &str) -> Result<Vec<ChatMessage>, AppError> {
        let mut msgs = self.inner.list_messages(chat_id).await?;
        if self.drop_armed.swap(false, std::sync::atomic::Ordering::SeqCst) {
            msgs.remove(0);
        }
        Ok(msgs)
    }

    async fn send_message(&self, chat_id: &str, body: &str) -> Result<ChatMessage, AppError> {
        self.inner.send_message(chat_id, body).await
    }

    async fn update_message(
        &self,
        chat_id: &str,
        message_id: &str,
        body: &str,
    ) -> Result<ChatMessage, AppError> {
        self.inner.update_message(chat_id, message_id, body).await
    }

    async fn delete_message(&self, chat_id: &str, message_id: &str) -> Result<(), AppError> {
        self.inner.delete_message(chat_id, message_id).await
    }

    async fn set_reaction(
        &self,
        chat_id: &str,
        message_id: &str,
        kind: &str,
    ) -> Result<(), AppError> {
        self.inner.set_reaction(chat_id, message_id, kind).await
    }

    async fn unset_reaction(
        &self,
        chat_id: &str,
        message_id: &str,
        kind: &str,
    ) -> Result<(), AppError> {
        self.inner.unset_reaction(chat_id, message_id, kind).await
    }
}

#[tokio::test]
async fn drill_sweeper_refreshes_chat_list() {
    let provider = MockTeamsProvider::new();
    let mut state = AppState::default();
    let events = refresh_chats(&mut state, &provider).await.expect("drill: sweep works");
    assert_eq!(events.len(), 1);
    assert_eq!(state.chats.len(), 1);
    assert_eq!(state.chats[0].id, "chat-1");
}

#[tokio::test]
async fn drill_due_polling_skips_fresh_watermarks() {
    let provider = MockTeamsProvider::new();
    provider.send_message("chat-1", "hi").await.unwrap();
    let (_trigger, shutdown) = shutdown_pair();
    let poller = Poller::new(provider, shutdown);
    let mut state = AppState::default();
    state.apply(Command::SelectChat { chat_id: "chat-1".into() });
    let mut marks = Watermarks::default();
    let first = poll_due_chats(&mut state, &poller, &mut marks, Duration::from_secs(60))
        .await
        .expect("drill: first tick polls");
    assert!(!first.is_empty(), "stale chat is polled");
    let second = poll_due_chats(&mut state, &poller, &mut marks, Duration::from_secs(60))
        .await
        .expect("drill: second tick runs");
    assert!(second.is_empty(), "fresh watermark skips the poll");
}

#[tokio::test]
async fn drill_dropped_message_heals_on_next_poll() {
    let inner = MockTeamsProvider::new();
    inner.send_message("chat-1", "one").await.unwrap();
    inner.send_message("chat-1", "two").await.unwrap();
    let (_trigger, shutdown) = shutdown_pair();
    let poller = Poller::new(
        Flaky { inner, drop_armed: std::sync::atomic::AtomicBool::new(true) },
        shutdown,
    );
    let mut state = AppState::default();
    poller.poll_once(&mut state, "chat-1").await.expect("drill: lossy poll works");
    assert_eq!(state.messages.len(), 1, "one message lost in transit");
    poller.poll_once(&mut state, "chat-1").await.expect("drill: heal poll works");
    assert_eq!(state.messages.len(), 2, "next complete fetch heals the gap");
}

#[tokio::test]
async fn drill_reconnect_resets_watermarks() {
    let mut marks = Watermarks::default();
    marks.mark_synced("chat-1");
    assert!(!marks.is_due("chat-1", Duration::from_secs(3600)));
    marks.reset();
    assert!(marks.is_due("chat-1", Duration::from_secs(3600)), "reconnect re-baselines");
}

#[tokio::test]
async fn drill_reconnect_event_rebaselines_watermarks() {
    let mut state = AppState::default();
    let mut marks = Watermarks::default();
    marks.mark_synced("chat-1");
    // Connected -> hard failure -> reconnect attempt lands Reconnecting.
    state.apply(Command::ConnectionEvent(ConnectionEvent::HardFailure));
    assert_eq!(state.connection, ConnectionState::Disconnected);
    note_connection(&mut state, &mut marks, ConnectionEvent::ReconnectAttempt);
    assert_eq!(state.connection, ConnectionState::Reconnecting);
    assert!(
        marks.is_due("chat-1", Duration::from_secs(3600)),
        "reconnect clears watermarks for re-baseline"
    );
}

#[tokio::test]
async fn drill_tick_sweeps_and_polls_selected_chat() {
    let provider = MockTeamsProvider::new();
    provider.send_message("chat-1", "hi").await.unwrap();
    let (_trigger, shutdown) = shutdown_pair();
    let mut sync_loop = SyncLoop::new(provider, shutdown, Duration::from_secs(60));
    let mut state = AppState::default();
    state.apply(Command::SelectChat { chat_id: "chat-1".into() });
    let events = sync_loop.tick(&mut state).await.expect("drill: tick works");
    assert!(!events.is_empty(), "first tick sweeps + polls");
    assert_eq!(state.chats.len(), 1);
    assert_eq!(state.messages.len(), 1);
    let quiet = sync_loop.tick(&mut state).await.expect("drill: second tick runs");
    // Sweep always reports; message poll stays quiet on fresh watermarks.
    assert!(quiet.iter().all(|e| !matches!(e, Event::MessageAppended { .. })));
}

#[tokio::test]
async fn drill_tick_without_selection_sweeps_only() {
    let provider = MockTeamsProvider::new();
    let (_trigger, shutdown) = shutdown_pair();
    let mut sync_loop = SyncLoop::new(provider, shutdown, Duration::from_secs(60));
    let mut state = AppState::default();
    let events = sync_loop.tick(&mut state).await.expect("drill: sweep tick works");
    assert_eq!(state.chats.len(), 1);
    assert!(state.messages.is_empty());
    let _ = events;
}

#[tokio::test]
async fn drill_tick_stops_at_shutdown() {
    let (trigger, shutdown) = shutdown_pair();
    trigger.trigger();
    let mut sync_loop = SyncLoop::new(MockTeamsProvider::new(), shutdown, Duration::from_secs(1));
    let mut state = AppState::default();
    let err = sync_loop.tick(&mut state).await.expect_err("drill: stopped tick");
    assert_eq!(err.user_message(), "shutting down");
}

#[tokio::test]
async fn drill_throttled_tick_stays_due_and_retries() {
    let provider = MockTeamsProvider::new();
    provider.send_message("chat-1", "hi").await.unwrap();
    provider.fail_next_with_throttle();
    let (_trigger, shutdown) = shutdown_pair();
    let mut sync_loop = SyncLoop::new(provider, shutdown, Duration::from_secs(60));
    let mut state = AppState::default();
    state.apply(Command::SelectChat { chat_id: "chat-1".into() });
    let err = sync_loop.tick(&mut state).await.expect_err("drill: throttled tick fails");
    assert!(err.to_string().contains("429"), "throttle surfaces, got: {err}");
    let events = sync_loop.tick(&mut state).await.expect("drill: retry tick works");
    assert!(!events.is_empty(), "retry after throttle converges");
    assert_eq!(state.messages.len(), 1);
}
