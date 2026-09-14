# Changelog

All notable changes are documented here. Format follows Keep a Changelog; versioning
is SemVer with `MVP ≠ 1.0.0`.

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

## [0.1.0] — 2026-09-14 (Phase 0 scaffold)

### Added
- Crate foundation: domain, app state machine, provider traits + mock, auth
  device-code helpers, keyring token store, graph retry classification, sanitizer,
  layered config, redacting telemetry, CLI skeleton, offline integration test.
- Repo hygiene: toolchain pin (1.89.0), fmt/clippy/deny configs, CI + security
  workflows, GPL-3.0 LICENSE/COPYING, docs skeleton.
