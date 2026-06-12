# EFFANITUR-002: Effect-driven animation scheduler core

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — TypeScript/React presentation shell only (`apps/web`); Rust/WASM untouched. The scheduler's sole input is the existing viewer-filtered `EffectEntry[]`.
**Deps**: EFFANITUR-001

## Problem

No animation scheduler exists; effects render only as text and `bot_vs_bot` autoplay paces with a raw `setTimeout` (`main.tsx`, 520 ms / 80 ms reduced) — exactly the timing-outside-the-manager shape that breaks skip/fast/replay modes (spec §3). The constitution mandates effect-driven animation (FOUNDATIONS §7/§11) and `docs/UI-INTERACTION.md` §10 specs the required behaviors, none of which exist. This ticket builds the single manager that owns all play-path animation timing (spec D1/D4/D5 / WB2).

## Assumption Reassessment (2026-06-12)

1. `apps/web/src/animation/bursts.ts` is created by EFFANITUR-001 and exports the burst segmentation this scheduler queues. Input effects come from `getEffects(matchId, sinceCursor, viewerMode): EffectEntry[]` (`apps/web/src/wasm/client.ts:1306`, type at `:1060-1074`). The persisted reduced-motion state (`system`/`reduce`/`motion`, key `rulepath.reducedMotion`) is read via `state.reducedMotion` (`apps/web/src/state/shellReducer.ts`; type/storage in `apps/web/src/components/effectFeedback.ts`). The Web Animations API (`Element.animate`, `Animation.finished`, `getAnimations()`) is available in the target browsers — no new dependency.
2. Spec D1/D4/D5 / WB2 require: one ordered queue per resolution burst, async promise-based steps advancing on resolution + a declared minimum dwell, manager-owned timing (no stray `setTimeout` in the play path), skip/flush-to-settle, global rate scale, reduced-motion collapse, and settle-to-view hooks. `docs/UI-INTERACTION.md` §10 enumerates the same behavior list (verified: §10 "Effect-log-driven animation").
3. Cross-artifact boundary under audit: the burst contract from `bursts.ts` (EFFANITUR-001) and the `EffectEntry` stream. The scheduler consumes `EffectEntry[]` and burst boundaries only; a combination the stream did not state cannot be animated.
4. FOUNDATIONS §7/§11/§12: the scheduler's input type is the effect stream alone — never renderer state diffs. This is the load-bearing §12 stop condition ("animation depends on guessed state diffs instead of Rust effects"); the scheduler API accepts no state-snapshot input, keeping it structurally clear.
5. Determinism (FOUNDATIONS §2/§11): the scheduler is wall-clock presentation timing that never enters Rust or any canonical form — command logs, traces, replays, and hashes are unaffected. Skip/flush uses `Animation.finish()` (commits end state), never `cancel()` (discards), so an interrupt at any instant settles correctly. No RNG, no `std::time` in replayed forms; fake-timer unit tests use `node:test` mock timers for deterministic dwell assertions.

## Architecture Check

1. A single promise-based queue (the BGA `notifqueue` / `setupPromiseNotifications` model) over the server-ordered effect stream is the platform-scale norm and makes skip a single flush-and-settle path rather than "1 ms durations everywhere" — directly avoiding the documented fast/replay breakage. All play-path timing flowing through one manager is cleaner and more debuggable than scattered ad-hoc waits.
2. No backwards-compatibility shim: the manager is the new single owner; the `bot_vs_bot` `setTimeout` is migrated out in EFFANITUR-005, not kept in parallel.
3. `engine-core` untouched; the scheduler is `apps/web`-local presentation infrastructure (§3). No `game-stdlib` promotion.

## Verification Layers

1. Queue advances on promise resolution + declared dwell; ordering preserved -> node smoke with `node:test` mock timers (`smoke-scheduler.mjs`).
2. Skip/flush finishes (never cancels) in-flight animation and drains remaining steps to settle -> mock-timer smoke asserting `finish()`-path and final settled state.
3. Global rate scale + reduced-motion collapse to instant-plus-feedback -> mock-timer smoke over both modes.
4. No state-diff input path exists -> codebase grep-proof (scheduler API accepts `EffectEntry[]`/bursts only).
5. Determinism unaffected -> FOUNDATIONS alignment check (wall-clock confined to presentation; no canonical-form input).

## What to Change

### 1. Scheduler module

Add `apps/web/src/animation/scheduler.ts`: an ordered queue per resolution burst (from `bursts.ts`); async step handlers returning promises; advancement on resolution + a declared minimum per-step dwell; a settle hook invoked after each drain/flush/interruption.

### 2. Flush / skip / rate control

Implement the single flush path — `getAnimations().forEach(a => a.finish())`, drain remaining steps instantly, settle — used by skip, interruption, and teardown. Add a global `playbackRate`-style rate scale applied through the manager. Never use `cancel()` to skip.

### 3. Reduced-motion collapse

Under reduced motion (system query or persisted override) collapse every step to instant transition plus a brief non-motion feedback slot, preserving dwell pacing at the existing fast values.

### 4. Scheduler unit smoke

Add `apps/web/scripts/smoke-scheduler.mjs` using `node:test` mock timers for deterministic queue/flush/rate/reduced-motion assertions (npm-script/CI wiring consolidated in EFFANITUR-009).

## Files to Touch

- `apps/web/src/animation/scheduler.ts` (new)
- `apps/web/scripts/smoke-scheduler.mjs` (new)

## Out of Scope

- The presentation realization (WAAPI/FLIP/ghost overlay, generic tone-keyed presentations) — EFFANITUR-003.
- Orchestration / bot decoupling (EFFANITUR-004) and `bot_vs_bot`/replay migration (EFFANITUR-005).
- Any Rust change or any read beyond the viewer-filtered effect stream.

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/scripts/smoke-scheduler.mjs` — ordered advancement, flush-to-settle via `finish()`, rate scale, and reduced-motion collapse all pass under mock timers.
2. Grep-proof: `scheduler.ts` contains no `cancel(` skip path and no acceptance of a state-snapshot argument (effect-stream/burst input only).
3. `npm --prefix apps/web run build` green (module type-checks against `bursts.ts`).

### Invariants

1. All play-path animation timing flows through this manager; the scheduler input is the effect stream/bursts alone (§7/§11/§12).
2. Skip/interruption finishes (never discards) in-flight animation and settles to the latest view; wall-clock never enters Rust/canonical forms (§2/§11).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-scheduler.mjs` — `node:test` mock-timer suite over queue, flush, rate, reduced-motion.

### Commands

1. `node apps/web/scripts/smoke-scheduler.mjs`
2. `npm --prefix apps/web run build`
3. A narrower targeted node smoke (not `smoke:e2e`) is the correct boundary here because the scheduler is pure timing logic with no DOM/browser surface yet — DOM realization is exercised by EFFANITUR-003/009.
