# GAT12FLOWATCOO-010: Level 0 and Level 1 cooperative bots

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/flood_watch/src/bots.rs` (`FloodWatchRandomBot`, `FloodWatchLevel1Bot`)
**Deps**: GAT12FLOWATCOO-006, GAT12FLOWATCOO-008

## Problem

The gate needs a cooperative bot baseline: a Level 0 random legal bot and a Level 1 rule-informed bot that can play **either role and either seat**, as a human's teammate or in bot-vs-bot cooperative replay, through the normal legal-action API. The Level 1 policy is a deterministic priority: rescue imminent losses first, mitigate forecast threats, reinforce by public expected-threat counting, forecast with spare budget, end turn when nothing improves the position. Bots read only the public view and their declared seed — never the undrawn deck order.

## Assumption Reassessment (2026-06-11)

1. GAT12FLOWATCOO-005 provides the legal action tree + per-choice metadata; GAT12FLOWATCOO-008 provides the public projection including `remaining_composition`. `games/masked_claims/src/bots.rs` (`MaskedClaimsLevel1Bot`) is the verified exemplar for a deterministic Level-1 policy with viewer-safe explanations through the legal API; `ai-core` provides the deterministic bot-RNG helpers the random bot uses.
2. The spec (§Implementation reference "Bot policy", Work-breakdown item 8, Assumptions A7/A9) fixes the Level 1 priority order (rescue level-2 districts → mitigate forecast threats → reinforce by expected-threat counting from remaining composition → forecast with spare budget → end turn) with a stable district-order tie-break, viewer-safe explanations, and decisions invariant to the hidden deck order when the public view is identical. Level 0 + Level 1 satisfy the gate (A9); a Level 2 claim would need the full evidence workflow.
3. Cross-artifact boundary under audit: bots consume the legal action API + public view contract only (FOUNDATIONS §8). The decision path must route through `legal_action_tree`/validation (mutating no state directly), and the explanation strings are a viewer-safe surface checked by the no-leak tests (GAT12FLOWATCOO-011) and the WASM `run_bot_turn` decision JSON (GAT12FLOWATCOO-014).
4. FOUNDATIONS §8 (public bots: competent, explainable, fair, deterministic under declared inputs, beatable, normal legal API, no hidden state) and the §11/§12 invariants (bots use the normal legal action API and allowed views only; v1/v2 exclude MCTS/ISMCTS/Monte Carlo/ML/RL) motivate this ticket. Deck-order inference beyond public composition counting is out of bounds.
5. Enforcement surface: the bot is a no-leak surface (§11) — its inputs are the public view + declared seed only, and its explanations must reference no undrawn-card identity or order. The §8 search-class exclusion holds: this is a deterministic priority policy, not a search/learning bot.

## Architecture Check

1. A deterministic priority policy keyed on the public view + remaining-composition counting is the right Level-1 shape: it is explainable, beatable, and provably invariant to the hidden deck order (same public view → same decision), which the no-leak tests can assert directly.
2. No backwards-compatibility aliasing/shims; built on the GAT12FLOWATCOO-005 tree and GAT12FLOWATCOO-008 view.
3. `engine-core` stays noun-free — bot policy is game-local in `games/flood_watch/src/bots.rs`; no bot helper enters `game-stdlib` (the GAT12FLOWATCOO-002 ledger authorized none).

## Verification Layers

1. Legal-only, both roles/seats -> bot legality check: Level 0 and Level 1 select only legal action paths in both roles and seats, constructing no out-of-tree action.
2. Deterministic under declared inputs -> bot test: identical public view + seed produces identical decision and rationale.
3. Hidden-order invariance -> no-leak test: decisions and rationales are unchanged when the hidden deck order differs but the public view is identical.
4. Finishes cooperative games -> simulation/CLI run: many cooperative games complete with no illegal action or invariant failure.

## What to Change

### 1. `games/flood_watch/src/bots.rs`

Implement `FloodWatchRandomBot` (uniform selection from the legal tree via deterministic bot-RNG helpers, either role/seat). Implement `FloodWatchLevel1Bot` with the priority policy: rescue (bail the most-threatened level-2 district by public expected-incoming-rise, stable tie-break), forecast mitigation (bail/reinforce a district the revealed top card would inundate, role-efficient choice), reinforce by expected threat, forecast with spare budget, end turn otherwise. Emit viewer-safe explanations with no undrawn-card reference. Route every decision through the legal tree + validation.

## Files to Touch

- `games/flood_watch/src/bots.rs` (modify — fill the stub)

## Out of Scope

- Bot-strategy evidence docs and the win-rate balance band (GAT12FLOWATCOO-013).
- WASM `run_bot_turn` wiring (GAT12FLOWATCOO-014) and the bot-teammate / bot-vs-bot UI flows (GAT12FLOWATCOO-017/018).
- Any Level 2 authored policy (out of gate scope, Assumption A9).

## Acceptance Criteria

### Tests That Must Pass

1. Bot tests prove Level 0 and Level 1 select only legal action paths in both roles and seats and are deterministic under declared inputs.
2. A bot test proves decisions and rationales are unchanged when the hidden deck order differs but the public view is identical.
3. `cargo run -p simulate -- --game flood_watch --games 1000` finishes with no illegal bot action or invariant failure (after tool registration, GAT12FLOWATCOO-015; unit-level bot tests gate this diff).

### Invariants

1. Bots use only the normal legal action API and the public view + declared seed; no undrawn-deck access, no MCTS/ISMCTS/Monte Carlo/ML/RL.
2. Bot explanations reference no undrawn-card identity or order.

## Test Plan

### New/Modified Tests

1. `games/flood_watch/tests/bots.rs` — legality, determinism, hidden-order invariance for both bots/roles/seats (expanded in GAT12FLOWATCOO-011).

### Commands

1. `cargo test -p flood_watch --test bots`
2. `cargo test -p flood_watch`
3. `cargo run -p simulate -- --game flood_watch --games 1000` is the full cooperative-playout boundary but needs `simulate` registration (GAT12FLOWATCOO-015); the bot unit tests are the correct boundary for this diff.
