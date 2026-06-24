# 8CR4NSEAPRITRI-010: Vow Tide C-02 WASM import-alias adapter

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `crates/wasm-api` (`src/seats.rs`, `src/games/vow.rs`); import aliases accepted, canonical output unchanged
**Deps**: 8CR4NSEAPRITRI-001, 8CR4NSEAPRITRI-008, 8CR4NSEAPRITRI-009

## Problem

`crates/wasm-api/src/games/vow.rs::parse_vow_seat` hand-accepts legacy hyphen aliases at the WASM import boundary via a local `seat-{n}` search instead of routing through the bounded shared import adapter in `crates/wasm-api/src/seats.rs` (MSC-8C-002). Add a bounded seven-seat Vow adapter and remove the local search, keeping aliases import-only with canonical underscore output unchanged (spec §3.5 Vow import aliases, §5.4).

## Assumption Reassessment (2026-06-24)

1. `crates/wasm-api/src/games/vow.rs::parse_vow_seat` currently accepts hyphen aliases via a local `seat-{n}` search; the shared `crates/wasm-api/src/seats.rs::{parse_seat_import, parse_seat_enum}` helpers exist. Confirmed during `/reassess-spec`.
2. Spec §3.5 classifies this as `migrate`; register MSC-8C-002 / `UNI8CMECSCA-009` own the import boundary; the game-level parser/formatter are migrated in `-008`/`-009` (this ticket `Deps` both, since the game parser precedes its WASM adapter per spec §5.4).
3. Cross-artifact: the import-compatibility contract lives in `crates/wasm-api/src/seats.rs`; canonical output remains Rust-authored. Accepted alias baseline comes from `-001`.
4. §2 behavior-authority motivates this ticket: TypeScript performs no seat repair/normalization; bounded hyphen aliases are accepted only at the Rust import boundary, and no new symbolic alias is inferred.
5. Enforcement surface = WASM seat-import acceptance + canonical output; routing through the shared adapter keeps canonical/bounded-hyphen imports accepted and underscore output unchanged.

## Architecture Check

1. Routing legacy aliases through the shared bounded adapter removes a per-game `seat-{n}` search and consolidates import compatibility in one owned WASM seat surface.
2. No backwards-compatibility aliasing is added beyond the existing bounded hyphen set; no new symbolic alias is inferred and no shim wraps canonical output.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4); TypeScript decides no legality (§2).

## Verification Layers

1. Canonical + bounded hyphen aliases accepted, malformed/out-of-range rejected -> table-driven import test in the `wasm-api` Vow tests.
2. Canonical output remains underscore form (no output flip) -> schema/serialization validation + codebase grep-proof.
3. Adapter adopted -> codebase grep-proof (`parse_vow_seat` routes through `seats.rs`; the local `seat-{n}` search gone).

## What to Change

### 1. Add a bounded seven-seat Vow import adapter

Add a bounded Vow adapter over `crates/wasm-api/src/seats.rs::{parse_seat_import, parse_seat_enum}` and replace `parse_vow_seat`'s local `seat-{n}` search with a call to it. Keep bounded hyphen aliases import-only; emit canonical underscore output unchanged.

## Files to Touch

- `crates/wasm-api/src/seats.rs` (modify)
- `crates/wasm-api/src/games/vow.rs` (modify)

## Out of Scope

- The game-level parser/formatter migrations (`-008`/`-009`).
- Any TypeScript seat repair, normalization, or legality decision (§2).
- Inferring any new alias spelling or flipping canonical output to hyphen form.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` is green, including a table-driven import test (canonical + bounded hyphen accepted; malformed/out-of-range rejected).
2. `cargo run -p replay-check -- --game vow_tide --all` passes with canonical output unchanged.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. Bounded hyphen aliases are accepted only at import; canonical output stays underscore form.
2. No new public symbol or shim is introduced; rollback removes only the Vow adapter.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api` Vow seat test module — add/strengthen a table-driven import-alias acceptance + canonical-output assertion.

### Commands

1. `cargo test -p wasm-api`
2. `cargo run -p replay-check -- --game vow_tide --all`
3. The `wasm-api` test is the correct boundary: legacy alias acceptance is a WASM-import concern, not a game-rule one.

## Outcome

Completed: 2026-06-24

What changed:
- Added `parse_vow_tide_seat` in the shared WASM seat adapter, bounded to seven seats and backed by `parse_seat_enum`.
- Replaced `crates/wasm-api/src/games/vow.rs::parse_vow_seat`'s local `seat-{n}` search with the shared adapter.
- Added a focused import-boundary test covering canonical and hyphen inputs for seats 0 through 6, malformed/out-of-range rejection, and underscore canonical output.

Deviations:
- None. Game-level parser/formatter migrations remain in `8CR4NSEAPRITRI-008` and `8CR4NSEAPRITRI-009`; no TypeScript repair or output alias was added.

Verification:
- `cargo fmt --all --check`
- `cargo test -p wasm-api`
- `cargo run -p replay-check -- --game vow_tide --all`
- `bash scripts/boundary-check.sh`
