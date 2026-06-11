# GAT11MASCLABLU-016: Remaining per-game docs (MECHANICS, AI, UI, HOW-TO-PLAY)

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — documentation only (new `games/masked_claims/docs/{MECHANICS,AI,UI,HOW-TO-PLAY}.md`; no Rust surface)
**Deps**: GAT11MASCLABLU-008, GAT11MASCLABLU-009

## Problem

The official-game contract requires the mechanic inventory (`MECHANICS.md`), the AI overview (`AI.md`), the UI doc (`UI.md`), and the player-facing how-to (`HOW-TO-PLAY.md`). `HOW-TO-PLAY.md` is the source for the generated player-rules copy and must carry the required player-rules section set.

## Assumption Reassessment (2026-06-10)

1. The implemented behavior (GAT11MASCLABLU-004–009) is the source of truth these docs describe. Templates `templates/GAME-MECHANICS.md`, `templates/GAME-AI.md`, `templates/GAME-UI.md`, `templates/GAME-HOW-TO-PLAY.md` are confirmed present; `games/plain_tricks/docs/{MECHANICS,AI,UI,HOW-TO-PLAY}.md` are the shape models.
2. Spec Deliverables per-game-docs row requires these four; `HOW-TO-PLAY.md` "carries the required player-rules section set" and is consumed by `scripts/copy-player-rules.mjs` to generate `apps/web/public/rules/masked_claims.md` (GAT11MASCLABLU-018).
3. Cross-artifact boundary under audit: `HOW-TO-PLAY.md` feeds the player-rules generation contract, and `UI.md` (with `RULES.md`) is read by `scripts/check-outcome-explanations.mjs` — both validated in GAT11MASCLABLU-018, which therefore depends on this ticket.
4. FOUNDATIONS §6 (evidence-heavy docs) and §7 (the UI doc describes a play-first, accessible board) are the principles under audit. `MECHANICS.md` records the realized first official use of the reaction-window / pending-response mechanic.

## Architecture Check

1. Authoring these docs after the behavior exists keeps the mechanic inventory and UI description truthful to the implementation.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free; these are game-local docs.

## Verification Layers

1. Doc-link integrity -> `node scripts/check-doc-links.mjs`.
2. `HOW-TO-PLAY.md` carries the required player-rules section set -> `node scripts/check-player-rules.mjs` (after generation in GAT11MASCLABLU-018) + manual review now.
3. Docs consistent with implemented behavior -> manual review against the crate modules and `MECHANICS.md` mechanic inventory.

## What to Change

### 1. `games/masked_claims/docs/MECHANICS.md`

Instantiate from `templates/GAME-MECHANICS.md`; record the mechanic inventory including the reaction-window / pending-response first official use and the deterministic shuffle / private-hand / staged-reveal fourth use.

### 2. `games/masked_claims/docs/AI.md`

Instantiate from `templates/GAME-AI.md`; describe the Level 0 / Level 1 bot design at the overview level.

### 3. `games/masked_claims/docs/UI.md`

Instantiate from `templates/GAME-UI.md`; describe the board, the reaction prompt / waiting state, galleries, reduced-motion, and accessibility — play-first, no-leak.

### 4. `games/masked_claims/docs/HOW-TO-PLAY.md`

Instantiate from `templates/GAME-HOW-TO-PLAY.md`; carry the required player-rules section set used to generate the public rules copy.

## Files to Touch

- `games/masked_claims/docs/MECHANICS.md` (new)
- `games/masked_claims/docs/AI.md` (new)
- `games/masked_claims/docs/UI.md` (new)
- `games/masked_claims/docs/HOW-TO-PLAY.md` (new)

## Out of Scope

- Generating `apps/web/public/rules/masked_claims.md` and registering `HIDDEN_INFO_GAMES` (GAT11MASCLABLU-018).
- `RULES.md` / `SOURCES.md` (GAT11MASCLABLU-001); `ADMISSION` / `PUBLIC-RELEASE-CHECKLIST` (GAT11MASCLABLU-020).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the four new docs.
2. `HOW-TO-PLAY.md` contains the required player-rules sections (verified by `check-player-rules.mjs` once generated in GAT11MASCLABLU-018).
3. `MECHANICS.md` records the reaction-window first-use and shuffle fourth-use consistent with the atlas.

### Invariants

1. Docs match implemented behavior (FOUNDATIONS §6).
2. The UI doc describes a play-first, accessible, no-leak board (FOUNDATIONS §7).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and the behavior it documents is covered by GAT11MASCLABLU-004–010.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `node scripts/check-player-rules.mjs` — confirms the `HOW-TO-PLAY.md` section set once GAT11MASCLABLU-018 generates the public copy.
3. Doc-link integrity is the runnable boundary now; player-rules sync is GAT11MASCLABLU-018's responsibility.

## Outcome

Completed on 2026-06-11.

- Added `MECHANICS.md` with the Masked Claims mechanic inventory, reaction-window first-use posture, and deterministic shuffle/private-hand fourth-use posture.
- Added `AI.md` documenting the Level 0 random legal bot, Level 1 baseline bot, information access, explanations, tests, benchmarks, and Level 2/3 deferrals.
- Added `UI.md` covering the play-first board plan, legal action mapping, reaction prompt and waiting state, no-leak outcome surfaces, replay, accessibility, and reduced motion.
- Added `HOW-TO-PLAY.md` with the required player-rules section set and player-facing rules/reveal timing.

Verification:

- `node scripts/check-doc-links.mjs`
- Manual section check: `HOW-TO-PLAY.md` contains every required player-rules section. Generated player-rules sync remains GAT11MASCLABLU-018 scope.
