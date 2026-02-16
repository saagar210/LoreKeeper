#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/lorekeeper-lean-dev.XXXXXX")"

cleanup() {
  rm -rf "$TMP_ROOT"
}

trap cleanup EXIT INT TERM

export CARGO_TARGET_DIR="$TMP_ROOT/cargo-target"
export VITE_CACHE_DIR="$TMP_ROOT/vite-cache"

mkdir -p "$CARGO_TARGET_DIR" "$VITE_CACHE_DIR"

echo "[lean-dev] Temporary Cargo target: $CARGO_TARGET_DIR"
echo "[lean-dev] Temporary Vite cache:   $VITE_CACHE_DIR"
echo "[lean-dev] Running: npm run tauri dev $*"

cd "$ROOT_DIR"
npm run tauri dev "$@"
