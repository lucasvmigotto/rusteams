# Phase 3 — Chat Read Experience (in progress)

## Objective

Conversation list, message history, presence, and search over live Graph.

## Delivered so far (`feat/graph-messages`, `feat/graph-search`, `feat/graph-mentions-read`)

- `GraphClient::list_chats` (paged), `list_messages` (paged + ordered),
  `send_message`, `my_presence` — all DTO-mapped with sanitized bodies/topics.
- `search_messages`: `POST /search/query` over `chatMessage`, ranked hits with
  sanitized summaries; absent fields stay absent.
- Mention reads: `mentions` array parsed to normalized `Mention` (user id +
  display name, best-effort first-occurrence offsets, documented approximate);
  `<at>` tags render as `@Name`. Other HTML passes through — full HTML→text
  remains deferred work, stated honestly here.
- `ChatProvider` + `PresenceProvider` bound to the adapter; trait-object drills green.
- Retry discipline shared by all endpoints (429/5xx, capped `Retry-After`, 401 fast-fail).

## Remaining scope

- `$filter`/`$orderby`/lazy loading against live data; `lastMessagePreview` hydration
- HTML→sanitized-text rendering model; mentions/links/code/emoji mapping
- Detail hydration of search hits via `list_messages`/`chatmessage-get`
- Edit/delete/reactions (moved to Phase 4 with sending)
- Initial read-only TUI panes (next: `feat/tui-read`)

## Non-Goals

Push realtime (Phase 5).

## Definition of Done

Read paths usable against test tenant within throttling budgets; adapter tests green.
