# GAT15RIVLEDTEX-006: Hand evaluator — five-card ranking and seven-card 21-subset search

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/src/evaluator.rs`, `src/lib.rs`; evaluator golden traces
**Deps**: GAT15RIVLEDTEX-003

## Problem

River Ledger needs a deterministic, auditable poker hand evaluator: a five-card ranker covering all nine categories (high card → straight flush, including the ace-low straight), and a seven-card best-hand search that enumerates the `C(7,5)=21` five-card subsets, returning `(category, ordered_tie_break_vector, exact_used_cards)`. Correctness and replayability beat throughput; no lookup-table evaluator.

## Assumption Reassessment (2026-06-14)

1. No shared evaluator exists to reuse — `games/poker_lite` ships only a simplified `(pair_flag, rank)` comparator in its `rules.rs`, not a full Hold'Em evaluator; this is a game-local full evaluator using the `cards.rs` `Rank`/`Suit`/`Card` types from 003.
2. `specs/...-base.md` §4.1 (`evaluator.rs`), §5 G15-RL-005 (required categories, tie-break design, no lookup table), and §6 exit row 3 fix `RL-EVAL-*`.
3. Cross-artifact boundary under audit: the evaluator's `(category, ordered_tie_break_vector, exact_used_cards)` tuple is the contract consumed by showdown/pot (007) and the outcome explanation; it depends only on `cards.rs` (003), so it is parallel to setup/betting (004/005).
4. FOUNDATIONS §2 (behavior authority) motivates this ticket: hand evaluation is Rust-owned and never recomputed in TypeScript; auditability and explanation fidelity are preferred over a lookup-table optimization (spec out-of-scope).
5. Determinism (§11) under audit: the comparator is a deterministic total order — reflexive, antisymmetric, transitive — with suits never breaking ties; the 21-subset search is deterministic; royal flush is the highest straight flush (no separate category).

## Architecture Check

1. A straightforward 21-subset search with an explicit `(category, tie-break vector, used cards)` tuple keeps the evaluator auditable and its explanations exact, which a packed lookup table would obscure — the spec's stated priority.
2. No backwards-compatibility aliasing/shims — new module.
3. `engine-core` stays noun-free (§3); the evaluator is crate-local with no shared-evaluator `game-stdlib` promotion (§4).

## Verification Layers

1. Every category + at least one tie-break per category family ranks correctly -> `cargo test -p river_ledger` evaluator unit tests.
2. Comparator is a deterministic total order (antisymmetric, transitive) -> property-style ordering unit tests.
3. Seven-card search returns the exact best five cards -> used-cards assertion tests.
4. Evaluator golden traces (`high-card-showdown`, `pair-beats-high-card`, `straight-ace-low`, `flush-kicker-order`, `full-house-tiebreak`) -> `cargo run -p replay-check -- --game river_ledger` after GAT15RIVLEDTEX-015; ranking is proven here by unit tests.

## What to Change

### 1. `games/river_ledger/src/evaluator.rs`

Five-card evaluator over all nine categories with ace-low straight handling; seven-card best-hand search enumerating the 21 five-card subsets; the comparable `(category, ordered_tie_break_vector, exact_used_cards)` tuple; deterministic comparator with no suit tie-break.

### 2. Tests + golden traces

Unit tests for every category, ace-low straight, kicker ordering, full-house tie-break, and comparator total-ordering; add golden traces `high-card-showdown`, `pair-beats-high-card`, `straight-ace-low`, `flush-kicker-order`, `full-house-tiebreak`.

## Files to Touch

- `games/river_ledger/src/evaluator.rs` (new)
- `games/river_ledger/src/lib.rs` (modify; created by 003)
- `games/river_ledger/tests/golden_traces/high-card-showdown.trace.json` (new)
- `games/river_ledger/tests/golden_traces/pair-beats-high-card.trace.json` (new)
- `games/river_ledger/tests/golden_traces/straight-ace-low.trace.json` (new)
- `games/river_ledger/tests/golden_traces/flush-kicker-order.trace.json` (new)
- `games/river_ledger/tests/golden_traces/full-house-tiebreak.trace.json` (new)

## Out of Scope

- Showdown comparison, pot allocation, and outcome explanation (GAT15RIVLEDTEX-007).
- Any lookup-table optimization (spec out-of-scope).
- Visibility/no-leak of evaluator intermediates (008/009).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — every category, ace-low straight, kicker order, full-house tie-break, exact used-cards.
2. Comparator total-ordering test: antisymmetric and transitive across a category sweep; suits never break ties.
3. `bash scripts/boundary-check.sh` — no mechanic noun reaches `engine-core`.

### Invariants

1. Hand evaluation is deterministic and Rust-owned (§2/§11); no lookup table.
2. The returned tuple records the exact five used cards for explanation (§2).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/src/evaluator.rs` (new) — `#[cfg(test)]` category, tie-break, and total-ordering unit tests.
2. Evaluator golden traces (5 files, new) — category/tie-break evidence.

### Commands

1. `cargo test -p river_ledger`
2. `cargo test -p river_ledger && bash scripts/boundary-check.sh`
3. Golden-trace replay validation runs via `cargo run -p replay-check -- --game river_ledger` after GAT15RIVLEDTEX-015; ranking is proven here by unit tests.

## Outcome

Completed: 2026-06-14

Implemented the game-local River Ledger evaluator in `games/river_ledger/src/evaluator.rs`. The evaluator ranks five-card hands across all nine categories, handles ace-low straights, compares category plus ordered tie-break vectors without suit tie-breaks, and searches all 21 five-card subsets from seven cards while returning the exact deterministic best-five card set for showdown/explanation consumers.

Added evaluator unit tests for category ordering, ace-low straight ranking, kicker-order comparisons, full-house tie-breaks, exact used-card selection from seven cards, suit-neutral ties, and comparator antisymmetry/transitivity across a category sweep. Added the five required evaluator golden-trace placeholder JSON files pending replay-check registration.

Deviations: showdown comparison/pot allocation/outcome explanation remains deferred to GAT15RIVLEDTEX-007. Golden-trace replay validation remains deferred until the River Ledger replay lane is registered in GAT15RIVLEDTEX-015.

Verification:

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p river_ledger`
- `bash scripts/boundary-check.sh`
- `git diff --check`

Unrelated pre-existing worktree changes left untouched:

- `.claude/skills/spec-to-tickets/SKILL.md`
- `.claude/skills/spec-to-tickets/references/decomposition-patterns.md`
