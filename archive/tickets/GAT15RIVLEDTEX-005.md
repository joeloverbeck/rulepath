# GAT15RIVLEDTEX-005: Fixed-limit betting engine and street advancement

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/river_ledger/src/actions.rs`, `src/betting.rs`, `src/rules.rs`, `tests/rules.rs`, `src/lib.rs`; betting golden traces
**Deps**: GAT15RIVLEDTEX-004

## Problem

The deterministic betting core: legal `fold`/`check`/`call`/`bet`/`raise` generation, fixed small/big bet units (small on preflop/flop, big on turn/river), the one-bet-plus-three-raises cap, the contribution ledger, live/folded seat statuses, betting-round closure, street advancement through flop/turn/river with community reveal, and the fold-out terminal — all Rust-owned and deterministic.

## Assumption Reassessment (2026-06-14)

1. `games/poker_lite/src/{actions,betting,rules}.rs` are the precedent for the legal-action tree + `apply` transition shape; like `poker_lite`'s `rules.rs`, this ticket's `apply_action` emits no effects (semantic effects deferred to GAT15RIVLEDTEX-008).
2. `specs/...-base.md` §4.1, §3.1 (cap = one opening bet plus three raises per street), and §6 exit row 3 fix `RL-BET-*`, `RL-STREET-*`, `RL-POT-SINGLE-*`, `RL-VIS-DIAGNOSTIC-*`.
3. Cross-artifact boundary under audit: `rules.rs` `apply` consumes the setup state from 004 and the `state.rs` ledger/status types from 003, extends the `tests/rules.rs` created in 004, and produces transitions that effects (008) and showdown (007) consume; the fold-out terminal is handled here, full showdown in 007.
4. FOUNDATIONS §2 (behavior authority) motivates this ticket: legal-action generation, validation, call-price/contribution matching, cap tracking, round closure, and street advancement are Rust-owned; TypeScript never computes legality or obligations.
5. Determinism + diagnostics no-leak (§11) under audit: identical command streams yield identical state; the raise-cap, wrong-seat, and stale-action diagnostics state a public reason without exposing private cards or hidden deck facts; the contribution ledger never goes negative and stays within the deterministic capacity (no all-in).

## Architecture Check

1. Pure state→state transitions with a separate legal-action tree keep golden traces and replay deterministic and keep validation fail-closed, matching the sibling rules-core pattern.
2. No backwards-compatibility aliasing/shims — new modules extending in-batch files.
3. `engine-core` stays noun-free (§3); betting/contribution/cap logic is crate-local — no `game-stdlib` promotion (§4).

## Verification Layers

1. Legal-action correctness per street (units, cap, call price) -> `cargo test -p river_ledger --test rules` betting tests.
2. Round closure + street advancement + fold-out terminal -> transition tests including 6-seat action-order wraparound.
3. Diagnostics no-leak (cap/wrong-seat/stale state a public reason only) -> diagnostic assertion tests (§11).
4. Determinism (same command stream → same ledger/state) -> deterministic transition test; full replay-hash deferred to GAT15RIVLEDTEX-010.

## What to Change

### 1. `games/river_ledger/src/actions.rs`

Rust legal-action tree + command validation for `fold`/`check`/`call`/`bet`/`raise`; no UI-only legality.

### 2. `games/river_ledger/src/betting.rs`

Fixed-limit unit selection (small/big by street), cap tracking (one bet + three raises), call price, contribution matching, round-closure detection, street advancement with community reveal.

### 3. `games/river_ledger/src/rules.rs` + tests + traces

`apply_action`, transitions, terminal checks (fold-out), diagnostics, rule-ID hooks; extend `tests/rules.rs`; add golden traces `preflop-blinds-call-check-advance`, `flop-small-bet-cap`, `turn-river-big-bet`, `raise-cap-diagnostic`, `foldout-last-live-hand`, `wrong-seat-diagnostic`.

## Files to Touch

- `games/river_ledger/src/actions.rs` (new)
- `games/river_ledger/src/betting.rs` (new)
- `games/river_ledger/src/rules.rs` (new)
- `games/river_ledger/tests/rules.rs` (modify; created by 004)
- `games/river_ledger/src/lib.rs` (modify; created by 003)
- `games/river_ledger/tests/golden_traces/preflop-blinds-call-check-advance.trace.json` (new)
- `games/river_ledger/tests/golden_traces/flop-small-bet-cap.trace.json` (new)
- `games/river_ledger/tests/golden_traces/turn-river-big-bet.trace.json` (new)
- `games/river_ledger/tests/golden_traces/raise-cap-diagnostic.trace.json` (new)
- `games/river_ledger/tests/golden_traces/foldout-last-live-hand.trace.json` (new)
- `games/river_ledger/tests/golden_traces/wrong-seat-diagnostic.trace.json` (new)

## Out of Scope

- Showdown evaluator, split pot, and outcome explanation beyond the fold-out terminal (GAT15RIVLEDTEX-006/007).
- Semantic effect emission (GAT15RIVLEDTEX-008).
- All-in/side-pot handling (Gate 15.1).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger --test rules` — legality by street, fixed units, cap, call/check/bet/raise/fold, round closure, street advancement, 6-seat wraparound, fold-out terminal.
2. Diagnostics for cap/wrong-seat/stale carry no hidden card or deck fact.
3. Same command stream from a fixed seed yields identical ledger and state.

### Invariants

1. Legality/validation/betting transitions are Rust-owned (§2); TS never computes them.
2. Contribution ledger is non-negative, within capacity, and matched-or-folded at street close (§11); diagnostics are no-leak.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/rules.rs` (modify) — betting legality, cap, round close, street advancement, fold-out, no-leak diagnostics.
2. Betting golden traces (6 files, new) — recorded command-stream evidence.

### Commands

1. `cargo test -p river_ledger --test rules`
2. `cargo test -p river_ledger && bash scripts/boundary-check.sh`
3. Golden-trace replay validation runs via `cargo run -p replay-check -- --game river_ledger` after GAT15RIVLEDTEX-015; behavior is proven here by the rule tests.

## Outcome

Completed: 2026-06-14

Implemented the Rust-owned River Ledger betting core with legal action generation and command validation for `fold`/`check`/`call`/`bet`/`raise`, fixed-limit small and big street units, one-opening-bet plus three-raise cap enforcement, deterministic response order, contribution-ledger accounting, street closure through flop/turn/river board reveals, and foldout terminal resolution. Added the crate-local `actions`, `betting`, and `rules` modules, exported the betting API from `lib.rs`, and extended state with public pending-response tracking for deterministic round closure.

Added rule tests for active-seat legality, call price metadata, preflop blind call/check advancement, flop raise cap diagnostics, turn/river big-bet units, 6-seat wraparound order, foldout terminal no-reveal behavior, public-only diagnostics, public legal-action metadata, and identical command-stream determinism. Added the six required betting golden-trace placeholder JSON files pending replay-check registration.

Deviations: full showdown resolution remains deferred to GAT15RIVLEDTEX-006/007, semantic effects remain deferred to GAT15RIVLEDTEX-008, and golden-trace replay validation remains deferred until the River Ledger replay lane is registered in GAT15RIVLEDTEX-015.

Verification:

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p river_ledger --test rules`
- `cargo test -p river_ledger`
- `bash scripts/boundary-check.sh`

Unrelated pre-existing worktree changes left untouched:

- `.claude/skills/spec-to-tickets/SKILL.md`
- `.claude/skills/spec-to-tickets/references/decomposition-patterns.md`
