# Phase 3 — Chat Read Experience (in progress)

## Objective

Conversation list, message history, presence, and search over live Graph.

## Delivered so far (`feat/graph-messages`)

- `GraphClient::list_chats` (paged), `list_messages` (paged + ordered),
  `send_message`, `my_presence` — all DTO-mapped with sanitized bodies/topics.
- `ChatProvider` + `PresenceProvider` bound to the adapter; trait-object drills green.
- Retry discipline shared by all endpoints (429/5xx, capped `Retry-After`, 401 fast-fail).

## Remaining scope

- `$filter`/`$orderby`/lazy loading against live data; `lastMessagePreview` hydration
- HTML→sanitized-text rendering model; mentions/links/code/emoji mapping
- `/search/query` for `chatMessage` + detail hydration
- Edit/delete/reactions (moved to Phase 4 with sending)
- Initial read-only TUI panes (next: `feat/tui-read`)

## Non-Goals

Push realtime (Phase 5).

## Definition of Done

Read paths usable against test tenant within throttling budgets; adapter tests green.
