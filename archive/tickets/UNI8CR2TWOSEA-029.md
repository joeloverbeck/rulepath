# UNI8CR2TWOSEA-029: Secret Draft — replay-command-v1 profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/secret_draft/tests/replay.rs`, `games/secret_draft/src/replay_support.rs`; adopts `game-test-support` `ReplayCommandV1Driver` (internal-dev)
**Deps**: 018, 021

## Problem

Spec §3.9 / task `8C-R2-602`: add a `replay-command-v1` profile driver for Secret Draft. The driver validates metadata and delegates to the existing internal command-trace validator; the private command authority remains internal only (`canonical_byte_authority: none`). It is `internal-dev`, rewrites no artifact. Shares `replay_support.rs` with `-018` and needs the `-021` dev-dependency (hence `Deps: 018, 021`).

## Assumption Reassessment (2026-06-23)

1. `game-test-support` exposes `ReplayCommandV1Driver` (`profiles.rs:96`) and `REPLAY_COMMAND_V1`; Secret gains the dev-dependency in `-021`.
2. Spec §3.9/§9: `migrate`; `internal-dev`; private command authority remains internal only; no omniscient export; no artifact rewrite.
3. Cross-crate boundary under audit: `game-test-support::profiles::ReplayCommandV1Driver` — asserts metadata, calls an existing validator; the game owns the trace bytes.
4. Determinism / no-leak: the driver delegates to the existing internal command-trace validator with byte equality to the `-001` baseline; the committed item never reaches a viewer surface via the trace (§11).

## Architecture Check

1. A thin profile driver over the existing validator adds typed evidence without a new canonical-byte authority — consistent across games.
2. No backwards-compat alias; no artifact rewrite.
3. `engine-core` untouched; the driver is dev-only `game-test-support`.

## Verification Layers

1. Driver validates metadata and rejects wrong id/owner/visibility/fields -> profile driver test (`cargo test -p secret_draft`).
2. Existing internal trace bytes unchanged -> deterministic replay-hash check (`replay-check --game secret_draft --all`).
3. `ReplayCommandV1Driver` adoption -> codebase grep-proof in `tests/replay.rs`.

## What to Change

### 1. Add the replay-command-v1 driver test

In `tests/replay.rs`, invoke `ReplayCommandV1Driver` over the existing internal command trace (thin `replay_support.rs` accessor only if required), asserting valid metadata and wrong-profile/owner/field rejection.

## Files to Touch

- `games/secret_draft/tests/replay.rs` (modify)
- `games/secret_draft/src/replay_support.rs` (modify; serialized after `-018`)

## Out of Scope

- Inventing an omniscient export; rewriting any trace/fixture; the public-export profile (`-037`).
- Any internal-trace byte change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p secret_draft` green, including the profile driver test.
2. `cargo run -p replay-check -- --game secret_draft --all` — internal trace bytes byte-identical to baseline.

### Invariants

1. The driver claims no new canonical bytes; the existing validator remains the authority.
2. The private command trace stays internal-dev.

## Test Plan

### New/Modified Tests

1. `games/secret_draft/tests/replay.rs` — `replay-command-v1` driver metadata + rejection test.

### Commands

1. `cargo test -p secret_draft`
2. `cargo run -p replay-check -- --game secret_draft --all`

## Outcome

Completed on 2026-06-23.

Added `games/secret_draft/tests/replay.rs::replay_command_v1_profile_driver_wraps_internal_trace_validator`.
The test validates `ReplayCommandV1Driver` metadata for
`profile_id = replay-command-v1`, `profile_version = v1`, `visibility_class =
internal-dev`, `validator_owner = secret_draft`, and
`canonical_byte_authority = none`, then delegates through `validate_with` to
the existing generated internal full-trace replay validator and compares the
resulting trace hash to the trace's existing stable hash. It also verifies
rejection for wrong profile id, wrong validator owner, and illegal profile
fields. No artifact was rewritten, no omniscient export was added, and no
`replay_support.rs` accessor was needed.

Verification passed:

1. `cargo fmt --all --check`
2. `cargo test -p secret_draft`
3. `cargo run -p replay-check -- --game secret_draft --all`
