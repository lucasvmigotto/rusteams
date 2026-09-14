# Release pipeline

Production delivery for `rusteams`: quality gates → verified build → artifacts
(+ `SHASUMS`) → hardened dual-registry image → bare `X.Y.Z` Git tag. Everything
derives from `Cargo.toml` (`0.2.0` today); the `v` prefix appears only in prose
and release titles, never on tags or images.

## Workflows

| Workflow | Trigger | Publishes? |
|---|---|---|
| `ci.yml` | PRs, pushes to `main`/`dev` | Never — gates only |
| `security.yml` | pushes, PRs, weekly | Never — audit/deny |
| `release.yml` | push to `main` only | Yes (see below) |

## Job graph (`release.yml`)

```text
quality (no secrets)
  → build [x86_64 ubuntu, aarch64 arm-runner]
  → artifacts ─┬─→ docker (DOCKER_HUB_PAT + GITHUB_TOKEN) ─┐
               └────────────────────────────────────────────┴─→ tag (contents:write)
```

- **quality:** fmt, check, clippy `-D warnings`, full test suite, `cargo audit`, `cargo deny`.
- **build:** `extract-version.sh` (Cargo metadata, no scraping) → `cargo build
  --release --locked --target …` → `dist/` layout (`rusteams-{VERSION}-{TARGET}`,
  `VERSION`, `BUILD-METADATA`, `SHASUMS`) → binary self-check (`version`
  output contains Cargo version; `sha256sum -c SHASUMS`) → `upload-artifact`.
- **docker:** re-verifies per-leg `SHASUMS`, stages `dist/linux-{amd64,arm64}`,
  Buildx `linux/amd64,linux/arm64` packaging the **already-built** binaries
  (no recompilation), pushes `lucasvmigotto/rusteams:{VERSION}` and
  `ghcr.io/lucasvmigotto/rusteams:{VERSION}` (**no `latest`**), OCI labels,
  Trivy scan (warn-only SARIF artifact; fail-gate is future work).
- **tag:** re-extracts version, aborts if `refs/tags/{VERSION}` exists remotely
  (tags are never moved), creates annotated tag on the built commit, pushes
  only that ref.
- **concurrency:** `release-${{ github.ref }}`, no cancellation — rapid pushes
  serialize; the second run fails safe on the existing tag.

## Versioning and tags

- Source of truth: `Cargo.toml → package.version`.
- Git tags are bare (`0.2.0`); Docker tags are bare (`:0.2.0`); `v` lives only
  in prose and release titles.
- Consistency enforced: binary `--version` output must contain the Cargo
  version or the build fails.

## Required repository secrets

| Secret | Scope | Used by |
|---|---|---|
| `DOCKER_HUB_USERNAME` | Docker Hub account | `docker` job only |
| `DOCKER_HUB_PAT` | Docker Hub PAT (write) | `docker` job only |
| `GITHUB_TOKEN` | automatic (`packages: write`) | `docker` job (GHCR) only |

No secret is available to quality/build/tag jobs. Never enable shell tracing
around credentials; never `env`/`printenv` in credential-bearing jobs.

## Docker usage

Pull and run (a TUI needs an interactive TTY):

```bash
docker pull lucasvmigotto/rusteams:0.2.0
docker run -it --rm \
  -e RUSTEAMS_CLIENT_ID=... \
  -e RUSTEAMS_TENANT_ID=organizations \
  lucasvmigotto/rusteams:0.2.0
```

Or `ghcr.io/lucasvmigotto/rusteams:0.2.0` — same image, same tag.

Notes and limitations:

- The image runs as UID 65532 (non-root) with no shell; `docker exec` debugging
  inside it is intentionally impossible.
- No secret-service/keyring daemon exists in the image: login works, but the
  refresh token cannot persist — expect to re-authenticate each container run.
  Never bake credentials into images, volumes, or env files you share.
- Config file: mount one if needed
  (`-v ~/.config/rusteams:/home/nonroot/.config/rusteams:ro` — path depends on
  `dirs::config_dir`; verify with `doctor`).
- Verify downloads: `sha256sum -c SHASUMS` against the release `SHASUMS` file.

## Local reproduction

```bash
cargo fmt --all -- --check
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo build --release
bash scripts/ci/extract-version.sh
docker build --platform linux/amd64 --build-arg RUSTEAMS_VERSION="$(bash scripts/ci/extract-version.sh)" .
```

Recommended extras (not vendored): `shellcheck`, `actionlint`, `act`.

## Troubleshooting

| Symptom | Cause / fix |
|---|---|
| `Cargo.toml version could not be determined.` | cargo/perl missing or manifest unparseable; see script diagnostics |
| `Git tag X.Y.Z already exists.` | Version not bumped; bump `Cargo.toml`, never move the tag |
| `Required Docker Hub credentials are not configured.` | Set `DOCKER_HUB_USERNAME` + `DOCKER_HUB_PAT` repo secrets |
| `Release artifact was not generated.` | `dist/` step failed; inspect build job before the docker job |
| Trivy findings | Warn-only today; triage, then track the fail-gate follow-up |

## Failure and retry model

- Build failure → nothing published, no tag.
- Docker failure → no tag; retry reuses version-gated tag creation.
- Publish succeeded but tag push failed → retry detects the image/tag state
  and stops before destructive action (tags never force-pushed).
- Two rapid pushes → serialized by concurrency group; same-version retry fails
  safe on the existing tag.

## Future work

- GitHub Releases consuming the exact `release-dist-*` artifacts + `SHASUMS`.
- Trivy fail-gate, SBOM/provenance attestations, image signing (needs repo config).
