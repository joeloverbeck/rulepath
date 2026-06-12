# GAT14EVEFROEVE-002: Primitive-pressure ledger and mechanic-atlas reviews

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — documentation only (`games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md`, `docs/MECHANIC-ATLAS.md`)
**Deps**: None

## Problem

FOUNDATIONS §4 makes the third official use of a mechanic shape a **hard gate**: the game must not proceed until the primitive-pressure ledger decides reuse / promote / defer-reject / ADR. Event Frontier introduces op funding, pass income, and Reckoning income — making it the **third** public economy/accounting use after `token_bazaar` (first) and `poker_lite` (second), a genuine hard gate. It is also a candidate **third** use of multi-action turn budgets (`flood_watch` first, `frontier_control` second), expected to resolve as a **non-use**. This ledger records both hard-gate decisions, five second-use comparisons, three non-use reviews, the mandatory `game-stdlib::board_space` audit, and four new `local-only` first-use rows, and updates `docs/MECHANIC-ATLAS.md` accordingly — spec Work-breakdown item 1, which "blocks all implementation tasks."

## Assumption Reassessment (2026-06-12)

1. The atlas state this ledger updates is current: verified `docs/MECHANIC-ATLAS.md` has §10/§10A/§10B; §10A (open promotion-debt register) records `_None_`; §10B "public resource accounting / shared ledgers" records `token_bazaar` (first) + `poker_lite` (second); §10B "multi-action turn budgets" records `flood_watch` + `frontier_control`; the second-use comparison rows (`event-deck environment automation`, `role-modified action effects`, `graph-map topology`, `site control`, `faction-asymmetric action sets and scoring`) and the non-use rows (`reaction window`, `shared-outcome cooperative terminal`, `deterministic shuffle / private hand`) exist.
2. The `board_space` promoted primitive exists: verified `pub mod board_space;` in `crates/game-stdlib/src/lib.rs`; `docs/OFFICIAL-GAME-CONTRACT.md` §7 makes its applicability audit (or an accepted exception) mandatory for every new game.
3. Cross-artifact boundary under audit: the ledger is the §4 contract that gates tickets 003+; its recorded decisions (defer/reject for resource accounting; non-use for budgets) must hold, or the gate stops. The atlas rows it writes are the shared state every later atlas reader depends on; only ticket 002 writes atlas rows for this gate.
4. FOUNDATIONS §4 (`game-stdlib` is earned) and §11 (third-use pressure resolved before proceeding) motivate this ticket. Restated before trusting the spec: a *promote* outcome on the resource-accounting row would create promotion debt requiring migration of `token_bazaar`/`poker_lite` (§4) and likely an ADR (§13); the spec's expected outcome is defer/reject, leaving §10A `_None_`.
5. Third-use mechanic hard gate (§4): the ledger is the enforcement surface itself. Confirm the resource-accounting decision records rationale, risk, and next trigger; confirm the budget review records the non-use rationale; confirm no recorded outcome silently authorizes a `game-stdlib` promotion. No hidden information or replay/hash surface is touched (documentation only).

## Architecture Check

1. Recording every review in one ledger before code is cleaner than discovering a third-use pressure mid-implementation: it forces the defer/reject vs promote decision while the cost of choosing wrong is a doc edit, not a migration.
2. No backwards-compatibility aliasing/shims — new ledger document plus additive atlas rows.
3. Confirms `engine-core` stays noun-free and that no `game-stdlib` promotion is authorized: the only promotion the ledger may even consider is the resource-accounting row, expected defer/reject; all new shapes are recorded `local-only`.

## Verification Layers

1. Hard-gate decisions recorded (§4) -> manual review that `PRIMITIVE-PRESSURE-LEDGER.md` records both hard gates with rationale/risk/next-trigger, and FOUNDATIONS alignment check that the outcomes are defer/reject (resource accounting) and non-use (budgets).
2. Atlas consistency -> grep `docs/MECHANIC-ATLAS.md` that every touched row names `event_frontier` with the recorded use class and that §10A still reads `_None_`.
3. `board_space` audit present -> grep the ledger for the `board_space` applicability outcome (expected: not applicable — graph sites, no rectangular coordinates).
4. Doc-link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Author `games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md`

Instantiate from `templates/PRIMITIVE-PRESSURE-LEDGER.md`. Record: (a) **hard gate — public resource accounting (third use)**: decision defer/reject with rationale (per-faction op funding differs structurally from market purchase accounting and pot/showdown allocation), risk, and next trigger; (b) **hard-gate candidate — multi-action turn budgets**: non-use rationale (one choice per event card; an operation is one compound command, not budgeted sequence) with the re-arm trigger; (c) five second-use comparisons (event-deck vs `flood_watch`; graph-map topology, site control, faction-asymmetric sets/scoring vs `frontier_control`; role/faction modifiers vs `flood_watch`/`frontier_control`) kept local; (d) three non-use reviews (reaction window, shared-outcome cooperative terminal, deterministic shuffle/private hand); (e) the `board_space` audit (expected: not applicable); (f) four new `local-only` first-use rows (event-card initiative/eligibility sequencing; periodic scoring/reset pipeline; asymmetric instant victory; timed rule-exception modifiers).

### 2. Update `docs/MECHANIC-ATLAS.md`

Apply every touched §10/§10B row update named above; add the four new first-use rows `local-only`; confirm §10A stays `_None_` and §10A maintenance interlock is satisfied (predecessor Gate 13 left no promotion debt).

## Files to Touch

- `games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new)
- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- Any crate, code, or data scaffolding (ticket 003 onward) — this ticket is the gate that must clear first.
- Authorizing any `game-stdlib` promotion or extraction ticket: the expected outcomes are defer/reject and non-use, so no extraction ticket is spawned. Proceeding to author a `game-stdlib` helper if the decision were *promote* would itself require an ADR and is not in this gate's scope.
- `RULES.md`/`SOURCES.md` (ticket 001) and the remaining per-game docs.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the ledger present.
2. `grep -n "event_frontier" docs/MECHANIC-ATLAS.md` shows the resource-accounting third-use decision, the budget non-use, the second-use comparisons, and the four new first-use rows.
3. `grep -n "_None_" docs/MECHANIC-ATLAS.md` confirms §10A still records `_None_` (no promotion debt opened).

### Invariants

1. Both hard gates are decided (resource accounting: defer/reject; budgets: non-use), each with rationale, risk, and next trigger.
2. No atlas row authorizes a `game-stdlib` promotion for this gate; every new shape is recorded `local-only` first use.

## Test Plan

### New/Modified Tests

1. `None — documentation/ledger ticket; verification is command-based (doc-link check + atlas greps) and FOUNDATIONS §4 alignment review.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -nE "event_frontier|_None_" docs/MECHANIC-ATLAS.md`
3. A compile/test command is not the correct boundary — the ledger gates code that does not yet exist; the verification boundary is the atlas/ledger consistency greps plus the §4 alignment review.

## Outcome

Completed: 2026-06-12

What changed:

- Added `games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md` with the Gate 14 pre-implementation hard-gate decisions and comparison reviews.
- Updated `docs/MECHANIC-ATLAS.md` to record Event Frontier's public-resource-accounting defer/reject decision, multi-action-budget non-use, `board_space` not-applicable audit, second-use/comparison rows, non-use reviews, and four new `local-only` first-use rows.

Deviations from original plan:

- None. No code, data, helper promotion, ADR, or extraction ticket was introduced.

Verification:

- `node scripts/check-doc-links.mjs` passed (`Checked 25 markdown files`).
- `grep -nE "event_frontier|_None_" docs/MECHANIC-ATLAS.md` showed Event Frontier in the resource-accounting hard-gate rows, budget non-use row, second-use comparison rows, new first-use rows, and §10A `_None_`.
- `grep -nE "resource-accounting|resource accounting|defer/reject|multi-action|non-use|board_space|local-only|promotion debt|_None_" games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md docs/MECHANIC-ATLAS.md` confirmed the ledger/atlas hard-gate and no-promotion-debt language.
