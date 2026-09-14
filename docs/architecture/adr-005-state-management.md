# ADR-005 — State management (pure transitions + diff)

- Status: accepted
- Context: sync correctness must be testable without timers/network.
- Decision: pure `transition()` state machine and `order/dedupe/diff_sync`
  functions; jitter/durations injected by callers.
- Consequences: deterministic unit + property tests; runtime owns RNG/clocks.
