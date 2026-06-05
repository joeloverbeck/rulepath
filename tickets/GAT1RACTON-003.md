# GAT1RACTON-003: engine-core game-entry, effect-log, hash & replay/serialization contracts

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/engine-core` gains a generic game-entry contract over an opaque payload, an effect cursor/log contract, a state/effect/action-tree/view hash contract, and the replay/command-stream + serialization boundary.
**Deps**: GAT1RACTON-002

## Problem

The apply/observe/replay half of the kernel contract: a generic **game-entry**
trait the engine drives over an opaque game-defined payload (FOUNDATIONS §3
decision rule), an **effect cursor/log** for incremental viewer-filtered effect
delivery (ARCHITECTURE §7, §10 `get_effects(since_cursor)`), a **hash contract**
covering state/effect/action-tree/view (ARCHITECTURE §8), and the
**replay/command-stream + serialization boundary** (ARCHITECTURE §8). These
complete the generic surface `race_to_n` exercises. All noun-free.

## Assumption Reassessment (2026-06-05)

1. After GAT1RACTON-002, `engine-core` exposes the action-tree, freshness-token,
   and RNG contracts; it still has `EffectEnvelope<T>`, `Diagnostic`,
   `CommandEnvelope`, `VisibilityScope`, `RulesVersion`/`SchemaVersion`/`Seed`
   (verified in `crates/engine-core/src/lib.rs`). No effect cursor/log, hash
   contract, game-entry trait, or replay/serialization boundary exists yet — all
   additive.
2. ARCHITECTURE §7 (semantic effect log: deterministic, ordered, replayable,
   hashable, viewer-filtered), §8 (replay SHOULD include state/effect/
   legal-action-tree/public-view hashes; identical inputs reproduce identical
   hashes), and §10 (`get_effects(match, since_cursor, viewer)`) define the
   contract shapes consumed here. FOUNDATIONS §3 lists `effect envelope`,
   `replay`, `checkpoint`, `hash`, and serialization boundary as allowed kernel
   nouns.
3. Shared boundary under audit: the game-entry trait is the seam between the
   generic engine and `games/race_to_n` (GAT1RACTON-004 implements it). It must
   carry the opaque payload as an associated type / generic, never a mechanic
   type. Consumers: `games/*` (implementors), `wasm-api` (driver),
   `ai-core` (reads action trees/views).
4. FOUNDATIONS §3 (contract kernel) and §11 (replay/hashes/serialization order
   remain deterministic) motivate this ticket. Kernel-change protocol
   (AGENT-DISCIPLINE §5) answered as in GAT1RACTON-002: required by `race_to_n`,
   generic-only, no game noun, determinism-preserving, instantiates §3
   vocabulary (no ADR — these are not a replay/hash *semantics change*, they are
   the first definition of the contract).
5. Determinism + serialization enforcement surface: the hash contract and
   serialization boundary defined here are exactly the surfaces GAT1RACTON-008
   (replay/hash tests) and GAT1RACTON-006 (game serialization) enforce. Confirm
   the contract mandates stable serialization order for hashing (ARCHITECTURE §9;
   FOUNDATIONS §11) and unknown-field rejection for hand-authored data
   (FOUNDATIONS §11) — the kernel defines the rule; games enforce it. No hidden
   information enters hashes by construction (hashes are over canonical state +
   viewer-scoped views).
6. Schema/contract extension: this extends the kernel contract set consumed by
   `games/*` and `wasm-api`. There are no existing consumers yet (greenfield game
   tree), so the extension is additive with no breakage; future consumers are the
   sequenced Gate 1 tickets.

## Architecture Check

1. A generic game-entry trait with an associated opaque payload type lets the
   engine run setup → legal-action-tree → validate → apply → project-view →
   replay (ARCHITECTURE §4) without knowing any mechanic — the cleanest
   expression of FOUNDATIONS §3's "knows a payload exists, not what it means".
   Alternative (concrete dispatch per game in the kernel) leaks game identity.
2. No backwards-compatibility shims — first definition of these contracts.
3. `engine-core` stays noun-free: effect cursor/log, hash, replay,
   serialization boundary are §3-allowed contract nouns; payloads are opaque.
   `game-stdlib` untouched.

## Verification Layers

1. Kernel noun-freeness -> codebase grep-proof (mechanic-noun grep over
   `crates/engine-core/src` clean; `scripts/boundary-check.sh` passes).
2. Hash/serialization determinism -> deterministic replay-hash check (unit test:
   stable serialization order; identical canonical input → identical hash).
3. Effect-cursor contract -> schema/serialization validation (cursor advances
   monotonically; effect log is ordered + viewer-filterable per ARCHITECTURE §7).
4. Game-entry opacity -> FOUNDATIONS alignment check (§3: associated payload type
   is opaque; trait signature names no mechanic).

## What to Change

### 1. Game-entry contract

Add a generic `Game`-style trait (e.g. `engine_core::Game`) with associated
types for the opaque state payload, action representation, and effect payload,
and methods mirroring ARCHITECTURE §4 (setup, legal action tree, validate, apply,
project view, replay). Mechanic-free signatures only.

### 2. Effect cursor/log contract

Add an effect cursor type and an ordered effect-log contract over the existing
`EffectEnvelope<T>` supporting `since_cursor` incremental reads (ARCHITECTURE
§7/§10), viewer-filtered.

### 3. Hash contract

Add a hash contract covering state, effect, action-tree, and public-view hashes
(ARCHITECTURE §8), built on stable serialization order (ARCHITECTURE §9).

### 4. Replay / command-stream + serialization boundary

Add the replay record contract (game id, rules/engine/data/schema version, seed,
seats, options, ordered command stream, optional checkpoints, hashes —
ARCHITECTURE §8) and the serialization boundary contract, mandating stable order
and unknown-field rejection for hand-authored data (FOUNDATIONS §11).

## Files to Touch

- `crates/engine-core/src/lib.rs` (modify)
- `crates/engine-core/src/game.rs` (new, optional — game-entry trait)
- `crates/engine-core/src/replay.rs` (new, optional — replay/hash/serialization)

## Out of Scope

- Action tree, freshness token, RNG (GAT1RACTON-002).
- Any `race_to_n` implementation of these contracts (GAT1RACTON-004+).
- The `wasm-api` driver that calls them (GAT1RACTON-011).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p engine-core` — game-entry trait object/impl smoke test, effect-cursor monotonicity, and hash-stability unit tests pass.
2. `cargo clippy -p engine-core --all-targets -- -D warnings` — clean.
3. `bash scripts/boundary-check.sh` — boundary check passes.

### Invariants

1. The game-entry trait names no mechanic noun; payload types are opaque (FOUNDATIONS §3).
2. Identical canonical state serializes to identical bytes → identical hash (ARCHITECTURE §8/§9; FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `crates/engine-core/src/replay.rs` (or `lib.rs`) `#[cfg(test)]` — stable serialization → stable hash for a fixture payload.
2. `crates/engine-core/src/game.rs` (or `lib.rs`) `#[cfg(test)]` — a trivial in-test `Game` impl drives setup→apply→project without the kernel naming a mechanic.

### Commands

1. `cargo test -p engine-core`
2. `cargo build --workspace && cargo clippy --workspace --all-targets -- -D warnings`
3. `grep -rniE 'board|deck|pile|card|track|resource|suit|hand' crates/engine-core/src` — expect zero hits (kernel-boundary proof).
