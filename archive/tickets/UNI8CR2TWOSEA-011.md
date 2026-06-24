# UNI8CR2TWOSEA-011: Masked Claims — canonical seat parser adoption

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/masked_claims/src/ids.rs`; delegates canonical acceptance to `engine-core` `SeatId::parse_canonical`
**Deps**: 001

## Problem

Spec §3.5 / task `8C-R2-204`: `MaskedClaimsSeat::parse` manually matches `seat_0`/`seat_1`. R2 migrates only canonical acceptance to `SeatId::parse_canonical`, then maps indices 0/1 to the typed enum. The parser must keep rejecting `seat-0`, symbolic aliases, ambiguous labels, out-of-range IDs, leading-zero variants, Unicode lookalikes, and role names. Masked's WASM adapter/output is already canonical (no output flip).

## Assumption Reassessment (2026-06-23)

1. `games/masked_claims/src/ids.rs::MaskedClaimsSeat::parse` exists (confirmed ~line 55) and currently matches `"seat_0"`/`"seat_1"` manually.
2. Spec §3.5: parser `migrate`; Masked's adapter/output are `already-discharged-by-8C-pilot`/already canonical (`masked_seats()`/`trace_masked_seat` emit underscore IDs); §9 forbids an output flip.
3. Cross-crate boundary under audit: `engine-core::SeatId::parse_canonical` (`crates/engine-core/src/lib.rs:58`) — canonical acceptance authority; index→enum mapping stays game-local.
4. Determinism / §2 authority: TypeScript never normalizes a seat ID; the canonical accept/reject set is unchanged and no existing trace/golden byte changes.

## Architecture Check

1. Delegating canonical acceptance to the `engine-core` authority removes duplicated manual matching while keeping the typed enum mapping local — consistent across the four parsers.
2. No backwards-compat alias; the manual match is replaced.
3. `engine-core` stays noun-free; no `game-stdlib` change.

## Verification Layers

1. Canonical-acceptance + strict-rejection vectors -> `cargo test -p masked_claims` (parser unit tests).
2. No existing trace/golden diff -> deterministic replay-hash check (`replay-check --game masked_claims --all`).
3. `SeatId::parse_canonical` adoption -> codebase grep-proof in `ids.rs`.

## What to Change

### 1. Delegate canonical acceptance

Replace the manual `seat_0`/`seat_1` match with a delegation to `SeatId::parse_canonical`, then map index 0/1 to `MaskedClaimsSeat`.

### 2. Add rejection-vector tests

Add canonical, out-of-range, leading-zero, alias, Unicode-lookalike, and role-label cases to the parser's unit tests.

## Files to Touch

- `games/masked_claims/src/ids.rs` (modify)

## Out of Scope

- The WASM adapter/output (already canonical; verified in `-012`); no output flip.
- Any trace/golden change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p masked_claims` green, with the new accept/reject vectors.
2. `cargo run -p replay-check -- --game masked_claims --all` — no trace/golden diff.

### Invariants

1. The parser rejects `seat-0`, symbolic aliases, ambiguous/out-of-range/leading-zero IDs, Unicode lookalikes, and role names.
2. Legacy aliases are accepted only by the `wasm-api` import adapter, never inside the game crate.

## Test Plan

### New/Modified Tests

1. `games/masked_claims/src/ids.rs` — unit tests for canonical acceptance and strict rejection vectors.

### Commands

1. `cargo test -p masked_claims`
2. `cargo run -p replay-check -- --game masked_claims --all`

## Outcome

Completed on 2026-06-23. `MaskedClaimsSeat::parse` now delegates canonical
seat grammar acceptance to `SeatId::parse_canonical`, then maps the parsed
zero-based index to the local two-seat enum. Masked Claims' WASM adapter/output
remain unchanged and already canonical.

Added parser coverage for canonical `seat_0`/`seat_1` acceptance plus
out-of-range, leading-zero, hyphen, symbolic alias, Unicode lookalike, and role
label rejection.

Verification passed:

1. `cargo fmt --all --check`
2. `cargo test -p masked_claims`
3. `cargo run -p replay-check -- --game masked_claims --all`
