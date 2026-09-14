// Notification queue tests: visual-only notices (no bell), sanitized text,
// bounded capacity, expiry, and unread flags per chat.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use rusteams::tui::{NoticeQueue, notify_for_message};
use std::time::Duration;

#[test]
fn queue_pushes_sanitized_notice_with_cap() {
    let mut q = NoticeQueue::new(2);
    q.push("chat-1", "\x1b[31mhi\x1b[0m");
    q.push("chat-1", "two");
    q.push("chat-2", "three");
    // Capacity 2: oldest evicted.
    assert_eq!(q.len(), 2);
    assert!(q.texts().iter().all(|t| !t.contains('\x1b')), "sanitized");
    assert_eq!(q.texts()[0], "two");
}

#[test]
fn unread_flags_track_chats_until_read() {
    let mut q = NoticeQueue::new(8);
    q.push("chat-1", "one");
    q.push("chat-2", "two");
    assert!(q.unread("chat-1"));
    assert!(q.unread("chat-2"));
    q.mark_read("chat-1");
    assert!(!q.unread("chat-1"));
    assert!(q.unread("chat-2"));
}

#[test]
fn expired_notices_drop_out() {
    let mut q = NoticeQueue::with_ttl(8, Duration::from_millis(1));
    q.push("chat-1", "old");
    std::thread::sleep(Duration::from_millis(5));
    q.evict_expired();
    assert_eq!(q.len(), 0);
}

#[test]
fn message_helper_builds_notice_text() {
    assert_eq!(notify_for_message("Alice", "hi"), "Alice: hi");
}
