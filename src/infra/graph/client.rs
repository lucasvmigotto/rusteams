// Minimal Microsoft Graph HTTP adapter (polling-first).
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Phase-0 contract: URL builders + error classification are pure and tested.
//! Live HTTP calls land in Phase 3 behind `TeamsProvider`.

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
