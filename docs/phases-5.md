# Phase 5 — Real-Time Event Architecture (Polling-First)

Strategic spec.

## Objective

Near-real-time sync without webhooks: poll + diff + reconnect state machine.

## Scope

- Active-chat poller + list sweeper honoring `Retry-After`, backoff+jitter caps
- `diff_sync` application, duplicate/out-of-order suppression, resync after gaps
- Subscription-renewal equivalent: re-baselining watermarks; optional webhook
  relay design doc (out of MVP scope)
- Chaos tests: drop/duplicate/reorder injections

## Non-Goals

Public webhook endpoint; background push.

## Definition of Done

Reconnect storms bounded; missed-event recovery demonstrated in tests.
