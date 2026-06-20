# GAT151RIVLED-003: Typed per-seat stack setup

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger` (`setup.rs`, `state.rs`, `variants.rs`), `data/manifest.toml`, `data/variants.toml`, new `data/fixtures/*`
**Deps**: GAT151RIVLED-001, GAT151RIVLED-002

## Problem

All-in and side-pot behavior requires genuinely different live contribution caps during a hand, which the shipped equal-implicit setup cannot reach. Per spec §3.2, add a configurable ordered per-seat starting-stack vector with an equal 24-unit default and deterministic fail-closed validation. No betting or all-in logic in this ticket — only the typed setup input, its validation, and the typed data/fixtures.

## Assumption Reassessment (2026-06-20)

1. Code: `games/river_ledger/src/setup.rs` defines `SetupOptions { variant: Variant, button_index: usize }` — no stack vector. `state.rs` posts blinds from hardcoded `STANDARD_SMALL_BLIND` / `STANDARD_BIG_BLIND`; contributions are tracked as `u16` on `SeatLedger`.
2. Docs: spec §3.2 mandates an ordered vector equal to the seat count in canonical order, equal-24 default on omission, positive bounded checked integers (no silent saturation), and minimum asymmetric fixtures `[8, 16, 24]` (3 seats) and `[4, 8, 12, 16, 20, 24]` (6 seats).
3. Cross-artifact boundary under audit: the setup schema ↔ `data/manifest.toml` + `data/variants.toml` typed parameters ↔ the WASM catalog. WASM marshalling of stacks is deferred to GAT151RIVLED-013; this ticket adds Rust-side validation and typed data only.
4. (schema/contract extension) Extends the static-data manifest/variants schema with stack defaults/presets — additive typed parameters only, no selectors/branches/conditions (§5). Consumers: `setup.rs` validation (this ticket) and the WASM catalog (GAT151RIVLED-013); the extension is additive with a default.
5. (§11 deterministic substrate) Introduces a checked game-local amount type wide enough to hold the maximum six-seat sum; confirm all behavior-critical arithmetic uses checked operations — this is the substrate the conservation invariants in GAT151RIVLED-004 enforce, and it must not admit a silent-saturation nondeterminism path.

## Architecture Check

1. One configurable model with an equal default is cleaner than an a/b hybrid and fits the Infra A–D N-seat setup/catalog bridge; the default profile keeps simple setup intact.
2. No backwards-compatibility shims; omission resolves to the typed default, not a legacy code path.
3. Stack values are typed content; all validation stays in Rust (§2/§5). `engine-core` is untouched — stack vocabulary is game-local.

## Verification Layers

1. Vector length == selected seat count, canonical seat order -> rule/unit tests in `tests/rules.rs`.
2. Omission selects the deterministic equal-24 default -> unit test.
3. Invalid input (wrong length, zero, overflow, out-of-range) rejected fail-closed -> rule tests (§11 reject-unknown / fail-closed).
4. Static data additive and typed -> schema validation against `docs/ENGINE-GAME-DATA-BOUNDARY.md`; `fixture-check` parses the new fixtures.

## What to Change

### 1. Setup schema + validation

Extend `SetupOptions` with an ordered per-seat starting-stack vector (or an equivalent typed N-seat structure); apply the equal 24-unit default on omission; add a checked game-local amount type; validate length-equals-seat-count, positive/bounded/non-overflowing values in canonical order, rejecting malformed setup with a deterministic diagnostic.

### 2. Typed data + fixtures

Add equal-default and neutral asymmetric presets to `data/manifest.toml` / `data/variants.toml` as typed parameters. Add asymmetric acceptance fixtures `[8, 16, 24]` and `[4, 8, 12, 16, 20, 24]`.

## Files to Touch

- `games/river_ledger/src/setup.rs` (modify)
- `games/river_ledger/src/state.rs` (modify)
- `games/river_ledger/src/variants.rs` (modify)
- `games/river_ledger/data/manifest.toml` (modify)
- `games/river_ledger/data/variants.toml` (modify)
- `games/river_ledger/data/fixtures/river_ledger_3p_asymmetric.fixture.json` (new)
- `games/river_ledger/data/fixtures/river_ledger_6p_asymmetric.fixture.json` (new)
- `games/river_ledger/tests/rules.rs` (modify)

## Out of Scope

- Betting, all-in, blind-capping-by-stack, or any state transition (GAT151RIVLED-004+).
- WASM setup marshalling and catalog projection (GAT151RIVLED-013).
- Bumping the rules/data version (GAT151RIVLED-011).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — setup unit/rule tests for 3–6 seats, malformed vectors, zero/out-of-range amounts, and deterministic default.
2. `cargo run -p fixture-check -- --game river_ledger` — new asymmetric fixtures parse.
3. `cargo run -p simulate -- --game river_ledger --games 100` — setup remains deterministic end-to-end.

### Invariants

1. The stack vector length always equals the selected seat count and preserves canonical seat order.
2. Each stack is a positive bounded integer; omission yields the deterministic equal-24 default; no silent saturation.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/rules.rs` — setup vector length/order/default, invalid values, checked maxima, 3–6-seat acceptance.
2. `games/river_ledger/data/fixtures/river_ledger_{3p,6p}_asymmetric.fixture.json` — asymmetric acceptance fixtures.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p fixture-check -- --game river_ledger`
3. `cargo run -p simulate -- --game river_ledger --games 100` — narrower than full workspace tests because this ticket changes only setup/data, with betting unchanged.

## Outcome

Completed: 2026-06-20

What changed:

- Added optional ordered per-seat starting stacks to `SetupOptions`, with an equal 24-unit default and Rust-side validation for seat-count length, positive bounded values, checked total arithmetic, and current forced-post capacity.
- Added `starting_stack` and `remaining_stack` to River Ledger seat ledgers and stable summaries, initialized from the validated setup vector while leaving all-in and blind-capping behavior for GAT151RIVLED-004.
- Added inert typed variant metadata for the default stack and neutral asymmetric stack presets.
- Added asymmetric 3-seat and 6-seat setup fixtures for `[8, 16, 24]` and `[4, 8, 12, 16, 20, 24]`.
- Updated direct test constructors in River Ledger and `wasm-api` tests to provide default stack vectors.

Deviations:

- Short forced-post stacks currently reject with `invalid_starting_stack_for_forced_post`; GAT151RIVLED-004 owns relaxing that path by capping forced posts and marking the posting seat all-in.

Verification:

- `cargo fmt --all --check` passed after formatting.
- `cargo test -p river_ledger` passed.
- `cargo run -p fixture-check -- --game river_ledger` passed (`fixture-check: all fixtures passed`).
- `cargo run -p simulate -- --game river_ledger --games 100` passed (`games_run=100`).
- `cargo test -p wasm-api` passed after the constructor-signature test update.
