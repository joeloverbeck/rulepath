# GAT203STACROTER-003: Web render of Starbridge outcome panel + terminal smoke

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web` (`src/components/outcomeExplanationTemplates.ts`, `src/components/StarbridgeCrossingBoard.tsx`, `e2e/starbridge-crossing.smoke.mjs`)
**Deps**: GAT203STACROTER-002

## Problem

`apps/web/src/components/StarbridgeCrossingBoard.tsx` is the only one of the 20 catalog game boards that never renders `OutcomeExplanationPanel`. At terminal it shows a raw heading (`terminalLabel(view)` → `"complete"` / `"turn limit:2000"`, `:418`/`:125`) and a `, rank N` legend suffix, with no decisive-cause copy, no per-seat standings panel, and no outcome `aria-live` announcement. Once GAT203STACROTER-002 serializes `terminal_rationale`, the board can render the documented outcome surface end to end. The static template for the turn-limit cause is also missing from `outcomeExplanationTemplates.ts` (only `starbridge_crossing.finish_order_complete` exists, `:157`).

## Assumption Reassessment (2026-06-28)

1. `StarbridgeCrossingBoard.tsx` does not import or render `OutcomeExplanationPanel` (confirmed: 19 of 20 `*Board.tsx` render it; this board is the sole absentee). It already has a move-summary `aria-live="polite"` `<p>` at `:132` (`liveSummary(...)`) — distinct from the outcome announcement this ticket adds. The established pattern is `BlackglassPactBoard.tsx` (`outcomeSurfaceData({... rationale: view.outcome_rationale ?? null ...})` `:95`; `<OutcomeExplanationPanel … />` + `outcomeAnnouncementText` `:292`–`:294`).
2. Spec `specs/gate-20-3-...md` §4 "Web rendering" / "Web template key": render via `outcomeSurfaceData({ gameId: "starbridge_crossing", …, rationale: view.terminal_rationale ?? null, … })`, source `finalStanding` from the Rust-projected `final_standing` (not recomputed), and add the `starbridge_crossing.turn_limit_progress_vector` template key. Confirmed.
3. Cross-artifact boundary under audit: the consumed contract is `view.terminal_rationale` (serialized by GAT203STACROTER-002) typed `client.ts::StarbridgeCrossingPublicView.terminal_rationale?` (`:1617`, `= StarbridgeCrossingOutcomeRationale = OutcomeRationalePayload`). `outcomeSurfaceData` (`OutcomeExplanationPanel.tsx:233`) maps `rationale.final_standing` → panel `finalStanding`. No TS type change needed (spec A5).
4. FOUNDATIONS §2 / §12 motivate this ticket: TypeScript MUST render Rust-authored standings only (`SC-FINISH-004`, `SC-UI-001`) — it maps rationale fields to copy/DOM and never derives the decisive cause, rule IDs, winner, or standings order. Synthesizing the explanation in TS would cross the §12 "TypeScript decides … behavior" stop condition; consuming the Rust field avoids it.
5. No-leak firewall surface (§11): the terminal DOM/announcement. Starbridge is all-public, so the panel adds only public facts; the extended e2e asserts the existing no-leak scan still passes on the terminal surface. The templates file is static copy only — no behavior language, no raw `seat_<n>` (guarded by `check-outcome-explanations.mjs` `FORBIDDEN_TEMPLATE_PATTERNS`).

## Architecture Check

1. Following the `BlackglassPactBoard` `outcomeSurfaceData` + `OutcomeExplanationPanel` + `aria-live` mirror pattern keeps every terminal board uniform and the Starbridge board consistent with the other 19; sourcing `finalStanding` from `final_standing` keeps Rust the standings authority.
2. No shim: the panel is added alongside the existing heading; `terminalLabel` is retained for the heading, the panel carries the structured copy.
3. No legality or outcome derivation in TypeScript (§2/§7); `engine-core` untouched.

## Verification Layers

1. Panel renders both causes → extended `e2e/starbridge-crossing.smoke.mjs`: drive a bot-vs-bot match to terminal, assert `OutcomeExplanationPanel` renders with finish-order / turn-limit decisive copy and a per-seat standing per seat.
2. Outcome announced → assert the terminal `aria-live` outcome mirror (`outcomeAnnouncementText`) is present and non-empty, distinct from the move summary.
3. No-leak preserved → the e2e no-leak scan passes on the terminal surface; `node scripts/check-outcome-explanations.mjs` passes (static templates clean).
4. Template-key existence → `outcomeExplanationTemplates.ts` contains both `starbridge_crossing.finish_order_complete` and `starbridge_crossing.turn_limit_progress_vector`.

## What to Change

### 1. Add the turn-limit template key

In `apps/web/src/components/outcomeExplanationTemplates.ts`, add `"starbridge_crossing.turn_limit_progress_vector"` (static copy, `allowedGameIds: ["starbridge_crossing"]`), alongside the existing `finish_order_complete` entry — no logic, selectors, or raw seat ids.

### 2. Render the panel + announcement in the board

In `StarbridgeCrossingBoard.tsx`, build `outcomeSurfaceData({ gameId: "starbridge_crossing", …, rationale: view.terminal_rationale ?? null, … })` and render `OutcomeExplanationPanel` plus the `aria-live` `outcomeAnnouncementText` mirror at terminal, following `BlackglassPactBoard.tsx:95`–`120` / `:292`–`296`. `finalStanding` comes from the Rust-projected `final_standing`.

### 3. Extend the browser smoke

In `e2e/starbridge-crossing.smoke.mjs`, drive a bot-vs-bot match to terminal and assert the panel renders with decisive copy + per-seat standings and the no-leak scan still passes.

## Files to Touch

- `apps/web/src/components/outcomeExplanationTemplates.ts` (modify)
- `apps/web/src/components/StarbridgeCrossingBoard.tsx` (modify)
- `apps/web/e2e/starbridge-crossing.smoke.mjs` (modify)

## Out of Scope

- The Rust rationale (GAT203STACROTER-001) and its wasm serialization (GAT203STACROTER-002).
- Any TS derivation of cause / rule IDs / winner / standings order.
- Re-labelling seats, ring names, or the in-match generic `Seat N` legend (unchanged).

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/e2e/starbridge-crossing.smoke.mjs` (extended): terminal match renders `OutcomeExplanationPanel` with finish-order / turn-limit decisive copy, a per-seat standing per seat, and an outcome `aria-live` announcement; no-leak scan passes.
2. `node scripts/check-outcome-explanations.mjs` passes.
3. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e`.

### Invariants

1. No TypeScript code derives the decisive cause, rule IDs, winner, or standings order; the shell renders Rust-provided `terminal_rationale` fields only.
2. The static templates file carries no behavior language or raw seat ids.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/starbridge-crossing.smoke.mjs` — extend to a terminal match asserting panel render, per-seat standings, outcome announcement, and no-leak.

### Commands

1. `node apps/web/e2e/starbridge-crossing.smoke.mjs`
2. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e && node scripts/check-outcome-explanations.mjs`
3. The web smoke is the correct boundary — Rust/bridge correctness is owned by GAT203STACROTER-001/002.
