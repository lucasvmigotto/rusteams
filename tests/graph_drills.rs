// Wiremock drills for the Graph HTTP adapter: behavior against a fake server,
// no credentials, no network beyond localhost.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use rusteams::infra::graph::GraphClient;
use wiremock::{matchers::method, Mock, MockServer, ResponseTemplate};

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
