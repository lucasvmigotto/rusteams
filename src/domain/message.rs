// Domain models: chats and messages as normalized Graph-independent types.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single chat message in normalized form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub chat_id: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub sender: String,
    /// Sanitized plain-text body ready for terminal rendering.
    pub body: String,
    pub reply_to_id: Option<String>,
    /// Reactions attached to this message.
    #[serde(default)]
    pub reactions: Vec<Reaction>,
    /// Mentions referenced by this message.
    #[serde(default)]
    pub mentions: Vec<Mention>,
    /// Whether the signed-in user has read this message.
    #[serde(default)]
    pub is_read: bool,
}

/// A normalized reaction (e.g. like, heart) from one user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reaction {
    pub kind: String,
    pub user_id: String,
    pub display_name: String,
}

/// A normalized @-mention inside a message body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mention {
    pub user_id: Option<String>,
    pub display_name: String,
    pub offset: usize,
    pub length: usize,
}

/// Sort messages oldest-first; tie-break on id for determinism.
/// Total and deterministic for inputs with unique ids (the Graph invariant);
/// resolve duplicates with [`dedupe_messages`] first.
pub fn order_messages(msgs: &mut [ChatMessage]) {
    msgs.sort_by(|a, b| a.created.cmp(&b.created).then_with(|| a.id.cmp(&b.id)));
}

/// Deduplicate by message id, keeping the most recently modified copy.
pub fn dedupe_messages(msgs: Vec<ChatMessage>) -> Vec<ChatMessage> {
    use std::collections::HashMap;
    let mut best: HashMap<String, ChatMessage> = HashMap::new();
    for m in msgs {
        best.entry(m.id.clone())
            .and_modify(|e| {
                if m.modified > e.modified {
                    *e = m.clone();
                }
            })
            .or_insert(m);
    }
    let mut out: Vec<_> = best.into_values().collect();
    order_messages(&mut out);
    out
}

/// Diff a freshly fetched page against cached ids.
/// Returns `(new_or_updated, deleted_ids)` where deletion is inferred from
/// ids present in cache but absent from a complete fetch.
pub fn diff_sync(
    cached: &[ChatMessage],
    fetched: &[ChatMessage],
    fetch_is_complete: bool,
) -> (Vec<ChatMessage>, Vec<String>) {
    use std::collections::{HashMap, HashSet};
    let cache: HashMap<&str, &ChatMessage> = cached.iter().map(|m| (m.id.as_str(), m)).collect();
    let fetched_ids: HashSet<&str> = fetched.iter().map(|m| m.id.as_str()).collect();
    let mut changed = Vec::new();
    for m in fetched {
        match cache.get(m.id.as_str()) {
            Some(c) if *c == m => {}
            _ => changed.push(m.clone()),
        }
    }
    let mut deleted = Vec::new();
    if fetch_is_complete {
        for c in cached {
            if !fetched_ids.contains(c.id.as_str()) {
                deleted.push(c.id.clone());
            }
        }
    }
    (changed, deleted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn msg(id: &str, secs: i64) -> ChatMessage {
        let t = Utc.timestamp_opt(secs, 0).unwrap();
        ChatMessage {
            id: id.into(),
            chat_id: "c1".into(),
            created: t,
            modified: t,
            sender: "alice".into(),
            body: "hi".into(),
            reply_to_id: None,
            reactions: vec![],
            mentions: vec![],
            is_read: false,
        }
    }

    #[test]
    fn ordering_is_oldest_first_and_deterministic() {
        let mut v = vec![msg("b", 20), msg("a", 10), msg("c", 10)];
        order_messages(&mut v);
        assert_eq!(v[0].id, "a");
        assert_eq!(v[1].id, "c");
        assert_eq!(v[2].id, "b");
    }

    #[test]
    fn dedupe_keeps_newest_modified_copy() {
        let mut m2 = msg("x", 10);
        m2.modified = Utc.timestamp_opt(99, 0).unwrap();
        m2.body = "edited".into();
        let out = dedupe_messages(vec![m2.clone(), msg("x", 10)]);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].body, "edited");
    }

    #[test]
    fn diff_detects_new_updated_and_deleted() {
        let cached = vec![msg("keep", 10), msg("gone", 10)];
        let mut changed_msg = msg("keep", 10);
        changed_msg.body = "edited".into();
        let fetched = vec![changed_msg, msg("fresh", 30)];
        let (changed, deleted) = diff_sync(&cached, &fetched, true);
        assert_eq!(changed.len(), 2);
        assert_eq!(deleted, vec!["gone".to_string()]);
    }

    #[test]
    fn diff_does_not_report_deletions_on_partial_fetch() {
        let cached = vec![msg("a", 10)];
        let (changed, deleted) = diff_sync(&cached, &[], false);
        assert!(changed.is_empty());
        assert!(deleted.is_empty());
    }
}
