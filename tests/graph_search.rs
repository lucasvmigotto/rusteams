// Message-search drills: /search/query against a fake server, hit mapping
// with sanitized summaries.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use rusteams::infra::graph::GraphClient;
use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

fn search_response() -> serde_json::Value {
    serde_json::json!({
        "value": [{
            "hitsContainers": [{
                "hits": [
                    {
                        "hitId": "m1",
                        "rank": 1,
                        "summary": "quarterly \x1b[31mreport\x1b[0m draft",
                        "resource": {
                            "@odata.type": "#microsoft.graph.chatMessage",
                            "id": "m1",
                            "chatId": "chat-1",
                            "subject": "Q1 planning"
                        }
                    },
                    {
                        "hitId": "m2",
                        "rank": 2,
                        "summary": "plain summary",
                        "resource": {
                            "@odata.type": "#microsoft.graph.chatMessage",
                            "id": "m2"
                        }
                    }
                ]
            }]
        }]
    })
}

#[tokio::test]
async fn drill_search_maps_hits_with_sanitized_summaries() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(search_response()))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let hits = client.search_messages("report", 10).await.expect("drill: search works");
    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].message_id, "m1");
    assert_eq!(hits[0].chat_id.as_deref(), Some("chat-1"));
    assert_eq!(hits[0].subject.as_deref(), Some("Q1 planning"));
    assert_eq!(hits[0].summary, "quarterly report draft");
    assert_eq!(hits[1].chat_id, None, "missing fields stay missing");
}

#[tokio::test]
async fn drill_empty_search_returns_no_hits() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "value": [{ "hitsContainers": [{ "hits": [] }] }]
        })))
        .mount(&server)
        .await;

    let client = GraphClient::new(&server.uri(), "test-token");
    let hits = client.search_messages("nothing-matches-xyz", 10).await.expect("drill: empty ok");
    assert!(hits.is_empty());
}
