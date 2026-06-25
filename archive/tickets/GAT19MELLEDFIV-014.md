# GAT19MELLEDFIV-014: Bots — L0 random-legal, optional L1 rule-informed, and seeded simulation

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/meldfall_ledger/src/bots.rs`; `tools/simulate` game arm; bot legality tests
**Deps**: GAT19MELLEDFIV-011, GAT19MELLEDFIV-013

## Problem

Meldfall Ledger needs a Level 0 random-legal bot (required) and an optional Level 1 rule-informed bot that scores legal Rust actions using public + own-private features only. L2 is deferred behind competent-player/strategy evidence. This ticket also adds the `simulate` game arm (game-id constant, validation, seat-count handling, bot dispatch, by-seat summaries) so a seeded 1,000-match smoke can run — the arm is co-located here because the bots ticket's smoke consumes it.

## Assumption Reassessment (2026-06-25)

1. `games/river_ledger/src/bots.rs` and `games/blackglass_pact/src/bots.rs` are the patterns (legal-action-API bots, no hidden state); `bots.rs` is a stub from GAT19MELLEDFIV-003; visibility/exports exist from 012/013. `tools/simulate/src/main.rs` validates the game against a hardcoded id list and dispatches per-game (confirmed during reassessment; `--game/--seat-count/--games/--start-seed/--action-cap` flags exist).
2. Spec §3.1 (Bot floor row), §4.2 (AI.md), Appendix A.3 (bot-policy research), and §8.1 (AI bots row) define the bot tiers and the MCTS/ISMCTS/ML/RL exclusion.
3. Cross-artifact: bots consume the legal action tree (`actions.rs`) and the seat-private view only; the `simulate` dispatch arm is the shared tool boundary — a new game-id constant + validation entry + match arm.
4. FOUNDATIONS §8 / §11: bots use the normal legal action API, mutate no state directly, and use no hidden information (opponents' hands, stock order are forbidden inputs); L1 scores only public + own-private + allowed-inference features; public v1/v2 exclude MCTS/ISMCTS/Monte Carlo/ML/RL.
5. FOUNDATIONS §11 determinism: L0 is deterministic by seed; the seeded `simulate` run reproduces identical results — the no-leak matrix (GAT19MELLEDFIV-013) already asserts bot-explanation surfaces leak nothing.

## Architecture Check

1. Building both bots on the GAT19MELLEDFIV-012/013 viewer-safe surfaces guarantees no hidden-state access structurally (a bot only ever sees its seat-private view).
2. No backwards-compatibility shims.
3. `engine-core` untouched; bot policy crate-local; the `simulate` arm is additive tool registration, not a kernel change.

## Verification Layers

1. L0 always picks a legal action and is deterministic by seed -> `cargo test -p meldfall_ledger` bot tests + `l0-random-legal-full-match.trace.json`.
2. L1 (if admitted) uses no opponents' hidden hands or stock order -> bot legality test asserting inputs are public + own-private only.
3. Seeded full-match simulation completes for 2/4/6 seats -> `cargo run -p simulate -- --game meldfall_ledger --seat-count {2,4,6} ...`.

## What to Change

### 1. `bots.rs`

L0 random-legal bot over the grouped legal action tree (deterministic by seed); optional L1 rule-informed bot scoring legal actions on public + own-private features (meld timing, discard risk, high-card penalty exposure per Appendix A.3); L2 remains not admitted.

### 2. `tools/simulate` arm

Add the `meldfall_ledger` game-id constant, the validation-chain entry, seat-count handling, bot dispatch, and by-seat summaries to `tools/simulate/src/main.rs`.

### 3. Bot tests + trace

`tests/bots.rs` (L0 legality + determinism; L1 input-restriction if admitted); `l0-random-legal-full-match.trace.json`; optional `l1-rule-informed-smoke.trace.json` (mark not-applicable with reason if L1 not admitted at ship).

## Files to Touch

- `games/meldfall_ledger/src/bots.rs` (modify; created by GAT19MELLEDFIV-003)
- `tools/simulate/src/main.rs` (modify)
- `games/meldfall_ledger/tests/bots.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/tests/golden_traces/l0-random-legal-full-match.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/l1-rule-informed-smoke.trace.json` (new — or recorded not-applicable)

## Out of Scope

- Bot-strategy/competent-player docs (GAT19MELLEDFIV-015) and any L2 policy.
- `replay-check`/`fixture-check`/`rule-coverage` registration (GAT19MELLEDFIV-016/018).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p meldfall_ledger` bot tests: L0 always legal, deterministic by seed; L1 (if admitted) uses only public + own-private inputs.
2. `cargo run -p simulate -- --game meldfall_ledger --seat-count 4 --games 1000 --start-seed 1901 --action-cap 4096` completes with by-seat summaries.
3. `cargo test --workspace` passes.

### Invariants

1. Bots use the legal action API and allowed views only; no hidden-state access; no MCTS/ISMCTS/ML/RL (FOUNDATIONS §8/§11).
2. L0 is deterministic under declared seed (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/bots.rs` — L0 legality + determinism; L1 input-restriction.
2. `games/meldfall_ledger/tests/golden_traces/l0-random-legal-full-match.trace.json` (+ optional L1 smoke).

### Commands

1. `cargo test -p meldfall_ledger`
2. `cargo run -p simulate -- --game meldfall_ledger --seat-count 2 --games 1000 --start-seed 1900 --action-cap 4096`
3. `cargo run -p simulate -- --game meldfall_ledger --seat-count 6 --games 1000 --start-seed 1902 --action-cap 8192`

## Outcome

Completed: 2026-06-26

Implemented the Meldfall Ledger L0 random-legal bot in `games/meldfall_ledger/src/bots.rs` using `ai-core::RandomLegalBot` over Rust-owned, viewer-authorized legal action trees. The bot input contains the seat-private view plus legal actions for the current active phase, and tests assert deterministic legal selection, rule-API application, own-hand-only input, and no stock/opponent-hand access.

L1 remains not admitted pending strategy evidence owned by GAT19MELLEDFIV-015; this is recorded in `l1-rule-informed-smoke.trace.json` rather than silently skipped. Added `l0-random-legal-full-match.trace.json` for the L0 policy.

Registered `meldfall_ledger` in `tools/simulate` with 2-6 seat validation and an L0 bounded playout arm. The current simulator honestly reports bounded nonterminal outcomes (`bounded_nonterminal_at_cap`) because meld-generation strategy is not admitted in this ticket; the smoke commands complete and provide by-seat summaries without pretending terminal match completion.

Verification:

1. `cargo fmt --all --check`
2. `cargo test -p meldfall_ledger`
3. `cargo run -p simulate -- --game meldfall_ledger --seat-count 2 --games 1000 --start-seed 1900 --action-cap 4096`
4. `cargo run -p simulate -- --game meldfall_ledger --seat-count 4 --games 1000 --start-seed 1901 --action-cap 4096`
5. `cargo run -p simulate -- --game meldfall_ledger --seat-count 6 --games 1000 --start-seed 1902 --action-cap 8192`
6. `cargo test --workspace`
