# 8CR3PUBCOOASY-603: C-08 Frontier Control replay-command profile driver

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/frontier_control/tests/replay.rs`, `games/frontier_control/src/replay_support.rs`
**Deps**: 8CR3PUBCOOASY-503

## Problem

C-08 adds a `replay-command-v1` evidence-profile driver for `frontier_control`:
a dev-only `ReplayCommandV1Driver` validating profile metadata then delegating
to the existing native replay validator (command authority, fully-public state).
Visibility may be `public` only if characterization proves no seed/hidden input;
otherwise `internal-dev`. The driver interprets no graph/clash/scoring rule. No
golden trace is rewritten.

## Assumption Reassessment (2026-06-24)

1. Shipped `ReplayCommandV1Driver` at `crates/game-test-support/src/profiles.rs:96`.
   Frontier `tests/replay.rs` and `src/replay_support.rs` exist; the dev-dep
   edge is added by 503. `replay-check` already dispatches `frontier_control`
   (confirmed). Frontier setup is RNG-free (no `next_bounded_index_unbiased`).
2. Spec §3.9 verdict for Frontier `replay-command-v1` is `migrate`; visibility
   `public` only if characterization proves no seed/hidden input, else
   `internal-dev`; §5.9 task `8C-R3-603` scopes the driver over current command
   authority and fully-public state.
3. Cross-artifact boundary under audit: the driver validates metadata and
   delegates; it must never execute game transitions to decide semantics.
4. FOUNDATIONS §11 (determinism): wraps existing artifacts read-only; rejects
   wrong metadata. The visibility class is pinned by characterization (001), not
   guessed.
5. Enforcement surface: the driver test in `tests/replay.rs` over existing
   native artifacts, byte-identical to the 001 baseline.

## Architecture Check

1. A metadata-validating driver delegating to the native validator shares the
   profile plumbing without duplicating replay semantics.
2. No backwards-compatibility alias — new dev-only test driver.
3. `engine-core`/`game-stdlib` untouched; `game-test-support` provides metadata
   validation only.

## Verification Layers

1. Metadata gating -> `tests/replay.rs`: valid metadata (visibility per
   characterization) passes; wrong profile/version/owner/visibility/field rejects.
2. Delegation correctness -> driver invokes the native validator; command/
   checkpoint/hash results equal the baseline.
3. No semantic interpretation -> no graph/clash/scoring rule in the driver; no
   trace rewrite.

## What to Change

### 1. Add the replay-command-v1 driver test

In `tests/replay.rs`, construct `ReplayCommandV1Driver`, validate the metadata
(visibility class per the 001 characterization), delegate command/checkpoint/
hash checks to the existing native validator, and add wrong-metadata rejection
cases. Touch `src/replay_support.rs` only if a thin adapter seam is required.

## Files to Touch

- `games/frontier_control/tests/replay.rs` (modify)
- `games/frontier_control/src/replay_support.rs` (modify; adapter seam only, if needed)

## Out of Scope

- Rewriting any golden trace or changing command/checkpoint/hash bytes.
- The setup/domain/public-export profiles (613/623/633).
- Any graph/clash/scoring interpretation in the driver.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p frontier_control` (driver metadata + rejection tests).
2. `cargo run -p replay-check -- --game frontier_control --all` — byte-identical to baseline.

### Invariants

1. The driver validates metadata then delegates; it interprets no game rule.
2. The visibility class matches the 001 characterization; wrong metadata is
   rejected fail-closed.

## Test Plan

### New/Modified Tests

1. `games/frontier_control/tests/replay.rs` — `ReplayCommandV1Driver`
   metadata-valid + wrong-metadata-rejection cases delegating to the native
   validator.

### Commands

1. `cargo test -p frontier_control`
2. `cargo run -p replay-check -- --game frontier_control --all`
3. A per-game test + replay-check is the correct boundary: the driver is
   test-side and read-only over existing artifacts.
