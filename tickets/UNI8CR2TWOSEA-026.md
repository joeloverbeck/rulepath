# UNI8CR2TWOSEA-026: Poker Lite — C-07 pairwise no-leak geometry

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/poker_lite/tests/{visibility,bots,replay}.rs`; adopts `game_test_support::no_leak` pairwise matrix
**Deps**: 022

## Problem

Spec §3.8 / task `8C-R2-512`: add the pairwise no-leak matrix for Poker Lite — private hand, center reveal, showdown, yield non-reveal, public/seat-private export, and bot surfaces, for both source seats. Hand privacy, showdown, and yield policy remain game-local. Existing focused tests are retained.

## Assumption Reassessment (2026-06-23)

1. `games/poker_lite/tests/{visibility,bots,replay}.rs` exist; the private crest is absent before showdown across viewer surfaces and the unshown losing crest stays hidden after a yield (spec §3.8 matrix).
2. Spec §3.8/§9: the harness may not decide showdown visibility or yield non-reveal; retain focused reveal tests; do not subsume them.
3. Cross-crate boundary under audit: `game-test-support::no_leak` (`ExposureExpectation`) — enumerates caller-supplied cases only; needs the `-022` dev-dependency.
4. §11 no-leak firewall: the private crest stays absent for observer/opponent before showdown and present for owner; both crests appear only after the authorized showdown; the unshown losing crest after a yield follows the existing owner-view policy; canaries are in-memory only (R2-EC-20).

## Architecture Check

1. A caller-driven pairwise matrix proves hand/showdown/yield redaction without moving policy into shared code — the geometry-only harness contract.
2. No backwards-compat alias; the generic matrix is added alongside the focused tests.
3. `engine-core` untouched; the harness is dev-only `game-test-support`.

## Verification Layers

1. Both seats × 3 viewers × all surfaces (deal/showdown/yield) -> no-leak visibility test (`tests/visibility.rs`, `tests/replay.rs`).
2. Bot input/explanation leak-free -> bot legality check (`tests/bots.rs`).
3. Showdown/yield phase behavior unchanged -> `replay-check --game poker_lite --all`.

## What to Change

### 1. Add the pairwise no-leak matrix

Using `game_test_support::no_leak`, enumerate source seat × viewer × surface cases covering private hand, center reveal, showdown, and yield, with in-memory canaries; retain the existing focused tests.

## Files to Touch

- `games/poker_lite/tests/visibility.rs` (modify)
- `games/poker_lite/tests/bots.rs` (modify)
- `games/poker_lite/tests/replay.rs` (modify)

## Out of Scope

- Moving hand-privacy, showdown, or yield policy into shared code.
- Any committed canary; any golden-trace or fixture change; the C-08 profile drivers.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite` green, including the new matrix and retained focused tests.
2. `cargo run -p replay-check -- --game poker_lite --all` — no byte change.

### Invariants

1. The private crest is absent for observer/opponent before showdown; the unshown losing crest stays hidden after a yield.
2. No canary appears in any committed trace, fixture, snapshot, log, or test ID.

## Test Plan

### New/Modified Tests

1. `games/poker_lite/tests/visibility.rs` — pairwise source × viewer × surface no-leak matrix (deal/showdown/yield).

### Commands

1. `cargo test -p poker_lite`
2. `cargo run -p replay-check -- --game poker_lite --all`
