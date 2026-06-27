# GAT192MELLEDSET-002: WASM bridge + `client.ts` settlement-view type

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `crates/wasm-api` (`src/games/meldfall.rs`); `apps/web/src/wasm/client.ts`
**Deps**: GAT192MELLEDSET-001

## Problem

GAT192MELLEDSET-001 adds `last_settlement: Option<MeldfallSettlementView>` to the
Rust `MeldfallView`, but the WASM bridge does not yet serialize it to JSON and the
TypeScript view type does not yet declare it, so the renderer cannot read it. This
ticket bridges the new Rust field across the WASM boundary and adds the matching
TypeScript type (spec §3.1.4, §4, §6.2).

## Assumption Reassessment (2026-06-27)

1. `meldfall_view_json(view: &MeldfallView, freshness_token: u64) -> String`
   (`crates/wasm-api/src/games/meldfall.rs:364`) hand-builds the view JSON with an
   explicit field list (it already serializes `round_end` at the `"round_end":{}`
   position). The new `last_settlement` is serialized here from
   `view.last_settlement`, mirroring the nullable shape (`null` when `None`).
2. `MeldfallLedgerPublicView` (`apps/web/src/wasm/client.ts:1181-1199`) is the TS
   mirror of the view JSON; it already declares `round_end: string | null`. Add the
   new `MeldfallLedgerSettlementView` type and a `last_settlement:
   MeldfallLedgerSettlementView | null` field. No legality or settlement math is
   added to TypeScript — it is a structural type mirror only (`ML-UI-001`, §2).
3. Cross-artifact boundary under audit: the WASM JSON contract between
   `meldfall_view_json` (producer) and `MeldfallLedgerPublicView` (consumer). The
   field names, ordering convention, and nullability must match exactly so the TS
   type faithfully describes the emitted JSON. This is the only shared surface; the
   Rust projection it carries is owned by GAT192MELLEDSET-001.
4. §11 no-leak restated at the bridge: the bridge serializes only the
   already-viewer-safe `MeldfallSettlementView` (totals/counts per `ML-VIS-006`); it
   adds no field and performs no re-derivation, so it introduces no leak path. The
   `hidden_fields` marker list in the emitted JSON is unchanged.
5. Schema extension: the view JSON and its TS type gain one additive nullable
   field. Consumer of the TS type: `MeldfallLedgerBoard.tsx` — updated in
   GAT192MELLEDSET-003, not here. Additive-only; no existing field changes.

## Architecture Check

1. Serializing the Rust-owned projection verbatim and mirroring it as a TS type is
   cleaner than computing any settlement detail in the bridge or the renderer — the
   value crosses the boundary already viewer-safe and already complete.
2. No backwards-compatibility shim: the JSON field and TS field are additive and
   nullable; existing consumers ignoring `last_settlement` are unaffected.
3. `engine-core` untouched; no `game-stdlib` change; the WASM bridge stays a
   presentation transport, not a behavior author.

## Verification Layers

1. JSON↔type fidelity: the emitted `last_settlement` JSON conforms to
   `MeldfallLedgerSettlementView` -> `npm --prefix apps/web run smoke:wasm` /
   typecheck against the produced view; manual review of field-name parity.
2. Nullability: `last_settlement` is `null` before any round settles and an object
   afterward -> WASM smoke / e2e capture (exercised end-to-end in GAT192MELLEDSET-003).
3. No-leak: the bridge emits no field outside the `ML-VIS-006` allow-list ->
   `a11y-noleak` smoke (GAT192MELLEDSET-003) + manual review that no new hidden
   field is serialized.

## What to Change

### 1. Serialize `last_settlement` (`crates/wasm-api/src/games/meldfall.rs`)

In `meldfall_view_json`, serialize `view.last_settlement` as a nullable JSON
object (round index, round-end reason, per-seat tabled/penalty/delta/cumulative/
rank/winner), emitting `null` when `None`. Follow the existing hand-built
serialization style and escaping helpers used for `round_end` and the seat arrays.

### 2. TypeScript type mirror (`apps/web/src/wasm/client.ts`)

Add `MeldfallLedgerSettlementView` (round index, round-end reason string, and the
per-seat numeric/boolean breakdown fields) and add `last_settlement:
MeldfallLedgerSettlementView | null` to `MeldfallLedgerPublicView`.

## Files to Touch

- `crates/wasm-api/src/games/meldfall.rs` (modify) — serialize `last_settlement` in `meldfall_view_json`
- `apps/web/src/wasm/client.ts` (modify) — `MeldfallLedgerSettlementView` type + field on `MeldfallLedgerPublicView`

## Out of Scope

- The Rust projection / retention itself — GAT192MELLEDSET-001.
- Rendering the field or retiring the effects-buffer capture — GAT192MELLEDSET-003.
- Any settlement math in TypeScript (`ML-UI-001`, §2) — the type is a structural mirror only.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:wasm` — the WASM build loads and the emitted
   view parses, including `last_settlement`.
2. `cargo test --workspace` — bridge change compiles and existing wasm-api tests pass.
3. TypeScript build is clean (`npm --prefix apps/web run build`).

### Invariants

1. The emitted `last_settlement` JSON is structurally identical to
   `MeldfallLedgerSettlementView`; field names and nullability match the Rust shape.
2. The bridge serializes only `ML-VIS-006`-authorized fields; the `hidden_fields`
   marker list is unchanged.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/games/meldfall.rs` — extend the existing view-JSON
   serialization test(s) to cover the `last_settlement` object and its `null` case.

### Commands

1. `npm --prefix apps/web run smoke:wasm`
2. `npm --prefix apps/web run build`
3. `cargo test --workspace`
4. The wasm smoke + TS build are the correct boundary for the JSON↔type contract;
   end-to-end rendering is exercised in GAT192MELLEDSET-003.
