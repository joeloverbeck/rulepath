# GAT7DRALITCOM-003: Conditional `game-stdlib` board-space primitive extraction

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `crates/game-stdlib/src/lib.rs` and a new board-space module (`crates/game-stdlib/src/board_space.rs` or equivalent): rule-agnostic rectangular-coordinate helper. CONDITIONAL on GAT7DRALITCOM-002.
**Deps**: 002

## Problem

Worked **only if** GAT7DRALITCOM-002 decides *promote*. If so, this ticket adds a narrow, rule-agnostic rectangular board-space primitive to `game-stdlib` (dimensions, row/column coordinate type, bounds checking, deterministic row-major iteration, `rNcM` id format/parse, signed offset arithmetic, optional generic parity helper), with tests, so `games/draughts_lite` builds its rules on a shared coordinate helper instead of a fully-local one. If GAT7DRALITCOM-002 decides defer/reject, this ticket is not-applicable and the coordinate logic stays local in `games/draughts_lite/src/rules.rs` (GAT7DRALITCOM-005).

## Assumption Reassessment (2026-06-07)

1. `crates/game-stdlib/src/lib.rs` is a placeholder (`placeholder_version()`); promotion adds a new module and exports it from `lib.rs`. `crates/game-stdlib/Cargo.toml` exists and is already a workspace member (`Cargo.toml:5`).
2. The promotion decision, limits, and reopen constraints are fixed by GAT7DRALITCOM-002's `PRIMITIVE-PRESSURE-LEDGER.md` and the superseded `docs/MECHANIC-ATLAS.md` rows; the spec §R12 "Minimal safe primitive" / "Explicitly out of scope for the primitive" lists are authoritative for the helper's surface.
3. Cross-crate boundary under audit: the helper is consumed by `games/draughts_lite/src/rules.rs` (GAT7DRALITCOM-005) and must NOT be consumed by `engine-core`. It must stay rule-agnostic — no draughts vocabulary (capture, king, dark-square, forced) and no origin/order policy.
4. FOUNDATIONS §4 motivates this ticket: restate before coding — a `game-stdlib` helper is earned, not speculative; its existence is justified only by the GAT7DRALITCOM-002 ledger decision, and its surface is bounded to behavior-free board-space concepts.
5. Third-use hard-gate enforcement surface (§4): the promoted helper must (a) introduce no hidden-information path — board geometry is public — and (b) force no trace/hash migration. Enforce (b) directly: existing games are NOT retrofitted (spec §R12 retrofit policy), so `cargo test --workspace` for `three_marks`/`column_four`/`directional_flip` must pass unchanged after the helper lands.

## Architecture Check

1. A single shared coordinate helper (when promoted) removes the duplicate `rNcM` parse/format + bounds + row-major iteration logic that would otherwise be re-implemented in `draughts_lite`; keeping it behavior-free means it cannot encode any one game's rules.
2. No backwards-compatibility shims; the placeholder is extended, not aliased.
3. `engine-core` stays noun-free (§3); the helper lives in `game-stdlib` and is earned via the mechanic atlas decision in GAT7DRALITCOM-002 (§4).

## Verification Layers

1. Helper is rule-agnostic -> codebase grep-proof: the new module contains no draughts vocabulary (`capture`, `king`, `dark`, `forced`, `promot`) — `grep -iE` returns nothing.
2. Helper correctness -> `game-stdlib` unit tests: bounds, row-major iteration order, `rNcM` round-trip parse/format, signed offset arithmetic.
3. Existing games unaffected -> `cargo test --workspace`: `three_marks`/`column_four`/`directional_flip` tests pass unchanged (no trace/hash migration).
4. Kernel boundary -> `bash scripts/boundary-check.sh`: `engine-core` stays noun-free.

## What to Change

### 1. Board-space module in `game-stdlib`

Add a behavior-free rectangular board-space module: a `Dimensions { rows, cols }` type, a coordinate type, `in_bounds`, deterministic row-major iteration, `rNcM` id format + parse (compatible with existing public ids), signed `(d_row, d_col)` offset arithmetic returning `Option<Coord>`, and an optional generic parity predicate (expressed as parity, never "dark square"). No occupancy, piece, move, capture, or promotion concept.

### 2. Export from `game-stdlib`

Export the module from `crates/game-stdlib/src/lib.rs`; retain `placeholder_version()` only if still referenced, otherwise replace it cleanly (no alias shim).

## Files to Touch

- `crates/game-stdlib/src/board_space.rs` (new)
- `crates/game-stdlib/src/lib.rs` (modify — export the new module)
- `crates/game-stdlib/Cargo.toml` (modify — only if a dev-dependency is needed for tests)

## Out of Scope

- Proceeding at all if GAT7DRALITCOM-002 decided defer/reject — this ticket is then not-applicable, and `games/draughts_lite/src/rules.rs` (GAT7DRALITCOM-005) implements coordinate handling locally instead.
- Any draughts rule (move generation, captures, mandatory continuation, promotion, playable-square policy) — these stay in `games/draughts_lite` (spec §R12 "out of scope for the primitive").
- Retrofitting `three_marks` / `column_four` / `directional_flip` to the helper (forbidden this gate).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p game-stdlib` — board-space unit tests pass.
2. `cargo test --workspace` — existing games pass unchanged (no trace/hash migration).
3. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. The helper is rule-agnostic — no draughts vocabulary in `game-stdlib` (FOUNDATIONS §4; §R12 out-of-scope list).
2. Existing games' replay hashes and traces are unchanged — promotion forces no migration (FOUNDATIONS §11/§13).

## Test Plan

### New/Modified Tests

1. `crates/game-stdlib/src/board_space.rs` (inline `#[cfg(test)]` module) — bounds, row-major iteration order, `rNcM` round-trip, offset arithmetic.

### Commands

1. `cargo test -p game-stdlib`
2. `cargo test --workspace && bash scripts/boundary-check.sh`
3. Workspace-wide tests are the correct boundary because the load-bearing invariant is "existing games do not regress"; a crate-scoped run alone would not prove the no-migration constraint.

## Outcome

Completed: 2026-06-07

What changed:
- Replaced the `game-stdlib` placeholder surface with an exported `board_space` module.
- Added `Dimensions`, `Coord`, `CoordIdError`, `Parity`, and `RowMajor` for behavior-free rectangular board-space use.
- Added helper coverage for nonzero dimensions, one-based coordinates, bounds checks, deterministic row-major iteration, stable `rNcM` parse/format, signed offsets, row-major indexes, and generic parity.

Deviations from original plan:
- None. GAT7DRALITCOM-002 chose `promote`, so this ticket was applicable.

Verification:
- `cargo test -p game-stdlib` passed (10 unit tests).
- `cargo fmt --all --check` passed after formatting the new module.
- `cargo test --workspace` passed.
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`).
- `grep -iE "capture|king|dark|forced|promot" crates/game-stdlib/src/board_space.rs` produced no matches.
