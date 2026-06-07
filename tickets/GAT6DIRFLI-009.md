# GAT6DIRFLI-009: Replay support & deterministic hashes

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/directional_flip/src/replay_support.rs` (replay command/effect/view serialization, stable hashes, import/export/step/reset support).
**Deps**: 007, 008

## Problem

Replay must reconstruct placements, flips, pass actions, terminal state, effects, views, and stable hashes deterministically across export/import/step/reset (FOUNDATIONS §2/§11, spec §7/§14 "replay reconstructs consequences", `DF-REPLAY-001`). This ticket adds the Rust-owned replay projection for `directional_flip`, mirroring `games/column_four/src/replay_support.rs`.

## Assumption Reassessment (2026-06-07)

1. `games/column_four/src/replay_support.rs` is the structural precedent. The command/effect/view surfaces it serializes now exist for `directional_flip`: `rules.rs` (apply/pass transitions, 005), `effects.rs` (008), `visibility.rs` (007). `wasm-api` exposes a `*_TRACE_RULES_VERSION` constant per game (`crates/wasm-api/src/lib.rs` lines 55–57) — a `directional_flip-rules-v1` analog is added in GAT6DIRFLI-015, not here.
2. Spec §5.1 (replay/hash), §14 ("replay reconstructs consequences"), `docs/TRACE-SCHEMA-v1.md`, and `docs/TESTING-REPLAY-BENCHMARKING.md` are authoritative. Rule id `DF-REPLAY-001` (deterministic across export/import/step/reset).
3. Cross-artifact boundary under audit: `games/directional_flip` ↔ the `engine-core` replay/checkpoint/hash contract, and ↔ `tools/replay-check` (which discovers and verifies golden traces, GAT6DIRFLI-013/016). Confirm the checkpoint/hash field names against `docs/ENGINE-GAME-DATA-BOUNDARY.md` and the engine-core replay module.
4. FOUNDATIONS §2 (Rust owns replay/hash behavior) and §11 (replay/hashes/serialization order deterministic) motivate this ticket: restate before coding — replay is Rust-owned and byte-deterministic; the browser may preview locally but owns no authoritative state (§9).
5. This is the **deterministic replay/hash & serialization** surface (FOUNDATIONS §11; a *change* to replay/hash semantics would trip the §13 ADR trigger — but `directional_flip` is a new game establishing its own first hashes, so no existing semantics change). Confirm: serialization order is stable (sorted/insertion-ordered, not hash-map iteration); no wall-clock/RNG enters canonical forms; the same trace replays to identical hashes. No hidden information enters replay exports (§11 no-leak firewall) — the export carries only viewer-safe views/effects.

## Architecture Check

1. A dedicated `replay_support.rs` that reuses the effect (008) and view (007) projections (rather than a parallel serialization) keeps the replayed consequences identical to live play — the determinism the gate's exit criteria require.
2. No backwards-compatibility shims; new replay surface for a new game.
3. `engine-core` stays noun-free — replay payloads are game-local within the generic replay/hash contract; no mechanic noun enters the kernel (§3).

## Verification Layers

1. Deterministic replay/hash -> golden trace / deterministic replay-hash check (`DF-REPLAY-001`): export → import → step → reset reproduces identical state, effects, views, and hashes (per `docs/TESTING-REPLAY-BENCHMARKING.md`).
2. Stable serialization order -> schema/serialization validation: trace serialization is byte-stable; no hash-map iteration leakage; conforms to `docs/TRACE-SCHEMA-v1.md`.
3. Replay-export no-leak -> no-leak visibility test: exported replay carries only viewer-safe views/effects (FOUNDATIONS §11).

## What to Change

### 1. Replay projection & hashes

In `replay_support.rs`, implement Rust-owned serialization of commands, effects, and views for `directional_flip`, with stable hashing and import/export/step/reset support, conforming to `docs/TRACE-SCHEMA-v1.md`. Reuse the GAT6DIRFLI-007/008 projections so replayed output matches live output exactly.

## Files to Touch

- `games/directional_flip/src/replay_support.rs` (new)
- `games/directional_flip/src/lib.rs` (modify — export the replay-support module)

## Out of Scope

- Golden trace fixtures themselves (GAT6DIRFLI-013) — this ticket provides the support they exercise.
- `tools/replay-check` registration (GAT6DIRFLI-016) and `wasm-api` replay export/import wiring (015).
- The `directional_flip-rules-v1` trace-version constant in `wasm-api` (GAT6DIRFLI-015).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p directional_flip` — replay round-trip + hash-stability tests pass.
2. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. Export/import/step/reset is byte-deterministic and reproduces identical hashes (FOUNDATIONS §11, `DF-REPLAY-001`).
2. Replay exports leak no hidden information (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `games/directional_flip/tests/replay.rs` — replay/hash determinism across export/import/step/reset (expanded in GAT6DIRFLI-012; golden-trace corpus in 013).

### Commands

1. `cargo test -p directional_flip replay`
2. `cargo test -p directional_flip && bash scripts/boundary-check.sh`
3. Crate-scoped tests are correct here; the full `cargo run -p replay-check -- --game directional_flip --all` gate runs in GAT6DIRFLI-016 once the tool is registered and golden traces (013) exist.
