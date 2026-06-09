# GAT101PLATRI-019: Trailing game docs (mechanics, UI, how-to-play, public-release checklist)

**Status**: COMPLETE
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — new `games/plain_tricks/docs/{MECHANICS.md,UI.md,HOW-TO-PLAY.md,PUBLIC-RELEASE-CHECKLIST.md}`. No code.
**Deps**: GAT101PLATRI-017

## Problem

The official-game doc set requires the trailing documents: `MECHANICS.md` (mechanic inventory), `UI.md` (with the mandatory "Outcome / victory explanation" section), `HOW-TO-PLAY.md` (player prose for the shared How to Play surface), and `PUBLIC-RELEASE-CHECKLIST.md`. These describe the implemented game and the rendered surface, so they land after the renderer.

## Assumption Reassessment (2026-06-09)

1. `games/poker_lite/docs/{MECHANICS.md,UI.md,HOW-TO-PLAY.md,PUBLIC-RELEASE-CHECKLIST.md}` exist as templates; the implemented behavior (rules, effects, views, renderer) is stable as of GAT101PLATRI-007/008/009/017. `RULES.md`/`SOURCES.md`/`AI.md`/`RULE-COVERAGE.md`/`BENCHMARKS.md`/bot-strategy docs already exist from earlier tickets.
2. Spec §4 ("Per-game documentation") requires `UI.md` to include the "Outcome / victory explanation" section per `docs/OFFICIAL-GAME-CONTRACT.md` §5/§10; `HOW-TO-PLAY.md` is player prose wired to the shared How to Play surface.
3. Shared boundary under audit: the `docs/OFFICIAL-GAME-CONTRACT.md` per-game deliverable set — this ticket completes the remaining required docs so the set is internally consistent.
4. FOUNDATIONS §7 (UI is central product work; outcome-explanation surface) and §10 (original IP) are under audit — `UI.md` documents the Rust-owned outcome rationale the TS shell renders without computing.

## Architecture Check

1. Authoring `MECHANICS`/`UI`/`HOW-TO-PLAY`/`PUBLIC-RELEASE-CHECKLIST` after the renderer (vs. up front) lets them describe the actual implemented surface accurately; co-locating the outcome-explanation section in `UI.md` satisfies OGC §5/§10.
2. No backwards-compatibility aliasing/shims; docs only.
3. `engine-core` untouched; no `game-stdlib` change.

## Verification Layers

1. `UI.md` contains the "Outcome / victory explanation" section naming the terminal variants, the Rust field/effect carrying the decisive cause, and the no-leak coverage -> manual review against OGC §5/§10 + grep for the section header.
2. Doc-link integrity -> `node scripts/check-doc-links.mjs`.
3. IP posture (original prose; no trade dress) -> manual IP audit.
4. Doc set internally consistent (all OGC-required docs present) -> manual review against `docs/OFFICIAL-GAME-CONTRACT.md`.

## What to Change

### 1. `games/plain_tricks/docs/MECHANICS.md`

Mechanic inventory: follow-suit legality, trick resolution, trick-winner-leads, deal rotation, trick scoring, hidden hand/tail, viewer-filtered deal — each marked local-only.

### 2. `games/plain_tricks/docs/UI.md`

Document the rendered surface and the mandatory "Outcome / victory explanation" section (terminal variants TrickWin/Split; the Rust public/terminal field or terminal effect carrying the decisive cause; per-seat breakdown; hidden-info redaction; smoke/no-leak coverage).

### 3. `games/plain_tricks/docs/HOW-TO-PLAY.md`

Player prose for the shared How to Play surface (objective, follow-suit, winning tricks, scoring, rounds).

### 4. `games/plain_tricks/docs/PUBLIC-RELEASE-CHECKLIST.md`

Public-release checklist (rules/IP/no-leak/bot/benchmarks/docs all green).

## Files to Touch

- `games/plain_tricks/docs/MECHANICS.md` (new)
- `games/plain_tricks/docs/UI.md` (new)
- `games/plain_tricks/docs/HOW-TO-PLAY.md` (new)
- `games/plain_tricks/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)

## Out of Scope

- Capstone atlas/status reconciliation and exit evidence (GAT101PLATRI-020).
- `RULES.md`/`SOURCES.md`/`AI.md`/`RULE-COVERAGE.md`/`BENCHMARKS.md`/bot-strategy docs (earlier tickets).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the four new docs present.
2. `grep -n "Outcome / victory explanation" games/plain_tricks/docs/UI.md` resolves.
3. Manual review confirms the OGC per-game doc set is complete and internally consistent.

### Invariants

1. `UI.md` documents a Rust-owned outcome rationale the TS shell renders without computing (FOUNDATIONS §2/§7; OGC §5/§10).
2. All public docs use original prose; no copied prose or trade dress (FOUNDATIONS §10).

## Test Plan

### New/Modified Tests

1. `None — documentation ticket; verification is command-based (doc-link check) and manual OGC review per Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `node scripts/check-catalog-docs.mjs`
3. Doc-link + catalog checks are the correct boundary; the outcome-explanation surface itself is exercised by the e2e smoke in GAT101PLATRI-018.

## Outcome

Completed 2026-06-09. Added the trailing Plain Tricks official-game docs: mechanics inventory, UI contract with the required "Outcome / victory explanation" section, and public-release checklist. `HOW-TO-PLAY.md` was already present from GAT101PLATRI-017 and remains part of the completed doc set.

Verification:

1. `node scripts/check-doc-links.mjs`
2. `node scripts/check-catalog-docs.mjs`
3. `grep -n "Outcome / victory explanation" games/plain_tricks/docs/UI.md`
