# GAT7DRALITCOM-021: Trailing game docs (MECHANICS / UI / AI / ADMISSION / PUBLIC-RELEASE-CHECKLIST)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — documentation only (`games/draughts_lite/docs/{MECHANICS.md,UI.md,AI.md,GAME-IMPLEMENTATION-ADMISSION.md,PUBLIC-RELEASE-CHECKLIST.md}`).
**Deps**: 012, 015, 016, 018, 019

## Outcome

Added the trailing Draughts Lite official-game docs against the shipped implementation:

1. `MECHANICS.md` records the compound action path contract (`from/`, `to/`, `jump/`), tree phases, metadata keys, effects, diagnostics, board coordinates, primitive-pressure posture, and local extraction decisions.
2. `UI.md` records the React/WASM renderer contract, pointer/keyboard path flow, forced-capture/continuation cues, reduced-motion behavior, replay path rendering, and TypeScript no-legality boundary.
3. `AI.md` records the Level 0 random legal bot, Level 1 authored policy `draughts_lite_level1_v1`, public explanation contract, and public v1/v2 exclusions.
4. `GAME-IMPLEMENTATION-ADMISSION.md` maps Draughts Lite to the official-game contract, boundary discipline, evidence surfaces, CI/tooling, and admission constraints.
5. `PUBLIC-RELEASE-CHECKLIST.md` is filled as a release-candidate checklist covering source/IP, bundle inspection, no-leak, replay/export, UI polish, accessibility, bot explanations, tests, traces, benchmarks, and constraints.
6. `BOT-STRATEGY-EVIDENCE-PACK.md` was truth-updated to stop calling `RULE-COVERAGE.md`, `MECHANICS.md`, and `AI.md` incomplete and to align benchmark status with the current smoke-floor posture.

Verification passed on 2026-06-07:

1. `node scripts/check-doc-links.mjs`
2. `cargo run -p rule-coverage -- --game draughts_lite`
3. `git diff --check`

## Problem

The official-game admission package's remaining docs describe surfaces that must exist coherently before they can be written accurately: `MECHANICS.md` (action-path segments, tree phases, effects, diagnostics, board coordinates), `UI.md` (board/pointer/keyboard/forced-continuation/animation/reduced-motion/no-legality), `AI.md` (Level 0/1 behavior + exclusions), `GAME-IMPLEMENTATION-ADMISSION.md` (how the game meets the OGC + boundary discipline), and `PUBLIC-RELEASE-CHECKLIST.md` (filled, not a template shell). This trailing docs ticket lands them atomically once their implementation surfaces ship.

## Assumption Reassessment (2026-06-07)

1. The documented surfaces are produced by GAT7DRALITCOM-006 (segments/tree phases), 008 (effects), 007 (diagnostics), 012 (bots), 016 (WASM), 018/019 (UI/a11y/reduced motion). `games/directional_flip/docs/{MECHANICS,UI,AI,GAME-IMPLEMENTATION-ADMISSION,PUBLIC-RELEASE-CHECKLIST}.md` are the precedents; filenames map from `templates/GAME-{MECHANICS,UI,AI,IMPLEMENTATION-ADMISSION}.md` and `templates/PUBLIC-RELEASE-CHECKLIST.md` (verified present).
2. The required doc content is fixed by spec §R20 "Required doc content" (MECHANICS: segments/phases/effects/diagnostics/coords; UI: rendering/pointer/keyboard/forced-continuation/animation/reduced-motion/no-legality; AI: Level 0/1 + exclusions; ADMISSION: OGC + boundary; PUBLIC-RELEASE-CHECKLIST: complete). `docs/OFFICIAL-GAME-CONTRACT.md` is the admission contract.
3. Cross-artifact boundary under audit: these docs cite the implemented segment vocabulary, effect/diagnostic names, bot policy, and UI behavior; they must match the shipped code (no drift). This is a cross-cutting docs ticket — it lands after the surfaces it documents exist.
4. FOUNDATIONS §6 motivates this ticket: restate before writing — official games are evidence-heavy; `MECHANICS.md` must be useful to future maintainers and `PUBLIC-RELEASE-CHECKLIST.md` must be genuinely complete for public-polish admission, not a template shell (spec §R20).

## Architecture Check

1. Landing these docs after their surfaces exist (vs. co-locating each with its implementing ticket) avoids a staleness window — the segment/effect/diagnostic names and UI behavior are final, so the docs cite real artifacts.
2. No backwards-compatibility shims; new docs.
3. `engine-core`/`game-stdlib` untouched (§3/§4); game-local docs.

## Verification Layers

1. Content completeness -> manual review against spec §R20: each doc covers its required sections.
2. Code-accuracy -> grep-proof: segment names (`from/`,`to/`,`jump/`), effect/diagnostic names, and bot levels cited in the docs match the shipped `actions.rs`/`effects.rs`/`rules.rs`/`bots.rs`.
3. Admission -> manual review against `docs/OFFICIAL-GAME-CONTRACT.md`: `GAME-IMPLEMENTATION-ADMISSION.md` maps each OGC deliverable to its evidence.
4. Doc-link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. MECHANICS / UI / AI

Author `MECHANICS.md` (action-path segments, tree phases, effects, diagnostics, board coordinate conventions), `UI.md` (board rendering, pointer + keyboard flow, forced-continuation UX, animation, effect feedback, reduced motion, no-legality-in-TypeScript boundary), and `AI.md` (Level 0/1 behavior and exclusions).

### 2. ADMISSION & PUBLIC-RELEASE-CHECKLIST

Author `GAME-IMPLEMENTATION-ADMISSION.md` (OGC + boundary-discipline mapping) and a fully completed `PUBLIC-RELEASE-CHECKLIST.md`.

## Files to Touch

- `games/draughts_lite/docs/MECHANICS.md` (new)
- `games/draughts_lite/docs/UI.md` (new)
- `games/draughts_lite/docs/AI.md` (new)
- `games/draughts_lite/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)
- `games/draughts_lite/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)

## Out of Scope

- `RULES.md`/`SOURCES.md` (GAT7DRALITCOM-001), bot-strategy docs (011), `BENCHMARKS.md` (015), `RULE-COVERAGE.md` (017), `PRIMITIVE-PRESSURE-LEDGER.md` (002) — authored with their validators/surfaces.
- The mechanic-atlas finalize and spec/index `Done`-flip (GAT7DRALITCOM-022).
- Any code change — docs only.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — passes.
2. `cargo run -p rule-coverage -- --game draughts_lite` — still passes (MECHANICS/coverage docs consistent with code).

### Invariants

1. Docs cite real shipped artifacts (segments, effects, diagnostics, bot levels) — no drift from code (FOUNDATIONS §6; spec §R20).
2. `PUBLIC-RELEASE-CHECKLIST.md` is complete, not a template shell (spec §R20).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is `check-doc-links.mjs`, `rule-coverage`, and manual review against shipped code + OGC.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `node scripts/check-doc-links.mjs && cargo run -p rule-coverage -- --game draughts_lite`
3. Doc-link + rule-coverage are the correct boundary for trailing docs; full exit-criteria closure is the capstone (GAT7DRALITCOM-022).
