# RIVLEDSHO-003: WASM bridge JSON + TypeScript view types for explanation fields

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs`, `apps/web/src/wasm/client.ts`
**Deps**: RIVLEDSHO-002

## Problem

The Rust-authored explanation fields (RIVLEDSHO-001, projected in RIVLEDSHO-002) must reach the browser through the single WASM JSON bridge, with matching TypeScript view types, so the React shell can render them without inventing any value. This ticket exposes the augmented terminal JSON and mirrors it in `client.ts` (spec WB3 / D2).

## Assumption Reassessment (2026-06-15)

1. Verified against current code: `crates/wasm-api/src/lib.rs` and `apps/web/src/wasm/client.ts` both reference `CardView` and the River Ledger terminal/rationale view types; the bridge serializes the viewer-scoped Rust projection to JSON and `client.ts` declares the matching TS types.
2. Verified against specs/docs: spec §6 D2 + §8 WB3; `docs/WASM-CLIENT-BOUNDARY.md` (Rust↔browser JSON bridge contract), `docs/UI-INTERACTION.md` (TS renders Rust output only), `RULES.md` `RL-UI-SHOWDOWN-001`.
3. Cross-artifact boundary under audit: the `wasm-api` JSON serialization seam and the `client.ts` type declarations; the bridge carries exactly the viewer-scoped fields RIVLEDSHO-002 projects — no widening.
4. FOUNDATIONS §2 behavior authority motivates this ticket: TypeScript types mirror the Rust shape; the bridge introduces no field TS computes and no evaluation logic.
5. §11 no-leak firewall is the enforcement surface: the bridge serializes only the already-viewer-scoped projection from RIVLEDSHO-002 (no unauthorized field crosses); a bridge-level no-leak assertion confirms folded/non-revealed seats carry no explanation in the JSON.
6. Extends the WASM view payload + TS view types (additive-only: new optional fields with defaults); the consumer is the React shell (RIVLEDSHO-004+).

## Architecture Check

1. Threading the fields through the existing single bridge seam (not a side channel) keeps one serialization boundary and one TS type source of truth.
2. No backwards-compatibility aliasing/shims; additive JSON fields with optional TS types.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4) — bridge + TS types only.

## Verification Layers

1. The augmented terminal JSON carries the explanation fields for authorized reveals -> `crates/wasm-api` bridge test (`cargo test -p wasm-api`).
2. Folded/non-revealed seats carry no explanation field in the bridged JSON -> bridge no-leak assertion.
3. `client.ts` types match the Rust JSON shape (no TS-invented field) -> `npm --prefix apps/web run build` (type-check) + manual review against the Rust struct.

## What to Change

### 1. `crates/wasm-api/src/lib.rs`

Serialize the additive explanation fields as part of the normal viewer-scoped terminal view payload; add/extend the bridge test asserting authorized presence and folded-seat absence.

### 2. `apps/web/src/wasm/client.ts`

Add TypeScript view types mirroring the new Rust JSON fields (`RiverLedgerTerminalView` / outcome-rationale types); fields optional to match the additive Rust shape.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)
- `apps/web/src/wasm/client.ts` (modify)

## Out of Scope

- Rendering the fields (RIVLEDSHO-004 panel; RIVLEDSHO-006 cards).
- Rust projection/no-leak in the game crate (RIVLEDSHO-002).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` — bridge test: explanation fields present for authorized reveals, absent for folded/non-revealed seats.
2. `npm --prefix apps/web run build` — `client.ts` types compile against the bridged JSON shape.
3. `cargo test --workspace` — no regression in bridge serialization.

### Invariants

1. The bridge serializes only the viewer-scoped projection; no unauthorized explanation field crosses (§11).
2. `client.ts` declares types only; it computes no category label, hand name, winner, or comparison (§2).

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/lib.rs` — bridge test for the augmented terminal JSON (authorized presence + folded absence).
2. `apps/web/src/wasm/client.ts` — type additions exercised by the web build's type-check.

### Commands

1. `cargo test -p wasm-api`
2. `npm --prefix apps/web run build`
3. The bridge test plus the web type-check are the correct boundary; visual rendering is exercised in RIVLEDSHO-004/005.
