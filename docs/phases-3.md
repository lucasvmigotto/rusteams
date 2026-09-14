# Phase 3 — Chat Read Experience (in progress)

## Objective

Conversation list, message history, presence, and search over live Graph.

## Delivered so far (`feat/graph-messages`, `feat/graph-search`, `feat/graph-mentions-read`, `feat/graph-query`, `feat/graph-htmltext`)

- `GraphClient::list_chats` (paged), `list_messages` (paged + ordered),
  `send_message`, `my_presence` — all DTO-mapped with sanitized bodies/topics.
- `search_messages`: `POST /search/query` over `chatMessage`, ranked hits with
  sanitized summaries; absent fields stay absent.
- Mention reads: `mentions` array parsed to normalized `Mention` (user id +
  display name, best-effort first-occurrence offsets, documented approximate);
  `<at>` tags render as `@Name`. Other HTML passes through — full HTML→text
  remains deferred work, stated honestly here.
- Incremental query: `messages_since_url` (`$filter lastModifiedDateTime gt` +
  `$top`, encoded), `list_messages_since`, single-message `get_message` for
  search-hit hydration, `lastMessagePreview` hydration onto chats (fail-soft
  timestamps).
- HTML rendering (`feat/graph-htmltext`): hand-rolled scanner (no new deps)
  producing `RichSegment`s — links as `text (url)`, code blocks as line lists,
  lists/paragraphs break lines, malformed markup falls back to text. Domain
  `body` stays plain (segments alongside); TUI maps segments to styled lines
  (underlined links, bold indented code) with plain fallback.
- `ChatProvider` + `PresenceProvider` bound to the adapter; trait-object drills green.
- Retry discipline shared by all endpoints (429/5xx, capped `Retry-After`, 401 fast-fail).

## Remaining scope

- `$orderby`/lazy loading against live data; unread via Graph viewpoint
  (list responses carry no read state — documented, deferred)

## Non-Goals

Push realtime (Phase 5).

## Definition of Done

Read paths usable against test tenant within throttling budgets; adapter tests green.
