#!/usr/bin/env bash
# Extract the `pls` binary from a maturin bin-wheel and write it to a zip
# containing a single top-level `pls` entry, ready as a GitHub release asset.
#
# Usage: extract_binary.sh <path-to-wheel> <output-zip>
set -euo pipefail

wheel="$1"
out="$2"

# Resolve the output to an absolute path before we cd around.
out="$(cd "$(dirname "$out")" && pwd)/$(basename "$out")"

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

unzip -q "$wheel" -d "$work"

# `grep -c .` exits 1 when count is zero; `|| true` keeps pipefail happy.
match_count="$(find "$work" -type f -path '*.data/scripts/pls' | grep -c . || true)"
if [[ "$match_count" -eq 0 ]]; then
	echo "error: pls binary not found in $wheel" >&2
	exit 1
elif [[ "$match_count" -gt 1 ]]; then
	echo "error: multiple pls binaries found in $wheel (expected exactly one)" >&2
	exit 1
fi
bin="$(find "$work" -type f -path '*.data/scripts/pls')"
chmod +x "$bin"

# `-j` junks the directory so the archive holds just `pls` at its root.
rm -f "$out"
zip -q -j "$out" "$bin"
echo "Wrote $out"
