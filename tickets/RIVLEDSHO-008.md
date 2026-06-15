# RIVLEDSHO-008: Action-panel copy from Rust legal-action metadata

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/src/actions.rs`, `apps/web/src/components/RiverLedgerBoard.tsx`
**Deps**: RIVLEDSHO-006

## Problem

The action panel does not clearly surface what each move costs. Call price and the amount added to the ledger already exist in the Rust legal-action metadata and only need rendering; the remaining raise-cap headroom ("cap remaining") is not yet a projected field and must be added Rust-side, never computed in TypeScript (spec WB8 / D7).

## Assumption Reassessment (2026-06-15)

1. Verified against current code: the legal-action choice carries `required_to_call` (`games/river_ledger/src/actions.rs:46`) and `adds_to_pot` (`:47`) — call price and adds-to-ledger are render-only of existing fields. No `cap_remaining`/`raises_remaining` field exists on the choice; the raise cap is computed in `betting.rs` but not projected as headroom, so cap-remaining needs an additive projection field.
2. Verified against specs/docs: spec §6 D7 + §8 WB8; `games/river_ledger/docs/UI.md` §Legal Action Mapping ("Required amount is Rust metadata/payload"; "Cap availability is Rust-owned"); `RULES.md` `RL-BET-CAP-001`.
3. Cross-artifact boundary under audit: the action-choice payload (`actions.rs`) and the `RiverLedgerBoard.tsx` action-panel render; the new cap-remaining field is additive to the choice schema.
4. FOUNDATIONS §2 behavior authority motivates this ticket: cap-remaining is a Rust-projected field; TypeScript renders `required_to_call`/`adds_to_pot`/cap-remaining and computes no amount, legality, or cap arithmetic.
5. Extends the action-tree/choice schema (additive-only: new `cap_remaining`-style field with a default); the consumer is the `RiverLedgerBoard` action panel, updated here. Confirm the field is derived in Rust from the existing cap state and that no other action-choice consumer breaks.

## Architecture Check

1. Modeling cap-remaining as an additive Rust projection field (mirroring `required_to_call`/`adds_to_pot`) keeps all action-affordance numbers Rust-owned — the panel never does cap arithmetic.
2. No backwards-compatibility aliasing/shims; additive field with a default.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4) — game-local action metadata + `apps/web` render.

## Verification Layers

1. The action panel shows call price, adds-to-ledger, and cap-remaining from Rust metadata -> `npm --prefix apps/web run smoke:ui`.
2. cap-remaining is an additive Rust field derived from cap state; existing action-choice consumers unbroken -> `cargo test -p river_ledger --test rules` + `cargo test -p river_ledger`.
3. No TS-computed amount/cap -> grep `RiverLedgerBoard.tsx` action panel for arithmetic on cap/amount (none) + manual review (§2).

## What to Change

### 1. `games/river_ledger/src/actions.rs`

Add an additive cap-remaining field to the legal-action choice, derived from the existing raise-cap state; keep `required_to_call`/`adds_to_pot` as-is.

### 2. `apps/web/src/components/RiverLedgerBoard.tsx`

Render call price (`required_to_call`), adds-to-ledger (`adds_to_pot`), and cap-remaining in the action panel; surface a viewer-safe unavailable reason only when Rust supplies one.

## Files to Touch

- `games/river_ledger/src/actions.rs` (modify)
- `apps/web/src/components/RiverLedgerBoard.tsx` (modify)

## Out of Scope

- Seat / turn-flow affordances (RIVLEDSHO-009).
- Casino-vocabulary copy audit (RIVLEDSHO-011).
- Card component (RIVLEDSHO-006).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger --test rules` — cap-remaining field derived correctly across streets/cap states; existing action-tree tests green.
2. `npm --prefix apps/web run smoke:ui` — action panel shows call price / adds-to-ledger / cap-remaining.
3. `cargo test -p river_ledger` — full crate green (additive field).

### Invariants

1. All action-affordance numbers are Rust-owned; TypeScript computes no amount, legality, or cap arithmetic (§2).
2. The cap-remaining field is additive-only; existing action-choice consumers are unbroken (§11 schema discipline).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/rules.rs` (modify) — cap-remaining derivation across streets and at the cap.
2. `apps/web/scripts/smoke-ui.mjs` (modify, as surfaced) — assert the action-panel cost copy.

### Commands

1. `cargo test -p river_ledger --test rules`
2. `npm --prefix apps/web run smoke:ui`
3. The crate action-tree test plus the UI smoke is the correct boundary; legality is unchanged (presentation + additive projection only).
