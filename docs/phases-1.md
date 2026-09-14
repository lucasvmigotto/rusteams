# Phase 1 — Core Domain and Application Architecture

## Objective

Harden domain models, app commands/events, provider contracts, error taxonomy,
sync model, and concurrency design — all offline, no live network.

## Motivation

Phase 0 proved the testing pipeline. Phase 1 builds the behavioral core that
Phases 2–5 will drive: every state evolution must be deterministic and covered
before network code can hide bugs behind flakiness.

## Scope

- Rich `ChatMessage`: `Reaction`, `Mention`, `is_read`, reply threading via
  `build_threads` (orphans/self-replies/cycles cut to roots, oldest-first)
- `Presence` + fail-safe `parse_availability` (unknown Graph values → `Unknown`)
- `Command`/`Event` + pure `AppState::apply` reducers (select, load, send with
  empty-body rejection, receive upsert, sync-diff application, connection events)
- Shared provider contract suite (`tests/provider_contract.rs`, C1–C6) that the
  Phase 3 Graph adapter must satisfy unchanged
- `proptest` invariants (`tests/property.rs`): sanitizer output safety +
  idempotence, ordering totality/idempotence, dedupe idempotence, diff
  completeness, thread-builder termination
- Concurrency: `Shutdown` trigger/signal, `DropGuard` teardown, `AppError::Shutdown`
- Error taxonomy: no speculative variants — `Sync` deferred until the sync
  engine (Phase 5) demands it; only `Shutdown` added, justified by slice 6

## Non-Goals

Live network calls; TUI widgets; persistent cache (still ADR-007 ephemeral).

## Dependencies

`proptest` (already in dev-dependencies). No new runtime dependencies.

## User Stories

- As a contributor I can add a provider by satisfying the contract suite.
- As a reviewer I can trust reducers by reading pure functions + property tests.

## Functional Requirements

- `build_threads` terminates on adversarial input (cycles, orphans, duplicates).
- Reducers reject empty sends, upsert receives, scope loads per chat.
- Contract C1–C6 pass on mock; suite is provider-generic for Phase 3 reuse.

## Non-Functional Requirements

- Property suites run in milliseconds (pure functions only, 256 cases default).
- Zero network in all Phase 1 tests.

## Architecture

ADR-005 finalized: pure transitions + reducers + diff; jitter/durations/RNG owned
by callers; shutdown cooperative via shared atomic signal.

## Testing Strategy

48 → 53 lib tests, integration + contract + 6 property tests, all offline.

## Acceptance Criteria

- [x] Contract suite + property tests green
- [x] ADR-005 finalized
- [x] `fmt`/`clippy -D warnings` clean
- [x] No new runtime dependencies

## Risks

- Graph's real mention/reaction payloads may carry fields the normalized model
  drops — mapped explicitly in the Phase 3 adapter, never silently.
- `Shutdown::wait` busy-polls via `yield_now`; acceptable for MVP scale, revisit
  with `Notify` if profiling (Phase 6) shows waste.

## Open Questions

- Exact search permission set (still Phase 3).
- Whether `Chat.ReadBasic` suffices for list-only fast path (still Phase 3).

## Definition of Done

Contract suite + property tests green; ADR-005 finalized; docs reflect code.

## Future Evolution

Phase 2 drives live device-code login through these reducers; Phase 3 binds the
Graph adapter to the contract suite.
