# RIVLEDSHO-002: Reveal-scoped projection of explanation fields + no-leak tests

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/src/visibility.rs`, `games/river_ledger/tests/visibility.rs`, no-leak golden traces
**Deps**: RIVLEDSHO-001

## Problem

The explanation fields from RIVLEDSHO-001 must be projected to each viewer exactly like the existing showdown reveal: only showdown-eligible revealed seats carry a hand explanation; folded and non-revealed seats carry none, and no explanation string names an unauthorized card. This ticket wires the fields through `visibility.rs` and proves the no-leak firewall holds across 3–6 seats (spec WB2 / D1).

## Assumption Reassessment (2026-06-15)

1. Verified against current code: `visibility.rs:81-99` defines `OutcomeRationaleView`, `SeatOutcomeBreakdownView`, and `ShowdownStrengthView`, which already project `category`/`tie_break_vector`/`best_five` for authorized showdown reveals; this ticket projects RIVLEDSHO-001's additive fields through the same authorized path.
2. Verified against specs/docs: spec §6 D1 + §8 WB2; `RULES.md` `RL-VIS-SHOWDOWN-001` (authorized showdown reveal), `RL-VIS-FOLDOUT-001` (foldout redaction), `RL-UI-NOLEAK-001`.
3. Cross-artifact boundary under audit: `visibility.rs` projections consume RIVLEDSHO-001's view fields; the existing pairwise no-leak harness (`games/river_ledger/tests/visibility.rs`, precedent `archive/tickets/GAT15RIVLEDTEX-009`) is extended to assert no explanation field reaches an unauthorized viewer.
4. FOUNDATIONS §11 acceptance invariant (pairwise seat-private redaction) motivates this ticket: an explanation private to a revealed contender must not expose another seat's hole cards, and folded seats get no explanation at all.
5. §11 no-leak firewall is the enforcement surface: assert every ordered seat pair in `N ∈ {3,4,5,6}`, the public observer, and foldout terminals expose no explanation naming an unauthorized/unrevealed card; folded/non-revealed seats carry `None` for the explanation fields. Confirm the harness exercises every ordered pair, not a sample.
6. Extends the public/private view projection (additive); the projected fields are the same ones RIVLEDSHO-001 added, gated by the existing reveal-authorization predicate.

## Architecture Check

1. Reusing the existing reveal-authorization predicate (the one that already gates `category`/`best_five`) for the new fields keeps a single authorization seam — no second, drifting leak boundary.
2. No backwards-compatibility aliasing/shims; projection is additive and gated.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4) — game-local projection + tests.

## Verification Layers

1. Revealed contenders carry the explanation fields; folded/non-revealed seats carry none -> `games/river_ledger/tests/visibility.rs` reveal-scope test.
2. No explanation field reaches any unauthorized viewer in every ordered pair (3/4/5/6) + observer + foldout -> pairwise no-leak sweep in `tests/visibility.rs`.
3. Viewer-scoped projections remain deterministic and redaction-stable -> no-leak golden traces via `cargo run -p replay-check -- --game river_ledger`.

## What to Change

### 1. `games/river_ledger/src/visibility.rs`

Project RIVLEDSHO-001's explanation fields through the showdown rationale views, gated by the existing reveal-authorization predicate; folded/non-revealed seats project `None`.

### 2. `games/river_ledger/tests/visibility.rs`

Extend the pairwise no-leak harness to assert the explanation fields are present only for authorized reveals and never name an unauthorized card; add foldout and observer assertions.

## Files to Touch

- `games/river_ledger/src/visibility.rs` (modify)
- `games/river_ledger/tests/visibility.rs` (modify)
- `games/river_ledger/tests/golden_traces/*.trace.json` (modify — no-leak/showdown traces where projected JSON gains fields)

## Out of Scope

- The explanation builder itself (RIVLEDSHO-001).
- WASM bridge / TypeScript types (RIVLEDSHO-003).
- Browser DOM/storage/log no-leak (RIVLEDSHO-005 e2e).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger --test visibility` — reveal-scope + pairwise/observer/foldout no-leak over the new fields.
2. `cargo test -p river_ledger` — full crate green.
3. `cargo run -p replay-check -- --game river_ledger` — viewer-scoped projections stay deterministic.

### Invariants

1. Explanation fields exist only for showdown-eligible revealed seats; folded/non-revealed seats carry none (§11, `RL-VIS-FOLDOUT-001`).
2. No explanation field names a card unauthorized for the viewer in any ordered pair (§11 no-leak firewall).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/visibility.rs` — reveal-scope + no-leak sweep over the explanation fields.
2. `games/river_ledger/tests/golden_traces/*.trace.json` — projected no-leak trace updates.

### Commands

1. `cargo test -p river_ledger --test visibility`
2. `cargo test -p river_ledger`
3. The crate-level visibility test is the correct boundary; browser no-leak is exercised in RIVLEDSHO-005.
