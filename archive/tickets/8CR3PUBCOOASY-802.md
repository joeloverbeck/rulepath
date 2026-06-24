# 8CR3PUBCOOASY-802: R3 register receipts and checkpoint matrix

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None (governance/register — `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`)
**Deps**: 8CR3PUBCOOASY-101, 8CR3PUBCOOASY-102, 8CR3PUBCOOASY-103, 8CR3PUBCOOASY-104, 8CR3PUBCOOASY-105, 8CR3PUBCOOASY-201, 8CR3PUBCOOASY-202, 8CR3PUBCOOASY-301, 8CR3PUBCOOASY-302, 8CR3PUBCOOASY-303, 8CR3PUBCOOASY-304, 8CR3PUBCOOASY-305, 8CR3PUBCOOASY-306, 8CR3PUBCOOASY-307, 8CR3PUBCOOASY-308, 8CR3PUBCOOASY-401, 8CR3PUBCOOASY-402, 8CR3PUBCOOASY-403, 8CR3PUBCOOASY-404, 8CR3PUBCOOASY-501, 8CR3PUBCOOASY-502, 8CR3PUBCOOASY-503, 8CR3PUBCOOASY-504, 8CR3PUBCOOASY-511, 8CR3PUBCOOASY-512, 8CR3PUBCOOASY-513, 8CR3PUBCOOASY-514, 8CR3PUBCOOASY-601, 8CR3PUBCOOASY-602, 8CR3PUBCOOASY-603, 8CR3PUBCOOASY-604, 8CR3PUBCOOASY-611, 8CR3PUBCOOASY-612, 8CR3PUBCOOASY-613, 8CR3PUBCOOASY-614, 8CR3PUBCOOASY-621, 8CR3PUBCOOASY-622, 8CR3PUBCOOASY-623, 8CR3PUBCOOASY-624, 8CR3PUBCOOASY-631, 8CR3PUBCOOASY-632, 8CR3PUBCOOASY-633, 8CR3PUBCOOASY-634, 8CR3PUBCOOASY-641, 8CR3PUBCOOASY-701, 8CR3PUBCOOASY-702, 8CR3PUBCOOASY-703

## Problem

Every R3 migration, exception, and N/A must be recorded as an append-only
receipt under the existing `MSC-8C-001…010` register entries, preserving the R1
and R2 receipt tables. This ticket appends the R3 receipts — including every
exception and not-applicable verdict that produced no code diff (private-effect
N/As, faction/role-order exceptions, range/ring N/As, Plain variant N/A,
Frontier C-07 N/A, the three seat-private export N/As, and Frontier C-09 N/A) —
and the C-06/C-09/C-10 checkpoint matrix with the C-10 non-promotion list.

## Assumption Reassessment (2026-06-24)

1. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` exists with `MSC-8C-001`
   (effect-envelope constructors), `MSC-8C-005` (stable-byte writer v1), and
   `MSC-8C-010` (behavioral-policy non-promotion, rejected/local-only) plus
   `MSC-8C-002…009`; final state records `001…009` accepted and `010` rejected.
   R1/R2 receipt tables are present and must be preserved.
2. Spec §4.3 maps each `MSC-8C-00N` to its required R3 receipt content; §3.10
   gives the C-06/C-09/C-10 checkpoint matrix; §5.15 task `8C-R3-802` scopes the
   register append. The N/A/exception receipts for tasks that produced no code
   (spec task 642's three seat-private N/As; C-03/C-07/C-09 N/As and exceptions)
   are recorded here.
3. Cross-artifact boundary under audit: the register is append-only governance;
   it must not weaken any existing decision or non-promotion boundary, and it
   records evidence produced by the migration tickets — it ships no code.
4. FOUNDATIONS §4/§10A + §11: C-10 stays rejected/local-only (no `game-stdlib`
   promotion; atlas debt remains `_None_`); every exception receipt names owner,
   compatibility, rollback/reversal, and next review trigger.
5. Enforcement surface: the appended R3 receipt tables and the C-10 non-promotion
   list; `node scripts/check-doc-links.mjs` for any new intra-doc links; no
   existing receipt or boundary is altered.

## Architecture Check

1. Appending R3 receipts under the existing entries (rather than parallel
   entries per shipped helper) keeps one register entry per shipped surface, as
   the spec requires; cleaner and non-duplicative.
2. No backwards-compatibility alias — append-only; R1/R2 rows untouched.
3. `engine-core`/`game-stdlib` untouched; C-10 records rejection of every
   behavior-bearing extraction (no promotion).

## Verification Layers

1. Receipt completeness -> manual + grep review: every `MSC-8C-001…010` carries
   its R3 receipt per spec §4.3; every exception/N/A from §3.4–§3.10 is recorded.
2. Non-promotion preserved -> `MSC-8C-010` stays rejected/local-only; atlas
   §10A `_None_` unchanged.
3. Doc integrity -> `node scripts/check-doc-links.mjs` passes; R1/R2 receipts
   intact (grep-proof their rows still present).

## What to Change

### 1. Append R3 receipt tables

Under each existing `MSC-8C-001…010` entry in
`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, append the R3 receipt content from
spec §4.3 (five C-01 migrations + three private N/As; Plain parser + four WASM
conformance exceptions; eight count predicates + variant/faction N/As; four
action-tree v1 receipts; writer-v1 + adjacent exceptions; four dev-only edges +
inverse-edge proof; the C-07 matrices + Frontier N/A + canary hygiene; twenty
profile decisions incl. three seat-private N/As; three sampler migrations +
Frontier C-09 N/A; the C-10 per-game rejected/local-only lists). Preserve R1/R2
tables.

## Files to Touch

- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (modify — append-only)

## Out of Scope

- Altering any R1/R2 receipt or any existing decision/boundary.
- Any code, test, fixture, or trace change.
- Promoting any helper into `game-stdlib` (C-10 stays rejected/local-only).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — passes.
2. Grep-proof: each `MSC-8C-001…010` carries an R3 receipt; R1/R2 rows still present.
3. `MSC-8C-010` remains rejected/local-only; `docs/MECHANIC-ATLAS.md` §10A still `_None_`.

### Invariants

1. The register is append-only; no existing decision or boundary is weakened.
2. Every exception receipt names owner/compatibility/rollback/next-trigger; every
   N/A names the missing surface and reopen condition.

## Test Plan

### New/Modified Tests

1. `None — governance/register ticket; verification is grep/doc-link based and the migration tickets supply the receipt evidence.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -nE "MSC-8C-0(0[1-9]|10)" docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
3. Doc-link + grep review is the correct boundary: this ticket ships only
   register prose recording evidence produced by the migration tickets.

## Outcome

- Appended R3 receipt tables under `MSC-8C-001` through `MSC-8C-010` in
  `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, preserving the existing R1/R2
  receipt rows and decisions.
- Recorded R3 migrations, exceptions, and N/A decisions, including C-06
  dev-only dependency receipts, C-08 seat-private N/As, C-09 Frontier Control
  RNG N/A, and C-10 rejected/local-only behavioral-policy receipts.
- Added a Unit 8C-R3 closeout note with the C-06/C-09/C-10 checkpoint matrix and
  confirmed `docs/MECHANIC-ATLAS.md` section 10A remains `Current debt: _None_`.
- Verification passed:
  - `node scripts/check-doc-links.mjs`
  - `grep -nE "MSC-8C-0(0[1-9]|10)" docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
  - `rg -n "R3 public cooperative/asymmetric trick receipts|Unit 8C-R3 closeout evidence|Current debt: _None_" docs/MECHANICAL-SCAFFOLDING-REGISTER.md docs/MECHANIC-ATLAS.md`
  - `git diff --check`
