# EFFANITUR-001: Shared burst-segmentation module + TurnReport/EffectLog re-base

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — TypeScript/React presentation shell only (`apps/web`); Rust/WASM untouched. Consumes the existing viewer-filtered `EffectEntry` stream unchanged.
**Deps**: None

## Problem

There is no single definition of a "resolution burst" (the effects between two player decision points). `TurnReportPanel.tsx` approximates one with a naive "last 6 non-marker effects" heuristic (`latestReportBurst`, `apps/web/src/components/TurnReportPanel.tsx:47`, `MAX_REPORT_EVENTS = 6`), and `EffectLog.tsx` renders the full flat history with no grouping. The forthcoming scheduler (EFFANITUR-002) needs an authoritative burst segmentation to play, and both narration surfaces should consume the same definition instead of re-deriving it. This is the foundation every downstream animation ticket builds on (spec D9 / WB1; absorbs brainstorm C8's effect-log residue).

## Assumption Reassessment (2026-06-12)

1. `apps/web/src/animation/` does not exist yet (confirmed: `find apps/web/src/animation` → no such dir); this ticket creates it with `bursts.ts`. `TurnReportPanel.tsx:47` `latestReportBurst` slices the last `MAX_REPORT_EVENTS=6` non-marker effects; decision markers are `action_started`/`choice_taken`/`bot_chose_action`/`bot_chose_action_public` (`isDecisionMarker`, `:52`). `EffectLog.tsx` maps the flat `effects` array (`:31-50`) with a reduced-motion `<select>`. Both consume `EffectEntry` (`apps/web/src/wasm/client.ts:1060-1074`).
2. Spec D9 / WB1 require one burst-segmentation module defining "resolution burst" once, consumed by the scheduler, `TurnReportPanel` (replacing last-6), and `EffectLog` (grouped browsable history). No new data crosses the boundary — segmentation is computed from the existing decision-marker effect kinds already present in the stream.
3. Cross-artifact boundary under audit: the `EffectEntry` stream contract (`client.ts` `getEffects` → `EffectEntry[]`) as consumed by three presentation surfaces. The burst module reads only `effect.payload.type` (already public, viewer-filtered); it introduces no new payload field and no read of any non-projected value.
4. FOUNDATIONS §7/§11 (semantic effects drive animation): burst boundaries derive from Rust-emitted decision-marker effect kinds, never from renderer state diffs or TS-inferred causality. This keeps the §12 stop condition ("animation depends on guessed state diffs") clear at the segmentation layer that the scheduler will consume.

## Architecture Check

1. One shared `bursts.ts` consumed by three surfaces replaces two independent ad-hoc derivations (the last-6 heuristic and the flat log), giving the scheduler and both panels a single source of truth — cleaner and less drift-prone than re-deriving grouping per surface.
2. No backwards-compatibility shim: the last-6 heuristic is removed and replaced by real burst boundaries, not aliased behind a flag.
3. `engine-core` untouched (no Rust change); animation vocabulary lives in `apps/web` per FOUNDATIONS §3. No `game-stdlib` promotion.

## Verification Layers

1. Burst boundaries split on decision-marker effect kinds (human / bot / automated advances) -> node smoke over crafted `EffectEntry[]` fixtures (`smoke-bursts.mjs`).
2. `TurnReportPanel` renders the latest burst from the shared module, not the last-6 slice -> codebase grep-proof (no `MAX_REPORT_EVENTS` slice survives) + `smoke:ui`.
3. `EffectLog` renders grouped, labeled bursts -> `smoke:ui` browsable-history assertion.
4. No new payload field / no hidden-information read -> codebase grep-proof (burst module reads only `effect.payload.type`).

## What to Change

### 1. Burst-segmentation module

Add `apps/web/src/animation/bursts.ts`: a pure function segmenting an `EffectEntry[]` into ordered resolution bursts, split at decision-marker boundaries (one burst per human action / bot turn / automated phase), with a stable per-burst label derived from the marker. Export both the full segmentation and a "latest burst" selector.

### 2. TurnReportPanel re-base

Replace `latestReportBurst`/`MAX_REPORT_EVENTS` in `TurnReportPanel.tsx` with the shared "latest burst" selector. Preserve the existing `ADOPTED_GAMES` gating and rendering.

### 3. EffectLog grouping

Re-base `EffectLog.tsx` to render the segmented bursts as grouped, labeled, browsable history (the absorbed C8 residue) rather than a flat list. Keep the reduced-motion `<select>`.

### 4. Burst unit smoke

Add `apps/web/scripts/smoke-bursts.mjs` asserting boundary placement across human/bot/automated fixtures (npm-script/CI wiring consolidated in EFFANITUR-009).

## Files to Touch

- `apps/web/src/animation/bursts.ts` (new)
- `apps/web/src/components/TurnReportPanel.tsx` (modify)
- `apps/web/src/components/EffectLog.tsx` (modify)
- `apps/web/scripts/smoke-bursts.mjs` (new)

## Out of Scope

- The scheduler itself (EFFANITUR-002) — this ticket only defines bursts and the two narration consumers.
- Any animation/motion (EFFANITUR-003); generic or per-game presentations.
- Full history-browsing / time-travel / click-to-inspect-past-state UX (spec §4.3 deferred).
- Any change to the `EffectEntry` payload or the `getEffects` projection.

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/scripts/smoke-bursts.mjs` — bursts split exactly at decision markers across human-action, bot-turn, and automated-phase fixtures.
2. Grep-proof: `MAX_REPORT_EVENTS` and the last-6 slice no longer exist in `TurnReportPanel.tsx`.
3. `npm --prefix apps/web run smoke:ui` green (TurnReport latest-burst + EffectLog grouped rendering).

### Invariants

1. Burst boundaries derive only from Rust-emitted effect kinds; no renderer state-diff inference (§7/§11).
2. The burst module reads only already-public `effect.payload.type`; no new payload field, no hidden-information read (§11).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-bursts.mjs` — boundary placement over crafted `EffectEntry[]` fixtures.
2. `apps/web/scripts/smoke-ui.mjs` — extend for grouped EffectLog + latest-burst TurnReport (as the `smoke:ui` harness covers these surfaces).

### Commands

1. `node apps/web/scripts/smoke-bursts.mjs`
2. `npm --prefix apps/web run smoke:ui`
3. `npm --prefix apps/web run build` (type-check the new module + re-based consumers)
