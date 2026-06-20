# GAT151RIVLED-002: Primitive-pressure ledger decision

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None (docs-only: records the primitive-pressure decision in `PRIMITIVE-PRESSURE-LEDGER.md` + `MECHANIC-ATLAS.md`)
**Deps**: GAT151RIVLED-001

## Problem

FOUNDATIONS §4 makes the layered contribution-cap / per-pot-eligibility / allocation mechanic a hard gate: the game must not proceed until a primitive-pressure decision is recorded. This ticket writes the complete ledger entry (spec §8.4) and records the decision — default **reuse-local / defer-reject `game-stdlib` promotion, no `engine-core` change, no ADR** — so the side-pot constructor and allocator can be built game-local in later tickets.

## Assumption Reassessment (2026-06-20)

1. Code: `games/river_ledger/src/pot.rs` contains only `allocate_single_pot(...)` and `winners_in_button_order(winners, button, seat_count)`; no layered/side-pot helper exists in `games/river_ledger`, `crates/game-stdlib`, or `crates/engine-core`. `games/poker_lite` has single-pot split only — not the repeated nested-eligibility shape.
2. Docs: `docs/MECHANIC-ATLAS.md` §10A open promotion-debt register reads `None`; the River Ledger entry explicitly says to reopen for Gate 15.1 side-pot/all-in work; `games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md` exists and holds the base entry.
3. Cross-artifact boundary under audit: the mechanic-atlas third-use ledger (`docs/MECHANIC-ATLAS.md`) ↔ this game's `PRIMITIVE-PRESSURE-LEDGER.md`.
4. (§4 third-use hard gate) Enforcement surface is the atlas open-debt register (§10A). Confirm the decision leaves it empty: defer/reject means no promotion debt, no named games to retrofit, no closure gate.

## Architecture Check

1. Recording defer/reject keeps poker nouns (seat, contribution, fold, all-in, pot, eligibility, button, showdown) out of shared crates until a genuine third use; premature extraction would freeze an under-proven eligibility/allocation API.
2. No backwards-compatibility shims introduced.
3. `engine-core` stays noun-free; `game-stdlib` is not promoted — the rejection is logged with rationale per the mechanic atlas.

## Verification Layers

1. Ledger completeness -> manual review: every spec §8.4 field is populated.
2. Debt register stays empty -> codebase grep-proof on `docs/MECHANIC-ATLAS.md` §10A (`Current debt: None`).
3. §4 alignment -> FOUNDATIONS alignment check: third-use hard gate resolved before behavior tickets.

## What to Change

### 1. Pressure-ledger entry

Write the full §8.4 field set into `PRIMITIVE-PRESSURE-LEDGER.md`: mechanic shape, status, games exerting pressure, what is repeated vs. what differs, why local duplication is acceptable, decision (reuse local winner-order helper + implement layer constructor/allocator locally; defer/reject promotion; no ADR), why-not-engine-core, why-not-game-stdlib, data/Rust boundary, replay/hash/visibility/bot/UI impact, tests/benchmarks required, back-port/conformance (N/A), affected prior games (none), closure gate (N/A).

### 2. Atlas update

Record the Gate 15.1 pressure decision in `docs/MECHANIC-ATLAS.md` and confirm §10A open-debt register remains empty.

## Files to Touch

- `games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md` (modify)
- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- Any extraction to `game-stdlib` (this ticket records defer/reject; a `game-stdlib` extraction ticket would be added only if the decision were *promote* — not expected per spec §8.4).
- Any `src/` code change or test.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — ledger/atlas links resolve.
2. `node scripts/check-catalog-docs.mjs` — catalog docs remain consistent.
3. Manual: every §8.4 ledger field is populated and the decision row reads defer/reject with rationale.

### Invariants

1. `docs/MECHANIC-ATLAS.md` §10A open promotion-debt register remains empty.
2. No `engine-core` or `game-stdlib` change accompanies this decision.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `node scripts/check-catalog-docs.mjs`
3. Doc-gate commands are the correct boundary: no Rust changes, so `cargo` gates are not exercised by this ticket.

## Outcome

Completed: 2026-06-20

What changed:

- Added the Gate 15.1 side-pot/all-in primitive-pressure decision to `games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md`.
- Recorded the decision to reuse River Ledger's local button-order helper and implement contribution-layer construction/allocation locally, with no `game-stdlib` promotion, no `engine-core` vocabulary expansion, no ADR, and no promotion debt.
- Updated `docs/MECHANIC-ATLAS.md` to reflect the Gate 15.1 reopen/close decision while preserving `Current debt: _None_.`

Deviations:

- None. The ticket remained documentation-only; no Rust crate, shared helper, or test file changed.

Verification:

- `node scripts/check-doc-links.mjs` passed (`Checked 27 markdown files`).
- `node scripts/check-catalog-docs.mjs` passed (`catalog-docs check passed -- 15 games reflected in intro, root, and smoke surfaces`).
- Manual/codebase check confirmed `docs/MECHANIC-ATLAS.md` §10A still reads `Current debt: _None_.`
