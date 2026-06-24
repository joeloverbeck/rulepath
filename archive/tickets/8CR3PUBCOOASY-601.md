# 8CR3PUBCOOASY-601: C-08 Plain Tricks replay-command profile driver

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (dev-only profile adapter) — `games/plain_tricks/tests/replay.rs`, `games/plain_tricks/src/replay_support.rs`
**Deps**: 8CR3PUBCOOASY-401, 8CR3PUBCOOASY-501

## Problem

C-08 adds a `replay-command-v1` evidence-profile driver for `plain_tricks`: a
dev-only `ReplayCommandV1Driver` that validates profile metadata (profile,
version, owner, visibility, fields) and then delegates all command/checkpoint/
hash validation to the existing native replay validator. The driver interprets
no trick rules. Default visibility is `internal-dev`. No golden trace is
rewritten.

## Assumption Reassessment (2026-06-24)

1. Shipped `ReplayCommandV1Driver` is at
   `crates/game-test-support/src/profiles.rs:96`/`:116`. Plain `tests/replay.rs`
   and `src/replay_support.rs` exist; the dev-dep edge is added by 501; the v1
   action-tree surface lands in 401 (shared `replay_support.rs` — serialized).
2. Spec §3.9 verdict for Plain `replay-command-v1` is `migrate`, default
   `internal-dev`; §5.9 task `8C-R3-601` scopes the driver invoking current
   native commands/checkpoints/hashes with no trace rewrite. The existing
   `replay-check` dispatch already covers `plain_tricks` (confirmed).
3. Cross-artifact boundary under audit: the profile driver validates metadata
   and delegates to the game/tool adapter; it must never execute game
   transitions to decide expected semantics (Forbidden change #5).
4. FOUNDATIONS §11 (determinism, no-leak) motivates this: the driver wraps
   existing artifacts read-only; it must reject wrong profile/version/owner/
   visibility/fields and leak no hidden command/deck order.
5. Enforcement surface: the driver test in `tests/replay.rs` (valid metadata
   passes; wrong-profile/version/owner/visibility/field rejects) over the
   existing native replay artifacts, byte-identical to the 001 baseline.

## Architecture Check

1. A metadata-validating driver that delegates to the native validator shares
   the profile-envelope plumbing without duplicating replay semantics; cleaner
   than a bespoke per-game profile harness.
2. No backwards-compatibility alias — a new dev-only test driver; existing
   validator unchanged.
3. `engine-core`/`game-stdlib` untouched; `game-test-support` provides profile
   metadata validation only.

## Verification Layers

1. Metadata gating -> `tests/replay.rs`: valid `internal-dev` metadata passes;
   wrong profile/version/owner/visibility/field rejects (fail-closed).
2. Delegation correctness -> the driver invokes the existing native replay
   validator; command/checkpoint/hash results equal the baseline.
3. No semantic interpretation -> grep/manual proof the driver decides no trick
   rule; no trace rewrite (`replay-check --game plain_tricks --all` unchanged).

## What to Change

### 1. Add the replay-command-v1 driver test

In `tests/replay.rs`, construct `ReplayCommandV1Driver`, validate the
`internal-dev` profile metadata, and delegate command/checkpoint/hash checks to
the existing native validator. Add the wrong-metadata rejection cases. Touch
`src/replay_support.rs` only if a thin adapter seam is required (no semantics).

## Files to Touch

- `games/plain_tricks/tests/replay.rs` (modify)
- `games/plain_tricks/src/replay_support.rs` (modify; adapter seam only, if needed — serialized after 401)

## Out of Scope

- Rewriting any golden trace or changing command/checkpoint/hash bytes.
- The setup/domain/public-export/seat-private profiles (611/621/631/641).
- Any trick-rule interpretation inside the driver.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks` (driver metadata + rejection tests).
2. `cargo run -p replay-check -- --game plain_tricks --all` — byte-identical to baseline.

### Invariants

1. The driver validates metadata then delegates; it interprets no trick rule.
2. Wrong profile/version/owner/visibility/field is rejected fail-closed.

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/tests/replay.rs` — `ReplayCommandV1Driver` metadata-valid
   + wrong-metadata-rejection cases delegating to the native validator.

### Commands

1. `cargo test -p plain_tricks`
2. `cargo run -p replay-check -- --game plain_tricks --all`
3. A per-game test + replay-check is the correct boundary: the driver is
   test-side and read-only over existing artifacts.

## Outcome

Completed: 2026-06-24

- Added a test-local `ReplayCommandV1Driver` wrapper in
  `games/plain_tricks/tests/replay.rs`.
- The wrapper validates `replay-command-v1` / `v1` / `internal-dev` metadata
  for owner `plain_tricks`, canonical byte authority `none`, and the
  `commands`, `checkpoints`, and `expected_hashes` fields, then delegates to the
  existing native golden-trace replay assertions.
- Wrong profile, version, visibility, owner, and field metadata reject
  fail-closed. No production replay support, golden trace bytes, command
  semantics, or replay hashes changed.
- Verification:
  - `cargo test -p plain_tricks`
  - `cargo run -p replay-check -- --game plain_tricks --all`
