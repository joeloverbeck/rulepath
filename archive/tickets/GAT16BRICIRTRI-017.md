# GAT16BRICIRTRI-017: Trailing game documentation (mechanics, UI, AI, bot evidence, release checklist)

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Medium
**Engine Changes**: None (docs — `games/briar_circuit/docs/{MECHANICS,UI,AI,COMPETENT-PLAYER,BOT-STRATEGY-EVIDENCE-PACK,PUBLIC-RELEASE-CHECKLIST}.md`)
**Deps**: 011, 014, 016

## Problem

Briar Circuit's official-game doc set must be completed: the mechanic inventory, the Rust/React UI boundary + viewer matrix (including the outcome/victory-explanation section `check-outcome-explanations` requires), the bot policy contract, the competent-player landscape, the bot-strategy evidence pack (`L2 not admitted`), and the public-release checklist. This closes the `check-outcome-explanations` window opened at GAT16BRICIRTRI-013.

## Assumption Reassessment (2026-06-20)

1. `games/{plain_tricks,river_ledger}/docs/` are the 13-file doc-set exemplars; the rules/sources/admission/how-to-play/coverage docs already exist (001/002/012). The bots (011), web surfaces (014), and benchmarks (016) exist, so MECHANICS/UI/AI/evidence docs can describe real behavior. `scripts/check-outcome-explanations.mjs` requires a "Outcome / victory explanation" section in `UI.md`.
2. `specs/gate-16-briar-circuit-trick-taking.md` §4.3 (doc requirements), §10.3, Appendix C (bot policy / L2 gate), and Appendix D (UI/accessibility) fix this content; `templates/GAME-{MECHANICS,UI,AI}.md`, `templates/COMPETENT-PLAYER.md`, `templates/BOT-STRATEGY-EVIDENCE-PACK.md`, `templates/PUBLIC-RELEASE-CHECKLIST.md` are the structural templates.
3. Cross-artifact boundary under audit: `UI.md`'s outcome section + the web `client.ts`/`outcomeExplanationTemplates.ts` (014) + `RULES.md` rule IDs (001) are the four surfaces `check-outcome-explanations` validates together; this ticket lands the last (`UI.md`).
4. FOUNDATIONS §8 under audit for the bot-evidence docs: `BOT-STRATEGY-EVIDENCE-PACK.md` states `L2 not admitted` with the evidence still needed before a later L2; `COMPETENT-PLAYER.md` does not falsely claim L1 is a competent-human proxy.

## Architecture Check

1. Authoring MECHANICS/UI/AI after the behavior, web, and benchmarks land keeps the docs describing real surfaces rather than intentions, the OGC trailing-docs order.
2. No backwards-compatibility aliasing/shims — new game-local docs only.
3. `engine-core`/games behavior untouched (§3); docs-only.

## Verification Layers

1. Doc set complete vs templates; explicit `not applicable` rows over silent omissions -> manual review against `templates/*` + `node scripts/check-doc-links.mjs`.
2. `UI.md` outcome section present -> `node scripts/check-outcome-explanations.mjs` (now fully green).
3. Bot-evidence docs honest (`L2 not admitted`; no false competence claim) -> manual review against §8 / Appendix C.3.

## What to Change

### 1. Mechanic / UI / AI docs

`MECHANICS.md` (atlas-category inventory incl. private commitment, trick flow, negative scoring, surface budgets), `UI.md` (Rust/React boundary, viewer matrix, pass handoff, legal-only controls, the outcome/victory-explanation section, accessibility, no-leak DOM/storage rules), `AI.md` (L0/L1 contract, authorized fields, deterministic tie-breaks, explanation shape, hard exclusions).

### 2. Bot-evidence + release docs

`COMPETENT-PLAYER.md` (strategy landscape + measurable future-L2 criteria), `BOT-STRATEGY-EVIDENCE-PACK.md` (`L2 not admitted`, required fields as not-applicable/deferred), `PUBLIC-RELEASE-CHECKLIST.md` (official/IP/no-leak/catalog/rules-copy/renderer/replay/bot/benchmark/smoke/closeout receipt).

## Files to Touch

- `games/briar_circuit/docs/MECHANICS.md` (new)
- `games/briar_circuit/docs/UI.md` (new)
- `games/briar_circuit/docs/AI.md` (new)
- `games/briar_circuit/docs/COMPETENT-PLAYER.md` (new)
- `games/briar_circuit/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)
- `games/briar_circuit/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)

## Out of Scope

- The central atlas/source/spec status updates and exit-evidence run (GAT16BRICIRTRI-018).
- Any behavior, bot, or web code change — docs-only.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-outcome-explanations.mjs` — passes (`UI.md` outcome section present).
2. `node scripts/check-doc-links.mjs` — passes with the full doc set linked.
3. Manual review — doc set matches templates; `BOT-STRATEGY-EVIDENCE-PACK.md` reads `L2 not admitted`; no false L1-competence claim.

### Invariants

1. Docs describe behavior, never encode it (§5); no copied prose (§10).
2. Bot docs claim no L2 competence from L0/L1 evidence (§8).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-outcome-explanations.mjs`
2. `node scripts/check-doc-links.mjs`
3. A docs-scope verification is correct here; behavior is already proven by 004–016 and exercised by the closeout capstone (018).

## Outcome

Completed: 2026-06-21

Implemented the trailing Briar Circuit official-game documentation set:

- Added `games/briar_circuit/docs/MECHANICS.md` with atlas-category inventory, trick-taking second-use/defer posture, no kernel/helper promotion, UI/effect/bot/benchmark notes, and the Gate 17 third-use warning.
- Added `games/briar_circuit/docs/UI.md` with Rust/React boundary, four-seat layout, legal action mapping, pass flow, effect presentation, replay/dev-inspector boundaries, accessibility notes, and the required `Outcome / victory explanation` section.
- Added `games/briar_circuit/docs/AI.md` documenting L0/L1 shipped bots, exact information access, explanations, known weaknesses, tests, benchmarks, and the constrained public-default posture.
- Added `games/briar_circuit/docs/COMPETENT-PLAYER.md` with original strategy analysis, visible-signal boundaries, private-inference exclusions, kingmaking/moon-risk notes, examples, anti-examples, and future Level 2 candidate features.
- Added `games/briar_circuit/docs/BOT-STRATEGY-EVIDENCE-PACK.md` explicitly recording `L2 not admitted` and the future evidence required before any Level 2 policy can be coded.
- Added `games/briar_circuit/docs/PUBLIC-RELEASE-CHECKLIST.md` with official-game, IP, no-leak, replay/export, UI/accessibility, bot, benchmark, and closeout gate status.

Deviations:

- `node scripts/check-outcome-explanations.mjs` revealed that Briar's existing `RULES.md` used headings that did not match the checker contract and that `apps/web/src/wasm/client.ts` did not yet mirror an optional Briar outcome rationale type/field. Those inert contract surfaces were updated so the required acceptance command passes. No rule behavior, bot behavior, or renderer logic changed.
- The UI doc records the current web outcome behavior honestly: the board formats the Rust/WASM projected terminal view into the shared outcome panel and uses `briar_circuit.low_score_win`; `briar_circuit.moon_adjustment` and `briar_circuit.tied_low_continuation` remain documented template keys for rule parity rather than claimed current terminal-panel branches.

Verification:

- `node scripts/check-outcome-explanations.mjs` passed: `outcome-explanations check passed — 16 catalog games validated`.
- `node scripts/check-doc-links.mjs` passed (`Checked 27 markdown files`).
- `npm --prefix apps/web run build` passed after the optional Briar outcome-rationale TypeScript mirror change.
