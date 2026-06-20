# GAT16BRICIRTRI-008: Hand scoring, shoot-the-moon, match accumulation, threshold/tie, and outcome

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/briar_circuit/src/scoring.rs` (+ outcome model in `state.rs`)
**Deps**: 007

## Problem

After 13 tricks, Briar Circuit must score the hand (each heart 1, Q♠ 13, 26 raw total), detect and transform shoot-the-moon (all 26 to one seat → shooter +0, each opponent +26), accumulate cumulative match scores, evaluate the 100-point threshold after each hand, continue dealing complete hands while the lowest score is tied, and produce the terminal per-seat breakdown. This ticket implements the typed scorer and outcome model.

## Assumption Reassessment (2026-06-20)

1. `games/briar_circuit/src/rules.rs` produces per-seat captured cards after GAT16BRICIRTRI-007; this ticket fills `scoring.rs` and the `Terminal { OutcomeBreakdown }` model stubbed in `state.rs`. Cumulative scores live in the `MatchState` from GAT16BRICIRTRI-004.
2. `specs/gate-16-briar-circuit-trick-taking.md` §3.1 (Hand score, Shoot the moon, Match threshold, Low-score tie rows), §4.2 (Scoring/Outcome rows), Appendix A `BC-SCORE-001..003`/`BC-MATCH-001..003`/`BC-OUTCOME-001`, and Appendix B.5 (outcome model fields) fix the behavior. Fixed add-26 is the only v1 moon resolution (no subtract-26 option).
3. Cross-artifact boundary under audit: the scorer's per-hand additions and terminal breakdown are the contract consumed by outcome explanations (web 014/017), scoring golden traces (010), and the simulator's terminal-reason summaries (011/012).
4. FOUNDATIONS §2/§11 determinism: scoring, moon transform, threshold/tie continuation, and the terminal winner are pure functions of captured-card state; a tied lowest score never resolves by seat order (display sorting may use seat order for stability only).

## Architecture Check

1. A typed per-card raw ledger → moon transform → cumulative addition pipeline keeps the 26-point conservation property checkable and the moon predicate exact (iff one seat owns all point cards).
2. No backwards-compatibility aliasing/shims — fills the scoring/outcome stubs.
3. `engine-core` untouched (§3); no `game-stdlib` scoring helper (§4) — penalty/moon semantics are game-local and incompatible with Plain Tricks' positive trick counts (per the 003 comparison).

## Verification Layers

1. Raw hand points total 26; per-card values correct -> `tests/property.rs` (`BC-SCORE-001/002`) + unit tests.
2. Moon iff one seat captures all 26; shooter +0, opponents +26 -> `tests/rules.rs` (`BC-SCORE-003`) + golden trace (010).
3. Threshold after a hand when any score ≥100; unique-low wins; tied-low continues with no seat-order tie-break -> `tests/rules.rs` (`BC-MATCH-002/003`) + `tests/property.rs` (scores monotonic).
4. Terminal breakdown is Rust-authored (raw points, moon adjustment, cumulative before/after, threshold/tie reason, one winner) -> `tests/rules.rs` (`BC-OUTCOME-001`).

## What to Change

### 1. `games/briar_circuit/src/scoring.rs`

Per-card raw point ledger; 26-point conservation; moon detection/transformation (fixed add-26); per-hand additions; cumulative totals; threshold evaluation; low-tie continuation; unique-low terminal winner.

### 2. `games/briar_circuit/src/state.rs` (outcome model)

`OutcomeBreakdown` per Appendix B.5: per-seat raw hearts/Q♠ capture, raw hand points, moon status, adjusted addition, cumulative before/after, rank, threshold flag, terminal/tie-continuation reason.

## Files to Touch

- `games/briar_circuit/src/scoring.rs` (modify; created by 004)
- `games/briar_circuit/src/state.rs` (modify; created by 004)
- `games/briar_circuit/tests/rules.rs` (modify; created by 004)
- `games/briar_circuit/tests/property.rs` (modify; created by 004)

## Out of Scope

- Public/private projection of the breakdown and effect filtering (GAT16BRICIRTRI-009).
- Scoring/moon/threshold golden traces (GAT16BRICIRTRI-010).
- The web outcome-explanation surface (GAT16BRICIRTRI-014/017).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit --test property` — 26-point conservation; moon predicate iff one seat owns all point cards; cumulative scores monotonic.
2. `cargo test -p briar_circuit --test rules` — moon transform, 100-threshold, unique-low terminal, tied-low continuation (no seat-order tie-break).
3. `cargo test -p briar_circuit` — full crate green.

### Invariants

1. A tied lowest cumulative score is non-terminal and never resolved by seat order (§2 determinism).
2. Raw hand points always total 26 before moon transformation (`BC-SCORE-002`).

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/rules.rs` — moon, threshold, tie-continuation, outcome breakdown cases.
2. `games/briar_circuit/tests/property.rs` — 26-conservation, moon-predicate, score-monotonicity properties.
3. `None` additional — scoring golden traces are authored in GAT16BRICIRTRI-010.

### Commands

1. `cargo test -p briar_circuit --test rules --test property`
2. `cargo test -p briar_circuit`
3. A per-test scope is correct because the deliverable is the scorer/outcome model; trace capture and projection are later tickets.
