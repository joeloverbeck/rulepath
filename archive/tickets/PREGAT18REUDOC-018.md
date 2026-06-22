# PREGAT18REUDOC-018: Slim the bot/strategy/UI/bench template cluster into GAME-EVIDENCE

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — docs-only (`templates/COMPETENT-PLAYER.md`, `templates/BOT-STRATEGY-EVIDENCE-PACK.md`, `templates/GAME-UI.md`, `templates/GAME-BENCHMARKS.md`)
**Deps**: 012

## Problem

The bot/strategy/UI/bench template cluster duplicates evidence — benchmark workload declarations (shared between `BOT-STRATEGY-EVIDENCE-PACK.md` and `GAME-BENCHMARKS.md`), the hidden-info safeguards matrix in `GAME-UI.md`, and strategy/level declarations across `COMPETENT-PLAYER.md` — that now belongs in `GAME-EVIDENCE.md`. Slim each to its domain authority + receipt links, migrating required proof.

## Assumption Reassessment (2026-06-22)

1. Verified all four templates exist and carry duplicated evidence (benchmark workload IDs duplicated between `BOT-STRATEGY-EVIDENCE-PACK.md` and `GAME-BENCHMARKS.md`; `GAME-UI.md` hidden-info safeguards matrix; `COMPETENT-PLAYER.md` strategy/level declarations) — confirmed during the `/reassess-spec` validation this session. Migrate to `GAME-EVIDENCE.md` (ticket 012); hence `Deps: 012`.
2. Verified against spec D12 + reassess finding M1: this ticket clusters the bot/strategy/UI/bench role group; the `B-NN → template` mapping is derived from the report §4 change-plan and recorded per template.
3. Cross-artifact boundary under audit: every migrated field lands in `GAME-EVIDENCE.md` (ticket 012); the slimmed templates link to it.
4. FOUNDATIONS §11 motivates the migration discipline: every removed required-proof field migrates to a named owner with the cross-reference in the same change; no §11 invariant is waived.
5. Touches the §11 no-leak firewall (`GAME-UI.md` hidden-info safeguards matrix): confirm the matrix lands in `GAME-EVIDENCE.md` (no proof lost) and the slim introduces no leak path.

## Architecture Check

1. Consolidating the duplicated benchmark workload IDs and no-leak matrix into the receipt removes the cross-template drift the reassessment confirmed, while each template keeps its authoritative domain content.
2. No backwards-compatibility shims; removed proof is migrated, not aliased.
3. `engine-core` (§3) / `game-stdlib` (§4) untouched; docs-only.

## Verification Layers

1. Each of the four templates is slimmed to domain authority + links to `GAME-EVIDENCE.md` -> codebase grep-proof.
2. Every removed required-proof field (benchmark workload IDs, no-leak matrix, strategy/level declaration) is migrated to `GAME-EVIDENCE.md` -> grep both sides.
3. The `B-NN → template` mapping is recorded -> manual review.
4. `templates/GAME-EVIDENCE.md` exists (Deps 012) -> `test -f`.
5. Links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Slim each template

Reduce `COMPETENT-PLAYER`, `BOT-STRATEGY-EVIDENCE-PACK`, `GAME-UI`, `GAME-BENCHMARKS` to their domain authority, with stable evidence IDs and links into `GAME-EVIDENCE.md`.

### 2. Migrate proof + record mapping

Migrate the benchmark workload IDs, the `GAME-UI` no-leak matrix, and the strategy/level declarations into `GAME-EVIDENCE.md`, and record the `B-NN → template` mapping per template.

## Files to Touch

- `templates/COMPETENT-PLAYER.md` (modify)
- `templates/BOT-STRATEGY-EVIDENCE-PACK.md` (modify)
- `templates/GAME-UI.md` (modify)
- `templates/GAME-BENCHMARKS.md` (modify)

## Out of Scope

- The rules/coverage cluster (ticket 017), the ledger (019), the receipt conversions (020), and `AGENT-TASK.md` (021).
- Authoring the `GAME-EVIDENCE.md` receipt (ticket 012).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -lF "GAME-EVIDENCE" templates/COMPETENT-PLAYER.md templates/BOT-STRATEGY-EVIDENCE-PACK.md templates/GAME-UI.md templates/GAME-BENCHMARKS.md` lists all four.
2. The `GAME-UI` hidden-info safeguards matrix and the shared benchmark workload IDs are present in `GAME-EVIDENCE.md` (`grep -niE "no-leak|benchmark workload" templates/GAME-EVIDENCE.md`).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. No required-proof field is deleted without migration to a named owner in the same change.
2. No §11 invariant is waived by the slim.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (receipt-link grep, migration grep, link check) named in Assumption Reassessment.`

### Commands

1. `grep -lF "GAME-EVIDENCE" templates/COMPETENT-PLAYER.md templates/BOT-STRATEGY-EVIDENCE-PACK.md templates/GAME-UI.md templates/GAME-BENCHMARKS.md`
2. `node scripts/check-doc-links.mjs`
3. The receipt-link grep + link check is the correct boundary; migrated-proof presence is grep-checked on the receipt side.

## Outcome

Completed: 2026-06-22

Slimmed the bot/strategy/UI/benchmark template cluster by adding
`GAME-EVIDENCE.md` receipt links, domain-owner notes, and report mapping notes:

- `B-10 -> COMPETENT-PLAYER.md`: human-readable strategy authority; routed bot
  level, policy ID, benchmark workload, and release status to `GAME-EVIDENCE.md`.
- `B-11 -> BOT-STRATEGY-EVIDENCE-PACK.md`: Level 2 policy translation
  authority; replaced repeated generic bot exclusions with an `AI-BOTS.md`
  reference plus per-game compliance statement and linked benchmark IDs to the
  receipt.
- `B-13 -> GAME-UI.md`: game-specific UI delta authority; replaced the detailed
  hidden-info safeguards matrix with UI no-leak evidence IDs linked to the
  `GAME-EVIDENCE.md` viewer matrix.
- `B-14 -> GAME-BENCHMARKS.md`: benchmark workload authority; added stable
  workload IDs, fixture/profile anchors, and compatibility rows that link to the
  receipt rollup.

Verification:

- `grep -lF "GAME-EVIDENCE" templates/COMPETENT-PLAYER.md templates/BOT-STRATEGY-EVIDENCE-PACK.md templates/GAME-UI.md templates/GAME-BENCHMARKS.md`
  listed all four templates.
- `grep -niE "no-leak|benchmark workload" templates/GAME-EVIDENCE.md` returned
  the receipt-side no-leak and benchmark workload owner surfaces.
- `grep -niE "B-10|B-11|B-13|B-14|BENCH-|LEAK-UI|bot level|policy" templates/COMPETENT-PLAYER.md templates/BOT-STRATEGY-EVIDENCE-PACK.md templates/GAME-UI.md templates/GAME-BENCHMARKS.md`
  returned the B-ID mappings and evidence/workload/policy fields.
- `test -f templates/GAME-EVIDENCE.md` passed.
- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).

Deviation: `GAME-AI.md` is intentionally left for ticket 020; this ticket only
updated the four listed bot/strategy/UI/benchmark templates.
