# MODECTRL-002: Human-vs-bot Skip and Pause controls must reflect actual orchestration activity

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — presentation/orchestration policy only. Touches `apps/web/src/components/ModeControls.tsx`, `apps/web/src/main.tsx`, and `apps/web/src/animation/scheduler.ts`. No Rust crates, schemas, traces, command/effect/view contracts, or determinism surfaces.
**Deps**: MODECTRL-001 (recommended landing order — both edit `ModeControls.tsx`; land the layout fix first to keep diffs clean)

## Problem

In the shared `ModeControls` panel (rendered for all 13 catalog games), the **Human vs bot** branch presents two controls that are live even when there is nothing for them to act on:

1. **Skip is enabled whenever the match is non-terminal.** `ModeControls.tsx:76` gates it only on `disabled={terminal}`. Skip calls `flushScheduler()` (`main.tsx:677`), which flushes in-flight/queued animation. On the human's idle turn — no animation playing, no bot turn pending — Skip is a clickable button that does nothing.
2. **Pause/Resume always renders and always toggles.** `ModeControls.tsx:79-87` always shows Pause (or Resume) in human-vs-bot mode, gated only on `terminal`. It toggles `state.orchestration.paused`, which only affects the bot auto-advance effect (`main.tsx:272-310`). On the human's idle turn there is no auto-advancing sequence, so pressing Pause→Resume is a visible no-op that falsely implies something was paused. (Human-vs-bot mode has no manual "Step Bot" control — that exists only in bot-vs-bot — so a pre-emptive pause latch has no manual step to pair with.)

The bot-vs-bot branch of the same component is **already correctly gated** (`Step Bot` on `canRunBot`, `Start Autoplay` on `canAutoplay`, `Pause` only while `autoplayRunning`). The fix is to bring the human-vs-bot branch up to the same discipline: both controls reflect whether a non-interactive advance (bot turn or animation burst) is actually active or pending.

Decision (from brainstorm, 2026-06-13): **disable both Skip and Pause when idle** — keep them visible but inert until a non-interactive advance is active/pending, enabling them the moment a bot turn or animation burst plays. This honors `docs/UI-INTERACTION.md §10A` ("skip is always available and instantaneous" — the control stays present) while removing the dead-control confusion.

## Assumption Reassessment (2026-06-13)

1. `apps/web/src/components/ModeControls.tsx:74-89` is the `playMode === "human_vs_bot"` branch: `Skip` (`disabled={terminal}`, `onClick={onSkip}`) and the `orchestrationPaused ? Resume : Pause` pair (each `disabled={terminal}`). `onSkip` is wired to `() => void flushScheduler()` at `main.tsx:677`; Pause/Resume dispatch `orchestrationPaused`/`orchestrationResumed` at `main.tsx:678-679`.
2. `apps/web/src/state/shellReducer.ts` models `orchestration: { paused, rate }` (no `active`/`busy` field) and a `pendingOperation` union that takes the value `"botTurn"` while a bot turn runs (`botTurnStarted` sets it at line 363, `botTurnCompleted` clears it at line 379) and `"applyAction"` during a human action. `pendingOperation` is render state already passed into `ModeControls` as `pending`.
3. Shared-boundary under audit: `ModeControls` is the single shared in-play control surface for all 13 games (`main.tsx:665-683`). The gating change is additive (a new prop + `disabled` predicate) and must not alter the already-correct bot-vs-bot or hotseat branches.
4. FOUNDATIONS principle under audit: `docs/UI-INTERACTION.md §10A` — "Orchestration is presentation policy in TypeScript… Wall-clock time stays out of Rust; command logs, traces, replays, and hashes are unaffected by pacing." This change is pure presentation policy: it gates control enablement on TS-side orchestration state and touches no Rust, command log, trace, replay, or hash. §10A also requires skip to remain available during non-interactive advances and pause/stop to be exposed for auto-playing sequences — the disable-when-idle model satisfies both (controls stay present; they enable exactly when an advance is live).
5. Load-bearing premise verified: the "is a non-interactive advance active/pending" signal is **not currently available as React render state**. The scheduler (`apps/web/src/animation/scheduler.ts`) tracks activity only privately (`running`, `flushing`, `inFlightAnimations`, `queue`) and exposes `pendingSteps` but no observable busy flag; the auto-bot effect tracks in-flight state via `autoBotInFlightRef` (a ref, not render state, `main.tsx:283/289/307`). Therefore this ticket must surface an observable signal — it cannot simply read an existing boolean. This is the core of the change.
6. Adjacent contradiction classification: the bot-vs-bot branch is correct and out of scope; the `gameId === "event_frontier"` bot-why hardcode is unrelated future cleanup (see MODECTRL-001 Out of Scope). Neither is changed here.

## Architecture Check

1. Cleanest design: introduce a single derived boolean, `orchestrationActive` (a non-interactive advance is playing or imminent), owned in `main.tsx` where the scheduler ref, the auto-bot effect, and the reducer state already live; pass it to `ModeControls` as a prop and gate both Skip and Pause/Resume `disabled` on `!orchestrationActive || terminal`. This mirrors the existing bot-vs-bot gating pattern rather than inventing a parallel mechanism, and keeps the legality/behavior boundary untouched (the signal is pacing state only).
2. No backwards-compatibility shims/aliases: the change replaces the `disabled={terminal}` predicate on the two controls; it does not keep the old always-enabled path behind a flag.
3. `engine-core` untouched and noun-free (§3); no `game-stdlib` change (§4 not engaged). The new signal is presentation-layer state, consistent with `docs/UI-INTERACTION.md §10A` treating orchestration as TypeScript presentation policy; it adds no payload, DOM attribute, or test-id carrying hidden state (§11 no-leak — the signal is a boolean about pacing, derived from no private/hidden game state).

## Verification Layers

1. Idle human turn disables both controls -> e2e DOM assertion (`Skip` and `Pause` carry `disabled` when it is the human's turn with no advance in flight).
2. An active/pending non-interactive advance enables the controls -> e2e DOM assertion (after a human move triggers the bot turn / animation burst, `Skip` enables; `Pause` enables while the bot sequence is active/pending).
3. The new observable activity signal correctly reflects scheduler busy + bot-advance state -> scheduler smoke (`apps/web/scripts/smoke-scheduler.mjs`) extended to cover the busy/active observable transitions.
4. Bot-vs-bot and hotseat branches unchanged -> codebase grep-proof + `npm --prefix apps/web run smoke:e2e` regression across all catalog games.
5. No Rust/determinism surface touched -> FOUNDATIONS alignment check (`docs/UI-INTERACTION.md §10A`; FOUNDATIONS §11 "Replay, hashes, serialization order, RNG, and traces remain deterministic"; pacing stays out of Rust).

## What to Change

### 1. Surface an observable orchestration-activity signal

In `apps/web/src/animation/scheduler.ts`, expose whether the scheduler is currently active (draining/running, has queued steps, or has in-flight animations) in a React-observable way — e.g. an `isActive`/`busy` getter plus a lightweight `onActivityChange` subscription (or activity callbacks fired around `drain`/`runStep`/`flush`), without changing the existing pacing/flush semantics. Do not break the existing `pendingSteps` getter or the flush-and-settle contract.

In `apps/web/src/main.tsx`, derive `orchestrationActive` from: the scheduler activity signal **OR** `state.pendingOperation === "botTurn"` **OR** the auto-bot drain being in flight (promote the `autoBotInFlightRef` state into a render-observable flag, or fold it into the derived signal) **OR** there being undrained effects queued to animate for a bot-controlled active seat (a bot advance is imminent). Memoize it.

### 2. Gate the human-vs-bot controls

Add an `orchestrationActive: boolean` prop to `ModeControls` and pass it from `main.tsx:665-683`. In the `playMode === "human_vs_bot"` branch (`ModeControls.tsx:74-89`), change both controls' `disabled` to `!orchestrationActive || terminal`. Add an `aria`/`title` affordance so a disabled Skip/Pause reads as "nothing to skip / pause right now" for screen readers (per `docs/UI-INTERACTION.md §17`). Leave the bot-vs-bot and hotseat branches unchanged.

## Files to Touch

- `apps/web/src/animation/scheduler.ts` (modify — expose observable activity)
- `apps/web/src/main.tsx` (modify — derive `orchestrationActive`, pass prop)
- `apps/web/src/components/ModeControls.tsx` (modify — new prop + `disabled` gating)

## Out of Scope

- The bot-vs-bot branch (already correctly gated) and the hotseat branch.
- The Mode panel layout/CSS (MODECTRL-001).
- Hiding the controls when idle, or a pre-emptive pause latch with a status badge — the chosen behavior is **disable when idle**, both controls stay visible.
- Adding a manual "Step Bot" control to human-vs-bot mode.
- Any Rust, WASM, command-log, effect, view, replay, or hash change. Pacing stays out of Rust.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` (includes `tsc --noEmit`) passes.
2. `npm --prefix apps/web run smoke:animation` passes (scheduler smoke, extended for the activity signal).
3. `npm --prefix apps/web run smoke:e2e` passes (Event Frontier + all catalog games; idle-disabled / active-enabled assertions hold).

### Invariants

1. In human-vs-bot mode, Skip and Pause/Resume are `disabled` whenever no non-interactive advance is active or pending, and enabled exactly when one is.
2. The signal driving enablement is derived only from TypeScript pacing/orchestration state; no Rust call, command log, trace, replay, or hash is affected, and no hidden game state is introduced into any payload, DOM attribute, or test-id.
3. The bot-vs-bot and hotseat branches of `ModeControls` are byte-for-byte unchanged except for the shared prop threading.

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-scheduler.mjs` — extend to assert the new activity observable transitions to active during a drain and back to inactive after settle/flush.
2. `apps/web/e2e/event-frontier.smoke.mjs` — assert Skip and Pause are `disabled` on the human's idle turn, and become enabled after a human move triggers the bot turn / animation burst.

### Commands

```bash
npm --prefix apps/web run build
npm --prefix apps/web run smoke:animation
npm --prefix apps/web run smoke:e2e
```
