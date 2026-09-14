# Phase 2 — Authentication

Strategic spec.

## Objective

Live Entra device-code login against a BYO public-client app.

## Scope

- `POST /devicecode` + `/token` polling with interval/`slow_down` handling
- Silent refresh via keyring refresh token; expiry fail-safe; logout clears
- Auth UX in CLI/TUI (code display, browser hint, timeout, decline paths)
- Security tests: no token in logs/errors, keyring failure modes

## Non-Goals

Graph data calls beyond `User.Read` smoke check.

## Definition of Done

Login/logout/status/doctor live against test tenant; secrets never touch disk
outside keyring.
