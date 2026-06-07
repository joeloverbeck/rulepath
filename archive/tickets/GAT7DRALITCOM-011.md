# GAT7DRALITCOM-011: Bot strategy docs (Level 1)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — documentation only (`games/draughts_lite/docs/COMPETENT-PLAYER.md`, `games/draughts_lite/docs/BOT-STRATEGY-EVIDENCE-PACK.md`).
**Deps**: 001

## Problem

Per the official-game-gate pattern, bot-strategy docs precede the bot implementation so the Level 1 policy is specified before it is coded. This ticket writes `COMPETENT-PLAYER.md` (what a competent casual Draughts Lite player understands, and how the UI teaches it) and `BOT-STRATEGY-EVIDENCE-PACK.md` (the modest Level 1 heuristics, with test scenarios and an explicit disclaimer of search strength), so GAT7DRALITCOM-012 implements a documented, bounded policy rather than an ad hoc one.

## Assumption Reassessment (2026-06-07)

1. No bot code exists yet for `draughts_lite`; `games/directional_flip/docs/{COMPETENT-PLAYER.md,BOT-STRATEGY-EVIDENCE-PACK.md}` are the structural precedents, and the doc filenames map from `templates/{COMPETENT-PLAYER.md,BOT-STRATEGY-EVIDENCE-PACK.md}` (verified present). `games/draughts_lite/docs/RULES.md` (GAT7DRALITCOM-001) is the rules reference these docs build on.
2. The Level 1 scope is fixed by spec §R17 (acceptable heuristics: prefer winning move when cheaply detectable, prefer captures, prefer promotion / capture-to-promotion, prefer longer capture paths as a heuristic only, prefer creating/preserving kings, avoid obviously-hanging a king via a one-ply local check, deterministic seeded tie-break) and the explicit exclusions (minimax/alpha-beta/MCTS/playout/transposition/endgame DB/opening book/external engines/strength claims).
3. Cross-artifact boundary under audit: these docs are the specification GAT7DRALITCOM-012 implements and the scenarios the bot tests (012/013) assert; they must stay consistent with the §R17 heuristic list and the FOUNDATIONS §8 bot policy.
4. FOUNDATIONS §8 motivates this ticket: restate before writing — public bots are product opponents, not research AI; they must be competent, explainable, fair, deterministic under declared inputs, beatable, and use the same legal action API as humans. Public v1/v2 exclude MCTS/ISMCTS/Monte Carlo/ML/RL — the evidence pack must disclaim search strength, not promise it.

## Architecture Check

1. Specifying the bounded heuristic policy in docs before coding prevents scope creep into search at implementation time — the evidence pack is the contract that keeps Level 1 modest.
2. No backwards-compatibility shims; new docs.
3. `engine-core` and `game-stdlib` are untouched (§3/§4); these are game-local docs.

## Verification Layers

1. Heuristic scope -> manual review: `BOT-STRATEGY-EVIDENCE-PACK.md` lists only §R17-acceptable heuristics and explicitly excludes search methods.
2. Strength disclaimer -> manual review / FOUNDATIONS alignment check: the docs disclaim competitive strength and name the §8 fairness/beatability/determinism properties.
3. Doc-link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `COMPETENT-PLAYER.md`

Author from `templates/COMPETENT-PLAYER.md`: what a competent casual player should understand (forced capture, continuation, promotion, basic tactics) and how the Draughts Lite UI teaches those concepts.

### 2. `BOT-STRATEGY-EVIDENCE-PACK.md`

Author from `templates/BOT-STRATEGY-EVIDENCE-PACK.md`: the Level 1 heuristics with concrete test scenarios (capture preference, promotion preference, forced-continuation completion, deterministic seed behavior) and an explicit disclaimer of search strength and excluded methods.

## Files to Touch

- `games/draughts_lite/docs/COMPETENT-PLAYER.md` (new)
- `games/draughts_lite/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)

## Out of Scope

- Implementing the Level 0/1 bots (GAT7DRALITCOM-012).
- `AI.md` (the Level 0/1 behavior + exclusions doc lands in the trailing docs, GAT7DRALITCOM-021).
- Any search/learning bot design (forbidden; FOUNDATIONS §8, spec §R17).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — passes.
2. Manual review: the evidence pack lists only §R17 heuristics, names the test scenarios, and disclaims search strength.

### Invariants

1. The documented policy is bounded (no search) and uses the legal action API only (FOUNDATIONS §8; spec §R17).
2. The bot is documented as deterministic-under-seed, fair, and beatable (FOUNDATIONS §8).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; the heuristic scenarios become executable bot tests in GAT7DRALITCOM-012.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `node scripts/check-doc-links.mjs && bash scripts/boundary-check.sh`
3. Doc-link + boundary checks are the correct boundary for a docs-only ticket; scenario execution is verified by the bot tests in GAT7DRALITCOM-012.

## Outcome

Authored the Draughts Lite competent-player analysis and Level 1 bot strategy
evidence pack. The policy scope is bounded to Rust legal action paths, terminal
wins, capture/promotion heuristics, one-ply king-safety checks, material
tie-breaks, and deterministic seeded tie-breaks. The evidence pack explicitly
excludes minimax, alpha-beta, MCTS/ISMCTS, Monte Carlo/playouts, transposition
tables, opening books, endgame databases, ML/RL, runtime LLM move selection,
and strong-engine claims.

Verification passed:

1. Manual review: only §R17-acceptable Level 1 heuristics are listed; test
   scenarios and search-strength disclaimers are explicit.
2. `node scripts/check-doc-links.mjs`
3. `bash scripts/boundary-check.sh`
