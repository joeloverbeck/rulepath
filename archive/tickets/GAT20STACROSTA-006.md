# GAT20STACROSTA-006: Setup, seat validation, and deterministic state

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/starbridge_crossing/src/{setup.rs,state.rs,variants.rs}`, `data/variants.toml`
**Deps**: GAT20STACROSTA-005

## Problem

Starbridge supports exactly `{2, 3, 4, 6}` seats with deterministic home assignment and 10 pegs per seat, and must reject unsupported counts (including 1 and 5) with Rust diagnostics. This ticket lands setup, the deterministic public state snapshot, and the variant content.

## Assumption Reassessment (2026-06-27)

1. `crates/game-stdlib/src/seat.rs` exports `SeatCount`, `SeatCountRange`, `next_ring_index`, `checked_index` — confirmed; reused for the discontinuous `{2,3,4,6}` declaration and clockwise ring ordering (home/target assignment + finish-skip stay game-local per the 003 audit).
2. `data/variants.toml` mirrors the sibling typed shape (`games/vow_tide/data/variants.toml`): one variant `starbridge_crossing_classic_star_v1`, typed parameters only (seat set, `max_plies` default 2000, peg count 10).
3. Cross-artifact boundary: setup writes the initial occupancy onto the 121-space topology (GAT20STACROSTA-005) and produces the public `state.rs` snapshot every later module reads; seat-label ring and seat→home mapping are pinned from spec §1 (`2`: north↔south; `3`: alternating points; `4`: two opposite pairs; `6`: all points).
4. §2 (behavior authority) motivates this ticket: Rust owns setup validation and seat-count rejection; unsupported counts are Rust setup diagnostics, never TypeScript-decided. Diagnostics are fail-closed and deterministic (§11).

## Architecture Check

1. A single Rust setup path producing a deterministic public snapshot keeps validation fail-closed and replayable; reusing `seat` helpers avoids re-implementing ring arithmetic.
2. No backwards-compatibility shims.
3. `engine-core` untouched; `game-stdlib::seat` reused (earned, already promoted), not extended.

## Verification Layers

1. Supported seat set (§2) -> rule test: setup succeeds for `{2,3,4,6}`, rejects `{1,5,7}` with stable diagnostics.
2. Peg/home placement -> rule test: each seat gets 10 pegs in its home point; opposite-home target matches topology mapping.
3. Deterministic state -> golden setup traces (authored with the trace catalog in 011) + serialization round-trip; here, a unit test on snapshot determinism for a fixed seed.
4. Diagnostic discipline (§11) -> rule test: rejection diagnostics are blocking and deterministic, not warnings.

## What to Change

### 1. Author `src/variants.rs` + `data/variants.toml`

Load the typed variant (seat set `{2,3,4,6}`, default 2, `max_plies` 2000, 10 pegs). Reject unknown fields.

### 2. Author `src/setup.rs`

Seat-count validation via `game-stdlib::seat`; deterministic home/target assignment per the `{2,3,4,6}` mapping; place 10 pegs per seat in its home; emit setup diagnostics for unsupported counts.

### 3. Author `src/state.rs`

The public match state snapshot: per-space occupancy, active seat, finish-rank ledger, terminal/turn-limit status — all public (perfect information).

## Files to Touch

- `games/starbridge_crossing/src/setup.rs` (new)
- `games/starbridge_crossing/src/state.rs` (new)
- `games/starbridge_crossing/src/variants.rs` (new)
- `games/starbridge_crossing/data/variants.toml` (new)
- `games/starbridge_crossing/src/lib.rs` (modify; created by 004 — add `pub mod {setup,state,variants};`)

## Out of Scope

- Legal move generation (step/hop) — GAT20STACROSTA-007/008.
- Finish/terminal detection — GAT20STACROSTA-009.
- Golden setup traces + fixtures — authored in GAT20STACROSTA-011.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p starbridge_crossing --test rules` (setup/seat-validation rules tests)
2. `cargo test -p starbridge_crossing`
3. `bash scripts/boundary-check.sh`

### Invariants

1. Setup supports exactly `{2,3,4,6}`; all other counts are rejected with stable Rust diagnostics (§2).
2. Each seat starts with 10 pegs in its home; the public snapshot leaks no non-existent private datum (perfect information).

## Test Plan

### New/Modified Tests

1. `games/starbridge_crossing/tests/rules.rs` — (created here) setup counts, seat rejection, home/peg placement.
2. `games/starbridge_crossing/src/state.rs` — inline snapshot-determinism unit test.

### Commands

1. `cargo test -p starbridge_crossing --test rules`
2. `cargo test -p starbridge_crossing && bash scripts/boundary-check.sh`
3. The `--test rules` boundary isolates setup/seat behavior; the full crate run confirms state/topology integration.

## Outcome

Completed: 2026-06-27

What changed:

- Added `games/starbridge_crossing/src/variants.rs` and
  `games/starbridge_crossing/data/variants.toml` for the classic Starbridge
  variant: supported seats `{2,3,4,6}`, default 2, max plies 2000, and 10 pegs
  per seat. The parser rejects unknown and behavior-looking fields.
- Added `games/starbridge_crossing/src/setup.rs` with Rust-owned variant
  validation, discontinuous seat-count validation, deterministic home/target
  assignment, and initial public peg placement.
- Added `games/starbridge_crossing/src/state.rs` with public state and snapshot
  types for seats, occupancy, pegs, active seat, finish ranks, terminal status,
  ply/command counters, freshness, and stable snapshot bytes.
- Added `games/starbridge_crossing/tests/rules.rs` covering supported and
  rejected seat counts, home/target assignment, peg placement, and empty
  non-home spaces.
- Updated `games/starbridge_crossing/src/lib.rs` to expose setup, state, and
  variant APIs.

Deviations from plan:

- Golden setup traces remain deferred to GAT20STACROSTA-011 as planned. This
  ticket pins deterministic setup through unit/integration tests and snapshot
  stable bytes.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p starbridge_crossing --test rules` passed: 5 integration tests.
- `cargo test -p starbridge_crossing` passed: 14 unit tests, 6 integration
  tests, 0 doctests.
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`;
  `game-test-support dev-only boundary check passed`).

Unrelated worktree changes left untouched:

- `.claude/skills/spec-to-tickets/SKILL.md`
