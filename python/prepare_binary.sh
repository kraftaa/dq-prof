#!/usr/bin/env bash
set -euo pipefail
# Copy built release binary into the Python package.
ROOT=$(cd "$(dirname "$0")/.." && pwd)
BIN_SRC="$ROOT/dist/dq-prof"
BIN_DST="$ROOT/python/dq_prof/bin/dq-prof"
if [ ! -f "$BIN_SRC" ]; then
  echo "Expected built binary at $BIN_SRC. Run ./release.sh first." >&2
  exit 1
fi
mkdir -p "$ROOT/python/dq_prof/bin"
cp "$BIN_SRC" "$BIN_DST"
chmod +x "$BIN_DST"
echo "Copied binary to $BIN_DST"
