// Polling-first sync engine: full-fetch polls per chat, diffed into AppState.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! MVP realtime without webhooks: each poll fetches the whole visible history,
//! diffs it, and applies the delta through the pure reducer. Duplicates and
//! reordering collapse in `diff_sync`/`order_messages`; gaps heal on the next
//! complete fetch (watermark re-baselining). Honors `Shutdown` fail-fast.

use super::connection::{ConnectionEvent, backoff_delay};
use super::reducer::{AppState, Command, Event};
use super::shutdown::{Shutdown, ShutdownTrigger};
use crate::app::app_state::ConnectionState;
use crate::domain::diff_sync;
use crate::error::AppError;
use crate::provider::ChatProvider;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Per-chat last-successful-poll watermarks driving poll scheduling.
#[derive(Debug, Default)]
pub struct Watermarks {
    last_poll: HashMap<String, Instant>,
}

impl Watermarks {
    /// Record a successful poll of `chat_id` now.
    pub fn mark_synced(&mut self, chat_id: &str) {
        self.last_poll.insert(chat_id.into(), Instant::now());
    }

    /// True when `chat_id` has never synced or its watermark is older than `interval`.
    pub fn is_due(&self, chat_id: &str, interval: Duration) -> bool {
        match self.last_poll.get(chat_id) {
            Some(t) => t.elapsed() >= interval,
            None => true,
        }
    }

    /// Forget all watermarks so every chat re-baselines on the next tick.
    /// Call this on reconnect: gaps heal via complete-fetch diffs.
    pub fn reset(&mut self) {
        self.last_poll.clear();
    }
}

/// Polls one provider into shared app state. Cheap to clone per task.
pub struct Poller<P> {
    provider: P,
    shutdown: Shutdown,
}

impl<P> Poller<P> {
    /// Borrow the backing provider (sweeps share the same source of truth).
    pub fn provider(&self) -> &P {
        &self.provider
    }

    /// True once the shared shutdown fired.
    pub fn shutdown_triggered(&self) -> bool {
        self.shutdown.is_triggered()
    }
}

impl<P: ChatProvider> Poller<P> {
    pub fn new(provider: P, shutdown: Shutdown) -> Self {
        Self { provider, shutdown }
    }

    /// One full-fetch poll of `chat_id`: diff against cached scope, apply the
    /// delta, return reducer events (empty when already converged).
    pub async fn poll_once(
        &self,
        state: &mut AppState,
        chat_id: &str,
    ) -> Result<Vec<Event>, AppError> {
        if self.shutdown.is_triggered() {
            return Err(AppError::Shutdown);
        }
        let fetched = self.provider.list_messages(chat_id).await?;
        let cached: Vec<_> =
            state.messages.iter().filter(|m| m.chat_id == chat_id).cloned().collect();
        let (changed, deleted) = diff_sync(&cached, &fetched, true);
        if changed.is_empty() && deleted.is_empty() {
            return Ok(vec![]);
        }
        Ok(state.apply(Command::SyncDiffApplied { chat_id: chat_id.into(), changed, deleted }))
    }
}

/// Sweep the chat list into state. Returns the replace event (single item).
pub async fn refresh_chats<P: ChatProvider>(
    state: &mut AppState,
    provider: &P,
) -> Result<Vec<Event>, AppError> {
    let chats = provider.list_chats().await?;
    Ok(state.apply(Command::ChatsLoaded { chats }))
}

/// One scheduler tick: poll the selected chat when its watermark is due.
/// Marks success only — throttled/failed polls stay due for the next tick.
pub async fn poll_due_chats<P: ChatProvider>(
    state: &mut AppState,
    poller: &Poller<P>,
    marks: &mut Watermarks,
    interval: Duration,
) -> Result<Vec<Event>, AppError> {
    let selected = match state.selected_chat.clone() {
        Some(id) => id,
        None => return Ok(vec![]),
    };
    if !marks.is_due(&selected, interval) {
        return Ok(vec![]);
    }
    let events = poller.poll_once(state, &selected).await?;
    marks.mark_synced(&selected);
    Ok(events)
}

/// Bind connection events to sync state: drive the reducer, and when the
/// machine lands on `Reconnecting`, clear watermarks so the next ticks
/// re-baseline every chat with complete-fetch diffs (gap healing).
pub fn note_connection(
    state: &mut AppState,
    marks: &mut Watermarks,
    ev: ConnectionEvent,
) -> Vec<Event> {
    let events = state.apply(Command::ConnectionEvent(ev));
    if state.connection == ConnectionState::Reconnecting {
        marks.reset();
    }
    events
}

/// Supervised sync loop: owns a [`SyncLoop`], sleeps `interval` between ticks,
/// backs off exponentially (capped) on failed ticks, and stops on shutdown.
/// `run` bounds the iteration count so the whole supervisor stays testable;
/// the endeless production loop is `run` without a bound (same code path).
pub struct Supervisor<P> {
    sync_loop: SyncLoop<P>,
    shutdown: Shutdown,
    interval: Duration,
    backoff_base: Duration,
}

impl<P: ChatProvider> Supervisor<P> {
    pub fn new(
        provider: P,
        shutdown: Shutdown,
        interval: Duration,
        backoff_base: Duration,
    ) -> Self {
        Self {
            sync_loop: SyncLoop::new(provider, shutdown.clone(), interval),
            shutdown,
            interval,
            backoff_base,
        }
    }

    /// Run up to `max_ticks` iterations. Fails fast on shutdown; backoff
    /// sleeps (jitter 0 in-loop; callers add jitter) cap reconnect storms.
    pub async fn run(
        &mut self,
        state: &mut AppState,
        max_ticks: u32,
    ) -> Result<Vec<Event>, AppError> {
        let mut out = Vec::new();
        let mut failures: u32 = 0;
        for _ in 0..max_ticks {
            if self.shutdown.is_triggered() {
                return Err(AppError::Shutdown);
            }
            match self.sync_loop.tick(state).await {
                Ok(mut events) => {
                    failures = 0;
                    out.append(&mut events);
                }
                Err(AppError::Shutdown) => return Err(AppError::Shutdown),
                Err(_) => {
                    failures += 1;
                    let wait = backoff_delay(failures, self.backoff_base, self.interval, 0);
                    tokio::time::sleep(wait).await;
                }
            }
            tokio::time::sleep(self.interval).await;
        }
        Ok(out)
    }

    /// Endless production loop: bounded runs back-to-back until shutdown.
    /// Same code path as the drilled bounded runs; returns Err(Shutdown).
    pub async fn run_endless(&mut self, state: &mut AppState) -> Result<Vec<Event>, AppError> {
        loop {
            // u32::MAX iterations per chunk keeps watermarks/backoff continuous
            // while preserving the testable bound.
            self.run(state, u32::MAX).await?;
        }
    }
}

/// Spawn a SIGINT watcher that fires the shared shutdown. The runtime owns
/// the task; dropping the handle does not disarm it. Live-signal delivery is
/// manual-verification only (see live-verification runbook).
pub fn watch_ctrl_c(trigger: ShutdownTrigger) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            trigger.trigger();
        }
    })
}

/// Timer-driven sync loop: one scheduler owning poller, watermarks, and
/// interval. `tick` sweeps the chat list, then polls the selected chat when
/// due. Throttled/failed ticks stay due — the loop never hammers the provider.
/// The sleeping supervisor (interval sleep + shutdown select) lives with the
/// runtime; this type is the testable scheduling core.
pub struct SyncLoop<P> {
    poller: Poller<P>,
    marks: Watermarks,
    interval: Duration,
}

impl<P: ChatProvider> SyncLoop<P> {
    pub fn new(provider: P, shutdown: Shutdown, interval: Duration) -> Self {
        Self { poller: Poller::new(provider, shutdown), marks: Watermarks::default(), interval }
    }

    /// One scheduler iteration. Fails fast on shutdown without touching state.
    pub async fn tick(&mut self, state: &mut AppState) -> Result<Vec<Event>, AppError> {
        if self.poller.shutdown_triggered() {
            return Err(AppError::Shutdown);
        }
        let mut events = refresh_chats(state, self.poller.provider()).await?;
        events.extend(poll_due_chats(state, &self.poller, &mut self.marks, self.interval).await?);
        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn watermarks_are_due_until_synced() {
        let mut w = Watermarks::default();
        assert!(w.is_due("c1", Duration::from_secs(15)));
        w.mark_synced("c1");
        assert!(!w.is_due("c1", Duration::from_secs(3600)));
        assert!(w.is_due("c1", Duration::from_secs(0)));
    }
}
