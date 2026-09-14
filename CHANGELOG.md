# Changelog

All notable changes are documented here. Format follows Keep a Changelog; versioning
is SemVer with `MVP ≠ 1.0.0`.

## [0.2.0] — 2026-09-14 (MVP core gate)

### Added
- Everything in 0.1.0, plus: message search, message ops (edit/delete/reactions
  with contract C7–C9), silent refresh with rotation, polling sync loop
  (sweeper, due-tick, reconnect re-baselining, chaos drills), TUI read panes,
  composer/palette models, key map with scripted harness, terminal lifecycle,
  perf smoke tests (10k render ~17ms, order ~218µs debug), ARM64 CI job.
- Docs: as-built phases 2–6 slices, ADRs 005/006 finalized, Entra setup guide,
  accessibility sign-off (screen-reader pass explicitly pending).

### Known gaps (not claimed)
- Live tenant verification of login/refresh/Graph calls.
- Live terminal event loop and timer-driven sync.
- Screen-reader end-to-end pass.

## [Unreleased] — Phase 1: core domain and application architecture

### Added
- Rich domain: reactions, mentions, read markers, reply-thread builder,
  fail-safe presence parsing.
- App commands/events with pure reducers; connection events drive the Phase 0
  state machine.
- Shared provider contract suite (C1–C6) for mock now, Graph adapter later.
- Property tests: sanitizer safety/idempotence, ordering/dedupe idempotence,
  diff completeness, thread-builder termination.
- Cooperative shutdown signal, drop-guard teardown, `AppError::Shutdown`.
- Graph adapter drills: `GraphClient` chat listing with 429/`Retry-After`
  retries, `@odata.nextLink` paging, 401/malformed failure modes (mocked).
- Device-code login: `DeviceCodeClient` with slow_down/expiry handling, CLI
  `login` wired to OS-keyring persistence, BYO-app setup guide. Live
  verification pending tenant access.
- Silent refresh: `refresh_session` exchanges and rotates keyring tokens,
  fail-closed without a session.
- Message adapter: `list_messages`/`send_message` with sanitizing DTO mapping,
  presence read, provider-trait binding.
- Message search: ranked `chatMessage` hits with sanitized summaries.
- Message ops: provider-trait edit/delete/reactions (+ contract C7–C9) with
  mock and adapter implementations, all drilled.
- Quote-reply per documented schema; hosted-content fetch with size cap.
- Mention send with escaped `<at>` payload; hosted-content listing refs.
- Mention reads: normalized `Mention`s with approximate offsets, `@Name` body
  rendering.
- Read-only TUI panes with snapshot tests; keyboard map stubbed for composer phase.
- Polling sync: per-chat `Poller` with watermark scheduling, chaos drills for
  duplicate/reordered pages, shutdown fail-fast.
- Sync loop: chat sweeper, due-gated tick, reconnect watermark re-baselining,
  drop-heal recovery drills.
- Supervised sync: interval ticks with capped exponential backoff on failures,
  shutdown stop.
- Timer-driven `SyncLoop`: sweep + due poll per tick, shutdown fail-fast,
  throttle-then-retry convergence.
- Composer buffer + palette filter; optimistic-send confirm flow in the reducer;
  keyboard map and accessibility notes started.
- Bound key-to-command map (Vim + Emacs) with scripted event-loop harness.
- TUI runtime wiring: terminal acquire/restore lifecycle, `ReadView` assembly,
  action folding, `AppError::Terminal`.
- Live event loop over crossterm with quit/shutdown handling (TTY-only).

## [0.1.0] — 2026-09-14 (Phase 0 scaffold)

### Added
- Crate foundation: domain, app state machine, provider traits + mock, auth
  device-code helpers, keyring token store, graph retry classification, sanitizer,
  layered config, redacting telemetry, CLI skeleton, offline integration test.
- Repo hygiene: toolchain pin (1.89.0), fmt/clippy/deny configs, CI + security
  workflows, GPL-3.0 LICENSE/COPYING, docs skeleton.
