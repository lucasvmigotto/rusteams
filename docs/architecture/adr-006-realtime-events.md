# ADR-006 — Realtime via polling (no webhooks in MVP)

- Status: accepted
- Context: Graph change notifications need a public HTTPS webhook — impossible
  for a local TUI without a relay server.
- Decision: polling-first MVP (active-chat poll + list sweep, Retry-After aware,
  backoff+jitter, watermark re-baselining). Document weaker latency honestly.
  Webhook relay is a Phase 5+ option, not MVP.
- Consequences: must never claim push latency; must respect 429s.
