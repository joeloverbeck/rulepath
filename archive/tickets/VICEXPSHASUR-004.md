# VICEXPSHASUR-004: Line-result outcome rationale — `three_marks` + `column_four`

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `games/three_marks` and `games/column_four` (`visibility.rs` rationale projection), per-game docs, and golden traces. No `engine-core`/`game-stdlib` change.
**Deps**: VICEXPSHASUR-003

## Problem

`three_marks` and `column_four` already compute the decisive winning line, but it is not projected through the shared outcome rationale shape, so the web UI cannot explain "won by completing this line" consistently. These two line games share an almost identical retrofit (the decisive line already lives in `TerminalView`), so they batch into one reviewable diff that follows the pilot (003) house style. Source: `archive/specs/victory-explanation-shared-surface.md` §9.2, §9.3.

## Assumption Reassessment (2026-06-09)

1. Verified current code: `three_marks` `TerminalView::Win { winning_seat, line: [CellId; 3] }` + `Draw` (`games/three_marks/src/visibility.rs:52-59`); effects `LineCompleted`/`GameEnded` (`effects.rs`). `column_four` `TerminalView::Win { winning_seat, line: [CellId; 4] }` (`games/column_four/src/visibility.rs:65-72`). Field is `winning_seat` (not `seat`) in both. The decisive line is already present — this projects it into the rationale shape (the lightest retrofit), it does not recompute it.
2. Spec §9.2/§9.3. Rule IDs use `TM-`/`CF-` prefixes (`games/three_marks/docs/RULES.md`, `games/column_four/docs/RULES.md`); the rationale cites line-completion and draw/full-board terminal IDs.
3. Cross-artifact boundary under audit: the rationale is projected through each game's `TerminalView` (`visibility.rs`), round-trips `games/<g>/tests/golden_traces`, and is consumed by `apps/web/src/wasm/client.ts` (010).
4. FOUNDATIONS §2 restated: Rust projects the line cells (and orientation, where derivable); TypeScript renders them. TS MUST NOT compute the winning line (no `findWinningLine` equivalent).
5. §11 determinism: both games are fully public (no hidden information), so no-leak is trivially satisfied; the new rationale fields must serialize deterministically (golden-trace migration), no nondeterministic input in canonical form.
6. Schema extension: this adds a rationale field to each game's `TerminalView` public-view schema. Consumers: golden traces (this ticket) + `client.ts` (010). Additive — no existing variant removed.

## Architecture Check

1. Reusing the existing `line` data and adding a game-local rationale (spec D2/D3) keeps the cause as deterministic view data; batching the two near-identical line games into one diff is more reviewable than two trivially-duplicated tickets, and they share no file so there is no merge hazard.
2. No backwards-compatibility shims: additive rationale projection; the existing `Win { winning_seat, line }` variants are reused, not aliased.
3. `engine-core` stays free of mechanic nouns (`line`/`cell` stay in the games); `game-stdlib` unchanged.

## Verification Layers

1. Line cause projected → grep-proof each game's `visibility.rs` carries the rationale; unit test asserts the win rationale names the ordered line cells.
2. Draw path → test: board-full-no-line produces the draw rationale (`*.full_board_draw` template key).
3. Determinism → golden trace / replay-hash check for both games (`docs/TESTING-REPLAY-BENCHMARKING.md`).
4. §2 behavior authority → manual/grep review: no TS line computation introduced (Rust + docs + traces only).

## What to Change

### 1. `three_marks` rationale (`games/three_marks/src/visibility.rs`)

Project a rationale on `TerminalView`: win = line completed (ordered cell IDs, orientation if Rust-derivable, final ply); draw = board full with no completed line (board-full flag); `decisive_rule_ids` = `TM-` line-completion/draw IDs; template keys `three_marks.line_completed`, `three_marks.full_board_draw`. Add the "Outcome / victory explanation" section to `games/three_marks/docs/UI.md` per the 001 template; ensure `RULES.md` exposes the cited IDs.

### 2. `column_four` rationale (`games/column_four/src/visibility.rs`)

Same shape over `[CellId; 4]`: win = four connected cells (ordered, orientation if derivable); draw = full-board no line; `decisive_rule_ids` = `CF-` four-in-a-row/full-board IDs; template keys `column_four.line_completed`, `column_four.full_board_draw`. Add the `UI.md` outcome section; ensure `RULES.md` IDs.

### 3. Tests + golden traces

Add win/draw rationale unit tests for both games; intentionally regenerate the affected `games/three_marks/tests/golden_traces` and `games/column_four/tests/golden_traces` entries.

## Files to Touch

- `games/three_marks/src/visibility.rs` (modify)
- `games/three_marks/docs/UI.md` (modify)
- `games/three_marks/docs/RULES.md` (modify)
- `games/three_marks/tests/golden_traces/` (modify — intentional regeneration)
- `games/column_four/src/visibility.rs` (modify)
- `games/column_four/docs/UI.md` (modify)
- `games/column_four/docs/RULES.md` (modify)
- `games/column_four/tests/golden_traces/` (modify — intentional regeneration)

## Out of Scope

- Any other game's rationale (003, 005–008).
- The shared panel, templates file, `client.ts` types, or board wiring (009–010).
- The browser smoke (011).
- Introducing TS line-computation logic or an `engine-core`/`game-stdlib` outcome type.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p three_marks` and `cargo test -p column_four` pass, including new win/draw rationale tests.
2. The win rationale for each game names the ordered decisive line cells; the draw rationale names the full-board-no-line cause.
3. `cargo run -p replay-check -- --game three_marks --all` and `cargo run -p replay-check -- --game column_four --all` pass.

### Invariants

1. The decisive line is Rust-projected view data; no downstream code recomputes it (FOUNDATIONS §2).
2. New rationale fields serialize deterministically; regenerated traces are replay-stable (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `games/three_marks/` + `games/column_four/` visibility/rationale unit tests (win line, full-board draw).
2. `games/three_marks/tests/golden_traces/` + `games/column_four/tests/golden_traces/` — intentionally regenerated terminal traces.

### Commands

1. `cargo test -p three_marks && cargo test -p column_four`
2. `cargo run -p replay-check -- --game three_marks --all && cargo run -p replay-check -- --game column_four --all`
3. `cargo run -p fixture-check -- --game three_marks && cargo run -p fixture-check -- --game column_four`
