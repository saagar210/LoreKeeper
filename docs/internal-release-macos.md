# LoreKeeper Internal Release Guide (macOS)

This guide defines the current internal release contract for LoreKeeper.

## Release Scope

- Platform: macOS only
- Distribution posture: unsigned internal build
- Signing/notarization: intentionally out of scope for this release
- Default branch for promotion: `master`
- Release candidate branch: `codex/fix/default-branch-risk-hardening`
- Candidate baseline before release-prep work: `42eed42`
- Fallback ref captured before release-prep promotion: `origin/master` at `990bae4`

Exact release commit SHA is recorded in the build manifest created by:

```bash
npm run release:internal:mac
```

The manifest is written to:

```text
release-artifacts/internal/macos/<version>-<shortsha>/release-manifest.json
```

## Branch Convergence

The current candidate already includes the branch work we needed for internal release:

- `codex/lean-dev-mode` -> included
- `codex/chore/bootstrap-codex-os` -> included
- `codex/build/default-branch-risk-hardening` -> included
- `codex/aggressive-prune-cleanup` -> included

`origin/master` still contains older Codex bootstrap defaults that diverged separately. For this internal release, those commits are treated as superseded by the current candidate branch and are not a separate release blocker.

## Release Commands

Run these commands in order when preparing an internal candidate:

```bash
npm run version:check
npm run verify:internal-release
npm run release:internal:mac
```

What they do:

- `npm run version:check`: ensures version parity across `package.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json`
- `npm run verify:internal-release`: runs the canonical repo verification flow, then full correctness verification, then browser-level E2E
- `npm run release:internal:mac`: builds the unsigned macOS artifact, writes checksums, and writes a release manifest

## Artifact Contract

Internal release artifacts are written to:

```text
release-artifacts/internal/macos/<version>-<shortsha>/
```

That folder contains:

- final artifact files produced from the Tauri macOS bundle output
- `release-manifest.json`
- `checksums.txt`
- `BUILD_INFO.txt`

Preferred artifact:

- default Tauri macOS bundle output for the current app version

Accepted secondary outputs:

- any other macOS bundle file emitted by Tauri during the same build

## Current Candidate Highlights

Compared with the default branch fallback line, this candidate adds:

- lean dev mode plus explicit cleanup commands
- canonical repo verification and CI/perf contract alignment
- repo-owned commit/perf tooling and guard scripts
- desktop hardening for modules, settings, themes, and save/load validation
- browser-level E2E coverage for the current validated flows

Required verification commands for every internal candidate:

```bash
bash .codex/scripts/run_verify_commands.sh
npm run verify:full
npm run test:e2e
npm run version:check
```

## Install and Run

1. Build or download the internal artifact directory.
2. Prefer the `.dmg` artifact if present.
3. If only the archived `.app` bundle is available:
   - extract the archive
   - move `LoreKeeper.app` into `/Applications` or another trusted local folder
4. Open the app normally.

If macOS Gatekeeper blocks launch because the build is unsigned:

1. Control-click the app and choose `Open`
2. If macOS still blocks it, remove the quarantine attribute:

```bash
xattr -dr com.apple.quarantine /Applications/LoreKeeper.app
```

3. Re-open the app

## Tester Charter

Validate these areas for the internal release:

- app launches successfully from the packaged build
- title screen loads without a blank window
- new game starts
- basic commands work: `look`, movement, inventory/help
- settings open and close cleanly
- save/load works
- quick save works
- theme change applies
- theme creator can save and load a valid theme
- invalid save names are rejected cleanly
- modules, stats, achievements, and replay screens open
- app works with Ollama disabled
- existing saves/settings do not crash the app

Do not block this internal release on:

- signing or notarization
- public-distribution polish
- non-macOS platforms
- updater/distribution automation beyond artifact generation

## Known Issues

- The internal macOS build is unsigned, so Gatekeeper may prompt on first launch.
- Internal release verification relies on the existing browser E2E harness plus local packaged-build smoke. There is no repo-owned fully automated GUI test harness for the packaged Tauri app yet.
- Ollama remains optional and local-only by design. Internal release validation should not treat unavailable Ollama as a blocker if the fallback narration path works.
- Tauri warns that the bundle identifier `com.lorekeeper.app` ends with `.app`. The current build still succeeds, but that identifier should be corrected before public distribution if we plan any data-path migration carefully.

## Reporting and Triage

When reporting an internal release issue, include:

- artifact version
- commit SHA from `release-manifest.json`
- the exact artifact file used
- reproduction steps
- whether the issue happens in packaged mode only or also in `tauri dev`

Treat these as release blockers:

- packaged app fails to launch
- blank window or missing frontend assets
- save/load breaks
- clear data-loss risk
- any known P0/P1 regression in the verified core flows

## Rollback

Rollback target for this internal release line:

```text
origin/master @ 990bae4
```

If the internal candidate is rejected, fall back to that ref and re-run the internal release verification set before distributing another artifact.
