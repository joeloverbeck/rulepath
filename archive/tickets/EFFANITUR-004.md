# EFFANITUR-004: Turn orchestration in human_vs_bot (auto-advance, remove manual trigger)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — TypeScript/React presentation shell only (`apps/web`). Orchestration changes *when* `runBotTurn` is called and *how* its effects render; it never changes what the bot decides, the seed derivation, or the submitted command bytes (FOUNDATIONS §2).
**Deps**: EFFANITUR-002

## Problem

In `human_vs_bot`, `api.runBotTurn(...)` is called synchronously inside the same `playChoice`/`playPath` callback as the human's `applyAction`, with one `refresh` at the end (`apps/web/src/main.tsx:162-174`, `:194-206`) — the human action and the bot's whole reply land as one instant state swap. A manual "Run Bot Turn" control (`ModeControls.tsx:58`) papers over bot-first starts and consecutive bot turns. This closes the predecessor's surviving audit findings O5 (auto-resolved phases happen invisibly) and O11 (bot turns require a manual trigger with no narration of pacing) (spec D6 / WB4).

## Assumption Reassessment (2026-06-12)

1. `playChoice`/`playPath` (`apps/web/src/main.tsx:150-212`) call `api.applyAction(...)` then synchronously `api.runBotTurn(matchId, afterHumanSeat, botSeed(afterHuman))` (`:171`, `:203`) then one `refresh`. `runBotStep` (`:227`) calls `runBotTurn(matchId, view.active_seat, botSeed(view))` and is wired to `ModeControls` via `onBotStep={runBotStep}` (`:566`). `ModeControls.tsx` renders "Run Bot Turn" for `human_vs_bot` (`:58`, button uses `onBotStep`/`canRunBot`). `shellReducer.ts` holds shell state (incl. `state.reducedMotion`, `state.pendingOperation`).
2. Spec D6 / WB4: decouple the bot turn from the human's synchronous frame so it plays after the human's effects settle, with authored dwell; auto-advance consecutive bot turns and bot-first starts; remove "Run Bot Turn"; add skip + pause controls; prove command-log/trace byte-identity before/after.
3. Cross-artifact boundary under audit: the scheduler (EFFANITUR-002) drives a new shell-level orchestration state machine in `shellReducer.ts`/`main.tsx`. `ModeControls.tsx` is shared with EFFANITUR-005 (which reworks `bot_vs_bot`'s `onBotStep`/"Step Bot" and autoplay controls) — coordinate the mechanical merge; EFFANITUR-005 `Deps` this ticket.
4. FOUNDATIONS §2/§11 (behavior authority): TS decides only *when to call* `runBotTurn` and *how to draw* its effects — presentation policy. No legality, outcome, or effect content is computed in TS. The bot still decides through the same API with the same seed.
5. Determinism (FOUNDATIONS §2/§11): `runBotTurn` is invoked with the same `botSeed(...)` derivation against the same state as today; only its position in the frame timeline changes. Command submission order, seed derivation, and serialization stay byte-identical — proven by a recorded command-log/replay-export comparison of a scripted `human_vs_bot` session before/after. Wall-clock dwell never enters Rust.
6. Removed surface — "Run Bot Turn": the control and its `onBotStep`/`canRunBot` path for `human_vs_bot` are removed (not aliased). Repo-wide grep (`apps/web/src`) confirms the only consumers are `ModeControls.tsx` (`:58`) and `main.tsx` (`:566` wiring); `bot_vs_bot`'s separate "Step Bot" button (`ModeControls.tsx:65`) is reworked under EFFANITUR-005, not here. No `docs/`/`specs/` reference depends on the manual trigger existing (spec A5: removed outright, no replacement setting).

## Architecture Check

1. A shell-level orchestration state machine driven by the scheduler — bot turn runs after the human burst settles, with per-effect-type dwell — is cleaner than the synchronous inline call: it makes auto-resolved phases visible (O5) and removes the manual trigger (O11) while keeping React state the settled truth at every instant.
2. No backwards-compatibility shim: the synchronous `runBotTurn` call and the "Run Bot Turn" control are removed, not kept behind a flag (spec A5).
3. `engine-core` untouched; orchestration is `apps/web`-local presentation timing (§3). No `game-stdlib` promotion.

## Verification Layers

1. Bot turn decoupled from the human frame; plays after the human burst settles with authored dwell -> `smoke:e2e` (`event-frontier.smoke.mjs`) auto-advance assertion.
2. Bot-first starts + consecutive bot turns auto-advance with no click; no "Run Bot Turn" control -> codebase grep-proof (control removed) + `smoke:e2e`.
3. Skip always available; acting mid-animation flushes-then-submits (never hard-blocks) -> `smoke:e2e` input-not-blocked assertion (via the shared flush path from EFFANITUR-002).
4. Command-log/trace/replay byte-identity before/after -> deterministic replay-hash check (recorded scripted-session comparison).
5. Behavior authority preserved -> FOUNDATIONS §2 alignment check (TS times the call; Rust decides).

## What to Change

### 1. Orchestration state machine

Add orchestration/pacing state to `shellReducer.ts` (running/paused, current burst, pending bot advance) and a shell-level machine in `main.tsx`: after a human action's burst settles, if the active seat is a bot, run its turn after an authored dwell and animate its burst; chain consecutive bot turns; auto-advance bot-first starts.

### 2. Decouple the bot call

Remove the synchronous `runBotTurn` from inside `playChoice`/`playPath`; the human action submits and its effects animate, then the orchestration machine schedules the bot turn. Preserve the exact `botSeed(...)` derivation and call against the same state.

### 3. Remove the manual trigger; add skip/pause

Remove the "Run Bot Turn" button and its `onBotStep`/`canRunBot` path for `human_vs_bot` from `ModeControls.tsx`; add skip and pause controls wired to the scheduler's flush/rate. Replace waiting-state copy so it reads as a turn in progress (O11), not breakage.

### 4. Byte-identity proof harness

Add a recorded command-log/replay-export comparison (scripted `human_vs_bot` session) demonstrating byte-identity before/after; surface it as a Test-Plan command (e2e wiring consolidated in EFFANITUR-009).

## Files to Touch

- `apps/web/src/main.tsx` (modify)
- `apps/web/src/state/shellReducer.ts` (modify)
- `apps/web/src/components/ModeControls.tsx` (modify; shared with EFFANITUR-005)

## Out of Scope

- `bot_vs_bot` autoplay + replay stepping migration (EFFANITUR-005) — though it shares `main.tsx`/`ModeControls.tsx`.
- Per-game animation registrations (EFFANITUR-006/007) and the new animation smoke file (EFFANITUR-009).
- Any change to bot decisions, seed derivation, or submitted command bytes (FOUNDATIONS §2 — forbidden).
- Reinstating the manual trigger as an opt-in toggle (spec A5 defers that to user-testing evidence).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e` (incl. `event-frontier.smoke.mjs`): bot turns auto-advance with visible pacing; bot-first/consecutive turns need no click; acting mid-animation flushes-then-submits.
2. Grep-proof: no "Run Bot Turn" control and no `human_vs_bot` `onBotStep` path remain in `ModeControls.tsx`/`main.tsx`.
3. Recorded byte-identity comparison: command log / replay export of a scripted `human_vs_bot` session is identical before/after orchestration.

### Invariants

1. TS controls call timing and rendering only; bot decisions, seeds, and command bytes are unchanged (§2).
2. Input never hard-blocks; the in-flight burst flushes to the settled view, then the action submits (§11, UI-INTERACTION §10A).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/event-frontier.smoke.mjs` — auto-advance, waiting-state copy, input-not-blocked (extended; new dedicated `animation.smoke.mjs` lands in EFFANITUR-009).
2. Scripted determinism comparison — command-log/replay-export byte-identity before/after.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `npm --prefix apps/web run build`
3. Recorded command-log/replay-export diff for a scripted `human_vs_bot` session (the determinism boundary; no Rust test changes are expected since no Rust changes).

## Outcome

Completed: 2026-06-12

What changed:

- Removed the synchronous `runBotTurn` call from human `playChoice` / `playPath` handlers.
- Added a shell-level `human_vs_bot` orchestration effect that waits for the scheduler to drain the latest Rust effects, then calls `runBotTurn` with the same `botSeed(view)` derivation and refreshes.
- Added always-visible `Skip` plus `Pause` / `Resume` controls for `human_vs_bot`; removed the normal-mode "Run Bot Turn" button.
- Added `orchestration.paused` state in `shellReducer`.
- Added `apps/web/scripts/smoke-human-vs-bot-orchestration.mjs` proving the old inline and new refresh-between-steps sequences produce identical command logs and normalized replay exports.
- Updated existing human-vs-bot browser smokes that still clicked the removed manual trigger to wait for auto-advanced Rust effects instead.
- Amended archived `EFFANITUR-001` outcome after preserving "Bot chose action" burst text and marker-inclusive turn-report rendering needed by existing player-facing/browser proof surfaces.

Deviations from the plan:

- The replay-export byte-identity smoke normalizes only the generated `trace_id` / export handle because two separate matches necessarily receive different match ids; command logs, hashes, and replay content otherwise match exactly.
- Frontier Control's human-vs-bot setup does not emit a `bot_chose_action` marker, so its smoke asserts the auto-advanced public Rust effects ("Turn ended") and freshness advancement instead of a nonexistent marker.

Verification results:

- `npm --prefix apps/web run build` passed.
- `node apps/web/scripts/smoke-human-vs-bot-orchestration.mjs` passed.
- `rg -n "Run Bot Turn|api\\.runBotTurn\\(matchId, afterHuman" apps/web/src/main.tsx apps/web/src/components/ModeControls.tsx` returned no matches.
- `node apps/web/e2e/three-marks.smoke.mjs` passed.
- `node apps/web/e2e/plain-tricks.smoke.mjs` passed.
- `node apps/web/e2e/flood-watch.smoke.mjs` passed.
- `node apps/web/e2e/frontier-control.smoke.mjs` passed.
- `node apps/web/e2e/event-frontier.smoke.mjs` passed.
- `npm --prefix apps/web run smoke:ui` passed.
- `npm --prefix apps/web run smoke:e2e` passed.
