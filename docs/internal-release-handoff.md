# LoreKeeper Internal Release Handoff

LoreKeeper is now in an internal release-ready state on `master`.

## Release status

- Current internal release ref: `master @ dd1e8fb`
- Promotion PR: `#8`
- Promotion source branch: `codex/fix/default-branch-risk-hardening`
- Rollback ref: `origin/master @ 990bae4`
- Platform target: macOS only
- Build posture: unsigned internal build

## What changed in this release line

- Lean dev mode and cleanup commands for lower-disk local development
- Canonical verification and CI alignment around the repo-defined command contract
- Desktop hardening for modules, settings, save/load flows, and custom themes
- Browser-level E2E coverage for the main validated user flows
- Repo-owned internal macOS build flow with manifest and checksums

## Teammate quick start

Use these commands when you need to verify or rebuild the internal candidate locally:

```bash
npm install
npm run version:check
npm run verify:internal-release
npm run release:internal:mac
```

Artifacts are written to:

```text
release-artifacts/internal/macos/<version>-<shortsha>/
```

For the full release workflow, tester charter, install steps, and known-issues list, use:

- `docs/internal-release-macos.md`
- `docs/internal-release-checklist.md`

## Current follow-ups

- One medium Rust dependency advisory remains open in the transitive GTK/Tauri desktop stack (`glib`). It is not directly fixable in this repo with a safe lockfile-only bump because the current Tauri GTK line requires `glib ^0.18`.
- The macOS build is unsigned, so Gatekeeper may require the documented first-launch workaround.
- Packaged Tauri GUI interaction still benefits from one human tester pass because the repo does not yet automate bundled-app UI interaction end to end.

## Team guidance

- Treat `master` as the current internal release line.
- Use the release docs above instead of relying on older task branches.
- If an issue appears in packaged mode, include the artifact version and commit SHA from `release-manifest.json` when reporting it.
