# Dependency security

`cargo audit` + `cargo deny check` in CI (`security.yml`, weekly schedule).
Lockfile committed. License policy: GPL-3.0-compatible only (`deny.toml`).
Prefer std/small crates over new deps; security-critical code (TLS via
rustls, keyring) uses mature audited libraries. Dependabot weekly for
cargo + actions.
