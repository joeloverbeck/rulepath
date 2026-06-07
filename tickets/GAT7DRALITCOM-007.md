# GAT7DRALITCOM-007: Multi-segment validation & atomic apply

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/draughts_lite/src/rules.rs` (add `validate_command` + atomic `apply` + stable diagnostic codes), `src/lib.rs` (export validate/apply surface).
**Deps**: 005, 006

## Problem

A complete draughts move — including a multi-jump sequence — is committed as one command carrying a multi-segment action path. This ticket validates that a submitted path is a leaf of the current legal action tree (under the command's freshness token and actor seat) and, if valid, applies it atomically: move the piece through the path, remove captured pieces, promote on the king row, stop a man's capture sequence on crowning, advance the active seat unless terminal, update the freshness token, and evaluate the terminal outcome. Invalid paths are rejected with stable, public-safe diagnostic codes and no state mutation.

## Assumption Reassessment (2026-06-07)

1. `games/draughts_lite/src/rules.rs` (GAT7DRALITCOM-005) supplies the legality functions this ticket re-invokes for validation; `games/draughts_lite/src/actions.rs` (GAT7DRALITCOM-006) defines the leaf-path / segment contract a valid command must match. `directional_flip::validate_command` (consumed by `crates/wasm-api/src/lib.rs:497`) is the precedent signature for a game's validate entry point.
2. The required diagnostic classes are fixed by spec §R9 "Invalid path diagnostics" (stale token, non-active seat, terminal reached, empty path, malformed segment, off-board/non-playable origin/destination, no piece at origin, wrong-seat origin, occupied destination, quiet-while-capture-exists, illegal movement/capture pattern, empty jumped cell, jumped-own-piece, non-empty landing, stops-before-mandatory-continuation, switches-piece-during-continuation, re-captures-already-captured, continues-after-promotion-stop, path-not-a-tree-leaf). The apply contract is spec §R11 "Apply".
3. Cross-artifact boundary under audit: `validate_command`/`apply` are the command-processing path consumed by WASM (016), bots (012, which validate their chosen path), replay (010, which re-applies command streams), and tests (013). Validation must recompute legality from state — it must not trust client metadata or labels (spec §R11 "Validation").
4. FOUNDATIONS §2/§11 motivate this ticket: restate before coding — validation is fail-closed and blocking; an invalid path mutates nothing. Multi-jump path state is computed internally without mutating real game state until apply. The validator proves the path is a current-tree leaf under freshness token + actor seat.
5. Determinism + no-leak enforcement surface (§11): apply must be deterministic and atomic (identical command+state → identical result state, effects, and hashes), and diagnostics must be public-safe — they expose a stable code + viewer-safe message + (when safe) the rejected path, never hidden/internal state. Diagnostic codes are part of the stable contract consumed by golden traces (014).

## Architecture Check

1. Recomputing legality inside `validate_command` (rather than trusting the submitted metadata) makes the client unable to forge a legal move — the validator is the authority, the tree is a convenience for the UI.
2. No backwards-compatibility shims; new validate/apply logic.
3. `engine-core` stays noun-free (§3) — validation/apply consume the generic command-envelope + action-path contract; capture/promotion/continuation logic is game-local in `rules.rs`.

## Verification Layers

1. Leaf-only acceptance -> rule test: a path that matches a current-tree leaf validates; a path that does not (wrong segment, partial continuation, switched piece) is rejected with the matching diagnostic code.
2. Atomic apply -> rule test: a valid multi-jump applies all hops, removes all captured pieces, promotes when applicable, advances the seat (unless terminal); an invalid command leaves state byte-identical.
3. Promotion-during-capture stop -> rule test: a command with a segment after a man promotes mid-capture is rejected (`continues-after-promotion-stop`).
4. Diagnostics stable & public-safe -> golden trace (landed in 014) + no-leak test: each diagnostic emits a stable code and viewer-safe message with no hidden state.
5. Determinism -> deterministic replay-hash check (in 010): apply is a pure function of (state, command, version).

## What to Change

### 1. `validate_command`

Recompute the legal action tree (via GAT7DRALITCOM-005/006), reject on freshness-token mismatch / non-active actor / terminal state, parse and bounds-check the multi-segment path, and confirm it is a current-tree leaf; return a typed diagnostic (stable code + viewer-safe message) otherwise. Compute continuation/capture state internally without mutating real state.

### 2. Atomic `apply`

On a validated action: move the piece through the path, remove captured pieces, promote on the king row, stop on crowning-during-capture, update the freshness token, advance the active seat unless terminal, and evaluate terminal outcome. Guarantee no mutation on validation failure.

## Files to Touch

- `games/draughts_lite/src/rules.rs` (modify — add `validate_command`, `apply`, diagnostic codes)
- `games/draughts_lite/src/lib.rs` (modify — export validate/apply surface)

## Out of Scope

- Action-tree generation / previews (GAT7DRALITCOM-006; consumed here).
- Semantic effect emission (GAT7DRALITCOM-008; apply produces the data, effects format it).
- Replay/hash plumbing (GAT7DRALITCOM-010).
- WASM exposure of validate/apply (GAT7DRALITCOM-016).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p draughts_lite` — validation (each diagnostic class) + atomic-apply tests pass, including a "no mutation on invalid" assertion.
2. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. Validation is fail-closed and blocking; an invalid command mutates no state (FOUNDATIONS §2/§11; spec §R11).
2. Apply is deterministic and atomic; diagnostics are stable, public-safe codes (FOUNDATIONS §11; spec §R9).

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/tests/rules.rs` (or inline) — validation diagnostic coverage + atomic-apply assertions (expanded into the full suite in GAT7DRALITCOM-013; diagnostic golden traces in 014).

### Commands

1. `cargo test -p draughts_lite rules`
2. `cargo test -p draughts_lite && bash scripts/boundary-check.sh`
3. Crate-scoped tests are correct; deterministic replay-hash proof over applied command streams lands in GAT7DRALITCOM-010.
