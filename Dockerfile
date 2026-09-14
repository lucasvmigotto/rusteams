# syntax=docker/dockerfile:1
#
# rusteams production image.
#
# The release binary is built by CI (release.yml) and staged per architecture
# under dist/linux-<TARGETARCH>/ — this image only packages it. No toolchain,
# no shell, no package manager in the final image.
#
# Base: distroless cc-debian12 (glibc for the Rust binary, CA bundle present,
# non-root UID 65532 by default). Digest-pinned; see ADR-009.
#
# Build (CI assembles dist/ first):
#   docker buildx build --platform linux/amd64,linux/arm64 \
#     --build-arg RUSTEAMS_VERSION=$(./scripts/ci/extract-version.sh) .
#
# Run (TUI needs a TTY):
#   docker run -it --rm \
#     -e RUSTEAMS_CLIENT_ID=... -e RUSTEAMS_TENANT_ID=organizations \
#     lucasvmigotto/rusteams:0.2.0

ARG RUSTEAMS_VERSION=0.0.0

FROM gcr.io/distroless/cc-debian12@sha256:9dac0a79194e45a7da0158a9c6da57b217585af0786db3845d1f0ec1a0dd182f

ARG TARGETARCH
ARG RUSTEAMS_VERSION

LABEL org.opencontainers.image.title="rusteams" \
      org.opencontainers.image.description="Terminal client for Microsoft Teams" \
      org.opencontainers.image.version="${RUSTEAMS_VERSION}" \
      org.opencontainers.image.source="https://github.com/lucasvmigotto/rusteams" \
      org.opencontainers.image.licenses="GPL-3.0-or-later"

# Explicit non-root (distroless :nonroot already defaults to 65532).
USER 65532:65532

COPY dist/linux-${TARGETARCH}/rusteams /usr/local/bin/rusteams

ENTRYPOINT ["/usr/local/bin/rusteams"]
CMD ["--help"]
