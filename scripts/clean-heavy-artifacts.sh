#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT_DIR"
rm -rf dist src-tauri/target node_modules/.vite

echo "Removed heavy build artifacts: dist, src-tauri/target, node_modules/.vite"
