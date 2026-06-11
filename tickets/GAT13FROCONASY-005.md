# GAT13FROCONASY-005: Faction action trees, validation, application, and clash

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/frontier_control/src/{actions,rules,effects}.rs` (two disjoint faction action trees, validation/diagnostics, application, asymmetric clash resolution)
**Deps**: GAT13FROCONASY-004

## Problem

This is the architectural center of the gate: two factions drawing legal actions from **disjoint** action sets, resolving contests by **asymmetric** rules, all flowing through the same generic action-tree/command/effect contracts that served symmetric games. The Garrison tree (`patrol`/`reinforce`/`dismantle`/`end_turn`) and the Prospector tree (`march`/`stake`/`muster`/`end_turn`) must each validate against faction, budget, adjacency, occupancy, caps, and stake/fort state; the tree must regenerate with remaining-budget metadata after each action; the waiting faction's tree must be empty with safe waiting metadata; `end_turn` must always be present so a turn never stalls. Clash resolution is asymmetric and lives in Rust application only.

## Assumption Reassessment (2026-06-11)

1. `games/flood_watch/src/actions.rs` (budgeted action phase, tree regeneration with remaining-budget metadata, waiting metadata, `end_turn` no-stall) is the closest exemplar; `games/draughts_lite` is the movement-shaped action-path exemplar. The state model + adjacency index from GAT13FROCONASY-004 are in place.
2. Spec §Legal action tree, §Validation, and §Turn flow define the two trees, the rejection set (stale token, wrong seat/faction, terminal, out-of-budget, non-adjacent, absent unit, illegal stake/muster/reinforce/dismantle), and the asymmetric clash rule (guard-enters-crews removes one crew and survives; crew-enters-guards removes one guard and is itself removed — A6).
3. Cross-crate boundary under audit: action paths and effect envelopes conform to the generic `engine-core` `ActionTree`/`CommandEnvelope`/`EffectEnvelope` contracts; the faction-specific action vocabulary stays entirely in `games/frontier_control`.
4. FOUNDATIONS §2 (behavior authority) and §3 (kernel boundary) under audit: all legality/validation/application/clash is Rust; the `faction`/`march`/`clash` nouns appear only in `games/frontier_control`, never in `engine-core` (boundary-check is extended to enforce `faction` in GAT13FROCONASY-013).
5. §11 fail-closed validation enforcement surface: validation is deterministic and blocking, returns viewer-safe faction-aware diagnostics, and never lets an illegal/out-of-budget/wrong-faction action through; there is no hidden information, so diagnostics carry no leak.
6. Schema extension: new action paths and new effect kinds (`CrewMarched`/`GuardPatrolled`/`ClashResolved`/`StakePlaced`/`StakeDismantled`/`CrewMustered`/`GuardReinforced`/`TurnEnded`) extend the action-tree/effect-envelope surface additively for this new game; consumers are this crate's visibility/replay (GAT13FROCONASY-007) and the web effect log (GAT13FROCONASY-015).

## Architecture Check

1. Generating each faction's legal tree from state (and emitting clash only inside application) keeps legality authoritative in one place and proves the generic contracts never assumed symmetry — cleaner than a shared "plays both factions" branch that would hide faction strategy.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` stays noun-free; no `game-stdlib` contest/asymmetry helper (first official use stays local per GAT13FROCONASY-002).

## Verification Layers

1. Faction-action separation (§3) -> rule tests (no Garrison action ever appears in a Prospector tree and vice versa).
2. Adjacency-constrained legality -> rule tests (every legal `march`/`patrol` follows an existing edge; no move crosses a non-edge) + property test (every move follows an edge).
3. Asymmetric clash correctness -> golden traces (`clash-crew-into-guards`, `clash-guard-into-crews`) + rule tests on both directions incl. multi-unit sites and caps.
4. No-stall + budget (§11) -> property test (action-phase tree always contains `end_turn`; budget tracked and exhausted correctly) + diagnostics tests for the rejection set.

## What to Change

### 1. Faction action trees (`actions.rs`)

Generate the active faction's tree with per-choice remaining-budget metadata, clash previews, and stake values (all Rust-supplied); empty waiting-faction tree with safe metadata; `end_turn` always present.

### 2. Validation + diagnostics (`rules.rs`)

Validate each submitted action against the full rejection set with viewer-safe, faction-aware messages.

### 3. Application + asymmetric clash (`rules.rs`, `effects.rs`)

Apply each action, decrement budget, regenerate the tree, and resolve clashes asymmetrically inside the mover's command, emitting the movement effect before the `ClashResolved` effect. Spending the final budget point ends the turn exactly as `end_turn`.

## Files to Touch

- `games/frontier_control/src/actions.rs` (modify)
- `games/frontier_control/src/rules.rs` (modify)
- `games/frontier_control/src/effects.rs` (modify)

## Out of Scope

- Round scoring, supply connectivity, terminal detection (GAT13FROCONASY-006).
- Visibility projection, replay, hashing (GAT13FROCONASY-007).
- Bots, full golden-trace set, benchmarks (later tickets).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p frontier_control` rule + diagnostics tests: faction-action separation, adjacency legality, both clash directions, budget exhaustion, illegal-action rejection set.
2. Property tests: every move follows an edge; unit counts stay within caps and never negative; the action-phase tree always contains `end_turn`.
3. `cargo clippy -p frontier_control --all-targets -- -D warnings` passes.

### Invariants

1. TypeScript decides no legality; the legal tree and all validation are Rust (§2).
2. Action vocabulary and clash logic stay in `games/frontier_control`; `engine-core` gains no mechanic noun (§3).

## Test Plan

### New/Modified Tests

1. `games/frontier_control/tests/rules.rs` — faction separation, adjacency, clash, budget, rejection set.
2. `games/frontier_control/tests/property.rs` — edge-following, cap, no-stall invariants (stubbed; expanded in GAT13FROCONASY-009).

### Commands

1. `cargo test -p frontier_control rules`
2. `cargo test -p frontier_control`
3. Crate-scoped rule/property tests are the correct boundary; the full golden-trace set and simulation runs land in GAT13FROCONASY-009.
