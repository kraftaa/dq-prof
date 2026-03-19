#!/usr/bin/env bash
set -euo pipefail

# Build release binary
cargo build --release

# Copy to dist
mkdir -p dist
cp target/release/dq-prof dist/

# Print usage
cat <<'USAGE'
Built dist/dq-prof
Distribute by attaching dist/dq-prof as a release asset.
On user machines:
  chmod +x dq-prof
  ./dq-prof --help
USAGE
