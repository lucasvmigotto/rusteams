# ADR-009 — Release image and tag conventions

- Status: accepted
- Context: production delivery needs a minimal runtime, deterministic tags,
  and dual-registry publishing without recompilation or secret leakage.
- Decisions:
  1. **Base image:** `gcr.io/distroless/cc-debian12:nonroot`, digest-pinned
     (`sha256:9dac0a…dd182f`). Verified: glibc (Rust binary links), CA bundle
     (`etc/ssl/certs/ca-certificates.crt`) present, default UID 65532, no
     shell. Rejected: `debian:slim` (larger, shell+package manager in final
     image), `alpine` (musl rebuild + cert management), `:latest` tags of
     anything (unjustified per policy).
  2. **Build-once packaging:** CI builds per-arch binaries; the Dockerfile only
     `COPY`s `dist/linux-$TARGETARCH/rusteams`. Trade-off documented: adding a
     third arch means extending the build matrix, not the Dockerfile.
  3. **Tags:** bare `X.Y.Z` on Git and both registries; no `latest`; `v`
     prefix lives only in prose/release titles.
  4. **Registries:** Docker Hub (`${{ github.repository }}`, username derived
     from the GitHub owner so only the PAT is secret) and GHCR
     (`ghcr.io/${{ github.repository }}`, `GITHUB_TOKEN` only).
  5. **Scan:** Trivy warn-only SARIF artifact; fail-gate deferred to keep the
     first pipeline shippable while findings are triaged.
- Consequences: images are non-debuggable by design; keyring-backed login
  cannot persist in containers (documented in `docs/release.md`).
