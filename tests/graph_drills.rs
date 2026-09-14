// Wiremock drills for the Graph HTTP adapter: behavior against a fake server,
// no credentials, no network beyond localhost.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use rusteams::infra::graph::GraphClient;
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
