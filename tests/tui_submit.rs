// Submit flow tests: composer text through provider send with optimistic
// confirm, using the offline mock. Provider failure leaves state untouched.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use rusteams::app::{AppState, Event};
use rusteams::provider::MockTeamsProvider;
use rusteams::tui::handle_submit;

#[tokio::test]
async fn submit_sends_and_confirms_single_message() {
    let provider = MockTeamsProvider::new();
    let mut state = AppState::default();
    let events =
        handle_submit(&mut state, &provider, "chat-1", "hello").await.expect("submit works");
    assert!(events.iter().any(|e| matches!(e, Event::MessageUpserted { .. })));
    assert_eq!(state.messages.len(), 1);
    assert_eq!(state.messages[0].body, "hello");
    assert!(!state.messages[0].id.starts_with("temp-"), "temp id swapped for real");
}

#[tokio::test]
async fn submit_rejects_blank_text_without_touching_state() {
    let provider = MockTeamsProvider::new();
    let mut state = AppState::default();
    let events = handle_submit(&mut state, &provider, "chat-1", "   ")
        .await
        .expect("blank is a rejection, not an error");
    assert_eq!(events, vec![Event::Rejected { reason: "empty body" }]);
    assert!(state.messages.is_empty());
}

#[tokio::test]
async fn submit_failure_leaves_state_untouched() {
    let provider = MockTeamsProvider::new();
    provider.fail_next_with_throttle();
    let mut state = AppState::default();
    let err = handle_submit(&mut state, &provider, "chat-1", "hello")
        .await
        .expect_err("throttled submit fails");
    assert!(err.to_string().contains("429"));
    assert!(state.messages.is_empty(), "optimistic temp rolled back");
}
