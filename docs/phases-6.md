# Phase 6 — Notifications and UX Hardening (in progress)

## Objective

Full TUI: sidebar, conversation, composer, status bar, command palette, keyboard
(Vim + Emacs), accessibility, resilience, large-conversation performance.

## Delivered so far (`feat/tui-read`)

- Read-only panes (`ReadView` → header/sidebar/conversation/status) over plain
  view data — no provider/network coupling; TestBackend snapshot tests.
- Empty states render hints; narrow terminals (20×8) render without panic.
- Connection state shown in header; status bar reserved for key hints.

## Keyboard map (stub — bindings land with `feat/tui-compose`)

| Keys | Action |
|---|---|
| `j/k`, `Ctrl+n`/`Ctrl+p` | Move in chat list (planned) |
| `Enter` | Open selected chat (planned) |
| `/`, `Ctrl+K` | Search / command palette (planned) |
| `Ctrl+Q` | Quit (planned) |

Vim + Emacs navigation both planned; no key is bound yet.

## Scope

- Keyboard map + help; command palette; notifications (terminal-native);
  connection status; resize/narrow-terminal behavior; benchmarks for 10k-message views

## Non-Goals

Themes (rendering stays theme-ready; spec only — see phases-7).

## Definition of Done

Keyboard-only operation; snapshot/state/keyboard tests; perf notes published.
