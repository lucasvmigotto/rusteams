# ADR-006 — Realtime via polling (no webhooks in MVP)

- Status: accepted (partially implemented: `Poller` + `Watermarks`)
- Context: Graph change notifications need a public HTTPS webhook — impossible
  for a local TUI without a relay server.
- Decision: polling-first MVP (active-chat poll + list sweep, Retry-After aware,
  backoff+jitter, watermark re-baselining). Document weaker latency honestly.
  Webhook relay is a Phase 5+ option, not MVP.
- Implementation: `Poller::poll_once` full-fetches one chat, diffs via
  `diff_sync(complete)`, applies through the reducer; `Watermarks::is_due`
  schedules polls; `Shutdown` fail-fast stops loops. Chaos drills prove
  duplicate/reorder convergence. List sweeper + reconnect-loop binding remain.
- Consequences: must never claim push latency; must respect 429s.
