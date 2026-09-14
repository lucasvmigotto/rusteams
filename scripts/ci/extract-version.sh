#!/usr/bin/env bash
# Extract the rusteams package version via Cargo metadata (no text scraping).
# Requires: cargo, perl (JSON::PP is Perl core — no extra dependencies).
# Usage: extract-version.sh [--manifest PATH]
# Prints: X.Y.Z  (fails with a diagnostic on stderr otherwise)
set -euo pipefail

MANIFEST="Cargo.toml"
if [[ "${1:-}" == "--manifest" && -n "${2:-}" ]]; then
  MANIFEST="$2"
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "extract-version: cargo not found on PATH" >&2
  exit 1
fi

if ! command -v perl >/dev/null 2>&1; then
  echo "extract-version: perl not found on PATH" >&2
  exit 1
fi

VERSION="$(cargo metadata --no-deps --format-version 1 --manifest-path "$MANIFEST" 2>/dev/null \
  | perl -MJSON::PP -ne '
      my $meta = decode_json($_);
      my ($pkg) = grep { $_->{name} eq "rusteams" } @{$meta->{packages}};
      $pkg or die "no rusteams package in metadata\n";
      print $pkg->{version};')" || {
  echo "extract-version: Cargo.toml version could not be determined." >&2
  exit 1
}

if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$ ]]; then
  echo "extract-version: invalid version '$VERSION' (expected X.Y.Z)." >&2
  exit 1
fi

printf '%s\n' "$VERSION"
