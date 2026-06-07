# GAT7DRALITCOM-005: Rules core — movement/capture legality, mandatory capture & continuation, promotion, terminal

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/draughts_lite/src/rules.rs` (legal generation: men/king diagonal movement, adjacent-jump captures, mandatory-capture suppression of quiet moves, same-piece mandatory continuation, promotion rules, terminal detection), `src/lib.rs` (export).
**Deps**: 004

## Problem

This is the correctness heart of Draughts Lite: Rust owns all legality (FOUNDATIONS §2). The rules core computes, from a state, the legal move/capture set with deterministic canonical ordering — men move/capture diagonally forward only, kings any diagonal one square, captures are mandatory (quiet moves suppressed when any capture exists), continuation by the same piece is mandatory, promotion occurs on the far king row, a man crowned during a capture stops immediately, and terminal wins fire when the opponent has no pieces or no legal move. This ticket provides the legality primitives that the action tree (006) shapes and the validator (007) recomputes.

## Assumption Reassessment (2026-06-07)

1. `games/draughts_lite/src/state.rs` / `setup.rs` / `ids.rs` (GAT7DRALITCOM-004) supply the state model, board dimensions, parity, and stable piece ids this ticket reads. `games/directional_flip/src/rules.rs` and `games/column_four/src/rules.rs` are the structural precedents for Rust-owned legality with canonical ordering.
2. The rules are fixed by `games/draughts_lite/docs/RULES.md` (GAT7DRALITCOM-001) and spec §R8 (movement/capture/mandatory/continuation/promotion/terminal) and §R11 "Legal generation" (canonical ordering: row-major origin by cell id; forward-left before forward-right for men; documented fixed diagonal order for kings; same order for continuation). `docs/ROADMAP.md` §9 forbids "full chess exception load" and "generic movement in `engine-core`".
3. Cross-artifact boundary under audit: the legal-generation function is the single source consumed by the action tree (006) AND re-invoked by validation (007); generation and validation must call the same legality code so they cannot diverge. Continuation state (which pieces are already captured in the in-progress sequence) must be modeled without mutating real game state.
4. FOUNDATIONS §2 motivates this ticket: restate before coding — Rust owns legal action generation, scoring, and terminal detection; no legality may move to TypeScript. Mandatory capture means "if any active piece can capture, no quiet path is legal" (spec §R8 "Mandatory capture"); the generator computes global capture availability before offering quiet moves.
5. Deterministic replay/hash enforcement surface (§11): the canonical legal-generation order is load-bearing because action-tree hashes and bot traces depend on it (spec §R11). Confirm ordering is fully determined by state (no hash-map iteration, no RNG) so identical states yield identical legal sets. Coordinate handling uses the `game-stdlib` board-space helper iff GAT7DRALITCOM-002 promoted it; otherwise it is local here (the conditional fallback).

## Architecture Check

1. One legality function consumed by both generation and validation structurally prevents generate/validate drift — there is no second legality path to keep in sync.
2. No backwards-compatibility shims; new logic.
3. `engine-core` stays noun-free (§3) — movement/capture/promotion are game-local in `rules.rs`, never the kernel (`docs/ROADMAP.md` §9 "generic movement in `engine-core`" forbidden). `game-stdlib` use is limited to the rule-agnostic coordinate helper from GAT7DRALITCOM-003 if promoted (§4).

## Verification Layers

1. Men move forward only / kings any diagonal -> rule tests: a man has no backward move/capture; a king moves and captures in all four diagonals.
2. Mandatory capture -> rule test: when any capture exists, the legal set contains no quiet move; with no capture, quiet diagonal moves into empty playable cells are legal.
3. Mandatory continuation -> rule test: after a capture, if the same piece can capture again, only continuation jumps for that piece are legal; a captured piece cannot be re-captured in the same sequence.
4. Promotion-during-capture stop -> rule test: a man reaching the king row mid-capture is crowned and the sequence ends even if a king there could continue.
5. Terminal detection -> rule test: opponent-no-pieces and opponent-no-legal-move both produce a win; standard setup is non-terminal; no-legal-move for the active player is a loss (not a draw).
6. Deterministic ordering -> rule test: the legal set is row-major / documented-diagonal-order and stable across runs.

## What to Change

### 1. Move & capture generation

In `rules.rs`, generate legal quiet moves and capture sequences from a state with the canonical ordering of §R11. Compute global capture availability first; suppress quiet moves when any capture exists. Model in-sequence captured pieces as unavailable without mutating real state.

### 2. Continuation, promotion, terminal

Generate same-piece mandatory continuations; apply promotion on the far king row; stop a man's capture sequence immediately on crowning. Provide terminal-outcome detection (no pieces / no legal move → mover wins).

## Files to Touch

- `games/draughts_lite/src/rules.rs` (new)
- `games/draughts_lite/src/lib.rs` (modify — export the rules module)

## Out of Scope

- Action-tree shaping / segment vocabulary / previews (GAT7DRALITCOM-006).
- Command validation and atomic apply / diagnostics (GAT7DRALITCOM-007).
- Semantic effects (GAT7DRALITCOM-008).
- Any TypeScript-side legality (forbidden; FOUNDATIONS §2, spec §R14).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p draughts_lite` — movement/capture/mandatory/continuation/promotion/terminal rule tests pass.
2. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. All legality is Rust-computed and deterministic; mandatory capture and same-piece continuation hold (FOUNDATIONS §2; spec §R8).
2. The canonical legal-generation order is fully state-determined (no RNG, no hash-map iteration) so it is replay/hash-stable (FOUNDATIONS §11; spec §R11).

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/src/rules.rs` (inline tests) or `games/draughts_lite/tests/rules.rs` — the rule cases above (expanded into the full suite in GAT7DRALITCOM-013).

### Commands

1. `cargo test -p draughts_lite rules`
2. `cargo test -p draughts_lite && bash scripts/boundary-check.sh`
3. Crate-scoped rule tests are the correct boundary; cross-surface proof (golden traces, replay hashes) lands in GAT7DRALITCOM-013/014.
