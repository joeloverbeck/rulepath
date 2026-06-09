# GAT10POKLITBET-010: Level 0 and Level 2 bots

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/poker_lite/src/bots.rs`, `games/poker_lite/tests/bots.rs`, `games/poker_lite/tests/golden_traces/bot-action.trace.json`. Consumes `ai-core` bot infrastructure + the legal-action API. No kernel change.
**Deps**: GAT10POKLITBET-009

## Problem

`poker_lite` requires a Level 0 random-legal bot and a Level 2 authored-policy bot. Both must use the same legal action API as humans, mutate no state directly, and use only the seat's own allowed view — never an opponent card, the hidden center before reveal, the deck tail, the seed, or any sampled/enumerated opponent holdings. The Level 2 policy must be authored, deterministic under declared inputs, explainable with viewer-safe explanations, and beatable.

## Assumption Reassessment (2026-06-08)

1. The bot pattern matches `games/secret_draft/src/bots.rs` / `games/high_card_duel/src/bots.rs`: Level 0 collects legal leaf paths from the Rust action tree and selects deterministically from a bot seed; higher levels are authored heuristics consuming a typed bot-input view. `crates/ai-core` provides the shared bot infrastructure. This ticket consumes the action tree (004), rules (005), and projection (007).
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §C Bot policy, §8 alignment, §5 WB8) fixes the Level 2 policy id `poker-lite-crest-ledger-level2-v1`, its allowed-input whitelist (own seat/private card, public center/pool/contribution/history, legal tree) and forbidden-input list (opponent private card, hidden center, deck tail, seed, full internal trace, sampled holdings), the heuristic priority order, the stable tie-break, and the viewer-safe explanation examples.
3. Cross-artifact boundary under audit: the legal-action API surface (the same `ActionTree`/`CommandEnvelope` path humans use), the typed `PokerLiteBotInput`, and the `bot_chose_action` effect shape from `effects.rs` (006). This ticket authors `bot-action.trace.json` (the only golden trace deferred from GAT10POKLITBET-009).
4. FOUNDATIONS §8 (public bots are product opponents — competent, explainable, fair, deterministic, beatable; no MCTS/ISMCTS/Monte Carlo/ML/RL) and §11 (bots use the normal legal action API and allowed views only) motivate this ticket. Restated before trusting the spec narrative.
5. No-leak + bot-legality surface under audit (§8/§11/§12): the Level 2 bot input MUST be exactly the whitelist — tests must assert no opponent private card, deck tail, hidden center before reveal, or hidden-state-derived ranking enters bot input or explanation; the bot must route through the legal action API and mutate no state; explanations must be viewer-safe (public: policy id + family; private-to-actor: own strength bucket only). No solver/sampling.

## Architecture Check

1. An authored deterministic policy over a typed whitelisted input is the §8 contract — it is explainable and beatable, and the typed input physically excludes hidden state, so a leak via bot reasoning is impossible by construction. Matches the sibling bot design.
2. No backwards-compatibility aliasing/shims — new module.
3. `engine-core` untouched (§3); no `game-stdlib` bot-policy promotion (§4, explicitly forbidden by spec §9); no MCTS/ISMCTS/ML/RL — no §13 new-bot-search-class ADR trigger.

## Verification Layers

1. Bot legality (both levels choose only legal actions, mutate no state) -> `cargo test -p poker_lite --test bots` legality tests + bot-legality check.
2. Input whitelist (Level 2 receives only `PokerLiteBotInput`; never opponent card/deck tail/hidden center) -> input-isolation test asserting the forbidden fields are absent from the bot's input type.
3. Determinism + beatability (Level 2 is deterministic under declared inputs; a documented line beats it) -> deterministic tie-break test + simulation; `cargo run -p simulate -- --game poker_lite` to terminal under the action cap (after GAT10POKLITBET-012).
4. Explanation no-leak (public explanation = policy id + family; private = own bucket only) -> explanation-content assertion over `bot_chose_action` effect JSON.

## What to Change

### 1. `games/poker_lite/src/bots.rs`

Level 0 random-legal (collect legal leaves, deterministic seed selection, validate/apply normally, diagnostic if no legal action). Level 2 `poker-lite-crest-ledger-level2-v1`: consume `PokerLiteBotInput` (whitelist only); apply the §C heuristic priorities (survive legality → protect made pair → respect public price → use high rank pre-reveal → avoid reckless lift → close when uncertain → stable tie-break); emit viewer-safe `bot_chose_action` explanations.

### 2. `games/poker_lite/tests/bots.rs` + `tests/golden_traces/bot-action.trace.json`

Legality, input-whitelist isolation, determinism, beatability-under-simulation, and explanation-no-leak tests; the `bot-action` golden trace.

## Files to Touch

- `games/poker_lite/src/bots.rs` (new)
- `games/poker_lite/tests/bots.rs` (new)
- `games/poker_lite/tests/golden_traces/bot-action.trace.json` (new)
- `games/poker_lite/src/lib.rs` (modify — add `mod bots;` + re-exports)

## Out of Scope

- Bot-strategy docs `COMPETENT-PLAYER.md` / `BOT-STRATEGY-EVIDENCE-PACK.md` / `AI.md` (GAT10POKLITBET-011).
- WASM bot-turn wiring (GAT10POKLITBET-014) and the web bot UI (GAT10POKLITBET-015).
- Any MCTS/ISMCTS/Monte Carlo/ML/RL/opponent-sampling approach (spec §9 forbidden).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite --test bots` — Level 0 + Level 2 legality, input-whitelist isolation, deterministic tie-break, explanation-no-leak.
2. Simulation to terminal under the action cap for both bot levels (via GAT10POKLITBET-012's `simulate` registration).
3. `cargo test -p poker_lite` passes overall.

### Invariants

1. Both bots use only the legal action API and the seat's allowed view; neither receives an opponent card, deck tail, hidden center, or seed (§8/§11).
2. Level 2 is authored and deterministic under declared inputs; no MCTS/ISMCTS/Monte Carlo/ML/RL/sampling (§8, spec §9).

## Test Plan

### New/Modified Tests

1. `games/poker_lite/tests/bots.rs` — legality, input whitelist, determinism, beatability, explanation no-leak.
2. `games/poker_lite/tests/golden_traces/bot-action.trace.json` — deterministic bot-action fixture.

### Commands

1. `cargo test -p poker_lite --test bots`
2. `cargo run -p simulate -- --game poker_lite --games 1000 --start-seed 0 --action-cap 16` (passes once GAT10POKLITBET-012 registers `simulate`)
3. `cargo test -p poker_lite`
