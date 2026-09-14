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

## Keyboard map (models ready; event-loop bindings pending)

| Keys | Action | State |
|---|---|---|
| `j/k`, `Ctrl+n`/`Ctrl+p` | Move in chat list | Planned |
| `Enter` | Open selected chat | Planned |
| `Ctrl+K`, `/` | Command palette / search | Model ready (`Palette::filter`) |
| Type + `Enter` | Compose and send | Model ready (`Composer`, `MessageConfirmed`) |
| `Ctrl+Q` | Quit | Planned |

## Accessibility notes (tracked, verified where testable)

- Meaning never conveyed by color alone (selection uses `●` marker + text).
- Empty/degraded states always expose text hints (snapshot-tested).
- Narrow-terminal (20×8) smoke test guards resize crashes.
- Screen-reader and full keyboard-only operation pending event-loop wiring.

## Scope

- Keyboard map + help; command palette; notifications (terminal-native);
  connection status; resize/narrow-terminal behavior; benchmarks for 10k-message views

## Non-Goals

Themes (rendering stays theme-ready; spec only — see phases-7).

## Definition of Done

Keyboard-only operation; snapshot/state/keyboard tests; perf notes published.
