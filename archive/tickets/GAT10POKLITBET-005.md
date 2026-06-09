# GAT10POKLITBET-005: Rules transition engine, showdown comparator, and shared-pool accounting

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/poker_lite/src/rules.rs` + extends `games/poker_lite/tests/rules.rs`. No kernel change.
**Deps**: GAT10POKLITBET-004

## Problem

The deterministic heart of `poker_lite`: apply `hold`/`press`/`lift`/`match`/`yield` transitions, close pledge rounds, reveal the center crest after round 1, run the showdown comparator after round 2, keep exact shared-pool accounting, resolve the terminal outcome (`YieldWin`/`ShowdownWin`/`Split`), and increment the freshness token. All Rust-owned and fully deterministic.

## Assumption Reassessment (2026-06-08)

1. The transition-engine shape matches `games/secret_draft/src/rules.rs` and `games/high_card_duel/src/rules.rs` (apply command → new state + outcome). The legal-action tree + validation it relies on were authored in GAT10POKLITBET-004 (`actions.rs`); this ticket consumes that legality, never re-deriving it.
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §A4 round close, §A5 showdown comparator, §A1 terminal outcomes + max contribution bound) fixes: round close conditions; round-1-close → reveal center, advance to round 2, set `active_seat=seat_1`, reset lift cap; round-2-close → showdown; comparator `strength=(pair_flag, private_rank_value)` lexicographic; exact split on equal strength (`each=shared_pool/2`); yield → opponent wins pool, no reveal; deterministic max contribution 7/seat.
3. Cross-artifact boundary under audit: `tests/rules.rs` is created by GAT10POKLITBET-004 and **extended** here (create-then-modify chain — Deps on 004 declared). The transition outputs feed `effects.rs` (006), `visibility.rs` (007), and `replay_support.rs` (009); the `TerminalOutcome` shape from `state.rs` (003) is the contract.
4. FOUNDATIONS §2 (behavior authority) motivates this ticket: scoring/terminal detection, turn order, round-close, reveal timing, and accounting are Rust-owned; TypeScript never computes the winner, tie-break, or pool split.
5. Determinism + no-leak surface under audit (§11): identical `(seed, command stream, version)` must produce identical state and outcome (replay determinism, asserted in 009); the comparator/accounting are pure functions of state. The yield terminal MUST NOT reveal the yielding seat's private card (no-leak); the showdown reveal is grouped (handled as an effect in 006). Confirm no accounting path can overrun the deterministic 7-marker bound and no terminal-outcome field carries a hidden card before its reveal point.

## Architecture Check

1. Keeping transitions, comparator, and accounting as pure state→state functions makes deterministic replay and golden traces trivially reproducible and keeps the no-leak firewall a projection concern. Matches the sibling rules-core pattern.
2. No backwards-compatibility aliasing/shims — new module; extends a test file created in this batch.
3. `engine-core` stays noun-free (comparator/accounting are crate-local, §3); shared-pool/betting/showdown are kept local — second/first use per spec §8, no `game-stdlib` promotion (§4).

## Verification Layers

1. Transition correctness (each family mutates state per §A3/§A4) -> `cargo test -p poker_lite --test rules` transition tests.
2. Comparator correctness (pair beats no-pair; higher rank wins; equal → split) -> comparator unit tests covering pair-beats-high-card, high-card showdown, and tie-split.
3. Accounting exactness (`shared_pool == Σ contributions`; no overrun past 7/seat; split is even) -> accounting assertions in `tests/rules.rs` (property coverage in GAT10POKLITBET-008).
4. Determinism (same command stream → same terminal) -> deterministic transition test; full replay-hash check deferred to GAT10POKLITBET-009.

## What to Change

### 1. `games/poker_lite/src/rules.rs`

Implement `apply` for the five families; round-close detection (both hold / match after press-or-lift / yield); round-1-close reveal+advance (`active_seat=seat_1`, lift cap reset); round-2-close showdown; the lexicographic comparator; exact accounting and `each=shared_pool/2` split; `TerminalOutcome` construction; freshness-token increments on each accepted command.

### 2. `games/poker_lite/tests/rules.rs` (modify — created by GAT10POKLITBET-004)

Add transition, round-close, center-reveal-timing, showdown-comparator (all three outcomes), accounting-exactness, yield-terminal-no-reveal, and determinism tests.

## Files to Touch

- `games/poker_lite/src/rules.rs` (new)
- `games/poker_lite/tests/rules.rs` (modify)
- `games/poker_lite/src/lib.rs` (modify — add `mod rules;` + re-exports)

## Out of Scope

- Semantic effect emission and viewer envelopes (GAT10POKLITBET-006).
- Public/private view projection (GAT10POKLITBET-007).
- Replay export/import and golden traces (GAT10POKLITBET-009).
- Bot decisions (GAT10POKLITBET-010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite --test rules` — transitions, round close, center-reveal timing, comparator (pair/high-card/tie), accounting, yield terminal.
2. Determinism test: identical command stream from a fixed seed yields identical terminal outcome and contributions.
3. `cargo test -p poker_lite` passes overall.

### Invariants

1. Winner/tie-break/pool-split/reveal-timing are computed only in Rust (§2); TS never decides them.
2. `shared_pool == sum(contributions)` always; no seat exceeds the 7-marker bound; yield never reveals a private card (§11).

## Test Plan

### New/Modified Tests

1. `games/poker_lite/tests/rules.rs` (modify) — transitions, round close, comparator (3 outcomes), accounting, yield-no-reveal, determinism.

### Commands

1. `cargo test -p poker_lite --test rules`
2. `cargo test -p poker_lite`
3. `bash scripts/boundary-check.sh` — confirms accounting/comparator added no mechanic noun to `engine-core`.

## Outcome

Completed: 2026-06-09

Changed:

- Added `games/poker_lite/src/rules.rs` with transition handling for hold, press, lift, match, and yield.
- Implemented round-close behavior: round 1 reveals the center and advances to round 2 with `seat_1` leading; round 2 resolves showdown and enters terminal.
- Implemented pair-before-high-card showdown comparison and terminal outcomes for `YieldWin`, `ShowdownWin`, and `Split`.
- Added exact shared-pool/contribution accounting with debug assertions for `shared_pool == sum(contributions)`, max contribution bound, and even split.
- Extended `games/poker_lite/tests/rules.rs` with transition, accounting, center reveal, yield terminal, comparator, showdown allocation, and deterministic command-stream coverage.
- Re-exported rules helpers from `games/poker_lite/src/lib.rs`.

Deviations from original plan:

- `apply_action` returns `Result<(), Diagnostic>` and emits no effects; semantic effects are intentionally deferred to GAT10POKLITBET-006.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p poker_lite --test rules` passed: 9 integration tests.
- `cargo test -p poker_lite` passed: 18 unit tests, 9 integration tests, and 0 doc tests.
- `bash scripts/boundary-check.sh` passed.
