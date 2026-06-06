# GAT5COLFOUPUB-005: Column Four semantic effects & effect log

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/column_four/src/effects.rs`
**Deps**: 003

## Problem

The web shell drives animation and comprehension from Rust-emitted semantic effects, not guessed state diffs. `column_four` needs ordered, deterministic, viewer-safe effects for drop accepted, piece landed, turn advanced, win detected, draw detected, and bot chose action (spec §8.4, FOUNDATIONS §7/§11 effects-drive-animation).

## Assumption Reassessment (2026-06-06)

1. `games/three_marks/src/effects.rs` is the template: a typed effect enum recorded into an `engine_core::EffectLog`, viewer-safe, ordered, replay-stable. Verified the module exists and `engine-core` provides the generic `EffectLog`/effect-envelope contract. This ticket mirrors it for column drops.
2. Spec §8.4 (effect model) defines required effects: drop/place accepted; piece landed (actor seat, column, row/cell id, from/to display anchors); turn advanced (non-terminal); win detected (winner + line ids); draw detected; bot chose action (public rationale). Rule transition and winning-line data come from GAT5COLFOUPUB-003; bot rationale payload is consumed by GAT5COLFOUPUB-008.
3. Cross-artifact boundary under audit: the `engine-core` effect-envelope contract and the no-leak firewall (`docs/WASM-CLIENT-BOUNDARY.md`). Effects carry only public facts; no candidate ranking, score array, or internal state enters an effect.
4. FOUNDATIONS §11 (semantic effects drive animation; renderer diffs are diagnostics only) and the no-leak invariant motivate this ticket. Restating: the effect log explains what happened in public terms; it is not the authoritative game state (the public view is), and it must never leak bot internals.

## Architecture Check

1. A typed semantic effect enum (landing anchors derived from Rust, not TS gravity) is the only design that keeps animation driven by Rust causality — cleaner and FOUNDATIONS-compliant versus renderer-diff guessing (a §12 stop condition).
2. No backwards-compatibility aliasing/shims — new module on the 003 transition.
3. `engine-core` stays free of mechanic nouns (effect variants are game-local, recorded via the generic `EffectLog`); `game-stdlib` untouched.

## Verification Layers

1. Effect-coverage invariant -> unit test: each spec §8.4 effect is emitted for its trigger (drop, land, turn-advance, win, draw, bot-chose).
2. Ordering/determinism invariant -> golden trace / deterministic replay-hash check: the effect sequence for a fixed command stream is stable and ordered.
3. No-leak invariant -> no-leak visibility test: no effect payload carries hidden/internal/candidate-ranking data; the bot-rationale field holds only viewer-safe prose.
4. Landing-anchor invariant -> unit test: piece-landed effect carries the Rust-computed column + landing cell id matching the rule engine.

## What to Change

### 1. `games/column_four/src/effects.rs`

Define the typed `ColumnFourEffect` enum and emission points: drop accepted; piece landed (actor seat, column id, landing cell/row, display from/to anchors); turn advanced when non-terminal; win detected (winning seat + ordered line cell ids); draw detected; bot chose action (viewer-safe rationale string). Record effects into the generic `EffectLog` in deterministic order during the apply-action transition.

## Files to Touch

- `games/column_four/src/effects.rs` (new)

## Out of Scope

- Replay projection (GAT5COLFOUPUB-006) and bot rationale generation (008 supplies the rationale text consumed here).
- Web animation scheduling (GAT5COLFOUPUB-014).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p column_four effects` — effect coverage, ordering, and landing-anchor tests pass.
2. `cargo test -p column_four` — no regression.
3. No-leak review: effect payloads carry only viewer-safe fields.

### Invariants

1. Effects are ordered, deterministic, and viewer-safe; the piece-landed effect's column/cell match the Rust rule engine.
2. The bot-chose-action effect's rationale is public prose, never a score array or candidate ranking.

## Test Plan

### New/Modified Tests

1. `games/column_four/src/effects.rs` (unit tests) — per-trigger emission, deterministic ordering, landing-anchor correctness, no-leak rationale shape.

### Commands

1. `cargo test -p column_four effects`
2. `cargo test -p column_four`
3. `cargo clippy -p column_four --all-targets -- -D warnings`
