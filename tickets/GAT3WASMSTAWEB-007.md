# GAT3WASMSTAWEB-007: Effect log, effect-driven feedback, and reduced motion

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — TypeScript/presentation only (`apps/web`); renders Rust semantic effects.
**Deps**: 004, 006

## Problem

Action feedback and the event log must be driven by Rust semantic effects, not
TypeScript state diffs (spec §14.1, FOUNDATIONS §7/§11 "semantic effects drive
animation; renderer diffs are diagnostics only"). The shell must also honor
`prefers-reduced-motion` (§14.4, §17.5). Today `main.tsx` keeps a flat
`effects: string[]` rendered as an `<ol>` and has no reduced-motion handling. This
ticket builds the effect log + effect-mapped feedback + reduced-motion preference
on the reducer.

## Assumption Reassessment (2026-06-06)

1. `apps/web/src/main.tsx` describes effects via `describeEffect(entry)` over the
   `EffectEntry` payload `type`s (`action_started`, `counter_advanced`,
   `turn_changed`, `game_ended`, `action_completed`, lines ~210–226) and renders the
   last 12 as text. These five `RaceEffect` kinds match `crates/wasm-api/src/lib.rs`
   `effect_json` (lines ~324–353). No reduced-motion code exists.
2. Spec §14.2 maps each effect kind to simple feedback; §14.3 requires a
   chronological viewer-safe effect log distinguishing new entries, avoiding raw
   JSON, usable with reduced motion, handling cursor reset; §14.4/§14.5 require
   reduced-motion to preserve all information without animation and the renderer to
   settle to the Rust view; §17.5 allows persisting a safe local reduced-motion
   preference.
3. Cross-artifact boundary under audit: the effect log consumes `EffectEntry` from
   `apps/web/src/wasm/client.ts` (GAT3WASMSTAWEB-001) and the effect cursor/queue
   reducer state (GAT3WASMSTAWEB-004); it mounts beside the renderer
   (GAT3WASMSTAWEB-006). Effect cursor handling MUST tolerate reset/restart cleanly.
4. FOUNDATIONS §7/§11 (effects drive animation): restated — feedback derives from
   Rust `RaceEffect` payloads, never from comparing successive views; view diffs may
   be a diagnostic only. After feedback, the renderer settles to the Rust view.

## Architecture Check

1. An `EffectLog` component reading the reducer's effect queue + a small
   effect→feedback mapping keeps Rust effects as the single feedback authority and
   isolates reduced-motion behavior, versus diffing views in components.
2. No backwards-compatibility shims: the inline `<ol>` is replaced by the component;
   no parallel effect-display path remains.
3. `engine-core` untouched; no mechanic logic in React; `game-stdlib` untouched.

## Verification Layers

1. Feedback is effect-driven → codebase grep-proof: feedback maps `RaceEffect`
   payload types; no view-to-view diffing drives animation in `apps/web/src/components`.
2. Reduced motion preserves information → manual review + simulation: with reduced
   motion on, counter/turn/winner/effect text remain present (no motion-only info);
   `prefers-reduced-motion` honored and a safe local override persists.
3. Effect cursor reset handled → simulation/CLI run: `smoke:ui` restart clears and
   rebuilds the log without stale entries.
4. Settle-to-view → FOUNDATIONS §7/§11 alignment check (§14.5).

## What to Change

### 1. New `apps/web/src/components/EffectLog.tsx`

Chronological viewer-safe effect summaries from the reducer effect queue, marking
recent entries, no raw JSON, reduced-motion-friendly, cursor-reset-safe.

### 2. New `apps/web/src/components/effectFeedback.ts` (+ reduced-motion hook)

Map each `RaceEffect` kind to calm feedback; a `usePrefersReducedMotion` hook reads
`prefers-reduced-motion` and a persisted safe local override; under reduced motion,
use instant updates/text/highlight instead of motion.

### 3. `apps/web/src/state/shellReducer.ts` + `apps/web/src/main.tsx`

Effect cursor/queue transitions (append since-cursor, reset on match/replay start)
and reduced-motion preference state; mount `EffectLog`.

## Files to Touch

- `apps/web/src/components/EffectLog.tsx` (new)
- `apps/web/src/components/effectFeedback.ts` (new)
- `apps/web/src/state/shellReducer.ts` (modify) — effect cursor/queue + reduced-motion transitions
- `apps/web/src/main.tsx` (modify) — mount effect log; wire reduced-motion

## Out of Scope

- Autoplay/bot-vs-bot pacing (consumes reduced motion) — GAT3WASMSTAWEB-008.
- Replay-cursor effect display — GAT3WASMSTAWEB-009.
- Dev-panel effect-cursor inspector — GAT3WASMSTAWEB-010.
- Full a11y audit — GAT3WASMSTAWEB-014.

## Acceptance Criteria

### Tests That Must Pass

1. `cd apps/web && npm run build` — typecheck + Vite build succeed.
2. `cd apps/web && npm run smoke:ui` — effect log appears, updates from Rust effects, and clears cleanly on restart.
3. `grep -rnE "prevView|previousView|diff" apps/web/src/components/effectFeedback.ts` — returns nothing (feedback is effect-driven, not view-diff-driven).

### Invariants

1. Animation/feedback is driven by Rust semantic effects; view diffs are diagnostics only.
2. No information is conveyed by motion alone; `prefers-reduced-motion` is honored.

## Test Plan

### New/Modified Tests

1. `None — UI ticket; verification is `smoke:ui` + `tsc`; reduced-motion/no-leak browser assertions land in GAT3WASMSTAWEB-013/-014.`

### Commands

1. `cd apps/web && npm run build`
2. `cd apps/web && npm run smoke:ui`
3. Reduced-motion emulation in a real browser is the Puppeteer harness's job (GAT3WASMSTAWEB-013); node smoke + the effect-driven grep-proof are the correct boundary here.
