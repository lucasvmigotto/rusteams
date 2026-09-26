# Full-usage simulation without real Teams (`e2e/`)

Runs the real `rusteams` binary against a local simulation stack — no Teams
account, no network beyond localhost, no secrets.

## Stack

| Piece | What | Trust |
|---|---|---|
| `graph-openapi.yaml` | Trimmed OpenAPI 3.1 of the Graph surface rusteams calls, served by `docker.io/scalarapi/mock-server:0.2.55` | Shapes mirror `src/infra/graph/client.rs` DTOs |
| `mock-oauth2/server.ts` | Device-code + refresh flow (Bun, zero deps): pending×N → success, plus `slow_down`/`expired` modes | No real Entra, no browser |
| `nginx-https.conf` | TLS terminator (self-signed sim CA) — MS SSO flows are HTTPS-only, hence the proxy | `e2e/.certs/` is gitignored |
| `simulate.sh` | Orchestrates: certs → stack → CLI assertions → pty checks → teardown | Fails loudly on first mismatch |

Two engine paths, auto-selected by `CONTAINER_ENGINE` (or podman-then-docker
detection): **podman** runs the three containers in one pod sharing
`localhost` (upstreams never go stale); **docker** uses `compose.yml` with
compose DNS (`nginx-https.docker.conf`). CI pins `CONTAINER_ENGINE=docker`.

## Run it

```bash
bash e2e/simulate.sh
```

Environment used (all local, all fake): `RUSTEAMS_CLIENT_ID=sim-client`,
`RUSTEAMS_TENANT_ID=sim-tenant`,
`RUSTEAMS_ENTRA_AUTHORITY=https://localhost:8443`,
`RUSTEAMS_GRAPH_BASE_URL=https://localhost:8444`,
`RUSTEAMS_CA_BUNDLE=e2e/.certs/ca.pem`,
`RUSTEAMS_TOKEN_FILE=e2e/.sim-tokens.json` (removed afterwards).
Localhost with per-service ports keeps system DNS out of the picture; the
sim certificate carries both localhost and the `.mock.local` SANs.

## CI auth

The Bun and nginx bases come from `dhi.io` (Docker Hardened Images), which is
a **separate registry from Docker Hub** — a Docker Hub login does not
authorize `dhi.io` pulls. `e2e-sim.yml` therefore logs in to both, using
`secrets.DOCKER_HUB_PAT` (the account needs DHI entitlement). Pin the engine
to Docker so those credentials reach the client doing the pull.

## Constraints (read before extending)

- **Stateless mocks.** Scalar returns canned responses per request: assert
  send-confirmation and list rendering, never send→appears-in-list.
  Retry/429 paths stay in the wiremock unit drills.
- **File token store is sim-only.** `RUSTEAMS_TOKEN_FILE` selects a `0600`
  JSON store and warn-logs; production always uses the OS keyring.
- **Live TUI loop (`run_live`) is not wired to the CLI yet** — the pty
  checks cover the binary's printed paths (bare run, login message).
  Full interactive TUI stays manual per `docs/development/live-verification.md`.
