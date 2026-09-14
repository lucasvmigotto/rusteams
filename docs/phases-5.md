# Phase 5 — Real-Time Event Architecture (Polling-First, in progress)

## Objective

Near-real-time sync without webhooks: poll + diff + reconnect state machine.

## Delivered so far (`feat/realtime-poll`)

- `Poller`: per-chat full-fetch, `diff_sync` delta, reducer application, quiet
  steady state, `Shutdown` fail-fast. Chaos drills (duplicate + reversed pages)
  converge to ordered truth.
- `Watermarks`: last-success scheduling (`is_due`), i.e. re-baselining primitive.
- Latency honesty: no push claims anywhere; see README.

## Remaining scope

- List sweeper honoring `Retry-After` + backoff/jitter caps in the poll loop
- Reconnect-loop binding (`transition()` + poller + `Watermarks` reset)
- Drop-injection chaos (message loss mid-page) and missed-event recovery demo

## Non-Goals

Public webhook endpoint; background push.

## Definition of Done

Reconnect storms bounded; missed-event recovery demonstrated in tests.
