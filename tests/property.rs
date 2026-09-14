// Property-based invariants: sanitizer output safety, ordering/dedupe
// idempotence, and sync-diff completeness over randomized inputs.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use chrono::{TimeZone, Utc};
use proptest::prelude::*;
use rusteams::domain::{ChatMessage, build_threads, dedupe_messages, diff_sync, order_messages};
use rusteams::sanitize::sanitize;

/// Graph invariant: message ids are unique. The generator assigns positional
/// ids so ordering tests exercise permutation-invariance, not duplicate
/// handling (duplicates belong to `dedupe_messages`, tested separately).
fn arb_unique_messages() -> impl Strategy<Value = Vec<ChatMessage>> {
    proptest::collection::vec((0..1000i64, proptest::option::of(0..8u8)), 0..20).prop_map(|pairs| {
        pairs
            .into_iter()
            .enumerate()
            .map(|(i, (secs, reply))| {
                let t = Utc.timestamp_opt(secs, 0).unwrap();
                ChatMessage {
                    id: format!("m{i}"),
                    chat_id: "c1".into(),
                    created: t,
                    modified: t,
                    sender: "alice".into(),
                    body: "hi".into(),
                    reply_to_id: reply.map(|r| format!("m{r}")),
                    reactions: vec![],
                    mentions: vec![],
                    is_read: false,
                }
            })
            .collect()
    })
}

fn arb_message() -> impl Strategy<Value = ChatMessage> {
    (0..8u8, 0..1000i64, proptest::option::of(0..8u8)).prop_map(|(id, secs, reply)| {
        let t = Utc.timestamp_opt(secs, 0).unwrap();
        ChatMessage {
            id: format!("m{id}"),
            chat_id: "c1".into(),
            created: t,
            modified: t,
            sender: "alice".into(),
            body: "hi".into(),
            reply_to_id: reply.map(|r| format!("m{r}")),
            reactions: vec![],
            mentions: vec![],
            is_read: false,
        }
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn sanitize_never_emits_terminal_controls(s in "\\PC*") {
        let out = sanitize(&s);
        for c in out.chars() {
            prop_assert!(
                c == '\n' || c == '\t' || !c.is_control(),
                "control char {c:?} survived sanitization"
            );
        }
        prop_assert!(!out.contains('\x1b'), "ESC survived sanitization");
        prop_assert!(!out.contains('\x7f'), "DEL survived sanitization");
    }

    #[test]
    fn sanitize_is_idempotent(s in "\\PC*") {
        prop_assert_eq!(sanitize(&sanitize(&s)), sanitize(&s));
    }

    #[test]
    fn ordering_is_idempotent_and_total(msgs in arb_unique_messages()) {
        let mut a = msgs.clone();
        order_messages(&mut a);
        let mut b = a.clone();
        order_messages(&mut b);
        prop_assert_eq!(&a, &b);
        // Total order: deterministic regardless of input permutation.
        let mut rev = msgs.clone();
        rev.reverse();
        order_messages(&mut rev);
        prop_assert_eq!(&a, &rev);
    }

    #[test]
    fn dedupe_is_idempotent(msgs in proptest::collection::vec(arb_message(), 0..20)) {
        let once = dedupe_messages(msgs);
        prop_assert_eq!(dedupe_messages(once.clone()), once);
    }

    #[test]
    fn diff_reports_every_new_message(
        cached in proptest::collection::vec(arb_message(), 0..10),
        extra in proptest::collection::vec(arb_message(), 0..10),
    ) {
        let mut fetched = dedupe_messages(cached.clone());
        for m in dedupe_messages(extra) {
            if !fetched.iter().any(|e| e == &m) {
                fetched.push(m);
            }
        }
        let (changed, _) = diff_sync(&dedupe_messages(cached.clone()), &fetched, false);
        for m in &fetched {
            if !dedupe_messages(cached.clone()).contains(m) {
                prop_assert!(changed.contains(m), "new message {} missed", m.id);
            }
        }
    }

    #[test]
    fn thread_builder_never_panics_or_loops(msgs in proptest::collection::vec(arb_message(), 0..20)) {
        let trees = build_threads(msgs.clone());
        let count: usize = trees.iter().map(|t| 1 + t.replies.len()).sum();
        prop_assert!(count <= msgs.len().max(1) * 2);
    }
}
