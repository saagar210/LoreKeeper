# Delta Plan

## A) Executive Summary

### Current state (repo-grounded)
- Desktop app uses Tauri 2 shell with Rust backend and React/TypeScript frontend (`src-tauri/src/lib.rs`, `src/App.tsx`).
- Command surface is exposed through Tauri `invoke_handler` in one centralized backend entry (`src-tauri/src/lib.rs`).
- Frontend has strong unit test coverage with Vitest (31 files/182 tests passing in baseline).
- CI pipeline already enforces typecheck, frontend tests, clippy, Rust tests, and Vite build (`.github/workflows/ci.yml`).
- Baseline local environment cannot run Rust verification due to missing `glib-2.0` system library.
- Current `package.json` scripts do not provide a single parity command that mirrors CI checks.
- README is player-facing and feature-rich, but lacks explicit engineering workflow guidance for contributors.

### Key risks
- Contributor friction due to fragmented verification commands.
- Frontend-only environments can silently skip Rust checks without clear documentation.
- CI parity drift risk when developers run partial local checks.
- Test output includes known warning noise, making signal-to-noise lower for true regressions.

### Improvement themes (prioritized)
1. **Verification ergonomics and CI parity** for day-to-day engineering workflow.
2. **Contributor documentation** for local/full verification and environment prerequisites.
3. **Session continuity artifacts** for interruption-safe autonomous execution.

## B) Constraints & Invariants (Repo-derived)

### Explicit invariants
- Existing game behavior and runtime contracts must remain unchanged.
- Existing CI command semantics in `.github/workflows/ci.yml` must remain valid.
- No weakening of type checks or tests.

### Implicit invariants (inferred)
- Frontend tests assume jsdom and mocked Tauri APIs for UI hooks/components.
- Rust build relies on system GTK/GLib libraries in Linux CI/runtime.

### Non-goals
- No gameplay/content refactor.
- No backend command contract changes.
- No persistence schema migration.

## C) Proposed Changes by Theme (Prioritized)

### Theme 1: Verification ergonomics and CI parity
- **Current approach:** Multiple discrete scripts and CI-only command sequence (`package.json`, `.github/workflows/ci.yml`).
- **Proposed change:** Add explicit npm scripts: `typecheck`, `test:frontend`, `build:frontend`, `lint:rust`, `test:rust`, `verify:frontend`, and `verify:full`.
- **Why:** Faster onboarding; reduced command ambiguity; easier local/CI parity.
- **Tradeoffs:** Slight script duplication with CI file; accepted for discoverability.
- **Scope boundary:** Package scripts only; no workflow YAML edits.
- **Migration approach:** Add scripts without removing existing commands.

### Theme 2: Contributor documentation
- **Current approach:** README focuses on product and quick start.
- **Proposed change:** Add engineering verification section documenting script usage and Linux system dependency caveat.
- **Why:** Makes workflow readiness explicit; prevents false-negative local failures.
- **Tradeoffs:** README grows moderately.
- **Scope boundary:** README only.
- **Migration approach:** Additive docs.

### Theme 3: Session continuity artifacts
- **Current approach:** No persistent codex progress artifacts.
- **Proposed change:** Maintain `codex/*.md` tracking plan, decisions, verification, checkpoints, changelog draft.
- **Why:** Enables clean interruption/resume and auditable trail.
- **Tradeoffs:** Extra repo docs overhead.
- **Scope boundary:** `codex/` folder only.

## D) File/Module Delta (Exact)

### ADD
- `codex/SESSION_LOG.md` — step-by-step execution log.
- `codex/PLAN.md` — this delta plan.
- `codex/DECISIONS.md` — judgment-call register.
- `codex/CHECKPOINTS.md` — periodic checkpoints + rehydration blocks.
- `codex/VERIFICATION.md` — command evidence ledger.
- `codex/CHANGELOG_DRAFT.md` — delivery-oriented change summary.

### MODIFY
- `package.json` — add workflow scripts for CI parity.
- `README.md` — add engineering verification guidance.

### REMOVE/DEPRECATE
- None.

### Boundary rules
- Allowed: package scripts and docs.
- Forbidden: game logic, backend contracts, persistence schemas.

## E) Data Models & API Contracts (Delta)
- **Current:** Contracts in `src/store/types.ts` and backend command handlers in `src-tauri/src/commands/*`.
- **Proposed changes:** None.
- **Compatibility:** Fully backward compatible; no command/data changes.
- **Migrations:** None.
- **Versioning strategy:** No version bump required for internal workflow/doc changes.

## F) Implementation Sequence (Dependency-Explicit)
1. **Step 1: Add script aliases and aggregate verification commands.**
   - Files: `package.json`
   - Preconditions: baseline frontend commands pass.
   - Dependencies: none.
   - Verify: `npm run typecheck`, `npm run test:frontend`, `npm run build:frontend`.
   - Rollback: revert `package.json`.
2. **Step 2: Document engineering verification workflow and environment caveats.**
   - Files: `README.md`
   - Preconditions: Step 1 scripts available.
   - Dependencies: Step 1.
   - Verify: `npm run verify:frontend`.
   - Rollback: revert README section.
3. **Step 3: Finalize codex artifacts and delivery changelog.**
   - Files: `codex/*.md`
   - Preconditions: implementation complete.
   - Dependencies: Steps 1-2.
   - Verify: `git diff --stat` sanity + rerun `npm run verify:frontend`.
   - Rollback: revert codex docs if inaccurate.

## G) Error Handling & Edge Cases
- Current error patterns: frontend catches `invoke` failures and displays status message (`src/App.tsx`); Rust command errors propagate via `Result`-based handlers.
- Proposed improvements: none to runtime error taxonomy.
- Edge cases addressed:
  - Running full verification in frontend-only environment.
  - Distinguishing local tooling blockers vs product regressions.
- Tests: existing test suite used as regression signal.

## H) Integration & Testing Strategy
- Integration points: npm script execution + README instructions.
- Unit tests: no new tests needed (no runtime behavior changes).
- Regression checks:
  - script-level: typecheck/test/build commands via new aliases.
  - docs-level: commands in README must map to real scripts.
- Definition of Done:
  - scripts added and executable,
  - README updated with accurate instructions,
  - codex logs/checkpoints complete,
  - final frontend verification green.

## I) Assumptions & Judgment Calls

### Assumptions
- CI remains source of truth for full-stack verification.
- Rust checks remain blocked locally without GTK/GLib packages.

### Judgment calls
- Chose script/documentation improvements over test-warning cleanup due lower risk and broad workflow impact.
- Avoided editing CI workflow to keep scope minimal and reversible.
