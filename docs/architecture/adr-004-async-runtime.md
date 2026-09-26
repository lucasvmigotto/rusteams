# ADR-004 — Async runtime (tokio)

- Status: accepted
- Context: TUI must stay responsive during network/sync work.
- Decision: tokio multi-thread runtime; UI event loop isolated from network tasks;
  cancellation via tokens, graceful shutdown restores terminal.
- Alternatives: async-std/smol (rejected: smaller ecosystem for reqwest/Graph).
