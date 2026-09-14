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

fn graph_message_with_mention(id: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "createdDateTime": "2026-09-14T10:00:00Z",
        "lastModifiedDateTime": "2026-09-14T10:00:00Z",
        "body": {
            "contentType": "html",
            "content": "hi <at id=\"0\">Alice</at>, review this"
        },
        "from": { "user": { "displayName": "Bob" } },
        "mentions": [{
            "id": 0,
            "mentionText": "Alice",
            "mentioned": {
                "user": {
                    "id": "user-1",
                    "displayName": "Alice",
                    "userIdentityType": "aadUser"
                }
            }
        }]
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

#[tokio::test]
async fn drill_update_returns_confirmed_message() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .respond_with(ResponseTemplate::new(204))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(graph_message("m1", "edited")))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let msg = client.update_message("chat-1", "m1", "edited").await.expect("drill: update works");
    assert_eq!(msg.id, "m1");
    assert_eq!(msg.body, "edited");
}

#[tokio::test]
async fn drill_delete_and_reactions_succeed() {
    let server = MockServer::start().await;
    Mock::given(method("POST")).respond_with(ResponseTemplate::new(204)).mount(&server).await;

    let client = GraphClient::new(&server.uri(), "test-token");
    client.delete_message("chat-1", "m1").await.expect("drill: delete works");
    client.set_reaction("chat-1", "m1", "like").await.expect("drill: react works");
    client.unset_reaction("chat-1", "m1", "like").await.expect("drill: unreact works");
}

#[tokio::test]
async fn drill_reply_with_quote_returns_created_message() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(201).set_body_json(graph_message("m7", "quoting you")))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let msg = client
        .reply_with_quote("chat-1", &["m1".to_string()], "quoting you")
        .await
        .expect("drill: quote works");
    assert_eq!(msg.id, "m7");
    assert_eq!(msg.body, "quoting you");
}

#[tokio::test]
async fn drill_hosted_content_returns_typed_bytes() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "image/png")
                .set_body_bytes(b"\x89PNG\r\n\x1a\n".to_vec()),
        )
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let content =
        client.get_hosted_content("chat-1", "m1", "h1").await.expect("drill: hosted works");
    assert_eq!(content.content_type, "image/png");
    assert_eq!(content.bytes, b"\x89PNG\r\n\x1a\n");
}

#[tokio::test]
async fn drill_send_mention_posts_html_with_at_tag() {
    use wiremock::matchers::{body_json, method, path_regex};

    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex(r"/me/chats/.*/messages$"))
        .and(body_json(serde_json::json!({
            "body": {
                "contentType": "html",
                "content": "hi <at id=\"0\">Alice</at>"
            },
            "mentions": [{
                "id": 0,
                "mentionText": "Alice",
                "mentioned": {
                    "user": {
                        "id": "user-1",
                        "displayName": "Alice",
                        "userIdentityType": "aadUser"
                    }
                }
            }]
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(graph_message("m8", "hi Alice")))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let msg = client
        .send_mention("chat-1", "hi ", "user-1", "Alice")
        .await
        .expect("drill: mention works");
    assert_eq!(msg.id, "m8");
}

#[tokio::test]
async fn drill_lists_hosted_contents() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "value": [{ "id": "h1", "contentType": "image/png" }]
        })))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let list = client.list_hosted_contents("chat-1", "m1").await.expect("drill: hosted list works");
    assert_eq!(list, vec![("h1".to_string(), "image/png".to_string())]);
}

#[tokio::test]
async fn drill_mention_maps_to_at_name_and_mention() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "value": [graph_message_with_mention("m3")]
        })))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let msgs = client.list_messages("chat-1").await.expect("drill: mention list works");
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].body, "hi @Alice, review this");
    assert_eq!(msgs[0].mentions.len(), 1);
    assert_eq!(msgs[0].mentions[0].display_name, "Alice");
    assert_eq!(msgs[0].mentions[0].user_id.as_deref(), Some("user-1"));
}

#[tokio::test]
async fn drill_incremental_poll_uses_filter_and_top() {
    use wiremock::matchers::{method, path_regex, query_param};
    use chrono::{TimeZone, Utc};

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(r"/me/chats/c1/messages$"))
        .and(query_param("$top", "25"))
        .and(query_param("$filter", "lastModifiedDateTime gt 2026-09-14T10:00:00+00:00"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "value": [graph_message("m2", "fresh")]
        })))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let since = Utc.timestamp_opt(1_789_380_000, 0).unwrap();
    let msgs =
        client.list_messages_since("c1", since, 25).await.expect("drill: incremental works");
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].id, "m2");
}

#[tokio::test]
async fn drill_preview_hydrates_chat_list() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "value": [{
                "id": "chat-1",
                "topic": "Engineering",
                "lastMessagePreview": {
                    "createdDateTime": "2026-09-14T10:05:00Z",
                    "body": { "content": "latest news" }
                }
            }]
        })))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let chats = client.list_chats().await.expect("drill: preview list works");
    assert_eq!(chats.len(), 1);
    assert_eq!(chats[0].last_message_preview.as_deref(), Some("latest news"));
    assert!(chats[0].last_message_at.is_some(), "preview timestamp mapped");
}

#[tokio::test]
async fn drill_single_message_hydrates_search_hit() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(graph_message("m9", "full body")))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let msg = client.get_message("chat-1", "m9").await.expect("drill: hydrate works");
    assert_eq!(msg.body, "full body");
    assert_eq!(msg.chat_id, "chat-1");
}
