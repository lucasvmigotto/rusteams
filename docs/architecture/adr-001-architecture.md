# ADR-001 — Layered single-crate architecture

- Status: accepted
- Context: MVP needs provider abstraction without workspace overhead.
- Decision: single crate with `tui -> app -> provider <- infra/graph` modules;
  TUI never touches Graph. Split into workspace crates only when compile times
  or team boundaries demand it.
- Alternatives: multi-crate workspace now (rejected: premature seams, slower start).
- Consequences: fast iteration; must police imports via code review.
- Security: provider seam keeps raw Graph DTOs out of TUI.
