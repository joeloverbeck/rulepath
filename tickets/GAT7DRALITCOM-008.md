# GAT7DRALITCOM-008: Semantic effects — move/capture/promotion/forced/terminal/bot/diagnostic

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/draughts_lite/src/effects.rs` (semantic effect vocabulary for one applied command), `src/lib.rs` (export); apply (GAT7DRALITCOM-007) emits these effects.
**Deps**: 007

## Problem

A single applied command emits multiple semantic effects describing every change: the move committed, each quiet/capture step, promotion, forced-capture availability, forced-continuation requirement, terminal win, bot action, and invalid-command diagnostic. These effects drive the web animation and effect log (FOUNDATIONS §7/§11 — animation is effect-driven, not state-diffed). This ticket defines the deterministic, public-safe, hash-stable effect set so a multi-jump reads as one move by one piece.

## Assumption Reassessment (2026-06-07)

1. `games/draughts_lite/src/rules.rs::apply` (GAT7DRALITCOM-007) is the producer that emits these effects; the state model (GAT7DRALITCOM-004) carries the accumulated/last effects field. `games/directional_flip/src/effects.rs` (grouped flip effect) and `games/column_four/src/effects.rs` are the structural precedents for game-prefixed effect vocabularies.
2. The required semantic coverage is fixed by spec §R10 "Effect semantics" (the event→required-information table: move committed, quiet step, capture step, promotion, forced-capture-available, forced-continuation-required, invalid selection, terminal win, bot action). Effect names may be game-prefixed and must be documented in `MECHANICS.md` (GAT7DRALITCOM-021) and reflected in golden traces (014).
3. Cross-artifact boundary under audit: the effect envelope is consumed by replay (010, effect hashes), WASM (016, effects path), and the web effect feedback (018, animation + effect log). The effect vocabulary is game-local within the generic `engine-core` effect-envelope contract.
4. FOUNDATIONS §7/§11 motivate this ticket: restate before coding — semantic effects drive animation; renderer diffs are diagnostics only. Each capture/promotion/forced-continuation event must be an explicit effect so the UI never reinterprets rules by diffing board state.
5. No-leak + determinism enforcement surface (§11): effects are emitted to browser payloads and the replay export, so confirm they expose only perfect-information facts (cells, piece ids, captured owner, public explanation) and no hidden/internal state. Effects must be deterministic and hash-stable (spec §R10 "deterministic effect hashes") — fixed ordering, no RNG, no wall-clock.

## Architecture Check

1. Emitting one explicit semantic effect per event (vs. leaving the UI to infer multi-jump structure from a before/after diff) is the §7 contract and is what makes a compound move legible and replay-stable.
2. No backwards-compatibility shims; new effect vocabulary.
3. `engine-core` stays noun-free (§3) — capture/promotion/continuation effect kinds are game-local strings within the generic effect-envelope contract; no draughts noun enters the kernel.

## Verification Layers

1. Effect coverage -> rule test: applying representative commands (quiet, single capture, multi-jump, promotion, promotion-during-capture, terminal) emits the spec §R10 effect set with required fields.
2. Determinism -> deterministic effect-hash check (in 010): identical command+state yields identical effects and effect hashes.
3. No-leak -> no-leak visibility test: effect payloads contain only viewer-safe perfect-information fields (FOUNDATIONS §11).
4. Animation source -> FOUNDATIONS alignment check: every animatable event (hop, capture, promotion, forced continuation, terminal) has a corresponding semantic effect (§7), so the web layer (018) need not diff state.

## What to Change

### 1. Effect vocabulary

In `effects.rs`, define the deterministic, public-safe effect set per spec §R10: move committed (action path, seat, moving piece id, start/final cell, move kind, path length); quiet step; capture step (hop origin/landing, captured cell + piece id + owner); promotion (piece id, seat, cell, during-capture flag); forced-capture-available; forced-continuation-required; terminal win (winner/loser/reason); bot action (level, policy version, selected path, short public rationale); invalid-command diagnostic (code, public message, rejected path when safe).

### 2. Emission from apply

Wire `apply` (GAT7DRALITCOM-007) to emit the effect sequence in deterministic order following the existing game convention (accumulate or replace per state model).

## Files to Touch

- `games/draughts_lite/src/effects.rs` (new)
- `games/draughts_lite/src/lib.rs` (modify — export the effects module)
- `games/draughts_lite/src/rules.rs` (modify — emit effects from `apply`)

## Out of Scope

- Web animation / effect-log rendering (GAT7DRALITCOM-018 — consumes these effects).
- Replay/effect-hash plumbing (GAT7DRALITCOM-010).
- Bot rationale policy itself (GAT7DRALITCOM-012; this ticket only defines the bot-action effect shape).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p draughts_lite` — effect-coverage tests pass for quiet/capture/multi-jump/promotion/terminal commands.
2. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. Every semantic change is an explicit effect; animation can be driven without state diffing (FOUNDATIONS §7; spec §R10).
2. Effects are deterministic, hash-stable, and public-safe (FOUNDATIONS §11; spec §R10).

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/tests/rules.rs` (or inline in `effects.rs`) — effect coverage per event type (cross-surface effect-hash determinism asserted in GAT7DRALITCOM-010; golden traces in 014).

### Commands

1. `cargo test -p draughts_lite effects`
2. `cargo test -p draughts_lite && bash scripts/boundary-check.sh`
3. Crate-scoped tests are correct; deterministic effect-hash proof lands in GAT7DRALITCOM-010 with replay support.
