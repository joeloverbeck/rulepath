# GAT17VOWTIDOHHEL-009: Exact-contract scoring, schedule progression, terminal standings, outcome model

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `games/vow_tide/src/scoring.rs`; modifies `state.rs`, `effects.rs`; new scoring/terminal golden traces
**Deps**: 008

## Problem

After each hand, score the exact contract (`10+bid` on exact, `0` on miss), record the hand breakdown, advance dealer + schedule, and at schedule end produce Rust-authored competition-ranked standings with co-winners and a seat-keyed outcome model. All scoring/standings facts come from Rust.

## Assumption Reassessment (2026-06-21)

1. `games/vow_tide/src/state.rs` (008) holds per-seat trick counts and bids after the last trick; this ticket adds `scoring.rs` and the hand-resolution + terminal transition. Sibling `games/briar_circuit/src/scoring.rs` is the structural precedent for a game-local scorer + outcome model.
2. Spec §3.1 + Appendix B.1.1/B.6 + `VT-SCORE/HAND-ADVANCE/TERMINAL/STANDINGS-001` fix exact-or-zero scoring, the atomic hand-result-then-advance order, the fixed-schedule terminal, and competition ranking (`1,1,3…`) with stable seat order for serialization only (never a tiebreak).
3. Cross-artifact boundary: the outcome model (B.6) is the contract consumed by the shared outcome surface (021/018) and `check-outcome-explanations`; the per-hand immutable summary + cumulative score are under audit.
4. FOUNDATIONS §2 under audit: Rust authors all score/rank/co-winner facts; the TypeScript outcome surface renders the supplied ranking and must not re-sort by a client score calculation.
5. Determinism enforcement surface: scoring/standings are deterministic functions of bids + trick counts (no RNG, no wall-clock); the terminal is reached only by the fixed schedule, never by a score target or action cap.

## Architecture Check

1. A dedicated `scoring.rs` plus an explicit hand-resolution transition keeps the exact-or-zero rule and the terminal standings in one auditable place, with no formula in static data.
2. No shims; new scorer module.
3. `engine-core` untouched; no `game-stdlib` change (scoring is game-local policy).

## Verification Layers

1. Exact `10+bid` / miss `0` (incl. successful zero = 10) → `cargo test -p vow_tide --test rules` + score traces.
2. Cumulative scores never decrease; hand result recorded atomically before advance → property + transition-order test.
3. Fixed terminal + competition-ranked co-winners → terminal unique/tie traces.
4. Outcome model viewer-safe + deterministic → serialization test; `check-outcome-explanations` (after 017/018/021 surfaces land).

## What to Change

### 1. Scoring + hand transition

`scoring.rs`: exact-or-zero per-hand addition; immutable completed-hand summary (bid, tricks, exact/miss, addition, cumulative before/after, dealer, trump). State transition: record result atomically, then rotate dealer + advance schedule + deal next hand, or enter `Terminal`.

### 2. Terminal standings + outcome model

Construct competition-ranked standings (co-winners share rank 1; stable seat order for display only) and the B.6 seat-keyed outcome model (rank, winner/co-winner, hands, exact made/missed, successful zeros, totals, per-hand rows, decisive public facts). Emit hand-result/score/schedule/terminal effects in stable order.

## Files to Touch

- `games/vow_tide/src/scoring.rs` (new)
- `games/vow_tide/src/state.rs` (modify)
- `games/vow_tide/src/effects.rs` (modify)
- `games/vow_tide/tests/golden_traces/exact-zero-scores-ten.trace.json` (new)
- `games/vow_tide/tests/golden_traces/terminal-co-winners-competition-rank.trace.json` (new)

## Out of Scope

- Visibility/effect filtering (010), replay export (011), bots (012).
- The TypeScript outcome rendering (018) and `UI.md` outcome section (021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide --test rules` — exact/under/over/zero scoring + terminal.
2. `cargo test -p vow_tide --test property` — cumulative monotonicity, competition ranking, atomic advance.
3. `cargo test -p vow_tide --test serialization` — outcome-model round-trip.

### Invariants

1. The match ends only after the fixed schedule; equal top scores co-win with competition ranks.
2. All score/rank/co-winner facts originate in Rust; no client re-ranking is possible.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/tests/rules.rs` — score exact/miss; `property.rs` — ranking/monotonicity.
2. `games/vow_tide/tests/golden_traces/{exact-positive-scores-ten-plus-bid,underbid-scores-zero,overbid-scores-zero,hand-score-dealer-and-size-advance,terminal-unique-high-score}.trace.json`.

### Commands

1. `cargo test -p vow_tide --test rules --test property --test serialization`
2. `cargo test -p vow_tide`
3. Narrower command rationale: scoring/terminal are native-deterministic; the outcome-explanation checker runs once the web/doc surfaces land (018/021).
