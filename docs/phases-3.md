# Phase 3 — Chat Read Experience

Strategic spec.

## Objective

Conversation list, message history, presence, and search over live Graph.

## Scope

- `GET /me/chats?$expand=lastMessagePreview`, `GET …/messages` with internal
  paging (`$top≤50`, `$filter`, `$orderby`); lazy loading, bounded memory
- HTML→sanitized-text rendering model; mentions/links/code/emoji mapping
- Presence read; `/search/query` for `chatMessage` + detail hydration
- wiremock adapter tests for 200/400/401/403/404/429/5xx + malformed JSON
- Initial read-only TUI panes

## Non-Goals

Sending/editing (Phase 4); push realtime (Phase 5).

## Definition of Done

Read paths usable against test tenant within throttling budgets; adapter tests green.
