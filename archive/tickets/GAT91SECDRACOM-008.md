# GAT91SECDRACOM-008: secret_draft Level 0 random + Level 1 VeiledDraftLevel1Bot + bot tests

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/secret_draft/src/bots.rs` and `games/secret_draft/tests/bots.rs`; no `engine-core` / `game-stdlib` change.
**Deps**: GAT91SECDRACOM-004, GAT91SECDRACOM-006

## Problem

The game needs a Level 0 random-legal bot and a Level 1 rule-informed `VeiledDraftLevel1Bot`. Both must call the normal legal action API, validate through Rust, mutate no state, and rank only candidates visible to their own seat — never sampling hidden opponent commitments. This proves the ROADMAP §11 "bots use allowed views" line for a simultaneous-commitment game.

## Assumption Reassessment (2026-06-08)

1. The legal-action API (GAT91SECDRACOM-004) and the seat-allowed view (GAT91SECDRACOM-006) are the only inputs a bot may read. `games/token_bazaar/src/bots.rs` (`TokenBazaarLevel1Bot`) and `games/high_card_duel/src/bots.rs` are precedents for Level 0 + Level 1 deterministic policy bots over the legal tree; `crates/ai-core` supplies deterministic bot RNG helpers.
2. Spec §"Bot policy" defines: Level 0 `SecretDraftRandomBot` chooses uniformly from the Rust legal tree via deterministic bot RNG; Level 1 `VeiledDraftLevel1Bot` ranks visible legal items by public marginal value (complete a thread set → higher value → high-thread terminal bonus → reduce deterministic-fallback vulnerability using only public priority/visible pool → stable item-ID tie-break → deterministic bot-seed tie-break). Explanations cite only public facts.
3. Cross-artifact boundary under audit: the bot↔legal-action-API boundary and the bot-explanation/candidate-ranking no-leak surface. If the opponent has committed, the bot sees only a pending flag, never the hidden item.
4. §8 public-bots + §11 no-leak are the motivating principles: restate before trusting spec — bots use the normal legal action API and allowed views only; no MCTS/ISMCTS/Monte Carlo/ML/RL/LLM, no hidden-state sampling, no opponent-commit peeking; explanations and candidate rankings leak no hidden choice. Deterministic under declared inputs.
5. Determinism: identical seat view + declared bot seed → identical decision and rationale. The key bot test asserts the decision/rationale is unchanged when the hidden opponent choice differs but the visible projection is identical (the spec's "actual hidden-state sampling" no-leak proof).

## Architecture Check

1. Routing both bots through the same legal-action API humans use (no private fast-path) is cleaner and is what makes "no hidden-state sampling" structurally true rather than asserted.
2. No backwards-compatibility aliasing/shims — fills GAT91SECDRACOM-002 stubs.
3. `engine-core` stays noun-free; bot policy is game-local. No `game-stdlib` bot-policy helper (forbidden by spec; first-use).

## Verification Layers

1. Bot legality -> bot test: every Level 0 / Level 1 decision is a path in the Rust legal tree; bots mutate no state directly.
2. Determinism -> bot test: same view + seed → same decision/rationale; bots finish many games.
3. No hidden-state sampling -> bot test: decision/rationale unchanged when hidden opponent commitment differs but visible projection is identical.
4. Explanation no-leak -> grep/manual review that rationale strings cite only public facts (no opponent-choice/candidate-sampling phrasing).

## What to Change

### 1. `src/bots.rs`

Implement `SecretDraftRandomBot` (Level 0, uniform over legal tree via deterministic bot RNG) and `VeiledDraftLevel1Bot` (Level 1 ranked policy per the spec ladder), both reading only the seat-allowed view and emitting viewer-safe rationale.

### 2. `tests/bots.rs`

Bot legality, determinism, finish-many-games, hidden-state-invariance, and explanation-no-leak tests.

## Files to Touch

- `games/secret_draft/src/bots.rs` (modify)
- `games/secret_draft/tests/bots.rs` (new)

## Out of Scope

- A Level 2 authored policy bot (spec: not required unless maintainers elect it).
- Bot-strategy docs `COMPETENT-PLAYER.md` / `BOT-STRATEGY-EVIDENCE-PACK.md` (GAT91SECDRACOM-017, trailing and depending on this ticket's fixtures).
- WASM `run_bot_turn` wiring (GAT91SECDRACOM-013).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p secret_draft --test bots` — legality, determinism, finish-many-games, hidden-state-invariance, explanation-no-leak all pass.
2. Both bots complete full games choosing only legal actions.

### Invariants

1. Bots use the normal legal action API and allowed views only; no MCTS/ISMCTS/Monte Carlo/ML/RL, no hidden-state sampling (§8/§11).
2. Decision and rationale are invariant to hidden opponent commitment when the visible projection is identical (§11 no-leak).

## Test Plan

### New/Modified Tests

1. `games/secret_draft/tests/bots.rs` — Level 0/1 legality, determinism, hidden-state invariance, explanation safety.

### Commands

1. `cargo test -p secret_draft --test bots`
2. `cargo test --workspace && bash scripts/boundary-check.sh`
3. End-to-end bot legality over 1000 games is exercised by `simulate` after tool registration (GAT91SECDRACOM-012); the `bots` test is the correct unit boundary here.
