# Phase 5 — Real-Time Event Architecture (Polling-First, core done)

## Objective

Near-real-time sync without webhooks: poll + diff + reconnect state machine.

## Delivered (`feat/realtime-poll`, `feat/sync-loop`)

- `Poller`: per-chat full-fetch, `diff_sync` delta, reducer application, quiet
  steady state, `Shutdown` fail-fast. Chaos drills (duplicate + reversed pages)
  converge to ordered truth.
- `refresh_chats` sweeper (`ChatsLoaded` → `ChatsReplaced`).
- `poll_due_chats` tick: selected chat polls when watermark due; failures stay
  due (throttle-safe: no hammering, `Retry-After` honored in the adapter).
- `Watermarks::reset` + `note_connection`: entering `Reconnecting` re-baselines
  every chat; next complete-fetch diffs heal gaps.
- Drop-injection drill: lost message reported, then healed on next poll —
  missed-event recovery demonstrated in tests.
- `SyncLoop` timer core (`feat/sync-timer`): sweep + due-gated poll per tick,
  shutdown fail-fast without touching state, throttled ticks stay due and
  converge on retry. Drilled: sweep-only without selection, throttle→retry.
- Latency honesty: no push claims anywhere; see README.

## Remaining (live-loop integration, with the runtime)

- Running the tick on a timer inside the supervised task tree (`SyncLoop::tick`
  is the testable core; the sleeping supervisor binds `Shutdown` + `interval`).
  Backoff/jitter caps on the schedule (primitives exist: `backoff_delay`, `is_due`).

## Non-Goals

Public webhook endpoint; background push.

## Definition of Done

- [x] Missed-event recovery demonstrated in tests
- [x] Reconnect re-baselines watermarks (storm-bounding via due-gating)
- [ ] Live-loop timer integration (runtime stage)
