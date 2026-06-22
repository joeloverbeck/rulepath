# PREGAT18REUDOC-017: Slim the rules/coverage template cluster into GAME-EVIDENCE

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — docs-only (`templates/GAME-SOURCES.md`, `templates/GAME-RULES.md`, `templates/GAME-RULE-COVERAGE.md`, `templates/GAME-MECHANICS.md`, `templates/GAME-HOW-TO-PLAY.md`)
**Deps**: 012

## Problem

The rules/coverage template cluster duplicates evidence (no-leak/viewer matrices, IP/source receipts, benchmark-relevance maps) that now belongs in the canonical `GAME-EVIDENCE.md` receipt. Slim each to its domain authority plus stable evidence IDs and links into the receipt — migrating, never silently deleting, any field that carries required proof.

## Assumption Reassessment (2026-06-22)

1. Verified all five templates exist and carry the duplicated evidence blocks (e.g. `GAME-RULE-COVERAGE.md` hidden-info / pairwise no-leak matrix; `GAME-SOURCES.md` IP/source receipt) confirmed during the `/reassess-spec` validation this session. The slim migrates these fields to `GAME-EVIDENCE.md` (ticket 012); hence `Deps: 012`.
2. Verified against spec D12 + reassess finding M1: decompose template slimming per reviewable diff; this ticket clusters the rules/coverage role group. The exact `B-NN → template` mapping is derived by re-reading the report §4 change-plan and **recorded per template in this ticket's implementation** (per M1, so each diff traces to its source claim).
3. Cross-artifact boundary under audit: every migrated field lands in `templates/GAME-EVIDENCE.md` (ticket 012); the slimmed templates link to it. `GAME-EVIDENCE.md` must exist first (`Deps: 012`).
4. FOUNDATIONS §11 motivates the migration discipline: restating — every removed field carrying required proof migrates to a named owner (`GAME-EVIDENCE.md`) with the cross-reference landing in the **same change**; no §11 invariant is waived by the slim.
5. Touches the §11 no-leak firewall (`GAME-RULE-COVERAGE.md` carries the no-leak/viewer matrix): confirm the no-leak matrix lands in `GAME-EVIDENCE.md` (no proof lost) and the slim introduces no leak path.

## Architecture Check

1. Slimming each template to its domain authority + receipt links removes cross-template duplication while keeping each template's authoritative role — cleaner than five partly-overlapping evidence copies.
2. No backwards-compatibility shims; removed proof is migrated, not aliased.
3. `engine-core` (§3) / `game-stdlib` (§4) untouched; docs-only.

## Verification Layers

1. Each of the five templates is slimmed to domain authority + links to `GAME-EVIDENCE.md` -> codebase grep-proof.
2. Every removed required-proof field is migrated to `GAME-EVIDENCE.md` with a cross-reference -> grep both sides (field present in receipt, link present in template).
3. The `B-NN → template` mapping is recorded -> manual review of the ticket's implementation notes.
4. `templates/GAME-EVIDENCE.md` exists (Deps 012) -> `test -f`.
5. Links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Slim each template

Reduce `GAME-SOURCES`, `GAME-RULES`, `GAME-RULE-COVERAGE`, `GAME-MECHANICS`, `GAME-HOW-TO-PLAY` to their domain authority, with stable evidence IDs and links into `GAME-EVIDENCE.md`.

### 2. Migrate proof + record mapping

Migrate each removed required-proof field (no-leak matrix, IP/source receipt, benchmark relevance) into `GAME-EVIDENCE.md`, and record the `B-NN → template` mapping per template.

## Files to Touch

- `templates/GAME-SOURCES.md` (modify)
- `templates/GAME-RULES.md` (modify)
- `templates/GAME-RULE-COVERAGE.md` (modify)
- `templates/GAME-MECHANICS.md` (modify)
- `templates/GAME-HOW-TO-PLAY.md` (modify)

## Out of Scope

- The bot/strategy/UI/bench cluster (ticket 018), the ledger (019), the receipt conversions (020), and `AGENT-TASK.md` (021).
- Authoring the `GAME-EVIDENCE.md` receipt (ticket 012).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -lF "GAME-EVIDENCE" templates/GAME-SOURCES.md templates/GAME-RULES.md templates/GAME-RULE-COVERAGE.md templates/GAME-MECHANICS.md templates/GAME-HOW-TO-PLAY.md` lists all five (each links to the receipt).
2. The no-leak/viewer matrix removed from `GAME-RULE-COVERAGE.md` is present in `GAME-EVIDENCE.md` (`grep -niE "no-leak|viewer matrix" templates/GAME-EVIDENCE.md`).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. No required-proof field is deleted without a migration to a named owner in the same change.
2. No §11 invariant is waived by the slim.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (receipt-link grep, migration grep, link check) named in Assumption Reassessment.`

### Commands

1. `grep -lF "GAME-EVIDENCE" templates/GAME-SOURCES.md templates/GAME-RULES.md templates/GAME-RULE-COVERAGE.md templates/GAME-MECHANICS.md templates/GAME-HOW-TO-PLAY.md`
2. `node scripts/check-doc-links.mjs`
3. The receipt-link grep + link check is the correct boundary; the migrated-proof presence is grep-checked on the receipt side.
