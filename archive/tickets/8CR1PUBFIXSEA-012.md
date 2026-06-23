# 8CR1PUBFIXSEA-012: C-02 output-only canonical seat/roster helper

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `wasm-api` (`src/seats.rs` new output formatter + local tests); no game document flips, no golden change
**Deps**: 8CR1PUBFIXSEA-001

## Problem

The six per-game WASM replay-document outputs still emit legacy seat spellings (five hyphen, one already canonical), and there is no output-only canonical formatter to migrate them onto. This ticket adds a parallel output-only canonical seat/roster helper in `crates/wasm-api/src/seats.rs` (default symbols `canonical_trace_seat_id` / `canonical_seats_for_count`) so each game-document flip (`-013`…`-018`) is a one-line consumer change. It is classified ADR-0009 `parallel-new`: no existing return value or document changes here.

## Assumption Reassessment (2026-06-23)

1. `crates/wasm-api/src/seats.rs` currently defines `trace_race_seat`/`trace_draughts_seat`/`trace_three_seat`/`trace_column_seat`/`trace_directional_seat` returning hyphen `seat-<n>`, `trace_token_seat` returning canonical `seat_<n>`, plus `parse_seat_import`, `seats`, and `seats_for_count`. No `canonical_trace_seat_id`/`canonical_seats_for_count` exists yet. Confirmed during reassessment.
2. Spec §3.4 and §5.5 (task `8C-R1-210`) classify this as `parallel-new`; the helper must leave every existing `trace_*_seat`, `parse_seat_import`, `seats`, and `seats_for_count` untouched. MSC-8C-002 owns the seat grammar.
3. Cross-artifact: this is the shared producer the six output-flip tickets consume; the seat-grammar contract is `engine-core::SeatId`. The before-baseline is `-001`.
4. §11 determinism motivates this ticket: the new formatter must derive canonical IDs from `engine-core` canonical seat identity deterministically, introducing no nondeterministic input.
5. Enforcement surface = the WASM output seat formatting that later golden migrations depend on; adding a parallel formatter changes no canonical byte and leaks no hidden information (all seats are public). The compatibility window keeps old imports readable indefinitely.

## Architecture Check

1. A parallel output formatter lets each game flip independently and reversibly, versus a flag-day rewrite of all six documents in one diff (which the spec forbids).
2. No backwards-compatibility aliasing is removed and no existing function is changed; the helper is purely additive.
3. `engine-core` stays noun-free (§3); the formatter lives in `wasm-api` (the transport boundary), not the kernel; no `game-stdlib` change (§4).

## Verification Layers

1. New helper emits canonical `seat_<n>` from canonical Rust IDs -> new local unit test in `wasm-api`.
2. Existing `trace_*_seat`/`parse_seat_import`/`seats`/`seats_for_count` return values unchanged -> codebase grep-proof + existing `wasm-api` tests stay green.
3. No golden trace changes in this ticket -> `git diff --stat` shows no `wasm-exported.trace.json` touched.

## What to Change

### 1. Add the canonical output formatter

In `crates/wasm-api/src/seats.rs`, add output-only `canonical_trace_seat_id` and `canonical_seats_for_count` deriving canonical `seat_<n>` spellings from `engine-core` canonical seat identity. Characterize the existing `trace_*_seat` functions read-only (do not change them).

### 2. Local tests

Add focused unit tests proving the formatter's canonical output and that no existing function's return value changed.

## Files to Touch

- `crates/wasm-api/src/seats.rs` (modify)

## Out of Scope

- Any game-document flip or `wasm-exported.trace.json` change (owned by `-013`…`-018`).
- Changing `trace_*_seat`, `parse_seat_import`, `seats`, or `seats_for_count`.
- Removing any legacy import reader.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` is green, including the new formatter unit tests.
2. `git diff --stat` shows no `games/*/tests/golden_traces/wasm-exported.trace.json` changed by this ticket.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. Every existing `seats.rs` function returns byte-identical values.
2. The new helper is output-only and derives canonical IDs deterministically from `engine-core` seat identity.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/seats.rs` unit tests — `canonical_trace_seat_id`/`canonical_seats_for_count` canonical output and a no-change assertion for the existing functions.

### Commands

1. `cargo test -p wasm-api`
2. `cargo test --workspace --all-targets`
3. The `wasm-api` unit tests are the correct boundary: this is a transport-layer formatter with no game-document consumer yet.

## Outcome

Completed: 2026-06-23

Changes:

- Added output-only `canonical_trace_seat_id` and `canonical_seats_for_count` helpers in `crates/wasm-api/src/seats.rs`, deriving canonical `seat_<n>` strings through `engine_core::SeatId::from_zero_based_index`.
- Added focused `wasm-api` tests proving the new helpers emit canonical underscore IDs and existing trace/roster helpers retain their legacy outputs.
- Added narrow `#[allow(dead_code)]` attributes to the two additive helpers because their consumers are intentionally staged in follow-up output-flip tickets.

Deviations:

- `cargo fmt --all -- --check` initially failed on rustfmt wrapping in the new tests; `cargo fmt --all` was run and the final format check passed.
- `cargo test --workspace --all-targets` exits 0, but local bench binaries inside that command still print some `pass:false` benchmark rows for existing thresholded smoke output; no test binary failed.

Verification:

- `cargo fmt --all -- --check` passed.
- `cargo test -p wasm-api` passed, including `seats::tests::canonical_output_helpers_emit_underscore_seat_ids` and `seats::tests::existing_trace_and_roster_helpers_keep_legacy_outputs`.
- `cargo test --workspace --all-targets` passed with exit 0.
- `bash scripts/boundary-check.sh` passed.
- `git diff --name-only games | rg 'games/.*/tests/golden_traces/wasm-exported\.trace\.json'` produced no matches, confirming no game document/golden flip happened in this ticket.
