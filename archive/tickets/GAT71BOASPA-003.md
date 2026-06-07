# GAT71BOASPA-003: Back-port `directional_flip` cell identity + offset stepping to `game-stdlib::board_space`

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/directional_flip` (`src/ids.rs`, `src/rules.rs`, `Cargo.toml`); consumes `crates/game-stdlib::board_space` (`Dimensions`, `Coord`, `CoordIdError`) — no `board_space` additions expected (§14). Docs: `games/directional_flip/docs/MECHANICS.md`, `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md`.
**Deps**: None

## Problem

`directional_flip` carries local `RowId`, `ColumnId`, and `CellId` types for an 8×8 board (`games/directional_flip/src/ids.rs:47,126,205`) and a local `step(cell, direction)` helper (`src/rules.rs:428`) that advances row/column indices and bounds through game-local cell conversion — duplicating `game-stdlib::board_space` coordinate identity, bounds, `rNcM` parse/format, row-major iteration, and bounded offset stepping. This is open promotion debt (spec `specs/gate-7-1-board-space-primitive-back-port.md` §11; atlas §10A) that must close before Gate 8. This ticket conforms the behavior-free coordinate subset and the bounded-offset step to `board_space` while keeping directional flip/bracketing/forced-pass logic local and all public surfaces unchanged.

## Assumption Reassessment (2026-06-07)

1. `games/directional_flip/src/ids.rs` defines `pub enum RowId` (`:47`), `pub enum ColumnId` (`:126`), and `pub struct CellId` (`:205`); `games/directional_flip/src/rules.rs` defines `pub enum Direction` (`:13`) and a private `fn step(cell, direction) -> Option<CellId>` (`:428`). `CellId`/`RowId`/`ColumnId` thread through the whole crate (`ids.rs`, `state.rs`, `setup.rs`, `rules.rs`, `effects.rs`, `visibility.rs`, `actions.rs`, `bots.rs`, `ui.rs`, `replay_support.rs`, `lib.rs`), so the retrofit MUST preserve those surfaces (thin `CellId(Coord)` wrapper / lossless conversion) to confine the diff.
2. `crates/game-stdlib/src/board_space.rs` exposes the needed behavior-free API, crucially `Dimensions::offset(coord, d_row, d_col) -> Option<Coord>` (`:46`) for bounds-safe stepping, plus `Dimensions::checked` (`:16`), `coord` (`:32`), `parse_coord_id` (`:37`), `row_major` (`:66`), `Coord::id`/`parse_id` (`:108,112`). `draughts_lite` already uses `offset` and `row_major` (`games/draughts_lite/src/ids.rs`, `actions.rs`), so no additions anticipated. Spec scope: §11.
3. Shared boundary under audit: `games/directional_flip` ↔ `crates/game-stdlib::board_space`, one-directional. `Direction` and `Direction::delta()` are game-local vocabulary; only the bounds-safe coordinate stepping inside `step` is delegated to `Dimensions::offset`.
4. FOUNDATIONS §4 (`game-stdlib` earned) and §11 (promoted primitive adopted by all matching games) motivate this ticket — the recorded back-port. Legal-placement generation, ray/bracket scanning, flip runs, forced pass, and terminal detection are game rules and stay local per §5.
5. Deterministic replay/hash & serialization surface: `CellId::ALL` order/`index()`/`rNcM` formatting, `Direction::ALL` order (`North, Northeast, East, Southeast, South, Southwest, West, Northwest`), flip-run/preview ordering, action ordering, diagnostics, and stable summary strings all feed golden traces and replay hashes and MUST stay byte-identical (FOUNDATIONS §11; spec §11.2, §8.2). `step` must produce identical results at interior cells, edges, and corners after delegating to `Dimensions::offset`. Any change is a §13 ADR-gated migration, out of scope.
6. Adjacent contradiction handling: if delegating `step` to `Dimensions::offset` or the `CellId` wrapper cannot preserve a signature a sibling file depends on, that sibling edit is a required consequence and joins Files to Touch; a non-coordinate defect becomes its own ticket.
7. Mismatch + correction: none at authoring — spec §11.1 matches `ids.rs`/`rules.rs`.

## Architecture Check

1. Routing `step` through `Dimensions::offset` removes the game's hand-rolled bounds arithmetic — the single highest-risk duplication of the promoted primitive — and aligns the 8×8 board with the `draughts_lite` exemplar's offset usage, so bounds-safety lives in one audited place.
2. No backwards-compatibility aliasing/shim: `CellId` is re-expressed over `Coord` and `step`'s local index/bounds math is removed in favor of `offset`; no parallel fallback path is retained.
3. `engine-core` untouched (§3). `game-stdlib` consumed, not broadened (§4); ray/bracket/flip/forced-pass logic remains in the game crate. `Direction::delta()` may stay local — only the bounds-safe step is delegated.

## Verification Layers

1. `CellId` ↔ `Coord` round-trips all 64 cells; `CellId::ALL`/`index()`/`rNcM` formatting unchanged → Rust unit tests (inline in `ids.rs`).
2. `step` through each `Direction` matches previous behavior at interior cells, edges, and corners → Rust unit test enumerating representative cells per direction.
3. `Direction::ALL` order and flip-run ordering unchanged → Rust unit test asserting the literal `Direction::ALL` sequence.
4. Trace/replay-hash preservation (flip preview, multi-direction flip, forced pass, diagnostics, terminals) → `cargo run -p replay-check -- --game directional_flip --all` (all fourteen golden traces unchanged).
5. Game-local placement/bracket/terminal logic intact → existing `games/directional_flip/tests/rules.rs` + `property.rs`.
6. `engine-core` noun-free and `board_space` not broadened → `bash scripts/boundary-check.sh` + FOUNDATIONS §3/§4 review.
7. Bot legality + visibility unchanged → existing `games/directional_flip/tests/bots.rs` + `visibility.rs`.

## What to Change

### 1. Add the `game-stdlib` dependency

In `games/directional_flip/Cargo.toml`, add `game-stdlib = { path = "../../crates/game-stdlib" }`.

### 2. Re-express `CellId` over `board_space`

In `games/directional_flip/src/ids.rs`:
- Use `Dimensions::checked(8, 8).expect("8x8 board dimensions are valid")` as the board-space dimension source.
- Make coordinate identity, bounds checks, `rNcM` parse/format, row-major indexing, and deterministic iteration rely on `Coord`/`Dimensions` (prefer a thin `CellId(Coord)` wrapper or lossless conversion). Preserve `CellId::ALL` order, `index()` values, and `rNcM` formatting exactly.

### 3. Delegate bounded stepping in `rules.rs`

In `games/directional_flip/src/rules.rs`:
- Reimplement `step(cell, direction)` to use `Dimensions::offset` (or an equivalent `board_space` operation) for bounds-safe coordinate stepping. `Direction::delta()` may remain game-local; only the index/bounds advance is delegated.
- Keep legal placement generation, ray/bracket scanning, grouped flip runs, ordered flip previews, forced pass, double-pass terminal detection, full-board terminal detection, scoring, effects, and UI preview metadata local. Preserve `Direction::ALL` order and preview/effect ordering.

### 4. Document conformance

- `games/directional_flip/docs/MECHANICS.md`: record `board_space` conformance for coordinates/offset stepping while preserving ray bracketing/flips/forced pass as local.
- `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md`: update prior board-space deferral language to state the narrow coordinate subset is now promoted and conformed, while scanning/flipping remains local.

## Files to Touch

- `games/directional_flip/Cargo.toml` (modify)
- `games/directional_flip/src/ids.rs` (modify)
- `games/directional_flip/src/rules.rs` (modify — `step` delegation; keep ray/flip/forced-pass local)
- `games/directional_flip/docs/MECHANICS.md` (modify)
- `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` (modify)
- Additional `games/directional_flip/src/*.rs` siblings (modify — only "as surfaced" if the wrapper cannot preserve a signature; see Assumption Reassessment 6)

## Out of Scope

- Any change to `CellId::ALL`/`index()`/`rNcM` formatting, `Direction::ALL` order, flip-run/preview ordering, action ordering, diagnostics, or stable summaries (preserve byte-identical).
- Promoting ray/bracket scanning, flip runs, forced pass, or terminal detection into `game-stdlib`/`engine-core` (§4, §18).
- A generic ray-scan / directional-flip primitive in `game-stdlib` (spec §14 forbidden list).
- Adding behavior to `board_space` beyond consuming its existing API (§14).
- Updating golden traces to absorb drift; a real trace change needs an explicit §13 migration decision.
- TypeScript legality/preview changes (§2).

## Acceptance Criteria

### Tests That Must Pass

1. New `ids.rs` unit test: `CellId` ↔ `Coord` round-trips all 64 cells; `CellId::ALL`/`index()`/`rNcM` formatting unchanged.
2. New `rules.rs` unit test: `step` matches pre-retrofit behavior for every `Direction` at interior cells, edges, and corners (including off-board → `None`).
3. New unit test: `Direction::ALL` equals `[North, Northeast, East, Southeast, South, Southwest, West, Northwest]`.
4. `cargo test -p directional_flip` (rules, property, visibility, replay, bot suites green).
5. `cargo run -p replay-check -- --game directional_flip --all` (all fourteen golden traces pass unchanged).

### Invariants

1. `games/directional_flip` depends on `game-stdlib` and no longer duplicates `board_space` coordinate identity or bounded offset stepping in the promoted scope.
2. Public cell IDs, `Direction::ALL`, flip/preview ordering, diagnostics, effects, and golden-trace hashes are byte-identical to pre-retrofit.

## Test Plan

### New/Modified Tests

1. `games/directional_flip/src/ids.rs` (inline `#[cfg(test)]`) — 64-cell round-trip + `ALL`/`index`/`rNcM` order tests.
2. `games/directional_flip/src/rules.rs` (inline `#[cfg(test)]`) — `step`-per-`Direction` interior/edge/corner equivalence + `Direction::ALL` order test.
3. Existing `games/directional_flip/tests/*.rs` — run unchanged as regression proof.

### Commands

1. `cargo test -p directional_flip`
2. `cargo run -p replay-check -- --game directional_flip --all`
3. `cargo run -p fixture-check -- --game directional_flip && cargo run -p rule-coverage -- --game directional_flip && bash scripts/boundary-check.sh && cargo clippy -p directional_flip --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-07

What changed:
- `games/directional_flip` now depends on `game-stdlib`.
- `RowId`, `ColumnId`, and `CellId` preserve their public surfaces while
  delegating bounds, indexing, canonical parsing, formatting, row-major cell
  order, and coordinate conversion to `game-stdlib::board_space`.
- `rules.rs::step` delegates one-step bounded movement to `Dimensions::offset`
  while keeping direction order, ray bracketing, flip grouping, forced pass,
  terminal checks, effects, previews, and bot policy local.
- `games/directional_flip/docs/MECHANICS.md` and
  `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` record conformance
  for the coordinate/offset subset and preserve local ray/flip policy.

Deviations from original plan:
- Kept the existing public `CellId { row, column }`, `RowId`, and `ColumnId`
  shapes instead of replacing them with raw coordinates so existing rule, view,
  replay, bot, and test surfaces stayed unchanged.

Verification results:
- `cargo fmt --all --check`
- `cargo test -p directional_flip`
- `cargo run -p replay-check -- --game directional_flip --all`
- `cargo run -p fixture-check -- --game directional_flip`
- `cargo run -p rule-coverage -- --game directional_flip`
- `bash scripts/boundary-check.sh`
- `cargo clippy -p directional_flip --all-targets -- -D warnings`
- Golden traces changed: no.
