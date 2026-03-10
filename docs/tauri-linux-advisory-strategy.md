# Tauri Linux Advisory Strategy

This note captures the current follow-up strategy for the open Rust advisory affecting the Linux GTK/Tauri desktop stack.

## Current state

- Advisory: `RUSTSEC-2024-0429`
- Affected crate in lockfile: `glib 0.18.5`
- Fixed upstream line: `glib >= 0.20.0`
- Current direct app dependency: `tauri = "2"`
- Internal release scope today: macOS only

## What we confirmed

- LoreKeeper does not depend on `glib` directly.
- The advisory comes in through the Linux desktop GUI stack used by Tauri and Wry.
- A direct lockfile-only bump is not currently available on this dependency line.
- A dry-run attempt to force `glib 0.20.0` fails because the current GTK stack requires `glib ^0.18`.
- Current official upstream package metadata still points to the same GTK3-era line:
  - latest `wry` still declares Linux `gtk = "0.18"` and `webkit2gtk = "2.0.1"`
  - latest `tauri-runtime-wry` still declares Linux `webkit2gtk = "2.0"`
- RustSec still marks `glib >= 0.20.0` as the fixed line for `RUSTSEC-2024-0429`.

In plain language: this is not a quick Cargo update. It is an upstream stack issue.

## Why this does not block the current internal release

- The current internal release target is macOS only.
- The affected dependency path is in the Linux GTK/WebKit side of the desktop stack.
- The current internal release already documents this as a known follow-up rather than hiding it.

## Practical strategy

### Short term

- Keep the issue documented in internal release materials.
- Do not attempt a risky manual dependency override just to silence the alert.
- Treat Linux desktop packaging as non-release scope until the stack path is clarified.

### Medium term

- Monitor Tauri, Wry, and the GTK/WebKit dependency line for a compatible upgrade path that moves off `glib ^0.18`.
- Re-test with a safe dependency update once the surrounding stack supports it.
- Re-run Rust audit and Linux-oriented verification after that upgrade path is available.

### Long term

- If Linux becomes a true release target, make GTK/Tauri dependency health a release gate rather than a documented waiver.
- Revisit the bundle identifier and any platform-specific packaging issues as part of that broader desktop distribution pass.

## Suggested next execution batch

1. Review current Tauri and Wry release notes for a Linux stack upgrade path.
2. Test the smallest safe dependency move on a dedicated branch.
3. Run Rust verification and `cargo audit` after any dependency shift.
4. Decide whether Linux should remain deferred or move into active release scope.

## Decision rule

- If the stack still requires `glib ^0.18`, keep this as a documented upstream-bound follow-up.
- If a compatible stack path opens, promote this from strategy note to real remediation work.

## Source notes

- RustSec advisory: `RUSTSEC-2024-0429`
- Official crate metadata reviewed:
  - `docs.rs/crate/wry/latest/source/Cargo.toml`
  - `docs.rs/crate/tauri-runtime-wry/latest/source/Cargo.toml`
  - `docs.rs/crate/gtk/latest/source/Cargo.toml`
