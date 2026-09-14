# rusteams

**Terminal user interface (TUI) client for Microsoft Teams** — via official Microsoft Graph APIs.

> Status: **MVP core (unreleased, v0.2.0)** — domain, sync engine, adapter
> drills, auth flow, and TUI models are implemented and tested. Live terminal
> loop, live tenant verification, and realtime timer integration remain. This
> README documents only what exists today.

## What works today

- `rusteams login` via device-code flow (BYO Entra app, refresh token in OS
  keyring) + `config|status|doctor|logout|version` — see
  `docs/development/entra-setup.md`
- Rich domain (reactions, mentions, threads, presence), pure reducers, connection
  state machine with backoff
- Terminal-output sanitizer (ANSI/OSC/C0-stripping security boundary)
- `GraphClient` drills (chat/message listing + incremental poll + send + search +
  edit/delete/reactions + quote-reply + mentions + hosted fetch/list, 429
  retry, paging, failure modes) against mock servers, bound to the provider
  traits — no live data yet; mention reads render `@Name`, file references
  attach, HTML renders to styled segments (links, code) with plain fallback
- Offline suites: unit + contract + property tests, `MockTeamsProvider`
- Read-only TUI panes (sidebar, conversation, status bar) with TestBackend
  snapshot tests — not yet wired to live state
- Composer buffer + palette filter models, optimistic-send reducer flow
  (`MessageSent` → `MessageConfirmed`) — event-loop keybindings pending
- Compose mode wired (`i`, type, `Enter` submit with rollback, `Esc` abandon)
  through `LiveServices::step`; live TTY feed verified manually
- Visual-only notifications (bounded queue, unread flags, no bell by policy)
- Bound keyboard map (`j/k`, Emacs aliases, palette, quit) with scripted-loop
  harness — live terminal feed lands with the runtime loop
- Live event loop (`run_live`): crossterm feed, action fold, re-render, restore
  on every exit — needs a real TTY, verified manually
- Polling sync engine (sweeper, due-gated `SyncLoop` tick, supervised backoff
  loop, reconnect re-baselining, chaos-drilled) — near-real-time only; this
  client never claims push latency

## What does NOT work yet

Live chat listing, message send/receive, TUI, realtime polling. Anything claiming
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

The interactive TUI runtime is under construction: terminal acquire/restore,
view assembly, and key folding exist and are tested, but the live event loop
is not wired yet — `rusteams` without a subcommand still prints guidance.

## Documentation

- `docs/phases-0.md` … `docs/phases-7.md` — roadmap (Phase 0 detailed, later phases strategic)
- `docs/development/live-verification.md` — manual TTY/tenant/screen-reader runbooks
- `docs/architecture/` — ADRs
- `docs/security/` — threat model, auth, terminal security
- `SECURITY.md`, `CONTRIBUTING.md`, `CHANGELOG.md`

## Security

All remote text is sanitized before terminal rendering. Never paste tokens into issues.
See `SECURITY.md`.

## License

GNU GPL v3 or later — see `LICENSE`/`COPYING`.
