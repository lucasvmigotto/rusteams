// Refresh-token drills: silent session renewal against a fake Entra server,
// refresh rotation persistence, and fail-closed expiry.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use rusteams::infra::auth::{RefreshClient, default_scopes, refresh_session};
use rusteams::infra::auth::token_store::MemoryStore;
use wiremock::{matchers::method, Mock, MockServer, ResponseTemplate};

fn token_body(access: &str, refresh: Option<&str>) -> serde_json::Value {
    serde_json::json!({
        "token_type": "Bearer",
        "scope": default_scopes(),
        "expires_in": 3600,
        "access_token": access,
        "refresh_token": refresh,
    })
}

#[tokio::test]
async fn drill_refresh_exchanges_rotates_and_persists() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(token_body("at-new", Some("rt-new"))))
        .mount(&server)
        .await;

    let store = MemoryStore::default();
    store.save_refresh_token("default", "rt-old").unwrap();
    let client = RefreshClient::new(&server.uri(), "tenant", "client-id");
    let access = refresh_session(&client, &store, "default").await.expect("drill: refresh works");
    assert_eq!(access, "at-new");
    assert_eq!(
        store.load_refresh_token("default").unwrap().as_deref(),
        Some("rt-new"),
        "rotated refresh token persists"
    );
}

#[tokio::test]
async fn drill_refresh_without_stored_token_fails_closed() {
    let server = MockServer::start().await;
    let store = MemoryStore::default();
    let client = RefreshClient::new(&server.uri(), "tenant", "client-id");
    let err = refresh_session(&client, &store, "default").await.expect_err("drill: no session");
    assert!(err.user_message().contains("log in"), "guides back to login, got: {}", err.user_message());
}

#[tokio::test]
async fn drill_rejected_refresh_is_an_auth_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "error": "invalid_grant"
        })))
        .mount(&server)
        .await;

    let store = MemoryStore::default();
    store.save_refresh_token("default", "rt-stale").unwrap();
    let client = RefreshClient::new(&server.uri(), "tenant", "client-id");
    let err = refresh_session(&client, &store, "default").await.expect_err("drill: rejected");
    assert_eq!(err.user_message(), "authentication failed");
}
