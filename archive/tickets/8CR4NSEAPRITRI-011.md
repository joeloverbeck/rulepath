# 8CR4NSEAPRITRI-011: Briar Circuit C-03 exact-four structural validation via SeatCountRange

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/briar_circuit` (`src/setup.rs`), `crates/wasm-api` (`src/games/briar.rs`); diagnostic bytes unchanged
**Deps**: 8CR4NSEAPRITRI-001

## Problem

`games/briar_circuit/src/setup.rs::setup_match` validates the fixed-four seat count by comparing `seats.len()` directly to four instead of using the shipped structural `SeatCountRange` helper (MSC-8C-003). Adopt `SeatCountRange::inclusive(4, 4).validate` beneath the existing game diagnostic, mapping the same `BC_UNSUPPORTED_SEAT_COUNT` text locally (spec §3.6 Briar exact-four, §5.5). No pass/dealer/partnership policy moves into the helper.

## Assumption Reassessment (2026-06-24)

1. `games/briar_circuit/src/setup.rs::setup_match` currently compares `seats.len() != 4` directly and emits the local `invalid_seat_count_diagnostic`; `crates/wasm-api/src/games/briar.rs::create_briar_circuit_match` enforces the same count at the bridge. `SeatCountRange::inclusive(4, 4)` and `.validate` exist in `crates/game-stdlib/src/seat.rs`. Confirmed during `/reassess-spec`.
2. Spec §3.6 classifies this as `migrate`; register MSC-8C-003 / `archive/tickets/UNI8CMECSCA-011.md` own the structural helper; River's range admission is pilot credit (not re-ticketed).
3. Cross-artifact: the structural count contract lives in `game-stdlib::seat`; the `BC_UNSUPPORTED_SEAT_COUNT` diagnostic stays game-local. Valid/invalid diagnostic baseline comes from `-001`.
4. §11 acceptance invariant motivates this ticket: valid setup/deal and the above/below invalid diagnostics MUST be byte-identical before/after — the helper validates structure only, the game owns diagnostic prose.
5. Enforcement surface = setup count validation + diagnostic bytes; using the structural helper beneath the existing diagnostic changes no emitted diagnostic and no setup byte.

## Architecture Check

1. Using the shared structural range removes a hand-rolled count comparison and routes structural admission through the single owned `game-stdlib::seat` helper, with the diagnostic kept local.
2. No backwards-compatibility aliasing or shim is introduced; the comparison is replaced, not wrapped. No pass/dealer/partnership rule enters the helper.
3. `engine-core` stays noun-free (§3); `game-stdlib::seat` is the already-earned structural helper, not broadened (§4).

## Verification Layers

1. Valid setup/deal unchanged -> golden trace / deterministic replay-hash check (`replay-check --game briar_circuit --all`).
2. Above/below invalid diagnostic bytes unchanged -> focused valid/invalid count-table test asserting `BC_UNSUPPORTED_SEAT_COUNT` equality.
3. Structural helper adopted -> codebase grep-proof (`SeatCountRange::inclusive(4, 4)` present in `setup_match`; the direct `seats.len()` comparison gone).

## What to Change

### 1. Adopt `SeatCountRange::inclusive(4, 4).validate`

In `setup_match`, validate the seat count via `SeatCountRange::inclusive(4, 4).validate` (or the equivalent existing exact-count path), then map the same `BC_UNSUPPORTED_SEAT_COUNT` diagnostic locally on failure. Mirror the structural check at `create_briar_circuit_match` if it currently duplicates the literal comparison.

## Files to Touch

- `games/briar_circuit/src/setup.rs` (modify)
- `crates/wasm-api/src/games/briar.rs` (modify)

## Out of Scope

- Pass direction, partnership shape, dealer, or trick policy (game-local; not entering the helper).
- River's range admission (pilot credit) and Vow's range/ring work (`-012`/`-013`).
- Changing the diagnostic text or any setup byte.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit` is green, including a valid/invalid count-table test with `BC_UNSUPPORTED_SEAT_COUNT` byte-equality.
2. `cargo test -p wasm-api` is green (bridge count validation unchanged) and `cargo run -p replay-check -- --game briar_circuit --all` passes.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. Valid four-seat setup and the above/below invalid diagnostics are byte-identical to baseline.
2. No pass/dealer/partnership rule moves into the structural helper; the diagnostic stays game-local.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/src/setup.rs` test module (or `tests/`) — add/strengthen a valid/invalid count-table test asserting the diagnostic bytes.

### Commands

1. `cargo test -p briar_circuit`
2. `cargo run -p replay-check -- --game briar_circuit --all`
3. The per-game test plus bridge test are the correct boundary: count validation is game-local structure over a stdlib helper.

## Outcome

Completed: 2026-06-24

What changed:
- Added `validate_standard_seat_count` in Briar setup, backed by `SeatCountRange::inclusive(4, 4).validate` and mapped back to the existing `BC_UNSUPPORTED_SEAT_COUNT` diagnostic.
- Routed `setup_match` and the WASM bridge precheck through the game-owned structural validator while preserving the bridge diagnostic JSON.
- Strengthened the Briar invalid-count test to assert exact diagnostic message bytes for every non-four count in the table.

Deviations:
- None. Pass direction, dealer, partnership, and trick policy remain game-local and unchanged.

Verification:
- `cargo fmt --all --check`
- `cargo test -p briar_circuit`
- `cargo test -p wasm-api`
- `cargo run -p replay-check -- --game briar_circuit --all`
- `bash scripts/boundary-check.sh`
