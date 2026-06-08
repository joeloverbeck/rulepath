# GAT9TOKBAZBRO-005: Apply transitions + effects + terminal/tie-breaks

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/token_bazaar/src/effects.rs` (new), `src/rules.rs` (modify, apply half), `src/lib.rs` (modify)
**Deps**: GAT9TOKBAZBRO-004

## Problem

This ticket implements the authoritative state transitions and the semantic
effect payloads for every action, plus market refill, turn advance, terminal
detection, and tie-breaks. Every resource gain, payment, exchange, supply return,
score change, and slot refill must be a structured, replayable effect carrying
enough accounting data for audit and UI — not a prose string. This is the core of
the gate's "public accounting correctness" proof.

## Assumption Reassessment (2026-06-08)

1. `games/token_bazaar/src/rules.rs` (GAT9TOKBAZBRO-004) provides validated
   commands; this ticket adds the `apply_action(state, validated) -> Vec<Effect>`
   path to that file and creates `effects.rs`. The sibling
   `games/high_card_duel/src/effects.rs` + the apply half of its `rules.rs`
   establish the house pattern (verified present). `src/lib.rs` modified to add
   `mod effects;`.
2. The transition + effect + terminal + tie-break rules are fixed by
   `specs/gate-9-token-bazaar-browser-proof.md` → "Legal actions" (the per-family
   Effect bullets), "Turn structure" (terminal: both seats took 8 turns, or last
   contract fulfilled with no slots remaining), "Winner and tie-breaks" (score →
   fulfilled count → total remaining inventory → draw), and "Effects" (the six
   required semantic effect shapes).
3. Cross-artifact boundary under audit: the effect-envelope contract from
   `docs/ARCHITECTURE.md` / `docs/ENGINE-GAME-DATA-BOUNDARY.md`. The effect kinds
   and their accounting fields are consumed by visibility (-006), replay (-007),
   serialization tests (-009), golden traces (-010), the effect log UI (-015), and
   WASM (-013). They must conform to the engine effect-envelope shape.
4. FOUNDATIONS §2 (behavior authority): scoring, terminal detection, refill, and
   tie-breaks are computed only in Rust. TypeScript never computes affordability,
   refill, winner, or terminal outcome (enforced downstream).
5. FOUNDATIONS §11 determinism: effects are deterministic and stably ordered;
   identical command stream → identical effect sequence → identical replay/hash
   (proved in -007/-010). Resource conservation (inventories + supply + paid
   contracts) holds across every transition; no count goes negative. All effects
   are public, so there is no redaction path — but the no-leak test (-009) asserts
   no effect carries a debug-only field.
6. Effect-envelope schema: this ticket introduces the closed set of effect kinds
   (resource collection, resource exchange, contract fulfilled, slot refilled/
   exhausted, turn advanced, terminal outcome). Consumers enumerated above; the set
   is new/additive for this game and listed here so trace/UI/WASM tickets cover
   every kind.

## Architecture Check

1. Building apply/effects on top of the already-reviewed legality (-004) keeps the
   mutating path isolated and reviewable; tie-breaks live beside terminal detection
   so the win contract is in one place.
2. No backwards-compatibility aliasing/shims — apply path and effects are new.
3. `engine-core` stays noun-free: effect kinds are game-local typed payloads the
   kernel transports opaquely. No `game-stdlib` resource/market/economy helper is
   created — accounting stays local first-use (spec generic-promotion decision;
   atlas records it as a later candidate, no open promotion debt).

## Verification Layers

1. Transition correctness per family -> `cargo test -p token_bazaar` (collect/
   exchange/fulfill apply tests asserting inventory/supply/score/slots after).
2. Resource conservation + no-negative -> property test (full suite in -009);
   targeted apply unit tests here.
3. Refill + terminal + tie-breaks -> unit tests for refill-from-queue,
   empty-when-exhausted, both-terminal conditions, and each tie-break tier.
4. Effects are deterministic + stably serialized -> serialization round-trip +
   deterministic replay-hash (proved end-to-end in -007/-010).

## What to Change

### 1. `games/token_bazaar/src/effects.rs`

Typed effect kinds with accounting fields: collection (seat, bundle, deltas,
inventory-after, supply-after); exchange (seat, paid, taken, inventory-after,
supply-after); contract fulfilled (seat, slot, contract, cost, points,
score-after); slot refilled/exhausted (slot, new contract or empty, remaining
queue length); turn advanced (next active seat, per-seat turn counts); terminal
outcome (winner/draw, scores, tie-break data). Stable serialization.

### 2. `games/token_bazaar/src/rules.rs` (modify — apply half)

`apply_action`: mutate state per family, emit the effects above, advance the
turn, run refill, check terminal, and on terminal emit the outcome with computed
tie-breaks. Enforce conservation and no-negative invariants.

### 3. `games/token_bazaar/src/lib.rs` (modify)

Add `mod effects;`; re-export the effect + apply surface.

## Files to Touch

- `games/token_bazaar/src/effects.rs` (new)
- `games/token_bazaar/src/rules.rs` (modify)
- `games/token_bazaar/src/lib.rs` (modify)

## Out of Scope

- Viewer projection / UI metadata (GAT9TOKBAZBRO-006).
- Replay support (GAT9TOKBAZBRO-007); bots (GAT9TOKBAZBRO-008).
- Full property/golden-trace suites (GAT9TOKBAZBRO-009/010) — only targeted unit
  tests here.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p token_bazaar` — apply tests for collect/exchange/fulfill assert
   exact inventory/supply/score/slot/queue/turn after each.
2. `cargo test -p token_bazaar` — terminal detection (both conditions) and each
   tie-break tier (score, fulfilled count, total inventory, draw).
3. `cargo test -p token_bazaar` — refill-from-queue and empty-slot-when-exhausted.

### Invariants

1. Resources are conserved across inventories + supply + held/paid contracts; no
   count is ever negative.
2. Scoring, refill, terminal, and tie-breaks are computed only in Rust and are
   deterministic.
3. Every accounting transition is represented by a structured effect (no
   prose-only accounting).

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/src/rules.rs` (unit) — apply per family + terminal + tie-breaks.
2. `games/token_bazaar/src/effects.rs` (unit) — effect field correctness + serialization.

### Commands

1. `cargo test -p token_bazaar`
2. `cargo build -p token_bazaar && bash scripts/boundary-check.sh`
3. End-to-end conservation across long playouts is verified by simulation/replay
   in GAT9TOKBAZBRO-010/012; per-crate unit tests are the correct boundary here.
