# Phase 2 — Authentication (as built)

## Objective

Device-code login against a BYO public-client app. Implemented; live
verification against a test tenant is a user-side step (gated on tenant access).

## As-built design

- `DeviceCodeClient` (`src/infra/auth/client.rs`): `POST {authority}/oauth2/v2.0/devicecode`
  to start, `POST {authority}/oauth2/v2.0/token` to poll. Authority is
  `https://login.microsoftonline.com/{tenant}` in production, wiremock base in tests.
- Polling: honors server `interval`, `slow_down` (+5s), `authorization_pending`;
  terminal states (`declined`, `bad_verification_code`, `expired_token`) and
  `max_polls` exhaustion surface as fixed-message `AppError::Auth` (secret-free).
- CLI `login`: requires configured client-id (fail-closed guidance otherwise),
  prints the sanitized Entra message, polls (60 rounds), persists refresh token
  via `KeyringStore`; access tokens memory-only. `logout` clears the entry.
- Scopes (single definition, `default_scopes`): `openid profile offline_access
  User.Read Chat.Read ChatMessage.Send Chat.ReadWrite Presence.Read Presence.Read.All`.

## Scope delivered

- Device-code start + polling with interval/`slow_down`/expiry handling
- Keyring persistence, logout clearing, fail-closed missing-client-id UX
- Setup + verification guide (`docs/development/entra-setup.md`)
- Drills (`tests/device_flow.rs`): pending→token, slow_down tolerance, expiry error

## Non-Goals (unchanged)

- Silent refresh on expiry (uses stored refresh token — next: wire into Graph calls)
- Graph data calls beyond auth; TUI auth UX (CLI only so far)

## Testing

Unit (classifier, scopes) + wiremock drills, all offline. Live checklist in
`entra-setup.md` steps 3–4 — pending tenant access.

## Residual risks

- Live flow unverified in CI (no tenant); behavior beyond the documented RFC 8628
  + Entra error surface is unknown until first live login.
- Keyring-less Linux hosts fall back to re-login each launch (documented).

## Definition of Done

- [x] Code + offline drills green
- [x] Setup/verification docs written
- [ ] Live login against test tenant (user-side, gated)
- [ ] Secrets audit on live run (no token in logs/errors/disk-outside-keyring)
