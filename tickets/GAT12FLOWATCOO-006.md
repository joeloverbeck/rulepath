# GAT12FLOWATCOO-006: Environment automation and event resolution

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/flood_watch/src/rules.rs` (environment phase inside the turn-ending command's application), `src/effects.rs` (grouped semantic automation effects)
**Deps**: GAT12FLOWATCOO-005

## Problem

This is the gate's headline architectural proof: the environment acts. After a turn ends (the final budget point spent, or `end_turn`), a deterministic automation phase draws and resolves event cards entirely inside Rust command application — replayable from the same command stream, expressed only through semantic effects, with no synthetic actor in the command stream and no TypeScript timer or client-side sequencing. Resolution order is fixed: levee absorption before flood-level rise, and inundation stops the phase immediately.

## Assumption Reassessment (2026-06-11)

1. GAT12FLOWATCOO-005 leaves the budget decrement and the turn-ending trigger point; GAT12FLOWATCOO-004 provides the internal-ordered `event_deck` and `drawn`. `games/masked_claims/src/effects.rs` is the exemplar for grouped semantic effects with viewer-scoped payloads; the environment batch is a new grouped-effect shape (the GAT12FLOWATCOO-002 ledger records it as a first official use).
2. The spec (§Implementation reference "Turn flow" step 2 + "Semantic effect model", Work-breakdown item 5, Assumption A6) fixes: draw N events (scenario `draws_per_phase`) from the top; for `Downpour`/`StormSurge`, levees absorb up to the rise amount (`LeveeAbsorbed`) then any remainder raises the level (`FloodLevelRose`); `Reprieve` is a no-op draw (`EventDrawn`, calm rendering); on level 3, `DistrictInundated` and resolution stops immediately (remaining draws do not occur); the effects are `EnvironmentPhaseBegan`, `EventDrawn`, `LeveeAbsorbed`, `FloodLevelRose`, `DistrictInundated`, `DeckExhausted`. Assumption A6: no `Phase::Environment` a seat can act in — resolution is atomic inside the turn-ending command's application.
3. Cross-artifact boundary under audit: the effect-envelope ordering and payloads are the replay/animation contract. `EventDrawn` is the first public appearance of a drawn card (unless forecast already revealed it); the effect batch order (`EnvironmentPhaseBegan` → per-draw `EventDrawn` → `LeveeAbsorbed` → `FloodLevelRose` → optional `DistrictInundated`/`DeckExhausted`) is what the browser and replay viewer render. Effect ordering is deterministic and pinned by golden traces (GAT12FLOWATCOO-011).
4. FOUNDATIONS §2 (Rust owns state transitions, deterministic randomness, semantic effects) and the §11/§12 invariant "semantic effects drive animation; renderer diffs are diagnostics only" / "animation depends on guessed state diffs instead of Rust effects" (stop condition) motivate this ticket: the automation must be a pure consequence of the command stream, emitting every observable step as an effect so the UI never sequences or times anything itself.
5. Enforcement surface: this touches the deterministic-replay invariant (§11) — the same command stream must reproduce identical draws, absorptions, rises, and stop point. The undrawn deck order remains hidden: only drawn cards appear (in their `EventDrawn`/already-`ForecastRevealed` effect); the no-leak firewall holds because the phase reveals exactly the cards it draws and no more.

## Architecture Check

1. Resolving the environment inside the turn-ending command's application (Assumption A6) — rather than as a separate synthetic-actor command — keeps the command-stream replay semantics identical to every prior game: there is no new actor, no new phase to validate, and replay is byte-stable from the existing command log.
2. No backwards-compatibility aliasing/shims; built on GAT12FLOWATCOO-005's turn-ending trigger.
3. `engine-core` stays noun-free — the environment phase is a game-local consequence of normal command application using the generic effect-envelope contract; `event`/`deck`/`levee`/`flood` nouns stay in `games/flood_watch`.

## Verification Layers

1. Deterministic, replayable automation -> golden trace / deterministic replay-hash check: same seed+seats+scenario+command stream reproduces identical draws, absorptions, rises, and effect hashes.
2. Resolution order (absorption before rise; early stop) -> rule tests: levee-absorption case, storm-surge double-rise case, mid-phase early-stop-on-inundation case, Reprieve no-op case.
3. Effect-driven animation contract -> schema/serialization validation: every automation step emits a typed semantic effect in deterministic batch order; no state change occurs without an effect.
4. No synthetic actor / no TS sequencing -> grep-proof that resolution lives in `src/rules.rs` command application and emits effects only; no command-stream actor is added.
5. No deck-order leak -> no-leak visibility test: only drawn-card identities appear in effects; undrawn order never serialized.

## What to Change

### 1. `games/flood_watch/src/rules.rs`

Implement the environment phase triggered inside the turn-ending command's application: emit `EnvironmentPhaseBegan { turn, draws }`, then for each of `draws_per_phase` draws pop the top card, emit `EventDrawn { index, card }`, and resolve: `Downpour`/`StormSurge` → consume up to the rise from the levee stack (`LeveeAbsorbed`), then raise the level by the remainder (`FloodLevelRose`); `Reprieve` → no state change. On reaching level 3, emit `DistrictInundated { district }` and stop the phase immediately. If the deck empties with no inundation, emit `DeckExhausted`. Then cleanup (alternate active seat, refill budget, clear a drawn forecast) unless terminal — terminal detection itself lands in GAT12FLOWATCOO-007.

### 2. `games/flood_watch/src/effects.rs`

Define the environment-phase semantic effects with public payloads per the spec's effect model (role/amount/new-level public; `EventDrawn` carries the drawn card identity; no undrawn-deck data). Group them so the batch renders in drawn order.

## Files to Touch

- `games/flood_watch/src/rules.rs` (modify — add environment resolution to the turn-ending application)
- `games/flood_watch/src/effects.rs` (modify — define the automation effect set)

## Out of Scope

- Terminal detection, the `Terminal` effect, and the shared `Won`/`Lost` outcome (GAT12FLOWATCOO-007) — this ticket emits `DistrictInundated`/`DeckExhausted` but does not finalize the outcome.
- Public/private projection and per-viewer effect filtering (GAT12FLOWATCOO-008).
- Any client-side animation (GAT12FLOWATCOO-017) — this ticket emits the effects the renderer will consume.

## Acceptance Criteria

### Tests That Must Pass

1. Rule tests cover event resolution order (levee absorption before rise), storm-surge double rises, the Reprieve no-op, and mid-phase early stop on inundation (remaining draws do not occur).
2. A property test asserts the environment phase runs exactly once per turn and each card resolves exactly once (the deck only shrinks).
3. A replay test asserts identical seed+seats+scenario+command stream reproduces identical draw ordering and effect hashes.

### Invariants

1. The environment resolves atomically inside the turn-ending command's application — no synthetic actor in the command stream, no `Phase::Environment` a seat can act in, no TypeScript timer/sequencing.
2. Only drawn cards appear (in their effect); the undrawn deck order never enters any effect payload.

## Test Plan

### New/Modified Tests

1. `games/flood_watch/tests/rules.rs` — absorption-before-rise, double-rise, Reprieve, early-stop cases (expanded in GAT12FLOWATCOO-011).
2. `games/flood_watch/tests/replay.rs` — draw-order + effect-hash determinism across the environment phase (expanded in GAT12FLOWATCOO-011).

### Commands

1. `cargo test -p flood_watch --test rules`
2. `cargo test -p flood_watch`
3. `cargo run -p replay-check -- --game flood_watch --all` is the eventual golden-trace boundary but needs the traces (GAT12FLOWATCOO-011) and tool registration (GAT12FLOWATCOO-015); the rule + replay unit tests are the correct boundary for this diff.
