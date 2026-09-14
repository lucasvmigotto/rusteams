// Wiremock drills for the Graph HTTP adapter: behavior against a fake server,
// no credentials, no network beyond localhost.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use rusteams::infra::graph::GraphClient;
use rusteams::provider::{ChatProvider, PresenceProvider};
use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

#[tokio::test]
async fn drill_lists_chats_from_graph_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "value": [
                { "id": "chat-1", "topic": "Engineering" },
                { "id": "chat-2", "topic": null }
            ]
        })))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let chats = client.list_chats().await.expect("drill: list_chats works");
    assert_eq!(chats.len(), 2);
    assert_eq!(chats[0].id, "chat-1");
    assert_eq!(chats[0].topic.as_deref(), Some("Engineering"));
}

#[tokio::test]
async fn drill_retries_once_after_429_then_succeeds() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(429).append_header("Retry-After", "1"))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "value": [{ "id": "chat-9", "topic": null }]
        })))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let chats = client.list_chats().await.expect("drill: 429 is retried");
    assert_eq!(chats.len(), 1);
    assert_eq!(chats[0].id, "chat-9");
}

#[tokio::test]
async fn drill_follows_next_link_pages() {
    let server = MockServer::start().await;
    let next = format!("{}/me/chats?$skip=1", server.uri());
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "value": [{ "id": "chat-1", "topic": null }],
            "@odata.nextLink": next,
        })))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "value": [{ "id": "chat-2", "topic": null }]
        })))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let chats = client.list_chats().await.expect("drill: pages followed");
    let ids: Vec<&str> = chats.iter().map(|c| c.id.as_str()).collect();
    assert_eq!(ids, vec!["chat-1", "chat-2"]);
}

#[tokio::test]
async fn drill_rejects_unauthorized_without_retry() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).respond_with(ResponseTemplate::new(401)).mount(&server).await;

    let client = GraphClient::new(&server.uri(), "bad-token");
    let err = client.list_chats().await.expect_err("drill: 401 fails");
    assert_eq!(err.user_message(), "authentication failed");
}

#[tokio::test]
async fn drill_rejects_malformed_payload() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not json{{"))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let err = client.list_chats().await.expect_err("drill: garbage fails");
    assert!(err.to_string().contains("malformed"));
}

fn graph_message(id: &str, body: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "createdDateTime": "2026-09-14T10:00:00Z",
        "lastModifiedDateTime": "2026-09-14T10:00:00Z",
        "body": { "contentType": "text", "content": body },
        "from": { "user": { "displayName": "Alice" } }
    })
}

#[tokio::test]
async fn drill_lists_messages_with_sanitized_bodies() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "value": [
                graph_message("m1", "hello"),
                graph_message("m2", "\x1b[31mhi\x1b[0m"),
            ]
        })))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let msgs = client.list_messages("chat-1").await.expect("drill: list works");
    assert_eq!(msgs.len(), 2);
    assert_eq!(msgs[0].chat_id, "chat-1");
    assert_eq!(msgs[0].body, "hello");
    assert_eq!(msgs[1].body, "hi");
}

#[tokio::test]
async fn drill_send_returns_created_message() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(201).set_body_json(graph_message("m9", "sent!")))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let msg = client.send_message("chat-1", "sent!").await.expect("drill: send works");
    assert_eq!(msg.id, "m9");
    assert_eq!(msg.body, "sent!");
}

#[tokio::test]
async fn drill_adapter_serves_provider_traits() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "value": [{ "id": "chat-1", "topic": null }]
        })))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "availability": "Available"
        })))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let provider: &dyn ChatProvider = &client;
    assert_eq!(provider.list_chats().await.expect("drill: trait list").len(), 1);
    let presence: &dyn PresenceProvider = &client;
    assert_eq!(presence.my_presence().await.expect("drill: trait presence"), "Available");
}
