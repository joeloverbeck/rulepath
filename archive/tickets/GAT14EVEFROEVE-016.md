# GAT14EVEFROEVE-016: Player and mechanic docs (MECHANICS, UI, AI)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — documentation only (`games/event_frontier/docs/{MECHANICS,UI,AI}.md`; sync `HOW-TO-PLAY.md`)
**Deps**: GAT14EVEFROEVE-010, GAT14EVEFROEVE-014

## Problem

The official-game doc set needs its mechanic and presentation docs: `MECHANICS.md` (the implemented mechanic inventory), `UI.md` (the viewer-facing presentation contract including the outcome/victory-explanation section that `check-outcome-explanations` reads), and `AI.md` (the bot overview). `HOW-TO-PLAY.md` (authored in ticket 014 with the hidden-info section) is reviewed and kept in sync. These docs describe behavior already implemented, so they trail the rules and bot tickets and must match the code.

## Assumption Reassessment (2026-06-12)

1. The behavior these docs describe exists: verified tickets 004–010 implement the mechanics, ticket 009 the visibility/outcome surfaces, and ticket 010 the bots; `HOW-TO-PLAY.md` was authored in ticket 014. The doc set + template mapping is the thirteen-doc convention (`templates/GAME-MECHANICS.md`, `GAME-UI.md`, `GAME-AI.md`).
2. The outcome-explanation contract is current: verified `scripts/check-outcome-explanations.mjs` reads the game's `docs/UI.md` "Outcome / victory explanation" section and `docs/RULES.md` rule IDs (catalog-driven off the wasm-api const, registered in ticket 014); `UI.md` must carry the victory-type/tiebreak explanation section for the checker.
3. Cross-artifact boundary under audit: `UI.md` and `RULES.md` are the doc half of the outcome-explanation contract whose code half (templates + rationale mirror) lands in ticket 017; `check-outcome-explanations` passes only once both halves exist (the script runs in ticket 018's gate-1 reconciliation). These docs must match the implemented mechanics exactly (no aspirational behavior).
4. FOUNDATIONS §6 (evidence-heavy: mechanic inventory + UI metadata) and §7 (public UI is product work) motivate this ticket. Restated before trusting the spec: docs describe implemented behavior; `UI.md` records the legal-only, effect-driven, viewer-safe presentation contract, not TypeScript-decided legality.

## Architecture Check

1. Trailing the mechanic/UI/AI docs after implementation keeps them truthful (transcribed from code), and co-locating the `UI.md` outcome section with the doc set lets `check-outcome-explanations` validate the doc half against the code half (ticket 017).
2. No backwards-compatibility aliasing/shims — new docs.
3. `engine-core` stays noun-free; no `game-stdlib` promotion; documentation only.

## Verification Layers

1. Doc completeness -> the three docs instantiate their templates; `MECHANICS.md` inventories every mechanic, `UI.md` carries the outcome/victory-explanation section, `AI.md` the bot overview.
2. Outcome-explanation doc half -> grep `UI.md` for the victory-type/tiebreak explanation section the checker reads; full `check-outcome-explanations` green lands in ticket 018 (templates in ticket 017).
3. Doc-link integrity -> `node scripts/check-doc-links.mjs`.
4. Fidelity to code -> manual review that `MECHANICS.md`/`UI.md` match the implemented behavior (tickets 004–010).

## What to Change

### 1. MECHANICS.md, UI.md, AI.md

Instantiate from `templates/GAME-MECHANICS.md`, `GAME-UI.md`, `GAME-AI.md`. `MECHANICS.md`: the implemented mechanic inventory (event deck, eligibility/initiative, operations, edicts, Reckoning, asymmetric victory). `UI.md`: the presentation contract — board layout, constrained menus, progressive op construction, edict banner, Reckoning panel, victory-progress indicators, and the **outcome / victory-explanation section** (victory-type + tiebreak) `check-outcome-explanations` reads. `AI.md`: the bot overview (Level 0 + Level 1 per faction).

### 2. Sync HOW-TO-PLAY.md

Review `HOW-TO-PLAY.md` (created in ticket 014) for consistency with the final implemented rules; keep the hidden-info section accurate.

## Files to Touch

- `games/event_frontier/docs/MECHANICS.md` (new)
- `games/event_frontier/docs/UI.md` (new)
- `games/event_frontier/docs/AI.md` (new)
- `games/event_frontier/docs/HOW-TO-PLAY.md` (modify; created by 014)

## Out of Scope

- The React board, outcome-explanation templates, and the `client.ts` rationale mirror (ticket 017) — this ticket authors the doc half of the outcome contract.
- ADMISSION / PUBLIC-RELEASE-CHECKLIST docs (ticket 019) and the bot-evidence docs (ticket 013).
- Running `check-outcome-explanations` to green (ticket 018, once the templates land).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the three new docs present.
2. `grep -niE "victory|tiebreak|outcome" games/event_frontier/docs/UI.md` shows the outcome/victory-explanation section.
3. The docs are consistent with implemented behavior (manual review against tickets 004–010).

### Invariants

1. Every doc describes implemented behavior; no aspirational mechanic appears.
2. `UI.md` carries the outcome-explanation doc half required by `check-outcome-explanations`.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is the doc-link check plus the outcome-section grep; full check-outcome-explanations green lands in ticket 018.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -niE "victory|tiebreak|outcome" games/event_frontier/docs/UI.md`
3. The doc-link check plus the outcome-section grep is the correct boundary — the runnable `check-outcome-explanations` gate needs the code half (ticket 017) and is run at the gate-1 reconciliation (ticket 018).

## Outcome

Authored the Event Frontier mechanic inventory, UI presentation contract, and AI
registry docs from the repo templates. Reviewed `HOW-TO-PLAY.md` against the
implemented rules and synced its maintainer references to the new docs. `UI.md`
includes the mandatory `Outcome / victory explanation` section with victory
types, final-fallback tiebreak handling, viewer-safe breakdown fields, no-leak
rules, and smoke expectations.

Verification completed:

1. `node scripts/check-doc-links.mjs`
2. `grep -niE "victory|tiebreak|outcome" games/event_frontier/docs/UI.md`
