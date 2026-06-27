# GAT20STACROSTA-020: Gate 20 capstone — exit-criteria verification and Done-flip

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — docs/status only (`specs/README.md`, spec Status)
**Deps**: GAT20STACROSTA-013, GAT20STACROSTA-016, GAT20STACROSTA-017, GAT20STACROSTA-018, GAT20STACROSTA-019

## Problem

Gate 20 is `Done` only when every §6 exit-criteria row has committed evidence. This capstone exercises the full acceptance suite the prior tickets composed, reconciles the `specs/README.md` index status, and flips the spec Status — introducing no new production logic.

## Assumption Reassessment (2026-06-27)

1. `specs/README.md` lists Gate 20 as order 11, Spec `_(seed; unwritten)_`, Status `Not started` (confirmed); this flips the Spec cell to the spec path and Status to `Done`, with completion date.
2. The spec's own §10 Documentation-updates assigns the `specs/README.md` row flip (Planned→Done) — so this capstone owns it, per the §Ticket-shapes Done-flip default.
3. Cross-artifact boundary: the capstone exercises every prior ticket's acceptance surface (rules/replay/visibility/bots/tools/wasm/web/benchmarks/governance) end-to-end; it modifies only the status-reconciliation surfaces, not upstream files.
4. §12 (stop conditions) motivates this ticket: it confirms no stop condition was crossed — `engine-core` noun-free, no TS legality, no hidden-info leak (perfect info), third-use ledger resolved, forward-v1 receipt present, no promotion debt — before declaring the gate done.

## Architecture Check

1. A verification-only capstone keeps the Done-flip gated on the full exit suite passing, with no new logic — the standard gate-closeout shape.
2. No backwards-compatibility shims.
3. `engine-core`/`game-stdlib` untouched; status/docs only.

## Verification Layers

1. Native suite -> `cargo test -p starbridge_crossing` + `cargo test -p wasm-api`.
2. Evidence tools -> `replay-check`/`fixture-check`/`rule-coverage` + `cargo bench -p starbridge_crossing` all green across `{2,3,4,6}`.
3. CI/catalog/governance gates -> `check-ci-games` + `check-catalog-docs` + `check-scaffolding-governance` + `check-outcome-explanations` + `boundary-check` green.
4. Web acceptance -> `npm --prefix apps/web run smoke:e2e` + `build`.
5. Status integrity -> grep-proof: `specs/README.md` Gate 20 row reads `Done` with the spec path; spec Status `Done`.

## What to Change

### 1. Run + record the §7 command suite

Execute the full acceptance suite (re-run, not copied) and record pass status for each §6 exit-criteria row.

### 2. Flip status

`specs/README.md` Gate 20 row → spec path + `Done` (+ completion date); spec `Status` → `Done`.

## Files to Touch

- `specs/README.md` (modify)
- `specs/gate-20-starbridge-crossing-star-halma.md` (modify; Status field only)

## Out of Scope

- Any production code, doc content, or test authoring (those landed in prior tickets; this capstone only verifies + reconciles status).
- Archival move (follow `docs/archival-workflow.md` separately if/when convention dictates).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p starbridge_crossing && cargo test -p wasm-api`
2. `cargo run -p replay-check -- --game starbridge_crossing --all && cargo run -p fixture-check -- --game starbridge_crossing && cargo run -p rule-coverage -- --game starbridge_crossing`
3. `node scripts/check-ci-games.mjs && node scripts/check-catalog-docs.mjs && node scripts/check-scaffolding-governance.mjs && bash scripts/boundary-check.sh && npm --prefix apps/web run smoke:e2e`

### Invariants

1. Every §6 exit-criteria row has committed, re-run evidence before the Done-flip.
2. No §12 stop condition was crossed; `specs/README.md` and the spec Status read `Done`.

## Test Plan

### New/Modified Tests

1. `None — verification-only capstone; it exercises the acceptance suite composed by prior tickets and adds no tests.`

### Commands

1. `cargo test -p starbridge_crossing && cargo test -p wasm-api`
2. `cargo run -p replay-check -- --game starbridge_crossing --all && cargo run -p rule-coverage -- --game starbridge_crossing && npm --prefix apps/web run smoke:e2e`
3. The full cross-tool + web suite is the correct boundary for a gate closeout; it composes every prior ticket's evidence without modifying them.

## Outcome

Completed 2026-06-27. Re-ran the Gate 20 capstone acceptance suite, flipped the
Gate 20 progress row to `Done`, archived the completed spec, and recorded the
spec Outcome. No production logic was changed.

Deviations from the original narrow files list:

1. `cargo run -p rule-coverage -- --game starbridge_crossing` initially failed
   because `SC-END-*` and `SC-SCORE-*` alias rule IDs existed in `RULES.md`
   without rows in `RULE-COVERAGE.md`. The closeout repaired those alias rows
   and reran the command successfully.
2. Series-level closeout required archiving the spec despite this ticket's
   local archival out-of-scope note.
3. Active Starbridge docs that still pointed at the live spec path or pending
   GAT20STACROSTA-019/GAT20STACROSTA-020 receipt status were reconciled to the
   archived spec and completed receipt status.

Verification:

1. `cargo test -p starbridge_crossing`
2. `cargo test -p wasm-api`
3. `cargo run -p simulate -- --game starbridge_crossing --seat-count 2 --games 100 --start-seed 20`
4. `cargo run -p simulate -- --game starbridge_crossing --seat-count 3 --games 100 --start-seed 20`
5. `cargo run -p simulate -- --game starbridge_crossing --seat-count 4 --games 100 --start-seed 20`
6. `cargo run -p simulate -- --game starbridge_crossing --seat-count 6 --games 100 --start-seed 20`
7. `cargo run -p replay-check -- --game starbridge_crossing --all`
8. `cargo run -p fixture-check -- --game starbridge_crossing`
9. `cargo run -p rule-coverage -- --game starbridge_crossing`
10. `cargo bench -p starbridge_crossing`
11. `node scripts/check-ci-games.mjs`
12. `node scripts/check-catalog-docs.mjs`
13. `node scripts/check-scaffolding-governance.mjs`
14. `node scripts/check-outcome-explanations.mjs`
15. `node scripts/check-doc-links.mjs`
16. `bash scripts/boundary-check.sh`
17. `npm --prefix apps/web run build`
18. `npm --prefix apps/web run smoke:e2e`
19. `git diff --check`
