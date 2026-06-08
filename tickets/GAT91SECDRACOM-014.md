# GAT91SECDRACOM-014: secret_draft TypeScript client bindings + catalog wiring

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/wasm/client.ts`, `apps/web/src/state/shellReducer.ts`, `apps/web/src/main.tsx`, `apps/web/src/components/DevPanel.tsx`, `apps/web/src/components/ReplayViewer.tsx`. No Rust/engine change; TypeScript stays presentation-only.
**Deps**: GAT91SECDRACOM-013

## Problem

The web shell's TypeScript layer must recognize `secret_draft` so the catalog, match lifecycle, action dispatch, effects, views, and replay import/export route through the Rust/WASM bridge. This is type/binding plumbing only — TypeScript decides no legality, scoring, reveal timing, or no-leak filtering.

## Assumption Reassessment (2026-06-08)

1. The non-component TS surfaces that reference `token_bazaar` are (verified by grep): `apps/web/src/wasm/client.ts`, `apps/web/src/main.tsx`, `apps/web/src/components/DevPanel.tsx`, `apps/web/src/components/ReplayViewer.tsx`; `apps/web/src/state/shellReducer.ts` is the shell reducer. These are the catalog/client/state wiring points a new game must extend.
2. The WASM registration (GAT91SECDRACOM-013) exposes `GAME_SECRET_DRAFT` + the per-path arms; this ticket binds the TS client/types/reducer to them. Spec §Deliverables (Browser row): "GamePicker/catalog support through Rust metadata; ActionControls support for simultaneous pending state without TS legality; shell reducer/client type coverage; safe dev panel output; replay import/export wiring."
3. Cross-artifact boundary under audit: the WASM JSON bridge ↔ TS client-binding contract. TS consumes Rust-provided catalog entries, action trees, effects, and views verbatim; it adds no derived legality or hidden-info filtering. DevPanel/ReplayViewer must render only the viewer-safe projection the bridge returns.
4. §2 behavior authority is the motivating principle: restate before trusting spec — TypeScript MUST NOT decide legality, reveal timing, conflict resolution, scoring, terminal detection, tie-breaks, replay authority, bot policy, or no-leak filtering. This ticket only routes Rust-provided data; a TS legality path would be a §12 stop condition.
5. No-leak: the dev panel and replay viewer receive only the same viewer-safe projection + safe command/effect summaries; they must not reconstruct or display pre-reveal committed item IDs.

## Architecture Check

1. Threading the new game through the existing catalog/client/reducer types (driven by Rust metadata) is cleaner than special-casing `secret_draft` in the UI — the shell stays game-agnostic and the Rust catalog remains the single source of truth.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched; TS is presentation-only and adds no mechanic logic. No `game-stdlib` change.

## Verification Layers

1. Catalog wiring -> `secret_draft`/`Veiled Draft` appears in the Rust-driven GamePicker catalog; type-check passes (`tsc --noEmit` via build).
2. Match/action/effect/view routing -> shell reducer + client bindings handle `secret_draft` match lifecycle through the WASM bridge.
3. No-leak (dev panel / replay viewer) -> these surfaces render only viewer-safe projection (manual review; e2e DOM/storage assertions in GAT91SECDRACOM-016).
4. Behavior-authority -> grep/manual review confirms no TS legality/scoring/reveal logic added (§2).

## What to Change

### 1. `apps/web/src/wasm/client.ts`

Add `secret_draft` to the client bindings/types (catalog entry, match/action/view/bot/export calls) routing to the WASM arms from GAT91SECDRACOM-013.

### 2. `apps/web/src/state/shellReducer.ts`

Extend shell state/reducer types to cover the `secret_draft` match lifecycle (pending/reveal phases surfaced via Rust effects/views, not TS-computed).

### 3. `apps/web/src/main.tsx`, `apps/web/src/components/DevPanel.tsx`, `apps/web/src/components/ReplayViewer.tsx`

Wire catalog selection, dev-panel safe output, and replay import/export for `secret_draft`, consuming only viewer-safe projection.

## Files to Touch

- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/state/shellReducer.ts` (modify)
- `apps/web/src/main.tsx` (modify)
- `apps/web/src/components/DevPanel.tsx` (modify)
- `apps/web/src/components/ReplayViewer.tsx` (modify)

## Out of Scope

- The `SecretDraftBoard` renderer, effect-feedback entries, pending/reveal UI, and styles (GAT91SECDRACOM-015).
- e2e smoke, gate-1 CI, and catalog README reconciliation (GAT91SECDRACOM-016).
- Any Rust/WASM change (GAT91SECDRACOM-013).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` (includes `tsc --noEmit`) succeeds with `secret_draft` wired.
2. `npm --prefix apps/web run smoke:wasm` still passes (bindings consume the registered ABI).
3. The Rust-driven catalog lists `Veiled Draft`; no TS legality/scoring/reveal logic is present.

### Invariants

1. TypeScript decides no legality, scoring, reveal timing, or no-leak filtering (§2 behavior authority).
2. Dev panel / replay viewer render only viewer-safe projection (§11 no-leak).

## Test Plan

### New/Modified Tests

1. `None — presentation-only binding ticket; verification is build/type-check plus the existing smoke:wasm ABI coverage. Rendered no-leak assertions land in GAT91SECDRACOM-016.`

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:wasm`
3. Build + `smoke:wasm` is the correct boundary for binding/catalog wiring; rendered-browser behavior is GAT91SECDRACOM-015/016.
