# Phase 6 — Notifications and UX Hardening (in progress)

## Objective

Full TUI: sidebar, conversation, composer, status bar, command palette, keyboard
(Vim + Emacs), accessibility, resilience, large-conversation performance.

## Delivered so far (`feat/tui-read`, `feat/tui-compose`)

- Read-only panes (`ReadView` → header/sidebar/conversation/status) over plain
  view data — no provider/network coupling; TestBackend snapshot tests.
- Empty states render hints; narrow terminals (20×8) render without panic.
- Connection state shown in header; status bar reserved for key hints.
- Composer buffer (Unicode-safe edit, blank-submit rejection) and
  case-insensitive palette filter as pure models with unit tests.
- Optimistic send flow: `MessageSent(temp)` → `MessageConfirmed(temp_id, real)`
  swaps the temp entry; missing temp appends (restart-safe).

## Keyboard map (`feat/tui-events`: mapper + harness bound)

| Keys | Action | State |
|---|---|---|
| `j/k`, `Ctrl+n`/`Ctrl+p` | Move in chat list | Bound (`map_key`, folded by `apply_action`) |
| `Enter` | Open selected chat | Bound (`map_key`) |
| `Ctrl+K`, `/` | Command palette / search | Bound (`map_key`, `Palette::filter`) |
| Type + `Enter` | Compose and send | Wired (`i` enters mode, type, `Enter` submits, `Esc` abandons; `handle_submit` optimistic cycle) |
| `Ctrl+Q` | Quit | Bound (terminates scripted stream) |
| Unmapped keys | Ignored, never panic | Tested |

## Live event loop (`feat/tui-event-loop`, `feat/tui-compose-send`)

- `fold_actions`: pure action fold, quit-terminating, tested.
- `run_live`: 100ms crossterm poll tick (shutdown/paint stay responsive),
  key-press filter, fold, re-render; acquire/restore + panic hook inside, so
  every exit path leaves the terminal usable.
- `LiveServices::step`: compose mode (`i`), printable capture, `Backspace`,
  `Enter` submit via `handle_submit` against the selected chat (no selection =
  stay composing), `Esc` abandon, `Ctrl+Q` quits from any mode, send errors
  recorded on `last_error` for future status rendering.
- Live TTY behavior is manual-verification only — never asserted in CI.

## Accessibility sign-off (MVP gate, re-verified)

- [x] Meaning never conveyed by color alone (selection `●` + text, snapshot-tested)
- [x] Empty/degraded states expose text hints (snapshot-tested)
- [x] Narrow-terminal (20×8) smoke test guards resize crashes
- [x] Keyboard map covers navigation/open/palette/quit (scripted-loop tested)
- [ ] Screen-reader pass — pending live terminal verification (not claimed)
- [ ] Full keyboard-only operation end-to-end — pending runtime loop

## Remaining (post-MVP)

- Live crossterm event feed, composer send wiring, notifications, benchmarks.

## Scope

- Keyboard map + help; command palette; notifications (terminal-native);
  connection status; resize/narrow-terminal behavior; benchmarks for 10k-message views

## Non-Goals

Themes (rendering stays theme-ready; spec only — see phases-7).

## Definition of Done

Keyboard-only operation; snapshot/state/keyboard tests; perf notes published.
