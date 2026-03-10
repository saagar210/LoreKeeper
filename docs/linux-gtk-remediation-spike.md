# Linux GTK Remediation Spike

This spike turns the current Linux desktop advisory follow-up into a bounded investigation we can run when Linux desktop becomes active scope.

## Goal

Determine whether LoreKeeper can move off the vulnerable `glib ^0.18` line through a safe upstream-compatible Tauri/Wry dependency path, without destabilizing the current macOS internal release line.

## Scope

In scope:
- inspect current dependency path and compatibility bounds
- identify the smallest plausible upstream-compatible upgrade path
- test whether that path resolves the `glib` advisory without introducing worse breakage
- capture a clear go / no-go recommendation

Out of scope:
- shipping Linux distribution in this spike
- forcing manual dependency overrides that break the supported Tauri stack
- changing product behavior or gameplay systems

## Starting point

- Current branch base for this spike family: `codex/chore/tauri-linux-advisory-strategy`
- Current known issue: `RUSTSEC-2024-0429`
- Current diagnosis command:

```bash
npm run diagnose:linux-stack
```

## Recommended execution order

1. Create a dedicated experiment branch from `codex/chore/tauri-linux-advisory-strategy`.
2. Run `npm run diagnose:linux-stack` and capture the current graph.
3. Review current Tauri, Wry, and related Linux stack release notes for GTK/WebKit dependency movement.
4. Attempt the smallest safe dependency update that still stays within upstream-supported lines.
5. Re-run:
   - `npm run test:rust`
   - `npm run lint:rust`
   - `cargo audit --file src-tauri/Cargo.lock`
6. Record whether the advisory is:
   - resolved,
   - still blocked upstream,
   - or replaced by a different blocker.

## Evidence to capture

- before/after `cargo tree --target all -i glib`
- before/after `Cargo.lock` glib entry
- exact Tauri/Wry versions tested
- Rust verification results
- audit results
- any new packaging or platform regressions discovered

## Exit criteria

A successful spike ends with one of these outcomes:

### Go path

- We find a supported dependency path that removes the vulnerable `glib` line.
- Rust verification remains healthy.
- No major new Linux desktop stack breakage is introduced.
- We can open a real remediation branch with confidence.

### No-go path

- Upstream still requires `glib ^0.18`, or
- the smallest safe update introduces larger platform instability, or
- the dependency path remains incompatible with the fixed advisory line.

If this happens, the spike is still a success if it records the blocker clearly.

## Decision output template

At the end of the spike, write down:

- tested versions
- result: go / no-go
- blocker summary
- recommended next action
- whether Linux desktop remains deferred or moves into active release scope
