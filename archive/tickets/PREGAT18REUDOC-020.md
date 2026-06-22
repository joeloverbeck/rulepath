# PREGAT18REUDOC-020: Convert GAME-AI / ADMISSION / PUBLIC-RELEASE-CHECKLIST to receipts/registries

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — docs-only (`templates/GAME-AI.md`, `templates/GAME-IMPLEMENTATION-ADMISSION.md`, `templates/PUBLIC-RELEASE-CHECKLIST.md`)
**Deps**: 012

## Problem

`GAME-AI.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, and `PUBLIC-RELEASE-CHECKLIST.md` are partly receipts already but still carry duplicated evidence (notably the comprehensive no-leak matrix in `PUBLIC-RELEASE-CHECKLIST.md`). Convert them fully to receipts/registries that reference `GAME-EVIDENCE.md`, keeping the lifecycle checkpoints (the report explicitly rejected *deleting* release/admission records) while removing duplicated proof.

## Assumption Reassessment (2026-06-22)

1. Verified the three templates exist: `GAME-AI.md` is a bot registry that already avoids duplicating the full evidence pack; `GAME-IMPLEMENTATION-ADMISSION.md` is a gate receipt; `PUBLIC-RELEASE-CHECKLIST.md` carries the comprehensive no-leak matrix duplicated with `GAME-UI`/`GAME-RULE-COVERAGE` — confirmed during the `/reassess-spec` validation this session. Migrate duplicated proof to `GAME-EVIDENCE.md` (ticket 012); hence `Deps: 012`.
2. Verified against spec D12 + reassess finding M1: convert to receipts/registries; `B-NN → template` mapping derived from report §4.
3. Cross-artifact boundary under audit: migrated fields land in `GAME-EVIDENCE.md` (ticket 012); the converted templates reference it.
4. FOUNDATIONS §11 motivates the discipline: restating — the report rejected deleting release/admission records, so the lifecycle checkpoints are *slimmed into receipts*, not removed; every removed required-proof field migrates to a named owner with the cross-reference in the same change.
5. Touches the §11 no-leak firewall (`PUBLIC-RELEASE-CHECKLIST.md` no-leak matrix): confirm the matrix lands in `GAME-EVIDENCE.md` (no proof lost) and the conversion introduces no leak path.

## Architecture Check

1. Converting to receipts/registries that point at `GAME-EVIDENCE.md` removes the duplicated no-leak matrix while preserving the admission/release decision checkpoints — cleaner than three partly-overlapping checklists.
2. No backwards-compatibility shims; removed proof is migrated, not aliased.
3. `engine-core` (§3) / `game-stdlib` (§4) untouched; docs-only.

## Verification Layers

1. Each of the three templates is a receipt/registry referencing `GAME-EVIDENCE.md` -> codebase grep-proof.
2. The `PUBLIC-RELEASE-CHECKLIST.md` no-leak matrix is migrated to `GAME-EVIDENCE.md` -> grep both sides.
3. The lifecycle checkpoints (admission decision, release decision, sign-off) are retained -> grep.
4. `templates/GAME-EVIDENCE.md` exists (Deps 012) -> `test -f`.
5. Links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Convert to receipts/registries

Convert `GAME-AI`, `GAME-IMPLEMENTATION-ADMISSION`, `PUBLIC-RELEASE-CHECKLIST` to receipts/registries that reference `GAME-EVIDENCE.md`, retaining their lifecycle decision checkpoints.

### 2. Migrate proof + record mapping

Migrate the duplicated no-leak matrix (and any other duplicated proof) into `GAME-EVIDENCE.md`; record the `B-NN → template` mapping per template.

## Files to Touch

- `templates/GAME-AI.md` (modify)
- `templates/GAME-IMPLEMENTATION-ADMISSION.md` (modify)
- `templates/PUBLIC-RELEASE-CHECKLIST.md` (modify)

## Out of Scope

- The rules/coverage cluster (017), the bot/strategy/UI/bench cluster (018), the ledger (019), and `AGENT-TASK.md` (021).
- Authoring the `GAME-EVIDENCE.md` receipt (ticket 012).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -lF "GAME-EVIDENCE" templates/GAME-AI.md templates/GAME-IMPLEMENTATION-ADMISSION.md templates/PUBLIC-RELEASE-CHECKLIST.md` lists all three.
2. The `PUBLIC-RELEASE-CHECKLIST.md` no-leak matrix is present in `GAME-EVIDENCE.md`; the admission/release lifecycle checkpoints remain in their templates (`grep -niE "decision|sign-off" templates/GAME-IMPLEMENTATION-ADMISSION.md`).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. No lifecycle checkpoint or required-proof field is deleted without migration to a named owner.
2. No §11 invariant is waived by the conversion.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (receipt-link grep, migration grep, checkpoint-retention grep, link check) named in Assumption Reassessment.`

### Commands

1. `grep -lF "GAME-EVIDENCE" templates/GAME-AI.md templates/GAME-IMPLEMENTATION-ADMISSION.md templates/PUBLIC-RELEASE-CHECKLIST.md`
2. `node scripts/check-doc-links.mjs`
3. The receipt-link grep + checkpoint-retention grep + link check is the correct boundary; migrated proof is grep-checked on the receipt side.

## Outcome

Completed on 2026-06-22. `templates/GAME-AI.md` is now a compact shipped
bot registry with report mapping `B-12 -> GAME-AI.md`; `templates/GAME-IMPLEMENTATION-ADMISSION.md`
is a pre-build admission and delta-admission receipt with mapping
`B-07 -> GAME-IMPLEMENTATION-ADMISSION.md`; and
`templates/PUBLIC-RELEASE-CHECKLIST.md` is a final release sign-off over
linked evidence with mapping `B-16 -> PUBLIC-RELEASE-CHECKLIST.md`.
The detailed hidden-information no-leak surface matrix now lives in
`templates/GAME-EVIDENCE.md`, and the release checklist links to that receipt
instead of duplicating the matrix.

Verification:

1. `grep -lF "GAME-EVIDENCE" templates/GAME-AI.md templates/GAME-IMPLEMENTATION-ADMISSION.md templates/PUBLIC-RELEASE-CHECKLIST.md` listed all three templates.
2. `grep -niE "Hidden-Information No-Leak Matrix|public/browser payload|candidate rankings|dev inspector/public build boundary" templates/GAME-EVIDENCE.md templates/PUBLIC-RELEASE-CHECKLIST.md` confirmed the no-leak matrix is owned by `GAME-EVIDENCE.md` and referenced by the release checklist.
3. `grep -niE "decision|sign-off" templates/GAME-IMPLEMENTATION-ADMISSION.md templates/PUBLIC-RELEASE-CHECKLIST.md` confirmed admission/release lifecycle checkpoints remain.
4. `test -f templates/GAME-EVIDENCE.md` passed.
5. `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).
