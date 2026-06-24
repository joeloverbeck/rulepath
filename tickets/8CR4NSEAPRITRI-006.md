# 8CR4NSEAPRITRI-006: Briar Circuit C-02 canonical formatter/roster adoption

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/briar_circuit` (`src/ids.rs`); emitted seat strings and roster order unchanged
**Deps**: 8CR4NSEAPRITRI-001, 8CR4NSEAPRITRI-005

## Problem

`games/briar_circuit/src/ids.rs::{BriarCircuitSeat::as_str, seat_id_for_index, canonical_seat_ids}` hand-format canonical seat strings and the fixed-four roster instead of delegating to the shipped canonical formatting/index helpers (MSC-8C-002). Adopt the helpers without changing emitted strings or roster order (spec §3.5 Briar formatter/roster, §5.4).

## Assumption Reassessment (2026-06-24)

1. `BriarCircuitSeat::as_str`, the standalone `seat_id_for_index`, and `canonical_seat_ids` currently format `seat_N` strings and assemble the four-seat roster by hand in `games/briar_circuit/src/ids.rs`; canonical formatting/index helpers exist in `engine-core::SeatId`. Confirmed during `/reassess-spec`.
2. Spec §3.5 classifies this as `migrate`; register MSC-8C-002 / `UNI8CMECSCA-009` own the grammar; the parser is migrated in `-005` (this ticket `Deps` it to serialize edits on the shared `ids.rs`).
3. Cross-artifact: source/API/WASM/trace seat outputs all read these formatters; the emitted-string baseline comes from `-001`.
4. §11 acceptance invariant motivates this ticket: emitted canonical strings (`seat_0…seat_3`) and the declared roster order MUST be byte-identical before/after.
5. Enforcement surface = canonical seat-output strings in traces/exports/WASM; delegation preserves every emitted string, so no trace/export byte changes and no hidden-info path is affected.

## Architecture Check

1. Delegating formatting/index construction to the kernel helper removes a second hand-rolled seat-grammar surface, completing the game-level C-02 parse+format pair.
2. No backwards-compatibility aliasing or shim is introduced; the formatters are replaced, not wrapped. This task does not touch pass targets (game-local).
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4).

## Verification Layers

1. Emitted seat strings unchanged -> golden trace / deterministic replay-hash check (`replay-check --game briar_circuit --all`) plus a focused format assertion.
2. Roster order/length (fixed four) unchanged -> codebase grep-proof + focused `canonical_seat_ids` assertion.
3. Canonical helpers adopted -> codebase grep-proof (canonical formatting/index helpers present; hand-format gone from the three symbols).

## What to Change

### 1. Delegate formatter/roster to canonical helpers

In `as_str`, `seat_id_for_index`, and `canonical_seat_ids`, construct canonical output and indices via the `engine-core::SeatId` formatting/index helpers, preserving every emitted string and the fixed-four roster order exactly.

## Files to Touch

- `games/briar_circuit/src/ids.rs` (modify; serialized after `-005`)

## Out of Scope

- The parser migration (`-005`) and WASM import-alias adapter (`-007`).
- Any change to `next_clockwise` / pass-target methods (game-local dealer/pass topology).
- Changing any emitted seat string or roster order.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit` is green, including a focused canonical-format + roster-order assertion.
2. `cargo run -p replay-check -- --game briar_circuit --all` passes with seat strings byte-identical to the `-001` baseline.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. Emitted canonical seat strings and the fixed-four roster order are unchanged.
2. No new public symbol or shim is introduced; pass-target methods are untouched.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/src/ids.rs` test module — add/strengthen a focused assertion that `as_str` / `seat_id_for_index` / `canonical_seat_ids` emit the baseline strings and roster order.

### Commands

1. `cargo test -p briar_circuit`
2. `cargo run -p replay-check -- --game briar_circuit --all`
3. The per-game replay-check is the correct boundary: canonical seat output is exercised by the game's traces.
