# GAT11MASCLABLU-004: State model, typed IDs, and deterministic setup

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `games/masked_claims/src/{state.rs,setup.rs,variants.rs}`, extends `src/ids.rs` and `src/lib.rs`; typed `data/manifest.toml` + `data/variants.toml` parsers
**Deps**: GAT11MASCLABLU-003

## Problem

The game needs its typed state, the eight-turn two-phase model, and a deterministic Rust-owned shuffle/deal before any legality or resolution logic can land. Setup must be replayable: identical seed + seats + options reproduce the same deal, with each seat view holding only its own hand.

## Assumption Reassessment (2026-06-10)

1. `src/ids.rs` skeleton (`MaskTileId`, `Grade`) lands in GAT11MASCLABLU-003; this ticket adds the state model. The shape model is `games/plain_tricks/src/state.rs` — `enum Phase`, a `<Game>State` struct with `freshness_token: FreshnessToken`, and a canonical summary string for hashing (all confirmed). Fields per the spec §State sketch: `Phase::{Claim, Reaction{responder}, Terminal}`, `hands: [Vec<MaskTileId>; 2]`, `pedestal: Option<PendingClaim { tile, declared }>`, `reserve`, `veiled_gallery`, `exposed_row`, `scores`/`exposed_lies`/`successful_challenges`/`challenges_declared`, `terminal_outcome`, `freshness_token`.
2. Setup constants per spec §Setup + Assumption A3: fifteen tiles (three per grade 1–5), five-tile hands, a five-tile never-dealt reserve, turn 1 claimant `seat_0` alternating, four claims per seat. The deterministic RNG contract is `engine_core::SeededRng` (confirmed `use engine_core::{FreshnessToken, SeatId, SeededRng};` in `games/plain_tricks/src/setup.rs`).
3. Cross-artifact boundary under audit: `data/manifest.toml` and `data/variants.toml` and their typed parsers — the static-data boundary (`docs/ENGINE-GAME-DATA-BOUNDARY.md`). Parsers must reject unknown and behavior-looking fields.
4. FOUNDATIONS §2 (setup is Rust-owned behavior) and §5 (manifest/variants are typed content/constants only — no selectors/branches/triggers) are restated as the principles under audit.
5. Determinism enforcement surface (§11/§2): the deterministic shuffle/deal feeds the replay/hash contract. Confirm the shuffle draws only from `SeededRng` (no wall-clock, no `std::time`, no incidental hash-map iteration order), so the canonical state summary is reproducible; setup introduces no hidden-info leak — each seat view excludes the opponent hand and the reserve.

## Architecture Check

1. Typed state plus a deterministic Rust setup keeps behavior authority in Rust (§2) and makes the deal byte-reproducible for replay — cleaner than a TS- or data-driven deal.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free; the shuffle stays game-local per the GAT11MASCLABLU-002 ledger decision (no `game-stdlib` helper unless that ledger authorized one).

## Verification Layers

1. Deterministic deal -> deterministic replay-hash check (full proof in GAT11MASCLABLU-011) + a setup unit test asserting a fixed seed yields a fixed deal.
2. Tile conservation (fifteen tiles across hands + reserve at setup) -> setup unit test (full property coverage in GAT11MASCLABLU-010).
3. Static data typed / no behavior -> `manifest.toml`/`variants.toml` parser rejects unknown fields (schema validation; `fixture-check` registered in GAT11MASCLABLU-015).
4. Setup view no-leak -> a seat view excludes opponent hand and reserve (no-leak visibility test, full suite in GAT11MASCLABLU-010).

## What to Change

### 1. `src/state.rs`

`Phase`, `PendingClaim`, `MaskedClaimsState` with the fields above, and a canonical summary string (insertion/sorted order only) for state hashing, mirroring the `plain_tricks` summary pattern.

### 2. `src/setup.rs`

Validate exactly two seats; load `masked_claims_standard` constants; `SeededRng` shuffle of fifteen tiles and deal five per seat (remaining five form the hidden reserve); set turn 1 (`claimant = seat_0`), empty pedestal/galleries, scores 0, terminal `None`, `freshness_token` 0. Emit setup state through Rust projection only.

### 3. `src/variants.rs` + typed parsers

`Variant` for `masked_claims_standard`; typed `manifest.toml`/`variants.toml` parsing that rejects unknown and behavior-looking fields.

## Files to Touch

- `games/masked_claims/src/state.rs` (new)
- `games/masked_claims/src/setup.rs` (new)
- `games/masked_claims/src/variants.rs` (new)
- `games/masked_claims/src/ids.rs` (modify)
- `games/masked_claims/src/lib.rs` (modify)

## Out of Scope

- Claim-phase action tree and validation (GAT11MASCLABLU-005).
- Reaction window, effects, resolution (GAT11MASCLABLU-006/007).
- Public/seat view projection (GAT11MASCLABLU-008).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p masked_claims` setup tests pass (two-seat validation, deal shape, fixed-seed determinism).
2. Tile conservation holds at setup: hands (5 + 5) + reserve (5) = fifteen distinct tiles.
3. `cargo fmt --all --check` passes.

### Invariants

1. The deal is deterministic under a fixed seed + seats + options (FOUNDATIONS §2/§11).
2. No seat view at setup contains the opponent hand or the reserve (FOUNDATIONS §11 no-leak).

## Test Plan

### New/Modified Tests

1. `games/masked_claims/src/setup.rs` `#[cfg(test)]` — two-seat validation, deal shape, fixed-seed determinism.
2. `games/masked_claims/src/state.rs` `#[cfg(test)]` — canonical summary stability.

### Commands

1. `cargo test -p masked_claims`
2. `cargo fmt --all --check`
3. Unit-level tests are the correct boundary here; cross-game replay-hash and property conservation are proven in GAT11MASCLABLU-010/011 once the full pipeline exists.
