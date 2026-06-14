# GAT15RIVLEDTEX-019: Trailing game docs — UI, admission receipt, public-release checklist

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None (docs — `games/river_ledger/docs/UI.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `PUBLIC-RELEASE-CHECKLIST.md`)
**Deps**: GAT15RIVLEDTEX-018

## Problem

After implementation evidence exists, River Ledger must complete its trailing official-game docs: the product UI plan (`UI.md`), the final implementation-admission receipt, and the public-release checklist — so the game has the full `docs/OFFICIAL-GAME-CONTRACT.md` document set before public admission.

## Assumption Reassessment (2026-06-14)

1. `games/poker_lite/docs/{UI,GAME-IMPLEMENTATION-ADMISSION,PUBLIC-RELEASE-CHECKLIST}.md` are the precedent (verified present); `MECHANICS.md`/`RULE-COVERAGE.md`/`AI.md` are already authored (002/013/015), so this ticket completes the remaining trailing docs.
2. `specs/...-base.md` §4.2 and §10.4 fix the content; `GAME-IMPLEMENTATION-ADMISSION.md` was created as the pre-coding receipt in 002 and is reconciled to the post-implementation receipt here.
3. Cross-artifact boundary under audit: `UI.md` documents the renderer (017), the N-seat viewer/pairwise no-leak matrix (009), and observer/seat-private projections (008); the admission/release docs reference the implemented surfaces and command evidence across 003–018.
4. FOUNDATIONS §6 (evidence-heavy) + §10 (IP) motivate this ticket: the docs record the rules/source/mechanic/coverage/UI/bot/no-leak/bench evidence and a neutral, original, no-casino visual direction.

## Architecture Check

1. Completing the trailing docs once implementation evidence exists keeps them accurate rather than speculative, matching the OGC trailing-docs order.
2. No backwards-compatibility aliasing/shims — docs only.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4).

## Verification Layers

1. UI plan covers the N-seat viewer matrix, pairwise no-leak matrix, observer/seat-private projections, and safe outcome explanation -> manual review against 008/009/017.
2. Admission receipt + release checklist cite real command evidence -> cross-check against the §7.1 acceptance suite.
3. Doc-link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `games/river_ledger/docs/UI.md`

Product UI plan: N-seat viewer matrix, pairwise no-leak matrix, observer/seat-private projections, surface budget, safe outcome explanation, no-casino visual direction.

### 2. `GAME-IMPLEMENTATION-ADMISSION.md` (final) + `PUBLIC-RELEASE-CHECKLIST.md`

Reconcile the admission receipt to post-implementation status; author the public-release checklist (IP, no-leak, catalog, docs, e2e, presentation-copy, smoke, replay export/import, bot-boundary checks).

## Files to Touch

- `games/river_ledger/docs/UI.md` (new)
- `games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md` (modify; created by 002)
- `games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)

## Out of Scope

- Final mechanic-atlas pressure review (GAT15RIVLEDTEX-020).
- The acceptance sweep + spec/index `Done`-flip (GAT15RIVLEDTEX-021).
- Any code change.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — passes with the trailing docs linked.
2. The full OGC document set exists for `games/river_ledger/docs/`.
3. Manual review confirms the release checklist names IP/no-leak/catalog/e2e/bot-boundary checks.

### Invariants

1. Docs reflect implemented evidence, not speculation (§6).
2. UI direction is neutral and original; no casino trade dress (§10).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `ls games/river_ledger/docs/` — confirms the full 13-doc OGC set.
3. A doc-link + manual review is the correct boundary; the acceptance command suite is run in GAT15RIVLEDTEX-021.
