# GAT151RIVLED-012: Bots (L0/L1/L2) for stack-capped play

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger` (`bots.rs`), tests
**Deps**: GAT151RIVLED-011

## Problem

Every shipped bot level must select correctly among the new stack-capped legal actions (fold, short/full call, short/full bet, short/full raise) and handle no-action all-in/terminal states, using only the legal-action API and an authorized seat view. L1/L2 policy may read public stack/pot/call/eligibility facts but never another seat's private cards, deck order, internal candidate rankings, or omniscient rollout. Bot explanations distinguish "call all-in", "short raise all-in", and ordinary fixed-unit actions without leaking hidden information.

## Assumption Reassessment (2026-06-20)

1. Code: `games/river_ledger/src/bots.rs` implements the L0/L1/L2 policies against the pre-stack legal-action API; they do not yet handle all-in terminality or short/full call/bet/raise distinctions. The stack-capped legal actions + metadata landed in GAT151RIVLED-005 and the authorized views in -010.
2. Docs: spec §3.3(8) — bots use the legal-action API and authorized seat view; correctly select among the stack-capped actions; L1/L2 use only public facts; explanations distinguish all-in variants. FOUNDATIONS §8 forbids MCTS/ISMCTS/Monte Carlo/ML/RL and hidden-state access.
3. Cross-artifact boundary under audit: the legal-action API + authorized seat-view surface (GAT151RIVLED-005, -010) consumed by `bots.rs`; bots mutate no state directly and choose through normal validation.
4. (§8 / §11 bot legality + no-leak) Restate: bots route through the same legal-action API as humans, use allowed views only, and never read opponent hands, deck order, candidate rankings, or rollout data. Confirm policy inputs gain only public stack/pot/call/eligibility fields; explanations carry no hidden information.

## Architecture Check

1. Reusing the legal-action API for all-in selection keeps bots and humans on one validation path; no parallel bot-only legality.
2. No backwards-compatibility shims; the policies extend to short/full variants in place.
3. No forbidden search/learning class is introduced; bot vocabulary stays game-local (§3/§8).

## Verification Layers

1. Every bot always returns a legal action, or no action when all-in/terminal -> bot legality tests across 3–6 seats.
2. Deterministic under declared seed -> seeded determinism tests.
3. No hidden-state access -> hidden-state poisoning / no-leak tests (corrupting opponent hands does not change a legal decision).
4. Explanations distinguish all-in variants without leaking -> explanation snapshot review.

## What to Change

### 1. Stack-aware policies

Update L0/L1/L2 in `bots.rs` to select among fold, short/full call, short/full bet, and short/full raise presented by Rust, and to take no action when the seat is all-in/terminal.

### 2. Authorized inputs + explanations

Feed L1/L2 only public stack/pot/call/eligibility facts; update deterministic policy evidence and explanations to name "call all-in" / "short raise all-in" / ordinary actions without leaking hidden information.

## Files to Touch

- `games/river_ledger/src/bots.rs` (modify)
- `games/river_ledger/tests/bots.rs` (modify)

## Out of Scope

- Bot-strategy doc updates (`AI.md`, `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md` — GAT151RIVLED-019).
- WASM/web exposure of bot decisions (GAT151RIVLED-013, -014).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — bots always legal/deterministic; all-in/terminal seats produce no action; hidden-state poisoning does not change decisions.
2. `cargo run -p simulate -- --game river_ledger --games 1000` — full 3–6-seat bot-vs-bot games terminate with only legal actions.
3. `cargo run -p rule-coverage -- --game river_ledger` — `RL-BOT-ALLIN-001` maps to the new tests.

### Invariants

1. Bots use the normal legal-action API and authorized views only; no MCTS/ISMCTS/Monte Carlo/ML/RL.
2. No bot reads another seat's private cards, deck order, candidate rankings, or rollout data.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/bots.rs` — legality, seeded determinism, no-action-when-all-in, and hidden-state no-leak cases.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p simulate -- --game river_ledger --games 1000`
3. `cargo run -p rule-coverage -- --game river_ledger` — bot-legality coverage is the correct boundary; cross-surface no-leak is proven in GAT151RIVLED-016.

## Outcome

Completed: 2026-06-20

Implemented stack-aware L1/L2 bot candidates by consuming Rust legal-action metadata (`amount_owed`, `adds_to_pot`, stack before/after, all-in/full-raise/reopen flags) instead of ranking action families alone. L1 and L2 continue to submit normal action paths through the legal-action API, and no-action all-in/terminal states remain represented by the existing `no_legal_actions` diagnostic.

Updated public bot explanations to distinguish call all-in, bet all-in, full raise all-in, short raise all-in, and ordinary fixed-unit actions using only public stack/call/ledger facts. Added bot tests for all-in/terminal no-action handling across L0/L1/L2, call-all-in and short-raise-all-in explanation labels, deterministic hidden-state poisoning, public input whitelisting, and unchanged command validation.

Added the minimal `RL-BOT-ALLIN-001` rule and coverage rows required for this ticket's coverage proof. The broader bot strategy docs remain owned by GAT151RIVLED-019.

Verification:

1. `cargo fmt --all --check` — passed.
2. `cargo test -p river_ledger --test bots` — passed.
3. `cargo test -p river_ledger` — passed.
4. `cargo run -p simulate -- --game river_ledger --games 1000` — passed.
5. `cargo run -p rule-coverage -- --game river_ledger` — passed.
