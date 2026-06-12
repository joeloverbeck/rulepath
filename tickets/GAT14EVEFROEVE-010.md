# GAT14EVEFROEVE-010: Per-faction Level 0 and Level 1 scripted bots

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/event_frontier/src/bots.rs` (per-faction Level 0 random + Level 1 scripted policy bots)
**Deps**: GAT14EVEFROEVE-008, GAT14EVEFROEVE-009

## Problem

The gate's "scripted bots are demo-coherent" exit line needs per-faction deterministic bots that select only from the engine's legal action tree — the digital correction of the COIN flowchart-bot pitfall (paper bots relied on humans to adjudicate legality). Level 0 is a per-faction random legal bot; Level 1 is a two-layer policy: a decision table for event-vs-operation-vs-pass keyed on public state, then ordered site-priority lists with explicit total-order tiebreaks (stable site index, then stable action-path order). Bots read only the legal tree, the public view, and their bot seed — never undrawn deck order.

## Assumption Reassessment (2026-06-12)

1. The legal tree, public view, and victory-distance summaries bots consume exist: verified tickets 005–008 produce the constrained menu + op tree and ticket 009's public projection carries eligibility, resources, and victory distances. The deterministic bot RNG helpers come from `crates/ai-core` (as used by `games/frontier_control/src/bots.rs`).
2. The bot policy is specified: verified the spec's "Bot policy" — `EventFrontierRandomBot` (Level 0, either faction); `EventCharterLevel1Bot` and `EventFreeholdersLevel1Bot` (Level 1) with the documented Layer-1 choice table and Layer-2 per-op-type site priorities and total-order tiebreaks.
3. Cross-crate boundary under audit: the bots use the same legal action API as humans (`engine-core` action tree), choose through normal validation, mutate no state, and use only allowed views; the two factions' policies are distinct, not one undifferentiated policy. Explanations are viewer-safe.
4. FOUNDATIONS §8 (public bots) and §11 (bots use the legal action API and allowed views only; v1/v2 exclude MCTS/ISMCTS/Monte Carlo/ML/RL) motivate this ticket. Restated before trusting the spec: Level 1 is deterministic priority policy only; no search/learning; bots never read undrawn deck order or any hidden state.
5. No-leak surface (§11): a bot explanation is a leak vector. Confirm bot inputs are restricted to the legal tree + public view + bot seed (no `undrawn` access), and that viewer-safe explanations narrate the plan without referencing hidden state. No replay/hash semantics change; bot determinism under declared inputs is required.

## Architecture Check

1. A two-layer filter-and-rank policy that only ever ranks the engine's legal tree (never generates moves) is cleaner and safer than a flowchart that assumes legality: out-of-tree actions are structurally impossible, and total-order tiebreaks make every decision deterministic.
2. No backwards-compatibility aliasing/shims — fills the bots stub.
3. `engine-core` stays noun-free; no `game-stdlib` `ScriptedBot` promotion (forbidden by the spec; ledger authorized none).

## Verification Layers

1. Bot legality (§8/§11) -> a bot test over many seeds that every chosen action is in the legal tree and no bot reads undrawn order.
2. Determinism -> a bot test that the same (state, seed) yields the same choice; decision-table conformance on constructed states.
3. Distinct per-faction policy -> a test that the Charter and Freeholder Level 1 bots apply different priority lists (not one shared policy).
4. Viewer-safe explanations -> a no-leak test that explanations reference only public state.

## What to Change

### 1. Level 0 (`src/bots.rs`)

`EventFrontierRandomBot`: selects uniformly from the legal tree via deterministic bot RNG helpers, for either faction.

### 2. Level 1 (`src/bots.rs`)

`EventCharterLevel1Bot` and `EventFreeholdersLevel1Bot`: Layer-1 choice table (event/op/pass keyed on current-event favorability, next-public-card value, resource pressure, victory distance own/opponent); Layer-2 per-op-type site priorities (Charter: deny imminent cache victory → extend majority → fortify → save; Freeholder: complete cache threshold → escort exposed caches → spread presence → save), each ending in a total-order tiebreak (stable site index, then stable action-path order). Viewer-safe explanations narrate the plan.

## Files to Touch

- `games/event_frontier/src/bots.rs` (modify; created by 003)

## Out of Scope

- Bot-strategy evidence docs and the balance result (ticket 013).
- The simulation CLI registration that runs 1000-game balance (ticket 015).
- Any Level 2 claim — not in scope (spec Assumption A9); no MCTS/ISMCTS/ML/RL.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p event_frontier` passes per-faction legality over many seeds (every action in the legal tree; no deck-order access).
2. Determinism + decision-table conformance tests pass on constructed states.
3. Distinct-per-faction-policy and viewer-safe-explanation tests pass.

### Invariants

1. Bots choose only from the legal action tree, mutate no state, and never read undrawn deck order or hidden state.
2. The two factions have distinct documented policies; every priority list ends in a total-order tiebreak.

## Test Plan

### New/Modified Tests

1. `games/event_frontier/tests/bots.rs` — legality, determinism, table conformance, distinct policies, explanation safety.

### Commands

1. `cargo test -p event_frontier --test bots`
2. `cargo test -p event_frontier`
3. The per-crate bot test is the correct boundary — legality/determinism are provable without the simulate CLI; aggregate balance is evidenced in ticket 013.
