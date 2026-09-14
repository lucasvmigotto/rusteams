# Phase 4 — Sending and Message Operations

Strategic spec.

## Objective

Send, edit (`PATCH`), soft-delete, quote-reply, reactions, mentions, attachments.

## Scope

- `POST /chats/{id}/messages`, `replyWithQuote`, `setReaction`/`unsetReaction`
- Composer UX, optimistic send with rollback, edit/delete confirmations
- Hosted-content upload path; file attachments via SharePoint/OneDrive handoff
  (exact flow verified against docs at implementation time)

## Non-Goals

Realtime event application (Phase 5).

## Definition of Done

All mutation paths tested against mocks + tenant smoke tests; failure UX safe.
