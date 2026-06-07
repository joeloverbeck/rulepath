# GAT72GAT8HIG-008: Viewer projections (visibility.rs) + UI-metadata (ui.rs)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/high_card_duel/src/visibility.rs`, `games/high_card_duel/src/ui.rs`
**Deps**: GAT72GAT8HIG-007

## Problem

Gate 8's central proof is viewer-safe projection. `high_card_duel` needs
`project_view` to produce observer, `seat_0`, and `seat_1` views where each
viewer receives only the fields they may know — and viewer-facing Rust UI
metadata (labels, tokens, accessibility text, preview copy) that itself never
leaks hidden identities.

## Assumption Reassessment (2026-06-07)

1. Verified the projection primitive: `crates/engine-core` `Game::project_view`
   accepts a `Viewer { seat_id: Option<SeatId> }` (imported in
   `crates/wasm-api/src/lib.rs:31` as `Viewer`; sibling
   `games/draughts_lite::project_view` is the shape precedent).
2. Verified against the spec: §4.2.2 "Views" fixes the observer field set (ids,
   round/limit, phase, active/lead/reply, scores, hand counts, deck count,
   commitment occupancy as face-down, revealed cards, public cursor, terminal)
   and the seat-private additions (own hand, own committed card after commit,
   private affordances/effects) plus the forbidden set (opponent hand, opponent
   face-down card before reveal, unrevealed deck order, future draws, bot
   candidates).
3. Cross-artifact boundary under audit: the public/private view schema
   (`docs/ENGINE-GAME-DATA-BOUNDARY.md`). These are new game-local view shapes;
   serialized public/observer JSON field names must not contain private
   identifiers.
4. FOUNDATIONS principle under audit (§11 no-leak firewall + §3): hidden state
   must not appear in any projection a viewer is not authorized for; `ui.rs`
   metadata is viewer-facing Rust output and is held to the same firewall. The
   kernel stays noun-free — card vocabulary lives only in this game module.
5. Enforcement surface named: the §11 no-leak firewall. Confirm the observer
   projection has no field carrying private hands / face-down identity /
   unrevealed deck order, and the seat projection excludes the opponent's hidden
   commitment before reveal. Proven by the no-leak suite (011).
6. Schema extension classification: the public/private view is game-local and
   new (not an extension of a shared serialized view); its consumers are the
   WASM boundary (016), the web client (017/018), and the no-leak/serialization
   tests (011). Additive — no existing view schema is mutated.

## Architecture Check

1. Single `project_view(viewer)` that derives each viewer's field set from state
   is cleaner and safer than building a superset and trimming in the client —
   the firewall lives in Rust, not React.
2. No backwards-compatibility shims — new game-local projection.
3. `engine-core` `Viewer`/view contract reused; no mechanic noun in the kernel;
   `ui.rs` is game-local viewer metadata, not a `game-stdlib` promotion.

## Verification Layers

1. Observer projection no-leak -> no-leak visibility test: no private hand id / face-down identity / unrevealed deck order in the observer view.
2. Seat projection scoping -> no-leak visibility test: `seat_0` view has `seat_0` hand only; reply actor view after lead commit lacks the lead card identity.
3. Serialized field names -> schema/serialization validation: public projection JSON keys contain no private identifiers (`hcd:r..`).
4. UI-metadata safety -> manual review + no-leak test: `ui.rs` labels/accessibility text carry no hidden facts.

## What to Change

### 1. `visibility.rs`

`project_view(state, viewer)` producing the observer field set, the seat-private
superset (own hand, own committed card after commit, private affordances/effects),
and excluding all forbidden fields; commitment occupancy is face-down/redacted
until reveal; terminal view still hides the unused deck tail.

### 2. `ui.rs`

Viewer-facing Rust UI metadata: neutral themed labels ("duel cards"/"trail
badges"/"runes"), accessibility text, preview copy, disabled-reason tokens —
none carrying hidden identities.

## Files to Touch

- `games/high_card_duel/src/visibility.rs` (modify — fill stub)
- `games/high_card_duel/src/ui.rs` (modify — fill stub)

## Out of Scope

- Effect-cursor filtering at the WASM boundary (016) and the TS/React renderer (018).
- Replay export projection (009).

## Acceptance Criteria

### Tests That Must Pass

1. `observer_view_has_no_private_hand_or_deck_or_facedown_identity`.
2. `seat_view_contains_only_own_hand`, `reply_actor_view_lacks_lead_card_before_reveal`.
3. `terminal_public_view_still_hides_unused_deck_tail`.

### Invariants

1. No projection exposes a field a viewer is unauthorized for (§11 no-leak firewall).
2. Public projection JSON field names contain no private identifiers.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/visibility.rs` — observer/seat projection no-leak assertions (extends the effect-visibility file from 005).

### Commands

1. `cargo test -p high_card_duel --test visibility`
2. `cargo test -p high_card_duel`
3. The visibility suite is the correct boundary; cross-cutting property/serialization no-leak proofs are consolidated in 011.

## Outcome

Completed: 2026-06-07

What changed:

- Added viewer-safe `project_view(state, viewer)` for observer and seat-private High Card Duel projections.
- Added public fields for game/variant IDs, round/limit, phase, active/lead/reply seats, score, hand counts, deck count, commitment occupancy, revealed cards, terminal state, freshness, and UI metadata.
- Added private seat projections containing only the authorized viewer's own hand and own committed card.
- Kept observer/opponent projections redacted for private hands, face-down commitment identities, and unrevealed deck order, including terminal public views.
- Added neutral game-local UI metadata and accessibility labels with no hidden card identities.
- Extended `games/high_card_duel/tests/visibility.rs` with observer, seat, reply-before-reveal, and terminal deck-tail no-leak projection tests.

Deviations from original plan:

- None.

Verification results:

- `cargo fmt --all --check` passed.
- `cargo test -p high_card_duel --test visibility` passed.
- `cargo test -p high_card_duel` passed.
