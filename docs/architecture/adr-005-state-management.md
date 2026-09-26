# ADR-005 — State management (pure transitions + reducers + diff)

- Status: accepted (finalized in Phase 1)
- Context: sync correctness must be testable without timers/network.
- Decision: pure `transition()` state machine, `AppState::apply` reducers over
  `Command`/`Event`, and `order/dedupe/diff_sync/build_threads` functions;
  jitter/durations/RNG owned by callers; cooperative shutdown via shared
  `Shutdown` signal with `DropGuard` teardown and `AppError::Shutdown`.
- Alternatives: actor framework (rejected: premature dependency for MVP scale).
- Consequences: deterministic unit + property + contract tests; runtime owns
  clocks/RNG/tasks. `Shutdown::wait` busy-polls — revisit with `Notify` only
  if Phase 6 profiling justifies it.
