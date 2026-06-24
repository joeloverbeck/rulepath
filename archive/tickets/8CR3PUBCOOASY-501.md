# 8CR3PUBCOOASY-501: C-06 Plain Tricks dev-only support edge

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only dependency) — `games/plain_tricks/Cargo.toml`
**Deps**: 8CR3PUBCOOASY-001

## Problem

`plain_tricks` does not yet depend on the dev-only `game-test-support` crate,
which the C-07 no-leak geometry and C-08 profile drivers require. C-06 adds a
fresh `[dev-dependencies]` edge only — never a normal or build edge — so the
test geometry/profile metadata helpers are available to tests without entering
the production dependency graph.

## Assumption Reassessment (2026-06-24)

1. `games/plain_tricks/Cargo.toml` has no `game-test-support` entry (confirmed
   absent in all four R3 games pre-R3). The crate exists at
   `crates/game-test-support` and is a workspace member (`Cargo.toml:6`).
2. Spec §3.2/§5.7 verdict for Plain C-06 is `migrate`; task `8C-R3-501` scopes a
   `[dev-dependencies]` edge with no production/build reverse dependency. Per
   §11.2 the C-06 edges land before C-07/C-08 tests import the crate.
3. Cross-crate boundary under audit: the dev-only dependency edge — it must be
   `[dev-dependencies]`-only; the inverse normal-edge proof
   (`cargo tree --workspace -e normal --invert game-test-support`) must show no
   game/tool/WASM/production target depends on it normally.
4. FOUNDATIONS §4/§11 motivate this: `game-test-support` is a dev-only evidence
   harness; a normal/build edge from a game is a §12 stop condition (Forbidden
   change #4).
5. Enforcement surface: the inverse `cargo tree` output (before/after) and
   `bash scripts/boundary-check.sh`; no production byte changes.

## Architecture Check

1. A `[dev-dependencies]`-only edge gives tests the shipped no-leak/profile
   helpers without coupling production code to the dev harness; cleaner than
   re-implementing the geometry per game.
2. No backwards-compatibility alias — a clean new dev-dep edge.
3. `engine-core` untouched; `game-test-support` stays dev-only and out of the
   normal dependency graph.

## Verification Layers

1. Dev-only edge correctness -> `cargo tree --workspace -e normal --invert game-test-support`
   shows no normal reverse dependency from `plain_tricks`.
2. Build integrity -> `cargo build -p plain_tricks` (production target builds
   without the dev-dep).
3. Boundary -> `bash scripts/boundary-check.sh`.

## What to Change

### 1. Add the dev-only dependency

In `games/plain_tricks/Cargo.toml`, add
`game-test-support = { path = "../../crates/game-test-support" }` under
`[dev-dependencies]` only.

## Files to Touch

- `games/plain_tricks/Cargo.toml` (modify)

## Out of Scope

- Any normal/build dependency edge to `game-test-support`.
- The C-07 (511) and C-08 (601/611/621/631/641) tests that consume the crate.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p plain_tricks` (production builds; dev-dep not in normal graph).
2. `cargo tree --workspace -e normal --invert game-test-support` — no
   `plain_tricks` normal reverse edge.
3. `bash scripts/boundary-check.sh`.

### Invariants

1. `game-test-support` appears only under `[dev-dependencies]` for `plain_tricks`.
2. No production/build/WASM target gains a normal edge to it.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the inverse `cargo tree` proof and boundary-check are the regression guard; no production byte changes.`

### Commands

1. `cargo tree --workspace -e normal --invert game-test-support`
2. `cargo build -p plain_tricks && bash scripts/boundary-check.sh`
3. The inverse `cargo tree` is the correct boundary: it directly proves the
   dev-only edge invariant this ticket exists to establish.

## Outcome

Completed: 2026-06-24

- Added `game-test-support = { path = "../../crates/game-test-support" }` only
  under `[dev-dependencies]` in `games/plain_tricks/Cargo.toml`.
- Included the corresponding `Cargo.lock` package dependency edge for
  `plain_tricks`; no normal/build dependency edge was added.
- Verified `cargo build -p plain_tricks`,
  `cargo tree --workspace -e normal --invert game-test-support`, and
  `bash scripts/boundary-check.sh`. The inverse normal graph printed only the
  `game-test-support` root package.
