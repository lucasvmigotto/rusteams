# Phase 4 — Sending and Message Operations (core done)

## Objective

Send, edit (`PATCH`), soft-delete, quote-reply, reactions, mentions, attachments.

## Delivered (`feat/graph-ops`)

- Provider surface extended with demonstrated need: `update_message`,
  `delete_message`, `set_reaction`, `unset_reaction` (+ contract C7–C9).
- Mock implements all four against in-memory state (sanitized edits, reaction
  dedupe, not-found errors).
- Adapter: `PATCH` + re-read confirm, `softDelete`, `set/unsetReaction`
  (`204`-tolerant via shared `check_mutation`), all drilled against wiremock.

## Remaining scope

- `replyWithQuote`, mentions attach, hosted-content upload, file attachments
  via SharePoint/OneDrive handoff (exact flows verified at implementation time)
- Composer UX confirmations, tenant smoke tests, failure UX

## Non-Goals

Realtime event application (Phase 5 — done separately).

## Definition of Done

All mutation paths tested against mocks + tenant smoke tests; failure UX safe.
