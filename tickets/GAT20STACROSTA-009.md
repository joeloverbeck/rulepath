# GAT20STACROSTA-009: Finish, rank, blocked pass, and turn-limit outcomes

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/starbridge_crossing/src/rules.rs` (finish/pass/terminal), `src/effects.rs`, `tests/rules.rs`
**Deps**: GAT20STACROSTA-008

## Problem

The game needs terminal behavior: a seat finishes when all 10 pegs occupy its target home; finished seats are skipped and ranked in order; the match ends when all but one seat have ranks; a forced `pass_blocked` covers no-legal-move seats; a deterministic `max_plies` turn limit projects standings. This ticket lands those Rust-owned outcomes and explanations.

## Assumption Reassessment (2026-06-27)

1. Finish/terminal detection extends `src/rules.rs` (created 007, extended 008), reading occupancy/target-home from `state.rs`/`topology.rs`; finish = all 10 pegs in the opposite home at the end of an accepted move (spec §3).
2. The `pass_blocked` forced action and `turn_limit` terminal are Rulepath replay/simulation resolutions (spec Appendix A): default `max_plies` 2000; on limit, record completed ranks + deterministic unfinished-seat standings by progress vector then clockwise seat order.
3. Cross-artifact boundary: continuing finish-order ranking and finished-seat skipping interact with the turn-order model in `state.rs`; the stable scoring/terminal rule IDs pinned in `RULES.md` (GAT20STACROSTA-001) are the IDs the outcome-explanation surfaces (015/018) consume.
4. §2 (behavior authority) motivates this ticket: scoring, terminal detection, and outcome explanations are Rust-owned; TypeScript never decides terminal state or computes standings.
5. Deterministic replay (§11): the finish-rank ledger, `pass_blocked` effect, and `turn_limit` standings projection are canonical replay/hash inputs; confirm the progress-vector tie-break is deterministic (no wall-clock / hash-map iteration) so the deferred trace/replay surfaces (011) stay byte-stable.

## Architecture Check

1. A single terminal-evaluation path producing ranks + explanation keeps scoring fail-closed and replayable, and the forced blocked pass guarantees simulation termination without ad-hoc loop breaks.
2. No backwards-compatibility shims.
3. `engine-core` untouched; no `game-stdlib` change.

## Verification Layers

1. Finish assignment (§2) -> rule test: all-pegs-in-opposite-home assigns the next rank; finished seat is skipped thereafter.
2. Continuing finish order + terminal -> rule test: 3+ seats rank in order; match ends when all but one finish; last seat gets final rank.
3. Blocked pass -> rule test + golden trace (011): a no-legal-move active seat exposes only `pass_blocked`, which records the no-move condition and advances the turn.
4. Turn-limit projection -> rule test: on `max_plies`, partial ranks + deterministic progress-vector standings are recorded.

## What to Change

### 1. Extend `src/rules.rs`

Finish detection at end of accepted move; continuing finish-rank ledger + finished-seat skipping; forced `pass_blocked` when no legal step/hop exists; `turn_limit` terminal with deterministic standings; Rust-owned outcome explanations keyed to the `RULES.md` rule IDs.

### 2. Extend `src/effects.rs`

`pass_blocked`, finish, and terminal effects.

## Files to Touch

- `games/starbridge_crossing/src/rules.rs` (modify; created by 007)
- `games/starbridge_crossing/src/effects.rs` (modify; created by 007)
- `games/starbridge_crossing/tests/rules.rs` (modify; created by 006 — add finish/pass/terminal cases)

## Out of Scope

- Public view/visibility projection — GAT20STACROSTA-010.
- Golden terminal/turn-limit/blocked traces + fixtures — GAT20STACROSTA-011.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p starbridge_crossing --test rules`
2. `cargo test -p starbridge_crossing`
3. `bash scripts/boundary-check.sh`

### Invariants

1. A seat finishes only with all 10 pegs in its target home; finished seats are skipped and ranked in order (§2).
2. A no-legal-move seat always has exactly the `pass_blocked` action; the turn-limit projection is deterministic (§11).

## Test Plan

### New/Modified Tests

1. `games/starbridge_crossing/tests/rules.rs` — finish, continuing order, terminal full standings, blocked pass, turn-limit cutoff.
2. `games/starbridge_crossing/src/effects.rs` — inline finish/terminal effect tests.

### Commands

1. `cargo test -p starbridge_crossing --test rules`
2. `cargo test -p starbridge_crossing && bash scripts/boundary-check.sh`
3. `--test rules` isolates terminal logic; full crate run confirms move-engine integration.
