# GAT20STACROSTA-004: Crate skeleton, workspace wiring, and id types

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `games/starbridge_crossing` crate (`Cargo.toml`, `src/lib.rs`, `src/ids.rs`); workspace `Cargo.toml` member
**Deps**: GAT20STACROSTA-002, GAT20STACROSTA-003

## Problem

With the topology hard gate (002) and forward-v1 admission (003) resolved, the game crate can be created. This ticket lands the crate skeleton, workspace wiring, and the game-local id/enum types every later module depends on, so subsequent pipeline tickets have a compiling crate to extend.

## Assumption Reassessment (2026-06-27)

1. Game crates depend on `engine-core`, `game-stdlib`, `ai-core`, and dev-dep `game-test-support`: confirmed `games/meldfall_ledger/Cargo.toml`. The new `Cargo.toml` mirrors this.
2. The workspace root `Cargo.toml` lists each game crate in `members`: confirmed; `games/starbridge_crossing` is appended.
3. Cross-crate boundary: id types are game-local (`StarSpaceId(u16)`, `StarPoint`, `StarZone`) per spec Appendix A; they must NOT leak into `engine-core`. `engine-core` is confirmed noun-free (no board/space/peg/coordinate type), so these stay in `games/starbridge_crossing/src/ids.rs`.
4. §3 (`engine-core` is a contract kernel) motivates this ticket: the new mechanic nouns (`StarSpaceId`, `StarPoint`, `StarZone`) belong in `games/*` first; placing them in the kernel would be a §3/§12 boundary failure.

## Architecture Check

1. A thin skeleton + id types first gives later tickets a compiling target and a single home for stable identifiers, avoiding churn across modules.
2. No backwards-compatibility shims; new crate.
3. `engine-core` stays noun-free; `game-stdlib` is reused (seat helpers wired via dep), not grown; this ticket adds no `game-stdlib` symbol.

## Verification Layers

1. Crate compiles + is workspace-wired -> `cargo build -p starbridge_crossing`.
2. Kernel boundary preserved (§3) -> `bash scripts/boundary-check.sh` (no new `engine-core` mechanic noun).
3. Id types are game-local -> codebase grep-proof: `StarSpaceId`/`StarPoint`/`StarZone` appear only under `games/starbridge_crossing/`.

## What to Change

### 1. Create the crate

`games/starbridge_crossing/Cargo.toml` (deps `engine-core`, `game-stdlib`, `ai-core`; dev-dep `game-test-support`) and `src/lib.rs` (module declarations stubbed for the pipeline: `ids`, plus `pub mod` stubs added as later tickets land).

### 2. Add `games/starbridge_crossing` to the workspace `members` in root `Cargo.toml`.

### 3. Author `src/ids.rs`

`StarSpaceId(u16)` (0..120, stable manifest order), `StarPoint` (North, NorthEast, SouthEast, South, SouthWest, NorthWest), `StarZone` (home/target/neutral classification), and a stable seat-label mapping aligned to the spec's clockwise ring.

## Files to Touch

- `Cargo.toml` (modify)
- `games/starbridge_crossing/Cargo.toml` (new)
- `games/starbridge_crossing/src/lib.rs` (new)
- `games/starbridge_crossing/src/ids.rs` (new)

## Out of Scope

- Topology content (`topology.rs`, `data/manifest.toml`) — GAT20STACROSTA-005.
- Setup/state/rules/effects/bots — later tickets.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p starbridge_crossing`
2. `cargo fmt --check`
3. `bash scripts/boundary-check.sh`

### Invariants

1. `engine-core` gains no mechanic noun (§3).
2. Id types live only in `games/starbridge_crossing` (§3 game-local).

## Test Plan

### New/Modified Tests

1. `games/starbridge_crossing/src/ids.rs` — inline unit tests for `StarSpaceId` range bound and `StarPoint`/`StarZone` exhaustiveness.

### Commands

1. `cargo test -p starbridge_crossing`
2. `cargo build --workspace && bash scripts/boundary-check.sh`
3. Narrower command (`-p starbridge_crossing`) is the correct boundary for the crate-local id tests; the workspace build confirms member wiring.

## Outcome

Completed: 2026-06-27

What changed:

- Added `games/starbridge_crossing` to the workspace members.
- Added `games/starbridge_crossing/Cargo.toml` with the same dependency shape
  as sibling new-game crates: `engine-core`, `game-stdlib`, `ai-core`, and
  dev-only `game-test-support`.
- Added `games/starbridge_crossing/src/lib.rs` and `src/ids.rs` with game-local
  constants, `StarSpaceId`, `StarPoint`, `StarZone`, supported-seat helpers,
  active point mapping, canonical seat ids, and inline unit tests.

Deviations from plan:

- None.

Verification:

- `cargo test -p starbridge_crossing` passed: 4 unit tests, 0 doctests.
- `cargo build -p starbridge_crossing` passed.
- `cargo fmt --all --check` passed after rustfmt formatting.
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`;
  `game-test-support dev-only boundary check passed`).
- `cargo build --workspace` passed.
- `rg -n "StarSpaceId|StarPoint|StarZone" crates games tools apps --glob '!games/starbridge_crossing/**'`
  produced no output, confirming the id types are source-local to the new game
  crate. A broader repository grep still finds planned references in active
  tickets/specs, which are not code ownership leaks.
- `git diff --check` passed.

Unrelated worktree changes left untouched:

- `.claude/skills/spec-to-tickets/SKILL.md`
