# GAT12FLOWATCOO-004: State model, typed IDs, and deterministic setup

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/flood_watch/src/state.rs`, `src/setup.rs` (deterministic shuffle via `engine-core::SeededRng`); typed state model
**Deps**: GAT12FLOWATCOO-003

## Problem

`flood_watch` needs its full typed state model and a deterministic setup that shuffles the event deck reproducibly before any action or environment logic can be written. The deck's internal order is hidden from everyone and must never enter a projection; the public remaining-composition counts are derived. Setup must assign roles from scenario data, seat both players, and initialise districts, levees, budget, turn/phase, the shared terminal slot, and a freshness token — all deterministically from the seed.

## Assumption Reassessment (2026-06-11)

1. `engine-core::SeededRng` is the deterministic RNG (verified: defined in `crates/engine-core/src/rng.rs`, `SeededRng::from_seed(seed)`, exported from `engine-core`). `games/masked_claims/src/setup.rs` is the exemplar: it imports `SeededRng` and runs an unbiased Fisher-Yates shuffle via `next_bounded_index_unbiased` (rejection sampling). `flood_watch` reuses that discipline for the event deck.
2. The spec (§Implementation reference "State" sketch, Work-breakdown item 3, Assumptions A3/A4/A7) fixes the state shape: `variant`, `seats: [SeatId; 2]`, `roles: [RoleId; 2]`, `turn_number`, `active_seat`, `phase` (`Action { budget_remaining }` / `Terminal`), `districts: [DistrictState; 5]` (flood 0–3, levees 0–cap), `event_deck` (internal order), `drawn` (public), `forecast: Option<EventCard>`, derived `remaining_composition`, `terminal_outcome: Option<SharedOutcome>`, `freshness_token`. There is no `Phase::Environment` a seat can act in (A6).
3. Cross-artifact boundary under audit: the state struct is the serialization-boundary contract consumed by `replay_support.rs`, `visibility.rs`, and the WASM bridge. Field ordering and collection types must be deterministic (sorted/insertion-ordered, no incidental hash-map iteration) so state hashes are stable (FOUNDATIONS §11). The deck-order field is internal-only and must be excluded from every projection by construction.
4. FOUNDATIONS §2 (Rust owns setup and deterministic randomness) and the §11 determinism invariant motivate this ticket: identical seed + seats + scenario must reproduce identical setup state and shuffle order; no wall-clock or thread-ordering input enters the canonical form.
5. Enforcement surface: setup is the no-leak firewall's origin — the shuffled `event_deck` order is the hidden information. It must live only in the internal state, never in a derived public field; `remaining_composition` is `scenario counts − drawn`, public derived data and not a leak (A7). The deterministic-shuffle discipline is the one the GAT12FLOWATCOO-002 ledger reviewed as "not a fifth use" (no private holdings).

## Architecture Check

1. Modeling the deck as an internal-only ordered `Vec<EventCard>` plus a derived public composition is cleaner than a single shared structure with per-field visibility flags: it makes the no-leak boundary structural (you cannot project what is not in the public view) rather than a runtime filter that can be forgotten.
2. No backwards-compatibility aliasing/shims; net-new state on the GAT12FLOWATCOO-003 skeleton.
3. `engine-core` stays noun-free — all flood/levee/deck/role state is in `games/flood_watch/src/state.rs`; setup uses only the generic `SeededRng`/`Seed`/`SeatId` contracts.

## Verification Layers

1. Deterministic setup -> golden/deterministic replay-hash check: same seed+seats+scenario produces an identical setup-state hash across runs.
2. Typed bounded state -> schema/serialization validation: flood levels stay `0..=3`, levees `0..=cap`, deck size matches scenario composition.
3. Deck-order never public -> no-leak visibility test (negative): the public projection (built in GAT12FLOWATCOO-008) contains no undrawn-deck field — asserted structurally here by the state type having the order internal-only.
4. Both scenarios set up correctly -> rule test covering `flood_watch_standard` and `flood_watch_deluge` starting constants.

## What to Change

### 1. `games/flood_watch/src/state.rs`

Define the typed state: `Phase` (`Action { budget_remaining: u8 }`, `Terminal`), `DistrictState { flood_level, levees }`, `EventCard`/`EventKind` usage, `SharedOutcome` (`Won` / `Lost { district }`), `FreshnessToken`, and the top-level state struct with deterministic field/collection ordering. Keep the undrawn-deck order in an internal-only field; expose `remaining_composition` as a derived accessor.

### 2. `games/flood_watch/src/setup.rs`

Implement deterministic setup: validate exactly two seats; assign roles from scenario data; load scenario constants; deterministically shuffle the event deck with `SeededRng` using the unbiased Fisher-Yates discipline from `masked_claims`; initialise turn 1 (active `seat_0`), full budget, no forecast, terminal `None`, freshness `0`. No projection or effect emission here beyond what setup returns.

## Files to Touch

- `games/flood_watch/src/state.rs` (modify — fill the stub)
- `games/flood_watch/src/setup.rs` (modify — fill the stub)

## Out of Scope

- Legal action generation, validation, and application (GAT12FLOWATCOO-005).
- Environment automation and event resolution (GAT12FLOWATCOO-006).
- Public/private projection and effect filtering (GAT12FLOWATCOO-008) — setup only stores state; projection is built there.

## Acceptance Criteria

### Tests That Must Pass

1. A `rules` test asserts deterministic setup for both scenarios (district count, starting levels, levee cap, deck size, budget, roles per seat).
2. A determinism test asserts identical seed+seats+scenario reproduces the identical shuffled deck order and setup-state hash.
3. `cargo clippy -p flood_watch -- -D warnings` and `cargo fmt --all --check` pass.

### Invariants

1. The undrawn event-deck order exists only in internal state; no public-derived field exposes it.
2. Flood levels and levee counts are bounded by construction (`0..=3`, `0..=cap`); setup is fully deterministic from the seed.

## Test Plan

### New/Modified Tests

1. `games/flood_watch/tests/rules.rs` — deterministic-setup cases for both scenarios (expanded in GAT12FLOWATCOO-011).
2. `games/flood_watch/tests/replay.rs` — setup-state-hash determinism under repeated runs (expanded in GAT12FLOWATCOO-011).

### Commands

1. `cargo test -p flood_watch --test rules`
2. `cargo test -p flood_watch`
3. The simulation CLI (`simulate --game flood_watch`) is the eventual end-to-end boundary but needs the full action/environment loop and tool registration; the setup-determinism unit tests are the correct boundary for this diff.

## Outcome

Completed: 2026-06-11

Filled `games/flood_watch/src/state.rs` with the typed setup state: action and
terminal phases, district state, duplicate-stable event cards, shared outcome,
private internal event deck, drawn history, forecast slot, roles, seats,
freshness token, derived remaining-composition accessors, and stable state
serialization for setup hash checks.

Filled `games/flood_watch/src/setup.rs` with deterministic setup from
`engine_core::SeededRng`: exactly-two-seat validation, scenario validation,
stable event deck construction, unbiased Fisher-Yates shuffle, role assignment,
turn 1 active seat, full budget, starting district levels, empty forecast/drawn
history, no terminal outcome, and freshness `0`.

Added `games/flood_watch/tests/rules.rs` and `games/flood_watch/tests/replay.rs`
to prove standard/deluge setup constants, bounded district/levee state,
deterministic deck order, and stable setup-state hashes.

Deviations from plan: none. Legal action generation, environment resolution,
visibility projection, and simulation/tool registration remain in later tickets.

Verification:

- `cargo test -p flood_watch --test rules` passed: 3 tests.
- `cargo test -p flood_watch --test replay` passed: 2 tests.
- `cargo test -p flood_watch` passed: 11 unit tests, 8 integration tests.
- `cargo clippy -p flood_watch --all-targets -- -D warnings` passed.
- `cargo fmt --all --check` passed.
