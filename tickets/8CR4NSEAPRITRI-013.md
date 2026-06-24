# 8CR4NSEAPRITRI-013: Vow Tide C-03 checked ring-index step via SeatCount::next_ring_index

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/vow_tide` (`src/ids.rs`, `src/setup.rs`); ring order and RNG consumption unchanged
**Deps**: 8CR4NSEAPRITRI-001, 8CR4NSEAPRITRI-012

## Problem

`games/vow_tide/src/ids.rs::VowTideSeat::next_clockwise(seat_count)` and `setup.rs::deal_order_after` compute one clockwise structural step with local modulo arithmetic instead of the shipped checked `SeatCount::next_ring_index` helper (MSC-8C-003). Adopt the checked helper for the structural step only, beneath game-owned dealer/deal/bid ordering; the helper selects no dealer, first bidder, contract order, or leader (spec §3.6 Vow ring/index, §5.5).

## Assumption Reassessment (2026-06-24)

1. `games/vow_tide/src/ids.rs::VowTideSeat::next_clockwise(seat_count)` and `setup.rs::deal_order_after` currently use local modulo arithmetic; `SeatCount::next_ring_index` exists in `crates/game-stdlib/src/seat.rs` (River's `next_ring_seat` already delegates to it). Confirmed during `/reassess-spec`.
2. Spec §3.6 classifies the ring step as `migrate`; register MSC-8C-003 / `UNI8CMECSCA-011` own the helper; this ticket `Deps` `-012` to serialize edits on the shared `ids.rs`/`setup.rs`.
3. Cross-artifact: the checked ring-index contract lives in `game-stdlib::seat`; the dealer/first-bidder/contract/leader policy stays game-local. Ring-order and RNG-consumption baseline come from `-001`.
4. §11 acceptance invariant motivates this ticket: every current index, wrap order, and dealer/first-bidder trace across counts 3–7 MUST be unchanged, and RNG draw count/consumption MUST be untouched (the helper performs no RNG).
5. Enforcement surface = ring-index order + dealer/deal traces; using the checked helper for one structural step preserves wrap order and consumes no RNG, so no trace byte or draw count changes.

## Architecture Check

1. Using the checked ring-index helper removes hand-rolled modulo arithmetic and gives a bounds-checked structural step under unchanged game-owned ordering.
2. No backwards-compatibility aliasing or shim is introduced; the modulo is replaced, not wrapped. The helper does not select dealer/bidder/leader.
3. `engine-core` stays noun-free (§3); `game-stdlib::seat` is not broadened (§4).

## Verification Layers

1. Ring order + wrap unchanged across 3–7 -> focused ring-step test over every current index and count.
2. Dealer/first-bidder/deal traces + RNG consumption unchanged -> golden trace / deterministic replay-hash check (`replay-check --game vow_tide --all`).
3. Checked helper adopted -> codebase grep-proof (`SeatCount::next_ring_index` present in `next_clockwise`/`deal_order_after`; local modulo gone).

## What to Change

### 1. Adopt `SeatCount::next_ring_index` for the structural step

In `VowTideSeat::next_clockwise` and `setup.rs::deal_order_after`, compute the one clockwise structural step via `SeatCount::next_ring_index`, preserving the surrounding game-owned dealer/deal/bid ordering and consuming no RNG.

## Files to Touch

- `games/vow_tide/src/ids.rs` (modify; serialized after `-012`)
- `games/vow_tide/src/setup.rs` (modify)

## Out of Scope

- Dealer, first-bidder, contract-order, or leader selection (game-local policy).
- Any change to RNG draw count, shuffle/deal order, or hand schedule.
- Briar's pass-target methods (game-local) and River's pilot-credit ring step.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide` is green, including a ring-step test over every index and wrap for counts 3–7.
2. `cargo run -p replay-check -- --game vow_tide --all` passes with dealer/deal traces and RNG consumption byte-identical to baseline.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. Every ring index, wrap order, and dealer/first-bidder trace across 3–7 is unchanged; RNG draw count/order is untouched.
2. The helper selects no dealer/bidder/leader; that policy stays game-local.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/src/ids.rs` / `setup.rs` test module — add/strengthen a ring-step + wrap-order assertion across counts 3–7.

### Commands

1. `cargo test -p vow_tide`
2. `cargo run -p replay-check -- --game vow_tide --all`
3. The per-game test plus replay-check are the correct boundary: ring stepping is game-local structure over a checked stdlib helper.
