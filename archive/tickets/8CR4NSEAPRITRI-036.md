# 8CR4NSEAPRITRI-036: R4 register receipts and C-05/C-06/C-09/C-10 checkpoints

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — register receipts + governance checkpoints (`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, characterization report); no code/byte change
**Deps**: 8CR4NSEAPRITRI-002, 8CR4NSEAPRITRI-003, 8CR4NSEAPRITRI-004, 8CR4NSEAPRITRI-005, 8CR4NSEAPRITRI-006, 8CR4NSEAPRITRI-007, 8CR4NSEAPRITRI-008, 8CR4NSEAPRITRI-009, 8CR4NSEAPRITRI-010, 8CR4NSEAPRITRI-011, 8CR4NSEAPRITRI-012, 8CR4NSEAPRITRI-013, 8CR4NSEAPRITRI-014, 8CR4NSEAPRITRI-015, 8CR4NSEAPRITRI-016, 8CR4NSEAPRITRI-017, 8CR4NSEAPRITRI-018, 8CR4NSEAPRITRI-019, 8CR4NSEAPRITRI-020, 8CR4NSEAPRITRI-021, 8CR4NSEAPRITRI-022, 8CR4NSEAPRITRI-023, 8CR4NSEAPRITRI-024, 8CR4NSEAPRITRI-025, 8CR4NSEAPRITRI-026, 8CR4NSEAPRITRI-027, 8CR4NSEAPRITRI-028, 8CR4NSEAPRITRI-029, 8CR4NSEAPRITRI-030, 8CR4NSEAPRITRI-031, 8CR4NSEAPRITRI-032, 8CR4NSEAPRITRI-033, 8CR4NSEAPRITRI-034, 8CR4NSEAPRITRI-035

## Problem

After the migration tickets land, the register must carry one R4 receipt per aggregate/sub-surface disposition, and the C-05/C-06/C-09/C-10 checkpoints must be recorded without any source change (MSC-8C, spec §3.7 C-05 adjacent, §3.10 C-06/C-09/C-10, §5.6 task 431, §5.7 task 501, §5.10 tasks 801/802, §5.11 task 901, §7.5). This ticket adds R4 receipt rows under `MSC-8C-001…010`, the adjacent C-05 exception receipts, the C-06 reverse-edge proof, the Briar/Vow C-09 legacy-sampler N/A receipts, and the three C-10 local-behavior bundles — no new helper entry.

## Assumption Reassessment (2026-06-24)

1. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` carries entries `MSC-8C-001…010` (confirmed during `/reassess-spec`); `games/{briar_circuit,vow_tide}/src/setup.rs::shuffle_deck` use legacy `DeterministicRng::next_index` (modulo) and `games/river_ledger/src/setup.rs::shuffle_deck` uses `next_index_unbiased_v1` (directly confirmed); all three games declare `game-test-support` only under `[dev-dependencies]`.
2. Spec §3.10 records C-09 Briar/Vow as `not-applicable` to in-wave substitution (separate ADR-0009 trigger), River C-09 as pilot-discharged; C-10 reaffirms the complete local-only behavior bundles; the `game-stdlib::trick_taking` helper is atlas-owned and neither broadened nor reclassified.
3. Cross-artifact: this is the register-receipts governance ticket; it `Deps` every migration `-002`…`-035` for their after-evidence, and reads the `-001` baseline. It co-edits only the register + report, consolidating per-migration after-receipts here rather than each migration co-editing the report.
4. §4/§11 motivate this ticket: no `MSC-8C-*` helper contract is silently broadened, and no new register entry is invented to absorb game behavior; promoted primitives stay adopted/excepted.
5. Enforcement surface = the register before/after review + the C-06 inverse `cargo tree` proof + the Briar/Vow C-09 modulo `next_index` draw/index/deck/deal vectors; no source byte, RNG algorithm, or draw count changes.

## Architecture Check

1. Consolidating after-receipts into one register ticket is cleaner than every mutually-independent migration appending to the shared report/register (which would force a many-way merge conflict).
2. No backwards-compatibility shim is introduced; no helper contract is broadened. Rollback reverts only the R4 receipt additions.
3. `engine-core` stays noun-free (§3); `game-stdlib::trick_taking` is not reopened/broadened and not reclassified as scaffolding (§4).

## Verification Layers

1. One R4 receipt per migrated/pilot/N-A/exception surface, no unowned row -> manual review against spec §3.3–§3.10 and the migration tickets' after-evidence.
2. C-06 reverse-edge empty -> dependency-tree check (`cargo tree --workspace -e normal --invert game-test-support` and the normal,build variant).
3. C-09 Briar/Vow modulo vectors pinned + no helper broadened -> codebase grep-proof (register before/after; `next_index` unchanged) and FOUNDATIONS §4 alignment check.

## What to Change

### 1. Add R4 register receipts

Under existing `MSC-8C-001…010` entries in `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, add an R4 row for every migrated surface (game, sites, helper, characterization evidence, equality/ADR class, tests, rollback), every pilot-credit surface (naming the original `UNI8CMECSCA-*` ticket), and every N/A/exception (owner, rationale, compatibility, rollback, next-review trigger), including the adjacent C-05 state/effect/view/replay/export/diagnostic exceptions.

### 2. Record C-06/C-09/C-10 checkpoints

Record the C-06 reverse-edge proof, the Briar/Vow C-09 legacy-modulo divergence receipts with the separate ADR-0009 next-review trigger, and the three games' C-10 local-behavior bundles — confirming no behavior owner moved and no behavioral helper was added/broadened.

## Files to Touch

- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (modify)
- `reports/8c-r4-n-seat-private-trick-scaffolding-characterization.md` (modify; consolidates after-receipts and checkpoint conclusions)

## Out of Scope

- The final command/diff audit and tracker `Done`-flip (`-037`).
- Any new helper entry or broadening of an `MSC-8C-*` contract.
- Any RNG algorithm change, source byte change, or golden regeneration.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo tree --workspace -e normal --invert game-test-support` and `cargo tree --workspace -e normal,build --invert game-test-support` report no reverse production/build edge.
2. `cargo test --workspace` is green and `bash scripts/boundary-check.sh` + `node scripts/check-doc-links.mjs` pass.
3. Manual review confirms every §3 aggregate/sub-surface cell has exactly one R4 receipt with no unowned row.

### Invariants

1. No `MSC-8C-*` helper contract is broadened and no new register entry absorbs game behavior.
2. The Briar/Vow C-09 cells close as reviewed N/A with a separately-versioned ADR-0009 next-review trigger; River C-09 stays pilot credit.

## Test Plan

### New/Modified Tests

1. `None — governance/register documentation ticket; verification is the command set above plus manual receipt review, with the migrations' own tests as the regression guard.`

### Commands

1. `cargo tree --workspace -e normal --invert game-test-support`
2. `cargo test --workspace`
3. `node scripts/check-doc-links.mjs` (register/report links resolve); manual receipt-completeness review is the narrower correctness boundary.

## Outcome

Completed: 2026-06-24

What changed:
- Added R4 receipt rows under existing `MSC-8C-001` through `MSC-8C-010` register entries, covering migrated surfaces, pilot-credit rows, N/A dispositions, exceptions, rollback notes, and next-review triggers.
- Updated the characterization report with the consolidated R4 after-receipt inventory plus C-05 adjacent-byte, C-06 reverse-edge, C-09 sampler, and C-10 local-behavior checkpoint conclusions.
- Confirmed no new helper entry was introduced and no `MSC-8C-*` helper contract was broadened.

Deviations:
- None.

Verification:
- `cargo tree --workspace -e normal --invert game-test-support`
- `cargo tree --workspace -e normal,build --invert game-test-support`
- `cargo test --workspace`
- `bash scripts/boundary-check.sh`
- `node scripts/check-doc-links.mjs`
