# UNI8CR2TWOSEA-030: Poker Lite — replay-command-v1 profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/poker_lite/tests/replay.rs`, `games/poker_lite/src/replay_support.rs`; adopts `game-test-support` `ReplayCommandV1Driver` (internal-dev)
**Deps**: 019, 022

## Problem

Spec §3.9 / task `8C-R2-603`: add a `replay-command-v1` profile driver for Poker Lite. The driver validates metadata and delegates to the existing internal command-trace validator; current internal trace bytes remain the authority (`canonical_byte_authority: none`). It is `internal-dev`, rewrites no artifact. Shares `replay_support.rs` with `-019` and needs the `-022` dev-dependency (hence `Deps: 019, 022`).

## Assumption Reassessment (2026-06-23)

1. `game-test-support` exposes `ReplayCommandV1Driver` (`profiles.rs:96`) and `REPLAY_COMMAND_V1`; Poker gains the dev-dependency in `-022`.
2. Spec §3.9/§9: `migrate`; `internal-dev`; current internal trace bytes remain authority; no omniscient export; no artifact rewrite.
3. Cross-crate boundary under audit: `game-test-support::profiles::ReplayCommandV1Driver` — asserts metadata, calls an existing validator; the game owns the trace bytes.
4. Determinism / no-leak: the driver delegates to the existing internal command-trace validator with byte equality to the `-001` baseline; no private crest reaches a viewer surface via the trace (§11).

## Architecture Check

1. A thin profile driver over the existing validator adds typed evidence without a new canonical-byte authority — consistent across games.
2. No backwards-compat alias; no artifact rewrite.
3. `engine-core` untouched; the driver is dev-only `game-test-support`.

## Verification Layers

1. Driver validates metadata and rejects wrong id/owner/visibility/fields -> profile driver test (`cargo test -p poker_lite`).
2. Existing internal trace bytes unchanged -> deterministic replay-hash check (`replay-check --game poker_lite --all`).
3. `ReplayCommandV1Driver` adoption -> codebase grep-proof in `tests/replay.rs`.

## What to Change

### 1. Add the replay-command-v1 driver test

In `tests/replay.rs`, invoke `ReplayCommandV1Driver` over the existing internal command trace (thin `replay_support.rs` accessor only if required), asserting valid metadata and wrong-profile/owner/field rejection.

## Files to Touch

- `games/poker_lite/tests/replay.rs` (modify)
- `games/poker_lite/src/replay_support.rs` (modify; serialized after `-019`)

## Out of Scope

- Inventing an omniscient export; rewriting any trace/fixture; the public-export profile (`-038`).
- Any internal-trace byte change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite` green, including the profile driver test.
2. `cargo run -p replay-check -- --game poker_lite --all` — internal trace bytes byte-identical to baseline.

### Invariants

1. The driver claims no new canonical bytes; the existing validator remains the authority.
2. The private command trace stays internal-dev.

## Test Plan

### New/Modified Tests

1. `games/poker_lite/tests/replay.rs` — `replay-command-v1` driver metadata + rejection test.

### Commands

1. `cargo test -p poker_lite`
2. `cargo run -p replay-check -- --game poker_lite --all`

## Outcome

Implemented in `games/poker_lite/tests/replay.rs` with a
`replay_command_v1_profile_driver_wraps_internal_trace_validator` test. The
test validates `replay-command-v1` metadata for `poker_lite`, delegates through
`ReplayCommandV1Driver::validate_with` to the existing internal trace replay
validator, confirms the returned hash is the trace stable hash, and asserts no
canonical byte claim is made.

The driver rejects wrong profile id, wrong validator owner, and an illegal
profile field. No trace bytes, fixtures, public exports, or `replay_support.rs`
behavior were changed because Poker Lite already exposed the required internal
trace generator and validator.

Verification passed:

1. `cargo test -p poker_lite replay_command_v1_profile_driver_wraps_internal_trace_validator -- --nocapture`
2. `cargo test -p poker_lite`
3. `cargo run -p replay-check -- --game poker_lite --all`
