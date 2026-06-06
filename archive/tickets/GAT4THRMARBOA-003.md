# GAT4THRMARBOA-003: Three Marks actions, rules, win/draw/terminal detection + rule/property tests

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new `games/three_marks/src/actions.rs`, `src/rules.rs`, application path in `src/lib.rs`; new `tests/rule_tests.rs`, `tests/property_tests.rs`
**Deps**: GAT4THRMARBOA-002

## Problem

Three Marks must own all rule behaviour in Rust: generate one legal placement action per empty cell for the active seat, validate submissions (actor, cell existence/emptiness, active seat, non-terminal, freshness), apply legal placements, and detect row/column/diagonal wins and full-board draws — reporting the winning seat and exact line cells. This is the legality core every other surface (effects, view, bots, replay, WASM, UI) consumes. Without it, TypeScript would be tempted to infer legality, violating FOUNDATIONS §2.

## Assumption Reassessment (2026-06-06)

1. `games/race_to_n/src/actions.rs` and `rules.rs` are the mirror: `race_to_n` exposes `validate_command` (used at `crates/wasm-api/src/lib.rs:191` and `tools/simulate`) returning a `Diagnostic` on rejection, and a flat action tree built from the generic `engine-core` action-tree/action-path/command-envelope contracts. `three_marks` mirrors this with placement actions. Verified `validate_command` is the cross-boundary entry point.
2. Spec `specs/gate-4-three-marks-board-smoke.md` §9 (flat targeted action model, path `place/<cell>`, freshness token, occupied-cell rejection, terminal action tree), §8.1 (action generation/validation/win-line/draw rows), §15.1 (rule tests), §15.2 (property/invariant tests). State scaffold from GAT4THRMARBOA-002 provides board occupancy/active seat/terminal fields.
3. Cross-crate boundary under audit: the `engine-core` action-tree / action-path / command-envelope / diagnostic contracts (FOUNDATIONS §3 generic nouns) — `three_marks` builds typed placement actions on top without adding board nouns to the kernel.
4. FOUNDATIONS §2 (Rust owns legal-action generation, validation, transitions, terminal detection) and §11 (validation fail-closed and blocking) motivate this ticket: legal actions are exactly the empty cells for the active seat before terminal; occupied/invalid/stale/wrong-actor/terminal submissions are rejected by Rust with safe diagnostics, never silently mutated.
5. Fail-closed validation enforcement surface (§11): `validate_command` (or equivalent) is the blocking gate — name it explicitly. It MUST reject unknown/occupied/stale/invalid-cell submissions deterministically and leave state unmutated; warnings are not conflated with blockers. Perfect-information game → diagnostics carry no hidden state (no leak path).
6. Extends the action-tree / command-envelope contract with game-specific placement actions (path `place/<cell>`). Consumers of the action tree are `crates/wasm-api` get-action-tree (GAT4THRMARBOA-009) and bots (006); the shape is additive game-specific JSON behind the generic contract — stable, unique action ids across equivalent states (spec §9 tests).

## Architecture Check

1. A flat targeted action tree (`place/r1c1` …) with Rust-side validation is the cleanest mapping for a fixed-position placement game and matches the established `race_to_n` legal-action API, so the WASM bridge and UI reuse the same dispatch path. Alternative (compound/progressive action tree) is unjustified for single-cell placement and rejected by spec §9.
2. No backwards-compatibility aliasing/shims — new modules; `race_to_n` untouched.
3. `engine-core` gains no board/line/pattern nouns — row/column/diagonal scanning lives only in `games/three_marks/src/rules.rs`; no `game-stdlib` extraction (first use, local-only).

## Verification Layers

1. Legal-actions-are-empty-cells invariant -> rule test (`tests/rule_tests.rs`: nine legal actions at setup; one per empty cell thereafter; none after terminal).
2. Occupied/invalid/stale/wrong-actor/terminal rejection invariant -> fail-closed validation check (`tests/rule_tests.rs`: each rejection returns a diagnostic and leaves state unmutated).
3. Win/draw correctness invariant -> rule test (row, column, main diagonal, anti-diagonal win with correct winner + exact line cells; full-board draw; no moves after terminal).
4. Structural invariants (no legal occupied target, mark-count/alternation validity, single mark per cell, bounded termination ≤9 plies, stable & unique action ids) -> property test (`tests/property_tests.rs`).

## What to Change

### 1. `src/actions.rs`

Generate the flat action tree for the active seat: one placement action per empty cell before terminal, none after. Stable path segments (`place` + cell id, e.g. `place/r1c1`), Rust-provided labels/short-labels/accessibility-labels/target metadata, and the engine-provided freshness token. Equivalent states yield identical, unique action ids.

### 2. `src/rules.rs`

Validation entry point (mirror `race_to_n::validate_command`): check actor, action path/cell existence, cell emptiness, active seat, non-terminal state, freshness token — return a `Diagnostic` on any failure with no state mutation. Win-line detection: after placement, scan rows/columns/both diagonals; on completion report winning seat + exact ordered line cells. Draw detection: full board with no winning line. Apply path (in `lib.rs` or `rules.rs`): apply a legal placement, update occupancy/active seat/turn count/terminal outcome.

### 3. `tests/rule_tests.rs` and `tests/property_tests.rs`

All §15.1 rule tests (initial board empty, correct active player, legal moves exactly empty cells, occupied illegal, alternating turns, row/column/main-diagonal/anti-diagonal win, draw on full board, no moves after terminal, invalid cell rejected, stale action rejected) and all §15.2 property/invariant tests (no legal occupied target, mark counts valid, active alternates, terminal matches pattern, single mark per cell, bounded termination, stable unique actions).

## Files to Touch

- `games/three_marks/src/actions.rs` (new)
- `games/three_marks/src/rules.rs` (new)
- `games/three_marks/src/lib.rs` (modify)
- `games/three_marks/tests/rule_tests.rs` (new)
- `games/three_marks/tests/property_tests.rs` (new)

## Out of Scope

- Semantic effects (GAT4THRMARBOA-004), view projection (005), bots (006), replay/hash & serialization round-trip (007).
- Any board/line noun in `engine-core` or helper extraction to `game-stdlib` (forbidden, spec §4/§17).
- Golden traces (land in 007).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p three_marks --test rule_tests` — all §15.1 rule tests pass.
2. `cargo test -p three_marks --test property_tests` — all §15.2 invariants hold.
3. `cargo test -p three_marks` — full crate test suite green; `bash scripts/boundary-check.sh` clean.

### Invariants

1. For any non-terminal legal state, the action tree's targets are exactly the empty cells for the active seat; terminal states expose no normal placement actions.
2. Every occupied/invalid/stale/wrong-actor/terminal submission is rejected by Rust with a diagnostic and leaves state byte-unchanged; legal action ids are stable and unique across equivalent states.

## Test Plan

### New/Modified Tests

1. `games/three_marks/tests/rule_tests.rs` — named rule tests for legality, rejection, win (4 line kinds), draw, terminal.
2. `games/three_marks/tests/property_tests.rs` — invariant/property coverage including bounded termination and stable-unique action ids.

### Commands

1. `cargo test -p three_marks --test rule_tests --test property_tests`
2. `cargo test -p three_marks && bash scripts/boundary-check.sh`
3. WASM/CLI end-to-end exercise is deferred (009/014); crate-level rule+property tests are the correct verification boundary for the rule-logic diff.

## Outcome

Completed: 2026-06-06

What changed:

- Added `games/three_marks/src/actions.rs` with flat Rust-generated placement actions, one `place/<cell>` choice per empty cell for the active actor, stable cell metadata, and terminal/wrong-actor empty action trees.
- Added `games/three_marks/src/rules.rs` with fail-closed validation for terminal, stale, unknown actor, wrong actor, invalid path/cell, and occupied-cell submissions; legal placement application; turn alternation; ply/freshness advancement; row, column, main diagonal, anti-diagonal win detection; full-board draw detection; and exact winning-line reporting.
- Wired `ThreeMarks` through the generic `engine_core::Game` trait for setup, legal action tree, validation, and apply. Effects and view projection remain placeholders for later tickets.
- Added rule and property tests for legal action generation, rejection/no-mutation behavior, turn advancement, row/column/diagonal wins, draw, terminal no-actions, game-trait wiring, stable unique action IDs, mark-count bounds, and bounded termination.

Deviations from original plan:

- Semantic effects remain out of scope and return an empty effect list until GAT4THRMARBOA-004.
- View projection remains out of scope and uses the unit view placeholder until GAT4THRMARBOA-005.

Verification results:

- `cargo fmt --all --check`
- `cargo test -p three_marks --test rule_tests --test property_tests`
- `cargo test -p three_marks`
- `bash scripts/boundary-check.sh`
