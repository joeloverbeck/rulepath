# GAT14EVEFROEVE-004: State model, typed IDs, and deterministic seeded setup

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/event_frontier/src/{state,setup}.rs` (state model, deterministic epoch shuffle); `games/event_frontier/data/fixtures/*.fixture.json` (three scenario fixtures)
**Deps**: GAT14EVEFROEVE-003

## Problem

The gate needs its full state model and a deterministic seeded setup that replay can reproduce byte-for-byte. State holds the site graph (per-site agents/settlers/depot/cache), resource pools, the deck (undrawn order, current, next-public, discard, epoch), eligibility markers, the active-edict list, scores, epoch/Reckoning markers, terminal outcome, and the freshness token. Setup performs an **epoch-wise seeded shuffle** with the typed constraint that each epoch's Reckoning is never that epoch's first card, resolved deterministically from the seed. The three scenario fixtures capture the computed start states for `fixture-check`.

## Assumption Reassessment (2026-06-12)

1. The state/setup modules and the deterministic RNG contract are in place: verified `games/event_frontier/src/{state,setup}.rs` are stubbed by ticket 003 and that sibling games (`games/frontier_control/src/setup.rs`) seed from `engine-core`'s deterministic RNG contract (never `std::time`/wall-clock). The `CardId`/`SiteId`/`FactionId` enums and the card inventory come from ticket 003's `ids.rs`/`cards.rs`.
2. The fixture pattern and scenario set are current: verified the sibling fixture filename pattern `<game>_<scenario>.fixture.json` (e.g. `frontier_control_standard.fixture.json`); the three scenarios (standard, hard_winter, land_rush) and their typed parameters come from `data/variants.toml` (ticket 003) and the spec's golden-trace names (`hard-winter-setup`, `land-rush-setup`).
3. Cross-artifact boundary under audit: the state shape authored here is the structure every later rule ticket mutates and every visibility/replay surface (ticket 009) projects/serializes; the undrawn-deck-order field is the single hidden surface and must live in internal state, never in a viewer projection. Stable serialization order is required for replay-hash stability.
4. FOUNDATIONS §11 (deterministic replay/hashes/serialization order/RNG) and §2 (deterministic randomness owned by Rust) motivate this ticket. Restated before trusting the spec: the epoch shuffle must use the declared deterministic RNG only; the Reckoning-placement constraint must be resolved deterministically from the seed so seed + scenario reproduce the exact deck order.
5. Deterministic replay/serialization surface (§11): the setup shuffle is canonical replay input. Confirm no nondeterministic input (wall-clock, hash-map iteration order) enters the deck order or any hashed state; confirm the undrawn-order field is internal-only and introduces no leak path the visibility firewall (ticket 009) would have to undo. No replay/hash *semantics* change — this is the first game using the existing contract.

## Architecture Check

1. Encoding the Reckoning-placement constraint as a deterministic seed-driven shuffle (rejection or constrained-permutation from the seed) is cleaner than post-hoc reordering: it keeps deck order a pure function of (seed, scenario) and replayable.
2. No backwards-compatibility aliasing/shims — fills stubs created by ticket 003.
3. `engine-core` stays noun-free (state nouns live in `games/event_frontier`); no `game-stdlib` promotion.

## Verification Layers

1. Deterministic setup (§11) -> a replay/property test that the same (seed, scenario) reproduces the identical undrawn deck order and state hash across runs.
2. Reckoning-placement constraint -> a rule test that, for all three scenarios over many seeds, each epoch's Reckoning is never the epoch's first card.
3. Fixture conformance -> the three `*.fixture.json` start states match the computed setup (validated later by `fixture-check`, ticket 015); grep the fixtures exist with the scenario names.
4. Serialization-order stability -> a serialization test that state round-trips with stable field/collection order (insertion-ordered or sorted, not incidental hash-map order).

## What to Change

### 1. State model (`src/state.rs`)

Define the full typed state: `scenario`, seats/factions, deck state (`undrawn: Vec<CardId>` hidden order, `current`, `next_public`, `discard`, `epoch`), sequence state (`eligibility: [Eligibility; 2]`, `card_phase`), map state (per-site `agents`/`settlers`/`depot`/`cache_count`, adjacency from validated edges), `funds`/`provisions`, `active_edicts`, `scores`, `terminal_outcome`, `freshness_token`. Use stable serialization order.

### 2. Deterministic seeded setup (`src/setup.rs`)

Implement epoch-wise shuffle from the deterministic RNG seed with the Reckoning-never-first constraint per epoch; deal the first current/next-public cards; initialize per-scenario starts (resources, components, thresholds) from `data/variants.toml`.

### 3. Scenario fixtures

Author `data/fixtures/event_frontier_standard.fixture.json`, `event_frontier_hard_winter.fixture.json`, `event_frontier_land_rush.fixture.json` capturing the computed start state for each scenario.

## Files to Touch

- `games/event_frontier/src/state.rs` (modify; created by 003)
- `games/event_frontier/src/setup.rs` (modify; created by 003)
- `games/event_frontier/data/fixtures/event_frontier_standard.fixture.json` (new)
- `games/event_frontier/data/fixtures/event_frontier_hard_winter.fixture.json` (new)
- `games/event_frontier/data/fixtures/event_frontier_land_rush.fixture.json` (new)

## Out of Scope

- Card reveal / eligibility transitions (ticket 005) — setup establishes the initial deck and phase only.
- Operations, events, edicts, Reckoning (tickets 006–008).
- `fixture-check` tool registration (ticket 015) — fixtures are authored here, validated there.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p event_frontier` passes the deterministic-setup test (same seed+scenario → identical deck order + state hash).
2. The Reckoning-placement test passes for all three scenarios over many seeds (Reckoning never first in its epoch).
3. State serialization round-trips with stable order (serialization test).

### Invariants

1. Deck order is a pure deterministic function of (seed, scenario); no nondeterministic input enters canonical state.
2. The undrawn-deck-order field is internal state only; no setup output exposes it.

## Test Plan

### New/Modified Tests

1. `games/event_frontier/tests/replay.rs` (or a setup unit test) — deterministic-setup reproduction.
2. `games/event_frontier/tests/rules.rs` — Reckoning-never-first constraint across scenarios/seeds.
3. `games/event_frontier/tests/serialization.rs` — stable state serialization order.

### Commands

1. `cargo test -p event_frontier`
2. `cargo test -p event_frontier --test replay --test serialization`
3. The per-crate test run is the correct boundary — setup determinism is provable without the tools/CI layer, which registers later.
