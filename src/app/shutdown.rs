// Cooperative shutdown and drop-guard primitives.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! UI, sync, and network tasks share a [`Shutdown`] signal. Cleanup that must
//! run even on early return (e.g. terminal restore) goes in a [`DropGuard`].

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

/// Broadcast shutdown signal. Clone freely across tasks.
#[derive(Debug, Clone)]
pub struct Shutdown {
    flag: Arc<AtomicBool>,
}

/// The single owner that triggers shutdown.
#[derive(Debug)]
pub struct ShutdownTrigger {
    flag: Arc<AtomicBool>,
}

impl Shutdown {
    /// Create a linked trigger/signal pair, initially untriggered.
    pub fn new() -> (ShutdownTrigger, Shutdown) {
        let flag = Arc::new(AtomicBool::new(false));
        (ShutdownTrigger { flag: flag.clone() }, Shutdown { flag })
    }

    /// Returns true once the trigger has fired.
    pub fn is_triggered(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }

    /// Resolve when shutdown fires. Pure polling future — no runtime handle needed.
    pub async fn wait(&self) {
        while !self.is_triggered() {
            tokio::task::yield_now().await;
        }
    }
}

impl ShutdownTrigger {
    /// Fire the signal. Idempotent.
    pub fn trigger(&self) {
        self.flag.store(true, Ordering::SeqCst);
    }
}

/// Runs `cleanup` exactly once when dropped. Used for terminal restore and
/// other must-run teardown; the TUI wires the real restore closure in Phase 6.
pub struct DropGuard<F: FnOnce()> {
    cleanup: Option<F>,
}

impl<F: FnOnce()> DropGuard<F> {
    pub fn new(cleanup: F) -> Self {
        Self { cleanup: Some(cleanup) }
    }
}

impl<F: FnOnce()> Drop for DropGuard<F> {
    fn drop(&mut self) {
        if let Some(cleanup) = self.cleanup.take() {
            cleanup();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::time::Duration;

    #[tokio::test]
    async fn trigger_resolves_waiters() {
        let (trigger, shutdown) = Shutdown::new();
        assert!(!shutdown.is_triggered());
        trigger.trigger();
        assert!(shutdown.is_triggered());
        tokio::time::timeout(Duration::from_secs(1), shutdown.wait())
            .await
            .expect("wait resolves after trigger");
    }

    #[tokio::test]
    async fn clones_share_the_signal() {
        let (trigger, shutdown) = Shutdown::new();
        let other = shutdown.clone();
        trigger.trigger();
        assert!(other.is_triggered());
    }

    #[test]
    fn drop_guard_runs_cleanup_exactly_once() {
        let runs = Arc::new(AtomicUsize::new(0));
        {
            let flag = runs.clone();
            let _guard = DropGuard::new(move || {
                flag.fetch_add(1, Ordering::SeqCst);
            });
        }
        assert_eq!(runs.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn shutdown_mid_sequence_leaves_valid_state() {
        use crate::app::{AppState, Command};
        use crate::domain::ChatMessage;
        use chrono::{TimeZone, Utc};

        let (trigger, shutdown) = Shutdown::new();
        let mut state = AppState::default();
        let t = Utc.timestamp_opt(10, 0).unwrap();
        let mk = |id: &str| ChatMessage {
            id: id.into(),
            chat_id: "c1".into(),
            created: t,
            modified: t,
            sender: "a".into(),
            body: "x".into(),
            reply_to_id: None,
            reactions: vec![],
            mentions: vec![],
            is_read: false,
            segments: vec![],
        };
        state.apply(Command::MessageReceived { message: mk("m2") });
        trigger.trigger(); // sync task would stop here
        assert!(shutdown.is_triggered());
        // Reducers are atomic: state is always valid, further applies still work.
        state.apply(Command::MessageReceived { message: mk("m1") });
        let ids: Vec<&str> = state.messages.iter().map(|m| m.id.as_str()).collect();
        assert_eq!(ids, vec!["m1", "m2"]);
    }
}
