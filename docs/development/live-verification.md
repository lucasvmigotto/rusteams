# Live verification runbooks (manual-only)

Some behaviors cannot be asserted headless: real TTYs, tenant auth, screen
readers. This page lists the exact manual runs that close those gaps, step by
step, so any reviewer can reproduce them. Check off items here as they pass;
do not claim them in README/CHANGELOG until then.

## A. Live TTY event loop

Prerequisites: a real terminal (not CI, not `docker exec` without `-t`).

```bash
cargo run
```

1. Sidebar lists chats; selection marker `●` moves with `j`/`k` and `Ctrl+n`/`Ctrl+p`.
2. `Enter` opens the selected chat; messages render sender, HH:MM, sanitized body.
3. `/` and `Ctrl+K` open the palette; typing filters; `Esc` closes.
4. Typing in the composer + `Enter` sends (mock-backed until live wiring).
5. `Ctrl+Q` quits; terminal is usable afterward (no raw mode, no alt screen).
6. Kill with SIGINT mid-render; terminal is usable afterward (panic-hook path:
   `kill -INT <pid>`, then type `echo ok` in the same shell).
7. Resize to 20×8 mid-session; app keeps rendering, no panic.

- [ ] A1–A5 pass on Linux x86_64
- [ ] A6 panic/SIGINT restore verified
- [ ] A7 narrow-terminal verified live

## B. Tenant verification (work/school account + BYO app)

Prerequisites: `docs/development/entra-setup.md` completed (client ID, consent).

1. `rusteams login` → device code → browser approval → `status` shows logged in.
2. `rusteams logout` → `status` shows logged out; keyring entry gone.
3. Expired session → command output guides back to `login` (no stack trace, no token).
4. `RUST_LOG=debug rusteams login 2>&1 | grep -iE 'bearer|eyJ|refresh'` → empty.
5. Graph smoke (once wired): list chats, list messages, send, edit, delete,
   react, quote, search — each against the test tenant, throttling observed
   (`429` → backoff, no hammering).

- [ ] B1–B3 pass against test tenant
- [ ] B4 log audit clean
- [ ] B5 Graph smoke passes within throttling budgets

## C. Screen-reader pass

1. Run the TUI under a terminal with screen-reader support; navigate the chat
   list and one conversation using keyboard only.
2. Confirm every meaning available visually is also available as text
   (selection, unread flags, connection state, errors).
3. Record terminal + reader versions and file deviations as issues.

- [ ] C1–C3 recorded, deviations tracked

## Rules

- Manual results are recorded here with date + environment, never as bare
  "verified" claims elsewhere.
- Any failure becomes a regression test if it can be reproduced headless;
  otherwise it becomes a tracked issue with repro steps.
