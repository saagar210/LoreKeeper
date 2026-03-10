# LoreKeeper

**A text adventure game where an AI narrator tells your story.**

You explore the dungeon. Rust owns the world state. An LLM writes the prose. No LLM? No problem — template narration kicks in so the game always works.

## The Game: _The Depths of Thornhold_

```
You descend into the ruins of Thornhold, a fortress long abandoned
to darkness. Somewhere below, the Dungeon Heart pulses with ancient
power. Will you claim it, destroy it, or strike a deal with its keeper?
```

- 14 handcrafted locations + 5 procedurally generated dungeon rooms
- 7 NPCs with memory, relationships, and LLM-powered dialogue
- 25+ items, a crafting system, and hidden secret commands
- 5 quests, multiple endings, and a full achievement system
- Combat, status effects, difficulty modes, and a journal/codex
- Replay system with ghost playthrough comparison
- Visual map editor for creating your own adventure modules

## Tech Stack

| Layer       | Tech                                                             |
| ----------- | ---------------------------------------------------------------- |
| Desktop app | [Tauri 2](https://tauri.app/)                                    |
| Game engine | Rust                                                             |
| Frontend    | React 19 + TypeScript (strict)                                   |
| Narration   | [Ollama](https://ollama.com/) (local LLM) with template fallback |
| Persistence | SQLite (saves, stats, achievements, themes)                      |

## Quick Start

```bash
# Prerequisites: Rust, Node.js, and optionally Ollama

npm install
npm run tauri dev

# For LLM narration (optional)
ollama pull llama3.2
```

## Internal Release

For the macOS internal release workflow, artifact contract, Gatekeeper notes, tester charter, and rollback reference, use:

- `docs/internal-release-macos.md`
- `docs/internal-release-checklist.md`

## Dev Modes

### Normal dev

```bash
npm run tauri dev
```

- Fastest restarts after the first compile because Rust/Vite build artifacts are kept locally (`$HOME/.cache/lorekeeper/cargo-target`, `node_modules/.vite`).
- Uses more disk over time.

### Lean dev (low disk)

```bash
npm run dev:lean
```

- Starts the same app flow (`tauri dev`) but redirects heavy temporary build output to OS temp directories.
- Temporary Cargo and Vite caches are removed automatically when the process exits.
- Extra Tauri args are supported with `npm run dev:lean -- <args>`.
- Tradeoff: first startup and recompiles are slower than normal dev because cached artifacts are not persisted between runs.

### Daily low-disk command

Use this daily command to keep local disk growth under control while developing:

```bash
npm run dev:lean
```

## Cleanup Commands

```bash
# Remove heavy build artifacts only (keeps dependencies installed)
npm run clean:heavy

# Remove all reproducible local caches (includes node_modules)
npm run clean:local
```

Both cleanup commands also clear the default Rust verification cache at
`$HOME/.cache/lorekeeper/cargo-target` (or `LOREKEEPER_CARGO_TARGET_DIR` if set).

## Engineering Verification Workflow

Use the command below depending on the level of confidence you need:

```bash
# Local-fast frontend check (no Rust/Tauri system packages required)
npm run verify:frontend

# Full app correctness gate used by the main CI workflow
npm run verify:full

# Canonical local gate runner for deterministic script order
bash .codex/scripts/run_verify_commands.sh

# Browser-level user flow coverage
npm run test:e2e
```

What each command means:

- `npm run verify:frontend`: fastest everyday check for TypeScript, Vitest, and frontend build health.
- `npm run verify:full`: the repo's main correctness gate; it runs frontend verification plus Rust lint and Rust tests.
- `bash .codex/scripts/run_verify_commands.sh`: the canonical deterministic local runner for `.codex/verify.commands`, including git hygiene and the non-Lighthouse perf checks.
- `npm run test:e2e`: Playwright coverage for browser-level flows using the existing `e2e` config.

CI mapping:

- `.github/workflows/ci.yml` runs `npm run verify:full`.
- `.github/workflows/e2e.yml` runs `npm run test:e2e` on pull requests.
- Performance checks stay in the dedicated perf workflows rather than the main correctness gate.

Granular commands:

- `npm run typecheck`
- `npm run test:frontend`
- `npm run build:frontend`
- `npm run lint:rust`
- `npm run test:rust`
- `npm run perf:bundle`
- `npm run perf:build`
- `npm run perf:assets`
- `npm run perf:memory`
- `npm run perf:lhci`
- `npm run perf:lhci:prod`

### Commit helper

`npm run commit` no longer uses an interactive third-party prompt. It now:

- suggests a Conventional Commit message when you have staged changes, or
- creates the commit directly when you pass a message, for example:

```bash
npm run commit -- "fix(repo): tighten lighthouse tooling"
```

### Lighthouse audits

`npm run perf:lhci` and `npm run perf:lhci:prod` now use a repo-owned Lighthouse runner.
It serves the built frontend with `vite preview`, runs the configured number of Lighthouse
passes, writes results to `.perf-results/lighthouse.json`, and enforces the score thresholds
defined in `lighthouserc.json` or `.lighthouserc.production.json`.

### Performance profile policy

- Baseline checks (`perf-foundation`) run on pull requests for continuous signal.
- Enforced production budgets (`perf-enforced`) run when `PERF_PROFILE=production`.
- Required gate policy: `fail` or `not-run` on required checks blocks done-state.
- SEO remains informational for this desktop Tauri app, so Lighthouse SEO is tracked as a warning rather than a release blocker.

### Linux note for Rust/Tauri checks

Rust/Tauri checks require GTK/GLib development libraries (for example `glib-2.0`).
If `cargo clippy` or `cargo test` fails with missing `glib-2.0.pc`, install the system
packages used in CI (`libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`, `libgtk-3-dev`) before rerunning full verification.

### Rust verification cache location

`npm run tauri ...`, `npm run lint:rust`, and `npm run test:rust` default Cargo build output to
`$HOME/.cache/lorekeeper/cargo-target` (override with `LOREKEEPER_CARGO_TARGET_DIR`).
This avoids path-separator issues on some machines and keeps cleanup behavior deterministic.

## Codex Local Environment Actions

Configure these in Codex App `Settings > Local environments` and check the generated `.codex` config into the repo.

Setup script:

```bash
npm install
npm run build:frontend
```

Recommended actions:

- `Run app`: `npm run tauri dev`
- `Lean dev`: `npm run dev:lean`
- `Verify frontend`: `npm run verify:frontend`
- `Verify full`: `npm run verify:full`
- `Verify canonical`: `bash .codex/scripts/run_verify_commands.sh`
- `Cleanup heavy`: `npm run clean:heavy`
- `Cleanup local`: `npm run clean:local`

## Feature Maturity Policy

- `Stable` features are default for production paths.
- `Beta` and `Experimental` features require explicit owner, rollback plan, and fallback path.
- Current repository default: multi-agent workflows are optional accelerators, not required gates.

## Desktop Security Notes

- The Tauri main window now uses a least-privilege capability file that grants event access without the broader `core:default` shell surface.
- Module loading avoids exposing absolute filesystem paths to the frontend; the UI works with safe module IDs and the backend resolves them inside the app-owned `modules/` directory.
- Module export follows the same rule: the editor gets back a safe module ID rather than a machine-specific path.
- Ollama integration is local-only by design. Settings now accept `http://localhost` and loopback IPs only, which keeps narration traffic on the same machine.
- Custom themes are validated on save and sanitized again before live CSS is applied, so malformed or unexpected theme payloads are ignored instead of mutating arbitrary styles.
- Custom module ingestion now enforces file-size and structural guardrails before a module can be listed, validated, exported, or loaded into live game state.

## Quality Snapshot

This repository keeps quality status tied to runnable commands instead of hard-coded counts that drift over time.

- Frontend health: `npm run verify:frontend`
- Full app correctness gate: `npm run verify:full`
- Canonical deterministic local gate runner: `bash .codex/scripts/run_verify_commands.sh`
- Browser-level user flows: `npm run test:e2e`
- Performance budgets and reports: `npm run perf:*` plus the dedicated perf GitHub Actions workflows

For the current state of the branch, run the commands above instead of relying on a static stats block.

## Architecture

```
┌─────────────────────────────────────────┐
│              Tauri Shell                │
│  ┌──────────────┐  ┌────────────────┐  │
│  │   React UI   │  │  Rust Engine   │  │
│  │  Terminal +  │◄─┤  Parser        │  │
│  │  Sidebar +   │  │  Executor      │  │
│  │  Map Editor  │  │  World State   │  │
│  └──────┬───────┘  │  Combat/Quests │  │
│         │          │  Crafting      │  │
│         │          └───────┬────────┘  │
│         │                  │           │
│         │          ┌───────▼────────┐  │
│         │          │   Narrator     │  │
│         ◄──────────┤  Ollama / LLM  │  │
│      events        │  or Templates  │  │
│                    └────────────────┘  │
└─────────────────────────────────────────┘
```

## License

MIT
