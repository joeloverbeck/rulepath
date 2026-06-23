# UNI8CR2TWOSEA-027: Masked Claims — C-07 pairwise no-leak geometry

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/masked_claims/tests/{visibility,bots,replay}.rs`; adopts `game_test_support::no_leak` pairwise matrix
**Deps**: 023

## Problem

Spec §3.8 / task `8C-R2-513`: add the pairwise no-leak matrix for Masked Claims — hand, pending claim, accepted-secret resolution, challenged reveal, responder action tree, public export, and bot surfaces, for both source seats. The reaction window, pending-responder, claim redaction, accepted-mask secrecy, and challenge reveal remain game-local. Existing focused tests are retained.

## Assumption Reassessment (2026-06-23)

1. `games/masked_claims/tests/{visibility,bots,replay}.rs` exist; claimed tile identity is absent for non-owner viewers during the pending response and an accepted mask the rules keep secret never reaches public surfaces (spec §3.8 matrix).
2. Spec §3.8/§9: the harness may not infer who may respond, when a claim is pending, whether an accepted mask stays hidden, or when a challenge reveals; retain focused tests; do not subsume them.
3. Cross-crate boundary under audit: `game-test-support::no_leak` (`ExposureExpectation`) — enumerates caller-supplied cases only; needs the `-023` dev-dependency.
4. §11 no-leak firewall: claimed/masked tile identity stays redacted for non-authorized viewers until the game-authorized challenge reveal; the accepted-secret mask stays hidden after resolution; canaries are in-memory only (R2-EC-20).

## Architecture Check

1. A caller-driven pairwise matrix proves claim/reaction/challenge redaction without moving reaction policy into shared code — the geometry-only harness contract.
2. No backwards-compat alias; the generic matrix is added alongside the focused tests.
3. `engine-core` untouched; the harness is dev-only `game-test-support`.

## Verification Layers

1. Both seats × 3 viewers × all surfaces (pending claim / accepted-secret / challenge reveal) -> no-leak visibility test (`tests/visibility.rs`, `tests/replay.rs`).
2. Responder action tree and bot input/explanation leak-free -> bot legality check (`tests/bots.rs`).
3. Reaction/challenge phase behavior unchanged -> `replay-check --game masked_claims --all`.

## What to Change

### 1. Add the pairwise no-leak matrix

Using `game_test_support::no_leak`, enumerate source seat × viewer × surface cases covering hand, pending claim, accepted-secret resolution, and challenged reveal, with in-memory canaries; retain the existing focused tests.

## Files to Touch

- `games/masked_claims/tests/visibility.rs` (modify)
- `games/masked_claims/tests/bots.rs` (modify)
- `games/masked_claims/tests/replay.rs` (modify)

## Out of Scope

- Moving reaction-window, pending-responder, redaction, or challenge-reveal policy into shared code.
- Any committed canary; any golden-trace or fixture change; the C-08 profile drivers.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p masked_claims` green, including the new matrix and retained focused tests.
2. `cargo run -p replay-check -- --game masked_claims --all` — no byte change.

### Invariants

1. Claimed/masked tile identity is redacted for non-authorized viewers until the authorized challenge reveal; an accepted-secret mask stays hidden after resolution.
2. No canary appears in any committed trace, fixture, snapshot, log, or test ID.

## Test Plan

### New/Modified Tests

1. `games/masked_claims/tests/visibility.rs` — pairwise source × viewer × surface no-leak matrix (pending claim / accepted-secret / challenge reveal).

### Commands

1. `cargo test -p masked_claims`
2. `cargo run -p replay-check -- --game masked_claims --all`
