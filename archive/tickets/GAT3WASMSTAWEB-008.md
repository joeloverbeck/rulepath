# GAT3WASMSTAWEB-008: Play modes — human-vs-bot, hotseat, bot-vs-bot autoplay

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — TypeScript/presentation only (`apps/web`); all bot decisions come from the Rust `run_bot_turn` op.
**Deps**: 004, 006, 007

## Problem

Gate 3 requires the repository's local-first modes at Race-to-N scale:
human-vs-bot, local hotseat, and bot-vs-bot step/autoplay (spec §8.1–§8.3; ROADMAP
Gate 3 exit "human vs bot, hotseat where applicable, bot-vs-bot replay … work").
Today `main.tsx` hardcodes a single flow: human is always `seat_0`, and a bot turn
for `seat_1` is auto-run inside `playChoice` (lines ~296–315). Bots MUST use Rust
bot operations only and never choose actions in TypeScript (§7.5, FOUNDATIONS
§8/§11 "bots use the normal legal action API"). Autoplay needs pause/step and must
respect reduced motion.

## Assumption Reassessment (2026-06-06)

1. `apps/web/src/main.tsx` `playChoice` applies the human action for `seat_0` then,
   if `active_seat === "seat_1"`, calls `api.runBotTurn(matchId, "seat_1", …)`
   (lines ~304–309) — a single hardwired human-vs-bot path with no mode selection,
   no hotseat, no bot-vs-bot, no pause/step. `crates/wasm-api` exposes
   `run_bot_turn(match_id, actor_seat, bot_seed)` backed by `RaceRandomBot`
   (lib.rs lines 115–142) — the only bot class, which is correct for Gate 3
   (§8.3 "only the existing random legal bot class is required").
2. Spec §8.1 (human-vs-bot: choose first/second, neutral seat labels, manual/auto
   bot turn), §8.2 (hotseat: two human seats, active seat labeled, only the active
   actor's Rust legal actions clickable; no privacy screen needed since `race_to_n`
   is perfect-information, but viewer/actor concepts retained), §8.3 (bot-vs-bot:
   step one turn, start/pause autoplay, record effects/terminal). §11 transitions
   `autoplayPaused` stops queued advancement.
3. Cross-artifact boundary under audit: modes are reducer state/transitions
   (`apps/web/src/state/shellReducer.ts`, GAT3WASMSTAWEB-004) over the renderer/
   controls (GAT3WASMSTAWEB-006) and effect/reduced-motion layer
   (GAT3WASMSTAWEB-007). Bot turns route through `client.runBotTurn` (wrapper from
   GAT3WASMSTAWEB-001) → Rust `run_bot_turn`. No bot policy in TS.
4. FOUNDATIONS §8/§11 (bots are product opponents via the legal action API):
   restated — every bot action is produced by Rust `run_bot_turn`; TypeScript only
   decides *when* to request a bot turn (manual/step/autoplay), never *which* action.
   Autoplay is a UI scheduler, not a decision-maker.

## Architecture Check

1. Modeling mode + autoplay as reducer state (mode, active-actor, autoplay
   running/paused, pending-bot-op) with a single "request bot turn" effect is
   cleaner than the hardwired in-`playChoice` bot call: it supports all three modes
   over one play surface and keeps pause/step explicit, with viewer/actor concepts
   ready for later hidden-info games (§8.2).
2. No backwards-compatibility shims: the hardcoded human-`seat_0`/auto-bot-`seat_1`
   path is replaced by mode-driven turn handling; no parallel flow remains.
3. `engine-core` untouched; no bot policy/legality in React; `game-stdlib` untouched.

## Verification Layers

1. Bots act only via Rust → codebase grep-proof: bot turns call `client.runBotTurn`
   only; no candidate/score/policy logic in `apps/web/src`.
2. Hotseat legality per active actor → simulation/CLI run: `smoke:ui` alternates two
   human seats with only the active actor's Rust action tree clickable.
3. Autoplay pause/step → simulation: bot-vs-bot advances one Rust bot turn per step,
   start/pause halts advancement (`autoplayPaused`), reduced motion shortens pacing.
4. Bot legality/fairness → bot legality check: bot actions go through the same
   `validate_command` path as humans (Rust `run_bot_turn`), FOUNDATIONS §8.

## What to Change

### 1. `apps/web/src/state/shellReducer.ts`

Add mode (`human_vs_bot | hotseat | bot_vs_bot`), seat-role assignment
(human-first/second), active-actor tracking, and autoplay state
(idle/running/paused) + transitions, including `autoplayPaused`.

### 2. New `apps/web/src/components/ModeControls.tsx` (or extend play region)

Mode-aware turn controls: human action dispatch for the active human seat; "Run bot
turn" (manual) and step/start/pause for autoplay; neutral seat labels ("Seat 1"/
"Seat 2"). Bot turns dispatch `client.runBotTurn`; autoplay schedules bot turns
respecting reduced-motion pacing.

### 3. `apps/web/src/main.tsx`

Replace the hardwired human/bot path with mode-driven turn handling reading the
reducer's active actor + mode.

## Files to Touch

- `apps/web/src/state/shellReducer.ts` (modify) — mode, seat roles, active actor, autoplay state + transitions
- `apps/web/src/components/ModeControls.tsx` (new)
- `apps/web/src/main.tsx` (modify) — mode-driven turn handling

## Out of Scope

- Replay viewer + import/export UI — GAT3WASMSTAWEB-009 (this ticket records runs; the viewer consumes them).
- Rust bot ops (already exist) and any new bot class (forbidden — §5, FOUNDATIONS §8).
- Dev-panel autoplay controls duplication — GAT3WASMSTAWEB-010 surfaces safe summaries only.

## Acceptance Criteria

### Tests That Must Pass

1. `cd apps/web && npm run build` — typecheck + Vite build succeed.
2. `cd apps/web && npm run smoke:ui` — human-vs-bot completes a turn pair; hotseat alternates seats; bot-vs-bot steps and pauses.
3. `grep -rnE "select.*action|score|policy|legal_additions" apps/web/src/components/ModeControls.tsx` — returns nothing (no TS bot decision/legality).

### Invariants

1. Every bot action is produced by Rust `run_bot_turn`; TypeScript only schedules when to request one.
2. In hotseat, only the active actor's Rust legal actions are clickable; autoplay pause stops all queued advancement.

## Test Plan

### New/Modified Tests

1. `None — UI/mode ticket; verification is `smoke:ui` (per-mode click paths) + `tsc`; rendered-browser per-mode assertions land in GAT3WASMSTAWEB-013.`

### Commands

1. `cd apps/web && npm run build`
2. `cd apps/web && npm run smoke:ui`
3. Full per-mode rendered-DOM assertions are the Puppeteer harness's job (GAT3WASMSTAWEB-013); node smoke + the no-decision grep-proof are the correct boundary for mode wiring.

## Outcome

Completed: 2026-06-06

What changed:

- Added reducer actions/state for bot-turn pending, autoplay start, and autoplay pause.
- Added `ModeControls` for human-vs-bot manual bot turn, bot-vs-bot step, and bot-vs-bot start/pause controls.
- Made `ActionControls` actor-seat-aware so hotseat can use the active human seat's Rust action tree.
- Replaced the hardwired Seat 0 human / Seat 1 bot path in `main.tsx` with mode-driven human/bot seat scheduling.
- Bot turns route through `RulepathApi.runBotTurn`; TypeScript only schedules when to request a bot turn.
- Added reduced-motion-aware autoplay pacing.
- Extended `smoke-ui.mjs` to cover Rust catalog, hotseat turn alternation, and bot-vs-bot bot steps in addition to the existing human-vs-bot flow.

Deviations from original plan:

- Per-mode rendered-DOM assertions remain deferred to the browser harness ticket; the existing smoke command now covers the mode flows through the Rust/WASM API boundary.

Verification results:

- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:ui` passed and reported modes `human_vs_bot`, `hotseat`, and `bot_vs_bot`.
- `grep -rnE "select.*action|score|policy|legal_additions" apps/web/src/components/ModeControls.tsx` returned no matches.
