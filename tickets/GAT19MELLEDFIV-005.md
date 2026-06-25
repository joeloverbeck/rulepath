# GAT19MELLEDFIV-005: Match/round state, action tree, effect groups, and replay skeleton

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/meldfall_ledger/src/{state,actions,effects,replay_support}.rs`
**Deps**: GAT19MELLEDFIV-004

## Problem

The behavior tickets need shared state and contract skeletons: `state.rs` (match/round state, private hands, stock, public discard pile, public meld tableau, score ledger, pending discard-pickup commitment, terminal summaries), `actions.rs` (Rust-owned action tree + command payloads for draw-source, discard-index, meld, lay-off, discard, turn-finish), `effects.rs` (viewer-safe semantic effect groups), and `replay_support.rs` (Trace Schema v1 integration + export scaffolding under ADR 0009). Later tickets fill the legality/scoring logic; this lands the typed shapes.

## Assumption Reassessment (2026-06-25)

1. `games/river_ledger/src/{state,actions,effects,replay_support}.rs` and `games/blackglass_pact/src/{state,actions,effects}.rs` are the structural patterns (private hands + public zones + viewer-safe effects); the skeleton stubs exist from GAT19MELLEDFIV-003 and `cards`/`setup` from GAT19MELLEDFIV-004.
2. Spec §4.1 module-responsibility table and Appendix B (suggested `MatchState`/`RoundState`/`MeldGroup`/`TableCard` shapes) define the state model; Appendix B.2 defines the progressive action tree.
3. Cross-artifact: the action-tree, command-envelope, and effect-envelope contracts are the shared boundary (`docs/ARCHITECTURE.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md`); this crate consumes engine-core's generic `ActionNode`/`ActionPath`/effect-envelope contracts and the MSC-8C-004 action-tree v1 framing — additive use, no kernel change.
4. FOUNDATIONS §2 behavior authority: the action tree and effects are Rust-owned; TypeScript will only render them. The skeleton must keep score/legality fields off the public projection until the rules tickets compute them.
5. FOUNDATIONS §11 determinism + ADR 0009: `replay_support.rs` integrates Trace Schema v1 with stable serialization order; this is substrate for the viewer-scoped exports (GAT19MELLEDFIV-013) — it must introduce no nondeterministic field (no wall-clock, insertion-ordered collections) the later export layer would have to undo.

## Architecture Check

1. Landing the typed state/action/effect/replay shapes first gives every later legality ticket a stable surface to extend, keeping each a small diff.
2. No backwards-compatibility shims — new shapes.
3. `engine-core` stays generic (only its contract vocabulary is consumed); `game-stdlib` unchanged; mechanic nouns stay in this crate.

## Verification Layers

1. State/action/effect shapes compile and round-trip serialize deterministically -> `cargo test -p meldfall_ledger` (serialization smoke).
2. Action tree conforms to the engine action-tree contract -> schema/serialization validation against `docs/ENGINE-GAME-DATA-BOUNDARY.md`.
3. Replay scaffolding emits Trace Schema v1 fields with stable order -> deterministic replay-hash check (full assertions in GAT19MELLEDFIV-013/016).

## What to Change

### 1. `state.rs`

`MatchState` (seats, cumulative scores, dealer, round, terminal), `RoundState` (active seat, phase, stock [internal], public discard, public meld tableau, pending pickup commitment, round-played scores), `SeatState` (private hand), `MeldGroup`/`TableCard` (origin seat + per-card score-credit owner).

### 2. `actions.rs`

Action tree + command payloads: `DrawFromStock` / `DrawFromDiscard{index}`, `MeldNew{cards}`, `LayOff{card,target_meld,position}`, `Discard{card}`, `GoOutWithoutDiscard`, turn-finish.

### 3. `effects.rs` and `replay_support.rs`

Viewer-safe semantic effect groups (draw / meld / lay-off / discard / round-score / match-terminal). Trace Schema v1 integration + export scaffolding (stable serialization order) under ADR 0009; redaction layer deferred to GAT19MELLEDFIV-012/013.

## Files to Touch

- `games/meldfall_ledger/src/state.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/src/actions.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/src/effects.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/src/replay_support.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/tests/serialization.rs` (modify; created by GAT19MELLEDFIV-003 — shape round-trip smoke)

## Out of Scope

- Meld/lay-off/draw/scoring legality (GAT19MELLEDFIV-006…011).
- View redaction and viewer-scoped exports (GAT19MELLEDFIV-012/013).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p meldfall_ledger` serialization smoke: state/action/effect round-trip with stable order.
2. `cargo build --workspace` succeeds.
3. Action-tree shapes match the engine action-tree contract (no game legality embedded in TS-facing fields).

### Invariants

1. Behavior authority stays in Rust; the action tree carries no precomputed legality TypeScript could invent (FOUNDATIONS §2).
2. Serialization order is stable and deterministic (FOUNDATIONS §11; ADR 0009).

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/serialization.rs` — state/action/effect shape round-trip + stable-order smoke.

### Commands

1. `cargo test -p meldfall_ledger`
2. `cargo test --workspace`
3. Full replay-hash/no-leak assertions are the boundary of GAT19MELLEDFIV-013/016; this ticket verifies shape + stable serialization only.
