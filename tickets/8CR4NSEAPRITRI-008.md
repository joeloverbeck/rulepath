# 8CR4NSEAPRITRI-008: Vow Tide C-02 canonical seat parser adoption

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/vow_tide` (`src/ids.rs`); accepted/rejected seat inputs and enum mapping unchanged
**Deps**: 8CR4NSEAPRITRI-001

## Problem

`games/vow_tide/src/ids.rs::VowTideSeat::parse` hand-matches `seat_0…seat_6` instead of delegating strict canonical parsing to `SeatId::parse_canonical` (MSC-8C-002). This is a genuine `migrate`, not pilot credit (grounded correction 2: Vow's parser is manual at `0d01901`). Adopt the canonical helper plus a bounded enum conversion with identical 0–6 acceptance, rejection, and enum mapping (spec §3.5 Vow parser, §5.4).

## Assumption Reassessment (2026-06-24)

1. `games/vow_tide/src/ids.rs::VowTideSeat::parse` currently matches `seat_0`…`seat_6` literally and returns `None` otherwise (confirmed directly during `/reassess-spec`); `SeatId::parse_canonical` exists in `crates/engine-core/src/lib.rs`.
2. Spec §3.5 + grounded correction 2 classify this as `migrate` (not `already-discharged-by-8C-pilot`); register MSC-8C-002 / `UNI8CMECSCA-009` own the canonical grammar.
3. Cross-artifact: the canonical seat-string contract lives in `engine-core::SeatId`; the formatter/roster surface is migrated in `-009`, WASM import aliases in `-010`. Baseline accepted/rejected strings come from `-001`.
4. §11 acceptance invariant motivates this ticket: the accepted set stays exactly `seat_0…seat_6` with 0–6 bounds, and malformed/out-of-range strings remain rejected.
5. Enforcement surface = seat-string parse acceptance + enum mapping in the Vow ID tests; delegation changes no accepted/rejected string and no canonical output.

## Architecture Check

1. Delegating strict canonical parse to the kernel helper removes a hand-rolled seven-arm match and routes acceptance through the single owned grammar.
2. No backwards-compatibility aliasing is introduced; hyphen aliases stay an import-only concern owned by `-010`.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4).

## Verification Layers

1. Accepted/rejected seat set unchanged -> table-driven canonical/malformed/out-of-range parse test in the Vow ID tests.
2. Enum mapping (`seat_N` -> `VowTideSeat::SeatN`, 0–6) preserved -> codebase grep-proof + focused unit assertion.
3. Canonical helper adopted -> codebase grep-proof (`SeatId::parse_canonical` present in `parse`; the manual `seat_0…seat_6` match gone).

## What to Change

### 1. Delegate strict parse to `SeatId::parse_canonical`

In `VowTideSeat::parse`, parse via `SeatId::parse_canonical`, then map the bounded 0–6 index to the `VowTideSeat` enum, rejecting out-of-range exactly as today. Preserve the `Option` return contract.

## Files to Touch

- `games/vow_tide/src/ids.rs` (modify)

## Out of Scope

- The formatter/roster migration (`-009`) and WASM import-alias adapter (`-010`).
- Applying C-02 to card, trick, contract, or other non-seat IDs.
- Any canonical output-string change or new alias acceptance.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide` is green, including a table-driven parse test (canonical 0–6 accepted, malformed/out-of-range rejected).
2. `cargo run -p replay-check -- --game vow_tide --all` passes (seat round-trips unchanged).
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The accepted seat set is exactly `seat_0…seat_6`; rejection of malformed/out-of-range inputs is unchanged.
2. No new public symbol or shim is introduced; enum mapping is identical.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/src/ids.rs` test module — add/strengthen a table-driven canonical/malformed/out-of-range parse assertion.

### Commands

1. `cargo test -p vow_tide`
2. `cargo run -p replay-check -- --game vow_tide --all`
3. The per-game test is the correct boundary: seat parsing is game-local ID grammar over a kernel helper.
