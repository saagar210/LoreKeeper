## Definition of Done: Tests + Docs (Blocking)

- Any production code change must include meaningful test updates in the same PR.
- Meaningful tests must include at least:
  - one primary behavior assertion
  - two non-happy-path assertions (edge, boundary, invalid input, or failure mode)
- Trivial assertions are forbidden (`expect(true).toBe(true)`, snapshot-only without semantic assertions, render-only smoke tests without behavior checks).
- Mock only external boundaries (network, clock, randomness, third-party SDKs). Do not mock the unit under test.
- UI changes must cover state matrix: loading, empty, error, success, disabled, focus-visible.
- API/command surface changes must update generated contract artifacts and request/response examples.
- Architecture-impacting changes must include an ADR in `/docs/adr/`.
- Required checks are blocking when `fail` or `not-run`: lint, typecheck, tests, coverage, diff coverage, docs check.
- Reviewer -> fixer -> reviewer loop is required before merge.
