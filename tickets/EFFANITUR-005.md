# EFFANITUR-005: bot_vs_bot autoplay + replay stepping on the scheduler

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — TypeScript/React presentation shell only (`apps/web`); Rust/WASM untouched. Replaces a raw `setTimeout` with scheduler-owned pacing; replay command order is unchanged.
**Deps**: EFFANITUR-002, EFFANITUR-004

## Problem

`bot_vs_bot` autoplay paces with a raw `setTimeout` (520 ms, 80 ms reduced; `main.tsx` autoplay `useEffect`) — timing outside the manager, the documented cause of broken fast/replay modes (spec §3). Replay stepping (`stepReplay`/`resetReplay`) exists but does not interrupt through a single flush path. This migrates both onto the scheduler built in EFFANITUR-002, satisfying UI-INTERACTION §14 and WCAG 2.2.2 (spec D4/D6 / WB5).

## Assumption Reassessment (2026-06-12)

1. The autoplay `useEffect` (`apps/web/src/main.tsx`) gates on `state.autoplay.running && state.setup.playMode === "bot_vs_bot"` and schedules `runBotStep` via `window.setTimeout(runBotStep, state.reducedMotion ? 80 : 520)`. `stepReplay` (`:271`) / `resetReplay` (`:280`) dispatch `replayStep`/`replayReset`. `ReplayViewer.tsx` renders replay controls. `ModeControls.tsx` already has `bot_vs_bot` "Step Bot" (`:65`), `onAutoplayPause`, and `autoplayRunning`.
2. Spec D4/D6 / WB5: remove the `setTimeout`; `bot_vs_bot` autoplay becomes the same orchestration machinery with a running/paused flag and speed control; replay stepping interrupts via the scheduler's single flush path; replay playback uses the same pacing (UI-INTERACTION §14 pause/speed/reduced-motion).
3. Cross-artifact boundary under audit: `main.tsx` and `ModeControls.tsx` are shared with EFFANITUR-004 (this ticket `Deps` it, so the orchestration machine already exists); `ReplayViewer.tsx` is this ticket's own. The flush path comes from `scheduler.ts` (EFFANITUR-002).
4. FOUNDATIONS §7/§11 + UI-INTERACTION §14 + WCAG 2.2.2: auto-playing sequences (autoplay, replay playback) expose pause/stop and speed; reduced-motion preserves pacing through the fast path. Pause/stop satisfies WCAG SC 2.2.2 (an auto-advancing bot game is auto-playing content > 5 s).
5. Determinism (FOUNDATIONS §2/§11): replacing the `setTimeout` changes only wall-clock presentation pacing — replay command order, seed derivation, and `replayStep`/`replayReset` results are unchanged. Skip/step uses the scheduler's `finish()` flush (never `cancel()`), so interrupting an in-flight replay step settles to the correct view. No wall-clock enters Rust or canonical forms; a recorded `bot_vs_bot` command-log/replay-export comparison proves byte-identity.

## Architecture Check

1. Routing `bot_vs_bot` and replay pacing through the one manager makes skip/fast a single flush-and-settle path and removes the last ad-hoc timer in the play path — the documented fix for fast/replay breakage, cleaner than per-mode timers.
2. No backwards-compatibility shim: the `setTimeout` is deleted, not kept alongside the scheduler.
3. `engine-core` untouched; pacing is `apps/web`-local presentation timing (§3). No `game-stdlib` promotion.

## Verification Layers

1. No raw `setTimeout` remains in the play path -> codebase grep-proof (`main.tsx` autoplay) + manual review.
2. Autoplay + replay playback expose pause/stop and speed; reduced-motion preserves pacing -> `smoke:e2e` (autoplay pause/speed) + UI-INTERACTION §14 conformance check.
3. Replay stepping interrupts via the single flush path and settles -> `smoke:e2e` replay-interrupt assertion.
4. Replay command order / byte-identity unchanged -> deterministic replay-hash check (recorded `bot_vs_bot` comparison).

## What to Change

### 1. Migrate autoplay to the scheduler

Remove the autoplay `setTimeout` in `main.tsx`; drive `bot_vs_bot` advancement through the orchestration machine (EFFANITUR-004) with a running/paused flag and per-effect-type dwell.

### 2. Speed + pause controls

In `ModeControls.tsx`, add/route speed control and ensure pause/stop is always reachable for `bot_vs_bot` autoplay (WCAG 2.2.2); skip stays live.

### 3. Replay stepping via flush

Route `stepReplay`/`resetReplay` and `ReplayViewer.tsx` interactions through the scheduler's single flush path (interrupt → finish in-flight → settle), and pace replay playback through the same machinery (UI-INTERACTION §14).

## Files to Touch

- `apps/web/src/main.tsx` (modify; shared with EFFANITUR-004)
- `apps/web/src/components/ModeControls.tsx` (modify; shared with EFFANITUR-004)
- `apps/web/src/components/ReplayViewer.tsx` (modify)

## Out of Scope

- The human_vs_bot orchestration machine itself (EFFANITUR-004, this ticket's dependency).
- Per-game animation registrations (EFFANITUR-006/007); the new `animation.smoke.mjs` (EFFANITUR-009).
- Any change to replay/hash semantics or command order (FOUNDATIONS §11/§13 — forbidden; this is pacing only).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e`: `bot_vs_bot` autoplay paces through the scheduler with reachable pause/stop and speed; replay stepping interrupts cleanly via flush.
2. Grep-proof: no raw `setTimeout` remains in the `main.tsx` play path.
3. Recorded byte-identity comparison: `bot_vs_bot` command log / replay export identical before/after.

### Invariants

1. All `bot_vs_bot` and replay pacing flows through the scheduler; no ad-hoc timer in the play path (§7/§11, UI-INTERACTION §10A/§14).
2. Replay command order, seeds, and hashes are unchanged; pacing is droppable presentation (§2/§11).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/` `bot_vs_bot`/replay smoke assertions — autoplay pause/speed + replay-step interrupt (extended; dedicated `animation.smoke.mjs` in EFFANITUR-009).
2. Scripted determinism comparison — `bot_vs_bot` command-log/replay-export byte-identity.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `npm --prefix apps/web run build`
3. Recorded `bot_vs_bot` command-log/replay-export diff (the determinism boundary; no Rust changes expected).
