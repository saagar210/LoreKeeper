# LoreKeeper Internal Release Announcement

LoreKeeper is now available as the current internal macOS release line on `master`.

## Release summary

- Release ref: `master @ dd1e8fb`
- Promotion PR: `#8`
- Platform: macOS only
- Build posture: unsigned internal build
- Rollback ref: `origin/master @ 990bae4`

## What is included

- lower-disk development workflow with explicit cleanup commands
- canonical verification and CI alignment around the repo-defined command contract
- desktop hardening across modules, save/load flows, settings, and custom themes
- browser-level E2E coverage for the validated core user flows
- repo-owned internal macOS packaging with manifest and checksum output

## How teammates should use it

1. Treat `master` as the current internal release line.
2. Use the release docs instead of older task branches:
   - `docs/internal-release-handoff.md`
   - `docs/internal-release-macos.md`
   - `docs/internal-release-checklist.md`
3. When rebuilding locally, run:

```bash
npm install
npm run version:check
npm run verify:internal-release
npm run release:internal:mac
```

## Known limits for this internal release

- The macOS build is unsigned, so Gatekeeper may prompt on first launch.
- Packaged bundled-app interaction still benefits from one human tester pass.
- One medium Rust advisory remains in the upstream Linux GTK/Tauri stack and is being tracked separately; it does not block this macOS internal release line.

## Reporting issues

When reporting an issue, include:

- the artifact version
- the commit SHA from `release-manifest.json`
- whether the issue happens only in packaged mode or also in `tauri dev`
