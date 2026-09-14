# Phase 1 — Core Domain and Application Architecture

Strategic spec (to be detailed as Phase 0 follow-ups land).

## Objective

Harden domain models (reactions, mentions, receipts, presence), app commands/
events, provider contracts, error taxonomy, sync model, and concurrency design.

## Scope

- Rich `ChatMessage`: reactions, mentions, reply threading, hosted-content refs
- `SearchQuery/Filters/Result/Service` traits (Graph-backed in Phase 3)
- Contract tests so any future provider passes the same suite
- Proptest: ordering, dedupe, diff, sanitizer invariants
- Cancellation/shutdown design (tokio supervision tree sketch)

## Non-Goals

Live network calls; TUI widgets.

## Definition of Done

Contract suite + property tests green; ADR-005 (state) finalized.
