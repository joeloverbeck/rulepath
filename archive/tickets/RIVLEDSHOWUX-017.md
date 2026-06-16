# RIVLEDSHOWUX-017: Closeout — docs reconciliation + spec/index `Done`-flip

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None
**Deps**: RIVLEDSHOWUX-002, RIVLEDSHOWUX-006, RIVLEDSHOWUX-010, RIVLEDSHOWUX-013, RIVLEDSHOWUX-015, RIVLEDSHOWUX-016

## Problem

Once the showcase workstreams land, the game docs and the spec index must be reconciled and the gate's exit criteria verified. This capstone reconciles `UI.md` and `RULE-COVERAGE.md` with the new surfaces, reconciles `apps/web/README.md` only if a surface name changed, runs the doc-link + catalog checks, verifies the spec's exit criteria end-to-end, and flips the `specs/README.md` index row `Planned → Done` with evidence.

## Assumption Reassessment (2026-06-16)

1. Verified: `games/river_ledger/docs/UI.md` and `RULE-COVERAGE.md` carry the V1 outcome/seat/action rows; `specs/README.md` lists this spec's non-gate row as `Planned`; `apps/web/README.md` carries the catalog/Shell Surface lists.
2. Verified against spec §9 Exit criteria + §12 Documentation updates + §8 WB17; this capstone exercises the surfaces composed by the leaf tickets (RIVLEDSHOWUX-002/006/010/013/015/016) whose transitive `Deps` cover all 16 prior tickets.
3. Cross-artifact boundary under audit: the status-reconciliation surfaces (`UI.md`, `RULE-COVERAGE.md`, `apps/web/README.md`, `specs/README.md`) are this capstone's own; it does NOT modify the upstream tickets' implementation files — it exercises and documents them.

## Architecture Check

1. A single trailing capstone for doc reconciliation + exit verification + `Done`-flip keeps the status reconciliation atomic once every surface exists; it introduces no production logic.
2. No shims; docs are reconciled to the shipped surfaces, not aliased.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4); docs/status only.

## Verification Layers

1. Exit criteria pass end-to-end -> the full re-run: `cargo test --workspace`, `cargo run -p {fixture-check,replay-check,rule-coverage} -- --game river_ledger`, `bash scripts/boundary-check.sh`, the `smoke:*` + e2e suite.
2. Docs reconciled and links resolve -> `node scripts/check-doc-links.mjs` + `node scripts/check-catalog-docs.mjs`.
3. Index row flipped `Done` with evidence -> grep `specs/README.md` for the row's `Done` status + evidence pointer.

## What to Change

### 1. `games/river_ledger/docs/UI.md` + `RULE-COVERAGE.md`

Reconcile outcome/seat/action/board/bot rows with the V2 panel, action-presentation rows, seat-ledger labels, board-slot view, and bot-explanation surface; update any `RL-UI-*` coverage rows (and add sub-rule IDs only if decomposition introduced them).

### 2. `apps/web/README.md`

Catalog / Shell Surface reconciliation only if a surface name changed (`check-catalog-docs.mjs` must stay green).

### 3. `specs/README.md`

Flip this spec's non-gate row `Planned → Done` with the acceptance-evidence pointer once exit criteria pass.

## Files to Touch

- `games/river_ledger/docs/UI.md` (modify)
- `games/river_ledger/docs/RULE-COVERAGE.md` (modify)
- `apps/web/README.md` (modify, only if a surface name changed)
- `specs/README.md` (modify)

## Out of Scope

- Any implementation surface owned by RIVLEDSHOWUX-001…016 (this capstone exercises, it does not modify them).
- Remove the RIVLEDSHOWUX-015 `Deps` entry if the bot "Why?" affordance is dropped per spec A5.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace` + `cargo run -p fixture-check -- --game river_ledger` + `cargo run -p replay-check -- --game river_ledger --all` + `cargo run -p rule-coverage -- --game river_ledger` + `bash scripts/boundary-check.sh` — all green.
2. `npm --prefix apps/web run build` + `npm --prefix apps/web run smoke:ui` + `smoke:wasm` + `smoke:effects` + `smoke:e2e` — all green.
3. `node scripts/check-doc-links.mjs` + `node scripts/check-catalog-docs.mjs` — pass; the `specs/README.md` row reads `Done` with evidence.

### Invariants

1. The spec's exit criteria (§9) are satisfied with re-run evidence before the index flips to `Done`.
2. Docs reflect the shipped surfaces; no implementation file is modified by this capstone (§6 evidence-heavy; docs/status only).

## Test Plan

### New/Modified Tests

1. `None — verification + docs-reconciliation capstone; it exercises the acceptance suite composed by prior tickets and reconciles docs/status, adding no test file.`

### Commands

1. `cargo test --workspace`
2. `npm --prefix apps/web run smoke:e2e`
3. `node scripts/check-doc-links.mjs` (the doc-link + catalog checks are the correct boundary for the docs/status closeout)

## Outcome

Completed the series capstone docs/status reconciliation for the River Ledger showcase UX workstream.

Updated `games/river_ledger/docs/UI.md` for the shipped V2 surfaces: Rust-authored Seat labels, action display rows, board-slot placeholders, seat-ledger display fields, V2 showdown panel, card-usage marks, live-region status, scheduler-routed River effects, compact bot `Why?`, and catalog identity. Updated `games/river_ledger/docs/RULE-COVERAGE.md` UI/bot rows to point at the new proof surfaces. Reconciled `apps/web/README.md` for the dedicated River catalog icon and River's authored animation adoption. Flipped `specs/README.md` to `Done` for `river-ledger-showcase-ux` with archived-ticket evidence.

Verification:

Rust and per-game:

1. `cargo test --workspace`
2. `cargo run -p fixture-check -- --game river_ledger`
3. `cargo run -p replay-check -- --game river_ledger --all`
4. `cargo run -p rule-coverage -- --game river_ledger`
5. `bash scripts/boundary-check.sh`

Docs:

1. `node scripts/check-doc-links.mjs`
2. `node scripts/check-catalog-docs.mjs`

Web:

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:wasm`
3. `npm --prefix apps/web run smoke:ui`
4. `npm --prefix apps/web run smoke:effects`
5. `npm --prefix apps/web run smoke:e2e`

`npm --prefix apps/web run smoke:effects` first passed its Node effect-feedback portion, then the browser animation segment failed to bind `127.0.0.1` with `listen EPERM` under the sandbox. The exact command was rerun with localhost/browser escalation and passed.

Reference closeout note: the series-level `ticket-series` workflow still requires archiving `specs/river-ledger-showcase-ux.md` after this capstone ticket commit.
