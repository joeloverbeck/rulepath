# GAT15RIVLEDTEX-007: Showdown, single-pot split allocation, remainder, and outcome explanation

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/river_ledger/src/showdown.rs`, `src/pot.rs`, `src/rules.rs` (terminal wiring), `tests/rules.rs`, `src/lib.rs`; split golden traces
**Deps**: GAT15RIVLEDTEX-005, GAT15RIVLEDTEX-006

## Problem

At showdown River Ledger must discover showdown-eligible seats, compare best hands via the evaluator, allocate the single pot (including even splits and a deterministic integer remainder rule by stable button-order among tied winners), and emit a mandatory, viewer-aware, Rust-authored outcome explanation. Side pots and all-in are explicitly rejected.

## Assumption Reassessment (2026-06-14)

1. `games/poker_lite/src/rules.rs` has a simplified two-seat showdown + even split as precedent; this ticket adds the N-seat eligible-set discovery, the evaluator-backed comparison, and the button-order remainder rule absent there.
2. `specs/...-base.md` §4.1 (`showdown.rs`/`pot.rs`), §5 G15-RL-005 (tie-break + remainder design), and §6 exit row 3 fix `RL-EVAL-*`, `RL-SHOW-*`, `RL-POT-SPLIT-*`, `RL-POT-REMAINDER-*`, `RL-VIS-SHOWDOWN-*`.
3. Cross-artifact boundary under audit: showdown consumes the evaluator `(category, tie-break, used-cards)` tuple from 006 and the betting terminal/contribution ledger from 005, and produces the `TerminalOutcome`/`ShowdownReveal`/`ShowdownSeatExplanation` records (from `state.rs`, 003) that visibility (008) projects; `pot.rs` is single-pot only and explicitly rejects side-pot/all-in design.
4. FOUNDATIONS §2 (behavior authority) motivates this ticket: winner selection, tie-break, split allocation, remainder, reveal timing, and the explanation are Rust-owned; TypeScript never computes them.
5. Determinism + no-leak (§11) under audit: split shares sum exactly to the pot; the remainder is assigned one unit at a time by stable button-order among tied winners (deterministic); the fold-out terminal uses a distinct "last live hand" rationale and reveals no folded private cards; the explanation discloses private cards only where the viewer is authorized.

## Architecture Check

1. Keeping showdown/pot as pure functions of `(eligible seats, evaluator results, ledger)` makes split/remainder deterministic and the no-leak reveal a projection concern, matching the sibling showdown pattern.
2. No backwards-compatibility aliasing/shims — new modules extending in-batch files.
3. `engine-core` stays noun-free (§3); pot/showdown logic is crate-local — no shared `game-stdlib` accounting/showdown promotion (§4).

## Verification Layers

1. Single-winner, tied-split, and remainder allocation correctness -> `cargo test -p river_ledger --test rules` showdown tests.
2. Split sums to pot; remainder by stable button-order -> accounting assertion tests (§11).
3. Fold-out + showdown reveals disclose only authorized cards -> no-leak assertions (full proof in 009).
4. Split golden traces (`split-pot-even`, `split-pot-remainder-button-order`) -> `cargo run -p replay-check -- --game river_ledger` after GAT15RIVLEDTEX-015; allocation proven here by rule tests.

## What to Change

### 1. `games/river_ledger/src/evaluator`-backed showdown (`showdown.rs`)

Showdown-eligible seat discovery, winner comparison via the evaluator tuple, and the Rust-authored per-seat outcome explanation (folded-vs-reached-showdown, authorized reveal, best five, category, tie-break vector, decisive comparison, pot allocation, split/tie/remainder rule, final ledger totals).

### 2. `games/river_ledger/src/pot.rs`

Single-pot contribution allocation, even split, and the one-unit-at-a-time button-order remainder rule; explicit rejection of side-pot/all-in design.

### 3. `rules.rs` terminal wiring + tests + traces

Wire showdown/fold-out terminal into `apply`; extend `tests/rules.rs`; add golden traces `split-pot-even`, `split-pot-remainder-button-order`.

## Files to Touch

- `games/river_ledger/src/showdown.rs` (new)
- `games/river_ledger/src/pot.rs` (new)
- `games/river_ledger/src/rules.rs` (modify; created by 005)
- `games/river_ledger/tests/rules.rs` (modify; created by 004)
- `games/river_ledger/src/lib.rs` (modify; created by 003)
- `games/river_ledger/tests/golden_traces/split-pot-even.trace.json` (new)
- `games/river_ledger/tests/golden_traces/split-pot-remainder-button-order.trace.json` (new)

## Out of Scope

- Side pots / all-in side-ledger (Gate 15.1).
- Lookup-table evaluator optimization (spec out-of-scope).
- Visibility projection and pairwise no-leak proof (008/009); effect emission (008).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger --test rules` — single winner, tied split, deterministic remainder, fold-out last-live-hand rationale.
2. Split shares sum to the pot; remainder assigned by stable button-order; no negative allocation.
3. `bash scripts/boundary-check.sh` — no mechanic noun reaches `engine-core`.

### Invariants

1. Winner/tie-break/split/remainder/reveal are Rust-owned (§2).
2. Allocation conserves the pot; fold-out and showdown reveal only authorized private cards (§11).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/rules.rs` (modify) — eligible-set, comparison, even split, remainder, fold-out, authorized-reveal.
2. Split golden traces (2 files, new) — allocation evidence.

### Commands

1. `cargo test -p river_ledger --test rules`
2. `cargo test -p river_ledger && bash scripts/boundary-check.sh`
3. Golden-trace replay validation runs via `cargo run -p replay-check -- --game river_ledger` after GAT15RIVLEDTEX-015; allocation is proven here by the rule tests.
