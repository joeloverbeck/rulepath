# GAT18BLAPACSPA-007: team scoring, bags, terminal target, and outcome arrays

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — `games/blackglass_pact` (scoring/state/effects) + golden traces
**Deps**: GAT18BLAPACSPA-006

## Problem

Implement the hand-scoring transition and terminal evaluation: ordinary base (±10×C), ordinary overtricks (+1/bag), nil ±100, blind nil ±200, failed-nil tricks excluded from the ordinary contract and converted to bags, cumulative 10-bag/−100 rollover (repeated per threshold), the exact §3.3 component order, the 500-point terminal with unique-higher-team rule and exact-tie continuation, and the Rust-authored `standings_by_team`/`standings_by_seat` outcome arrays (spec §3.3, §3.1 scoring/terminal rows, Appendix C vectors, Appendix B.8, `BP-SCORE-*`/`BP-END-*`, candidate task `GAT18-BLAPAC-007`).

## Assumption Reassessment (2026-06-25)

1. The `team_scores`/`team_bags`/`tricks_won` fields and `MatchOutcome`/`TeamStanding`/`SeatStanding` shapes are pinned in Appendix B.3/B.8; scoring transitions from the `PlayingTrick` end state produced in GAT18BLAPACSPA-006.
2. Spec §3.3 fixes the exact arithmetic and component order; Appendix C.1–C.8 are normative test vectors; §3.1 fixes the 500/tie terminal.
3. Cross-artifact boundary under audit: the per-seat/per-team breakdown is consumed by effects, outcome, traces, simulator, and the UI — all read the Rust-authored components, never a recomputation.
4. FOUNDATIONS §2 (Rust owns scoring/terminal detection) motivates this ticket: every score, bag, penalty, and rank field is Rust-authored; the client may not recompute them.
5. Determinism surface: scores use bounded integer types (no float) with overflow tests; bag/score remainder are separate fields; the ordered hand breakdown is stable for replay/hash. Cross-viewer projection safety is enforced in GAT18BLAPACSPA-008.

## Architecture Check

1. A single Rust-authored ordered hand breakdown (vs. piecemeal score mutation) makes traces/outcomes stable and is the §2-clean source the UI renders.
2. No shims; integer-only arithmetic, no decimal-score tricks.
3. `engine-core` untouched; scoring/bags/terminal is game-local; no `game-stdlib` change.

## Verification Layers

1. The Appendix C vectors reproduce exactly (made/set/nil/blind/bags/double-threshold/two-failed-nils) -> scoring unit tests keyed to C.1–C.8 + scoring traces.
2. Failed-nil tricks never raise `O`; bags persist and roll over repeatedly; exact tie cannot terminate -> property tests + `failed-nil-tricks-bag-even-when-team-set`, `multiple-bag-thresholds-one-hand`, `exact-tie-at-or-above-500-continues` traces.
3. Outcome arrays are stable-ordered with winner/rank fields -> outcome snapshot test + terminal-standings trace.

## What to Change

### 1. Hand scoring

`scoring.rs`: compute `C`,`O`, ordinary base/overtricks, nil/blind deltas, failed-nil bags, `raw_bags`/penalty/`next_bags`, `hand_delta`, `next_score` in the §3.3 order; store score and bag remainder separately.

### 2. Terminal + outcome

`state.rs`/`scoring.rs`: after the 13th trick score the hand; enter `Terminal` only when ≥1 team ≥500 with a unique higher score; otherwise advance dealer + start a fresh blind-eligibility phase; build `MatchOutcome` with `standings_by_team`/`standings_by_seat`.

### 3. Effects + traces

`effects.rs`: `HandScored`/`BagPenaltyApplied`/`DealerAdvanced`/`MatchCompleted` (public). Add scoring/terminal golden traces (spec §7.6 #37–#52), including a negative-score start and a set-team bag-threshold crossing.

## Files to Touch

- `games/blackglass_pact/src/{scoring,state,effects}.rs` (modify)
- `games/blackglass_pact/tests/{rules,property}.rs` (modify)
- `games/blackglass_pact/tests/golden_traces/*.trace.json` (new — scoring/bags/terminal)
- `games/blackglass_pact/data/fixtures/blackglass_pact_{bags_rollover,double_bag_penalty,target_tie}.fixture.json` (new)

## Out of Scope

- Visibility projection of the breakdown across viewers (GAT18BLAPACSPA-008).
- Bot scoring heuristics (GAT18BLAPACSPA-009).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p blackglass_pact --test rules` (Appendix C.1–C.8 worked vectors).
2. `cargo test -p blackglass_pact --test property` (bag rollover, failed-nil attribution, tie non-termination, integer overflow bounds).
3. Outcome snapshot test (stable `standings_by_team`/`standings_by_seat`).

### Invariants

1. Failed nil/blind tricks add +1 point and one bag and never help the ordinary contract; bags roll over repeatedly.
2. The match terminates only after a complete hand with ≥1 team ≥500 and a unique higher score; exact ties continue.

## Test Plan

### New/Modified Tests

1. `games/blackglass_pact/tests/rules.rs` — C.1–C.8 vectors as named cases.
2. `games/blackglass_pact/tests/property.rs` — bag rollover + tie/terminal logic + overflow bounds.
3. `games/blackglass_pact/tests/golden_traces/multiple-bag-thresholds-one-hand.trace.json` — repeated-penalty evidence.

### Commands

1. `cargo test -p blackglass_pact --test rules --test property`
2. `cargo test -p blackglass_pact`
3. Crate-scoped tests are the boundary; trace validation runs at `replay-check` registration (GAT18BLAPACSPA-011).
