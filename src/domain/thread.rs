// Reply-thread tree builder.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use super::message::ChatMessage;
use serde::{Deserialize, Serialize};

/// A root message with its nested replies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThreadNode {
    pub message: ChatMessage,
    pub replies: Vec<ThreadNode>,
}

/// Build ordered thread trees from a flat message list.
/// - Roots (no `reply_to_id`, missing parent, or self-parent) sort oldest-first.
/// - Replies nest under their parent; orphans become roots.
/// - Cycles and self-replies never loop: offending links are cut to roots.
pub fn build_threads(msgs: Vec<ChatMessage>) -> Vec<ThreadNode> {
    use std::collections::{HashMap, HashSet};

    let mut by_id: HashMap<String, ChatMessage> = HashMap::new();
    for m in msgs {
        by_id.insert(m.id.clone(), m);
    }
    // Parent link, cut when missing/self/cyclic.
    let mut parent_of: HashMap<String, Option<String>> = HashMap::new();
    for (id, m) in &by_id {
        let parent = match &m.reply_to_id {
            Some(p) if p != id && by_id.contains_key(p) => Some(p.clone()),
            _ => None,
        };
        parent_of.insert(id.clone(), parent);
    }
    // Break longer cycles: walk ancestors; on revisit, cut this link.
    for id in by_id.keys().cloned().collect::<Vec<_>>() {
        let mut seen = HashSet::new();
        seen.insert(id.clone());
        let mut cursor = parent_of[&id].clone();
        while let Some(p) = cursor {
            if !seen.insert(p.clone()) {
                parent_of.insert(id.clone(), None);
                break;
            }
            cursor = parent_of[&p].clone();
        }
    }
    // Collect children per parent, then order roots oldest-first.
    let mut children: HashMap<String, Vec<ChatMessage>> = HashMap::new();
    let mut roots: Vec<ChatMessage> = Vec::new();
    let mut ordered: Vec<&ChatMessage> = by_id.values().collect();
    ordered.sort_by(|a, b| a.created.cmp(&b.created).then_with(|| a.id.cmp(&b.id)));
    for m in ordered {
        match &parent_of[m.id.as_str()] {
            Some(p) => children.entry(p.clone()).or_default().push(m.clone()),
            None => roots.push(m.clone()),
        }
    }
    fn node_for(msg: ChatMessage, children: &HashMap<String, Vec<ChatMessage>>) -> ThreadNode {
        let replies = children
            .get(&msg.id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|c| node_for(c, children))
            .collect();
        ThreadNode { message: msg, replies }
    }
    roots.into_iter().map(|r| node_for(r, &children)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    fn msg(id: &str, secs: i64, reply_to: Option<&str>) -> ChatMessage {
        let t = Utc.timestamp_opt(secs, 0).unwrap();
        ChatMessage {
            id: id.into(),
            chat_id: "c1".into(),
            created: t,
            modified: t,
            sender: "alice".into(),
            body: "hi".into(),
            reply_to_id: reply_to.map(str::to_string),
            reactions: vec![],
            mentions: vec![],
            is_read: false,
            segments: vec![],
        }
    }

    #[test]
    fn nests_replies_under_roots_in_order() {
        let v =
            vec![msg("r1", 30, Some("root")), msg("root", 10, None), msg("r2", 20, Some("root"))];
        let trees = build_threads(v);
        assert_eq!(trees.len(), 1);
        assert_eq!(trees[0].message.id, "root");
        assert_eq!(trees[0].replies.len(), 2);
        assert_eq!(trees[0].replies[0].message.id, "r2");
        assert_eq!(trees[0].replies[1].message.id, "r1");
    }

    #[test]
    fn orphans_and_self_replies_become_roots() {
        let v = vec![
            msg("orphan", 20, Some("missing")),
            msg("selfish", 30, Some("selfish")),
            msg("root", 10, None),
        ];
        let trees = build_threads(v);
        assert_eq!(trees.len(), 3);
        assert_eq!(trees[0].message.id, "root");
    }

    #[test]
    fn out_of_order_input_still_deterministic() {
        let a = vec![msg("b", 20, None), msg("a", 10, None)];
        let b = vec![msg("a", 10, None), msg("b", 20, None)];
        assert_eq!(build_threads(a), build_threads(b));
    }
}
