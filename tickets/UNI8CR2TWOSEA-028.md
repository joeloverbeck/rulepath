# UNI8CR2TWOSEA-028: High Card Duel — replay-command-v1 profile driver

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/high_card_duel/tests/replay.rs`, `games/high_card_duel/src/replay_support.rs`; adopts `game-test-support` `ReplayCommandV1Driver` (internal-dev)
**Deps**: 017

## Problem

Spec §3.9 / task `8C-R2-601`: add a `replay-command-v1` profile driver for High Card Duel. The driver validates metadata and delegates to the existing internal command-trace validator; the current internal trace bytes remain the authority (`canonical_byte_authority: none`). It is `internal-dev`, parses no commands, and rewrites no artifact. Shares `replay_support.rs` with `-017` (hence `Deps: 017`).

## Assumption Reassessment (2026-06-23)

1. `game-test-support` exposes `ReplayCommandV1Driver` (`crates/game-test-support/src/profiles.rs:96`) and the `REPLAY_COMMAND_V1` = `"replay-command-v1"` id; HCD already carries the dev-dependency.
2. Spec §3.9/§9: `migrate`; `internal-dev`; current internal trace bytes remain authority; no new omniscient export; no profile task may rewrite an artifact merely to add metadata.
3. Cross-crate boundary under audit: `game-test-support::profiles::ReplayCommandV1Driver` — it asserts metadata and calls an existing validator; the game owns the trace bytes.
4. Determinism / no-leak: the driver delegates to the existing internal command-trace validator with byte equality to the `-001` baseline; the internal command trace is `internal-dev`, not a viewer surface (§11).

## Architecture Check

1. A thin profile driver over the existing trace validator adds typed evidence without a new canonical-byte authority — cleaner than a bespoke check and consistent with the other games.
2. No backwards-compat alias; no artifact rewrite; the driver is additive test evidence.
3. `engine-core` untouched; the driver is dev-only `game-test-support`.

## Verification Layers

1. Driver validates metadata and rejects wrong id/owner/visibility/fields -> profile driver test (`cargo test -p high_card_duel`).
2. Existing internal trace bytes unchanged -> deterministic replay-hash check (`replay-check --game high_card_duel --all`).
3. `ReplayCommandV1Driver` adoption, `canonical_byte_authority: none` -> codebase grep-proof in `tests/replay.rs`.

## What to Change

### 1. Add the replay-command-v1 driver test

In `tests/replay.rs`, invoke `ReplayCommandV1Driver` over the existing internal command trace (adding a thin `replay_support.rs` evidence accessor only if required), asserting valid metadata and wrong-profile/owner/field rejection.

## Files to Touch

- `games/high_card_duel/tests/replay.rs` (modify)
- `games/high_card_duel/src/replay_support.rs` (modify; serialized after `-017`)

## Out of Scope

- Inventing an omniscient export; rewriting any trace/fixture; the public-export profile (`-036`).
- Any internal-trace byte change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p high_card_duel` green, including the profile driver test.
2. `cargo run -p replay-check -- --game high_card_duel --all` — internal trace bytes byte-identical to baseline.

### Invariants

1. The driver claims no new canonical bytes; the existing validator remains the authority.
2. The internal command trace stays internal-dev (no viewer surface gains it).

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/replay.rs` — `replay-command-v1` driver metadata + rejection test.

### Commands

1. `cargo test -p high_card_duel`
2. `cargo run -p replay-check -- --game high_card_duel --all`
