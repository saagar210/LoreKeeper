# Post-Release Branch Audit

This note records the current status of older task branches after the internal release promotion to `master`.

## Branches retired in this pass

These branches were true ancestors of `master` and were safe to delete:

- `codex/aggressive-prune-cleanup`
- `codex/lean-dev-mode`
- `codex/remediate-tests-docs-contracts-v1`

Remote branch also deleted:

- `origin/codex/lean-dev-mode`

## Branches intentionally kept

### `codex/build/default-branch-risk-hardening`

Decision: keep for now as archive history.

Why:

- It is not a true ancestor of `master`.
- It still shows a wide diff against `master`, including the module-hardening commit history.
- Some of its work was promoted through later branches and release promotion steps rather than a clean merge lineage.

Practical takeaway:

- Do not treat it as active delivery work.
- Keep it until we either perform a deeper history reconciliation or decide we no longer need that historical trace.

### `codex/chore/bootstrap-codex-os`

Decision: keep for now as archive history.

Why:

- It is not a true ancestor of `master`.
- Its remaining diff is broad and overlaps with work that was later folded into the shipped release line through different commits.
- Its PR was closed as superseded, but the branch still serves as historical context for earlier repo bootstrap work.

Practical takeaway:

- Keep it archived, not active.
- Revisit only if we want a deeper historical cleanup or branch retirement pass later.

## Current rule of thumb

- Delete older branches automatically only when they are true merged ancestors of `master`.
- Keep non-ancestor branches unless we do a deliberate history-review pass first.
