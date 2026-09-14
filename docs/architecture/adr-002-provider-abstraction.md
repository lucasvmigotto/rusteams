# ADR-002 — Provider abstraction

- Status: accepted
- Context: core must not couple to Microsoft Graph; tests must run offline.
- Decision: `ChatProvider` + `PresenceProvider` async traits; `MockTeamsProvider`
  fixture-backed fake; contract tests shared by all providers.
- Alternatives: concrete Graph client everywhere (rejected: untestable).
- Consequences: new capabilities extend traits only with demonstrated need.
