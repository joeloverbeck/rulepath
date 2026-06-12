# ACTCONMAT-009: Match-setup variant selector + picker hygiene

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs` (catalog projection of per-variant `{id, label}` objects); plus presentation (`apps/web/src/wasm/client.ts` `GameCatalogEntry.variants` object type, `apps/web/src/main.tsx`, `apps/web/src/components/GamePicker.tsx`).
**Deps**: None

## Problem

No variant selector exists, so `event_frontier_hard_winter`, `event_frontier_land_rush` (and `frontier_control_highlands`, `flood_watch_deluge`) are unreachable in the UI. Picker cards and the setup panel show raw engine strings ("rules 1 / schema 1", raw variant IDs), and the picker card is clickable only on its inner button, not the whole card surface. Multi-variant games must be startable in every variant, with engine strings out of normal mode.

## Assumption Reassessment (2026-06-12)

1. The catalog projection (`crates/wasm-api/src/lib.rs:406-482`) emits each game's `variants` as a flat array of bare ID strings; `GameCatalogEntry.variants` is `string[]` (`apps/web/src/wasm/client.ts:73`). `GamePicker.tsx` renders `rules {rules_version} / schema {schema_version} / {variants.join(", ")}` and consumes `game.variants` as strings; the `.game-option` button spans the card body but the `.game-card` wrapper itself is not the click target, and a separate "How to Play" button sits beside it. Per-variant `*_display_name` exists in `variants.toml` (e.g. `hard_winter_display_name = "Event Frontier: Hard Winter"`) but is NOT projected.
2. Spec D9 / §4.2 / A9: variant display labels are projected Rust-side from the `*_display_name` already in `variants.toml`; descriptions are out of scope (labels only). Extend the catalog projection to carry per-variant `{id, label}` objects and the matching `GameCatalogEntry.variants` type; default remains the standard variant; drop "rules N / schema N" and raw variant IDs from normal mode (dev panel keeps them); make the whole `.game-card` the click target while preserving the separate "How to Play" button.
3. Cross-artifact boundary under audit: the wasm catalog JSON projection (producer `crates/wasm-api/src/lib.rs`) and `GameCatalogEntry` (`client.ts`). Shared files `crates/wasm-api/src/lib.rs` and `apps/web/src/main.tsx` are also touched by ACTCONMAT-005 (faction labels) — coordinate the mechanical merge.
4. FOUNDATIONS §2: variant labels are Rust-sourced (`variants.toml` → catalog projection); TypeScript renders them and never invents variant presentation strings.
5. Schema restructure: `GameCatalogEntry.variants` changes from `string[]` to an array of `{id, label}` objects. Consumers asserting `variants: string[]` — `GamePicker.tsx` (`variants.join`, `variants.length`) and any `main.tsx` setup use — MUST update in this ticket. This is a breaking restructure of an existing field, so all consumers are enumerated in Files to Touch.

## Architecture Check

1. Projecting `{id, label}` from `variants.toml` keeps variant presentation Rust-authored (the catalog is the single source of truth) rather than mapping IDs to labels in TS. Reusing the existing catalog-const projection avoids a parallel variant-metadata channel.
2. No shim: `variants` becomes objects outright; no dual string|object union. Consumers are migrated in the same ticket.
3. `engine-core` untouched — variant metadata is `games/*` data projected by `wasm-api`. No `game-stdlib` change.

## Verification Layers

1. Per-variant `{id, label}` reaches the catalog -> schema/serialization validation of the wasm catalog JSON + `client.ts` typecheck.
2. Every multi-variant game startable in every variant -> UI smoke (`apps/web/e2e/event-frontier.smoke.mjs`: start a non-standard variant).
3. No "rules N / schema N" / raw variant IDs in normal mode; whole-card click -> codebase grep-proof + UI smoke.

## What to Change

### 1. Catalog projection of variant objects

In `crates/wasm-api/src/lib.rs`, project each game's variants as `{id, label}` objects (label from `variants.toml` `*_display_name`).

### 2. client.ts type

Change `GameCatalogEntry.variants` to the object array; update consumers.

### 3. Variant selector + picker hygiene

In `main.tsx`/`GamePicker.tsx`: add a typed variant selector (default standard); remove "rules N / schema N" and raw variant IDs from normal mode (dev panel keeps them); make the whole `.game-card` the click target while preserving the "How to Play" button.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify; per-variant `{id, label}` projection)
- `apps/web/src/wasm/client.ts` (modify; `GameCatalogEntry.variants` object type)
- `apps/web/src/main.tsx` (modify; variant selector wiring)
- `apps/web/src/components/GamePicker.tsx` (modify; engine-string removal, whole-card click, object consumption)
- `apps/web/e2e/event-frontier.smoke.mjs` (modify; start a non-standard variant)

## Out of Scope

- Variant *descriptions* (out of scope per A9 — labels only; no description source exists).
- Faction/seat labels in the catalog (ACTCONMAT-005), though it shares `wasm-api/lib.rs` and `main.tsx`.
- Picker visual redesign (card art/layout) — deferred successor per spec §13.

## Acceptance Criteria

### Tests That Must Pass

1. `apps/web/e2e/event-frontier.smoke.mjs` starts a non-standard variant (e.g. `event_frontier_hard_winter`) from the UI.
2. Grep-proof: no "rules N / schema N" or raw variant ID in normal-mode picker/setup copy; whole `.game-card` is the click target.
3. `npm --prefix apps/web run build` (typecheck the variants-object restructure) + `npm --prefix apps/web run smoke:e2e` green.

### Invariants

1. Variant labels are Rust-sourced via the catalog projection; TS never invents them (§2).
2. The `variants` restructure migrates every consumer in this ticket — no `string[]` assertion remains.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/event-frontier.smoke.mjs` — non-standard-variant start + picker-hygiene assertions.
2. wasm catalog smoke — `npm --prefix apps/web run smoke:wasm` exercises the catalog JSON shape.

### Commands

1. `cargo test -p wasm-api`
2. `npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run build`
3. `npm --prefix apps/web run smoke:e2e`

## Outcome

Projected catalog variants as `{id, label}` objects from Rust-authored variant
data and migrated the TypeScript catalog type/consumers away from `string[]`.
Added `rulepath_new_match_with_variant` while preserving the default
`rulepath_new_match` path, and wired setup state so multi-variant games start
the selected Rust variant.

Added a normal-mode variant selector in setup, removed rules/schema/raw variant
IDs from picker/setup copy, and made the `.game-card` wrapper mouse-clickable
while keeping the existing game button as the keyboard target and the separate
How-to-Play button intact.

Updated the wasm smoke scripts and Event Frontier browser smoke for the catalog
object shape, picker/setup hygiene, and a UI start of
`event_frontier_hard_winter`.

Verification passed:

1. `cargo test -p wasm-api`
2. `git diff --check`
3. `npm --prefix apps/web run smoke:wasm`
4. `npm --prefix apps/web run build`
5. `node apps/web/e2e/a11y-noleak.smoke.mjs` after the initial tab-order fix
6. `npm --prefix apps/web run smoke:e2e`
