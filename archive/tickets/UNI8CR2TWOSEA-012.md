# UNI8CR2TWOSEA-012: WASM seat-compatibility audit and legacy-roster exception receipt

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (compatibility evidence) — `crates/wasm-api/src/seats.rs` and `crates/wasm-api/src/games/{high_card,secret,poker,masked}.rs`; verification/tests only, no output flip
**Deps**: 001

## Problem

Spec §3.5 / task `8C-R2-205`: verify the bounded legacy-alias import adapter, confirm Masked's canonical output, and record the legacy HCD/Secret/Poker runtime-roster spellings as an explicit accepted `exception` (owned by `wasm-api`, preserved through C-11). No default output flip — changing runtime `SeatId` bytes would touch state/effect visibility and hashes, which is out of scope for this wave.

## Assumption Reassessment (2026-06-23)

1. `parse_high_card_seat`/`parse_secret_seat`/`parse_poker_seat`/`parse_masked_seat` exist in `crates/wasm-api/src/seats.rs` and delegate through `bounded_canonical_seat` → `parse_seat_import`; `high_card_replay_to_cursor` (`src/games/high_card.rs:115`) uses legacy `seats()`; `trace_high_card_seat` (`seats.rs:156`) emits `seat-0`/`seat-1`; `masked_seats()`/`trace_masked_seat` emit canonical underscore IDs — all confirmed in the reassess pass.
2. Spec §3.5/§9: the HCD/Secret/Poker runtime roster stays an accepted `exception`; the next trigger is a dedicated WASM runtime-seat migration; this ticket performs no output flip.
3. Cross-crate boundary under audit: the `wasm-api` import adapter (`parse_seat_import`/`bounded_canonical_seat`) — the only surface allowed to accept legacy aliases; game crates stay canonical-only.
4. Determinism / no-leak: flipping the legacy runtime roster to canonical bytes would change `SeatId` bytes feeding state/effect visibility and hashes (§11), so it is recorded as an exception here, not migrated; recording the receipt changes no canonical byte.

## Architecture Check

1. A single audit-and-receipt ticket keeps the legacy-roster exception owned and triggered in one place rather than smuggled into a parser task — cleaner and matches the spec's one-surface-per-diff admission rule.
2. No backwards-compat alias is added; the existing import adapter is the only alias path, recorded read-only.
3. `engine-core` is untouched; no `game-stdlib` change.

## Verification Layers

1. Alias import still accepted; Masked output canonical -> `cargo test -p wasm-api` (seat import/output tests).
2. No runtime `SeatId` byte/hash change -> deterministic replay-hash check (`replay-check --all` per game) + grep-proof that `seats()`/`trace_high_card_seat` spellings are unchanged.
3. Legacy-roster exception recorded with owner/compat/rollback/trigger -> FOUNDATIONS alignment check against spec §3.5.

## What to Change

### 1. Alias-import and canonical-output receipts

Add/confirm `wasm-api` tests proving the import adapter accepts the legacy aliases, Masked emits canonical IDs, and the HCD/Secret/Poker runtime roster spellings are unchanged.

### 2. Record the legacy-roster exception

Capture the exception owner, compatibility window, rollback boundary, and next trigger as evidence for the `-045` register receipt (`MSC-8C-002`).

## Files to Touch

- `crates/wasm-api/src/seats.rs` (modify) — alias-import + legacy-roster receipt tests
- `crates/wasm-api/src/games/high_card.rs` (modify; evidence-only)
- `crates/wasm-api/src/games/secret.rs` (modify; evidence-only)
- `crates/wasm-api/src/games/poker.rs` (modify; evidence-only)
- `crates/wasm-api/src/games/masked.rs` (modify; evidence-only)

## Out of Scope

- Any runtime-roster output flip or `SeatId` byte change (that is a separately characterized future migration).
- The four game-crate parsers (`-008`…`-011`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` green, with alias-import and canonical-output assertions.
2. `cargo run -p replay-check -- --game high_card_duel --all` (and the other three) — no seat/state/effect byte change.

### Invariants

1. The import adapter is the only surface accepting legacy aliases.
2. No runtime `SeatId` byte, state hash, or effect hash changes in this diff.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/seats.rs` — alias-acceptance and canonical-output unit tests; legacy-roster spelling guards.

### Commands

1. `cargo test -p wasm-api`
2. `cargo run -p replay-check -- --game masked_claims --all`

## Outcome

Completed on 2026-06-23. Added focused `wasm-api` seat tests proving the
bounded import adapter remains the only legacy alias surface for the four Unit
8C games, Masked Claims output helpers emit canonical underscore IDs, and the
existing legacy roster/trace helper spellings are guarded without any output
flip.

The HCD/Secret/Poker runtime roster exception remains owned by `wasm-api`; the
next trigger is a dedicated WASM runtime-seat migration because changing those
`SeatId` bytes would affect state/effect visibility and replay hashes.

Verification passed:

1. `cargo fmt --all --check`
2. `cargo test -p wasm-api`
3. `cargo run -p replay-check -- --game high_card_duel --all`
4. `cargo run -p replay-check -- --game secret_draft --all`
5. `cargo run -p replay-check -- --game poker_lite --all`
6. `cargo run -p replay-check -- --game masked_claims --all`
