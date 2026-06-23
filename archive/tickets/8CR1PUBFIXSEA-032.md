# 8CR1PUBFIXSEA-032: Token Bazaar C-08 replay-command profile driver

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/token_bazaar` (`Cargo.toml` dev-dep + `tests/replay.rs`); trace/export bytes unchanged
**Deps**: 8CR1PUBFIXSEA-001

## Problem

Token Bazaar has no `replay-command-v1` profile adapter around its existing replay evidence. Following the shipped Race pilot, add a dev-only `game-test-support` dependency and a parallel replay-profile adapter — kept separate from the public-export profile (`-036`) — that validates `replay-command-v1` metadata with `ReplayCommandV1Driver::new("replay-check")`, then delegates to the existing replay/hash assertions. The committed trace/export bytes, declared hashes, and public visibility are unchanged; no `profile_id` field is inserted.

## Assumption Reassessment (2026-06-23)

1. `crates/game-test-support/src/profiles.rs` exposes `ProfileArtifact`, `ProfileMetadata`, `ReplayCommandV1Driver`, and the `REPLAY_COMMAND_V1`/`PROFILE_VERSION_V1` constants (Race pilot pattern). `games/token_bazaar/tests/replay.rs` exists. Confirmed during reassessment.
2. Spec §3.7 and §5.8 (task `8C-R1-505`) classify Token Bazaar `replay-command-v1` as `migrate` and keep it separate from the `public-export-v1` profile; the driver validates metadata only and delegates behavior to the existing game test. MSC-8C-008 owns evidence-profile drivers.
3. Cross-artifact: `game-test-support` is a dev-only crate; the adapter wraps existing committed trace evidence and stays distinct from the `PublicReplayExport` profile surface. Before-baseline from `-001`.
4. §6 (evidence-heavy) and §3/§11 motivate this ticket: the driver classifies evidence without replaying or deciding rules; behavior stays in the game test.
5. Enforcement surface = the dev-only `game-test-support` boundary (C-06) and the committed trace/export bytes; the adapter is `[dev-dependencies]`-only and changes no byte — `cargo tree` proves no production/build path.
6. This extends `games/token_bazaar/Cargo.toml` `[dev-dependencies]` additively (the `[dependencies]` `game-stdlib` edge from `-023` is a separate section); `cargo tree -e normal` is the consumer-side proof.

## Architecture Check

1. A parallel typed replay-profile adapter, kept distinct from the public-export profile, classifies replay evidence without touching the committed trace/export bytes.
2. No backwards-compatibility shim is introduced; the adapter is additive dev-only test code.
3. `engine-core` stays noun-free (§3); `game-test-support` stays dev-only (§4/C-06).

## Verification Layers

1. `replay-command-v1` metadata validated -> `ReplayCommandV1Driver` assertion in `tests/replay.rs`.
2. Existing replay/hash assertions still pass after delegation -> `cargo test -p token_bazaar` + `replay-check --game token_bazaar --all`.
3. Dev-only dependency, no production path; trace/export bytes unchanged -> `cargo tree -p token_bazaar -e normal` + `git diff` shows no trace change.

## What to Change

### 1. Add the dev-only dependency

Add `game-test-support = { path = "../../crates/game-test-support" }` to `games/token_bazaar/Cargo.toml` `[dev-dependencies]`.

### 2. Add the replay-profile adapter test

In `games/token_bazaar/tests/replay.rs`, build a `ProfileArtifact` from the existing committed command trace, validate with `ReplayCommandV1Driver::new("replay-check")`, then run the existing replay/hash assertions. Keep this separate from the public-export profile. Do not insert `profile_id` into the trace.

## Files to Touch

- `games/token_bazaar/Cargo.toml` (modify)
- `games/token_bazaar/tests/replay.rs` (modify)

## Out of Scope

- The public-export profile (owned by `-036`).
- Inserting `profile_id` or any profile field into the committed trace/export.
- Moving replay behavior into `game-test-support`, `replay-check`, or `fixture-check`.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p token_bazaar` is green, including the new replay-profile-validation assertion.
2. `cargo tree -p token_bazaar -e normal` shows no `game-test-support` production edge.
3. `cargo run -p replay-check -- --game token_bazaar --all` passes; trace/export bytes and declared hashes unchanged.

### Invariants

1. `game-test-support` is a dev-dependency edge only.
2. The committed trace/export bytes are unchanged; the replay profile stays distinct from the public-export profile.

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/tests/replay.rs` — `replay-command-v1` profile-metadata validation wrapping the existing replay/hash assertions, separate from the public-export profile.

### Commands

1. `cargo test -p token_bazaar`
2. `cargo tree -p token_bazaar -e normal`
3. The per-game test plus the `cargo tree` dev-only proof are the correct boundary.

## Outcome

Completed on 2026-06-23.

Added `game-test-support` as a Token Bazaar dev-dependency and recorded the
corresponding `Cargo.lock` package dependency entry. No normal dependency path
was introduced: `cargo tree -p token_bazaar -e normal` lists only `ai-core`,
`engine-core`, and `game-stdlib`.

Added `replay_command_v1_driver_replays_shortest_normal_fixture` in
`games/token_bazaar/tests/replay.rs`. The test builds a typed
`ProfileArtifact` with `replay-command-v1` / `v1`, `internal-dev` visibility,
`replay-check` validator ownership, and `token_bazaar::replay_support`
canonical byte authority, then validates with
`ReplayCommandV1Driver::new("replay-check")` before delegating to the existing
fixture replay/hash assertions. This replay profile remains separate from the
public-export profile surface. The committed trace/export JSON remains
profile-free and byte-unchanged.

Verification:

1. `cargo test -p token_bazaar replay_command_v1_driver_replays_shortest_normal_fixture -- --exact`
2. `cargo test -p token_bazaar`
3. `cargo tree -p token_bazaar -e normal`
4. `cargo run -p replay-check -- --game token_bazaar --all`
5. `cargo fmt --all -- --check`
6. `git diff --name-only -- games/token_bazaar/tests/golden_traces`
