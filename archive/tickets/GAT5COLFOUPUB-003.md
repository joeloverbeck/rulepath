# GAT5COLFOUPUB-003: Column Four rules core — gravity, legality, terminal, diagnostics

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/column_four/src/actions.rs`, `games/column_four/src/rules.rs`, `games/column_four/src/state.rs`
**Deps**: 002

## Problem

The core of `column_four` is Rust-owned game behavior: legal-column generation, gravity/landing, occupancy mutation, win detection in four directions, draw detection, win-precedence, terminal no-actions, and public diagnostics. This is the authoritative rule engine every other surface (view, effects, replay, bots, WASM, web) depends on (spec §7 rules model, §4 Rust authority).

## Assumption Reassessment (2026-06-06)

1. `games/three_marks/src/{actions.rs,rules.rs,state.rs}` is the template: local action parsing, legal-target generation, occupancy mutation, line scan, winner/draw/terminal resolution, and `Diagnostic`-returning validation. Verified the modules exist; this ticket mirrors the shape for a 7×6 board with gravity and 4-in-a-row.
2. Spec §7 (rules model) and §8.3 (diagnostics) define behavior: legal action = non-full column on active non-terminal turn; gravity lands at lowest empty row; win = four contiguous same-seat H/V/both-diagonal; draw = full board, no line; win precedes draw; required diagnostics `stale_action`, `not_active_seat`, `invalid_action_path`, `unknown_column`, `full_column`, `terminal_match` (spec §8.3). Coordinate/seat types come from GAT5COLFOUPUB-002 (`src/ids.rs`).
3. Cross-artifact boundary under audit: the `engine-core` action-tree / command-envelope / `Diagnostic` contracts (`docs/ARCHITECTURE.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md`). This ticket implements a game-local rule engine that produces those generic envelopes; it adds no mechanic noun to `engine-core`.
4. FOUNDATIONS §2 (behavior authority) motivates this ticket: setup, legal-action generation, validation, state transitions, and terminal detection MUST be Rust-owned and deterministic; TypeScript never computes legality, landing, or terminal state. Restating before trusting the spec narrative: full columns must be absent from the legal action tree, not merely rejected after click.

## Architecture Check

1. A single game-local rule module owning gravity + line detection keeps the kernel generic and the rules deterministic and testable in one place — cleaner than splitting legality between Rust and a UI helper. Alternative (TS-side full-column/landing inference) is a §12 stop condition and is rejected.
2. No backwards-compatibility aliasing/shims — new logic on the 002 skeleton.
3. `engine-core` stays free of `board`/`grid`/`cell`/`line`/`gravity`/`column` nouns (all local to `games/column_four`); `game-stdlib` untouched — line/gravity duplication vs. `three_marks` is recorded as pressure in GAT5COLFOUPUB-018, not extracted (spec §16, FOUNDATIONS §4).

## Verification Layers

1. Legal-action invariant -> unit/rule test: only non-full columns appear in the active actor's action tree; full columns absent (not post-hoc rejected).
2. Gravity invariant -> unit test: a placement lands at the lowest empty row of the chosen column for representative occupancy states.
3. Win-detection invariant -> rule tests for horizontal, vertical, and both diagonal directions, plus the documented tie-break when multiple lines complete.
4. Draw / win-precedence invariant -> rule test: full board with no line is draw; a final move that fills the board and completes a line is a win.
5. Diagnostic invariant -> unit test: each of `stale_action`, `not_active_seat`, `invalid_action_path`, `unknown_column`, `full_column`, `terminal_match` is produced for its trigger and is public/viewer-safe.
6. Determinism invariant -> FOUNDATIONS alignment check (§2): identical command sequences produce identical state/terminal outcome.

## What to Change

### 1. `games/column_four/src/actions.rs`

Action parsing/validation: parse a one-segment column action path; validate freshness token, active seat, path shape, known column, non-full column, non-terminal match; return the appropriate `Diagnostic` on each failure (spec §8.3 codes).

### 2. `games/column_four/src/rules.rs`

Legal-column generation (active actor, non-terminal); gravity landing-row computation; occupancy mutation; win scan over horizontal/vertical/both diagonals returning the winning seat + ordered winning-line cell ids with a deterministic tie-break; draw detection (full board, no line); terminal resolution with win-precedence and no-legal-actions-after-terminal.

### 3. `games/column_four/src/state.rs`

Extend state with the apply-action transition (place → land → mutate → resolve terminal → advance seat when non-terminal), preserving the winning line in state for view/effects/replay.

## Files to Touch

- `games/column_four/src/actions.rs` (new)
- `games/column_four/src/rules.rs` (new)
- `games/column_four/src/state.rs` (modify)

## Out of Scope

- Public-view projection (GAT5COLFOUPUB-004) and effects (GAT5COLFOUPUB-005).
- Replay support (006), bots (008), and the comprehensive test suite (009) — this ticket carries only the focused unit/rule tests proving its own invariants.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p column_four rules` — gravity, legal-column, win (4 directions), draw, win-precedence, and terminal tests pass.
2. `cargo test -p column_four actions` — each diagnostic code is produced for its trigger.
3. `cargo build --workspace` — no regression to existing crates.

### Invariants

1. Legality, gravity, winner/draw, and terminal detection are computed in Rust; full columns never appear as legal choices.
2. Multiple-line completion resolves to one documented deterministic winning line.

## Test Plan

### New/Modified Tests

1. `games/column_four/src/rules.rs` (unit tests) — gravity landing, horizontal/vertical/rising/falling diagonal wins, full-board draw, win-precedence-over-draw, terminal no-actions.
2. `games/column_four/src/actions.rs` (unit tests) — stale, not-active-seat, invalid-path, unknown-column, full-column, terminal-match diagnostics.

### Commands

1. `cargo test -p column_four`
2. `cargo build --workspace`
3. `cargo clippy -p column_four --all-targets -- -D warnings` — lint the new behavior surface.

## Outcome

Completed: 2026-06-06

What changed:

- Added `games/column_four/src/actions.rs` with Rust-owned legal action tree generation for active actors, one legal action per non-full column, column action parsing, and actor-seat mapping.
- Added `games/column_four/src/rules.rs` with command validation, required public diagnostic codes, gravity landing, occupancy mutation, terminal resolution, legal-column calculation, horizontal/vertical/rising-diagonal/falling-diagonal win detection, draw detection, win-precedence, and deterministic multi-line tie-break behavior.
- Extended `games/column_four/src/state.rs` with `WinningLine`, `TerminalOutcome`, state occupancy mutation helpers, terminal storage, and stable snapshot summaries that include terminal state.
- Added focused unit coverage for legal-column filtering, inactive actor legal-tree emptiness, stale/not-active/invalid-path/unknown-column/full-column/terminal diagnostics, gravity landing, all four win directions, deterministic multi-line tie-break, draw, win-over-draw precedence, and terminal no-actions.

Deviations from original plan:

- Semantic effects were intentionally not added to `apply_action`; they remain in GAT5COLFOUPUB-005. This ticket stores the terminal outcome and winning line needed by later view/effect/replay tickets.

Verification results:

- Passed: `cargo test -p column_four rules`
- Passed: `cargo test -p column_four actions`
- Passed: `cargo test -p column_four`
- Passed: `cargo clippy -p column_four --all-targets -- -D warnings`
- Passed: `cargo fmt --all --check`
- Passed: `cargo build --workspace`
- Passed: `bash scripts/boundary-check.sh`
