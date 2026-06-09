# GAT101PLATRI-008: Viewer-scoped semantic effects

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/plain_tricks/src/effects.rs`. No `engine-core`/`game-stdlib` change.
**Deps**: GAT101PLATRI-007

## Problem

The game needs typed semantic effects with public/private visibility envelopes that drive animation and feedback (FOUNDATIONS §7/§11): private deal effects (own cards only), and public play/trick/score/rotation/terminal effects. Effects must already be safe for the receiving viewer — no hidden card reaches a non-owner.

## Assumption Reassessment (2026-06-09)

1. `games/plain_tricks/src/rules.rs` (GAT101PLATRI-007) produces the state transitions whose semantics these effects describe. The effect-envelope + `VisibilityScope` contracts come from `engine-core` (`EffectEnvelope`, `PrivateToSeat`, public scope) per `docs/ENGINE-GAME-DATA-BOUNDARY.md`; mirror `games/poker_lite/src/effects.rs`.
2. Spec §4 (`effects.rs`) and appendix A7 fix the effect set: `deal_started` (public, counts/round), `hand_dealt` (PrivateToSeat, own cards), `deal_completed` (public, counts + leader), `card_played` (public, seat + card + led-suit flag), `trick_resolved` (public, both cards + winner + counts), `round_scored` (public, per-seat counts + totals), `deal_rotated` (public, round-2 leader), `match_resolved`/`terminal` (public, totals + decisive cause), `bot_chose_action` (public: policy id + action family only).
3. Shared boundary under audit: the effect-envelope + visibility-scope schema (`engine-core`). The extension is additive (a new game's effect set), not a change to the shared envelope shape.
4. FOUNDATIONS §7 (semantic effects drive animation) and §11 (effect logs must not leak hidden info) are under audit.
5. Enforcement surface: §11 no-leak firewall on effect logs. Only `hand_dealt` carries card identities and only under `PrivateToSeat` to the owning seat; every public effect carries counts/played-cards/winner/scores but never an unplayed card or the tail. A card identity becomes public exactly via `card_played`. Verified by the no-leak suite in GAT101PLATRI-009.
6. Extends the effect-envelope contract additively: consumers are the visibility projection (GAT101PLATRI-009), replay (GAT101PLATRI-011), the WASM bridge (GAT101PLATRI-016), and `effectFeedback.ts` (GAT101PLATRI-017); all are new arms, no existing effect kind changes.

## Architecture Check

1. Emitting effects from the resolved transition (vs. letting the renderer infer state diffs) makes animation Rust-driven and viewer-safe (FOUNDATIONS §7); renderer diffs stay diagnostics only.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched and noun-free; effect payload types are `plain_tricks`-local. No `game-stdlib` change.

## Verification Layers

1. Private deal effects scoped `PrivateToSeat` (own cards only); all reveal-bearing effects public only after play -> no-leak visibility tests (GAT101PLATRI-009).
2. Effect set matches A7 and carries decisive-cause on terminal -> effect unit tests + golden traces (GAT101PLATRI-011).
3. Effect-envelope schema conformance (additive) -> schema/serialization validation.
4. `bot_chose_action` public payload carries policy id + action family only -> no-leak unit test (full bot wiring in GAT101PLATRI-013).

## What to Change

### 1. `games/plain_tricks/src/effects.rs`

Implement the typed semantic effects from A7 with their visibility envelopes: `hand_dealt` as `PrivateToSeat` (own cards only); all others public with counts/played-cards/winner/scores/leader/decisive-cause as specified. Wire effect emission to the GAT101PLATRI-007 transitions. Ensure no public effect carries an unplayed card id, suit/rank label, or tail card.

## Files to Touch

- `games/plain_tricks/src/effects.rs` (new)

## Out of Scope

- View projection and the exhaustive no-leak string-search suite (GAT101PLATRI-009).
- Bot explanation effects content (GAT101PLATRI-013) — this ticket defines the `bot_chose_action` public shape only.
- `effectFeedback.ts` browser copy (GAT101PLATRI-017).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks` effect tests: `hand_dealt` is PrivateToSeat with own cards only; `card_played`/`trick_resolved`/`round_scored`/`terminal` public payloads match A7.
2. No public effect JSON contains an unplayed card id, suit/rank label, or tail card.
3. `match_resolved`/`terminal` carries the decisive cause (per-round + total trick counts).

### Invariants

1. A card identity becomes public exactly via `card_played`, never earlier via any effect (FOUNDATIONS §11 no-leak firewall).
2. Semantic effects (not renderer diffs) describe every state change for animation (FOUNDATIONS §7).

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/tests/visibility.rs` (effect-scope cases) — `hand_dealt` private; public effects carry no hidden identity.
2. Effect-set unit tests in `effects.rs` — payload shape per A7.

### Commands

1. `cargo test -p plain_tricks --test visibility`
2. `cargo test -p plain_tricks`
3. Per-crate scope is correct; cross-surface (DOM/export) no-leak proofs land in GAT101PLATRI-009/011/018.
