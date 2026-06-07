# GAT6DIRFLI-008: Semantic effects — grouped flip effect

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/directional_flip/src/effects.rs` (semantic effect types, grouped flip children, deterministic ordering, display anchors).
**Deps**: 005

## Problem

UI animation authority comes from Rust semantic effects, not renderer state diffs (FOUNDATIONS §7/§11, spec §7.3). This ticket defines the `directional_flip` effect family — placement accepted, disc placed, **one grouped `DiscsFlipped` effect with ordered child entries** (cell id, previous owner, new owner, direction, distance/order index), pass taken, active-player changed, game ended, bot chose action — with stable deterministic ordering matching the previews. Realizes `DF-EFFECT-001`/`002`.

## Assumption Reassessment (2026-06-07)

1. `games/column_four/src/effects.rs` is the structural precedent for a game's semantic effect family. The flip-collection function in `games/directional_flip/src/rules.rs` (GAT6DIRFLI-005) produces the canonical ordered flip set this effect encodes; the preview in `actions.rs` (006) encodes the same set — all three must agree.
2. Spec §7.3 (effect family + grouped-flip child fields) and rule ids `DF-EFFECT-001` (placement emits accepted/place/grouped-flip/turn/terminal effects as applicable) and `DF-EFFECT-002` (grouped flip child entries match deterministic order) are authoritative. The direction order is N, NE, E, SE, S, SW, W, NW; within a direction, nearest→farthest (spec §6.4).
3. Cross-artifact boundary under audit: the effect-envelope contract in `engine-core` (consumed by replay 009, `wasm-api`/effect JSON mapping 015, web `effectFeedback.ts` 017, trace-viewer 016). `directional_flip` adds game-local effect kinds within the generic effect-envelope; confirm the envelope shape against `docs/ENGINE-GAME-DATA-BOUNDARY.md`.
4. FOUNDATIONS §11 "semantic effects drive animation; renderer diffs are diagnostics only" motivates this ticket: restate before coding — the grouped flip effect is the authoritative animation source; the renderer must not infer flip targets from state diffs (spec §12.2 forbidden).
5. Effects are serialized into golden traces and replay (deterministic replay/hash & serialization surface, FOUNDATIONS §11). Confirm the grouped-flip child order is the §6.4 canonical order (pure function, no RNG/time/iteration-order), so traces are byte-stable. The `BotChoseAction` effect carries a safe rationale only — no hidden-information leak in the effect log (§11 no-leak firewall).

## Architecture Check

1. Emitting one grouped `DiscsFlipped` with ordered children (rather than N independent flip effects) gives the renderer a single coherent multi-piece animation unit (spec §7.3, ROADMAP "multi-piece effects") and keeps the trace compact and order-stable.
2. No backwards-compatibility shims; new effect types.
3. `engine-core` stays noun-free — effect *kind* names (`DiscsFlipped`, …) are game-local payload within the generic effect-envelope; no mechanic noun enters the kernel (§3).

## Verification Layers

1. Effect-emission invariant -> rule test (`DF-EFFECT-001`): a placement emits accepted/placed/grouped-flip/turn/terminal effects as applicable; a forced pass emits pass + (on double pass) terminal effects.
2. Deterministic order invariant -> golden trace + property test (`DF-EFFECT-002`): grouped-flip children match the canonical direction + near→far order, identical across runs.
3. Effect-log no-leak -> no-leak visibility test: `BotChoseAction` rationale and all effect payloads are viewer-safe (FOUNDATIONS §11).
4. Effect-envelope conformance -> schema/serialization validation against `docs/ENGINE-GAME-DATA-BOUNDARY.md`.

## What to Change

### 1. Effect types

In `effects.rs`, define the effect family per spec §7.3: `PlacementAccepted` (or equivalent), `DiscPlaced`, `DiscsFlipped` (grouped, ordered children with cell id / previous owner / new owner / direction / distance-or-order-index), `PassTaken`/`TurnPassed`, `ActivePlayerChanged`, `GameEnded` (outcome/counts/reason/step), `BotChoseAction` (level/id/path/safe rationale/seed-tiebreak summary).

### 2. Deterministic ordering & display anchors

Encode the grouped-flip children in the §6.4 canonical order, with display anchors for the renderer. Ensure the encoded set equals the preview (006) and apply (005) sets.

## Files to Touch

- `games/directional_flip/src/effects.rs` (new)
- `games/directional_flip/src/rules.rs` (modify — emit effects from apply/pass/terminal transitions)
- `games/directional_flip/src/lib.rs` (modify — export the effects module)

## Out of Scope

- Replay serialization of effects (GAT6DIRFLI-009) and trace fixtures (013).
- Web effect-feedback mapping (GAT6DIRFLI-017) and trace-viewer rendering (016).
- Bot rationale wording (GAT6DIRFLI-011); this ticket defines the carrying effect only.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p directional_flip` — effect-emission + deterministic-order tests pass.
2. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. Animation-driving effects are Rust-emitted; grouped-flip child order is deterministic and equals preview/apply order (FOUNDATIONS §11, `DF-EFFECT-002`).
2. Effect payloads (incl. bot rationale) are viewer-safe (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `games/directional_flip/tests/rules.rs` — effect emission per transition + grouped-flip order property (expanded in GAT6DIRFLI-012; serialized order proven by the golden traces in 013).

### Commands

1. `cargo test -p directional_flip effects`
2. `cargo test -p directional_flip && bash scripts/boundary-check.sh`
3. Crate-scoped tests are correct; byte-stable serialized effect order is proven by the golden traces / replay-check in GAT6DIRFLI-013.
