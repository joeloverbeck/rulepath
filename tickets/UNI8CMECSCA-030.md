# UNI8CMECSCA-030: Seed four bounded C-11 follow-on rows in `specs/README.md`

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — docs/index (`specs/README.md`)
**Deps**: UNI8CMECSCA-028, UNI8CMECSCA-029

## Problem

8C deliberately does not execute the C-11 retrofits, but it must seed them so every remaining game audit is owned before Gate 18. This ticket adds four bounded C-11 follow-on rows (8C-R1…R4) to `specs/README.md` with exact game sets, applicable-helper audits, hash/visibility obligations, rollback, and admission conditions, and updates the Gate 18 interlock to require those rows closed / not-applicable / accepted-excepted. It writes no game retrofit and executes nothing — the rows stay `Not started`.

## Assumption Reassessment (2026-06-22)

1. `specs/README.md` carries the unit index; row 8C is `Planned` after UNI8CMECSCA-001; the Gate 18 row currently interlocks on 8C scaffolding debt. The index does not yet have 8C-R1…R4 rows (confirmed by grep at the reassessed commit).
2. Spec §5 "Required C-11 forward-wave seeds" + §10.B fix the decomposition: **8C-R1** public/fixed-seat (`race_to_n` residual, `draughts_lite` residual, `three_marks`, `column_four`, `directional_flip`, `token_bazaar`); **8C-R2** two-seat hidden/reaction (`high_card_duel` residual, `secret_draft`, `poker_lite`, `masked_claims`); **8C-R3** public/co-op/asymmetric/trick (`plain_tricks`, `flood_watch`, `frontier_control`, `event_frontier`); **8C-R4** N-seat/private/trick (`river_ledger` residual, `briar_circuit`, `vow_tide`). This assigns every one of the 17 official game crates exactly once; pilot games are residual-only.
3. Cross-artifact boundary under audit: the `specs/README.md` index and its Gate 18 interlock. Exact unit labels are one-line-correctable; the bounded game sets and evidence rules are not.
4. FOUNDATIONS §4/§11/§12: every official game lands in exactly one bounded follow-on audit (no unowned "remaining cleanup" bucket); Gate 18 stays `Not started` until those rows close or are accepted-excepted and the partnership/trick atlas interlock resolves.
5. Determinism/no-leak (§11): a docs/index-only change; it seeds rows and admission conditions and touches no code/byte/fixture.

## Architecture Check

1. Seeding four bounded rows (vs. one open-ended cleanup unit) preserves exact per-game ownership and keeps each future migration a reviewable diff.
2. No backwards-compatibility shim — index rows; nothing aliased.
3. `engine-core`/`game-stdlib` untouched; the rows describe future work without authoring it.

## Verification Layers

1. Four rows 8C-R1…R4 present, each with game set, applicable-helper audit, hash/visibility, rollback, admission/exit shape → grep-proof on `specs/README.md`.
2. Every official game appears exactly once across the four rows; pilots marked residual-only → manual cross-check against the 17 `games/*` dirs + grep.
3. Gate 18 interlock updated to require the rows closed/N-A/excepted; Gate 18 stays `Not started` → grep-proof.
4. Doc links resolve → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `specs/README.md` — four C-11 follow-on rows

Insert 8C-R1…R4 after 8C and before Gate 18, each `Not started`, with the exact game sets above, applicable-helper audit (C-01…C-08), hash/visibility/rollback obligations, and admission/exit conditions (audit-only for residual pilots; one surface per diff or accepted not-applicable/exception).

### 2. `specs/README.md` — Gate 18 interlock

Update the Gate 18 row to require the four rows closed / explicitly not-applicable / accepted-excepted (plus the existing partnership/trick atlas interlock); keep Gate 18 `Not started`.

## Files to Touch

- `specs/README.md` (modify)

## Out of Scope

- Writing or executing any C-11 game retrofit.
- Flipping 8C to `Done` (UNI8CMECSCA-031).
- Editing `docs/ROADMAP.md` (spec §9.22).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -nE '8C-R[1-4]' specs/README.md` shows four rows, each `Not started` with game set + audit + hash/visibility + rollback + admission.
2. Every game under `games/*` appears in exactly one C-11 row (cross-checked); pilots are residual-only.
3. `node scripts/check-doc-links.mjs` passes; the Gate 18 row stays `Not started` with the updated interlock.

### Invariants

1. The four rows partition all 17 official games exactly once; no unowned cleanup bucket exists.
2. No game retrofit is authored or executed; rows stay `Not started`.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE '8C-R[1-4]' specs/README.md`
2. `node scripts/check-doc-links.mjs`
3. The index plus doc-link integrity is the correct boundary — the deliverable is seeded rows, not code.
