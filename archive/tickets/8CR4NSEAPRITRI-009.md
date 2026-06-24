# 8CR4NSEAPRITRI-009: Vow Tide C-02 canonical formatter/roster adoption

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/vow_tide` (`src/ids.rs`); emitted seat strings and 3–7 roster order unchanged
**Deps**: 8CR4NSEAPRITRI-001, 8CR4NSEAPRITRI-008

## Problem

`games/vow_tide/src/ids.rs::{VowTideSeat::as_str, seat_id_for_index, canonical_seat_ids}` hand-format canonical seat strings and the declared roster instead of delegating to the shipped canonical formatting/index helpers (MSC-8C-002). Adopt the helpers without changing emitted underscore strings or the declared 3–7 roster length/order (spec §3.5 Vow formatter/roster, §5.4).

## Assumption Reassessment (2026-06-24)

1. `VowTideSeat::as_str`, the standalone `seat_id_for_index`, and `canonical_seat_ids` currently format `seat_N` strings and assemble the roster by hand in `games/vow_tide/src/ids.rs`; canonical formatting/index helpers exist in `engine-core::SeatId`. Confirmed during `/reassess-spec`.
2. Spec §3.5 classifies this as `migrate`; register MSC-8C-002 / `UNI8CMECSCA-009` own the grammar; the parser is migrated in `-008` (this ticket `Deps` it to serialize edits on the shared `ids.rs`).
3. Cross-artifact: source/API/WASM/trace/export seat outputs read these formatters; the emitted-string baseline comes from `-001`.
4. §11 acceptance invariant motivates this ticket: emitted canonical strings (`seat_0…seat_6`) and the declared 3–7 roster length/order MUST be byte-identical before/after.
5. Enforcement surface = canonical seat-output strings in traces/exports/WASM; delegation preserves every emitted string, so no trace/export byte changes.

## Architecture Check

1. Delegating formatting/index construction to the kernel helper completes the game-level C-02 parse+format pair for Vow.
2. No backwards-compatibility aliasing or shim is introduced; the formatters are replaced, not wrapped. Hand schedule / dealer / deal capacity remain game-local.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4).

## Verification Layers

1. Emitted seat strings unchanged -> golden trace / deterministic replay-hash check (`replay-check --game vow_tide --all`) plus a focused format assertion.
2. Declared 3–7 roster length/order unchanged -> codebase grep-proof + focused `canonical_seat_ids` assertion across counts.
3. Canonical helpers adopted -> codebase grep-proof (canonical formatting/index helpers present; hand-format gone from the three symbols).

## What to Change

### 1. Delegate formatter/roster to canonical helpers

In `as_str`, `seat_id_for_index`, and `canonical_seat_ids`, construct canonical output and indices via the `engine-core::SeatId` formatting/index helpers, preserving every emitted string and the declared 3–7 roster length/order exactly.

## Files to Touch

- `games/vow_tide/src/ids.rs` (modify; serialized after `-008`)

## Out of Scope

- The parser migration (`-008`) and WASM import-alias adapter (`-010`).
- Any change to `next_clockwise`, hand schedule, dealer, or deal-capacity logic (game-local).
- Changing any emitted seat string or roster order.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide` is green, including a focused canonical-format + 3–7 roster-order assertion.
2. `cargo run -p replay-check -- --game vow_tide --all` passes with seat strings byte-identical to the `-001` baseline.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. Emitted canonical seat strings and the declared 3–7 roster length/order are unchanged.
2. No new public symbol or shim is introduced; ring/schedule/dealer methods are untouched.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/src/ids.rs` test module — add/strengthen a focused assertion that `as_str` / `seat_id_for_index` / `canonical_seat_ids` emit the baseline strings and roster order across 3–7.

### Commands

1. `cargo test -p vow_tide`
2. `cargo run -p replay-check -- --game vow_tide --all`
3. The per-game replay-check is the correct boundary: canonical seat output is exercised by the game's traces.

## Outcome

Completed: 2026-06-24

What changed:
- Added a Vow canonical-seat roster built with `SeatId::from_zero_based_index`, and routed `VowTideSeat::as_str` and `seat_id_for_index` through canonical helper-backed output.
- Added a focused formatter/roster test asserting baseline `seat_0` through `seat_6` strings and the declared 3-through-7 roster order.

Deviations:
- None. Parser work was already completed in `8CR4NSEAPRITRI-008`; WASM import aliases remain owned by `8CR4NSEAPRITRI-010`.

Verification:
- `cargo fmt --all --check`
- `cargo test -p vow_tide`
- `cargo run -p replay-check -- --game vow_tide --all`
- `bash scripts/boundary-check.sh`
