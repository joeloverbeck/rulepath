# 8CR3PUBCOOASY-504: C-06 Event Frontier dev-only support edge

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only dependency) — `games/event_frontier/Cargo.toml`
**Deps**: 8CR3PUBCOOASY-001

## Problem

`event_frontier` does not yet depend on the dev-only `game-test-support` crate
required by C-07 no-leak geometry and C-08 profile drivers. C-06 adds a fresh
`[dev-dependencies]` edge only — never a normal/build edge. Event Frontier
already has its normal `game-stdlib` edge, so this ticket touches only the
dev-dependency section.

## Assumption Reassessment (2026-06-24)

1. `games/event_frontier/Cargo.toml` has no `game-test-support` entry (confirmed
   absent) but already has `game-stdlib` (`:11`). The crate exists at
   `crates/game-test-support` and is a workspace member (`Cargo.toml:6`).
2. Spec §3.2/§5.7 verdict for Event C-06 is `migrate`; task `8C-R3-504` scopes a
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
   coupling production code.
2. No backwards-compatibility alias — a clean new dev-dep edge.
3. `engine-core` untouched; `game-test-support` stays out of the normal graph.

## Verification Layers

1. Dev-only edge correctness -> `cargo tree --workspace -e normal --invert game-test-support`
   shows no `event_frontier` normal reverse dependency.
2. Build integrity -> `cargo build -p event_frontier`.
3. Boundary -> `bash scripts/boundary-check.sh`.

## What to Change

### 1. Add the dev-only dependency

In `games/event_frontier/Cargo.toml`, add
`game-test-support = { path = "../../crates/game-test-support" }` under
`[dev-dependencies]` only.

## Files to Touch

- `games/event_frontier/Cargo.toml` (modify)

## Out of Scope

- Any normal/build dependency edge to `game-test-support`.
- The C-07 (514) and C-08 (604/614/624/634) tests that consume the crate.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p event_frontier`.
2. `cargo tree --workspace -e normal --invert game-test-support` — no
   `event_frontier` normal reverse edge.
3. `bash scripts/boundary-check.sh`.

### Invariants

1. `game-test-support` appears only under `[dev-dependencies]` for
   `event_frontier`.
2. No production/build/WASM target gains a normal edge to it.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the inverse `cargo tree` proof and boundary-check are the regression guard; no production byte changes.`

### Commands

1. `cargo tree --workspace -e normal --invert game-test-support`
2. `cargo build -p event_frontier && bash scripts/boundary-check.sh`
3. The inverse `cargo tree` is the correct boundary: it proves the dev-only edge
   invariant directly.

## Outcome

Completed: 2026-06-24

- Added `game-test-support = { path = "../../crates/game-test-support" }` only
  under `[dev-dependencies]` in `games/event_frontier/Cargo.toml`.
- Included the corresponding `Cargo.lock` package dependency edge for
  `event_frontier`; no normal/build dependency edge was added.
- Verified `cargo build -p event_frontier`,
  `cargo tree --workspace -e normal --invert game-test-support`, and
  `bash scripts/boundary-check.sh`. The inverse normal graph printed only the
  `game-test-support` root package.
