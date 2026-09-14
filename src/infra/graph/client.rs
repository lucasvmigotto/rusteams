// Minimal Microsoft Graph HTTP adapter (polling-first).
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Phase-0 contract: URL builders + error classification are pure and tested.
//! Live HTTP calls land in Phase 3 behind `TeamsProvider`.

use crate::domain::Chat;
use crate::error::AppError;
use serde::Deserialize;

/// Blocking-free Graph HTTP adapter. Bodies and topics are sanitized at the
/// boundary; auth failures never blind-retry (see [`classify_status`]).
#[derive(Debug, Clone)]
pub struct GraphClient {
    http: reqwest::Client,
    base: String,
    token: String,
}

#[derive(Debug, Deserialize)]
struct Envelope<T> {
    value: Vec<T>,
    #[serde(rename = "@odata.nextLink", default)]
    next_link: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatDto {
    id: String,
    #[serde(default)]
    topic: Option<String>,
}

impl GraphClient {
    pub fn new(base: &str, token: &str) -> Self {
        Self {
            http: reqwest::Client::new(),
            base: base.trim_end_matches('/').to_string(),
            token: token.to_string(),
        }
    }

    fn auth(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        req.bearer_auth(&self.token)
    }

    async fn get(&self, url: &str) -> Result<reqwest::Response, AppError> {
        // Bounded retries; 429/5xx honor Retry-After (capped), 401 fails fast.
        let mut attempts = 0;
        loop {
            let resp = self
                .auth(self.http.get(url))
                .send()
                .await
                .map_err(|e| AppError::Network(safe_network_message(&e)))?;
            let status = resp.status().as_u16();
            let retry_after = resp
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok());
            match classify_status(status, retry_after) {
                RetryHint::RetryAfter(secs) if attempts < 3 => {
                    attempts += 1;
                    tokio::time::sleep(std::time::Duration::from_secs(secs.min(5))).await;
                }
                RetryHint::RetryAfter(_) => {
                    return Err(AppError::Network(format!(
                        "graph HTTP {status} (retries exhausted)"
                    )));
                }
                RetryHint::NoRetry => return Ok(resp),
            }
        }
    }

    /// List the signed-in user's chats (`GET /me/chats`), following
    /// `@odata.nextLink` pages (bounded to 20 pages against runaway servers).
    pub async fn list_chats(&self) -> Result<Vec<Chat>, AppError> {
        let mut out = Vec::new();
        let mut url = Some(chats_url(&self.base));
        for _ in 0..20 {
            let next = match url {
                Some(u) => u,
                None => break,
            };
            let resp = self.get(&next).await?;
            let status = resp.status().as_u16();
            if status == 401 {
                return Err(AppError::Auth("graph rejected credentials".into()));
            }
            if !(200..300).contains(&status) {
                return Err(AppError::Network(format!("graph HTTP {status}")));
            }
            let env: Envelope<ChatDto> = resp
                .json()
                .await
                .map_err(|_| AppError::Provider("malformed graph response".into()))?;
            out.extend(env.value.into_iter().map(|c| Chat {
                id: c.id,
                topic: c.topic.map(|t| crate::sanitize::sanitize(&t)),
                last_message_preview: None,
                last_message_at: None,
                unread: false,
            }));
            url = env.next_link;
        }
        Ok(out)
    }
}

/// Network error text without URLs, tokens, or bodies.
fn safe_network_message(_: &reqwest::Error) -> String {
    "request failed".into()
}

/// Throttling / retry classification for Graph HTTP responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryHint {
    /// Do not retry (4xx other than 429/503-class).
    NoRetry,
    /// Retry after `retry_after_secs` (from `Retry-After` header or default).
    RetryAfter(u64),
}

pub fn classify_status(status: u16, retry_after: Option<u64>) -> RetryHint {
    match status {
        429 | 503 => RetryHint::RetryAfter(retry_after.unwrap_or(5).max(1)),
        408 | 500 | 502 | 504 => RetryHint::RetryAfter(2),
        _ if status >= 500 => RetryHint::RetryAfter(2),
        401 => RetryHint::NoRetry, // auth: re-login, never blind-retry
        _ => RetryHint::NoRetry,
    }
}

pub fn chats_url(base: &str) -> String {
    format!("{}/me/chats?$expand=lastMessagePreview", base.trim_end_matches('/'))
}

pub fn messages_url(base: &str, chat_id: &str) -> String {
    format!("{}/me/chats/{chat_id}/messages", base.trim_end_matches('/'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_limit_maps_to_retry_with_header() {
        assert_eq!(classify_status(429, Some(30)), RetryHint::RetryAfter(30));
        assert_eq!(classify_status(429, None), RetryHint::RetryAfter(5));
    }

    #[test]
    fn unauthorized_never_blind_retries() {
        assert_eq!(classify_status(401, None), RetryHint::NoRetry);
    }

    #[test]
    fn not_found_never_retries() {
        assert_eq!(classify_status(404, None), RetryHint::NoRetry);
    }

    #[test]
    fn url_builders_respect_custom_base() {
        assert_eq!(
            chats_url("https://graph.microsoft.com/v1.0"),
            "https://graph.microsoft.com/v1.0/me/chats?$expand=lastMessagePreview"
        );
        assert!(messages_url("https://x/", "c1").ends_with("/me/chats/c1/messages"));
    }
}
