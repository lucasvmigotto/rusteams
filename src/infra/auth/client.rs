// Device-code HTTP client for Microsoft Entra ID (RFC 8628).
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! `POST {authority}/oauth2/v2.0/devicecode` to start, then poll
//! `POST {authority}/oauth2/v2.0/token` until the user approves, the flow
//! expires, or `max_polls` is reached. Interval honors server `slow_down`.

use super::device_code::{
    DeviceCodeResponse, PollOutcome, TokenResponse, classify_poll_error, default_scopes,
};
use crate::error::AppError;

/// HTTP driver for the device authorization grant.
#[derive(Debug, Clone)]
pub struct DeviceCodeClient {
    http: reqwest::Client,
    authority: String,
    client_id: String,
}

impl DeviceCodeClient {
    /// `authority_base` is e.g. `https://login.microsoftonline.com/{tenant}`.
    /// In tests it points at a wiremock server serving the same paths.
    pub fn new(authority_base: &str, tenant: &str, client_id: &str) -> Self {
        let authority = if authority_base.contains("login.microsoftonline.com") {
            format!("{}/{}", authority_base.trim_end_matches('/'), tenant)
        } else {
            authority_base.trim_end_matches('/').to_string()
        };
        Self { http: reqwest::Client::new(), authority, client_id: client_id.to_string() }
    }

    /// Scopes this client requests. Public so CLI/docs share one definition.
    pub fn scopes() -> String {
        default_scopes()
    }

    /// Start the flow; returns the code the user must enter in a browser.
    pub async fn request_code(&self, scope: &str) -> Result<DeviceCodeResponse, AppError> {
        let url = format!("{}/oauth2/v2.0/devicecode", self.authority);
        let resp = self
            .http
            .post(&url)
            .form(&[("client_id", self.client_id.as_str()), ("scope", scope)])
            .send()
            .await
            .map_err(|_| AppError::Auth("devicecode endpoint unreachable".into()))?;
        if !resp.status().is_success() {
            return Err(AppError::Auth("devicecode request rejected".into()));
        }
        resp.json().await.map_err(|_| AppError::Auth("bad devicecode response".into()))
    }

    /// Poll for tokens. `interval`/`slow_down` per RFC 8628; gives up after
    /// `max_polls` with an auth error (caller maps to user-facing retry/exit).
    pub async fn poll_for_token(
        &self,
        code: &DeviceCodeResponse,
        max_polls: u32,
    ) -> Result<TokenResponse, AppError> {
        let url = format!("{}/oauth2/v2.0/token", self.authority);
        let mut interval = code.interval.max(1);
        for _ in 0..max_polls.max(1) {
            tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
            let resp = self
                .http
                .post(&url)
                .form(&[
                    ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                    ("client_id", self.client_id.as_str()),
                    ("device_code", code.device_code.as_str()),
                ])
                .send()
                .await
                .map_err(|_| AppError::Auth("token endpoint unreachable".into()))?;
            if resp.status().is_success() {
                return resp.json().await.map_err(|_| AppError::Auth("bad token response".into()));
            }
            let body = resp.text().await.unwrap_or_default();
            match classify_poll_error(&body)? {
                PollOutcome::Pending => {}
                PollOutcome::SlowDown => interval += 5,
            }
        }
        Err(AppError::Auth("device flow timed out".into()))
    }
}
