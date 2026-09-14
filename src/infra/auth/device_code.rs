// Device Code Flow (OAuth2 device authorization grant, RFC 8628) against
// Microsoft Entra ID. Pure-protocol implementation over reqwest so the flow
// stays auditable and dependency-light.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct DeviceCodeRequest {
    pub client_id: String,
    pub scope: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TokenResponse {
    pub token_type: String,
    pub scope: String,
    pub expires_in: u64,
    pub access_token: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct TokenErrorBody {
    error: String,
}

/// Poll-state returned to the caller; the caller sleeps `interval` between polls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PollOutcome {
    Pending,
    SlowDown,
}

/// Classify a token-endpoint error body per RFC 8628 §3.5.
pub fn classify_poll_error(body: &str) -> Result<PollOutcome, AppError> {
    let parsed: TokenErrorBody =
        serde_json::from_str(body).map_err(|_| AppError::Auth("token endpoint error".into()))?;
    match parsed.error.as_str() {
        "authorization_pending" => Ok(PollOutcome::Pending),
        "slow_down" => Ok(PollOutcome::SlowDown),
        "authorization_declined" | "bad_verification_code" | "expired_token" => {
            Err(AppError::Auth("device flow ended".into()))
        }
        _ => Err(AppError::Auth("token endpoint error".into())),
    }
}

/// MVP delegated scopes for chat + presence + offline refresh.
pub fn default_scopes() -> String {
    [
        "openid",
        "profile",
        "offline_access",
        "User.Read",
        "Chat.Read",
        "ChatMessage.Send",
        "Chat.ReadWrite",
        "Presence.Read",
        "Presence.Read.All",
    ]
    .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_pending_and_slow_down() {
        assert_eq!(
            classify_poll_error(r#"{"error":"authorization_pending"}"#).unwrap(),
            PollOutcome::Pending
        );
        assert_eq!(classify_poll_error(r#"{"error":"slow_down"}"#).unwrap(), PollOutcome::SlowDown);
    }

    #[test]
    fn terminal_errors_are_auth_failures_without_detail() {
        let err = classify_poll_error(r#"{"error":"expired_token"}"#).unwrap_err();
        assert_eq!(err.user_message(), "authentication failed");
    }

    #[test]
    fn scopes_include_offline_access_for_refresh() {
        assert!(default_scopes().contains("offline_access"));
        assert!(default_scopes().contains("Chat.ReadWrite"));
    }
}
