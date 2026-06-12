# GAT13FROCONASY-004: State model, typed IDs, and deterministic setup

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/frontier_control/src/{state,ids,setup,variants}.rs` (typed state model, ID types, deterministic setup, map-graph validation)
**Deps**: GAT13FROCONASY-003

## Problem

Frontier Control needs a typed state model and a fully deterministic setup driven only by the chosen map variant — no RNG anywhere in game rules. The validated site graph (typed site IDs, adjacency from edge data), per-site occupancy, faction-to-seat assignment, round/turn markers, two-action budget, per-faction scores, terminal outcome, and freshness token are the substrate every later pipeline ticket reads and writes. Map data must be validated (connected graph, valid endpoints, no duplicates, starts on existing sites) so invalid content fails closed at setup.

## Assumption Reassessment (2026-06-11)

1. `games/flood_watch/src/state.rs` and `setup.rs` are the shape exemplars (freshness-token field, variant-driven deterministic setup, typed ID newtypes in `ids.rs`); the skeleton modules from GAT13FROCONASY-003 are in place to fill.
2. Spec §State and §Setup sketches define the fields (`round_number: u8`, `active_faction`, `Phase::Action { budget_remaining }` / `Phase::Terminal`, `sites: Vec<SiteState>` with `guards/crews/stake/fort/stake_value` + validated edge index, `scores: [u16; 2]`, `terminal_outcome`, `freshness_token`) and the standard-map constants (A3).
3. Cross-crate boundary under audit: the state struct is the shared contract consumed by `actions.rs`/`rules.rs` (legality + application), `visibility.rs` (projection), and `replay_support.rs` (hash inputs); adjacency is held as a validated edge index derived once at setup, never re-parsed.
4. FOUNDATIONS §2 behavior authority is under audit: setup, ID assignment, and map validation are Rust; setup is fully determined by the variant with no seed-derived game state.
5. §11 determinism enforcement surface: state serialization order and setup output must be byte-identical for identical (seats, variant) inputs — no wall-clock, no hash-map iteration order in canonical forms; this state feeds the replay/hash surface in GAT13FROCONASY-007 and must introduce no nondeterminism path. No hidden information exists, so there is no leak path to guard here.

## Architecture Check

1. Deriving adjacency into a validated edge index at setup (vs re-traversing raw edge lists per legality check) keeps every later adjacency/connectivity query cheap and proves the graph valid exactly once; failing closed at setup is cleaner than per-action graph re-validation.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched; site/edge/fort/guard/crew types stay in `games/frontier_control`; no `game-stdlib` graph helper is introduced (first official use stays local per GAT13FROCONASY-002).

## Verification Layers

1. Deterministic setup (§2/§11) -> golden trace / deterministic replay-hash check (identical state hash for identical variant; `highlands-setup.trace.json` exercises the second map — authored in GAT13FROCONASY-009).
2. Map-data validation (fail-closed) -> schema/serialization validation + rule tests (disconnected graphs, dangling edges, duplicate sites/edges, off-graph starts rejected).
3. State-contract integrity -> codebase grep-proof (state fields match the spec §State sketch; adjacency held as a validated index).

## What to Change

### 1. Typed IDs (`ids.rs`)

Site IDs, faction IDs, seat mapping newtypes; the seven standard site IDs and faction IDs (`faction_garrison`, `faction_prospectors`).

### 2. State model (`state.rs`)

The full state struct per the spec §State sketch, including `Phase`, `SiteState`, scores, terminal outcome, and freshness token, with stable serialization order.

### 3. Deterministic setup + validation (`setup.rs`, `variants.rs`)

Load and validate the chosen variant; build the adjacency index; place start units deterministically (two guards per fort, three crews at base camp for standard); reject disconnected graphs, dangling/duplicate edges, duplicate sites, and off-graph starts with fail-closed diagnostics.

## Files to Touch

- `games/frontier_control/src/ids.rs` (modify)
- `games/frontier_control/src/state.rs` (modify)
- `games/frontier_control/src/setup.rs` (modify)
- `games/frontier_control/src/variants.rs` (modify)

## Out of Scope

- Action trees, validation of moves, application, clash (GAT13FROCONASY-005).
- Scoring, connectivity, terminal detection (GAT13FROCONASY-006).
- Visibility projection and replay hashing (GAT13FROCONASY-007).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p frontier_control` setup tests: both maps build deterministically; invalid map data (disconnected/dangling/duplicate/off-graph start) is rejected.
2. Repeated setup for the same variant yields an identical state hash.
3. `cargo clippy -p frontier_control --all-targets -- -D warnings` passes.

### Invariants

1. No RNG enters game-rule setup; setup is a pure function of (seats, variant).
2. Adjacency is a validated edge index; no off-graph or duplicate edge survives setup.

## Test Plan

### New/Modified Tests

1. `games/frontier_control/tests/rules.rs` — setup determinism + map-validation cases (stubbed here, expanded in GAT13FROCONASY-009).
2. `games/frontier_control/tests/serialization.rs` — stable state serialization order (stubbed here).

### Commands

1. `cargo test -p frontier_control setup`
2. `cargo test -p frontier_control`
3. Crate-scoped tests are the correct boundary; cross-tool determinism (replay-check) is exercised after the replay surface lands in GAT13FROCONASY-007.

## Outcome

Completed: 2026-06-11

What changed:
- Added the typed `FrontierControlState`, `Phase`, `SiteState`, validated adjacency index, faction scores, terminal outcome placeholder, and stable state summary/hash surface.
- Implemented deterministic setup as a pure function of seats plus variant, with no game-rule RNG.
- Added variant validation for seat count, nonzero budget/round/cap, required faction order, duplicate edges/sites, disconnected graphs, duplicate starts/stake values, and over-cap starting units.
- Added setup tests for standard/highlands initialization, repeated-setup stable hash equality, wrong seat count, duplicate edge rejection, disconnected graph rejection, duplicate start rejection, and over-cap start rejection.

Deviations from the plan:
- None. Action legality/application, scoring/connectivity, visibility, and replay hashing beyond the stable state surface remain for later tickets.

Verification:
- `cargo fmt --all --check` passed.
- `cargo test -p frontier_control setup` passed (5 setup-filtered tests).
- `cargo test -p frontier_control` passed (11 tests).
- `cargo clippy -p frontier_control --all-targets -- -D warnings` passed.
