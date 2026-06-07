# GAT6DIRFLI-020: Trailing game docs

**Status**: COMPLETE
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None (documentation-only — `games/directional_flip/docs/{MECHANICS,UI,AI,GAME-IMPLEMENTATION-ADMISSION,PUBLIC-RELEASE-CHECKLIST}.md`).
**Deps**: 011, 014, 015, 017

## Problem

The official-game contract requires the trailing documentation set that describes the implemented game: mechanic inventory, UI design, bot levels/determinism, the admission checklist, and the public-release checklist (FOUNDATIONS §6, spec §8.2, `docs/OFFICIAL-GAME-CONTRACT.md`). These land after the surfaces they describe exist, and `MECHANICS.md` must link to the primitive-pressure ledger (GAT6DIRFLI-002). This ticket authors those five documents.

## Assumption Reassessment (2026-06-07)

1. The `games/column_four/docs/` set is the precedent (`MECHANICS.md`, `UI.md`, `AI.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `PUBLIC-RELEASE-CHECKLIST.md`); templates exist under `templates/` (`GAME-MECHANICS.md`, `GAME-UI.md`, `GAME-AI.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `PUBLIC-RELEASE-CHECKLIST.md`). The described surfaces exist: bots (011), benches (014), wasm (015), renderer (017).
2. Spec §8.2 (required doc content), §10 (MECHANICS must include primitive-pressure entries linking the ledger), and `docs/OFFICIAL-GAME-CONTRACT.md` (mechanic-inventory categories) are authoritative. `RULES.md`/`SOURCES.md` (001), strategy docs (010), `RULE-COVERAGE.md` (016), and `BENCHMARKS.md` (014) are authored elsewhere and are out of scope here.
3. Cross-artifact boundary under audit: `MECHANICS.md` ↔ `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` (002) and `docs/MECHANIC-ATLAS.md` — the inventory must reference the recorded extraction decision. `AI.md` ↔ the bot code (011) + strategy docs (010); `UI.md` ↔ the renderer (017).
4. FOUNDATIONS §6 (official games are evidence-heavy) motivates this ticket: restate before authoring — the doc set is required evidence; `GAME-IMPLEMENTATION-ADMISSION.md` admits the game as official only after all evidence passes, and `PUBLIC-RELEASE-CHECKLIST.md` must pass before picker exposure (gated by GAT6DIRFLI-021), so these docs describe the as-built game accurately.

## Architecture Check

1. Authoring the descriptive docs after the surfaces exist (rather than speculatively) keeps the inventory/UI/AI docs accurate to as-built code; `MECHANICS.md` linking the ledger closes the §10 primitive-pressure loop in the per-game docs.
2. No backwards-compatibility shims; new documents.
3. `engine-core` / `game-stdlib` untouched (documentation only).

## Verification Layers

1. Doc completeness -> manual review against spec §8.2 + `docs/OFFICIAL-GAME-CONTRACT.md`: each doc has its required content (mechanic inventory categories, UI accessibility/preview/effect ownership, bot levels/determinism/exclusions, admission checklist, release checklist).
2. Ledger linkage -> codebase grep-proof: `MECHANICS.md` links to `PRIMITIVE-PRESSURE-LEDGER.md` (spec §10/§8.3).
3. Doc-link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. MECHANICS.md

Mechanic inventory across the `docs/OFFICIAL-GAME-CONTRACT.md` / mechanic-atlas categories (topology, action shape, turn/phase, randomness, visibility, resource, movement/capture/placement, pattern/directional scanning, scoring, semantic effects, UI, bots, replay, benchmark pressure) plus repeated-shape comparison vs `three_marks`/`column_four`, linking the primitive-pressure ledger.

### 2. UI.md / AI.md

`UI.md`: public visual language, accessibility behavior, keyboard mapping, preview/effect responsibilities, reduced motion, no-leak notes. `AI.md`: bot levels, rationale shape, legal-API validation, deterministic seed/tie-break behavior, exclusions.

### 3. Admission & release checklists

`GAME-IMPLEMENTATION-ADMISSION.md` (official-game admission checklist) and `PUBLIC-RELEASE-CHECKLIST.md` (release evidence; must pass before picker exposure).

## Files to Touch

- `games/directional_flip/docs/MECHANICS.md` (new)
- `games/directional_flip/docs/UI.md` (new)
- `games/directional_flip/docs/AI.md` (new)
- `games/directional_flip/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)
- `games/directional_flip/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)

## Out of Scope

- `RULES.md`/`SOURCES.md` (001), strategy docs (010), `RULE-COVERAGE.md` (016), `BENCHMARKS.md` (014).
- The mechanic-atlas finalize, status flips, and public exposure (GAT6DIRFLI-021).
- Any code change (documentation only).

## Acceptance Criteria

### Tests That Must Pass

1. `test -f` for all five docs returns present.
2. `grep -q 'PRIMITIVE-PRESSURE-LEDGER' games/directional_flip/docs/MECHANICS.md` — ledger linkage present.
3. `node scripts/check-doc-links.mjs` — links resolve.

### Invariants

1. The doc set accurately describes the as-built game and covers the `docs/OFFICIAL-GAME-CONTRACT.md` categories (FOUNDATIONS §6).
2. `MECHANICS.md` references the recorded primitive-pressure decision (spec §10).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (presence + doc-link + grep) and the described surfaces are tested by their own tickets.`

### Commands

1. `for d in MECHANICS UI AI GAME-IMPLEMENTATION-ADMISSION PUBLIC-RELEASE-CHECKLIST; do test -f games/directional_flip/docs/$d.md || echo "MISSING $d"; done`
2. `node scripts/check-doc-links.mjs`
3. Presence + doc-link checks are the correct boundary; the docs describe surfaces verified by their own tickets.

## Outcome

Completed 2026-06-07. Added the trailing Directional Flip official-game docs: mechanics inventory, UI notes, AI notes, implementation admission receipt, and public release checklist. `MECHANICS.md` links the primitive-pressure ledger and records the third-use rectangular coordinate/ray decision.

Verification:

1. `for d in MECHANICS UI AI GAME-IMPLEMENTATION-ADMISSION PUBLIC-RELEASE-CHECKLIST; do test -f games/directional_flip/docs/$d.md || echo MISSING $d; done`
2. `grep -q 'PRIMITIVE-PRESSURE-LEDGER' games/directional_flip/docs/MECHANICS.md`
3. `node scripts/check-doc-links.mjs`
4. `git diff --check`
