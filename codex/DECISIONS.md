# Decisions Log

## 2026-02-10
1. **Scope narrowed to workflow + docs improvements**
   - Rationale: user requested project-state and usefulness improvement; safest high-impact delta is contributor workflow parity and clarity without runtime risk.
   - Alternatives rejected: refactoring test suites to silence warning noise (broader touch, less predictable).

2. **Do not modify CI workflow in this pass**
   - Rationale: CI already enforces core checks; local script parity can be improved independently.
   - Alternatives rejected: introducing matrix/build changes would exceed minimal-risk scope.

3. **Treat Rust verification failure as environment blocker, not product failure**
   - Evidence: missing `glib-2.0.pc` in container prevents Tauri dependency build.
   - Mitigation: document prerequisite and keep frontend verification green.
