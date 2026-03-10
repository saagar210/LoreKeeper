# LoreKeeper Internal Release Checklist

This checklist records the current internal release candidate evidence for the macOS unsigned build line.

## Candidate

- Candidate branch: `codex/fix/default-branch-risk-hardening`
- Default branch fallback ref: `origin/master @ 990bae4`
- Exact release commit SHA: read from the generated `release-manifest.json`
- Exact artifact directory: `release-artifacts/internal/macos/<version>-<shortsha>/`

## Branch Convergence Decision

- `codex/lean-dev-mode` -> included in candidate
- `codex/chore/bootstrap-codex-os` -> included in candidate
- `codex/build/default-branch-risk-hardening` -> included in candidate
- `codex/aggressive-prune-cleanup` -> included in candidate
- `origin/master` Codex bootstrap-default commits -> treated as superseded by the candidate branch, not a separate release blocker

## Verification Results

- `npm run version:check` -> Pass
- `npm run verify:internal-release` -> Pass
- `npm run release:internal:mac` -> Pass

## Current Artifact Evidence

- Artifact directory: `release-artifacts/internal/macos/<version>-<shortsha>/`
- Manifest: `release-artifacts/internal/macos/<version>-<shortsha>/release-manifest.json`
- Checksums: `release-artifacts/internal/macos/<version>-<shortsha>/checksums.txt`
- Produced files:
  - `LoreKeeper_0.1.0_aarch64.dmg`
  - `LoreKeeper.app.tar.gz`

## QA Matrix

### Core product smoke

- Launch packaged app -> Pass
- Packaged app creates a visible window named `LoreKeeper` -> Pass
- Start new game -> Pass via browser E2E coverage
- Basic commands (`look`, movement, inventory/help) -> Pass via browser E2E coverage
- Open settings -> Pass via browser E2E coverage
- Save game -> Pass via browser E2E coverage
- Load saved game -> Pass via browser E2E coverage
- Quick save -> Pass via browser E2E coverage
- Theme change applies -> Pass via browser E2E coverage
- Theme creator valid save/load -> Pass via component and integration coverage
- Invalid save name rejected -> Pass via browser E2E coverage
- Module screen opens -> Pass via component coverage
- Stats, achievements, replay screens open -> Pass via component coverage

### Persistence and compatibility smoke

- Malformed save/theme/settings rows fail safe -> Pass via Rust and frontend automated coverage
- Existing settings load without crashing -> Pass via settings persistence coverage
- Existing custom themes either load or are omitted safely -> Pass via theme sanitization coverage
- Existing save slots with pre-hardening names remain loadable, deletable, and overwriteable -> Pass via Rust and frontend coverage
- Existing local saves from a prior real user environment -> Waiver

Waiver note:

- We validated compatibility behavior through automated persistence tests and fail-safe handling, but we did not run a manual packaged-app session against a previously accumulated real user data set on this machine during this pass.

### Narration posture smoke

- App works with Ollama disabled -> Pass via current automated coverage and fallback-path behavior
- Local-only Ollama validation still holds -> Pass via Rust and frontend tests
- No network dependency blocks startup -> Pass

### Packaged-app behavior smoke

- Packaged app opens without missing frontend assets -> Pass
- No blank-window startup on launch -> Pass
- Packaged-mode interactive gameplay beyond launch/window smoke -> Waiver

Waiver note:

- There is no repo-owned packaged Tauri GUI automation harness yet. Interactive bundled-mode confidence is covered by browser E2E plus direct packaged-app launch/window verification, and should receive one human tester pass during internal distribution.

## Known Issues

- The macOS artifact is unsigned, so Gatekeeper may prompt on first launch.
- Packaged interactive smoke is partially waived because the repo does not yet automate bundled Tauri UI interaction.
- Tauri warns that the current bundle identifier ends with `.app`; this is not blocking the internal build but should be corrected before any public distribution work.

## Release Decision

- Current status: Go for internal macOS release candidate distribution, with documented low-risk waivers only
- Blocking P0/P1 issues found in this pass: none

## Rollback

- Roll back to: `origin/master @ 990bae4`
- Re-run before redistributing a replacement candidate:
  - `npm run version:check`
  - `npm run verify:internal-release`
