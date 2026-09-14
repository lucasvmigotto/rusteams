# Phase 8 — Release Engineering (CI/CD foundation)

## Objective

Production pipeline on `main`: quality gates → verified release build →
artifacts + `SHASUMS` → hardened dual-registry image → bare `X.Y.Z` tag,
with the application version visible in CLI and TUI.

## Motivation

Manual releases don't scale and invite secret/version drift. This phase makes
every `main` commit releasable by construction while keeping PRs publish-free.

## Scope

- `release.yml`: quality → build matrix → artifacts → docker → tag
- `scripts/ci/extract-version.sh` (Cargo metadata, fixture-verified)
- Hardened `Dockerfile` + `.dockerignore`, OCI labels, Trivy warn-only
- TUI status-bar version + version surface tests
- `docs/release.md`, ADR-009, Docker usage documentation

## Non-Goals

GitHub Releases automation, image signing, SBOM/provenance (documented future
work); `latest` tag; `v`-prefixed tags.

## Architecture

See `docs/release.md` (job graph, secret isolation, failure/retry model) and
ADR-009 (image + tag conventions).

## Testing Strategy

- App: version surface tests (shape + status-bar presence, no hard-coding)
- CI: actionlint-clean workflows, fixture-driven script validation
  (prerelease, wrong package, malformed manifest), local `docker build` +
  `version`/`--version` run, secret-grep audit

## Acceptance Criteria

- [x] `main` push triggers; PRs publish nothing
- [x] Gates (fmt/check/clippy/test/audit/deny) precede all publishing
- [x] Release binaries per target with `SHASUMS`, uploaded as artifacts
- [x] Dual-registry image from verified artifacts, OCI labels, Trivy addressed
- [x] Bare `X.Y.Z` tag, never moved, duplicate fails safe
- [x] Version visible in CLI + TUI, derived from Cargo metadata, tested
- [x] No Microsoft credentials required in CI

## Risks

- ARM runner availability on GitHub-hosted infra (job fails visibly, blocks tag)
- Trivy warn-only may normalize ignored findings — fail-gate follow-up tracked
- First live `main` run is unverified until merged (documented prerequisites:
  `DOCKER_HUB_*` secrets, branch protection compatible)

## Future Evolution

GitHub Releases from `release-dist-*`, Trivy fail-gate, SBOM/provenance,
signing, `replyWithQuote`-era version bumps stay manual via `Cargo.toml`.
