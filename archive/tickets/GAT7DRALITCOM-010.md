# GAT7DRALITCOM-010: Replay support & deterministic hashes (one-command multi-segment)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/draughts_lite/src/replay_support.rs` (replay command application, deterministic state/effect/action-tree/public-view/replay hashes), `src/lib.rs` (export).
**Deps**: 007, 008

## Problem

A complete draughts move, including a multi-jump, is stored as **one** high-level replay command with a multi-segment `action_path` — not one command per jump, and not UI origin-selection events. This ticket provides the replay-support surface (reconstruct state from a seed + command stream, expose deterministic state/effect/action-tree/public-view/replay hashes) so `replay-check` (GAT7DRALITCOM-017) and golden traces (014) can prove byte-stable reproduction. Trace Schema v1 is retained — a path length > 1 needs no schema bump.

## Assumption Reassessment (2026-06-07)

1. `games/draughts_lite/src/rules.rs::{validate_command,apply}` (GAT7DRALITCOM-007) and `effects.rs` (008) are the surfaces replay re-drives. `games/directional_flip/src/replay_support.rs` exposes `replay_commands` and a `ReplayHashes` type (consumed by `tools/seed-reducer/src/main.rs:3` and `tools/trace-viewer`) — the structural precedent for this module.
2. The replay/trace contract is fixed by spec §R10 ("one high-level replay command" with multi-segment `action_path`; retain Trace Schema v1; the replay checker proves deterministic state/effect/action-tree/public-view/replay hashes + stable invalid-diagnostic handling). `crates/engine-core/src/lib.rs:59` (`ActionPath { segments: Vec<String> }`) and existing golden traces already store `action_path` as a list, so multi-segment is additive.
3. Cross-artifact boundary under audit: the replay-support surface is consumed by `tools/replay-check` (017), golden traces (014), WASM replay export (016), and `tools/simulate` (017). The hash set must be deterministic across native and WASM builds (spec §R10).
4. FOUNDATIONS §2/§11 motivate this ticket: restate before coding — replay, hashes, serialization order, RNG, and traces remain deterministic. A multi-jump must round-trip as one command preserving segment order exactly.
5. Determinism / replay-hash enforcement surface (§11, §13 ADR-trigger boundary): confirm this ticket does NOT change replay/hash *semantics* (which would require an ADR) — it retains Trace Schema v1 and only allows `action_path` length > 1, which the existing list-shaped schema already permits. Hashes must be functions of (seed, command stream, versions) with no wall-clock/RNG nondeterminism.

## Architecture Check

1. Storing a multi-jump as one command (vs. one-command-per-jump) preserves the rule meaning — one turn, one active piece, one state transition — and keeps replays legible; it reuses the existing list-shaped `action_path`, so no schema migration is needed.
2. No backwards-compatibility shims; new replay-support module.
3. `engine-core` stays noun-free (§3) — replay reuses the generic command-envelope, checkpoint, and hash contracts; segment semantics stay game-local.

## Verification Layers

1. Multi-segment round-trip -> golden trace (landed in 014) + replay test: a multi-jump command reconstructs identical state and hashes.
2. Deterministic hashes -> deterministic replay-hash check: state/effect/action-tree/public-view/replay hashes are stable across repeated runs and across native builds.
3. One-segment compatibility -> `cargo test --workspace`: existing games' replay hashes are unchanged (Trace Schema v1 retained).
4. Invalid-diagnostic stability -> replay test: invalid commands reproduce the same stable diagnostic on replay (spec §R10).

## What to Change

### 1. Replay support module

In `replay_support.rs`, reconstruct state from a seed + command stream by re-driving `validate_command`/`apply`, and expose the deterministic hash set (state, effect, action-tree, public-view, replay) following the `directional_flip` `ReplayHashes` precedent. Preserve multi-segment `action_path` order exactly.

### 2. Hash determinism

Ensure all hashes are pure functions of (seed, command stream, rules/data versions) with stable serialization order and no nondeterministic input.

## Files to Touch

- `games/draughts_lite/src/replay_support.rs` (new)
- `games/draughts_lite/src/lib.rs` (modify — export replay_support)

## Out of Scope

- The `replay-check` / `simulate` tool registration (GAT7DRALITCOM-017).
- Authoring the golden trace files (GAT7DRALITCOM-014; this ticket provides the surface they assert against).
- WASM-exported trace shape (GAT7DRALITCOM-016).
- Any Trace Schema version bump (forbidden; spec §R10 — would trip a §13 ADR trigger).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p draughts_lite` — replay round-trip + deterministic-hash tests pass for a multi-jump command.
2. `cargo test --workspace` — existing games' replay hashes unchanged (Trace Schema v1 retained).

### Invariants

1. A multi-jump is one replay command with an ordered multi-segment `action_path`; replay reproduces identical hashes (FOUNDATIONS §11; spec §R10).
2. Replay/hash semantics are unchanged for existing games — no schema bump, no §13 ADR trigger (FOUNDATIONS §11/§13).

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/tests/replay.rs` — multi-segment round-trip + deterministic hash assertions (golden trace files land in GAT7DRALITCOM-014).

### Commands

1. `cargo test -p draughts_lite replay`
2. `cargo test --workspace`
3. Workspace-wide tests are the correct boundary because the load-bearing invariant includes "existing one-segment game traces stay stable"; the full `replay-check --all` run lands with tool registration in GAT7DRALITCOM-017.

## Outcome

Implemented Draughts Lite replay support for ordered multi-segment command paths.
The module can rebuild state from seed plus command stream, project per-step board
and effect summaries, and expose deterministic state/effect/action-tree/public-view/
replay hashes plus stable invalid/stale diagnostic hashes without changing Trace
Schema v1.

Verification passed:

1. `cargo test -p draughts_lite replay`
2. `cargo test -p draughts_lite`
3. `cargo fmt --all --check`
4. `cargo test --workspace`
