#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT_DIR"
rm -rf dist src-tauri/target node_modules/.vite coverage node_modules

echo "Removed reproducible local artifacts: dist, src-tauri/target, node_modules/.vite, coverage, node_modules"
