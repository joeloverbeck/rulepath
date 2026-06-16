# RIVLEDSHOWUX-003: Rust-authored per-action presentation rows

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/src/betting.rs`, `games/river_ledger/src/actions.rs`, `games/river_ledger/src/ui.rs`, `crates/wasm-api/src/lib.rs`, `apps/web/src/wasm/client.ts`, `apps/web/src/components/RiverLedgerBoard.tsx`
**Deps**: None

## Problem

`RiverLedgerBoard.tsx:314-323` renders `Call price` / `Adds` / `Cap left` rows for *every* action unconditionally, so Fold and Check show decision-irrelevant `Call price 0` / `Cap left 3`. Rust should author per-action display rows (which rows are relevant, plus helper text); TypeScript renders only the supplied rows.

## Assumption Reassessment (2026-06-16)

1. Verified: `RiverLedgerBoard.tsx:314-323` reads raw metadata (`required_to_call`, `adds_to_pot`, `cap_remaining`) and renders all three rows per choice; `cap_remaining` is already a projected metadata field (`:317`).
2. Verified against spec §6 D2 + §8 WB3; `RULES.md` `RL-UI-ACTIONS-001` (Rust owns legal-action metadata).
3. Boundary under audit: the legal-action metadata projection (`crates/wasm-api/src/lib.rs`) — the new per-action presentation object is additive to the existing action-choice metadata; TS stops reading raw counters directly.
4. FOUNDATIONS §2 / §7 motivate this: Rust decides which rows are semantically relevant (Fold/Check carry no `Call price`/`Cap left`); TS renders only what Rust supplies — it computes no relevance.
5. Schema extension: the action-choice projection gains an additive `presentation` object (`segment`, `label`, `helper_text`, `display_rows[]{label,value,tone}`, `accessibility_label`); the consumer is `RiverLedgerBoard.tsx`; additive-only (existing raw metadata retained for compatibility).

## Architecture Check

1. Rust-authored display rows (vs TS deciding relevance) keeps the legal-action contract in Rust; the additive `presentation` object avoids breaking existing metadata consumers.
2. No shims; the TS unconditional-counter block is replaced by render-supplied-rows.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4).

## Verification Layers

1. Fold/Check show no Call price / Cap left -> `npm --prefix apps/web run smoke:ui` + e2e assertion.
2. Rust authors the rows; TS renders only them -> grep `RiverLedgerBoard.tsx` shows no per-action relevance logic (no raw `metadataValue(..., "cap_remaining")` gating).
3. Legal-action metadata unchanged in behavior -> `cargo test -p river_ledger` (action-generation tests).

## What to Change

### 1. `games/river_ledger/src/{betting,actions,ui}.rs`

Author a per-segment presentation object: helper text + the relevant `display_rows` (fold: `Adds 0`; check: `Adds 0`; call: `Call price` + `Adds`; bet: `Adds` + `Raises left`; raise: `Call price` + `Adds` + `Raises left`) + `accessibility_label`.

### 2. `crates/wasm-api/src/lib.rs` + `apps/web/src/wasm/client.ts`

Project the presentation object additively on the action choice; add the matching TS type.

### 3. `apps/web/src/components/RiverLedgerBoard.tsx`

Render only `choice.presentation.display_rows`; remove the unconditional `Call price` / `Adds` / `Cap left` block (`:314-323`).

## Files to Touch

- `games/river_ledger/src/betting.rs` (modify)
- `games/river_ledger/src/actions.rs` (modify)
- `games/river_ledger/src/ui.rs` (modify)
- `crates/wasm-api/src/lib.rs` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/RiverLedgerBoard.tsx` (modify)

## Out of Scope

- Seat-ledger labels (RIVLEDSHOWUX-005); board-slot labels (RIVLEDSHOWUX-004); table recomposition (RIVLEDSHOWUX-011).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — per-action presentation rows authored; Fold/Check carry no call/cap rows.
2. `npm --prefix apps/web run smoke:ui` + `npm --prefix apps/web run build` — TS renders supplied rows; type-checks.
3. `cargo run -p fixture-check -- --game river_ledger` — additive projection passes.

### Invariants

1. Rust authors which rows are relevant; TS computes no relevance (§2, `RL-UI-ACTIONS-001`).
2. Additive only — existing legal-action metadata retained (§11 additive schema).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/rules.rs` — assert fold/check presentation carry no `Call price` / `Cap left` rows.
2. `apps/web/e2e/river-ledger.smoke.mjs` (modify, as surfaced) — Fold/Check action-row assertion.

### Commands

1. `cargo test -p river_ledger`
2. `npm --prefix apps/web run smoke:ui`
3. `cargo run -p fixture-check -- --game river_ledger`

## Outcome

Completed: 2026-06-16

Changed:

- Added River-Ledger-local action presentation rows and helper text for
  `fold`, `check`, `call`, `bet`, and `raise`; Fold/Check carry only the relevant
  `Adds 0` row, while Call/Raise carry call-price/adds rows and Raise/Bet carry
  `Raises left` where relevant.
- Kept the existing raw action metadata intact and added namespaced
  `presentation_*` metadata fields that `crates/wasm-api` projects into an
  additive browser-facing `choice.presentation` object.
- Updated the TypeScript action-choice type and River Ledger action renderer so
  the board renders only Rust-supplied `presentation.display_rows`, with no
  per-action relevance logic in React.
- Added Rust tests for segment-relevant rows and updated the River Ledger smoke
  to prove Fold omits `Call price` / `Raises left` while Raise renders
  `Raises left`.

Deviations:

- The ticket's draft mentioned an additive action-choice schema extension while
  also requiring `engine-core` to remain untouched. The implementation keeps
  `engine-core` unchanged and performs the additive `presentation` projection in
  `wasm-api` from River-Ledger-authored metadata.

Verification:

- `cargo fmt --all --check`
- `cargo test -p river_ledger`
- `cargo run -p fixture-check -- --game river_ledger`
- `npm --prefix apps/web run smoke:ui`
- `node apps/web/e2e/river-ledger.smoke.mjs`
