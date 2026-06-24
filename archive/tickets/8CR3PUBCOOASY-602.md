# 8CR3PUBCOOASY-602: C-08 Flood Watch replay-command profile driver

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/flood_watch/tests/replay.rs`, `games/flood_watch/src/replay_support.rs`
**Deps**: 8CR3PUBCOOASY-502

## Problem

C-08 adds a `replay-command-v1` evidence-profile driver for `flood_watch`: a
dev-only `ReplayCommandV1Driver` validating profile metadata then delegating to
the existing native replay validator (commands, event-deck/setup checkpoints,
hashes). Default visibility is `internal-dev`. The driver interprets no role,
forecast, or event rule. No golden trace is rewritten.

## Assumption Reassessment (2026-06-24)

1. Shipped `ReplayCommandV1Driver` at `crates/game-test-support/src/profiles.rs:96`.
   Flood `tests/replay.rs` and `src/replay_support.rs` exist; the dev-dep edge is
   added by 502. `replay-check` already dispatches `flood_watch` (confirmed).
2. Spec §3.9 verdict for Flood `replay-command-v1` is `migrate`, default
   `internal-dev`; §5.9 task `8C-R3-602` scopes the driver over current command
   authority and event-deck/setup checkpoints and hashes.
3. Cross-artifact boundary under audit: the driver validates metadata and
   delegates; it must never execute game transitions to decide semantics.
4. FOUNDATIONS §11 (determinism, no-leak): wraps existing artifacts read-only;
   rejects wrong metadata; leaks no hidden event-deck order.
5. Enforcement surface: the driver test in `tests/replay.rs` over existing
   native artifacts, byte-identical to the 001 baseline.

## Architecture Check

1. A metadata-validating driver delegating to the native validator shares the
   profile plumbing without duplicating replay semantics.
2. No backwards-compatibility alias — new dev-only test driver.
3. `engine-core`/`game-stdlib` untouched; `game-test-support` provides metadata
   validation only.

## Verification Layers

1. Metadata gating -> `tests/replay.rs`: valid `internal-dev` metadata passes;
   wrong profile/version/owner/visibility/field rejects.
2. Delegation correctness -> driver invokes the native validator; command/
   checkpoint/hash results equal the baseline.
3. No semantic interpretation -> no role/forecast/event rule in the driver; no
   trace rewrite.

## What to Change

### 1. Add the replay-command-v1 driver test

In `tests/replay.rs`, construct `ReplayCommandV1Driver`, validate the
`internal-dev` metadata, delegate command/checkpoint/hash checks to the existing
native validator, and add wrong-metadata rejection cases. Touch
`src/replay_support.rs` only if a thin adapter seam is required.

## Files to Touch

- `games/flood_watch/tests/replay.rs` (modify)
- `games/flood_watch/src/replay_support.rs` (modify; adapter seam only, if needed)

## Out of Scope

- Rewriting any golden trace or changing command/checkpoint/hash bytes.
- The setup/domain/public-export profiles (612/622/632).
- Any role/forecast/event-rule interpretation in the driver.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p flood_watch` (driver metadata + rejection tests).
2. `cargo run -p replay-check -- --game flood_watch --all` — byte-identical to baseline.

### Invariants

1. The driver validates metadata then delegates; it interprets no game rule.
2. Wrong profile/version/owner/visibility/field is rejected fail-closed.

## Test Plan

### New/Modified Tests

1. `games/flood_watch/tests/replay.rs` — `ReplayCommandV1Driver` metadata-valid
   + wrong-metadata-rejection cases delegating to the native validator.

### Commands

1. `cargo test -p flood_watch`
2. `cargo run -p replay-check -- --game flood_watch --all`
3. A per-game test + replay-check is the correct boundary: the driver is
   test-side and read-only over existing artifacts.

## Outcome

Completed: 2026-06-24

- Added a test-local `ReplayCommandV1Driver` wrapper in
  `games/flood_watch/tests/replay.rs`.
- The wrapper validates `replay-command-v1` / `v1` / `internal-dev` metadata
  for owner `flood_watch`, canonical byte authority `none`, and the `commands`,
  `checkpoints`, and `expected_hashes` fields, then delegates to native setup,
  command, event-deck trace, action-tree, effect, and public-export hash
  evidence.
- Wrong profile, version, visibility, owner, and field metadata reject
  fail-closed. No production replay support, golden trace bytes, event/forecast
  semantics, or replay hashes changed.
- Verification:
  - `cargo test -p flood_watch`
  - `cargo run -p replay-check -- --game flood_watch --all`
