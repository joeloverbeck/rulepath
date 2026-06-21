# GAT16BRICIRTRI-005: Setup, deterministic deal, dealer and pass-cycle state

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/briar_circuit/src/setup.rs` (+ `state.rs` deal/rotation state)
**Deps**: 004

## Problem

With the crate skeleton, card model, and phase state in place, Briar Circuit needs its deterministic match setup: a seeded 52-card shuffle dealing 13 private cards per seat, the initial dealer (`seat_0`), dealer rotation one seat clockwise per hand, the deal starting left of the dealer, and the pass direction derived from hand index (left, right, across, hold). These are the deterministic foundations every later behavior and replay depends on.

## Assumption Reassessment (2026-06-20)

1. `games/briar_circuit/src/{cards,state,ids}.rs` and the fixed-four-seat setup diagnostic exist after GAT16BRICIRTRI-004; this ticket fills `setup.rs` and the deal/rotation substate it stubs. `engine-core`'s `Seed`/`DeterministicRng` (`crates/engine-core/src/lib.rs`) supply the seeded RNG contract — no game-local RNG.
2. `specs/gate-16-briar-circuit-trick-taking.md` §3.1 (Dealer and deal, Pass cycle rows), §4.2 (Setup row), and Appendix A `BC-SETUP-*`/`BC-DEAL-*`/`BC-PASS-001` fix the behavior; spec Assumption that `seat_0` is the deterministic initial dealer holds (changeable only before traces ship).
3. Cross-artifact boundary under audit: the deterministic deal output is the contract consumed by serialization (GAT16BRICIRTRI-010 hashes), the pass phase (GAT16BRICIRTRI-006 reads the per-hand direction), and replay; the seeded RNG draw order must be stable.
4. FOUNDATIONS §2/§11 determinism is the principle under audit: identical seed + version yield identical deal, dealer, and pass direction. No wall-clock or hash-map iteration enters the shuffle or canonical state; the RNG is the engine's declared deterministic contract.

## Architecture Check

1. Deriving pass direction from hand index (a pure function) rather than storing a mutable cursor keeps setup deterministic and replay-reconstructable from seed + hand number.
2. No backwards-compatibility aliasing/shims — fills the stubs created in GAT16BRICIRTRI-004.
3. `engine-core` untouched beyond its generic RNG/seat contracts (§3); no `game-stdlib` deal helper (§4) — the second-use decision keeps deal rotation local.

## Verification Layers

1. Seeded shuffle/deal gives 13 unique private cards per seat and no remainder -> `tests/property.rs` (deck partition) + `tests/replay.rs` (deterministic redeal).
2. Dealer rotates clockwise per hand; deal starts left of dealer -> `tests/rules.rs` (`BC-DEAL-002`).
3. Pass cycle is left/right/across/hold by hand index -> `tests/rules.rs` (`BC-PASS-001`) + unit test of the index→direction mapping.
4. Identical seed reproduces identical deal -> deterministic replay-hash check (`tests/replay.rs`).

## What to Change

### 1. `games/briar_circuit/src/setup.rs`

Establish ordered seats, deterministic RNG from seed, initial dealer (`seat_0`), hand index, pass direction, zeroed cumulative scores, and the first deal: shuffle the 52-card deck and deal 13 cards clockwise beginning left of the dealer until each seat holds 13.

### 2. `games/briar_circuit/src/state.rs` (deal/rotation substate)

Fill the dealer-rotation and pass-direction-by-hand-index logic the phase state stubbed; expose the per-hand direction the pass phase reads.

## Files to Touch

- `games/briar_circuit/src/setup.rs` (modify; created by 004)
- `games/briar_circuit/src/state.rs` (modify; created by 004)
- `games/briar_circuit/tests/rules.rs` (modify; created by 004)
- `games/briar_circuit/tests/property.rs` (modify; created by 004)
- `games/briar_circuit/tests/replay.rs` (new)

## Out of Scope

- Pass select/confirm/exchange actions (GAT16BRICIRTRI-006) — this ticket only exposes the per-hand pass direction.
- Trick play and scoring (GAT16BRICIRTRI-007/008).
- Public/private projection of the dealt hands (GAT16BRICIRTRI-009).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit --test property` — deck partitions into four 13-card hands, no duplicate/lost card.
2. `cargo test -p briar_circuit --test rules` — dealer rotation, deal-left-of-dealer, and pass-cycle-by-index.
3. `cargo test -p briar_circuit --test replay` — identical seed reproduces the identical deal.

### Invariants

1. Deal, dealer, and pass direction are pure functions of seed + hand index (§2/§11 determinism).
2. Every card is dealt; no remainder; 13 per seat (`BC-DEAL-001`).

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/replay.rs` — deterministic deal reproduction from seed.
2. `games/briar_circuit/tests/property.rs` — deck-partition and 13-per-seat properties (extends 004).
3. `games/briar_circuit/tests/rules.rs` — `BC-DEAL-002` / `BC-PASS-001` rotation and cycle cases (extends 004).

### Commands

1. `cargo test -p briar_circuit --test rules --test property --test replay`
2. `cargo test -p briar_circuit`
3. A per-test scope is correct because the deliverable is deterministic setup; full no-leak/WASM proof belongs to later tickets.

## Outcome

Completed: 2026-06-21

What changed:

- Implemented seeded setup/deal for Briar Circuit using `engine_core::SeededRng` and the crate-local canonical 52-card deck.
- Dealt all cards clockwise starting left of the dealer into four 13-card private hands with no remainder.
- Added dealer-order helpers, clockwise dealer rotation, and pure pass-direction derivation from hand index.
- Extended state construction so `setup_match` stores the dealt hands and pass phase for hand 0.
- Added replay tests for identical-seed deal reproducibility, seed variance, and sequential hand-index/pass-direction reproduction.
- Extended property and rule tests for full-deck partitioning, deal-left-of-dealer order, and dealer rotation.

Deviations from plan:

- None. Pass actions, projection/no-leak filtering, trick play, and scoring remain for later tickets.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p briar_circuit --test property` passed (4 tests).
- `cargo test -p briar_circuit --test rules` passed (4 tests).
- `cargo test -p briar_circuit --test replay` passed (3 tests).
- `cargo test -p briar_circuit` passed (15 total tests including crate unit and serialization tests).
