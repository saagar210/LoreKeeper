#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root/src-tauri"

echo "LoreKeeper Linux GTK/Tauri stack trace"
echo "Repo: $repo_root"
echo

echo "== Direct app dependency =="
rg -n '^tauri\s*=\s*\{\s*version\s*=\s*"[^"]+"' Cargo.toml || true

echo
echo "== Locked glib package =="
python3 - <<'PY'
from pathlib import Path
text = Path('Cargo.lock').read_text()
needle = 'name = "glib"'
idx = text.find(needle)
if idx == -1:
    raise SystemExit('glib package not found in Cargo.lock')
start = text.rfind('[[package]]', 0, idx)
end = text.find('[[package]]', idx + 1)
print(text[start:end].strip())
PY

echo
echo "== Reverse dependency tree for glib (all targets) =="
cargo tree --target all -i glib || true

echo
echo "== Filtered GTK / WebKit / Wry chain =="
cargo tree --target all | rg '\b(glib|gtk|webkit2gtk|wry|tauri-runtime-wry|tauri)\b' || true

echo
echo "== Dry-run compatibility checks =="
echo '$ cargo update -p glib --dry-run'
cargo update -p glib --dry-run || true

echo
echo '$ cargo update -p glib --precise 0.20.0 --dry-run'
cargo update -p glib --precise 0.20.0 --dry-run || true
