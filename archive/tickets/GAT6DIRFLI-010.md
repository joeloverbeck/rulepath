# GAT6DIRFLI-010: Bot strategy docs (Level 2-lite)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None (documentation-only — `games/directional_flip/docs/COMPETENT-PLAYER.md`, `games/directional_flip/docs/BOT-STRATEGY-EVIDENCE-PACK.md`)
**Deps**: 001

## Problem

Per FOUNDATIONS §8 and spec §11.1, **no Level 2-lite bot code may be committed before `COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md` exist and cite strategy evidence.** This ticket authors those two strategy documents — the human-understandable strategic principles and the source-backed policy evidence — so the Level 2-lite bot (GAT6DIRFLI-011) has a documented, reviewable basis. It writes no code.

## Assumption Reassessment (2026-06-07)

1. `games/column_four/docs/COMPETENT-PLAYER.md` and `games/column_four/docs/BOT-STRATEGY-EVIDENCE-PACK.md` are the structural precedents; `templates/COMPETENT-PLAYER.md` and `templates/BOT-STRATEGY-EVIDENCE-PACK.md` are the canonical templates. The target `games/directional_flip/docs/` directory exists from GAT6DIRFLI-001.
2. Spec §11.2 (Level 2-lite allowed/forbidden boundary), §11.3 (recommended lexicographic policy outline), and §4 (strategy research: greed trap, corners, X/C squares, mobility, frontier, stability) are authoritative. The strategy sources are the Othello Belgium and Nederlandse Othello Vereniging guides (spec §22.1).
3. Cross-artifact boundary under audit: these docs are the contract the Level 2-lite bot (GAT6DIRFLI-011) implements and its determinism tests pin. The lexicographic order documented here must match the order the bot code and `AI.md` (GAT6DIRFLI-020) describe.
4. FOUNDATIONS §8 (public bots are product opponents, not research AI) motivates this ticket: restate before authoring — public bots must be competent, explainable, fair, human-plausible, deterministic under declared inputs, and beatable; personality may vary policy order/risk/tie-breakers but never means cheating, hidden-state access, or giant weight soup. The docs must bound the bot to one-ply visible-feature evaluation with no minimax/MCTS/ML/RL/LLM (spec §11.2).

## Architecture Check

1. Authoring strategy docs before the bot enforces the §8 "docs precede Level 2 code" gate and keeps the bot's rationale auditable against a written policy rather than reverse-rationalized from code.
2. No backwards-compatibility shims; new documents.
3. `engine-core` / `game-stdlib` untouched (documentation only).

## Verification Layers

1. Docs-precede-code gate -> FOUNDATIONS alignment check (§8): both docs exist and cite strategy evidence before GAT6DIRFLI-011 is worked.
2. Policy completeness -> manual review against spec §11.2/§11.3: allowed/forbidden boundary, lexicographic order, deterministic tie-break, and "no search" limits are all documented.
3. Source grounding -> manual review: claims trace to the cited strategy sources (spec §22.1), not invented heuristics.

## What to Change

### 1. `games/directional_flip/docs/COMPETENT-PLAYER.md`

Human-understandable strategic principles for Level 2-lite: corners as stable anchors, X-square/C-square danger near open corners, own/opponent mobility, stable edge/corner extension, frontier discs, and phase-aware disc-count tie-break (opening greed is a trap). Follow `templates/COMPETENT-PLAYER.md`.

### 2. `games/directional_flip/docs/BOT-STRATEGY-EVIDENCE-PACK.md`

Sources, representative positions, expected policy priorities, anti-patterns, the deterministic tie-break policy, and explicit "no search" boundaries (spec §11.2). Follow `templates/BOT-STRATEGY-EVIDENCE-PACK.md`.

## Files to Touch

- `games/directional_flip/docs/COMPETENT-PLAYER.md` (new)
- `games/directional_flip/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)

## Out of Scope

- Bot Rust code (GAT6DIRFLI-011).
- `AI.md` (GAT6DIRFLI-020), which documents bot levels/rationale/determinism after implementation.

## Acceptance Criteria

### Tests That Must Pass

1. `test -f games/directional_flip/docs/COMPETENT-PLAYER.md && test -f games/directional_flip/docs/BOT-STRATEGY-EVIDENCE-PACK.md` — both exist.
2. Manual review: lexicographic policy outline and "no search" boundary are documented and source-cited.
3. `node scripts/check-doc-links.mjs` — links resolve.

### Invariants

1. Strategy docs exist and cite evidence before any Level 2-lite code is committed (FOUNDATIONS §8, spec §11.1).
2. The documented policy excludes all forbidden search/ML/LLM approaches (FOUNDATIONS §8, spec §11.2).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and the bot's determinism/legality tests are owned by GAT6DIRFLI-011.`

### Commands

1. `test -f games/directional_flip/docs/COMPETENT-PLAYER.md && test -f games/directional_flip/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
2. `node scripts/check-doc-links.mjs`
3. Doc presence + link check is the correct boundary; the policy is exercised by the bot tests in GAT6DIRFLI-011.

## Outcome

Authored `games/directional_flip/docs/COMPETENT-PLAYER.md` and `games/directional_flip/docs/BOT-STRATEGY-EVIDENCE-PACK.md` before Level 2-lite bot code. The documents cite the project rule/spec authority plus the recorded strategy references, define the corner/mobility/frontier/phase-aware count policy, spell out deterministic tie-break behavior, and exclude minimax/search/playouts/ML/RL/LLM.

Verification:

1. `test -f games/directional_flip/docs/COMPETENT-PLAYER.md && test -f games/directional_flip/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
2. `node scripts/check-doc-links.mjs`
