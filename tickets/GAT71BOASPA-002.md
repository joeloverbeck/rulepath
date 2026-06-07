# GAT71BOASPA-002: Back-port `column_four` cell/coordinate identity to `game-stdlib::board_space`

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/column_four` (`src/ids.rs`, `src/rules.rs` as needed, `Cargo.toml`); consumes `crates/game-stdlib::board_space` (`Dimensions`, `Coord`, `CoordIdError`) — no `board_space` additions expected (§14). Docs: `games/column_four/docs/MECHANICS.md`.
**Deps**: None

## Problem

`column_four` carries local `ColumnId`, `RowId`, and `CellId` types for a 6×7 board (`games/column_four/src/ids.rs:47,120,180`), with `CellId` implementing local `index`, `as_string`, and `parse` over a 42-cell row-major `ALL` — duplicating `game-stdlib::board_space` coordinate identity, bounds, `rNcM` parse/format, and row-major iteration. This is open promotion debt (spec `specs/gate-7-1-board-space-primitive-back-port.md` §10; atlas §10A) that must close before Gate 8. This ticket conforms the behavior-free coordinate/cell subset to `board_space` while keeping column actions, gravity, full-column checks, landing-row calculation, and four-in-a-row scans local and all public surfaces unchanged.

## Assumption Reassessment (2026-06-07)

1. `games/column_four/src/ids.rs` defines `pub enum ColumnId` (`:47`), `pub enum RowId` (`:120`), and `pub struct CellId` (`:180`) with local `index` (`:235`) and `as_string` (`:239`). `CellId`/`RowId`/`ColumnId` thread through the whole crate (`ids.rs`, `state.rs`, `rules.rs`, `effects.rs`, `visibility.rs`, `actions.rs`, `bots.rs`, `ui.rs`, `replay_support.rs`, `lib.rs`), so the retrofit MUST preserve those public surfaces (thin `CellId(Coord)` wrapper / lossless conversion) to confine the diff. Gravity lives in `rules.rs::landing_cell` (`rules.rs:141`), consumed by `visibility.rs`, `bots.rs`, and `rules.rs` — it stays local.
2. `crates/game-stdlib/src/board_space.rs` exposes the needed behavior-free API (`Dimensions::checked` `:16`, `coord` `:32`, `parse_coord_id` `:37`, `row_major` `:66`, `Coord::row`/`col` `:86,90`, `row_index`/`col_index` `:94,98`, `id`/`parse_id` `:108,112`). `draughts_lite/src/ids.rs:1` already consumes it; no additions anticipated. Spec scope: §10.
3. Shared boundary under audit: `games/column_four` ↔ `crates/game-stdlib::board_space`, one-directional. `ColumnId` (the player action vocabulary, `c1..c7`) and `RowId` are game-local vocabulary; only their duplicated parse/index/bounds behavior within the `board_space` scope is delegated.
4. FOUNDATIONS §4 (`game-stdlib` earned) and §11 (promoted primitive adopted by all matching games) motivate this ticket — the recorded back-port, not a new promotion. Gravity, full-column, four-in-a-row, and winning-line tie-breaks are game rules and stay local per §5.
5. Deterministic replay/hash & serialization surface: `CellId::ALL` order and `index()` values (`r1c1`..`r6c7`, row-major), `CellId` display strings, `ColumnId::ALL` action ordering (`c1`..`c7`), `landing_cell` results, and terminal/highlight effects all feed golden traces and replay hashes and MUST stay byte-identical (FOUNDATIONS §11; spec §10.2, §8.2). The **row-origin semantics** are load-bearing: if row 1 is the public lowest/bottom landing row, generic abstract row-major iteration MUST NOT flip it (spec §10.2). Any change is a §13 ADR-gated migration, out of scope.
6. Adjacent contradiction handling: if the wrapper cannot preserve a signature a sibling file depends on, that sibling edit is a required consequence and joins Files to Touch; a non-coordinate defect becomes its own ticket.
7. Mismatch + correction: none at authoring — spec §10.1 matches `ids.rs`/`rules.rs`.

## Architecture Check

1. Delegating cell/coordinate identity to `board_space` removes 42 cells' worth of duplicated parse/index/bounds/iteration while keeping `ColumnId` as the readable action vocabulary — one coordinate contract, no per-game fork, consistent with the `draughts_lite` exemplar.
2. No backwards-compatibility aliasing/shim: `CellId` is re-expressed over `Coord`; `RowId` becomes a thin adapter (around `Coord::row()`) or a retained thin wrapper with its duplicated parse/index/bounds delegated — the old local bodies are removed, not kept as fallbacks.
3. `engine-core` untouched (§3). `game-stdlib` consumed, not broadened (§4); gravity / full-column / four-in-a-row / tie-breaks remain in the game crate.

## Verification Layers

1. `CellId` ↔ `Coord` round-trips all 42 cells → Rust unit test (inline in `ids.rs`).
2. `CellId::ALL` + `index()` match the previous row-major public order, and `ColumnId::ALL` action order is unchanged → Rust unit tests asserting the literal sequences.
3. `landing_cell` behavior unchanged for empty, partially-filled, and full columns → Rust unit test (mirrors the existing `rules.rs:287` `landing_cell` usage) + existing `games/column_four/tests/rules.rs`.
4. Trace/replay-hash preservation → `cargo run -p replay-check -- --game column_four --all` (all eleven golden traces unchanged).
5. Row-origin not flipped; game-local gravity/win logic intact → existing `games/column_four/tests/property.rs` + `replay.rs`.
6. `engine-core` noun-free and `board_space` not broadened → `bash scripts/boundary-check.sh` + FOUNDATIONS §3/§4 review.
7. Bot legality unchanged → existing `games/column_four/tests/bots.rs`.

## What to Change

### 1. Add the `game-stdlib` dependency

In `games/column_four/Cargo.toml`, add `game-stdlib = { path = "../../crates/game-stdlib" }`.

### 2. Re-express `CellId` (and delegate `RowId` mechanics) over `board_space`

In `games/column_four/src/ids.rs`:
- Use `Dimensions::checked(6, 7).expect("6x7 board dimensions are valid")` as the board-space dimension source.
- Make `CellId` rely on `Coord`/`Dimensions` for cell coordinate identity, bounds, `rNcM` parse/format, row-major indexing, and deterministic iteration (prefer a thin `CellId(Coord)` wrapper or lossless conversion). Preserve cell IDs `r1c1`..`r6c7`, display strings, `ALL` order, and `index()` values exactly.
- Keep `ColumnId` as the player action vocabulary — do not replace it with raw coordinates. Preserve `ColumnId::ALL` ordering (`c1`..`c7`).
- Reduce `RowId` to a thin adapter around `Coord::row()` (or a retained thin wrapper) and delegate its duplicated parse/index/bounds within the `board_space` scope. Preserve the public row-origin orientation.

### 3. Keep rules local

`landing_cell`/gravity, full-column checks, draw/full-board detection, four-in-a-row scans, winning-line tie-breaks, terminal effects, and highlight semantics stay in `src/rules.rs`. Touch `rules.rs` only where it constructs coordinates directly and can delegate to `Dimensions`/`Coord` without changing observable behavior (notably the row-origin used by `landing_cell`).

### 4. Document conformance

In `games/column_four/docs/MECHANICS.md`, record `board_space` conformance for cell/coordinate identity while preserving columns/gravity/four-in-a-row as local.

## Files to Touch

- `games/column_four/Cargo.toml` (modify)
- `games/column_four/src/ids.rs` (modify)
- `games/column_four/src/rules.rs` (modify — coordinate construction in `landing_cell` and scans only, preserving row-origin)
- `games/column_four/docs/MECHANICS.md` (modify)
- Additional `games/column_four/src/*.rs` siblings (modify — only "as surfaced" if the wrapper cannot preserve a signature; see Assumption Reassessment 6)

## Out of Scope

- Flipping row origin or display orientation (spec §10.2, §18).
- Any change to `CellId::ALL`/`index()`, cell display strings, or `ColumnId::ALL` action ordering.
- Replacing `ColumnId` with raw cell coordinates.
- Promoting gravity, full-column, four-in-a-row, or tie-break logic into `game-stdlib`/`engine-core` (§4, §18).
- Adding behavior to `board_space` beyond consuming its existing API (§14).
- Updating golden traces to absorb drift; a real trace change needs an explicit §13 migration decision.
- TypeScript legality/preview changes (§2).

## Acceptance Criteria

### Tests That Must Pass

1. New `ids.rs` unit test: `CellId` ↔ `Coord` round-trips all 42 cells losslessly.
2. New `ids.rs` unit test: `CellId::ALL`/`index()` match the prior row-major public order and `ColumnId::ALL` is `[c1..c7]`.
3. New unit test: `landing_cell` returns the same cell for empty, partially-filled, and full columns as pre-retrofit (row-origin preserved).
4. `cargo test -p column_four` (rules, property, visibility, replay, bot suites green).
5. `cargo run -p replay-check -- --game column_four --all` (all eleven golden traces pass unchanged).

### Invariants

1. `games/column_four` depends on `game-stdlib` and no longer duplicates `board_space` coordinate parse/index/bounds/iteration in the promoted scope.
2. Public cell IDs, `ColumnId` action ordering, row-origin, landing behavior, diagnostics, effects, and golden-trace hashes are byte-identical to pre-retrofit.

## Test Plan

### New/Modified Tests

1. `games/column_four/src/ids.rs` (inline `#[cfg(test)]`) — 42-cell round-trip + `ALL`/`index`/`ColumnId::ALL` order tests.
2. `games/column_four/tests/rules.rs` — add/confirm a landing-cell-by-column case (empty/partial/full) proving row-origin preservation.
3. Existing `games/column_four/tests/*.rs` — run unchanged as regression proof.

### Commands

1. `cargo test -p column_four`
2. `cargo run -p replay-check -- --game column_four --all`
3. `cargo run -p fixture-check -- --game column_four && cargo run -p rule-coverage -- --game column_four && bash scripts/boundary-check.sh && cargo clippy -p column_four --all-targets -- -D warnings`
