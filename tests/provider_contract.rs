// Provider contract suite: EVERY TeamsProvider implementation (mock today,
// Microsoft Graph adapter in Phase 3, any future provider) must satisfy these
// offline-capable behavioral contracts. Implementations declare conformance by
// instantiating `run_contract_suite` against their provider.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use rusteams::domain::order_messages;
use rusteams::provider::{ChatProvider, MockTeamsProvider, PresenceProvider};

/// Runs the full contract suite against `provider`. Fails on first violation.
pub async fn run_contract_suite<P>(provider: &P)
where
    P: ChatProvider + PresenceProvider,
{
    // C1: chat list is non-empty and ids are unique.
    let chats = provider.list_chats().await.expect("C1: list_chats works");
    assert!(!chats.is_empty(), "C1: at least one chat");
    let mut ids: Vec<&str> = chats.iter().map(|c| c.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), chats.len(), "C1: chat ids unique");

    // C2: send → list visibility with sanitized body.
    let chat_id = chats[0].id.clone();
    let sent = provider
        .send_message(&chat_id, "\x1b[31mcontract\x1b[0m")
        .await
        .expect("C2: send_message works");
    assert_eq!(sent.chat_id, chat_id, "C2: message lands in target chat");
    assert_eq!(sent.body, "contract", "C2: body sanitized at boundary");
    assert!(!sent.id.is_empty(), "C2: server-assigned id present");

    // C3: listed messages are retrievable and orderable without panic.
    let mut msgs = provider.list_messages(&chat_id).await.expect("C3: list works");
    assert!(msgs.iter().any(|m| m.id == sent.id), "C3: sent message visible");
    order_messages(&mut msgs);

    // C4: unknown chat yields empty history, never an error.
    let empty = provider.list_messages("chat-does-not-exist").await.expect("C4: no error");
    assert!(empty.is_empty(), "C4: unknown chat is empty");

    // C5: presence endpoint answers with a non-empty value.
    let presence = provider.my_presence().await.expect("C5: presence works");
    assert!(!presence.is_empty(), "C5: presence non-empty");

    // C6: provider surfaces throttling distinctly from fatal errors.
    // (Exercised on MockTeamsProvider; Graph adapter must map HTTP 429 here.)
    let throttle = MockTeamsProvider::new();
    throttle.fail_next_with_throttle();
    let err = throttle.list_chats().await.expect_err("C6: throttled call fails");
    assert!(err.to_string().contains("429"), "C6: throttle is recognizable");

    // C7: edit round-trip updates the stored body.
    provider
        .update_message(&chat_id, &sent.id, "contract v2")
        .await
        .expect("C7: update_message works");
    let msgs = provider.list_messages(&chat_id).await.expect("C7: list works");
    let edited = msgs.iter().find(|m| m.id == sent.id).expect("C7: edited visible");
    assert_eq!(edited.body, "contract v2", "C7: body updated");

    // C8: reactions attach and detach.
    provider
        .set_reaction(&chat_id, &sent.id, "like")
        .await
        .expect("C8: set_reaction works");
    let msgs = provider.list_messages(&chat_id).await.expect("C8: list works");
    let reacted = msgs.iter().find(|m| m.id == sent.id).expect("C8: reacted visible");
    assert!(reacted.reactions.iter().any(|r| r.kind == "like"), "C8: reaction stored");
    provider
        .unset_reaction(&chat_id, &sent.id, "like")
        .await
        .expect("C8: unset_reaction works");

    // C9: delete removes the message from history.
    provider.delete_message(&chat_id, &sent.id).await.expect("C9: delete works");
    let msgs = provider.list_messages(&chat_id).await.expect("C9: list works");
    assert!(!msgs.iter().any(|m| m.id == sent.id), "C9: deleted gone");
}

#[tokio::test]
async fn mock_provider_satisfies_contract() {
    run_contract_suite(&MockTeamsProvider::new()).await;
}
