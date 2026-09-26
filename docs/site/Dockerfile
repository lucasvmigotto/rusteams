# syntax=docker/dockerfile:1
#
# rusteams documentation website image.
#
# Multi-stage: Bun builder compiles the static site, nginx serves it.
# Hash-based routing means the server only ever serves /index.html —
# no SPA fallback rewrites needed.
#
# Build from the repo root (version is read from Cargo.toml at build time):
#   docker build -f docs/site/Dockerfile . -t rusteams-website:test
#
# Run:
#   docker run --rm -p 8080:8080 rusteams-website:test

FROM dhi.io/bun:1-debian13-dev AS builder

WORKDIR /app

# Version source of truth lives outside docs/site/ — copy only this file.
COPY Cargo.toml ./
COPY docs/site/package.json docs/site/bun.lock ./docs/site/
RUN bun install --frozen-lockfile --cwd docs/site

COPY docs/site/ ./docs/site/

RUN bun run --cwd docs/site build

FROM dhi.io/nginx:1-alpine

LABEL org.opencontainers.image.title="rusteams-website" \
      org.opencontainers.image.description="Official documentation for rusteams: Teams for the terminal" \
      org.opencontainers.image.source="https://github.com/lucasvmigotto/rusteams" \
      org.opencontainers.image.licenses="GPL-3.0-or-later"

# Vite static output (single index.html + content-hashed assets).
COPY --from=builder /app/docs/site/dist /usr/share/nginx/html

COPY docs/site/nginx.conf /etc/nginx/nginx.conf

USER nginx

EXPOSE 8080
