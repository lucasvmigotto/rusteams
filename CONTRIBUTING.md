# Contributing

1. **TDD is mandatory**: write the failing test first (`cargo test`), then the
   minimal implementation, then refactor.
2. No network in unit/integration tests — use `MockTeamsProvider` or `wiremock`.
3. Every PR must pass: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test`. All Rust work runs inside the devcontainer:
   `docker exec -w /workspaces/rusteams rusteams cargo …`.
4. Security: sanitize all remote text, never log secrets, keep permissions minimal.
5. Docs follow code — update `docs/phases-*.md` and ADRs with behavior changes.
6. License: contributions are GPL-3.0-or-later.
