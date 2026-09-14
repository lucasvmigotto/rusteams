# Security Policy

## Supported versions

Only the latest `dev`/`main` snapshot is supported during the pre-1.0 MVP phase.

## Reporting a vulnerability

Email the maintainers (see GitHub repo contacts) with:

1. Affected version/commit
2. Reproduction steps (no real credentials — use redacted fixtures)
3. Impact assessment

We aim to acknowledge within 72 hours. Do **not** open public issues for
vulnerabilities involving token leakage, terminal escape injection, or
credential-store bypass until a fix is available.

## Guarantees and non-guarantees

- Remote Teams content is treated as hostile and sanitized before rendering.
- Refresh tokens live in the OS keyring; access tokens are memory-only.
- No telemetry leaves the machine. Logs redact `Bearer` material.
- Personal Microsoft accounts are unsupported by the underlying APIs; the app
  must fail closed with an honest message rather than pretend to work.
- Terminal takeover is always reversible: acquire fails closed without a TTY,
  restore runs best-effort on exit, shutdown, and panics (panic hook), so the
  user's shell is never left in raw/alternate-screen mode by our code paths.
