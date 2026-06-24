# UNI8CR2TWOSEA-031: Masked Claims — replay-command-v1 profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/masked_claims/tests/replay.rs`, `games/masked_claims/src/replay_support.rs`; adopts `game-test-support` `ReplayCommandV1Driver` (internal-dev)
**Deps**: 020, 023

## Problem

Spec §3.9 / task `8C-R2-604`: add a `replay-command-v1` profile driver for Masked Claims, profiling the existing command/replay evidence (rule replay builder) without inventing an omniscient export. Current internal trace bytes remain the authority (`canonical_byte_authority: none`); the driver is `internal-dev`. Shares `replay_support.rs` with `-020` and needs the `-023` dev-dependency (hence `Deps: 020, 023`).

## Assumption Reassessment (2026-06-23)

1. `game-test-support` exposes `ReplayCommandV1Driver` (`profiles.rs:96`) and `REPLAY_COMMAND_V1`; Masked gains the dev-dependency in `-023`; the rule replay builder/`replay_support.rs` exist.
2. Spec §3.9/§9: `migrate`; `internal-dev`; use existing rule/replay construction, not a new omniscient export; no artifact rewrite.
3. Cross-crate boundary under audit: `game-test-support::profiles::ReplayCommandV1Driver` — asserts metadata, calls an existing validator; the game owns the trace bytes.
4. Determinism / no-leak: the driver delegates to the existing replay/command evidence with byte equality to the `-001` baseline; claimed tile identity never reaches a viewer surface via the trace (§11).

## Architecture Check

1. A thin profile driver over the existing rule/replay evidence adds typed metadata without a new canonical-byte authority or omniscient export — consistent across games.
2. No backwards-compat alias; no artifact rewrite.
3. `engine-core` untouched; the driver is dev-only `game-test-support`.

## Verification Layers

1. Driver validates metadata and rejects wrong id/owner/visibility/fields -> profile driver test (`cargo test -p masked_claims`).
2. Existing internal trace bytes unchanged -> deterministic replay-hash check (`replay-check --game masked_claims --all`).
3. `ReplayCommandV1Driver` adoption -> codebase grep-proof in `tests/replay.rs`.

## What to Change

### 1. Add the replay-command-v1 driver test

In `tests/replay.rs`, invoke `ReplayCommandV1Driver` over the existing rule/replay command evidence (thin `replay_support.rs` accessor only if required), asserting valid metadata and wrong-profile/owner/field rejection.

## Files to Touch

- `games/masked_claims/tests/replay.rs` (modify)
- `games/masked_claims/src/replay_support.rs` (modify; serialized after `-020`)

## Out of Scope

- Inventing an omniscient export; rewriting any trace/fixture; the public-export profile (`-039`).
- Any internal-trace byte change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p masked_claims` green, including the profile driver test.
2. `cargo run -p replay-check -- --game masked_claims --all` — internal trace bytes byte-identical to baseline.

### Invariants

1. The driver claims no new canonical bytes; the existing validator remains the authority.
2. No omniscient export is introduced; claimed tile identity stays redacted on viewer surfaces.

## Test Plan

### New/Modified Tests

1. `games/masked_claims/tests/replay.rs` — `replay-command-v1` driver metadata + rejection test.

### Commands

1. `cargo test -p masked_claims`
2. `cargo run -p replay-check -- --game masked_claims --all`

## Outcome

Implemented in `games/masked_claims/tests/replay.rs` with
`replay_command_v1_profile_driver_wraps_rule_replay_evidence`. The test
validates `replay-command-v1` metadata for `masked_claims`, delegates through
`ReplayCommandV1Driver::validate_with` to the existing deterministic
rule/replay evidence builder, and confirms the driver makes no canonical byte
claim.

The driver rejects wrong profile id, wrong validator owner, wrong visibility,
and an illegal profile field. No omniscient export, trace fixture rewrite, or
`replay_support.rs` behavior change was introduced.

Verification passed:

1. `cargo test -p masked_claims replay_command_v1_profile_driver_wraps_rule_replay_evidence -- --nocapture`
2. `cargo test -p masked_claims`
3. `cargo run -p replay-check -- --game masked_claims --all`
