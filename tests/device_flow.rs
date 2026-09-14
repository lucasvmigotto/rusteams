// Device-code login drills against a fake Entra server (wiremock).
// Proves the polling state machine without credentials or network.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use rusteams::infra::auth::{DeviceCodeClient, default_scopes};
use wiremock::{matchers::method, Mock, MockServer, ResponseTemplate};

fn device_code_body(server_uri: &str) -> serde_json::Value {
    serde_json::json!({
        "device_code": "dc-123",
        "user_code": "ABCD-EFGH",
        "verification_uri": "https://microsoft.com/devicelogin",
        "expires_in": 900,
        "interval": 0,
        "message": format!("Go to {} and enter ABCD-EFGH", server_uri)
    })
}

#[tokio::test]
async fn drill_device_flow_polls_until_token() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(device_code_body(&server.uri())))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "error": "authorization_pending"
        })))
        .up_to_n_times(2)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "token_type": "Bearer",
            "scope": default_scopes(),
            "expires_in": 3600,
            "access_token": "at-123",
            "refresh_token": "rt-123"
        })))
        .mount(&server)
        .await;

    let client = DeviceCodeClient::new(&server.uri(), "tenant", "client-id");
    let code = client.request_code(&default_scopes()).await.expect("drill: code issued");
    assert_eq!(code.user_code, "ABCD-EFGH");
    let token = client.poll_for_token(&code, 5).await.expect("drill: token issued");
    assert_eq!(token.refresh_token.as_deref(), Some("rt-123"));
}
