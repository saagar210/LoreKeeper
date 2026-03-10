#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CARGO_CACHE_DIR="${LOREKEEPER_CARGO_TARGET_DIR:-$HOME/.cache/lorekeeper/cargo-target}"

cd "$ROOT_DIR"
rm -rf dist src-tauri/target node_modules/.vite coverage node_modules
if [[ -n "$CARGO_CACHE_DIR" && "$CARGO_CACHE_DIR" != "/" ]]; then
  rm -rf "$CARGO_CACHE_DIR"
fi

echo "Removed reproducible local artifacts: dist, src-tauri/target, node_modules/.vite, coverage, node_modules, $CARGO_CACHE_DIR"
