# 8CR4NSEAPRITRI-007: Briar Circuit C-02 WASM import-alias adapter

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `crates/wasm-api` (`src/seats.rs`, `src/games/briar.rs`); import aliases accepted, canonical output unchanged
**Deps**: 8CR4NSEAPRITRI-001, 8CR4NSEAPRITRI-005, 8CR4NSEAPRITRI-006

## Problem

`crates/wasm-api/src/games/briar.rs::parse_briar_seat` hand-matches legacy `seat-0…seat-3` hyphen aliases at the WASM import boundary instead of routing through the bounded shared import adapter in `crates/wasm-api/src/seats.rs` (MSC-8C-002). Add a bounded Briar adapter and replace the local match, keeping aliases import-only with canonical underscore output unchanged (spec §3.5 Briar import aliases, §5.4).

## Assumption Reassessment (2026-06-24)

1. `crates/wasm-api/src/games/briar.rs::parse_briar_seat` currently accepts `seat-0…seat-3` via a local match; the shared `crates/wasm-api/src/seats.rs::{parse_seat_import, parse_seat_enum}` helpers exist. Confirmed during `/reassess-spec`.
2. Spec §3.5 classifies this as `migrate`; register MSC-8C-002 / `UNI8CMECSCA-009` own the import boundary; the game-level parser/formatter are migrated in `-005`/`-006` (this ticket `Deps` both, since the game parser precedes its WASM adapter per spec §5.4).
3. Cross-artifact: the import-compatibility contract lives in `crates/wasm-api/src/seats.rs`; canonical output remains Rust-authored. Accepted alias baseline comes from `-001`.
4. §2 behavior-authority motivates this ticket: TypeScript performs no seat repair/normalization; aliases are accepted only at the Rust import boundary and canonical output never flips to hyphen form.
5. Enforcement surface = WASM seat-import acceptance + canonical output; routing through the shared adapter keeps canonical/`seat-N` imports accepted and underscore output unchanged, leaking nothing and changing no output byte.

## Architecture Check

1. Routing legacy aliases through the shared bounded adapter removes a per-game hyphen match and consolidates import compatibility in one owned WASM seat surface.
2. No backwards-compatibility aliasing is added beyond the existing bounded `seat-0…seat-3` set; no new symbolic alias is inferred and no shim wraps canonical output.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4); TypeScript decides no legality (§2).

## Verification Layers

1. Canonical + `seat-0…seat-3` imports accepted, malformed/out-of-range rejected -> table-driven import test in the `wasm-api` Briar tests.
2. Canonical output remains underscore form (no output flip) -> schema/serialization validation + codebase grep-proof.
3. Adapter adopted -> codebase grep-proof (`parse_briar_seat` routes through `seats.rs`; the local hyphen match gone).

## What to Change

### 1. Add a bounded Briar import adapter

Add a bounded Briar adapter over `crates/wasm-api/src/seats.rs::{parse_seat_import, parse_seat_enum}` and replace `parse_briar_seat`'s local hyphen-alias match with a call to it. Keep `seat-0…seat-3` import-only; emit canonical underscore output unchanged.

## Files to Touch

- `crates/wasm-api/src/seats.rs` (modify)
- `crates/wasm-api/src/games/briar.rs` (modify)

## Out of Scope

- The game-level parser/formatter migrations (`-005`/`-006`).
- Any TypeScript seat repair, normalization, or legality decision (§2).
- Adding any new alias spelling or flipping canonical output to hyphen form.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` is green, including a table-driven import test (canonical + `seat-0…seat-3` accepted; malformed/out-of-range rejected).
2. `cargo run -p replay-check -- --game briar_circuit --all` passes with canonical output unchanged.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. Legacy hyphen aliases are accepted only at import; canonical output stays underscore form.
2. No new public symbol or shim is introduced; rollback removes only the Briar adapter.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api` Briar seat test module — add/strengthen a table-driven import-alias acceptance + canonical-output assertion.

### Commands

1. `cargo test -p wasm-api`
2. `cargo run -p replay-check -- --game briar_circuit --all`
3. The `wasm-api` test is the correct boundary: legacy alias acceptance is a WASM-import concern, not a game-rule one.

## Outcome

Completed: 2026-06-24

What changed:
- Added `parse_briar_circuit_seat` in the shared WASM seat adapter, bounded to four seats and backed by `parse_seat_enum`.
- Replaced `crates/wasm-api/src/games/briar.rs::parse_briar_seat`'s local `seat-0` through `seat-3` match with the shared adapter.
- Added a focused import-boundary test covering canonical and hyphen inputs for seats 0 through 3, malformed/out-of-range rejection, and underscore canonical output.

Deviations:
- None. Game-level parser/formatter migrations remain in `8CR4NSEAPRITRI-005` and `8CR4NSEAPRITRI-006`; no TypeScript repair or output alias was added.

Verification:
- `cargo fmt --all --check`
- `cargo test -p wasm-api`
- `cargo run -p replay-check -- --game briar_circuit --all`
- `bash scripts/boundary-check.sh`
