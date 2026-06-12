# ACTCONMAT-003: Cost/consequence rendering in shared action surfaces

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/ActionPathBuilder.tsx`, `apps/web/src/components/ActionControls.tsx`. No Rust/engine behavior.
**Deps**: ACTCONMAT-001, ACTCONMAT-002

## Problem

The funds-spend in Event Frontier happens silently: operation/target buttons show no cost, and the confirm stage shows neither cost nor resource balance. Rust already emits live, edict-adjusted `cost` leaf metadata and `eligibility_consequence`; the data crosses the boundary and dies unrendered in `ActionPathBuilder`/`ActionControls`. The headline complaint — "funds exist but nothing says what uses them" — must be closed at choice time and confirm time.

## Assumption Reassessment (2026-06-12)

1. `ActionPathBuilder.tsx` currently renders only `choice.label` (no `metadata`/`cost`/`tags`); `ActionControls.tsx` renders `choice.label`/`accessibility_label` with no metadata. Both consume the `ActionChoice` type (`apps/web/src/wasm/client.ts`).
2. Spec D2: choices carrying `cost` render a cost chip; stages carrying `eligibility_consequence` render the resolved template line; the confirm stage renders a summary interpolating cost against the viewer's visible resource balance. The reserved keys + resolved template text are supplied by ACTCONMAT-002; resolved labels by ACTCONMAT-001.
3. Cross-artifact boundary: the `ActionChoice` metadata contract (read side) and the public view's resource balance (read side). No write path; TS interpolates Rust-supplied numbers only.
4. FOUNDATIONS §2: TypeScript presentation-only. The confirm summary ("Spends 2 of your 2 funds") is display interpolation of Rust-supplied `cost` and the public view's balance — no TS arithmetic deciding legality, cost, or eligibility.

## Architecture Check

1. Rendering reserved metadata in the existing shared surfaces (rather than a new component) keeps cost/consequence display co-located with the choices it annotates, and inherits to any future game emitting the reserved keys — the §9 preview doctrine made visible.
2. No shim: this is additive rendering of already-present metadata; no parallel data path.
3. No engine change; `engine-core` untouched. TS computes no legality.

## Verification Layers

1. Cost chip + consequence line render from reserved metadata -> UI smoke (`apps/web/e2e/event-frontier.smoke.mjs`).
2. Confirm summary interpolates cost vs. visible balance without TS legality -> manual review + FOUNDATIONS §2 alignment check.
3. No hidden information rendered -> no-leak visibility test (all rendered facts are viewer-safe leaf metadata / public view fields).

## What to Change

### 1. Cost chips on choices

In `ActionPathBuilder.tsx`/`ActionControls.tsx`, when a choice carries the reserved `cost` key, render a cost chip ("2 funds") before selection.

### 2. Consequence line per stage

When a stage carries `eligibility_consequence`, render the resolved template line (from ACTCONMAT-002) once per stage.

### 3. Confirm-stage summary

Render a confirm summary built from Rust labels, leaf metadata, and the public view's resource balance: "Survey — Charterhouse and Crossing · Spends 2 of your 2 funds · Acting now forfeits your next-card eligibility." Display interpolation only.

## Files to Touch

- `apps/web/src/components/ActionPathBuilder.tsx` (modify)
- `apps/web/src/components/ActionControls.tsx` (modify)
- `apps/web/e2e/event-frontier.smoke.mjs` (modify; assert cost display)

## Out of Scope

- Multi-target composer (ACTCONMAT-004) — this ticket renders cost/consequence on the existing choice flow.
- Any cost/eligibility computation in TS — values come from Rust.
- Reserved-key data/template authoring (ACTCONMAT-002) and label resolution (ACTCONMAT-001).

## Acceptance Criteria

### Tests That Must Pass

1. `apps/web/e2e/event-frontier.smoke.mjs` asserts a cost chip appears on operation choices and the confirm stage states cost against the current balance.
2. `npm --prefix apps/web run smoke:e2e` green.
3. `npm --prefix apps/web run build` typechecks.

### Invariants

1. Every rendered cost/consequence fact is Rust-supplied; TS performs display interpolation only (§2).
2. No raw identifier or hidden fact appears in the rendered chips/summary (§11).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/event-frontier.smoke.mjs` — cost-display + confirm-summary assertions.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `npm --prefix apps/web run build`
3. `npm --prefix apps/web run smoke:ui` (shared-surface render smoke)
