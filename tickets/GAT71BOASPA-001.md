# GAT71BOASPA-001: Back-port `three_marks` cell identity to `game-stdlib::board_space`

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/three_marks` (`src/ids.rs`, `src/rules.rs` as needed, `Cargo.toml`); consumes `crates/game-stdlib::board_space` (`Dimensions`, `Coord`, `CoordIdError`) — no `board_space` additions expected (§14). Docs: `games/three_marks/docs/MECHANICS.md`.
**Deps**: None

## Problem

Gate 7 promoted the behavior-free rectangular board-space primitive into `crates/game-stdlib/src/board_space.rs` and made Draughts Lite use it, but left `three_marks` carrying its own local 3×3 coordinate machinery. `games/three_marks/src/ids.rs` defines a `CellId` enum with local `ALL`, `index`, `as_str`, and `parse`, duplicating `board_space` coordinate identity, bounds, `rNcM` parse/format, and row-major iteration. This is open promotion debt (spec `specs/gate-7-1-board-space-primitive-back-port.md` §9; atlas `docs/MECHANIC-ATLAS.md` §10A) that must close before Gate 8. This ticket conforms the behavior-free coordinate subset to `board_space` while keeping all game rules and public surfaces unchanged.

## Assumption Reassessment (2026-06-07)

1. `games/three_marks/src/ids.rs` currently defines `pub enum CellId` with `pub const ALL: [Self; 9]`, `pub fn index`, `pub fn as_str`, and `pub fn parse` (verified `ids.rs:47,60,72,86,100`). `CellId` is referenced across the whole crate — `ids.rs`, `state.rs`, `rules.rs`, `effects.rs`, `visibility.rs`, `actions.rs`, `bots.rs`, `ui.rs`, `replay_support.rs`, `lib.rs` — so the retrofit MUST preserve the `CellId` public surface (a thin wrapper around `Coord` or lossless `CellId` ↔ `Coord` conversion) to keep the diff confined to `ids.rs` and avoid rippling into the other ten files.
2. `crates/game-stdlib/src/board_space.rs` exposes exactly the needed behavior-free API: `Dimensions::checked` (`:16`), `Dimensions::contains` (`:28`), `Dimensions::coord` (`:32`), `Dimensions::parse_coord_id` (`:37`), `Dimensions::row_major` (`:66`), `Coord::checked` (`:82`), `Coord::row_index`/`col_index` (`:94,98`), `Coord::row_col_index` (`:102`), `Coord::id`/`parse_id` (`:108,112`), and `CoordIdError` (`:140`). `draughts_lite` already consumes it (`games/draughts_lite/src/ids.rs:1`), so no `board_space` additions are anticipated. Spec scope: `specs/gate-7-1-board-space-primitive-back-port.md` §9.
3. Shared boundary under audit: `games/three_marks` ↔ `crates/game-stdlib::board_space`. The contract is one-directional — the game consumes generic coordinate identity and keeps all rule semantics local. No type crosses back into `game-stdlib`.
4. FOUNDATIONS §4 (`game-stdlib` is earned) and §11 (promoted primitives are adopted by all matching official games, or carry an accepted exception) motivate this ticket: this is the recorded back-port, not a new promotion. The win-line logic stays local per §5 (static win lines are game rules, not the board-space primitive).
5. Deterministic replay/hash & serialization surface: `CellId::ALL` order (`r1c1, r1c2, r1c3, r2c1, r2c2, r2c3, r3c1, r3c2, r3c3`), `CellId::index()` row-major values, `CellId::as_str()` output, action paths (`place/r1c1`), and `ThreeMarksSnapshot::stable_summary()` ordering all feed golden traces and replay hashes. The retrofit MUST hold these byte-identical (FOUNDATIONS §11 deterministic-replay; spec §9.2, §8.2); any change is a §13 ADR-gated migration, out of scope here.
6. Adjacent contradiction handling: if the wrapper/conversion cannot preserve a signature that a sibling file (`state.rs`, `effects.rs`, …) depends on, that sibling edit is a *required consequence* of this ticket and joins Files to Touch — it is NOT a separate bug. If a non-coordinate defect surfaces, it becomes its own ticket.
7. Mismatch + correction: none at authoring time — the spec's §9.1 description of the current shape matches `ids.rs`.

## Architecture Check

1. Delegating coordinate identity to `board_space` removes nine cells' worth of duplicated parse/index/bounds/iteration logic and makes `three_marks` consistent with the `draughts_lite` exemplar, so future readers find one coordinate contract rather than per-game forks. A thin `CellId` wrapper preserves the game's readable semantic vocabulary in traces/UI while delegating the mechanics.
2. No backwards-compatibility aliasing or shim: `CellId` is re-expressed in terms of `Coord`, not duplicated alongside it; the old local parse/index bodies are removed, not retained as fallbacks.
3. `engine-core` is untouched (no mechanic noun enters the kernel, §3). The `game-stdlib` change is consumption-only of an already-earned primitive (§4); no new helper is promoted, and `WINNING_LINES` stays in the game crate.

## Verification Layers

1. `CellId` ↔ `Coord` round-trips all 9 cells → Rust unit test (inline `#[cfg(test)]` in `ids.rs`, matching the `draughts_lite/src/ids.rs` house pattern).
2. `CellId::ALL` order and `CellId::index()` values are byte-identical to the pre-retrofit list → Rust unit test asserting the literal sequence.
3. Trace/replay-hash preservation → `cargo run -p replay-check -- --game three_marks --all` (golden traces unchanged, per `docs/TESTING-REPLAY-BENCHMARKING.md`).
4. Game-local win/occupancy/diagnostic behavior unchanged → existing `games/three_marks/tests/rule_tests.rs` + `property_tests.rs`.
5. `engine-core` stays noun-free and `board_space` is consumed not broadened → `bash scripts/boundary-check.sh` + FOUNDATIONS §3/§4 alignment review.
6. Bot legality unchanged (same legal action API) → existing `games/three_marks/tests/bot_tests.rs`.

## What to Change

### 1. Add the `game-stdlib` dependency

In `games/three_marks/Cargo.toml`, add `game-stdlib = { path = "../../crates/game-stdlib" }` (matching `games/draughts_lite/Cargo.toml:11`).

### 2. Re-express `CellId` over `board_space`

In `games/three_marks/src/ids.rs`:
- Introduce a board-dimensions accessor equivalent to `Dimensions::checked(3, 3).expect("3x3 board dimensions are valid")`.
- Make `CellId` a thin semantic wrapper around `Coord` (preferred) or provide lossless `CellId` ↔ `Coord` conversion, so the public `CellId` surface is unchanged.
- Delegate coordinate construction, `rNcM` parse/format, bounds checks, row-major indexing, and deterministic iteration to `Coord`/`Dimensions`/`RowMajor`. Remove the duplicated local `parse`/`index`/bounds bodies (no shim left behind).
- Preserve `CellId::ALL` (exact order), `CellId::index()` (row-major values), and `CellId::as_str()` output.

### 3. Keep rules local

`WINNING_LINES`, line-completion, occupancy, terminal detection, diagnostics, effects, and `ThreeMarksSnapshot::stable_summary()` formatting stay in the game crate. Touch `src/rules.rs` only if it constructs coordinates directly and can delegate to `Dimensions`/`Coord` without altering observable behavior.

### 4. Document conformance

In `games/three_marks/docs/MECHANICS.md`, record that 3×3 coordinate/cell identity now conforms to `game-stdlib::board_space` while win-line logic remains local.

## Files to Touch

- `games/three_marks/Cargo.toml` (modify)
- `games/three_marks/src/ids.rs` (modify)
- `games/three_marks/src/rules.rs` (modify — only if it constructs coordinates directly; else untouched)
- `games/three_marks/docs/MECHANICS.md` (modify)
- Additional `games/three_marks/src/*.rs` siblings (modify — only "as surfaced" if the wrapper cannot preserve a signature; see Assumption Reassessment 6)

## Out of Scope

- Any change to `CellId::ALL` order, `index()` values, `as_str()` output, action paths, or `stable_summary()` formatting (preserve byte-identical).
- Promoting win-line / pattern detection or any rule logic into `game-stdlib` or `engine-core` (spec §4, §18).
- Adding behavior to `board_space` beyond consuming its existing API (§14).
- Updating golden traces to absorb drift (spec §8.7); a genuine trace change requires an explicit §13 migration decision, not this ticket.
- TypeScript legality/preview changes (FOUNDATIONS §2).

## Acceptance Criteria

### Tests That Must Pass

1. New `ids.rs` unit test: `CellId` ↔ `Coord` round-trips all 9 cells losslessly.
2. New `ids.rs` unit test: `CellId::ALL` equals `[r1c1, r1c2, r1c3, r2c1, r2c2, r2c3, r3c1, r3c2, r3c3]` and `index()` is the matching 0..9 row-major sequence.
3. `cargo test -p three_marks` (rules, property, serialization, visibility, replay, bot suites all green).
4. `cargo run -p replay-check -- --game three_marks --all` (all eight golden traces in `games/three_marks/tests/golden_traces/` pass unchanged).

### Invariants

1. `games/three_marks` depends on `game-stdlib` and no longer maintains local coordinate parse/index/bounds/iteration that duplicates `board_space` within the promoted scope.
2. Public `CellId` surface, action paths, diagnostics, effects, summaries, and golden-trace hashes are byte-identical to pre-retrofit.

## Test Plan

### New/Modified Tests

1. `games/three_marks/src/ids.rs` (inline `#[cfg(test)]`) — `CellId` ↔ `Coord` round-trip and `ALL`/`index` order-preservation tests.
2. Existing `games/three_marks/tests/*.rs` — run unchanged as regression proof that behavior is preserved.

### Commands

1. `cargo test -p three_marks`
2. `cargo run -p replay-check -- --game three_marks --all`
3. `cargo run -p fixture-check -- --game three_marks && cargo run -p rule-coverage -- --game three_marks && bash scripts/boundary-check.sh && cargo clippy -p three_marks --all-targets -- -D warnings`
