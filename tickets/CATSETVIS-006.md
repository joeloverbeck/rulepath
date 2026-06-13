# CATSETVIS-006: WASM/TS `description?` projection

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs`, `apps/web/src/wasm/client.ts`
**Deps**: 005

## Problem

The variant `description` authored in CATSETVIS-005 lives only in Rust variant data; the catalog JSON and TypeScript type still expose `id` + `label` only. This ticket projects the optional `description` from the three multi-variant games' variant structs through `variant_json`/`variants_json` into `GameVariantCatalogEntry.description?`, **omitting it entirely when absent** (never `null`, never TS-synthesized), and adds a defensive selection helper that returns nothing for an absent/blank value. Spec WB6 / §6 D9; scope is multi-variant only (§4.3).

## Assumption Reassessment (2026-06-13)

1. `variant_json(id: &str, label: &str)` (`crates/wasm-api/src/lib.rs:376`) and `variants_json(variants: &[(&str, &str)])` (`lib.rs:384`) emit `id` + `label` only; `GameVariantCatalogEntry` is `{ id; label }` (`apps/web/src/wasm/client.ts:75-78`). In `list_games()` the three multi-variant games are **struct-fed** (`lib.rs:478-504`, e.g. `flood_variants.deluge.display_name`) while single-variant games are **constant-fed** (`lib.rs:513`) — verified this session. This ticket extends the struct-fed path only.
2. Spec §6 D9 (emit `description` only when present, never `null`, never synthesized in TS; trim defensively and render nothing when absent/blank; never parse it for behavior) and §4.3 (multi-variant scope) govern.
3. Cross-artifact schema extension: `variant_json`/`variants_json` gain an optional `description` parameter fed from the three struct-fed `list_games()` entries; `GameVariantCatalogEntry` gains `description?`. Consumers are the card summary (CATSETVIS-004), the setup variant-description display (CATSETVIS-007), and the `smoke-ui.mjs` shape assertion (CATSETVIS-008). `Deps: 005` (the Rust source field).
4. FOUNDATIONS §2 (behavior authority): Rust owns the projection; TypeScript renders only and never synthesizes a description from a raw `id`. §11 no-leak: the field carries no hidden state.
5. No-leak / fail-closed surface (§11): the projection emits `description` **only when `Some`** — never as `null` — so an absent value is structurally absent from the payload; the TS selector trims and returns `undefined` for blank, and the value is never parsed for conditions/tags/layout. No hidden-information path is added to the catalog payload.
6. Schema extension: `GameVariantCatalogEntry.description?` and the `variant_json`/`variants_json` output are **additive-only** (a new optional field); every existing consumer tolerates its absence, so no consumer breaks.

## Architecture Check

1. Threading `description` through the existing `variant_json`/`variants_json` helpers (struct-fed entries) keeps Rust the projection authority and TypeScript render-only; emitting only-when-present rather than `null` keeps the field genuinely optional and the absent case unambiguous.
2. No backwards-compatibility shims; the field is additive and optional at every layer.
3. `engine-core` untouched; `crates/wasm-api` is the existing JSON bridge — no mechanic noun enters the kernel, no `game-stdlib` change.

## Verification Layers

1. `description` projected only when present → `smoke-ui.mjs` shape assertion (added in CATSETVIS-008) + grep-proof that `variant_json`/`variants_json` carry an optional `description`.
2. Never `null` / never TS-synthesized → manual review of `client.ts` (`description?: string`; the selection helper returns `undefined` for blank, with no fallback synthesis from `id`).
3. No-leak → the `smoke:e2e` no-leak suite + the negative shape assertion (absent variant ⇒ property absent).
4. ABI / build integrity → `smoke:wasm` (the wasm artifact loads with the extended catalog) + `npm run build`.

## What to Change

### 1. `crates/wasm-api/src/lib.rs` projection

Extend `variant_json`/`variants_json` to carry an optional description and emit the `"description"` key only when `Some`; pass the `description` from the three multi-variant games' variant structs in `list_games()`. Single-variant constant-fed tuples are untouched.

### 2. `apps/web/src/wasm/client.ts` type + selector

Add `description?: string` to `GameVariantCatalogEntry`, and a small `selectVariantDescription(variant)` helper that trims and returns `undefined` for absent/blank (consumed by CATSETVIS-004/007).

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify; `variant_json`/`variants_json` + the three struct-fed `list_games()` entries)
- `apps/web/src/wasm/client.ts` (modify; `GameVariantCatalogEntry.description?` + the selection helper)

## Out of Scope

- The Rust `description` field + parser (CATSETVIS-005).
- Single-variant constant-fed projection (§4.3).
- The DOM rendering of the description (card summary CATSETVIS-004, setup display CATSETVIS-007) and the `smoke-ui.mjs` assertion (CATSETVIS-008).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:wasm` is green (the wasm artifact loads with the extended catalog) and `npm --prefix apps/web run build` succeeds.
2. A catalog JSON sample shows `description` **present** on a described multi-variant entry and the property **absent** (not `null`) on an undescribed one.
3. `grep -n 'description' crates/wasm-api/src/lib.rs` shows the optional projection, and `grep -n 'description?' apps/web/src/wasm/client.ts` shows the additive type field.

### Invariants

1. `description` is emitted only when present (never `null`) and never synthesized in TypeScript.
2. TypeScript never parses `description` for conditions, rules, tags, or layout.

## Test Plan

### New/Modified Tests

1. `None — the variant-description shape assertion (present/absent, length, no behavior prose) is authored in CATSETVIS-008's `apps/web/scripts/smoke-ui.mjs`; this ticket is verified by `smoke:wasm` + `build` + the catalog JSON present/absent sample.`

### Commands

1. `npm --prefix apps/web run smoke:wasm`
2. `npm --prefix apps/web run build`
3. `npm --prefix apps/web run smoke:ui` — confirms the extended catalog shape still loads (the precise `description?` assertion is added in CATSETVIS-008).
