# GAT12FLOWATCOO-016: Player and mechanic docs (HOW-TO-PLAY, MECHANICS, UI, AI)

**Status**: ACCEPTED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — documentation only (`games/flood_watch/docs/{HOW-TO-PLAY,MECHANICS,UI,AI}.md`); no code surface
**Deps**: GAT12FLOWATCOO-009, GAT12FLOWATCOO-010, GAT12FLOWATCOO-011

## Problem

`flood_watch` needs its descriptive per-game docs once behavior is implemented: `HOW-TO-PLAY.md` (the player-facing rules, generated into the web shell), `MECHANICS.md` (the mechanic inventory), `UI.md` (UI metadata + the outcome/victory explanation section the outcome-check consumes), and `AI.md` (bot overview). These describe implemented behavior, so they trail the rules/visibility/bot tickets; `HOW-TO-PLAY.md` and `UI.md` are inputs the web ticket depends on.

## Assumption Reassessment (2026-06-11)

1. GAT12FLOWATCOO-005/006/007/008 implemented the rules/visibility behavior these docs describe; GAT12FLOWATCOO-010 the bots `AI.md` covers. `games/masked_claims/docs/{HOW-TO-PLAY,MECHANICS,UI,AI}.md` are the verified exemplars; templates `templates/GAME-HOW-TO-PLAY.md`, `GAME-MECHANICS.md`, `GAME-UI.md`, `GAME-AI.md` exist.
2. The spec (§Deliverables "Per-game docs", Work-breakdown item 11) requires these four among the thirteen docs. `HOW-TO-PLAY.md` carries the required player-rules section set (it is the source `scripts/copy-player-rules.mjs` copies to `apps/web/public/rules/flood_watch.md` in GAT12FLOWATCOO-017). `UI.md` must carry the "Outcome / victory explanation" section that `scripts/check-outcome-explanations.mjs` reads (verified surface #2).
3. Cross-artifact boundary under audit: `HOW-TO-PLAY.md` is the source-of-truth for the generated player-rules file — GAT12FLOWATCOO-017 runs the copy script against it and adds `flood_watch` to `HIDDEN_INFO_GAMES`; `UI.md`'s outcome section + `RULES.md`'s terminal section (GAT12FLOWATCOO-001) are both consumed by `check-outcome-explanations.mjs`. So GAT12FLOWATCOO-017 `Deps` this ticket. These docs must stay consistent with implemented behavior (`src/ui.rs` labels, the bot policy, the rules).
4. FOUNDATIONS §6 (evidence-heavy: original rules summary, mechanic inventory, UI metadata, bot overview are part of the done contract) and §7 (UI metadata is viewer-facing) motivate this ticket.
5. Enforcement surface: `HOW-TO-PLAY.md` is the hidden-info player doc — `scripts/check-player-rules.mjs` requires `flood_watch` in its `HIDDEN_INFO_GAMES` set (deck order is hidden) and the generated file in sync; that wiring lands in GAT12FLOWATCOO-017, but the doc authored here must describe the deck-order hidden-information posture correctly (no leak, composition counts are public).

## Architecture Check

1. Trailing placement (after behavior lands) keeps the docs consistent with implemented mechanics rather than aspirational; front-loading only `HOW-TO-PLAY`/`UI` ahead of the web ticket (via the `Deps` edge) avoids a docs-after-web staleness window.
2. No backwards-compatibility aliasing/shims; net-new docs.
3. `engine-core`/`game-stdlib` untouched; docs are game-local.

## Verification Layers

1. Player-rules source present + correct section set -> manual review against `templates/GAME-HOW-TO-PLAY.md`; consumed by the copy script in GAT12FLOWATCOO-017.
2. Outcome-explanation section present -> grep-proof `UI.md` carries the "Outcome / victory explanation" section `check-outcome-explanations.mjs` reads.
3. Consistency with behavior -> manual review against `src/ui.rs`, the rules, and the bot policy.
4. Doc link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `HOW-TO-PLAY.md`

Instantiate from `templates/GAME-HOW-TO-PLAY.md`; author the player-facing rules with the required section set, describing the cooperative shared outcome, the budgeted turn, the environment phase, forecast, and the deck-order hidden-information posture (composition counts public).

### 2. `MECHANICS.md`

Instantiate from `templates/GAME-MECHANICS.md`; record the mechanic inventory (shared outcome, event-deck automation, role-modified actions, multi-action budget) consistent with the GAT12FLOWATCOO-002 ledger.

### 3. `UI.md`

Instantiate from `templates/GAME-UI.md`; document UI metadata (labels, gauges, levees, deck/forecast displays, budget indicator, waiting state) and the "Outcome / victory explanation" section the outcome-check consumes.

### 4. `AI.md`

Instantiate from `templates/GAME-AI.md`; overview the Level 0/Level 1 bots, the priority policy, and the teammate/bot-vs-bot modes.

## Files to Touch

- `games/flood_watch/docs/HOW-TO-PLAY.md` (new)
- `games/flood_watch/docs/MECHANICS.md` (new)
- `games/flood_watch/docs/UI.md` (new)
- `games/flood_watch/docs/AI.md` (new)

## Out of Scope

- The generated `apps/web/public/rules/flood_watch.md` and `HIDDEN_INFO_GAMES` registration (GAT12FLOWATCOO-017).
- `GAME-IMPLEMENTATION-ADMISSION.md` + `PUBLIC-RELEASE-CHECKLIST.md` (closeout, GAT12FLOWATCOO-019).
- Bot-strategy evidence docs (GAT12FLOWATCOO-013).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with all four docs present.
2. `grep -i "Outcome / victory explanation" games/flood_watch/docs/UI.md` returns the section header.
3. `HOW-TO-PLAY.md` carries the required player-rules section set (manual review against the template) and describes the deck-order hidden-info posture.

### Invariants

1. Docs are consistent with implemented behavior (`src/ui.rs`, rules, bots).
2. `HOW-TO-PLAY.md` describes the deck order as hidden and composition counts as public — no leak in the player doc.

## Test Plan

### New/Modified Tests

1. `None — documentation ticket; verification is command-based (doc-link integrity + section grep-proofs). The player-rules sync check runs in GAT12FLOWATCOO-017 via `check-player-rules.mjs`.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -i "Outcome / victory explanation" games/flood_watch/docs/UI.md`
3. `node scripts/check-player-rules.mjs` is the full player-rules-sync boundary but requires the generated file + `HIDDEN_INFO_GAMES` entry (GAT12FLOWATCOO-017); doc-link + section grep are the correct boundary for the authoring diff.

## Outcome

Accepted on 2026-06-11. Added Flood Watch `HOW-TO-PLAY.md`,
`MECHANICS.md`, `UI.md`, and `AI.md`. The player doc includes the required
section set and states that undrawn deck order is hidden while remaining
composition counts are public. `UI.md` includes the mandatory
`Outcome / victory explanation` section and Rust-owned terminal copy contract.

Verification:

1. `node scripts/check-doc-links.mjs`
2. `grep -i "Outcome / victory explanation" games/flood_watch/docs/UI.md`
3. `rg -n "What you can see|Setup|On your turn|Actions|Scoring and winning|Hidden information and reveal timing|Common terms|undrawn deck order|remaining composition" games/flood_watch/docs/HOW-TO-PLAY.md`
