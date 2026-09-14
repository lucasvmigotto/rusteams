# ADR-003 — Authentication (device code + BYO app + keyring)

- Status: accepted
- Context: TUI has no browser; needs terminal-friendly Entra sign-in.
- Decision: OAuth2 device authorization grant against user-registered public-client
  app (`organizations` default); refresh token in OS keyring, access tokens
  memory-only; work/school accounts only (Graph limitation, documented).
- Alternatives: auth-code+PKCE with loopback (rejected for MVP: browser variance);
  shared multi-tenant app (deferred: maintainer-tenant burden).
- Security: no secrets in config/logs/errors; logout clears keyring.
