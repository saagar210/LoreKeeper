#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEFAULT_CARGO_TARGET_DIR="${HOME}/.cache/lorekeeper/cargo-target"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-${LOREKEEPER_CARGO_TARGET_DIR:-$DEFAULT_CARGO_TARGET_DIR}}"

mkdir -p "$CARGO_TARGET_DIR"
cd "$ROOT_DIR"
./node_modules/.bin/tauri "$@"
