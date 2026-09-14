# Phase 0 — Discovery, Architecture, Threat Model and Foundation

## Objective

Establish requirements, API feasibility, architecture, threat model, repo
foundation, and TDD infrastructure before any live Graph integration.

## Motivation

A Teams client handles enterprise secrets and hostile remote content. Architecture,
auth, and terminal-security decisions must precede feature code.

## Scope

- API feasibility audit (Graph chat/presence/search/subscriptions, Entra device code)
- Architecture proposal + ADRs 001–008
- Threat model (`docs/security/threat-model.md`)
- Repo foundation: Cargo layout, toolchain, CI, license, docs, mock provider
- TDD infrastructure: offline unit/integration tests, sanitizer property tests (next)

## Non-Goals

Live Graph calls, real sign-in, TUI widgets, message cache persistence.

## Dependencies

Rust 1.89.0 in devcontainer; Microsoft Learn docs (no API keys needed for audit).

## User Stories

- As a developer I can `cargo test` fully offline and green.
- As a security reviewer I can read the threat model and sanitizer tests.
- As a user I learn personal accounts are unsupported before wasting setup time.

## Functional Requirements

- `cargo test` passes offline; `fmt`/`clippy` clean; MSRV CI job exists.
- Provider traits compile; mock covers list/send/throttle paths.

## Non-Functional Requirements

- MSRV 1.89.0; Linux x86_64 + ARM64 mandatory; minimal dependency graph.

## Architecture

Single crate, strict module boundaries (`lib.rs` docs). Workspace split deferred
to ADR-001. Layers: `tui -> app -> provider <- infra/graph`.

## Components

`sanitize` (security boundary), `error` (secret-free), `config` (CLI>env>file>
defaults), `domain` (order/dedupe/diff), `app::connection` (state machine +
backoff), `provider` traits + mock, `infra::auth` (device-code helpers,
keyring/memory stores), `infra::graph` (URLs + retry hints), `telemetry`
(redaction), `cli` skeleton.

## Interfaces

`ChatProvider`, `PresenceProvider`, `SecretStore` — all offline-testable.

## Data Models

`Chat`, `ChatMessage` (normalized; body is sanitized plain text).

## State Machines

`Connected/Degraded/Disconnected/Reconnecting/Syncing` with pure `transition()`
+ `backoff_delay()` (deterministic; jitter supplied by caller).

## API Integration

Polling-first (decision): active-chat poll + `lastMessagePreview` list sweep,
`$filter lastModifiedDateTime gt`, page size ≤50. Webhooks deferred (require
public HTTPS endpoint — incompatible with local TUI).

## Authentication

BYO Entra public-client app + device code flow; `offline_access` refresh via OS
keyring; access tokens memory-only. Work/school accounts only.

## Security Requirements

See `docs/security/threat-model.md`. Sanitizer + redaction tested; keyring
failures are `Security` errors; no secrets in config files.

## Testing Strategy

37 lib tests + 1 integration test, all offline. Next: proptest for sanitizer and
ordering, wiremock adapter tests in Phase 3.

## Acceptance Criteria

- [x] `cargo test` green offline (37 + 1)
- [x] `cargo fmt --check`, `clippy -D warnings` clean
- [x] CI + security workflows, GPL LICENSE/COPYING, docs skeleton
- [ ] Proptest + coverage gates (Phase 1 follow-up)

## Risks

- Graph subscription/webhook gap forces honest polling latency limits.
- Keyring availability varies per Linux desktop; `MemoryStore` escape hatch exists.

## Open Questions

- Exact search permission set (verify against live docs in Phase 3).
- Whether `Chat.ReadBasic` suffices for list-only fast path.

## Definition of Done

Gates 1–9 per DIRECTIVES §40 for foundation scope (no live integration yet).

## Future Evolution

Phase 1 deepens domain/app; Phase 2 live device-code login; Phase 3 read paths.
