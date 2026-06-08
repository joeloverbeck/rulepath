# GAT9TOKBAZBRO-009: Rust rule/property/serialization/visibility tests + standard fixture

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/token_bazaar/tests/{rules,property,serialization,visibility}.rs` (new), `data/fixtures/token_bazaar_standard.fixture.json` (new)
**Deps**: GAT9TOKBAZBRO-006, GAT9TOKBAZBRO-008

## Problem

The official-game contract requires unit/rule/property/serialization/visibility
coverage. This ticket assembles the integration test suite that proves the gate's
invariants end-to-end — resource conservation, no-negative counts, fulfilled
contracts can't be re-fulfilled, illegal commands reject without mutation,
terminal exposes no actions, action IDs are stable/duplicate-free, public views
are viewer-safe with no leak — and authors the canonical `token_bazaar_standard`
fixture that `fixture-check` and replay consume.

## Assumption Reassessment (2026-06-08)

1. The full game surface exists by now: `state`/`setup`/`actions`/`rules`/`effects`
   (GAT9TOKBAZBRO-003/004/005), `visibility`/`ui` (-006), and `bots` (-008). The
   sibling `games/high_card_duel/tests/{rules,property,serialization,visibility}.rs`
   + `data/fixtures/high_card_duel_standard.fixture.json` establish the house
   pattern (verified present).
2. The invariants are fixed by `specs/gate-9-token-bazaar-browser-proof.md` →
   "Fixtures, properties, rule coverage, and benchmarks" (the property/invariant
   list) and "Replay, export, and no-leak requirements" (no fixture/DOM/storage/
   export/rationale carries internal-only fields). The fixture mirrors the
   `high_card_duel` fixture schema for `fixture-check`.
3. Cross-artifact boundary under audit: the fixture is shared data consumed by
   `fixture-check` (-012) and available to replay/trace tickets; its schema must
   match what `tools/fixture-check` expects (verified: `fixture-check` reads a
   per-game `*_standard.fixture.json`). The fixture is `(new)`; the `fixture-check`
   arm that reads it lands in -012, which `Deps` on this ticket.
4. FOUNDATIONS §11 (conservation, fail-closed rejection, viewer-safe no-leak):
   restating before trusting the spec — resources are conserved across inventories
   + supply + paid contracts; invalid commands mutate no state; the public view and
   bot rationale leak no internal field. These are the suite's load-bearing assertions.
5. No-leak + determinism surface: the visibility test is the no-leak negative test
   (no debug/candidate field in any payload); the serialization test proves stable
   ordering / deterministic round-trip. Since the game is public there is no hidden
   field to redact, but the negative assertions still bind so a future regression
   that adds a debug field is caught.

## Architecture Check

1. Grouping the integration tests by concern (rules / property / serialization /
   visibility) matches the `high_card_duel` test layout and keeps each file a
   focused reviewable diff; the fixture lives in `data/fixtures/` so the tool and
   tests share one source of truth.
2. No backwards-compatibility aliasing/shims — new files.
3. `engine-core` untouched; tests assert the kernel stays noun-free via the
   boundary check. No `game-stdlib` helper introduced.

## Verification Layers

1. Resource conservation + no-negative -> property test (`tests/property.rs`).
2. Re-fulfill prevention, terminal-no-actions, stable/duplicate-free action IDs ->
   rules test (`tests/rules.rs`).
3. Stable serialization round-trip -> serialization test (`tests/serialization.rs`).
4. Viewer-safe / no-leak (public view + bot rationale carry no internal field) ->
   no-leak visibility test (`tests/visibility.rs`).
5. Fixture validity -> consumed by `fixture-check` in -012; here a test loads the
   fixture and asserts it round-trips to the standard state.

## What to Change

### 1. `games/token_bazaar/tests/rules.rs`

Illegal-command rejection without mutation (each diagnostic), re-fulfill
prevention, terminal exposes no normal actions, action IDs stable + duplicate-free.

### 2. `games/token_bazaar/tests/property.rs`

Randomized playouts asserting conservation, no-negative counts, legal actions
never panic / never create invalid state, and bot actions always validate.

### 3. `games/token_bazaar/tests/serialization.rs`

State + effect serialization round-trip with stable ordering.

### 4. `games/token_bazaar/tests/visibility.rs`

Observer view == seat view; no payload/view/rationale carries an internal-only
field (no-leak negative test).

### 5. `games/token_bazaar/data/fixtures/token_bazaar_standard.fixture.json`

The canonical standard fixture (schema mirroring `high_card_duel_standard.fixture.json`).

## Files to Touch

- `games/token_bazaar/tests/rules.rs` (new)
- `games/token_bazaar/tests/property.rs` (new)
- `games/token_bazaar/tests/serialization.rs` (new)
- `games/token_bazaar/tests/visibility.rs` (new)
- `games/token_bazaar/data/fixtures/token_bazaar_standard.fixture.json` (new)

## Out of Scope

- Golden traces + replay test (GAT9TOKBAZBRO-010).
- `RULE-COVERAGE.md` and tool registration (GAT9TOKBAZBRO-012).
- Benchmarks (GAT9TOKBAZBRO-011).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p token_bazaar` — rules/property/serialization/visibility suites pass.
2. `cargo test -p token_bazaar` — property test confirms conservation + no-negative
   over many randomized playouts.
3. `cargo test -p token_bazaar` — visibility test confirms no internal field leaks.

### Invariants

1. Resources are always conserved; no count goes negative; a fulfilled contract is
   never fulfilled again.
2. Invalid commands never mutate state; terminal states expose no normal actions.
3. Action-tree choice IDs are stable and duplicate-free; no internal field leaks
   to any viewer surface.

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/tests/rules.rs` — legality/rejection/terminal/ID stability.
2. `games/token_bazaar/tests/property.rs` — conservation/no-negative/no-panic/bot-validates.
3. `games/token_bazaar/tests/serialization.rs` — round-trip stability.
4. `games/token_bazaar/tests/visibility.rs` — observer==seat + no-leak negative.

### Commands

1. `cargo test -p token_bazaar`
2. `cargo test --workspace && bash scripts/boundary-check.sh`
3. Workspace test is justified here because this suite is the gate's invariant
   floor and must pass alongside every other crate before tool/WASM tickets build on it.

## Outcome

Completed: 2026-06-08

What changed:

- Added Token Bazaar integration tests for rules, property/invariant coverage,
  serialization, and visibility/no-leak surfaces.
- Added `games/token_bazaar/data/fixtures/token_bazaar_standard.fixture.json`
  as the standard fixture metadata for later fixture-check/tool registration.

Deviations from original plan:

- None.

Verification results:

- `cargo test -p token_bazaar` passed, including unit tests plus bots,
  property, rules, serialization, and visibility integration suites.
- `cargo test --workspace` passed.
- `bash scripts/boundary-check.sh` passed.
