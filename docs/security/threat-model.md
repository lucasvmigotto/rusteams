# Threat model

## Trust boundaries

1. Microsoft Entra/Graph (trusted infra, untrusted content)
2. Provider seam (DTOs validated here, never passed raw to TUI)
3. Sanitizer (remote text -> safe presentation model)
4. Renderer (ratatui only sees sanitized data)
5. Local machine (keyring, config, logs)

## Attack surfaces

- Malicious message bodies (ANSI/OSC/C0 injection, title/clipboard abuse)
- Malformed API responses (huge payloads, bad Unicode, hostile filenames)
- Credential theft (logs, crash dumps, config files, shoulder-surfed device codes)
- Subscription/poll abuse (429-triggered hammering, reconnect loops)
- Supply chain (dependency compromise, license conflict)

## Assumptions

Work/school tenant enforces its own authz; TLS roots trustworthy; OS keyring
isolates secrets per user; terminal emulator honors sanitized output.

## Mitigations

Sanitizer + tests; secret-free errors/logs with redaction tests; keyring-only
refresh storage; Retry-After-respecting backoff; cargo audit/deny in CI;
least-privilege Graph scopes documented per capability.

## Residual risks

- Polling latency means deleted/edited messages linger briefly.
- Keyring-less Linux hosts fall back to memory-only (re-login each launch).
- Emulator bugs below our output layer are out of scope.
