# GAT14EVEFROEVE-007: Event effects and edict modifier system

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/event_frontier/src/{cards,effects,rules}.rs` (exhaustive typed event match; typed edict modifier list)
**Deps**: GAT14EVEFROEVE-006

## Problem

This is the gate's defining hazard held: **events are code, not data.** Each of the fourteen ordinary events and four edicts is an exhaustive typed Rust match on the closed `CardId` enum; the card files declare only identity and parameters. Edicts are **rule exceptions as projections, not mutations**: each appends a typed modifier variant to an active-modifier list consulted at exactly the validation/application points it modifies, applied in stable `(kind, activation-index)` order; expiry is a list clear. Base rules are never patched and never reverse-patched. This is the FOUNDATIONS §12 "static files start acting procedural" line, held by construction.

## Assumption Reassessment (2026-06-12)

1. The closed `CardId` enum and the op validation/consultation point exist: verified ticket 003 authored the closed `CardId` enum in `cards.rs` (effect bodies stubbed/deferred here) and ticket 006 left a documented active-edict consultation point in `rules.rs` op validation. The effect envelope and per-component effects come from ticket 005/006's `effects.rs`.
2. The event/edict catalog is specified: verified the spec's "Events and edicts" — fourteen typed one-shot events plus four edicts (`Toll Roads` +1 resource/selected site until Reckoning; `Survey Ban` no survey/rally at contested sites; `Requisition` Charter ops at depot sites cost 0; `Long Season` first eligible may select one extra site) — each a typed modifier variant active until the next Reckoning.
3. Cross-crate boundary under audit: the exhaustive match means adding/removing a card is a compile error until its typed behavior exists; the edict modifier list is consulted at the specific op cost/legality points (ticket 006) and the eligibility/ops-value points it modifies, never by mutating base state. The modifier order must be stable `(kind, activation-index)` for deterministic replay.
4. FOUNDATIONS §5 (static data is not behavior) and §12 (stop condition: static files acting procedural) motivate this ticket. Restated before trusting the spec: no card file may carry `when`/`if`/`condition`/`trigger`/`selector`/`effect`/`script`; every legality consequence is typed Rust; the edict system is a typed modifier list, not an interpreter.
5. Fail-closed / determinism surface (§5/§11): serialization tests (ticket 011) reject behavior-looking card fields, and replay must reproduce edict ordering exactly. Confirm the modifier list is applied in stable `(kind, activation-index)` order (no hash-map iteration dependence), that expiry is a deterministic list clear at Reckoning, and that no edict reverse-patches base rules. No replay/hash *semantics* change.

## Architecture Check

1. Layered typed modifiers consulted at fixed points (the MtG-layers-style pattern adapted as a typed list) is cleaner and more deterministic than patching/un-patching base rules: base legality stays a single source of truth, and "what an edict changes" is localized to the consultation points.
2. No backwards-compatibility aliasing/shims — fills `cards.rs` effect bodies and the edict consultation.
3. `engine-core` stays noun-free (edict/event nouns are local); no `game-stdlib` promotion (`ModifierStack`/`VictoryCondition` helpers are explicitly forbidden by the spec; the §4 ledger authorized none).

## Verification Layers

1. Every event's typed effect (§5) -> a rule test per event card asserting its component/resource effect; a serialization test that behavior-looking card fields (`when`, `condition`, `trigger`, `effect`, `script`) are rejected.
2. Edict activation/modification/expiry -> rule tests for each edict's activation, the modified-rule consequence at its consultation point, and expiry as a list clear at the next Reckoning.
3. Stable modifier order (§11) -> a replay/property test that two edicts active simultaneously apply in stable `(kind, activation-index)` order and reproduce identical hashes.
4. No base-rule mutation -> a test/inspection that an edict never mutates base legality state; removing the edict restores base behavior exactly (no reverse-patch).

## What to Change

### 1. Exhaustive event match (`src/cards.rs`, `src/effects.rs`)

Implement the typed effect for each of the fourteen events as an exhaustive match on `CardId` (component placement/removal/relocation, resource swings, targeted at typed parameters), each emitting one semantic effect per change plus `EventResolved { card, summary }`. The match is non-exhaustive-forbidden: every `CardId` arm is present.

### 2. Edict modifier system (`src/cards.rs`, `src/rules.rs`)

Implement the four edicts as typed `ActiveEdict` variants appended to `state.active_edicts` (typed variant + activation index). Consult the list at the op cost/legality/ops-value points it modifies (the ticket-006 consultation point and the eligibility selection point), applied in stable `(kind, activation-index)` order. Emit `EdictActivated { card, edict }`; expiry at Reckoning (ticket 008) is a list clear emitting `EdictExpired { edict }`.

## Files to Touch

- `games/event_frontier/src/cards.rs` (modify; created by 003)
- `games/event_frontier/src/effects.rs` (modify; created by 003)
- `games/event_frontier/src/rules.rs` (modify; created by 003)

## Out of Scope

- The Reckoning pipeline that triggers edict expiry and victory (ticket 008) — this ticket emits expiry effects only when invoked by the Reckoning reset there.
- Any data-driven card behavior, DSL, or expression language — explicitly forbidden.
- Promoting the edict/modifier machinery to `game-stdlib` — forbidden by the spec.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p event_frontier` passes a per-event typed-effect test for all fourteen events.
2. Each edict's activation, modified-rule consequence, and expiry test passes; two simultaneous edicts apply in stable order.
3. The serialization test rejecting behavior-looking card fields passes.

### Invariants

1. Card behavior is an exhaustive typed Rust match on a closed enum; no card file carries a behavior field.
2. Edicts are a typed modifier list consulted at fixed points in stable `(kind, activation-index)` order; base rules are never patched or reverse-patched.

## Test Plan

### New/Modified Tests

1. `games/event_frontier/tests/rules.rs` — per-event effects; per-edict activation/consequence/expiry.
2. `games/event_frontier/tests/serialization.rs` — behavior-looking card field rejection.
3. `games/event_frontier/tests/property.rs` — stable simultaneous-edict ordering and no-reverse-patch invariant.

### Commands

1. `cargo test -p event_frontier --test rules --test serialization`
2. `cargo test -p event_frontier`
3. The per-crate rule/serialization tests are the correct boundary — the §5 boundary is provable at the crate level; the full no-leak suite lands in ticket 011.
