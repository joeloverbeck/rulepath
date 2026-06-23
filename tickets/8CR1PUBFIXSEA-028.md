# 8CR1PUBFIXSEA-028: Draughts Lite C-08 replay-command profile driver

**Status**: PENDING
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/draughts_lite` (`Cargo.toml` dev-dep + `tests/replay.rs`); trace bytes unchanged
**Deps**: 8CR1PUBFIXSEA-001

## Problem

Draughts Lite has no `replay-command-v1` profile adapter around its existing replay evidence. Following the shipped Race pilot, add a dev-only `game-test-support` dependency and a parallel test adapter that builds a typed `ProfileArtifact`, validates `replay-command-v1` metadata with `ReplayCommandV1Driver::new("replay-check")`, then delegates to the existing replay/hash assertions. The committed trace bytes are unchanged; no `profile_id` field is inserted.

## Assumption Reassessment (2026-06-23)

1. `crates/game-test-support/src/profiles.rs` exposes `ProfileArtifact`, `ProfileMetadata`, `ReplayCommandV1Driver`, and the `REPLAY_COMMAND_V1`/`PROFILE_VERSION_V1` constants; the Race pilot uses them in `games/race_to_n/tests/replay_tests.rs` (`ReplayCommandV1Driver::new("replay-check")`). `games/draughts_lite/tests/replay.rs` exists. Confirmed during reassessment.
2. Spec §3.7 and §5.8 (task `8C-R1-501`) classify Draughts `replay-command-v1` as `migrate`; the driver validates metadata only and delegates behavior to the existing game test. MSC-8C-008 owns evidence-profile drivers.
3. Cross-artifact: `game-test-support` is a dev-only crate; the adapter wraps existing committed trace evidence. Before-baseline (trace bytes) from `-001`.
4. §6 (evidence-heavy) and §3/§11 motivate this ticket: the driver classifies evidence without replaying or deciding rules; behavior stays in the game test, not the dev harness or a tool.
5. Enforcement surface = the dev-only `game-test-support` boundary (C-06) and the committed trace bytes; the adapter is `[dev-dependencies]`-only and changes no trace byte — `cargo tree` proves no production/build path, and no hidden information is exposed.
6. This extends `games/draughts_lite/Cargo.toml` `[dev-dependencies]` additively (a new dev edge on an existing workspace crate); `cargo tree -e normal` is the consumer-side proof that no production path is introduced.

## Architecture Check

1. A parallel typed profile adapter classifies replay evidence without touching the committed trace — cleaner than rewriting traces with inline profile fields.
2. No backwards-compatibility shim is introduced; the adapter is additive dev-only test code.
3. `engine-core` stays noun-free (§3); `game-test-support` stays dev-only (§4/C-06); no production dependency added.

## Verification Layers

1. `replay-command-v1` metadata validated (class/version/visibility/owner/byte-authority) -> `ReplayCommandV1Driver` assertion in `tests/replay.rs`.
2. Existing replay/hash assertions still pass after delegation -> `cargo test -p draughts_lite` + `replay-check --game draughts_lite --all`.
3. Dev-only dependency, no production path; trace bytes unchanged -> `cargo tree -p draughts_lite -e normal` + `git diff` shows no trace change.

## What to Change

### 1. Add the dev-only dependency

Add `game-test-support = { path = "../../crates/game-test-support" }` to `games/draughts_lite/Cargo.toml` `[dev-dependencies]`.

### 2. Add the profile adapter test

In `games/draughts_lite/tests/replay.rs`, build a `ProfileArtifact` from the existing committed command trace, validate with `ReplayCommandV1Driver::new("replay-check")`, then run the existing replay/hash assertions. Do not insert `profile_id` into the trace.

## Files to Touch

- `games/draughts_lite/Cargo.toml` (modify)
- `games/draughts_lite/tests/replay.rs` (modify)

## Out of Scope

- Inserting `profile_id` or any profile field into the committed trace.
- Moving replay behavior into `game-test-support`, `replay-check`, or `fixture-check`.
- Adding any production/build dependency on `game-test-support`.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p draughts_lite` is green, including the new profile-validation assertion.
2. `cargo tree -p draughts_lite -e normal` shows no `game-test-support` production edge.
3. `cargo run -p replay-check -- --game draughts_lite --all` passes; trace bytes unchanged.

### Invariants

1. `game-test-support` is a dev-dependency edge only.
2. The committed trace bytes are unchanged; the driver validates metadata and delegates behavior to the game test.

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/tests/replay.rs` — `replay-command-v1` profile-metadata validation wrapping the existing replay/hash assertions.

### Commands

1. `cargo test -p draughts_lite`
2. `cargo tree -p draughts_lite -e normal`
3. The per-game test plus the `cargo tree` dev-only proof are the correct boundary: the adapter is dev-only and game-local.
