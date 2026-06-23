# 8CR1PUBFIXSEA-023: Token Bazaar C-03 exact seat-count validation + game-stdlib dependency

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/token_bazaar` (`Cargo.toml` normal dep + `src/setup.rs`); diagnostic bytes and setup state byte-identical
**Deps**: 8CR1PUBFIXSEA-001

## Problem

`games/token_bazaar/src/setup.rs::setup_match` validates seat count with a hand-written predicate (`seats.len() != options.variant.seat_count as usize`) and the crate does not yet depend on `game-stdlib`. Add a normal `game-stdlib` dependency and adopt `SeatCountRange::inclusive(...).validate(...)` (MSC-8C-003), preserving the `invalid_seat_count` diagnostic code, message, accepted count, ordering, and setup state exactly. No additional production dependency beyond `game-stdlib`; the game keeps its typed `other()` mapping.

## Assumption Reassessment (2026-06-23)

1. `setup_match` currently uses `if seats.len() != options.variant.seat_count as usize { … invalid_seat_count … }`; `games/token_bazaar/Cargo.toml` does NOT list `game-stdlib` (the other five games do); `SeatCountRange::inclusive` + `.validate` exist in `crates/game-stdlib/src/seat.rs`. Confirmed during reassessment.
2. Spec §3.5 and §5.6 (task `8C-R1-305`) classify this as `migrate` / ADR-0009 `unchanged` and explicitly call for the new normal `game-stdlib` dependency; MSC-8C-003 owns seat-count validation. Replacing typed `other()` with generic ring arithmetic is forbidden churn (spec §9).
3. Cross-artifact: the helper is `game-stdlib::seat`; the game owns its diagnostic mapping and setup state. Before-baseline from `-001`.
4. §4 (`game-stdlib` is earned) and §11 motivate this ticket: adoption of an already-shipped promotion for count/range validity only.
5. Enforcement surface = the `invalid_seat_count` diagnostic bytes and setup-state hashes; the helper decides only count/range validity — no hash drift, no hidden-information leak. `cargo tree` must show no production dependency added beyond `game-stdlib`.
6. This extends the crate's dependency graph (`Cargo.toml`): the addition is additive (a new normal dependency on an existing workspace crate), with `cargo tree` as the consumer-side proof that no transitive production surface beyond `game-stdlib` enters.

## Architecture Check

1. Adopting the shared count validator removes a hand-rolled predicate while keeping the game's diagnostic/state ownership intact; the dependency on `game-stdlib` matches the other five games.
2. No backwards-compatibility shim is introduced; the predicate is replaced in place.
3. `engine-core` stays noun-free (§3); `game-stdlib::seat` is an earned, already-promoted helper (§4) — adoption only.

## Verification Layers

1. Valid exact-two setup unchanged -> unit test (setup equality) + replay-hash check.
2. Wrong-count diagnostic equality (code + message + accepted count) -> focused diagnostic-bytes assertion.
3. No extra production dependency; no ordering/state/hash drift -> `cargo tree -p token_bazaar -e normal` + `replay-check --game token_bazaar --all`.

## What to Change

### 1. Add the `game-stdlib` dependency

Add `game-stdlib = { path = "../../crates/game-stdlib" }` to `games/token_bazaar/Cargo.toml` `[dependencies]`.

### 2. Adopt the count validator

Replace the hand-written predicate in `setup_match` with `SeatCountRange::inclusive(...).validate(seats.len())`, mapping its result to the existing `invalid_seat_count` diagnostic. Preserve diagnostic bytes, accepted count, and setup state exactly.

## Files to Touch

- `games/token_bazaar/Cargo.toml` (modify)
- `games/token_bazaar/src/setup.rs` (modify)

## Out of Scope

- Replacing the typed `other()` mapping with generic `usize` ring arithmetic.
- Adding any production dependency other than `game-stdlib`.
- Any change to the `invalid_seat_count` code/message or setup state.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p token_bazaar` is green, including a wrong-count diagnostic-equality assertion.
2. `cargo tree -p token_bazaar -e normal` shows `game-stdlib` as the only newly-added production dependency.
3. `cargo run -p replay-check -- --game token_bazaar --all` passes with setup-state and replay hashes unchanged.

### Invariants

1. The `invalid_seat_count` diagnostic bytes, accepted count, ordering, and setup state are byte-identical.
2. No production dependency beyond `game-stdlib` is added; no generic ring arithmetic replaces the typed `other()` mapping.

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/` setup test module — valid-count setup equality + wrong-count diagnostic-bytes assertion across the helper adoption.

### Commands

1. `cargo test -p token_bazaar`
2. `cargo tree -p token_bazaar -e normal`
3. The per-game setup tests plus the `cargo tree` production-edge proof are the correct boundary: validity is game-local and the dependency addition must stay production-minimal.

## Outcome

Completed on 2026-06-23.

- Added `game-stdlib` as the only new normal dependency for `games/token_bazaar`; `Cargo.lock` records that dependency edge.
- `games/token_bazaar/src/setup.rs::setup_match` now validates the fixed two-seat range with `SeatCountRange::inclusive(...).validate(...)`.
- The game-owned `invalid_seat_count` diagnostic mapping remains unchanged, and the focused wrong-count test now pins both diagnostic code and message.
- The typed `TokenBazaarSeat::other()` mapping and setup ordering/state construction were left unchanged.

Verification:

- `cargo fmt --all -- --check`
- `cargo test -p token_bazaar`
- `cargo tree -p token_bazaar -e normal` showed `ai-core`, `engine-core`, and the newly added `game-stdlib` normal dependency; `game-stdlib` only brings `engine-core`.
- `cargo run -p replay-check -- --game token_bazaar --all`
