# GAT5COLFOUPUB-002: Column Four crate skeleton & workspace wiring

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — new crate `games/column_four` (`Cargo.toml`, `src/lib.rs`, `src/ids.rs`, `src/state.rs`, `src/setup.rs`, `src/variants.rs`), `games/column_four/data/manifest.toml`, `games/column_four/data/variants.toml`; modify root `Cargo.toml` workspace members
**Deps**: 001

## Problem

`column_four` needs a compilable crate skeleton — typed ids, empty state, setup, variant metadata, and static data files — registered in the workspace, before rules/view/effects/bot logic can attach. This establishes the local game-owned vocabulary (coordinates, seats, variant) that all later tickets build on (spec §5 game-admission scope, §7 identity).

## Assumption Reassessment (2026-06-06)

1. `games/three_marks` is the structural template: it carries `src/{ids,state,setup,variants,lib}.rs` plus `data/{manifest.toml,variants.toml}` and depends on `engine-core` + `ai-core`. Verified via `games/three_marks/src/` listing and `games/three_marks/data/`. This ticket mirrors that layout for `column_four`.
2. Root `Cargo.toml` lists workspace members including `games/race_to_n` and `games/three_marks` (verified lines in `[workspace].members`). Spec §5 requires adding `games/column_four`. Identity values come from GAT5COLFOUPUB-001's `RULES.md` (`column_four`, `column_four_standard`, `column_four-rules-v1`, cell ids `r1c1`..`r6c7`).
3. Cross-crate boundary under audit: the new crate is a `games/*` member that MAY use typed mechanic nouns locally; it depends on `engine-core` contracts (ids, action/effect envelopes, seeds) but MUST NOT add mechanic nouns to `engine-core` itself. `scripts/boundary-check.sh` only scans `crates/engine-core/src`, so adding `games/column_four` cannot trip it.
4. FOUNDATIONS §3 (`engine-core` is a contract kernel) and §16 of the spec (local-only mechanics, no extraction) motivate this ticket: coordinate/occupancy/gravity types live in `games/column_four`, never in `engine-core` or `game-stdlib` this gate.

## Architecture Check

1. A standalone crate mirroring `three_marks` keeps `column_four` mechanics local and the kernel generic — cleaner than threading column/gravity types through `engine-core`. Alternative (extract a shared board primitive now) is forbidden by spec §16 and FOUNDATIONS §4 (earned-only promotion; extraction deferred past `directional_flip`).
2. No backwards-compatibility aliasing/shims — new crate and new static-data files.
3. `engine-core` gains no mechanic nouns (the crate only consumes engine-core contracts); `game-stdlib` is untouched (no helper extraction).

## Verification Layers

1. Workspace-membership invariant -> codebase grep-proof (`column_four` present in root `Cargo.toml` members) + `cargo build -p column_four` compiles.
2. Local-vocabulary invariant -> codebase grep-proof (coordinate/seat/variant types defined under `games/column_four/src/`, absent from `crates/engine-core/src`).
3. Kernel-boundary invariant -> `bash scripts/boundary-check.sh` stays green (no engine-core mechanic noun added).
4. Static-data-not-behavior invariant -> manual review of `data/*.toml` (typed variant/metadata only, no selectors/conditions — FOUNDATIONS §5).

## What to Change

### 1. Crate skeleton

Create `games/column_four/Cargo.toml` (lib crate; deps `engine-core`, `ai-core`, serde as the `three_marks` crate uses). Add `games/column_four/src/lib.rs` re-exporting modules. Add `src/ids.rs` (typed `ColumnFourSeat`, column id `c1`..`c7`, cell id `r1c1`..`r6c7`, rules/variant id constants), `src/state.rs` (empty 7×6 occupancy state + ply/active-seat), `src/setup.rs` (deterministic initial state, `seat_0` starts), `src/variants.rs` (`column_four_standard` typed variant).

### 2. Static data

Create `games/column_four/data/manifest.toml` and `games/column_four/data/variants.toml` mirroring `three_marks` shape — typed display metadata, variant id, board dimensions as parameters. No behavior fields.

### 3. Workspace registration

Add `"games/column_four",` to root `Cargo.toml` `[workspace].members`.

## Files to Touch

- `games/column_four/Cargo.toml` (new)
- `games/column_four/src/lib.rs` (new)
- `games/column_four/src/ids.rs` (new)
- `games/column_four/src/state.rs` (new)
- `games/column_four/src/setup.rs` (new)
- `games/column_four/src/variants.rs` (new)
- `games/column_four/data/manifest.toml` (new)
- `games/column_four/data/variants.toml` (new)
- `Cargo.toml` (modify)

## Out of Scope

- Action parsing, gravity, legal/terminal logic (GAT5COLFOUPUB-003).
- Public view, effects, replay, bots, tests, traces, benchmarks (004+).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p column_four` — crate compiles.
2. `grep -q '"games/column_four"' Cargo.toml` — workspace member registered.
3. `bash scripts/boundary-check.sh` — engine-core remains noun-free.

### Invariants

1. Coordinate/seat/variant types exist only under `games/column_four/src/`, not in `crates/engine-core/src`.
2. `data/*.toml` carry typed content/metadata only — no selectors, conditions, or rule branches.

## Test Plan

### New/Modified Tests

1. `games/column_four/src/setup.rs` — unit test asserting initial 7×6 empty state, `seat_0` active, ply 0.
2. `games/column_four/src/variants.rs` — unit test asserting `column_four_standard` metadata (7 columns, 6 rows).

### Commands

1. `cargo build -p column_four`
2. `cargo test -p column_four`
3. `bash scripts/boundary-check.sh` — the boundary check is the correct narrow surface for the kernel-cleanliness invariant.
