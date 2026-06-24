# 8CR4NSEAPRITRI-012: Vow Tide C-03 3–7 structural range validation via SeatCountRange

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/vow_tide` (`src/ids.rs`, `src/setup.rs`), `crates/wasm-api` (`src/games/vow.rs`); diagnostic bytes unchanged
**Deps**: 8CR4NSEAPRITRI-001, 8CR4NSEAPRITRI-009

## Problem

`games/vow_tide/src/ids.rs::supported_seat_count` and repeated checks in `setup.rs` / WASM creation validate the 3–7 seat range with hand-rolled comparisons instead of centralizing through the shipped structural `SeatCountRange` helper (MSC-8C-003). Centralize only the inclusive range validation through `SeatCountRange`, preserving the `VT_INVALID_SEAT_COUNT` text locally (spec §3.6 Vow admission, §5.5). Hand schedule and deal capacity remain game-owned.

## Assumption Reassessment (2026-06-24)

1. `games/vow_tide/src/ids.rs::supported_seat_count` plus `setup.rs::{setup_match, invalid_seat_count_diagnostic}` and `crates/wasm-api/src/games/vow.rs::create_vow_tide_match` currently validate 3–7 with local checks; `SeatCountRange::inclusive` / `.validate` exist in `crates/game-stdlib/src/seat.rs`. Confirmed during `/reassess-spec`.
2. Spec §3.6 classifies the structural range as `migrate`; register MSC-8C-003 / `UNI8CMECSCA-011` own the helper; hand-schedule/deal-capacity predicates are explicit `exception` (game policy, not seat geometry).
3. Cross-artifact: the structural range contract lives in `game-stdlib::seat`; this ticket `Deps` `-009` to serialize edits on the shared `games/vow_tide/src/ids.rs`. Valid/invalid diagnostic baseline comes from `-001`.
4. §11 acceptance invariant motivates this ticket: valid 3–7 and invalid 0–2/8+ diagnostics MUST be byte-identical before/after; the helper validates structure only.
5. Enforcement surface = setup range validation + `VT_INVALID_SEAT_COUNT` bytes; centralizing through the helper changes no emitted diagnostic and no setup byte.

## Architecture Check

1. Centralizing the range through one shared helper removes duplicated 3–7 comparisons across `ids.rs`/`setup.rs`/WASM while keeping the diagnostic local.
2. No backwards-compatibility aliasing or shim is introduced; comparisons are replaced, not wrapped. Hand schedule / deal capacity stay game-local exceptions.
3. `engine-core` stays noun-free (§3); `game-stdlib::seat` is not broadened (§4).

## Verification Layers

1. Valid 3–7 setup unchanged -> golden trace / deterministic replay-hash check (`replay-check --game vow_tide --all`).
2. Invalid 0–2/8+ diagnostic bytes unchanged -> focused valid/invalid count-table test asserting `VT_INVALID_SEAT_COUNT` equality across boundaries.
3. Structural helper adopted -> codebase grep-proof (`SeatCountRange` present; the duplicated local 3–7 comparisons gone).

## What to Change

### 1. Centralize the 3–7 range through `SeatCountRange`

Replace the duplicated local range checks in `supported_seat_count`, `setup.rs`, and `create_vow_tide_match` with a single `SeatCountRange::inclusive(3, 7)` validation, mapping the same `VT_INVALID_SEAT_COUNT` diagnostic locally. Do not touch hand-schedule or deal-capacity predicates.

## Files to Touch

- `games/vow_tide/src/ids.rs` (modify; serialized after `-009`)
- `games/vow_tide/src/setup.rs` (modify)
- `crates/wasm-api/src/games/vow.rs` (modify)

## Out of Scope

- `max_hand_size_for_seats`, `hand_schedule_for_seats`, `deal_hand` capacity (game policy `exception`).
- The checked ring-index step (`-013`) and dealer/bid/contract diversity (game-local).
- Changing the diagnostic text or any setup byte.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide` is green, including a valid/invalid count-table test with `VT_INVALID_SEAT_COUNT` byte-equality at 0–2/3–7/8+.
2. `cargo test -p wasm-api` is green and `cargo run -p replay-check -- --game vow_tide --all` passes.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. Valid 3–7 setup and invalid 0–2/8+ diagnostics are byte-identical to baseline.
2. Hand schedule and deal capacity remain game-owned; only structural range moves to the helper.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/src/setup.rs` test module (or `tests/`) — add/strengthen a valid/invalid range count-table test asserting the diagnostic bytes.

### Commands

1. `cargo test -p vow_tide`
2. `cargo run -p replay-check -- --game vow_tide --all`
3. The per-game test plus bridge test are the correct boundary: range validation is game-local structure over a stdlib helper.
