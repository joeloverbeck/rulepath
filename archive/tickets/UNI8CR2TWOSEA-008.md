# UNI8CR2TWOSEA-008: High Card Duel — canonical seat parser adoption

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/high_card_duel/src/ids.rs`; delegates canonical acceptance to `engine-core` `SeatId::parse_canonical`
**Deps**: 001

## Problem

Spec §3.5 / task `8C-R2-201`: `HighCardDuelSeat::parse` manually matches `seat_0`/`seat_1`. R2 migrates only canonical acceptance to `SeatId::parse_canonical`, then maps indices 0/1 to the typed enum. The parser must keep rejecting `seat-0`, symbolic aliases, ambiguous labels, out-of-range IDs, leading-zero variants, Unicode lookalikes, and role names. Legacy aliases remain accepted only by the `wasm-api` import adapter (not this crate).

## Assumption Reassessment (2026-06-23)

1. `games/high_card_duel/src/ids.rs::HighCardDuelSeat::parse` exists (confirmed ~line 122) and currently matches `"seat_0"`/`"seat_1"` manually.
2. Spec §3.5: parser `migrate`; the WASM import adapter is `already-discharged-by-8C-pilot`; §9 forbids changing the WASM runtime roster in a parser task and forbids any output flip.
3. Cross-crate boundary under audit: `engine-core::SeatId::parse_canonical` (`crates/engine-core/src/lib.rs:58`) — the canonical acceptance authority; index→enum mapping stays game-local.
4. Determinism / §2 authority: TypeScript never repairs or normalizes a seat ID; the canonical accept/reject set is unchanged and no existing trace/golden byte changes.

## Architecture Check

1. Delegating canonical acceptance to the `engine-core` authority removes duplicated manual matching while keeping the typed enum mapping local — cleaner and consistent across the four games' parsers.
2. No backwards-compat alias; the manual match is replaced, and legacy aliases stay only in the `wasm-api` adapter.
3. `engine-core` stays noun-free; no `game-stdlib` change.

## Verification Layers

1. Canonical-acceptance + strict-rejection vectors -> `cargo test -p high_card_duel` (parser unit tests for `seat-0`, aliases, out-of-range, leading-zero, lookalikes, role labels).
2. No existing trace/golden diff -> deterministic replay-hash check (`replay-check --game high_card_duel --all`).
3. `SeatId::parse_canonical` adoption -> codebase grep-proof in `ids.rs`.

## What to Change

### 1. Delegate canonical acceptance

Replace the manual `seat_0`/`seat_1` match with a delegation to `SeatId::parse_canonical`, then map index 0/1 to `HighCardDuelSeat`.

### 2. Add rejection-vector tests

Add canonical, out-of-range, leading-zero, alias, Unicode-lookalike, and role-label cases to the parser's unit tests.

## Files to Touch

- `games/high_card_duel/src/ids.rs` (modify)

## Out of Scope

- The WASM import adapter / legacy runtime roster (that is `-012`); no output flip.
- Any trace/golden change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p high_card_duel` green, with the new accept/reject vectors.
2. `cargo run -p replay-check -- --game high_card_duel --all` — no trace/golden diff.

### Invariants

1. The parser rejects `seat-0`, symbolic aliases, ambiguous/out-of-range/leading-zero IDs, Unicode lookalikes, and role names.
2. Legacy aliases are accepted only by the `wasm-api` import adapter, never inside the game crate.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/src/ids.rs` — unit tests for canonical acceptance and strict rejection vectors.

### Commands

1. `cargo test -p high_card_duel`
2. `cargo run -p replay-check -- --game high_card_duel --all`

## Outcome

Completed on 2026-06-23. `HighCardDuelSeat::parse` now delegates canonical
seat grammar acceptance to `SeatId::parse_canonical`, then maps the parsed
zero-based index to the local two-seat enum. Legacy hyphen and symbolic aliases
remain outside the game crate in the WASM import adapter.

Added parser coverage for canonical `seat_0`/`seat_1` acceptance plus
out-of-range, leading-zero, hyphen, symbolic alias, Unicode lookalike, and role
label rejection.

Verification passed:

1. `cargo fmt --all --check`
2. `cargo test -p high_card_duel`
3. `cargo run -p replay-check -- --game high_card_duel --all`
