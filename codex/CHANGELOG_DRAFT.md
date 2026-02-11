# Changelog Draft

## Theme: Verification Ergonomics
- Added CI-parity npm scripts:
  - `typecheck`, `test:frontend`, `build:frontend`
  - `lint:rust`, `test:rust`
  - `verify:frontend`, `verify:full`
- Why: make verification repeatable and discoverable for contributors.

## Theme: Contributor Workflow Documentation
- Added `Engineering Verification Workflow` section to README.
- Documented local frontend-only verification command and full parity command.
- Added Linux prerequisite note for Rust/Tauri checks requiring GTK/GLib development packages.

## Theme: Autonomous Session Continuity
- Added codex artifacts:
  - `SESSION_LOG.md`
  - `PLAN.md`
  - `DECISIONS.md`
  - `CHECKPOINTS.md`
  - `VERIFICATION.md`
  - `CHANGELOG_DRAFT.md`
- Why: preserve auditable, interruption-safe context with clear resume steps.
