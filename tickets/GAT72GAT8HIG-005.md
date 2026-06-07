# GAT72GAT8HIG-005: Effect families + visibility filtering (effects.rs)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/high_card_duel/src/effects.rs`
**Deps**: GAT72GAT8HIG-004

## Problem

Gate 8 animation and audit trail are driven by Rust semantic effects, and the
effects are a primary hidden-information leak surface. `high_card_duel` needs its
typed effect families with correct `Public` / `PrivateToSeat(owner)` visibility,
so the transitions ticket can emit them and viewers receive only what their seat
may know.

## Assumption Reassessment (2026-06-07)

1. Verified the effect-visibility primitive: `crates/engine-core` exposes
   `EffectLog` and `VisibilityScope` (imported by `crates/wasm-api/src/lib.rs:31`
   as `EffectLog, … VisibilityScope`); sibling games define a game effect enum
   and emit through `EffectLog` (e.g. `games/draughts_lite/src/effects.rs`).
2. Verified against the spec: §4.2.2 effects table fixes the families and
   visibility — `hcd_deal_private_card`/`hcd_own_commit_confirmed`/
   `hcd_private_diagnostic` = `PrivateToSeat(owner)`; `hcd_hand_count_changed`/
   `hcd_commit_face_down`/`hcd_cards_revealed`/`hcd_round_scored`/
   `hcd_refill_started`/`hcd_terminal`/`hcd_public_diagnostic` = `Public`.
3. Cross-artifact boundary under audit: the effect-envelope visibility contract
   (`docs/ENGINE-GAME-DATA-BOUNDARY.md`) — these are new effect *kinds* on an
   existing envelope; the extension is additive (new variants) and each carries
   an explicit visibility scope.
4. FOUNDATIONS principle under audit (§11 no-leak firewall): no private card
   identity may appear in a `Public` effect payload, key, or text.
5. Enforcement surface named: the §11 hidden-information no-leak firewall. This
   ticket builds the effect side of it — confirm `hcd_commit_face_down` (Public)
   carries occupancy only, never the committed card identity, and
   `hcd_own_commit_confirmed` (private) is the only place an owner sees their own
   committed card before reveal. Filtering correctness is proven by the no-leak
   suite (GAT72GAT8HIG-011) and effect-filter tests.

## Architecture Check

1. Defining effect *kinds* with their visibility scopes before transitions emit
   them is cleaner than inferring visibility at emit sites — the scope is a
   property of the effect kind, centralized here.
2. No backwards-compatibility shims — new game-local effect enum.
3. `engine-core` `EffectLog`/`VisibilityScope` reused as-is; no mechanic noun
   enters the kernel; no `game-stdlib` change.

## Verification Layers

1. Public effects carry no private identity -> no-leak visibility test: `hcd_commit_face_down`/`hcd_hand_count_changed` payloads contain occupancy/counts only.
2. Private effects scoped to owner -> schema/serialization validation: `hcd_deal_private_card`/`hcd_own_commit_confirmed`/`hcd_private_diagnostic` are `PrivateToSeat(owner)`.
3. Reveal is simultaneous + public -> golden trace check (exercised in 012): `hcd_cards_revealed` is the first public surfacing of both committed cards.

## What to Change

### 1. `effects.rs`

Define the `HighCardDuelEffect` enum with the ten families from spec §4.2.2,
each annotated with its `VisibilityScope`, plus the constructor/emit helpers the
transitions ticket calls. Public diagnostics use redacted public tokens;
private diagnostics never expose opponent/deck facts (`HCD-DIAG-003/005`).

## Files to Touch

- `games/high_card_duel/src/effects.rs` (modify — fill stub)

## Out of Scope

- Emitting effects from transitions (GAT72GAT8HIG-007 calls these).
- View projection / effect-cursor filtering at the WASM boundary (008/016).

## Acceptance Criteria

### Tests That Must Pass

1. `effect_visibility_scopes_match_spec` — each effect kind carries the spec §4.2.2 visibility scope.
2. `public_effects_contain_no_private_card_identity` — Public payloads have no `hcd:r..` card-id field/text.

### Invariants

1. No private card identity in any `Public` effect payload/key/text (§11).
2. Effect kinds are additive variants on the existing effect envelope; each has an explicit visibility scope.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/visibility.rs` — effect-visibility assertions (file extended by 008/011 with view-level no-leak tests).

### Commands

1. `cargo test -p high_card_duel --test visibility effect`
2. `cargo test -p high_card_duel`
3. Effect-level tests are the correct boundary here; observer/seat effect-set filtering is proven end-to-end in the no-leak suite (011).
