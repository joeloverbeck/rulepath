# GAT91SECDRACOM-017: secret_draft trailing game docs (MECHANICS/UI/AI/ADMISSION/PUBLIC-RELEASE/COMPETENT-PLAYER/BOT-STRATEGY)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — authors `games/secret_draft/docs/{MECHANICS,UI,AI,GAME-IMPLEMENTATION-ADMISSION,PUBLIC-RELEASE-CHECKLIST,COMPETENT-PLAYER,BOT-STRATEGY-EVIDENCE-PACK}.md` only.
**Deps**: GAT91SECDRACOM-008, GAT91SECDRACOM-015

## Problem

The official-game contract's 11-doc set must be complete and consistent with implemented behavior. RULES/SOURCES landed in GAT91SECDRACOM-001, BENCHMARKS in 011, RULE-COVERAGE in 012; this ticket authors the remaining seven trailing docs, including the bot-evidence docs whose fixtures come from the Level 1 bot tests.

## Assumption Reassessment (2026-06-08)

1. `games/token_bazaar/docs/` carries the full 11-doc set; this ticket instantiates the remaining seven for `secret_draft`: `MECHANICS.md`, `UI.md`, `AI.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`. Templates exist: `templates/GAME-MECHANICS.md`, `GAME-UI.md`, `GAME-AI.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md` (verified).
2. The implemented bot (GAT91SECDRACOM-008) and board (GAT91SECDRACOM-015) are inputs: `COMPETENT-PLAYER.md` / `BOT-STRATEGY-EVIDENCE-PACK.md` document the `VeiledDraftLevel1Bot` policy + fixtures (placed trailing because a Level 1 evidence pack documents the bot's already-produced test fixtures); `UI.md` documents the rendered board; `MECHANICS.md` the mechanic inventory; `AI.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `PUBLIC-RELEASE-CHECKLIST.md` the remaining contract docs.
3. Cross-artifact boundary under audit: docs must match implemented behavior (no drift) and stay IP-clean (§10). `BOT-STRATEGY-EVIDENCE-PACK.md` must reflect the actual Level 1 policy/rationale/no-leak posture, not an idealized one.
4. §6 evidence-heavy + §11 docs-coverage are the motivating principles: restate before trusting spec — every official game must carry the full doc set consistent with behavior; a browser-playable game without complete docs is a demo shell, not an official game. Bot explanations documented here cite only public facts (no hidden-state phrasing).
5. No Level 2 bot is shipped (spec: not required); `BOT-STRATEGY-EVIDENCE-PACK.md` documents the Level 1 policy. If maintainers later elect Level 2, the pack expands then.

## Architecture Check

1. Trailing the descriptive docs after the implementing tickets is cleaner than front-loading them: they describe real behavior (UI, bot rationale, mechanic inventory) rather than intended behavior, avoiding doc/code drift.
2. No backwards-compatibility aliasing/shims — new doc files.
3. `engine-core` untouched; docs are game-local prose. No `game-stdlib` change.

## Verification Layers

1. Doc completeness -> all seven docs present; combined with 001/011/012 the 11-doc set is complete (manual review against `docs/OFFICIAL-GAME-CONTRACT.md`).
2. Behavior consistency -> manual review that MECHANICS/UI/bot docs match GAT91SECDRACOM-005/006/008/015 behavior.
3. IP cleanliness -> manual §10 audit (original prose, no copied content).
4. Doc-link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. The seven trailing docs

Instantiate from the corresponding templates and fill with `secret_draft`-accurate content: `MECHANICS.md` (mechanic inventory: simultaneous commitment, reveal, drafting-with-removal, conflict fallback, scoring), `UI.md` (board, pending/reveal UX, reduced motion, no-leak anchors), `AI.md` (bot overview), `GAME-IMPLEMENTATION-ADMISSION.md` (admission evidence pointers), `PUBLIC-RELEASE-CHECKLIST.md`, `COMPETENT-PLAYER.md` (how a competent player approaches Veiled Draft), `BOT-STRATEGY-EVIDENCE-PACK.md` (Level 1 policy, rationale examples, no-leak tests, fixtures from GAT91SECDRACOM-008).

## Files to Touch

- `games/secret_draft/docs/MECHANICS.md` (new)
- `games/secret_draft/docs/UI.md` (new)
- `games/secret_draft/docs/AI.md` (new)
- `games/secret_draft/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)
- `games/secret_draft/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)
- `games/secret_draft/docs/COMPETENT-PLAYER.md` (new)
- `games/secret_draft/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)

## Out of Scope

- RULES/SOURCES (GAT91SECDRACOM-001), BENCHMARKS (011), RULE-COVERAGE (012).
- Repository-level docs: spec index flip, MECHANIC-ATLAS first-use, progress.md, root README catalog (GAT91SECDRACOM-018 / 016).
- Any code change.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with all seven docs present.
2. Manual review confirms the 11-doc set is complete and consistent with implemented behavior.
3. `BOT-STRATEGY-EVIDENCE-PACK.md` rationale examples cite only public facts (no hidden-state phrasing).

### Invariants

1. The full official-game doc set exists and matches behavior (§6/§11 evidence coverage).
2. Docs are original and IP-clean (§10).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is check-doc-links plus manual completeness/behavior/IP review.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `ls games/secret_draft/docs/` confirms all 11 docs present (7 from this ticket + RULES/SOURCES + BENCHMARKS + RULE-COVERAGE).
3. Manual review is the correct boundary for prose docs; behavior consistency is cross-checked against the implementing tickets.
