# 8CR3PUBCOOASY-503: C-06 Frontier Control dev-only support edge

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only dependency) — `games/frontier_control/Cargo.toml`
**Deps**: 8CR3PUBCOOASY-305

## Problem

`frontier_control` does not yet depend on the dev-only `game-test-support` crate
required by C-07 no-leak geometry and C-08 profile drivers. C-06 adds a fresh
`[dev-dependencies]` edge only — never a normal/build edge. Serialized after 305
because both edit `Cargo.toml` (305 adds the normal `game-stdlib` edge).

## Assumption Reassessment (2026-06-24)

1. `games/frontier_control/Cargo.toml` has no `game-test-support` entry
   (confirmed absent). The crate exists at `crates/game-test-support` and is a
   workspace member (`Cargo.toml:6`). 305 adds the `[dependencies]` `game-stdlib`
   edge to the same file.
2. Spec §3.2/§5.7 verdict for Frontier C-06 is `migrate`; task `8C-R3-503`
   scopes a `[dev-dependencies]` edge with no production/build reverse
   dependency. Per §11.2 the C-06 edge lands before C-07/C-08 tests import the
   crate.
3. Cross-crate boundary under audit: the dev-only dependency edge; the inverse
   normal-edge proof must show no normal reverse dependency.
4. FOUNDATIONS §4/§11: dev-only evidence harness; a normal/build edge is a §12
   stop condition (Forbidden change #4).
5. Enforcement surface: inverse `cargo tree` output (before/after) +
   `bash scripts/boundary-check.sh`; no production byte changes.

## Architecture Check

1. A `[dev-dependencies]`-only edge gives tests the shipped helpers without
   coupling production code.
2. No backwards-compatibility alias — a clean new dev-dep edge.
3. `engine-core` untouched; `game-test-support` stays out of the normal graph.

## Verification Layers

1. Dev-only edge correctness -> `cargo tree --workspace -e normal --invert game-test-support`
   shows no `frontier_control` normal reverse dependency.
2. Build integrity -> `cargo build -p frontier_control`.
3. Boundary -> `bash scripts/boundary-check.sh`.

## What to Change

### 1. Add the dev-only dependency

In `games/frontier_control/Cargo.toml`, add
`game-test-support = { path = "../../crates/game-test-support" }` under
`[dev-dependencies]` only.

## Files to Touch

- `games/frontier_control/Cargo.toml` (modify; serialized after 305)

## Out of Scope

- Any normal/build dependency edge to `game-test-support`.
- The C-07 (513) and C-08 (603/613/623/633) tests that consume the crate.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p frontier_control`.
2. `cargo tree --workspace -e normal --invert game-test-support` — no
   `frontier_control` normal reverse edge.
3. `bash scripts/boundary-check.sh`.

### Invariants

1. `game-test-support` appears only under `[dev-dependencies]` for
   `frontier_control`.
2. No production/build/WASM target gains a normal edge to it.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the inverse `cargo tree` proof and boundary-check are the regression guard; no production byte changes.`

### Commands

1. `cargo tree --workspace -e normal --invert game-test-support`
2. `cargo build -p frontier_control && bash scripts/boundary-check.sh`
3. The inverse `cargo tree` is the correct boundary: it proves the dev-only edge
   invariant directly.
