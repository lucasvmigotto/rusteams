// Minimal Microsoft Graph HTTP adapter (polling-first).
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Phase-0 contract: URL builders + error classification are pure and tested.
//! Live HTTP calls land in Phase 3 behind `TeamsProvider`.

use crate::domain::{Chat, ChatMessage};
use crate::error::AppError;
use crate::provider::{ChatProvider, PresenceProvider};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
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
    #[serde(default)]
    #[serde(rename = "lastMessagePreview")]
    last_message_preview: Option<PreviewDto>,
}

#[derive(Debug, Deserialize)]
struct PreviewDto {
    #[serde(rename = "createdDateTime", default)]
    created: Option<String>,
    #[serde(default)]
    body: Option<ItemBodyDto>,
}

#[derive(Debug, Deserialize)]
struct HostedContentDto {
    id: String,
    #[serde(default)]
    #[serde(rename = "contentType")]
    content_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ItemBodyDto {
    #[serde(default)]
    content: String,
    #[serde(default)]
    #[serde(rename = "contentType")]
    content_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FromUserDto {
    #[serde(default)]
    #[serde(rename = "displayName")]
    display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FromDto {
    #[serde(default)]
    user: Option<FromUserDto>,
}

#[derive(Debug, Deserialize)]
struct MessageDto {
    id: String,
    #[serde(rename = "createdDateTime")]
    created: String,
    #[serde(rename = "lastModifiedDateTime", default)]
    modified: Option<String>,
    #[serde(default)]
    body: Option<ItemBodyDto>,
    #[serde(default)]
    from: Option<FromDto>,
    #[serde(default)]
    mentions: Vec<MentionDto>,
}

#[derive(Debug, Deserialize)]
struct MentionDto {
    #[serde(default)]
    #[serde(rename = "mentionText")]
    mention_text: Option<String>,
    #[serde(default)]
    mentioned: Option<MentionedDto>,
}

#[derive(Debug, Deserialize)]
struct MentionedDto {
    #[serde(default)]
    user: Option<MentionedUserDto>,
}

#[derive(Debug, Deserialize)]
struct MentionedUserDto {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    #[serde(rename = "displayName")]
    display_name: Option<String>,
}

fn parse_time(s: &str) -> Result<DateTime<Utc>, AppError> {
    DateTime::parse_from_rfc3339(s)
        .map(|t| t.with_timezone(&Utc))
        .map_err(|_| AppError::Provider("malformed timestamp".into()))
}

/// Split a preview into sanitized text + fail-soft timestamp. Previews are
/// best-effort UI hints, so a bad timestamp yields `None`, never an error.
fn preview_parts(p: PreviewDto) -> (Option<String>, Option<DateTime<Utc>>) {
    let text = p.body.map(|b| crate::sanitize::sanitize(&b.content));
    let at = p.created.and_then(|s| parse_time(&s).ok());
    (text, at)
}

/// Incremental message URL: `$filter lastModifiedDateTime gt {since}` +
/// `$top`, query-encoded. `since` renders RFC 3339 (`+00:00` suffix).
pub fn messages_since_url(base: &str, chat_id: &str, since: DateTime<Utc>, top: u8) -> String {
    let base_url = messages_url(base, chat_id);
    let filter = format!("lastModifiedDateTime gt {}", since.to_rfc3339());
    reqwest::Url::parse_with_params(&base_url, &[("$top", top.to_string()), ("$filter", filter)])
        .map(|u| u.to_string())
        .unwrap_or(base_url)
}

fn map_message(chat_id: &str, dto: MessageDto) -> Result<ChatMessage, AppError> {
    let created = parse_time(&dto.created)?;
    let modified = match dto.modified {
        Some(s) => parse_time(&s)?,
        None => created,
    };
    let (raw, is_html) = dto
        .body
        .map(|b| (b.content, b.content_type.as_deref() == Some("html")))
        .unwrap_or_default();
    let with_ats = super::html::render_at_tags(&raw);
    let mut segments = if is_html {
        super::html::render_html_body(&with_ats)
    } else {
        vec![crate::domain::RichSegment::Text(with_ats.clone())]
    };
    sanitize_segments(&mut segments);
    let body = segments.iter().map(|s| s.plain()).collect::<Vec<_>>().join("");
    let sender = dto
        .from
        .and_then(|f| f.user)
        .and_then(|u| u.display_name)
        .unwrap_or_else(|| "unknown".into());
    let mentions = dto
        .mentions
        .into_iter()
        .map(|m| {
            let user = m.mentioned.and_then(|d| d.user);
            let display_name = m
                .mention_text
                .or_else(|| user.as_ref().and_then(|u| u.display_name.clone()))
                .unwrap_or_default();
            // Best-effort offset: first occurrence of the display name in the
            // rendered body. Graph exposes no offsets; 0 when absent.
            let offset = body.find(&display_name).unwrap_or(0);
            crate::domain::Mention {
                user_id: user.and_then(|u| u.id),
                length: display_name.len(),
                display_name,
                offset,
            }
        })
        .collect();
    Ok(ChatMessage {
        id: dto.id,
        chat_id: chat_id.into(),
        created,
        modified,
        sender,
        body,
        reply_to_id: None,
        reactions: vec![],
        mentions,
        segments,
        is_read: false,
    })
}

/// Sanitize every segment text in place (structure preserved).
fn sanitize_segments(segments: &mut [crate::domain::RichSegment]) {
    for seg in segments {
        match seg {
            crate::domain::RichSegment::Text(t) => *t = crate::sanitize::sanitize(t),
            crate::domain::RichSegment::Link { text, url } => {
                *text = crate::sanitize::sanitize(text);
                *url = crate::sanitize::sanitize(url);
            }
            crate::domain::RichSegment::CodeBlock { lines } => {
                for line in lines {
                    *line = crate::sanitize::sanitize(line);
                }
            }
        }
    }
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
            out.extend(env.value.into_iter().map(|c| {
                let (preview_text, preview_at) =
                    c.last_message_preview.map(preview_parts).unwrap_or((None, None));
                Chat {
                    id: c.id,
                    topic: c.topic.map(|t| crate::sanitize::sanitize(&t)),
                    last_message_preview: preview_text,
                    last_message_at: preview_at,
                    // Read receipts need the Graph viewpoint, which list responses
                    // don't carry — unread stays provider-managed (mock) or false.
                    unread: false,
                }
            }));
            url = env.next_link;
        }
        Ok(out)
    }

    /// Fetch one message page: checked response mapped to DTOs + next link.
    async fn fetch_message_page(
        &self,
        url: &str,
    ) -> Result<(Vec<MessageDto>, Option<String>), AppError> {
        let resp = self.get(url).await?;
        let status = resp.status().as_u16();
        if status == 401 {
            return Err(AppError::Auth("graph rejected credentials".into()));
        }
        if !(200..300).contains(&status) {
            return Err(AppError::Network(format!("graph HTTP {status}")));
        }
        let env: Envelope<MessageDto> =
            resp.json().await.map_err(|_| AppError::Provider("malformed graph response".into()))?;
        Ok((env.value, env.next_link))
    }

    /// List messages in a chat (`GET /me/chats/{id}/messages`), following
    /// `@odata.nextLink` pages like [`GraphClient::list_chats`].
    pub async fn list_messages(&self, chat_id: &str) -> Result<Vec<ChatMessage>, AppError> {
        let mut out = Vec::new();
        let mut url = Some(messages_url(&self.base, chat_id));
        for _ in 0..20 {
            let next = match url {
                Some(u) => u,
                None => break,
            };
            let (dtos, link) = self.fetch_message_page(&next).await?;
            for dto in dtos {
                out.push(map_message(chat_id, dto)?);
            }
            url = link;
        }
        crate::domain::order_messages(&mut out);
        Ok(out)
    }

    /// Incremental poll: messages modified after `since`, page size `top`.
    /// Drives watermark-based polling without full-history refetch.
    pub async fn list_messages_since(
        &self,
        chat_id: &str,
        since: DateTime<Utc>,
        top: u8,
    ) -> Result<Vec<ChatMessage>, AppError> {
        let mut out = Vec::new();
        let mut url = Some(messages_since_url(&self.base, chat_id, since, top));
        for _ in 0..20 {
            let next = match url {
                Some(u) => u,
                None => break,
            };
            let (dtos, link) = self.fetch_message_page(&next).await?;
            for dto in dtos {
                out.push(map_message(chat_id, dto)?);
            }
            url = link;
        }
        crate::domain::order_messages(&mut out);
        Ok(out)
    }

    /// Hydrate one message (`GET …/messages/{id}`) — e.g. search-hit detail.
    pub async fn get_message(
        &self,
        chat_id: &str,
        message_id: &str,
    ) -> Result<ChatMessage, AppError> {
        let url = self.message_url(chat_id, message_id);
        let resp = self.get(&url).await?;
        let status = resp.status().as_u16();
        if status == 401 {
            return Err(AppError::Auth("graph rejected credentials".into()));
        }
        if !(200..300).contains(&status) {
            return Err(AppError::Network(format!("graph HTTP {status}")));
        }
        let dto: MessageDto =
            resp.json().await.map_err(|_| AppError::Provider("malformed graph response".into()))?;
        map_message(chat_id, dto)
    }

    /// Send a message (`POST /me/chats/{id}/messages`). The body is sent as-is;
    /// the returned message is mapped through the same sanitizing path as reads.
    pub async fn send_message(&self, chat_id: &str, body: &str) -> Result<ChatMessage, AppError> {
        let url = messages_url(&self.base, chat_id);
        let resp = self
            .auth(self.http.post(&url))
            .json(&serde_json::json!({
                "body": { "contentType": "text", "content": body }
            }))
            .send()
            .await
            .map_err(|e| AppError::Network(safe_network_message(&e)))?;
        let status = resp.status().as_u16();
        if status == 401 {
            return Err(AppError::Auth("graph rejected credentials".into()));
        }
        if !(200..300).contains(&status) {
            return Err(AppError::Network(format!("graph HTTP {status}")));
        }
        let dto: MessageDto =
            resp.json().await.map_err(|_| AppError::Provider("malformed graph response".into()))?;
        map_message(chat_id, dto)
    }

    fn message_url(&self, chat_id: &str, message_id: &str) -> String {
        format!("{}/messages/{message_id}", messages_url(&self.base, chat_id))
    }

    /// Check a mutation response: 401 → auth, 2xx → ok, else network error.
    /// Consumes the response; 204 bodies are never parsed.
    async fn check_mutation(resp: reqwest::Response) -> Result<(), AppError> {
        let status = resp.status().as_u16();
        if status == 401 {
            return Err(AppError::Auth("graph rejected credentials".into()));
        }
        if !(200..300).contains(&status) {
            return Err(AppError::Network(format!("graph HTTP {status}")));
        }
        Ok(())
    }

    /// Edit a message (`PATCH …/messages/{id}`), then re-read it for the
    /// confirmed state (delegated updates return `204 No Content`).
    pub async fn update_message(
        &self,
        chat_id: &str,
        message_id: &str,
        body: &str,
    ) -> Result<ChatMessage, AppError> {
        let url = self.message_url(chat_id, message_id);
        let resp = self
            .auth(self.http.patch(&url))
            .json(&serde_json::json!({
                "body": { "contentType": "text", "content": body }
            }))
            .send()
            .await
            .map_err(|e| AppError::Network(safe_network_message(&e)))?;
        Self::check_mutation(resp).await?;
        let dto: MessageDto = self
            .get(&url)
            .await?
            .json()
            .await
            .map_err(|_| AppError::Provider("malformed graph response".into()))?;
        map_message(chat_id, dto)
    }

    /// Soft-delete a message (`POST …/messages/{id}/softDelete`).
    pub async fn delete_message(&self, chat_id: &str, message_id: &str) -> Result<(), AppError> {
        let url = format!("{}/softDelete", self.message_url(chat_id, message_id));
        let resp = self
            .auth(self.http.post(&url))
            .send()
            .await
            .map_err(|e| AppError::Network(safe_network_message(&e)))?;
        Self::check_mutation(resp).await
    }

    /// Attach a reaction (`POST …/messages/{id}/setReaction`).
    pub async fn set_reaction(
        &self,
        chat_id: &str,
        message_id: &str,
        kind: &str,
    ) -> Result<(), AppError> {
        self.react(chat_id, message_id, kind, "setReaction").await
    }

    /// Detach a reaction (`POST …/messages/{id}/unsetReaction`).
    pub async fn unset_reaction(
        &self,
        chat_id: &str,
        message_id: &str,
        kind: &str,
    ) -> Result<(), AppError> {
        self.react(chat_id, message_id, kind, "unsetReaction").await
    }

    async fn react(
        &self,
        chat_id: &str,
        message_id: &str,
        kind: &str,
        action: &str,
    ) -> Result<(), AppError> {
        let url = format!("{}/{action}", self.message_url(chat_id, message_id));
        let resp = self
            .auth(self.http.post(&url))
            .json(&serde_json::json!({ "reactionType": kind }))
            .send()
            .await
            .map_err(|e| AppError::Network(safe_network_message(&e)))?;
        Self::check_mutation(resp).await
    }

    /// Reply with quote (`POST …/messages/replyWithQuote`, schema per
    /// Microsoft Learn `chatmessage-replywithquote`). Returns the created message.
    pub async fn reply_with_quote(
        &self,
        chat_id: &str,
        message_ids: &[String],
        body: &str,
    ) -> Result<ChatMessage, AppError> {
        let url = format!("{}/messages/replyWithQuote", messages_url(&self.base, chat_id));
        let resp = self
            .auth(self.http.post(&url))
            .json(&serde_json::json!({
                "messageIds": message_ids,
                "replyMessage": { "body": { "contentType": "text", "content": body } }
            }))
            .send()
            .await
            .map_err(|e| AppError::Network(safe_network_message(&e)))?;
        let status = resp.status().as_u16();
        if status == 401 {
            return Err(AppError::Auth("graph rejected credentials".into()));
        }
        if !(200..300).contains(&status) {
            return Err(AppError::Network(format!("graph HTTP {status}")));
        }
        let dto: MessageDto =
            resp.json().await.map_err(|_| AppError::Provider("malformed graph response".into()))?;
        map_message(chat_id, dto)
    }

    /// Escape text embedded in outgoing HTML (`<at>` tags). Directory data is
    /// not trusted inside markup.
    fn escape_html(s: &str) -> String {
        s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
    }

    /// Mention a user (`POST …/messages`, HTML body + mentions array per the
    /// documented `<at id>` schema). Display names are HTML-escaped.
    pub async fn send_mention(
        &self,
        chat_id: &str,
        text_before: &str,
        user_id: &str,
        display_name: &str,
    ) -> Result<ChatMessage, AppError> {
        let safe_name = Self::escape_html(display_name);
        let content = format!("{}<at id=\"0\">{safe_name}</at>", Self::escape_html(text_before));
        let url = messages_url(&self.base, chat_id);
        let resp = self
            .auth(self.http.post(&url))
            .json(&serde_json::json!({
                "body": { "contentType": "html", "content": content },
                "mentions": [{
                    "id": 0,
                    "mentionText": display_name,
                    "mentioned": {
                        "user": {
                            "id": user_id,
                            "displayName": display_name,
                            "userIdentityType": "aadUser"
                        }
                    }
                }]
            }))
            .send()
            .await
            .map_err(|e| AppError::Network(safe_network_message(&e)))?;
        let status = resp.status().as_u16();
        if status == 401 {
            return Err(AppError::Auth("graph rejected credentials".into()));
        }
        if !(200..300).contains(&status) {
            return Err(AppError::Network(format!("graph HTTP {status}")));
        }
        let dto: MessageDto =
            resp.json().await.map_err(|_| AppError::Provider("malformed graph response".into()))?;
        map_message(chat_id, dto)
    }

    /// Send a file reference (`POST …/messages`, HTML body + reference
    /// attachment per the documented schema). The file must already live in
    /// SharePoint/OneDrive: pass the driveItem eTag GUID (webDavUrl flow) or
    /// any GUID (share-link flow) as `attachment_id`. Uploading bytes is
    /// OneDrive API territory — explicitly out of scope here.
    pub async fn send_file_reference(
        &self,
        chat_id: &str,
        text_before: &str,
        attachment_id: &str,
        file_name: &str,
        content_url: &str,
    ) -> Result<ChatMessage, AppError> {
        let content = format!(
            "{}<attachment id=\"{}\"></attachment>",
            Self::escape_html(text_before),
            attachment_id
        );
        let url = messages_url(&self.base, chat_id);
        let resp = self
            .auth(self.http.post(&url))
            .json(&serde_json::json!({
                "body": { "contentType": "html", "content": content },
                "attachments": [{
                    "id": attachment_id,
                    "contentType": "reference",
                    "contentUrl": content_url,
                    "name": file_name,
                }]
            }))
            .send()
            .await
            .map_err(|e| AppError::Network(safe_network_message(&e)))?;
        let status = resp.status().as_u16();
        if status == 401 {
            return Err(AppError::Auth("graph rejected credentials".into()));
        }
        if !(200..300).contains(&status) {
            return Err(AppError::Network(format!("graph HTTP {status}")));
        }
        let dto: MessageDto =
            resp.json().await.map_err(|_| AppError::Provider("malformed graph response".into()))?;
        map_message(chat_id, dto)
    }

    /// List hosted-content refs for a message (`GET …/hostedContents`).
    /// Returns `(content_id, content_type)` pairs; fetch bytes separately.
    pub async fn list_hosted_contents(
        &self,
        chat_id: &str,
        message_id: &str,
    ) -> Result<Vec<(String, String)>, AppError> {
        let url = format!("{}/hostedContents", self.message_url(chat_id, message_id));
        let resp = self.get(&url).await?;
        let status = resp.status().as_u16();
        if status == 401 {
            return Err(AppError::Auth("graph rejected credentials".into()));
        }
        if !(200..300).contains(&status) {
            return Err(AppError::Network(format!("graph HTTP {status}")));
        }
        let env: Envelope<HostedContentDto> =
            resp.json().await.map_err(|_| AppError::Provider("malformed graph response".into()))?;
        Ok(env
            .value
            .into_iter()
            .map(|h| (h.id, h.content_type.unwrap_or_else(|| "application/octet-stream".into())))
            .collect())
    }

    /// Fetch hosted message content bytes (images, code snippets).
    /// Capped at 5 MiB against hostile payloads; content type is informational.
    pub async fn get_hosted_content(
        &self,
        chat_id: &str,
        message_id: &str,
        content_id: &str,
    ) -> Result<HostedContent, AppError> {
        let url =
            format!("{}/hostedContents/{content_id}/$value", self.message_url(chat_id, message_id));
        let resp = self.get(&url).await?;
        let status = resp.status().as_u16();
        if status == 401 {
            return Err(AppError::Auth("graph rejected credentials".into()));
        }
        if !(200..300).contains(&status) {
            return Err(AppError::Network(format!("graph HTTP {status}")));
        }
        let content_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();
        let bytes = resp
            .bytes()
            .await
            .map_err(|_| AppError::Provider("malformed graph response".into()))?;
        if bytes.len() > 5 * 1024 * 1024 {
            return Err(AppError::Security("hosted content exceeds size cap".into()));
        }
        Ok(HostedContent { content_type, bytes: bytes.to_vec() })
    }

    /// Search the signed-in user's messages (`POST /search/query`,
    /// `entityTypes: ["chatMessage"]`). Returns ranked hits; use
    /// `list_messages`/`chatmessage-get` to hydrate full bodies.
    pub async fn search_messages(&self, query: &str, size: u8) -> Result<Vec<SearchHit>, AppError> {
        let url = format!("{}/search/query", self.base);
        let resp = self
            .auth(self.http.post(&url))
            .json(&serde_json::json!({
                "requests": [{
                    "entityTypes": ["chatMessage"],
                    "query": { "queryString": query },
                    "from": 0,
                    "size": size,
                }]
            }))
            .send()
            .await
            .map_err(|e| AppError::Network(safe_network_message(&e)))?;
        let status = resp.status().as_u16();
        if status == 401 {
            return Err(AppError::Auth("graph rejected credentials".into()));
        }
        if !(200..300).contains(&status) {
            return Err(AppError::Network(format!("graph HTTP {status}")));
        }
        let body: SearchResponseDto =
            resp.json().await.map_err(|_| AppError::Provider("malformed graph response".into()))?;
        Ok(body
            .value
            .into_iter()
            .flat_map(|v| v.hits_containers)
            .flat_map(|c| c.hits)
            .filter_map(|h| {
                let resource = h.resource?;
                Some(SearchHit {
                    message_id: resource.id.or(h.hit_id)?,
                    chat_id: resource.chat_id,
                    subject: resource.subject,
                    summary: crate::sanitize::sanitize(h.summary.as_deref().unwrap_or("")),
                })
            })
            .collect())
    }
}

/// Fetched hosted content: raw bytes plus the server-declared type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostedContent {
    pub content_type: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct PresenceDto {
    #[serde(default)]
    availability: Option<String>,
}

/// One ranked message-search hit. Summaries are sanitized; absent fields stay absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchHit {
    pub message_id: String,
    pub chat_id: Option<String>,
    pub subject: Option<String>,
    pub summary: String,
}

#[derive(Debug, Deserialize)]
struct SearchHitDto {
    #[serde(rename = "hitId", default)]
    hit_id: Option<String>,
    #[serde(default)]
    summary: Option<String>,
    #[serde(default)]
    resource: Option<SearchResourceDto>,
}

#[derive(Debug, Deserialize)]
struct SearchResourceDto {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    #[serde(rename = "chatId")]
    chat_id: Option<String>,
    #[serde(default)]
    subject: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HitsContainerDto {
    #[serde(default)]
    hits: Vec<SearchHitDto>,
}

#[derive(Debug, Deserialize)]
struct SearchResponseDto {
    #[serde(default)]
    value: Vec<SearchValueDto>,
}

#[derive(Debug, Deserialize)]
struct SearchValueDto {
    #[serde(rename = "hitsContainers", default)]
    hits_containers: Vec<HitsContainerDto>,
}

#[async_trait]
impl ChatProvider for GraphClient {
    async fn list_chats(&self) -> Result<Vec<Chat>, AppError> {
        GraphClient::list_chats(self).await
    }

    async fn list_messages(&self, chat_id: &str) -> Result<Vec<ChatMessage>, AppError> {
        GraphClient::list_messages(self, chat_id).await
    }

    async fn send_message(&self, chat_id: &str, body: &str) -> Result<ChatMessage, AppError> {
        GraphClient::send_message(self, chat_id, body).await
    }

    async fn update_message(
        &self,
        chat_id: &str,
        message_id: &str,
        body: &str,
    ) -> Result<ChatMessage, AppError> {
        GraphClient::update_message(self, chat_id, message_id, body).await
    }

    async fn delete_message(&self, chat_id: &str, message_id: &str) -> Result<(), AppError> {
        GraphClient::delete_message(self, chat_id, message_id).await
    }

    async fn set_reaction(
        &self,
        chat_id: &str,
        message_id: &str,
        kind: &str,
    ) -> Result<(), AppError> {
        GraphClient::set_reaction(self, chat_id, message_id, kind).await
    }

    async fn unset_reaction(
        &self,
        chat_id: &str,
        message_id: &str,
        kind: &str,
    ) -> Result<(), AppError> {
        GraphClient::unset_reaction(self, chat_id, message_id, kind).await
    }
}

#[async_trait]
impl PresenceProvider for GraphClient {
    async fn my_presence(&self) -> Result<String, AppError> {
        let url = format!("{}/me/presence", self.base);
        let resp = self.get(&url).await?;
        if resp.status().as_u16() == 401 {
            return Err(AppError::Auth("graph rejected credentials".into()));
        }
        let dto: PresenceDto =
            resp.json().await.map_err(|_| AppError::Provider("malformed graph response".into()))?;
        Ok(dto.availability.unwrap_or_else(|| "Unknown".into()))
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
