# Internal Release Team Update

Use the message below as a ready-to-send internal update.

## Short version

LoreKeeper is now available on `master` as the current internal macOS release line.

Highlights:
- lower-disk dev workflow plus cleanup commands
- stronger desktop hardening across modules, saves, settings, and themes
- repo-owned internal macOS packaging and verification flow
- browser-level E2E coverage for the validated core flows

Use these docs for the release workflow and tester guidance:
- `docs/internal-release-handoff.md`
- `docs/internal-release-macos.md`
- `docs/internal-release-checklist.md`

Known limits:
- macOS build is unsigned, so Gatekeeper may prompt on first launch
- packaged bundled-app interaction still benefits from one human tester pass
- the open Linux GTK/Tauri advisory is being tracked separately and does not block this macOS internal release line

## Slack-ready version

LoreKeeper is now on `master` as the current internal macOS release line.

What’s in it:
- leaner low-disk dev flow and cleanup commands
- stronger desktop hardening for modules, saves, settings, and custom themes
- repo-owned internal macOS packaging with manifest/checksum output
- browser-level E2E coverage for the current validated core flows

Where to look:
- `docs/internal-release-handoff.md`
- `docs/internal-release-macos.md`
- `docs/internal-release-checklist.md`

Current limits:
- unsigned macOS build, so Gatekeeper may prompt
- bundled-app GUI still benefits from one human tester pass
- Linux GTK/Tauri advisory is tracked separately and is not blocking this macOS internal release line
