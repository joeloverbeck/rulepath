# 8CR3PUBCOOASY-502: C-06 Flood Watch dev-only support edge

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only dependency) — `games/flood_watch/Cargo.toml`
**Deps**: 8CR3PUBCOOASY-302

## Problem

`flood_watch` does not yet depend on the dev-only `game-test-support` crate
required by C-07 no-leak geometry and C-08 profile drivers. C-06 adds a fresh
`[dev-dependencies]` edge only — never a normal/build edge. Serialized after 302
because both edit `Cargo.toml` (302 adds the normal `game-stdlib` edge).

## Assumption Reassessment (2026-06-24)

1. `games/flood_watch/Cargo.toml` has no `game-test-support` entry (confirmed
   absent). The crate exists at `crates/game-test-support` and is a workspace
   member (`Cargo.toml:6`). 302 adds the `[dependencies]` `game-stdlib` edge to
   the same file.
2. Spec §3.2/§5.7 verdict for Flood C-06 is `migrate`; task `8C-R3-502` scopes a
   `[dev-dependencies]` edge with no production/build reverse dependency. Per
   §11.2 the C-06 edge lands before C-07/C-08 tests import the crate.
3. Cross-crate boundary under audit: the dev-only dependency edge; the inverse
   normal-edge proof must show no normal reverse dependency.
4. FOUNDATIONS §4/§11: dev-only evidence harness; a normal/build edge is a §12
   stop condition (Forbidden change #4).
5. Enforcement surface: inverse `cargo tree` output (before/after) +
   `bash scripts/boundary-check.sh`; no production byte changes.

## Architecture Check

1. A `[dev-dependencies]`-only edge gives tests the shipped helpers without
   coupling production code; cleaner than re-implementing per game.
2. No backwards-compatibility alias — a clean new dev-dep edge.
3. `engine-core` untouched; `game-test-support` stays out of the normal graph.

## Verification Layers

1. Dev-only edge correctness -> `cargo tree --workspace -e normal --invert game-test-support`
   shows no `flood_watch` normal reverse dependency.
2. Build integrity -> `cargo build -p flood_watch`.
3. Boundary -> `bash scripts/boundary-check.sh`.

## What to Change

### 1. Add the dev-only dependency

In `games/flood_watch/Cargo.toml`, add
`game-test-support = { path = "../../crates/game-test-support" }` under
`[dev-dependencies]` only.

## Files to Touch

- `games/flood_watch/Cargo.toml` (modify; serialized after 302)

## Out of Scope

- Any normal/build dependency edge to `game-test-support`.
- The C-07 (512) and C-08 (602/612/622/632) tests that consume the crate.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p flood_watch`.
2. `cargo tree --workspace -e normal --invert game-test-support` — no
   `flood_watch` normal reverse edge.
3. `bash scripts/boundary-check.sh`.

### Invariants

1. `game-test-support` appears only under `[dev-dependencies]` for `flood_watch`.
2. No production/build/WASM target gains a normal edge to it.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the inverse `cargo tree` proof and boundary-check are the regression guard; no production byte changes.`

### Commands

1. `cargo tree --workspace -e normal --invert game-test-support`
2. `cargo build -p flood_watch && bash scripts/boundary-check.sh`
3. The inverse `cargo tree` is the correct boundary: it proves the dev-only edge
   invariant directly.
