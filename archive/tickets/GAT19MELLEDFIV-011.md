# GAT19MELLEDFIV-011: Round and match scoring — 500 target with unique-winner tie continuation (first-use primitive)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/meldfall_ledger/src/scoring.rs`; scoring golden traces; first-use ledger entry ML-PP-005
**Deps**: GAT19MELLEDFIV-007, GAT19MELLEDFIV-008, GAT19MELLEDFIV-010

## Problem

Meldfall Ledger needs Rust-owned scoring: each seat scores positive for the card values it played onto the public table (by `played_by` credit), minus the values of cards left in its hand at round end. Values: ace 15, K/Q/J/10 = 10, 2–9 pip. Round deltas add to cumulative match scores (which may go negative). The first seat to reach/exceed 500 after a round is eligible; the unique highest at/above 500 wins; if tied for highest, rounds continue until a unique winner exists. First official use of this rummy cumulative-scoring target (`ML-PP-005`, `local-only`).

## Assumption Reassessment (2026-06-25)

1. Card values live in `cards.rs` (GAT19MELLEDFIV-004); per-card `played_by` credit in the tableau (007/008); round-end transition in `rules.rs` (010). Cumulative-target patterns follow `games/blackglass_pact/src/scoring.rs` / `games/vow_tide/src/scoring.rs` (confirmed during reassessment) but the combination (tabled positive + in-hand penalty + laid-off credit + negative scores + tie continuation) is new.
2. Spec §3.1 (Scoring / Match-end rows), Appendix A.2 (Card-values / Round-scoring / Match-target rows), Appendix B.3 (score-credit formula), and Appendix D (`ML-PP-005`) define the model.
3. Cross-artifact: the score-credit ledger (`played_by` per `TableCard`) + the `cumulative_scores` vector in `MatchState` are the boundary; round score = sum(value where `played_by == seat`) − sum(value of held cards).
4. FOUNDATIONS §4: this rummy cumulative-scoring-to-500 pattern is a first official use, `local-only` (`ML-PP-005`); no helper promotion.
5. FOUNDATIONS §11 no-leak: public round settlement exposes per-seat positive totals, in-hand penalty totals/counts, round delta, and cumulative score — never opponents' exact unmelded card identities. Seat-private settlement may include the viewer's own remaining cards. The export enforcement is GAT19MELLEDFIV-013; this ticket keeps the public settlement output count/total-only.

## Architecture Check

1. Deriving round score from the per-card `played_by` ledger plus held-card values keeps scoring a pure function of state, making negative scores and tie continuation deterministic and testable.
2. No backwards-compatibility shims.
3. `engine-core` untouched; `ML-PP-005` is first-use local-only, not a promotion.

## Verification Layers

1. Card values + positive/negative round scoring correct -> `cargo test -p meldfall_ledger` scoring tests + `round-scoring-positive-negative.trace.json`.
2. Scores can go negative; cumulative across rounds -> `scores-can-go-negative.trace.json`.
3. 500 target + unique-winner tie continuation -> `multi-round-first-to-500.trace.json`, `target-tie-continues.trace.json`.

## What to Change

### 1. `scoring.rs` — round + match scoring

Card-value table application; per-seat round score = positive tabled (`played_by`) − in-hand penalties; cumulative match scores (may be negative); 500 eligibility; unique-highest-at/above-500 winner; tie -> continue rounds; per-seat terminal rankings.

### 2. Public settlement projection

Public round-settlement output: positive tabled total, in-hand penalty total/count, remaining-hand count, round delta, cumulative, terminal rank — no opponent card identities.

### 3. Scoring golden traces + ledger entry

`round-scoring-positive-negative`, `scores-can-go-negative`, `multi-round-first-to-500`, `target-tie-continues`. Record `ML-PP-005` (`local-only`).

## Files to Touch

- `games/meldfall_ledger/src/scoring.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/tests/rules.rs` (modify — scoring/terminal cases)
- `games/meldfall_ledger/tests/golden_traces/round-scoring-positive-negative.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/scores-can-go-negative.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/multi-round-first-to-500.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/target-tie-continues.trace.json` (new)

## Out of Scope

- Viewer-scoped exports / pairwise no-leak matrix (GAT19MELLEDFIV-013).
- Bots (GAT19MELLEDFIV-014); the atlas/ledger doc reconciliation (GAT19MELLEDFIV-022).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p meldfall_ledger`: card values (ace 15, faces/10 = 10, pips), positive tabled minus in-hand penalty, negative scores, cumulative totals.
2. 500 eligibility, unique-highest winner, and tie-continues-until-unique are correct.
3. `cargo test --workspace` passes.

### Invariants

1. Public settlement exposes totals/counts only, never opponent card identities (FOUNDATIONS §11).
2. Scoring + terminal detection are Rust-owned and deterministic (FOUNDATIONS §2); first-use stays local (FOUNDATIONS §4).

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/rules.rs` — value table, round delta, cumulative, 500/tie terminal logic.
2. `games/meldfall_ledger/tests/golden_traces/{round-scoring-positive-negative,scores-can-go-negative,multi-round-first-to-500,target-tie-continues}.trace.json`.

### Commands

1. `cargo test -p meldfall_ledger`
2. `cargo test --workspace`
3. The full all-viewer settlement no-leak matrix is GAT19MELLEDFIV-013; this ticket asserts public settlement is total/count-only.

## Outcome

Completed: 2026-06-26

Implemented Rust-owned Meldfall Ledger round and match scoring in `games/meldfall_ledger/src/scoring.rs`: card values, tabled positive score credit, in-hand penalties, round deltas, cumulative scores that may go negative, 500 target eligibility, unique-highest terminal outcome, tied-highest continuation, and stable seat-order standings.

Added public settlement structures and stable public strings that expose only per-seat totals/counts, round deltas, cumulative scores, ranks, and winner flags; remaining hand card identities stay out of public settlement output.

Added scoring tests for positive table credit minus hand penalties, negative cumulative scores, unique winner at/above 500, and tied-highest continuation. Added golden traces for `round-scoring-positive-negative`, `scores-can-go-negative`, `multi-round-first-to-500`, and `target-tie-continues`. Recorded `ML-PP-005` as a local-only first-use primitive.

Verification:

1. `cargo fmt --all --check`
2. `cargo test -p meldfall_ledger`
3. `cargo test --workspace`
