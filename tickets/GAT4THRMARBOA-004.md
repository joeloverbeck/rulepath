# GAT4THRMARBOA-004: Three Marks semantic effects

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — new `games/three_marks/src/effects.rs`; effect coverage in `tests/rule_tests.rs`
**Deps**: GAT4THRMARBOA-003

## Problem

Three Marks must emit Rust semantic effects (facts, not animation instructions) sufficient for UI, logs, replay, diagnostics, and bot explanations: mark placed, turn changed, line completed, draw reached, game ended, placement-rejected diagnostic, and bot-chose-action. Per FOUNDATIONS §11, animation is driven by these effects, not by guessed renderer diffs — so they must exist before the WASM/UI tickets consume them.

## Assumption Reassessment (2026-06-06)

1. `games/race_to_n/src/effects.rs` is the mirror; `crates/wasm-api/src/lib.rs` exposes a get-effects path and effect counts (`effect_count` at line 326), and `apps/web/src/components/EffectLog.tsx` + `effectFeedback.ts` consume them. `three_marks` mirrors the effect-envelope shape with game-specific payloads.
2. Spec `specs/gate-4-three-marks-board-smoke.md` §10 defines the minimum effect set and payloads; §15.1 implies effect assertions in rule tests. The apply path and diagnostics from GAT4THRMARBOA-003 are the producers.
3. Cross-artifact boundary under audit: the generic `engine-core` effect-envelope contract (FOUNDATIONS §3) — `three_marks` adds typed effect kinds without leaking board nouns into the kernel.
4. FOUNDATIONS §11 (semantic effects drive animation; renderer diffs are diagnostics only) and §7 (animation MUST be driven by Rust effects) motivate this ticket: every animatable board event (placement, turn change, line completion, draw, game end) is a Rust effect; the browser schedules animation from them and settles to the latest public view.
5. No-leak firewall enforcement surface (§11): effect payloads and the placement-rejected diagnostic are viewer-safe — name the firewall. Three Marks is perfect-information, so there is no hidden state to redact; the rejection diagnostic carries a safe reason category (occupied / stale / invalid cell / wrong actor / terminal) and human-readable label only, never private internals or debug detail.
6. Extends the effect-envelope contract with game-specific effect kinds. Consumers: `crates/wasm-api` get-effects (GAT4THRMARBOA-009), `apps/web` EffectLog/bot-explanation (011), and replay step projection (007). The extension is additive (new discriminated effect kinds behind the generic envelope).

## Architecture Check

1. Modelling each board transition as a semantic fact (cause) rather than an animation instruction keeps Rust the behaviour authority and lets React schedule motion downstream — cleaner and matching `race_to_n`. Alternative (UI infers events from view diffs) is a §11/§12 stop condition and rejected.
2. No backwards-compatibility aliasing/shims — new module.
3. `engine-core` gains no mechanic nouns (effect kinds are game-local in `effects.rs`); no `game-stdlib` extraction.

## Verification Layers

1. Effect-emission invariant -> rule test (applying a legal placement emits `mark placed` + turn/terminal effects with correct seat/cell/ply payload).
2. Win/draw effect invariant -> rule test (`line completed` carries winning seat + exact ordered cells; `draw reached` carries full-board indicator; `game ended` carries outcome).
3. Diagnostic-effect no-leak invariant -> no-leak visibility test (rejected-placement diagnostic exposes only a safe reason category + label, no private/debug internals).

## What to Change

### 1. `src/effects.rs`

Define the game-specific semantic effect kinds and payloads from spec §10: match-started/setup-complete (game/variant/rules version/seats), turn-started/active-player-changed (active seat, ply, previous seat), mark-placed (seat, cell, ply, occupancy summary/hash ref), placement-rejected diagnostic (safe reason category + label), line-completed (winning seat + ordered cells), draw-reached (full-board indicator), game-ended (outcome, winner/draw, final ply, terminal hash ref), bot-chose-action (level/policy id, chosen cell/action id, viewer-safe inputs + concise Level-1 explanation). Wire emission into the apply/reject paths from GAT4THRMARBOA-003.

### 2. `tests/rule_tests.rs` (extend)

Add effect assertions: placement emits mark-placed + turn/terminal effects; win emits line-completed with exact cells; draw emits draw-reached + game-ended; rejection emits a safe diagnostic effect.

## Files to Touch

- `games/three_marks/src/effects.rs` (new)
- `games/three_marks/src/lib.rs` (modify)
- `games/three_marks/tests/rule_tests.rs` (modify)

## Out of Scope

- The bot-chose-action *explanation content* and policy (effect kind is defined here; the Level-1 policy that fills it lands in GAT4THRMARBOA-006).
- View projection (005), replay step effects/hashes (007), EffectLog rendering (011).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p three_marks --test rule_tests` — effect-emission assertions pass.
2. `cargo test -p three_marks` — full crate suite green.
3. `bash scripts/boundary-check.sh` — no mechanic noun entered `engine-core`.

### Invariants

1. Every board transition (placement, turn change, win line, draw, game end) and every rejection has a corresponding Rust semantic effect; no animatable event is left to UI inference.
2. The placement-rejected diagnostic effect is viewer-safe — reason category + safe label only.

## Test Plan

### New/Modified Tests

1. `games/three_marks/tests/rule_tests.rs` — effect-emission and diagnostic-safety assertions for placement/win/draw/rejection.

### Commands

1. `cargo test -p three_marks --test rule_tests`
2. `cargo test -p three_marks && bash scripts/boundary-check.sh`
3. Replay-step effect projection is verified in 007; rule-test-level emission checks are the correct boundary for the effect-definition diff.
