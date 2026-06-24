# 8CR3PUBCOOASY-102: C-01 Plain Tricks seat-private effect constructor

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/plain_tricks/src/effects.rs`
**Deps**: 8CR3PUBCOOASY-101

## Problem

`plain_tricks` is the only R3 game with real seat-private effects (the private
deal). It builds the private envelope with a game-local literal instead of the
shipped `EffectEnvelope::private_to`. C-01 adopts the canonical seat-private
constructor while the recipient identity, filtered deal payload, and reveal
policy stay game-owned. Because this surface carries hidden information, the
migration must preserve the exact recipient `SeatId`, filtering, and every
no-leak surface.

## Assumption Reassessment (2026-06-24)

1. `games/plain_tricks/src/effects.rs::private_effect` (line ~69) and its caller
   `hand_dealt_effect` (line ~114) build the seat-private envelope locally; the
   shipped `EffectEnvelope::private_to(seat_id, payload)` exists at
   `crates/engine-core/src/lib.rs:158`.
2. Spec §3.4 verdict for `plain_tricks` C-01 private is `migrate`; §5.3 task
   `8C-R3-102` scopes `private_effect` and its callers including
   `hand_dealt_effect`. The other three games have no seat-private effect class
   (private N/A, recorded by the register ticket 802).
3. Cross-crate boundary under audit: `EffectEnvelope::private_to` vs the local
   private wrapper — only envelope assembly moves; `hand_dealt_effect` continues
   to decide the owner `SeatId` and the dealt cards.
4. FOUNDATIONS §11 no-leak firewall motivates this ticket: a seat-private fact
   must reach only its owning viewer, never the public observer, opponent,
   effect log, bot explanation, or export.
5. Enforcement surfaces: the seat-private view/export and filtered effect stream
   (`tests/visibility.rs`, `tests/replay.rs`); the migration must keep them
   byte-identical to baseline and leak no hidden card. This depends on 101 only
   to serialize the shared `effects.rs` edit (no semantic dependency).

## Architecture Check

1. Adopting `EffectEnvelope::private_to` shares the recipient-tagged envelope
   assembly without moving any reveal/recipient decision; cleaner than a local
   private constructor duplicating the engine contract.
2. No backwards-compatibility alias — the local private literal is replaced.
3. `engine-core` already owns the seat-private envelope contract; no mechanic
   noun added, no `game-stdlib` change.

## Verification Layers

1. Recipient identity preserved -> `tests/visibility.rs` seat-private view
   asserts the correct owner sees the deal; opponent/observer do not.
2. No-leak firewall -> pairwise visibility test: hand cards absent for
   observer/opponent before authorized reveal (byte-identical to baseline).
3. Byte/hash neutrality -> `replay-check --game plain_tricks --all` +
   serialization tests unchanged from 001 baseline.

## What to Change

### 1. Adopt `EffectEnvelope::private_to`

In `games/plain_tricks/src/effects.rs::private_effect`, construct the envelope
via `EffectEnvelope::private_to(seat_id, payload)`. Leave `hand_dealt_effect`'s
owner-choice and payload formation untouched — it still supplies the `SeatId`
and dealt cards.

## Files to Touch

- `games/plain_tricks/src/effects.rs` (modify)
- `games/plain_tricks/tests/visibility.rs` (modify; only if an assertion references the constructor path — otherwise unchanged)

## Out of Scope

- The public constructor (101) and any other game's effects.
- Changing recipient selection, deal filtering, or reveal timing.
- Broadening the owner view/export or exposing a hidden card.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks`
2. `cargo run -p replay-check -- --game plain_tricks --all` — byte-identical to baseline.
3. Seat-private no-leak visibility test passes with hidden cards absent for non-owners.

### Invariants

1. The recipient `SeatId`, filtered deal effects, seat-private view/export
   bytes, and effect hash are unchanged from baseline.
2. No hidden hand/tail card reaches observer, opponent, or any public surface.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the existing seat-private visibility/replay suites and the 001 baseline are the regression guard.`

### Commands

1. `cargo test -p plain_tricks`
2. `cargo run -p replay-check -- --game plain_tricks --all`
3. A per-game `replay-check` plus the visibility suite is the correct boundary:
   only `plain_tricks` private effects change.

## Outcome

Completed: 2026-06-24

Changed `games/plain_tricks/src/effects.rs::private_effect` to construct
seat-private effect envelopes via
`EffectEnvelope::private_to(owner_seat_id, payload)`. The change is
constructor-only: `hand_dealt_effect` still chooses the same owner `SeatId` and
dealt-card payload, and filtering/reveal/export policy remains game-owned.

Deviations: none.

Verification:

- `cargo test -p plain_tricks` passed, including seat-private visibility and
  effect-scope tests.
- `cargo run -p replay-check -- --game plain_tricks --all` passed.
- `cargo run -p fixture-check -- --game plain_tricks` passed.
- No golden trace, fixture, export, or test file changed.
