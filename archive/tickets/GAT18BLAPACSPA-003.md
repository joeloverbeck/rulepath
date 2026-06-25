# GAT18BLAPACSPA-003: crate skeleton, fixed-four setup, seats/teams, card and state model

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new crate `games/blackglass_pact` (lib, ids, cards, state, setup, partnerships), workspace `Cargo.toml`, `games/blackglass_pact/data/{manifest,variants}.toml`
**Deps**: GAT18BLAPACSPA-002

## Problem

Establish the `games/blackglass_pact` crate and its deterministic foundation: fixed-four setup, stable `SeatId`/`TeamId` mappings, the 52-card deck/card model, the typed `Phase`/`GameState` skeleton, the fixed partnership mapping, stable diagnostics, and the non-behavioral static-data files. Every later module ticket builds on these stubs (spec §4.1, §4.3, Appendix B, candidate task `GAT18-BLAPAC-003`).

## Assumption Reassessment (2026-06-25)

1. The sibling `games/briar_circuit/src/` layout (`ids,cards,state,setup,rules,scoring,effects,visibility,bots,replay_support,ui,variants,lib`) is the convention; Blackglass Pact adds `bidding.rs` and `partnerships.rs` (spec §4.1). This ticket creates the crate + the foundation modules and stubs the rest so later tickets `(modify)` them.
2. `SeatId`/`TeamId`/`Bid`/`Suit`/`Rank`/`Phase`/`GameState` shapes are pinned in spec Appendix B.1–B.3; `TeamId` is deliberately game-local (no kernel/team semantics).
3. Cross-crate boundary under audit: the crate joins the workspace `Cargo.toml` members and depends on `engine-core` + `game-stdlib` (the trick helpers are consumed later in GAT18BLAPACSPA-006). No new shared-crate symbol is introduced.
4. FOUNDATIONS §3 (`engine-core` is a contract kernel) motivates this ticket: `Card`/`Suit`/`Rank`/`Seat`/`Team` types stay in `games/blackglass_pact`; none enters `engine-core`. `boundary-check.sh` must stay green.
5. Determinism substrate: stable `seat_0..seat_3` / `team_0..team_1` serialization order and canonical card ordering are the inputs the later replay/hash and no-leak surfaces (GAT18BLAPACSPA-008/011) enforce. This ticket introduces no map-ordered serialization and no nondeterministic input; it names those deferred enforcement surfaces.

## Architecture Check

1. A dedicated skeleton ticket (vs. growing the crate organically per feature) gives every later ticket a stable module set and avoids create-then-modify churn on `Cargo.toml`/`lib.rs`.
2. No backwards-compatibility shims; greenfield crate.
3. `engine-core` stays noun-free (only `games/blackglass_pact` gains card/seat/team types); no `game-stdlib` change.

## Verification Layers

1. Exactly four seats accepted, every other count rejected with a stable diagnostic -> `cargo test -p blackglass_pact` setup unit tests.
2. `team_0={seat_0,seat_2}`, `team_1={seat_1,seat_3}`, stable serialization order -> serialization unit test + grep-proof of canonical order.
3. Card conservation (52 unique) and noun-free kernel -> property test + `bash scripts/boundary-check.sh`.

## What to Change

### 1. Crate + workspace wiring

`games/blackglass_pact/Cargo.toml` (deps `engine-core`, `game-stdlib`, serde); add `games/blackglass_pact` to the workspace `Cargo.toml` members; `src/lib.rs` re-exports.

### 2. Core types and setup

`ids.rs` (`SeatId`,`TeamId`), `cards.rs` (`Card`,`Suit`,`Rank`,`CardId`, deterministic deck), `state.rs` (`Phase`,`GameState`,`Bid`,`BlindNilChoice` per Appendix B), `partnerships.rs` (fixed seat→team mapping), `setup.rs` (fixed-four validation, initial dealer `seat_0`, hand context). Stub `rules.rs`/`scoring.rs`/`effects.rs`/`visibility.rs`/`bots.rs`/`replay_support.rs`/`bidding.rs`/`ui.rs`/`variants.rs` so later tickets modify them.

### 3. Static data

`data/manifest.toml` + `data/variants.toml`: typed identity, version strings, presentation params only — no selectors/conditions/formulas.

## Files to Touch

- `games/blackglass_pact/Cargo.toml` (new)
- `Cargo.toml` (modify — add workspace member)
- `games/blackglass_pact/src/{lib,ids,cards,state,setup,partnerships,rules,scoring,effects,visibility,bots,replay_support,bidding,ui,variants}.rs` (new)
- `games/blackglass_pact/data/{manifest,variants}.toml` (new)
- `games/blackglass_pact/tests/{rules,serialization,property}.rs` (new — seed cases; grown in later tickets)

## Out of Scope

- Blind/deal/bid/play/scoring behavior (GAT18BLAPACSPA-004–007 modify the stubs).
- WASM/tool/CI registration and `ci/games.json` (later tickets).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p blackglass_pact` (setup, team-mapping, card-conservation, serialization-order cases).
2. `cargo build --workspace` succeeds with the new member.
3. `bash scripts/boundary-check.sh` passes (no mechanic noun in `engine-core`).

### Invariants

1. Setup accepts only `{4}`; all other counts return a stable Rust diagnostic.
2. Seat/team serialization order is canonical and stable; no unordered map defines bytes.

## Test Plan

### New/Modified Tests

1. `games/blackglass_pact/tests/rules.rs` — setup seat-count acceptance/rejection.
2. `games/blackglass_pact/tests/serialization.rs` — stable seat/team/card order.
3. `games/blackglass_pact/tests/property.rs` — 52-card conservation/uniqueness.

### Commands

1. `cargo test -p blackglass_pact`
2. `cargo build --workspace && bash scripts/boundary-check.sh`
3. Crate-scoped tests + boundary check are the correct boundary; full-pipeline tools run after behavior lands.

## Outcome

Completed: 2026-06-25

Implemented the Blackglass Pact crate foundation:

- Added `games/blackglass_pact` as a workspace crate with `engine-core` and `game-stdlib` dependencies.
- Added game-local IDs, fixed team mapping, canonical 52-card deck/card IDs, setup validation, state/phase scaffolding, strict static-data parsing, non-behavioral `manifest.toml` and `variants.toml`, and stub modules for later behavior tickets.
- Added seed tests for fixed-four setup rejection/acceptance, stable team mapping, stable card/seat/team order, static-data behavior-key rejection, card conservation, card ID round-trips, and the pre-deal no-private-hand setup state.

Deviations from plan: the skeleton intentionally does not deal private hands at setup; it starts in `BlindNilCommitment` with no hands populated, matching the Gate 18 pre-deal blind-nil invariant. The deterministic full deal lands in GAT18BLAPACSPA-004.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p blackglass_pact` passed (1 lib test, 3 property tests, 3 rules tests, 2 serialization tests).
- `cargo build --workspace` passed.
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`; `game-test-support dev-only boundary check passed`).
