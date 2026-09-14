// Visual-only notification queue: status-bar notices, no audible bell.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Incoming messages and errors surface as sanitized text notices with a TTL.
//! Deliberately no `\x07` bell: our own sanitizer strips control characters,
//! and unrequested audible output is hostile in shared terminals. A bell
//! option is documented future work, not MVP behavior.

use std::collections::{HashSet, VecDeque};
use std::time::{Duration, Instant};

/// One rendered notice line.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Notice {
    chat_id: String,
    text: String,
    created: Instant,
}

/// Bounded visual notice queue with per-chat unread flags.
#[derive(Debug)]
pub struct NoticeQueue {
    items: VecDeque<Notice>,
    cap: usize,
    ttl: Duration,
    unread_chats: HashSet<String>,
}

impl NoticeQueue {
    pub fn new(cap: usize) -> Self {
        Self::with_ttl(cap, Duration::from_secs(30))
    }

    pub fn with_ttl(cap: usize, ttl: Duration) -> Self {
        Self { items: VecDeque::new(), cap: cap.max(1), ttl, unread_chats: HashSet::new() }
    }

    /// Push a notice for `chat_id`. Text is sanitized at the boundary; oldest
    /// entries evict past capacity.
    pub fn push(&mut self, chat_id: &str, text: &str) {
        let clean = crate::sanitize::sanitize(text);
        while self.items.len() >= self.cap {
            self.items.pop_front();
        }
        self.items.push_back(Notice {
            chat_id: chat_id.into(),
            text: clean,
            created: Instant::now(),
        });
        self.unread_chats.insert(chat_id.into());
    }

    /// Drop notices older than the TTL.
    pub fn evict_expired(&mut self) {
        let ttl = self.ttl;
        self.items.retain(|n| n.created.elapsed() < ttl);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn texts(&self) -> Vec<String> {
        self.items.iter().map(|n| n.text.clone()).collect()
    }

    pub fn unread(&self, chat_id: &str) -> bool {
        self.unread_chats.contains(chat_id)
    }

    pub fn mark_read(&mut self, chat_id: &str) {
        self.unread_chats.remove(chat_id);
    }
}

/// Standard `Sender: body` notice text for an incoming message.
pub fn notify_for_message(sender: &str, body: &str) -> String {
    format!("{sender}: {body}")
}
