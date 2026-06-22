# UNI8CMECSCA-010: Add `game-stdlib::seat` — `SeatCount` / `SeatCountRange` / ring index

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `crates/game-stdlib/src/seat.rs` (new), `crates/game-stdlib/src/lib.rs`
**Deps**: UNI8CMECSCA-002

## Problem

Typed nonzero seat counts, exact/range validation, and checked ring-index arithmetic are reusable game-layer plumbing duplicated across games today, but they are not kernel vocabulary (exact/range admission and diagnostics stay game-owned). This ticket adds a new `game-stdlib::seat` module with `SeatCount`, `SeatCountRange`, `checked_index`, and `next_ring_index`, returning typed structural errors only — no turn/dealer/partnership/pass policy and no generated seat enum (C-03).

## Assumption Reassessment (2026-06-22)

1. `crates/game-stdlib/src/` currently holds `board_space.rs`, `trick_taking.rs`, `lib.rs` — no `seat.rs` and no existing seat-count/ring helper (confirmed by `ls`/grep at the reassessed commit). `crates/game-stdlib/src/lib.rs` is the module-declaration site.
2. Spec §4.3 C-03 fixes the surface: `SeatCount::new(usize) -> Result<Self, SeatCountError>`, `get`, `checked_index(usize) -> Result<usize, SeatIndexError>`, `next_ring_index(usize) -> Result<usize, SeatIndexError>`; `SeatCountRange::inclusive(min,max) -> Result<Self, SeatCountRangeError>`, `validate(usize) -> Result<SeatCount, SeatCountError>`. Register entry `MSC-8C-003` homes this in `game-stdlib::seat`.
3. Cross-artifact boundary under audit: the `game-stdlib` module surface (`crates/game-stdlib/src/lib.rs`) and the mechanical-scaffolding lane (ADR 0008). This lands via the **scaffolding register**, not the §4 behavioral mechanic atlas — it is behavior-free typed plumbing, so no third-use primitive-pressure ledger applies.
4. FOUNDATIONS §4 + §3: the helper is earned via the scaffolding register (not speculative), and it stays out of `engine-core` to protect noun-free minimalism. It encodes no role/team/deal/pass/turn policy; admission ranges and diagnostic mapping remain game-owned.
5. Determinism (§11): the arithmetic is total and deterministic; errors are typed structural values (zero rejection, invalid current index, min/max inversion, overflow-safe construction), never panics in the happy path.

## Architecture Check

1. `game-stdlib::seat` is the narrowest lawful home: reusable count/ring geometry belongs at the game layer, but exact/range admission and diagnostics stay game-local, so the kernel stays noun-free.
2. No backwards-compatibility shim — a new module; nothing is aliased.
3. `engine-core` untouched; `game-stdlib` growth is earned through the register (`MSC-8C-003`), satisfying §4.

## Verification Layers

1. Valid counts across a bounded range construct; zero rejects; min/max inversion rejects; overflow-safe construction → `game-stdlib` property tests.
2. `checked_index` rejects out-of-range; `next_ring_index` wraps correctly across the full range → property tests.
3. Errors are structural and noun-free → grep-proof on the new error enums + `bash scripts/boundary-check.sh` (game-stdlib is not the kernel, but identifiers stay policy-free).

## What to Change

### 1. `crates/game-stdlib/src/seat.rs` (new)

Implement `SeatCount`, `SeatCountRange`, `SeatCountError`, `SeatIndexError`, `SeatCountRangeError`, and the methods above. Structural errors only; no policy.

### 2. `crates/game-stdlib/src/lib.rs`

Add `pub mod seat;` and re-export the public types.

## Files to Touch

- `crates/game-stdlib/src/seat.rs` (new)
- `crates/game-stdlib/src/lib.rs` (modify)

## Out of Scope

- Adopting the helper in any game (UNI8CMECSCA-011).
- Any pass-direction/dealer/bidding/partnership/active-seat policy or generated seat enum.
- Diagnostic-text mapping (stays game-local).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p game-stdlib` passes, including the seat property tests.
2. Property tests cover: all valid counts in a bounded range, ring wraparound, zero rejection, invalid current index, min/max inversion, overflow-safe construction.
3. `cargo build --workspace` passes.

### Invariants

1. `game-stdlib::seat` contains no turn/dealer/partnership/pass policy and no generated enum.
2. All errors are typed structural values; no panic on invalid input.

## Test Plan

### New/Modified Tests

1. `crates/game-stdlib/src/seat.rs` (inline `#[cfg(test)]`) — `SeatCount`/`SeatCountRange`/ring property + rejection tests.

### Commands

1. `cargo test -p game-stdlib`
2. `cargo build --workspace`
3. The `game-stdlib` suite is the correct boundary — no game adopts the module until UNI8CMECSCA-011.

## Outcome

Completed: 2026-06-22

What changed:
- Added `game_stdlib::seat` with `SeatCount`, `SeatCountRange`, and structural error enums for count, range, and index validation.
- Added nonzero count construction, inclusive range validation, checked index validation, and `next_ring_index` wraparound.
- Re-exported the seat-count types from `game-stdlib`.
- Added inline tests covering valid count ranges, ring wraparound, zero rejection, invalid current index, min/max inversion, and maximum-size ring arithmetic.

Deviations:
- None. No game adopted the helper in this ticket, and no turn/dealer/pass/partnership policy or generated seat enum was added.

Verification:
- `cargo fmt --all --check`
- `cargo test -p game-stdlib`
- `cargo build --workspace`
- `bash scripts/boundary-check.sh`
