# 8CR1PUBFIXSEA-037: Register receipts and C-06/C-07/C-09/C-10 checkpoints

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — governance docs (`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, characterization report); no code/hash/visibility change
**Deps**: 8CR1PUBFIXSEA-002, 8CR1PUBFIXSEA-003, 8CR1PUBFIXSEA-004, 8CR1PUBFIXSEA-005, 8CR1PUBFIXSEA-006, 8CR1PUBFIXSEA-007, 8CR1PUBFIXSEA-008, 8CR1PUBFIXSEA-009, 8CR1PUBFIXSEA-010, 8CR1PUBFIXSEA-011, 8CR1PUBFIXSEA-012, 8CR1PUBFIXSEA-013, 8CR1PUBFIXSEA-014, 8CR1PUBFIXSEA-015, 8CR1PUBFIXSEA-016, 8CR1PUBFIXSEA-017, 8CR1PUBFIXSEA-018, 8CR1PUBFIXSEA-019, 8CR1PUBFIXSEA-020, 8CR1PUBFIXSEA-021, 8CR1PUBFIXSEA-022, 8CR1PUBFIXSEA-023, 8CR1PUBFIXSEA-024, 8CR1PUBFIXSEA-025, 8CR1PUBFIXSEA-026, 8CR1PUBFIXSEA-027, 8CR1PUBFIXSEA-028, 8CR1PUBFIXSEA-029, 8CR1PUBFIXSEA-030, 8CR1PUBFIXSEA-031, 8CR1PUBFIXSEA-032, 8CR1PUBFIXSEA-033, 8CR1PUBFIXSEA-034, 8CR1PUBFIXSEA-035, 8CR1PUBFIXSEA-036

## Problem

After every migration lands, R1 must record one register/report receipt per migration, not-applicability decision, and accepted exception, and complete the C-06/C-07/C-09/C-10 audit checkpoints — leaving no unnamed "remaining cleanup" bucket (spec §3.8, §4.3, §5.10 task `8C-R1-601`, EC-R1-18). This ticket writes those receipts into `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (under the relevant MSC-8C entries) and completes the append-only checkpoint conclusions in the characterization report. No code, hash, or visibility change.

## Assumption Reassessment (2026-06-23)

1. Every migration ticket (`-002`…`-036`) has landed its named surface; the characterization report (`reports/8c-r1-public-fixed-seat-scaffolding-characterization.md`, created by `-001`) carries the baselines this ticket closes out.
2. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` defines the receipt schema (game, surface, decision state, evidence link, hash/visibility impact, rollback, next review trigger) and entries MSC-8C-001…010; spec §4.3 and §5.10 own these receipts. Confirmed during reassessment.
3. Cross-artifact: the register and the characterization report are the two governance surfaces; this ticket reconciles their receipts against the landed migrations and the verdict matrix from `-001`.
4. §11/§12 motivate this ticket: the C-06 (dev-only dependency), C-07 (no-leak not-applicable), C-09 (no RNG consumption change), and C-10 (non-promotion) checkpoints are acceptance invariants that must be explicitly recorded, not assumed.
5. Enforcement surfaces audited: C-06 dev-only edge (`cargo tree` proof), C-07 no hidden source datum/viewer pair, C-09 unchanged RNG vectors/hashes, C-10 no behavioral promotion into shared code; recording these introduces no leak and no nondeterminism — it is documentation of the proofs the migration tickets already produced.
6. The register receipts use the existing MSC-8C entry schema additively (new bounded R1 adoption/exception rows under existing entries); consumers of the register are governance/audit readers and the boundary-check process — the extension is additive-only.

## Architecture Check

1. One reconciled receipt pass after all migrations is cleaner than scattering register edits across 35 migration diffs — it gives a single auditable exception ledger.
2. No backwards-compatibility shim is introduced; this ticket records decisions only.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4); C-10 audit affirms no behavioral promotion through the scaffolding lane.

## Verification Layers

1. C-06 dev-only dependency proof -> `cargo tree --workspace -e normal --invert game-test-support` shows no production/build path.
2. C-07 not-applicability + C-09 unchanged RNG -> characterization-report rationale + existing visibility/RNG tests stay green.
3. C-10 non-promotion + C-02/C-04/C-05 exception rows -> `bash scripts/boundary-check.sh` + register receipts (one per migration/N-A/exception) with owner, evidence, rollback, and next trigger.

## What to Change

### 1. Register receipts

In `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, add/update bounded R1 adoption receipts under the relevant MSC-8C-001…010 entries: each migration, each not-applicability decision, and each accepted exception (C-02 native `default_seats` / non-WASM trace seats; C-04 legacy-hash compatibility; C-05 adjacent state/effect/view/replay/export/diagnostic surfaces) with game, surface, decision state, evidence link, hash/visibility impact, rollback, and next review trigger.

### 2. Checkpoint conclusions

In the characterization report, record the C-06 dependency proof, C-07 not-applicability, C-09 no-consumption-change result, and C-10 non-promotion result.

## Files to Touch

- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (modify)
- `reports/8c-r1-public-fixed-seat-scaffolding-characterization.md` (modify; created by 8CR1PUBFIXSEA-001)

## Out of Scope

- Any code migration or hash/visibility change.
- Foundation-doc or ADR amendment (a discovered need is a blocking §8.4 trigger, not routine closeout).
- The §7 consolidated verification and the `Done`-flip (owned by `-038`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo tree --workspace -e normal --invert game-test-support` shows no production/build path (C-06).
2. `bash scripts/boundary-check.sh` passes (C-10 non-promotion; `engine-core` noun-free).
3. The register + report contain a receipt for every migration, not-applicability decision, and accepted exception, with no unresolved "remaining cleanup" row.

### Invariants

1. Every audited surface has exactly one recorded receipt; C-06/C-07/C-09/C-10 conclusions are explicit.
2. No code, hash, seat, or visibility byte changes in this ticket.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (cargo tree, boundary-check) and the migration tickets' suites are the regression guard.`

### Commands

1. `cargo tree --workspace -e normal --invert game-test-support`
2. `bash scripts/boundary-check.sh`
3. These two commands are the correct boundary: C-06 is a dependency-graph proof and C-10 is a boundary proof; no new test artifact is produced.
