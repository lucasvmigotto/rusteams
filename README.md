# rusteams

**Terminal user interface (TUI) client for Microsoft Teams** — via official Microsoft Graph APIs.

> Status: **Phase 0 scaffold (v0.1.0)**. Chat read/send, auth, and real-time sync are
> under construction. This README documents only what exists today.

## What works today

- `rusteams config|status|doctor|login|logout|version` CLI skeleton
- Core domain (ordering, dedupe, sync diff), connection state machine with backoff
- Terminal-output sanitizer (ANSI/OSC/C0-stripping security boundary)
- Device-code protocol helpers + OS-keyring refresh-token store abstraction
- Graph URL builders + 429/5xx retry classification
- `MockTeamsProvider` for fully offline tests

## What does NOT work yet

Real Microsoft sign-in, live chat listing, message send/receive, TUI. Anything claiming
otherwise is a bug — see `docs/phases-*.md` for the roadmap.

## Prerequisites

- Linux x86_64/ARM64, Rust 1.89.0 (`rust-toolchain.toml`), Docker for the devcontainer
- A **work or school** Microsoft 365 account (personal accounts are not supported by
  the Teams chat APIs) and your own Entra ID public-client app registration (BYO-app model)

## Quick start

```bash
docker exec -w /workspaces/rusteams rusteams cargo test
docker exec -w /workspaces/rusteams rusteams cargo run -- config
```

## Documentation

- `docs/phases-0.md` … `docs/phases-7.md` — roadmap (Phase 0 detailed, later phases strategic)
- `docs/architecture/` — ADRs
- `docs/security/` — threat model, auth, terminal security
- `SECURITY.md`, `CONTRIBUTING.md`, `CHANGELOG.md`

## Security

All remote text is sanitized before terminal rendering. Never paste tokens into issues.
See `SECURITY.md`.

## License

GNU GPL v3 or later — see `LICENSE`/`COPYING`.
