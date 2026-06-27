# GAT20STACROSTA-012: L0 random-legal bot, seeded simulation, and AI.md

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/starbridge_crossing/src/bots.rs`, `tests/bots.rs`, `tools/simulate/src/main.rs` (arm), `games/starbridge_crossing/docs/AI.md`
**Deps**: GAT20STACROSTA-011

## Problem

Every official game needs a Level 0 random-legal bot (`docs/FOUNDATIONS.md` §8). This ticket lands the L0 bot, its legality tests, the `simulate` arm that drives the seeded many-game smoke (the arm's consumer), and `AI.md` documenting the bot floor and the gated L2/L3 ceiling.

## Assumption Reassessment (2026-06-27)

1. Bot registration is direct import + a game-specific `simulate` arm (no shared registry): confirmed `tools/simulate/src/main.rs` imports `MeldfallL0Bot` and routes `"meldfall_ledger" => run_..._simulation`. Starbridge follows: `StarbridgeCrossingL0Bot` + `parse_bot_action` exported, a `"starbridge_crossing"` arm added.
2. `simulate` flags are `--game --games --start-seed --action-cap --seat-count` (no `--bot`/`--seed`) — confirmed; the L0 bot is wired into the dispatch, matching the reassessed spec's acceptance commands.
3. Cross-artifact boundary: the bot consumes the Rust legal action tree (007/008) and drives full matches to terminal (009/011); the `simulate` arm is placed with this consumer ticket (not the tool-registration ticket) per the official-game pattern.
4. §8 (public bots) motivates this ticket: the L0 bot uses the normal legal action API and the public view only, mutates no state directly, is deterministic under seed, and uses no hidden information (there is none). No MCTS/ISMCTS/Monte Carlo/ML/RL/runtime-LLM (§8/§11 forbidden).
5. Bot-legality enforcement surface (§8/§11): `tests/bots.rs` + the seeded `simulate` smoke. Confirm the bot only ever selects a path present in Rust's action tree, deterministically under a fixed start seed, across `{2,3,4,6}`.

## Architecture Check

1. An L0 bot selecting uniformly from Rust's legal action tree with seeded tie-breaks is the minimal correct opponent; placing the `simulate` arm with the bot keeps the seeded smoke self-contained.
2. No backwards-compatibility shims.
3. `engine-core`/`ai-core` reused; no mechanic noun added; no `game-stdlib` change.

## Verification Layers

1. Bot legality (§8) -> bot legality check: `tests/bots.rs` asserts every selected path is in the legal action tree, across seeds + seat counts.
2. Determinism -> bot test: identical seed → identical selections.
3. Seeded simulation smoke -> simulation/CLI run: `simulate --game starbridge_crossing --seat-count {2,3,4,6} --games 100 --start-seed 20` completes with no illegal action.
4. Forbidden-technique exclusion (§8) -> manual review: no search/learning class.

## What to Change

### 1. Author `src/bots.rs`

`StarbridgeCrossingL0Bot` (seeded random-legal over the action tree), `L0_POLICY_ID`, `parse_bot_action`.

### 2. Add the `simulate` arm

`tools/simulate/src/main.rs`: import the bot, add the `"starbridge_crossing"` arm running the seeded simulation; respect `--seat-count {2,3,4,6}`.

### 3. Author `docs/AI.md`

Document the L0 floor; record L2/L3 as gated behind `COMPETENT-PLAYER.md` + `BOT-STRATEGY-EVIDENCE-PACK.md` (not shipped this gate); list forbidden techniques.

### 4. Add the `bot-l0` golden trace to the catalog created in 011.

## Files to Touch

- `games/starbridge_crossing/src/bots.rs` (new)
- `games/starbridge_crossing/tests/bots.rs` (new)
- `games/starbridge_crossing/docs/AI.md` (new)
- `tools/simulate/src/main.rs` (modify)
- `games/starbridge_crossing/src/lib.rs` (modify; created by 004 — add `pub mod bots;`)
- `games/starbridge_crossing/tests/golden_traces/bot-l0-action.trace.json` (new; created by 011, added here)

## Out of Scope

- L1/L2/L3 bots and `COMPETENT-PLAYER.md`/`BOT-STRATEGY-EVIDENCE-PACK.md` content (the latter is authored not-started in 018).
- `replay-check`/`fixture-check`/`rule-coverage` arms — GAT20STACROSTA-013.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p starbridge_crossing --test bots`
2. `cargo run -p simulate -- --game starbridge_crossing --seat-count 6 --games 100 --start-seed 20`
3. `bash scripts/boundary-check.sh`

### Invariants

1. The L0 bot only selects legal action paths, deterministically under seed (§8).
2. No forbidden bot technique is introduced (§8/§11).

## Test Plan

### New/Modified Tests

1. `games/starbridge_crossing/tests/bots.rs` — legality over many seeds across `{2,3,4,6}`; determinism.
2. `games/starbridge_crossing/tests/golden_traces/bot-l0-action.trace.json` — deterministic seeded selection.

### Commands

1. `cargo test -p starbridge_crossing --test bots`
2. `cargo run -p simulate -- --game starbridge_crossing --seat-count 2 --games 100 --start-seed 20 && cargo run -p simulate -- --game starbridge_crossing --seat-count 6 --games 100 --start-seed 20`
3. The bots test + seeded simulate run are the correct legality boundary; CLI exercises the arm end-to-end.

## Outcome

Implemented the Starbridge Crossing L0 random-legal bot as a deterministic seeded selector over the Rust legal action tree, exported the parser/decision helpers, and added focused bot legality coverage across the supported seat counts `{2,3,4,6}`.

Added the `starbridge_crossing` `simulate` arm and dependency so the CLI drives the Rust setup, L0 action selection, action parsing, and Rust rule application end to end. Added `games/starbridge_crossing/docs/AI.md` documenting the L0-only admitted bot policy, public-information boundary, and forbidden MCTS/ISMCTS/Monte Carlo/ML/RL/runtime-LLM techniques. Added `bot-l0-action.trace.json` as the versioned bot trace receipt.

Verification:

1. `cargo test -p starbridge_crossing --test bots` — passed, 3 tests.
2. `cargo test -p starbridge_crossing` — passed, 53 tests/doc-tests across unit, property, replay, rules, serialization, visibility, and bot coverage.
3. `cargo run -p simulate -- --game starbridge_crossing --seat-count 2 --games 100 --start-seed 20` — passed with no illegal actions; `capped_matches=100`, `total_actions=6400`, `average_length=64.00`.
4. `cargo run -p simulate -- --game starbridge_crossing --seat-count 6 --games 100 --start-seed 20` — passed with no illegal actions; `capped_matches=100`, `total_actions=6400`, `average_length=64.00`.
5. `bash scripts/boundary-check.sh` — passed.
6. `git diff --check` — passed.

The random simulations all reached the default short `action_cap=64`; this is expected for random movement on the full Star Halma board and is recorded as capped smoke evidence, not terminal-game strategy evidence. The unrelated dirty file `.claude/skills/spec-to-tickets/SKILL.md` was left untouched.
