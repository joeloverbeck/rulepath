# GAT13FROCONASY-008: Per-faction Level 0 and Level 1 bots

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/frontier_control/src/bots.rs` (Level 0 random + Level 1 `FrontierGarrisonLevel1Bot` / `FrontierProspectorLevel1Bot`); `ai-core` deterministic RNG helpers (consumed, not modified)
**Deps**: GAT13FROCONASY-006, GAT13FROCONASY-007

## Problem

The ROADMAP §14 exit line "each faction has random and baseline bot" requires Level 0 random and Level 1 rule-informed bots **for each faction**, with distinct deterministic priority policies, viewer-safe faction-appropriate explanations, and useful bot-vs-bot simulation metrics. A single "plays both factions" policy that hides faction strategy in untestable branches is forbidden — each faction's Level 1 policy is its own documented, tested policy. Bots act only through the legal action API and read only the public view and declared bot seed.

## Assumption Reassessment (2026-06-11)

1. `games/flood_watch/src/bots.rs` is the per-role Level 0/Level 1 exemplar; `crates/ai-core` supplies the deterministic bot-RNG helpers (consumed for tie-breaks; bot RNG is bot infrastructure, not game-rule chance). The legal tree (GAT13FROCONASY-005) and public view (GAT13FROCONASY-007) are the only inputs.
2. Spec §Bot policy defines `FrontierRandomBot` (uniform over the legal tree, either faction) and the two Level 1 priority lists (Garrison: hold forts → cut richest supply → dismantle → reinforce/end; Prospector: stake richest → open path by deterministic shortest path → muster when crews < 2 → end), with stable site-order tie-breaks.
3. Cross-crate boundary under audit: bots call the same legal-action API humans use (`actions.rs` tree + `rules.rs` validation) and read only `visibility.rs`'s public view + the declared seed; they mutate no state directly.
4. FOUNDATIONS §8 (public bots) under audit: bots are deterministic under declared inputs, beatable, explainable, fair, and use no search beyond the documented Level 1 priorities — no MCTS/ISMCTS/Monte Carlo/ML/RL (§8/§11/§13).
5. §11 no-leak firewall: viewer-safe per-faction explanations and any candidate ranking name only public information; there is no hidden information to leak, but explanations must stay viewer-safe and faction-appropriate (a Garrison tree never yields a Prospector action).

## Architecture Check

1. Two separate, independently-tested Level 1 policies (vs one shared branchy policy) make each faction's strategy auditable and keep the determinism + legality guarantees per-faction; this is the FOUNDATIONS §8 "no giant weight soup / no hidden branches" posture.
2. No backwards-compatibility aliasing/shims.
3. `engine-core`/`game-stdlib` untouched; no `AsymmetricBot` helper is promoted (first official use, local per GAT13FROCONASY-002).

## Verification Layers

1. Bot legality (§8) -> bot legality check (Level 0 + Level 1 select only legal action paths for both factions across many seeds).
2. Determinism -> bot tests (identical (public view, seed) yields identical action) + `bot-vs-bot-full-game.trace.json` reproduces.
3. Distinct-policy + viewer-safe explanations -> bot tests (a Garrison tree never yields a Prospector action; explanations name faction-appropriate public reasons) + no-leak visibility test.
4. Useful metrics -> simulation/CLI run (bot-vs-bot reports per-faction win rates — exercised via `simulate` in GAT13FROCONASY-013).

## What to Change

### 1. Level 0 random (`bots.rs`)

`FrontierRandomBot`: uniform selection from the legal tree via deterministic bot-RNG helpers, either faction; never constructs out-of-tree actions.

### 2. Level 1 Garrison (`FrontierGarrisonLevel1Bot`)

The documented priority policy with stable site-order tie-breaks and viewer-safe explanations.

### 3. Level 1 Prospector (`FrontierProspectorLevel1Bot`)

The documented priority policy (deterministic shortest path, trade-only-when-profitable arithmetic, muster threshold) with viewer-safe explanations.

## Files to Touch

- `games/frontier_control/src/bots.rs` (modify)

## Out of Scope

- The full bot test suite + golden trace (GAT13FROCONASY-009) — stubs only here.
- Bot-strategy evidence docs + balance band (GAT13FROCONASY-011).
- Simulation tool registration (GAT13FROCONASY-013).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p frontier_control` bot tests: Level 0 + Level 1 select only legal paths for both factions; deterministic under declared inputs; finish games in L0vL0, L1vL1, and L1vL0 both ways.
2. Bot explanations are viewer-safe and faction-appropriate; a Garrison tree never yields a Prospector action.
3. `cargo clippy -p frontier_control --all-targets -- -D warnings` passes.

### Invariants

1. Bots use the normal legal action API and the public view + declared seed only; no hidden state, no direct mutation (§8/§11).
2. No search beyond Level 1 priorities; no MCTS/ISMCTS/Monte Carlo/ML/RL (§8).

## Test Plan

### New/Modified Tests

1. `games/frontier_control/tests/bots.rs` — per-faction legality, determinism, distinct-policy, explanation copy (expanded in GAT13FROCONASY-009).

### Commands

1. `cargo test -p frontier_control bots`
2. `cargo test -p frontier_control`
3. Crate-scoped bot tests are the correct boundary; per-faction win-rate metrics come from `simulate` after tool registration in GAT13FROCONASY-013.
