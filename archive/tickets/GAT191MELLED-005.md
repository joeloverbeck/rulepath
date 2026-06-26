# GAT191MELLED-005: Web round-transition feedback + browser verification

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web` (`src/components/effectFeedback.ts`, `scripts/smoke-effect-feedback.mjs`)
**Deps**: GAT191MELLED-004

## Problem

Once the apply path emits `next_round_dealt`, the web shell needs to present it and
the board must visibly advance out of the transient "Round settled" state into the
next round, reaching the terminal outcome panel rather than the
"No actions available" dead-end. This ticket adds the web feedback case for the
Rust-authored effect and verifies, in the browser, that a full bot-vs-bot match
autoplays to the terminal panel.

## Assumption Reassessment (2026-06-26)

1. `apps/web/src/components/effectFeedback.ts` has one shared `describeEffect`
   switch keyed on `payload.type ?? payload.kind` (line 16). Meldfall cases
   (`round_score` line 822, `match_terminal` 828, `draw`/`meld`/`discard`) use
   `meldfallSeatLabel` (line 962, renders `seat_N` → `Seat N+1`). The
   `refill_started` case (line 834) belongs to `high_card_duel` and is left
   untouched; the new meldfall case keys on `next_round_dealt` (GAT191MELLED-003).
2. `apps/web/src/components/MeldfallLedgerBoard.tsx` renders the "Round settled"
   heading (line 404) gated on `view.phase === "round_settled"` (line 49). After
   the transition the board auto-advances into `Draw`, so this copy stays valid as
   a transient state (spec §3.1.8) — verify-only, edit only if stale.
3. Cross-artifact boundary under audit: the effect-kind contract from
   GAT191MELLED-003 — kind `next_round_dealt` with public fields
   `next_round_number`, `next_lead_seat`, `new_dealer`. The web case must read
   exactly those fields. `apps/web/scripts/smoke-effect-feedback.mjs:340` already
   carries a `["meldfall_ledger", "draw"]` assertion pair to extend.
4. FOUNDATIONS §2 behavior authority restated: TypeScript presents the
   Rust-authored effect only — it decides no legality and invents no state, merely
   formatting the Rust-supplied counts/seat labels into copy.

## Architecture Check

1. A distinct `next_round_dealt` case (rather than editing `high_card_duel`'s
   shared `refill_started` case) keeps each game's presentation isolated in the
   shared switch and avoids regressing high_card_duel.
2. No backwards-compatibility shim: the case is additive presentation; no alias.
3. `engine-core` / `game-stdlib` untouched; no legality decided in TypeScript
   (§2); the board settles to the latest viewer-safe public view after the effect.

## Verification Layers

1. `effectFeedback` renders `next_round_dealt` -> `smoke:effects` assertion for the new meldfall pair.
2. `high_card_duel`'s `refill_started` presentation is unchanged -> grep-proof the existing case (line ~834) is untouched.
3. Board reaches the terminal panel with no `round_settled` dead-end -> manual browser runbook (autoplay a full bot-vs-bot match).

## What to Change

### 1. Add the `next_round_dealt` case (`effectFeedback.ts`)

Add `case "next_round_dealt"` using `meldfallSeatLabel`, with friendly copy naming
the new round, dealer, and lead seat (e.g. "Round 2 dealt — Seat 3 deals; Seat 4
leads off"). Leave the existing `refill_started` case for `high_card_duel` intact.

### 2. Extend the effect smoke (`smoke-effect-feedback.mjs`)

Add a `["meldfall_ledger", "next_round_dealt"]` assertion pair (alongside the
existing `["meldfall_ledger", "draw"]`) so the new case is covered.

### 3. Browser verification runbook (manual)

Launch a bot-vs-bot Meldfall Ledger match in the web shell, enable autoplay, and
confirm the board advances past "Round settled" into the next round and reaches the
terminal outcome panel with no "No actions available" dead-end. Capture a
screenshot as evidence (spec exit criterion 6).

## Files to Touch

- `apps/web/src/components/effectFeedback.ts` (modify)
- `apps/web/scripts/smoke-effect-feedback.mjs` (modify)
- `apps/web/src/components/MeldfallLedgerBoard.tsx` (verify-only — confirm the
  `round_settled` transient copy stays valid; edit only if stale)

## Out of Scope

- The effect definition / emission and the apply-path wiring — GAT191MELLED-003 / -004.
- Any engine, scoring, or legality logic (Rust owns it; this ticket presents only).
- The golden trace, coverage/evidence docs, and closeout — GAT191MELLED-006.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:effects` — the new meldfall `next_round_dealt` assertion passes.
2. `npm --prefix apps/web run build` — type-checks; `npm --prefix apps/web run smoke:ui` green.
3. Manual browser runbook: a full bot-vs-bot match autoplays to the terminal outcome panel (no `round_settled` dead-end), screenshot captured.

### Invariants

1. TypeScript presents the Rust-authored `next_round_dealt` effect only; it decides
   no legality and invents no presentation data beyond the Rust-supplied fields.
2. `high_card_duel`'s `refill_started` presentation is unchanged.

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-effect-feedback.mjs` — assert the meldfall
   `next_round_dealt` effect renders friendly copy.

### Commands

1. `npm --prefix apps/web run smoke:effects`
2. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:ui`
3. Manual runbook (not CI-runnable — the web shell has no browser-automation
   harness): launch bot-vs-bot Meldfall Ledger, autoplay, confirm advance past
   "Round settled" to the terminal panel, capture a screenshot.

## Outcome

Completed: 2026-06-26

Added the Meldfall-owned `next_round_dealt` feedback case in the shared web
effect presenter. The copy presents only Rust-authored fields:
`next_round_number`, `new_dealer`, and `next_lead_seat`, using the existing
Meldfall seat-label helper. The existing `refill_started` case remains intact for
High Card Duel.

Extended the effect feedback smoke with a focused Meldfall bot-turn runner that
observes `next_round_dealt` from the WASM host and asserts the rendered copy:
`Round 2 dealt - Seat 2 deals; Seat 3 leads off.`

Browser evidence: launched the production web preview, selected Meldfall Ledger
4-seat Bot vs bot, started autoplay, and captured the terminal outcome panel at
`output/playwright/gat191melled-005-meldfall-terminal.png`. The browser run
ended with `seat_1 won with 547`, visible `Match Complete`/`Seat 2 wins` copy,
and no `No actions available` dead-end. To make the long autoplay run practical
inside the browser harness, the evidence run capped page timers to collapse
animation dwell while still using the app's Bot vs bot autoplay controls.

Verification:

- `npm --prefix apps/web run smoke:effects`
- `npm --prefix apps/web run smoke:ui`
- Browser screenshot evidence:
  `output/playwright/gat191melled-005-meldfall-terminal.png`
