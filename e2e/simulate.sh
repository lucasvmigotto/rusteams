#!/usr/bin/env bash
# Full-usage simulation without real Teams: mock Entra + mock Graph behind a
# self-signed HTTPS proxy, real binary, file token store, pty for TUI paths.
# No secrets, no network beyond localhost. Fails loudly on first mismatch.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
E2E="$ROOT/e2e"
ENGINE="${CONTAINER_ENGINE:-$(command -v podman >/dev/null 2>&1 && echo podman || echo docker)}"
BIN="$ROOT/target/debug/rusteams"

pass=0
fail=0
check() { # check <name> <command...>
  local name="$1"; shift
  if "$@" >/tmp/sim-check.log 2>&1; then
    pass=$((pass + 1)); echo "ok: $name"
  else
    fail=$((fail + 1)); echo "FAIL: $name"; tail -5 /tmp/sim-check.log
  fi
}
expect_grep() { # expect_grep <name> <pattern> <command...>
  local name="$1" pattern="$2"; shift 2
  local out
  out="$("$@" 2>&1)" || true
  if printf '%s' "$out" | grep -q "$pattern"; then
    pass=$((pass + 1)); echo "ok: $name"
  else
    fail=$((fail + 1)); echo "FAIL: $name (missing '$pattern')"; printf '%s\n' "$out" | tail -5
  fi
}

echo "== certs =="
bash "$E2E/setup-certs.sh" >/dev/null

echo "== stack up =="
# Podman: one pod sharing localhost — nginx upstreams can never go stale
# (no container DNS, no cached IPs). Docker: compose with service DNS.
if [ "$ENGINE" = "podman" ]; then
  STACK_MODE=pod
  $ENGINE pod rm -f sim-pod >/dev/null 2>&1 || true
  $ENGINE pod create --name sim-pod \
    -p 127.0.0.1:8443:8443 \
    -p 127.0.0.1:8444:8444 >/dev/null
  $ENGINE run -d --pod sim-pod --name graph-mock \
    -v "$E2E/graph-openapi.yaml:/docs/openapi.yaml:ro" \
    docker.io/scalarapi/mock-server:0.2.55 >/dev/null
  $ENGINE run -d --pod sim-pod --name entra-mock \
    -v "$E2E/mock-oauth2:/app:ro" -w /app \
    dhi.io/bun:1-debian13-dev bun server.ts >/dev/null
  $ENGINE run -d --pod sim-pod --name sim-proxy \
    -v "$E2E/nginx-https.conf:/etc/nginx/nginx.conf:ro" \
    -v "$E2E/.certs:/etc/nginx/certs:ro" \
    dhi.io/nginx:1-alpine >/dev/null
else
  STACK_MODE=compose
  (cd "$E2E" && $ENGINE compose -f compose.yml up -d --quiet-pull 2>&1 | tail -2)
fi
echo "containers up"

ENTRA_URL="https://localhost:8443"
GRAPH_URL="https://localhost:8444"

echo "== wait for mocks (60s) =="
for _ in $(seq 1 60); do
  if curl -sk "$ENTRA_URL/health" 2>/dev/null | grep -q '"ok"' \
    && curl -sk "$GRAPH_URL/openapi.json" 2>/dev/null | grep -q '"openapi"'; then
    break
  fi
  sleep 1
done
curl -sk "$ENTRA_URL/health" | grep -q '"ok"' || { echo "entra mock never ready"; exit 1; }
curl -sk "$GRAPH_URL/openapi.json" | grep -q '"openapi"' || { echo "graph mock never ready"; exit 1; }
echo "mocks ready"

echo "== reset mock state =="
curl -sk -X POST "$ENTRA_URL/__reset" >/dev/null

echo "== build binary =="
cargo build --locked --bin rusteams 2>&1 | tail -2

export RUSTEAMS_CLIENT_ID=sim-client
export RUSTEAMS_TENANT_ID=sim-tenant
export RUSTEAMS_ENTRA_AUTHORITY="$ENTRA_URL"
export RUSTEAMS_GRAPH_BASE_URL="$GRAPH_URL"
export RUSTEAMS_CA_BUNDLE="$E2E/.certs/ca.pem"
export RUSTEAMS_TOKEN_FILE="$E2E/.sim-tokens.json"
rm -f "$RUSTEAMS_TOKEN_FILE"

echo "== CLI assertions =="
expect_grep "version matches Cargo.toml" "rusteams 0.2.0" "$BIN" version
expect_grep "status logged out" "logged_in: false" "$BIN" status
expect_grep "config shows sim tenant" "sim-tenant" "$BIN" config
expect_grep "doctor prints config path" "doctor: config file" "$BIN" doctor
expect_grep "login stores token" "Logged in: refresh token stored" "$BIN" login
expect_grep "status logged in" "logged_in: true" "$BIN" status
check "logout clears session" "$BIN" logout
expect_grep "status logged out again" "logged_in: false" "$BIN" status

echo "== pty paths (script(1)) =="
check "bare run under pty exits 0" script -qec "$BIN" /dev/null
expect_grep "pty login prints device message" "SIM-CODE" script -qec "$BIN login" /dev/null
# Live TUI event loop (run_live) is not wired to the CLI yet — bare run
# prints the launch hint. Full interactive TUI stays manual until then.

echo "== teardown =="
if [ "${STACK_MODE:-pod}" = "pod" ]; then
  $ENGINE pod rm -f sim-pod >/dev/null 2>&1 || true
else
  (cd "$E2E" && $ENGINE compose -f compose.yml down >/dev/null 2>&1 || true)
fi
rm -f "$RUSTEAMS_TOKEN_FILE"

echo "== result: $pass passed, $fail failed =="
[[ "$fail" -eq 0 ]]
