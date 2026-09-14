// Silent session renewal via refresh token (OAuth2 refresh_token grant).
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Loads the stored refresh token, exchanges it at `{authority}/oauth2/v2.0/token`,
//! persists a rotated replacement, and returns the fresh access token (memory
//! only). No stored token or a rejected grant fails closed with login guidance.

use super::device_code::{TokenResponse, default_scopes};
use super::token_store::SecretStore;
use crate::error::AppError;

/// HTTP driver for the refresh_token grant.
#[derive(Debug, Clone)]
pub struct RefreshClient {
    http: reqwest::Client,
    authority: String,
    client_id: String,
}

impl RefreshClient {
    /// Same authority convention as [`super::client::DeviceCodeClient`].
    pub fn new(authority_base: &str, tenant: &str, client_id: &str) -> Self {
        let authority = if authority_base.contains("login.microsoftonline.com") {
            format!("{}/{}", authority_base.trim_end_matches('/'), tenant)
        } else {
            authority_base.trim_end_matches('/').to_string()
        };
        Self { http: reqwest::Client::new(), authority, client_id: client_id.to_string() }
    }

    /// Exchange one refresh token. Rejection maps to a fixed auth error —
    /// the grant response must never leak into logs or displays.
    pub async fn refresh(&self, refresh_token: &str) -> Result<TokenResponse, AppError> {
        let url = format!("{}/oauth2/v2.0/token", self.authority);
        let resp = self
            .http
            .post(&url)
            .form(&[
                ("grant_type", "refresh_token"),
                ("client_id", self.client_id.as_str()),
                ("refresh_token", refresh_token),
                ("scope", default_scopes().as_str()),
            ])
            .send()
            .await
            .map_err(|_| AppError::Auth("token endpoint unreachable".into()))?;
        if !resp.status().is_success() {
            return Err(AppError::Auth("authentication failed".into()));
        }
        resp.json().await.map_err(|_| AppError::Auth("bad token response".into()))
    }
}

/// Renew the session for `account`: load → exchange → persist rotation →
/// return fresh access token. Fails closed when no session exists.
pub async fn refresh_session(
    client: &RefreshClient,
    store: &dyn SecretStore,
    account: &str,
) -> Result<String, AppError> {
    let stored = store.load_refresh_token(account)?;
    let rt = stored.ok_or_else(|| AppError::Config("no session; run `rusteams login`".into()))?;
    let token = client.refresh(&rt).await?;
    if let Some(rotated) = token.refresh_token.clone() {
        store.save_refresh_token(account, &rotated)?;
    }
    Ok(token.access_token)
}
