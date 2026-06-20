# GAT151RIVLED-013: WASM bridge marshalling

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api` (`src/games/river.rs`, `src/json.rs`, `tests/api_surface.rs`)
**Deps**: GAT151RIVLED-012

## Problem

The WASM bridge must accept the typed stack setup and carry the all-in / pot-tier / uncalled-return / per-pot-allocation projections across the JSON boundary, without duplicating any rule or allocation logic. It marshals only Rust-projected fields; TypeScript reconstructs no caps, tiers, eligibility, winners, or remainders. Deterministic API-surface tests and snapshots cover the new shapes.

## Assumption Reassessment (2026-06-20)

1. Code: `crates/wasm-api/src/games/river.rs` marshals the base River Ledger setup and views; `src/json.rs` carries the serialization; `tests/api_surface.rs` snapshots the API surface. River Ledger is already registered in the wasm catalog (`crates/wasm-api/src/catalog.rs`); this ticket extends an existing registration, it does not add a new game.
2. Docs: spec §4 (WASM seam) + §7 — marshal typed stack setup in, project all-in/pot/return/allocation views out, add deterministic API surface tests and snapshots; `docs/WASM-CLIENT-BOUNDARY.md` requires a thin viewer-scoped JSON transport, not a second rules engine.
3. Cross-artifact boundary under audit: the WASM JSON contract ↔ the Rust projections from GAT151RIVLED-010 (`SeatStackView`/`PotTierView`/`UncalledReturnView`/`PotAllocationView`) and the typed setup from -003; the bridge transports these unchanged.
4. (§2 behavior authority) Restate: the bridge marshals projections only; no legality, sizing, cap, eligibility, winner, or remainder computation occurs in `wasm-api` or TypeScript. Confirm the new fields are pass-through, validated Rust-side.
5. (schema extension) The bridge JSON gains the new view fields — additive; consumers are the web renderer (GAT151RIVLED-014) and `api_surface.rs` snapshots; setup parsing rejects malformed stack vectors with the Rust diagnostic.

## Architecture Check

1. Keeping the bridge a thin transport over the GAT151RIVLED-010 projections prevents a second allocation implementation drifting from Rust.
2. No backwards-compatibility shims; setup parsing and view marshalling extend the existing river module.
3. No mechanic noun enters `engine-core`; `wasm-api` stays a marshalling layer (§2/§3).

## Verification Layers

1. Typed stack setup parsed + validated Rust-side -> wasm setup tests (malformed vectors rejected with the Rust diagnostic).
2. All-in/pot/return/allocation views marshalled faithfully -> API-surface snapshot tests.
3. JSON ordering + redaction stable -> serialization/redaction checks (no private card/deck field crosses).
4. No duplicate legality/allocation in the bridge -> code review + grep-proof (no cap/winner math in `wasm-api`).

## What to Change

### 1. Setup + view marshalling

In `src/games/river.rs`, accept the typed stack setup and marshal the projected all-in/pot-tier/return/allocation views; keep `src/json.rs` carrying them with stable ordering and redaction.

### 2. API-surface tests + snapshots

Extend `tests/api_surface.rs` with deterministic snapshots of the new setup and view shapes and malformed-setup diagnostics.

## Files to Touch

- `crates/wasm-api/src/games/river.rs` (modify)
- `crates/wasm-api/src/json.rs` (modify)
- `crates/wasm-api/tests/api_surface.rs` (modify)

## Out of Scope

- Web renderer and e2e smoke (GAT151RIVLED-014).
- Catalog README reconciliation (GAT151RIVLED-019/-020); the wasm catalog const already lists river_ledger, so no `check-catalog-docs` const change occurs here.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` — setup parsing, projected JSON, API-surface snapshots, and malformed-setup diagnostics.
2. `cargo test --workspace` — the bridge change does not regress other games.
3. `npm --prefix apps/web run smoke:wasm` — the WASM module loads and exposes the new river fields.

### Invariants

1. No cap/eligibility/winner/remainder computation exists in `wasm-api` or is required of TypeScript.
2. No private card/deck/evaluator field crosses the JSON boundary to an unauthorized viewer.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/tests/api_surface.rs` — new setup/view snapshots and malformed-setup diagnostics.

### Commands

1. `cargo test -p wasm-api`
2. `npm --prefix apps/web run smoke:wasm`
3. `cargo test --workspace` — confirms the shared bridge change is cross-game safe; the targeted `wasm-api` test is the primary boundary.
