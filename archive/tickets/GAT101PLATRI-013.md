# GAT101PLATRI-013: Level 0 and Level 2 bots and AI.md

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/plain_tricks/src/bots.rs`, `games/plain_tricks/tests/bots.rs`, new `games/plain_tricks/docs/AI.md`. Bot infra via `crates/ai-core`. No `engine-core`/`game-stdlib` change.
**Deps**: GAT101PLATRI-012

## Problem

The game needs a Level 0 random-legal bot and a Level 2 authored-policy bot (`plain-tricks-level2-v1`) that consumes only its own hand, its legal tree, and public trick/score/void history — never opponent hand, tail, seed, or sampled holdings. The bots prove a fair hidden-hand opponent for state-dependent legality. `AI.md` documents the bot surface.

## Assumption Reassessment (2026-06-09)

1. `games/plain_tricks/src/{actions,visibility}.rs` (legal tree + seat view) exist; `crates/ai-core` provides bot infrastructure; the strategy contract is recorded in `games/plain_tricks/docs/{COMPETENT-PLAYER.md,BOT-STRATEGY-EVIDENCE-PACK.md}` (GAT101PLATRI-012). Mirror `games/poker_lite/src/bots.rs`.
2. Spec §5 item 9 and appendix B fix: Level 0 collects legal leaf paths and selects deterministically from the bot seed; Level 2 uses `PlainTricksBotInput` (own hand + public state) with the B2 heuristic and deterministic tie-break (suit order → rank → stable card id). Explanations are viewer-safe (own-hand + public-history reasoning only).
3. Shared boundary under audit: the legal-action API (bots use the same API as humans, mutate no state directly) and the `PlainTricksBotInput` view contract — no opponent hand or tail enters bot input.
4. FOUNDATIONS §8 (competent, explainable, fair, deterministic, beatable; no MCTS/ISMCTS/Monte Carlo/ML/RL/hidden-state sampling) is under audit.
5. Enforcement surface: §11 no-leak firewall on bot explanations and candidate rankings. Tests must assert no opponent card, tail card, seed, or hidden-state-derived ranking enters bot input or explanations; `bot_chose_action` public payload carries policy id + action family only (shape from GAT101PLATRI-008).

## Architecture Check

1. Authored deterministic policy over public information (vs. any search/sampling bot) is the only FOUNDATIONS-§8-permitted approach for v1/v2 and proves fair hidden-hand play; reusing the legal-action API keeps bots from bypassing validation.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched and noun-free; bot policy types are `plain_tricks`-local; bot infra is `ai-core`. No `game-stdlib` change.

## Verification Layers

1. Level 0 uses legal actions only; Level 2 action is legal, deterministic under tie-breaks -> bot legality tests (`tests/bots.rs`).
2. Level 2 receives only `PlainTricksBotInput`; never opponent hand/tail/seed -> bot-input no-leak test.
3. Explanations are viewer-safe (no opponent-hand claims) -> no-leak test on explanation effects.
4. Bots complete simulations to terminal under the action cap -> simulation/CLI run (GAT101PLATRI-014) + within-crate playout test.

## What to Change

### 1. `games/plain_tricks/src/bots.rs`

Implement Level 0 random-legal (collect legal leaves, deterministic seed selection, validate/apply normally) and Level 2 `plain-tricks-level2-v1` per appendix B2 (win cheaply when justified else discard low; lead established winners else low from longest suit; deterministic tie-break). Produce viewer-safe explanation effects.

### 2. `games/plain_tricks/tests/bots.rs`

Level 0 legality; Level 2 legality + determinism; `PlainTricksBotInput`-only (no opponent hand/tail); viewer-safe explanations; both complete simulations under cap.

### 3. `games/plain_tricks/docs/AI.md`

Document the bot levels, the Level 2 policy id, allowed inputs, and the fairness/no-hidden-state guarantees.

## Files to Touch

- `games/plain_tricks/src/bots.rs` (new)
- `games/plain_tricks/tests/bots.rs` (new)
- `games/plain_tricks/docs/AI.md` (new)

## Out of Scope

- `simulate` tool registration / CLI playout wiring (GAT101PLATRI-014) — this ticket provides the bots it drives.
- WASM run-bot-turn branch (GAT101PLATRI-016).
- `COMPETENT-PLAYER.md` / `BOT-STRATEGY-EVIDENCE-PACK.md` (authored in GAT101PLATRI-012).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks --test bots`: Level 0/Level 2 legality, Level 2 determinism, input-restriction, viewer-safe explanations.
2. Tests assert no opponent card, tail card, seed, or hidden-state-derived ranking enters bot input or explanation.
3. Both bots complete simulations to terminal under the action cap.

### Invariants

1. Bots use the normal legal action API and allowed views only; mutate no state directly (FOUNDATIONS §8/§11).
2. No MCTS/ISMCTS/Monte Carlo/ML/RL/hidden-state sampling appears (FOUNDATIONS §8).

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/tests/bots.rs` — legality, determinism, no-leak input/explanation, playout-to-terminal.
2. Refresh `games/plain_tricks/tests/golden_traces/bot-action.trace.json` (authored in GAT101PLATRI-011) with the real bot output.

### Commands

1. `cargo test -p plain_tricks --test bots`
2. `cargo test -p plain_tricks`
3. Per-crate bot tests are the correct boundary; multi-game simulation throughput is exercised via `simulate` in GAT101PLATRI-014.

## Outcome

Completed: 2026-06-09

What changed:

1. Added `games/plain_tricks/src/bots.rs` with `PlainTricksRandomBot`, `PlainTricksLevel2Bot`, `PlainTricksBotInput`, legal-tree candidate extraction, deterministic tie-breaks, and viewer-safe bot-choice effects.
2. Added `games/plain_tricks/tests/bots.rs` covering Level 0 and Level 2 legality, non-mutation, determinism, input whitelist/no-leak boundaries, authored priority examples, explanation no-leak, repeated terminal playouts, and bot trace metadata.
3. Added `games/plain_tricks/docs/AI.md` documenting bot levels, policy ids, allowed/forbidden inputs, decision summary, explanation boundary, and verification.
4. Refreshed `games/plain_tricks/tests/golden_traces/bot-action.trace.json` with real `plain-tricks-level2-v1` metadata, expected action, command stream, and deterministic hashes.
5. Added `ai-core` as the `plain_tricks` crate dependency for `RandomLegalBot`.

Deviations from original plan:

1. Bot explanations use the existing GAT101PLATRI-008 `BotChoseActionPublic` effect shape: public policy id and action family only. No new private bot explanation effect was added.
2. The opening Level 2 example selects `river_5` because the policy treats it as a likely winning lead from the longer suit under the documented deterministic priority order.

Verification:

1. `cargo test -p plain_tricks --test bots` passed.
2. `cargo test -p plain_tricks --test replay` passed.
3. `cargo test -p plain_tricks` passed.
4. `cargo fmt --all --check` passed.
5. `node scripts/check-doc-links.mjs` passed.
