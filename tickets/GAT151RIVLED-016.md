# GAT151RIVLED-016: Pairwise N-seat no-leak proof

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/river_ledger/tests/visibility.rs`, `apps/web/e2e/a11y-noleak.smoke.mjs`
**Deps**: GAT151RIVLED-014

## Problem

Run the Infra D pairwise no-leak matrix for N=3,4,5,6 across seat A→seat B, self, and observer views, covering actions, previews, diagnostics, effects, bot rationale, replay, JSON, DOM, a11y, logs, storage, and dev surfaces. The new public stack/eligibility facts must be distinguished from prohibited card/deck/private-evaluation facts: adding public fields must provably not carry adjacent private structures.

## Assumption Reassessment (2026-06-20)

1. Code: `games/river_ledger/tests/visibility.rs` holds the base pairwise checks; `apps/web/e2e/a11y-noleak.smoke.mjs` is the existing browser no-leak smoke. Both extend to the new stack/all-in/pot/return/allocation surfaces from GAT151RIVLED-010/-013/-014.
2. Docs: spec §7.5 defines the matrix (every source seat A → every viewer B, self, observer; pre-action, post-action, all-in waiting, runout, showdown, foldout, replay-import states) and the per-surface intentionally-public vs. prohibited table.
3. Cross-artifact boundary under audit: the viewer-safe projection contract spanning Rust views (GAT151RIVLED-010), WASM JSON (-013), and DOM/a11y (-014) — the matrix proves redaction holds across all three lanes.
4. (§11 no-leak firewall) Restate: facts private to seat A (hole cards, deck tail, unrevealed board, internal evaluator data) must not reach seat B, the observer, DOM/storage/logs, bot explanations, replay exports, or test IDs. Public stack/eligibility are intentionally public; confirm adding them carries no adjacent private structure.

## Architecture Check

1. A single pairwise matrix over all N and all lifecycle states is the strongest proof that the new public fields did not open a private-data path.
2. No backwards-compatibility shims; this extends the Infra D harness in place.
3. No production logic changes; the firewall lives in the GAT151RIVLED-010 projections this ticket audits (§11).

## Verification Layers

1. Seat-A→seat-B / self / observer projections redact private cards/deck -> pairwise visibility tests for N=3..6.
2. Diagnostics/effects/bot rationale carry no hidden card/deck/candidate fields -> no-leak unit tests.
3. Browser DOM/a11y/logs/storage/test-ids carry no private payload -> `a11y-noleak.smoke.mjs` assertions.
4. Public stack/eligibility present without adjacent private structure -> targeted positive/negative assertions.

## What to Change

### 1. Rust pairwise matrix

Extend `tests/visibility.rs` with the N=3..6 A→B/self/observer matrix across all lifecycle states, asserting the §7.5 public-vs-prohibited table for actions, previews, diagnostics, effects, bot rationale, and replay/JSON.

### 2. Browser no-leak smoke

Extend `a11y-noleak.smoke.mjs` to assert DOM/a11y/logs/storage/test-id surfaces expose only viewer-authorized projection for an asymmetric multi-pot hand.

## Files to Touch

- `games/river_ledger/tests/visibility.rs` (modify)
- `apps/web/e2e/a11y-noleak.smoke.mjs` (modify)

## Out of Scope

- Golden traces (GAT151RIVLED-017) — though the public-observer and seat-private multi-pot no-leak traces there reuse this matrix's expectations.
- Any production projection change (owned by GAT151RIVLED-010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger visibility` — the full N=3..6 pairwise matrix passes.
2. `npm --prefix apps/web run smoke:e2e` — the a11y/no-leak browser smoke passes for an asymmetric multi-pot hand.
3. `cargo run -p rule-coverage -- --game river_ledger` — `RL-VIS-POT-001` maps to the matrix.

### Invariants

1. No fact private to one seat reaches another seat, the observer, DOM/storage/logs, bot explanations, replay exports, or test IDs.
2. Public stack/eligibility fields are present with no adjacent private (card/deck/evaluator) structure.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/visibility.rs` — N=3..6 pairwise matrix across lifecycle states.
2. `apps/web/e2e/a11y-noleak.smoke.mjs` — browser DOM/a11y/storage/log no-leak assertions.

### Commands

1. `cargo test -p river_ledger visibility`
2. `npm --prefix apps/web run smoke:e2e`
3. `cargo run -p rule-coverage -- --game river_ledger` — the visibility suite plus browser smoke are the correct no-leak boundary across Rust and DOM lanes.
