use rusteams::provider::{ChatProvider, MockTeamsProvider};

#[tokio::test]
async fn full_stack_with_mock_provider_needs_no_network() {
    let provider = MockTeamsProvider::new();
    let chats = provider.list_chats().await.expect("mock lists chats");
    assert!(!chats.is_empty());
    let chat_id = chats[0].id.clone();

    let sent = provider.send_message(&chat_id, "\x1b[31mhello\x1b[0m").await.expect("mock sends");
    // Bodies are sanitized at the provider boundary.
    assert_eq!(sent.body, "hello");

    let mut msgs = provider.list_messages(&chat_id).await.expect("mock lists messages");
    rusteams::domain::order_messages(&mut msgs);
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].id, sent.id);
}
