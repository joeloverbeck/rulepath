# GAT15RIVLEDTEX-012: Bot-strategy docs — competent player and Level 2 evidence pack

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None (docs — `games/river_ledger/docs/COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`)
**Deps**: GAT15RIVLEDTEX-007

## Problem

River Ledger ships a Level 2 authored-policy bot. Per the official-game-gate pattern, the human strategy analysis (`COMPETENT-PLAYER.md`) and the formal Level 2 evidence pack (`BOT-STRATEGY-EVIDENCE-PACK.md`) are authored before the bot, so the bot implementation (013) has an explicit, authorized-information-only policy to encode.

## Assumption Reassessment (2026-06-14)

1. `games/poker_lite/docs/{COMPETENT-PLAYER,BOT-STRATEGY-EVIDENCE-PACK}.md` are the precedent (verified present); these docs are authored from authorized information only, before the L2 bot.
2. `specs/...-base.md` §4.2 and §5 G15-RL-008 fix the content: the L2 priority vector (terminal safety → legal availability → fold/call/check obligation → own-hole class → board texture → call price → live-opponent count → street/cap pressure → deterministic tie-break) and the forbidden hidden facts.
3. Cross-artifact boundary under audit: the evidence pack maps every bot input field to an authorized view field projected by `visibility.rs` (008); it is the contract the bot (013) and its no-leak tests implement.
4. FOUNDATIONS §8 (public bots are product opponents) motivates this ticket: the documented policy is competent, explainable, deterministic, beatable, and uses only the bot seat's authorized projection plus public state — no MCTS/ISMCTS/Monte Carlo/ML/RL, no hidden-state sampling, no solver.

## Architecture Check

1. Authoring the policy and its information boundary before the bot keeps the implementation a faithful encoding rather than an ad-hoc heuristic, matching the L2 gate precedent.
2. No backwards-compatibility aliasing/shims — new docs.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4) — docs only.

## Verification Layers

1. Every L2 input field maps to an authorized view field -> manual cross-check against `visibility.rs` (008) projection fields.
2. Forbidden hidden facts are enumerated and excluded -> FOUNDATIONS §8 alignment check.
3. Doc-link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `games/river_ledger/docs/COMPETENT-PLAYER.md`

Human strategy analysis written only from authorized information, feeding the L2 policy.

### 2. `games/river_ledger/docs/BOT-STRATEGY-EVIDENCE-PACK.md`

Formal L2 evidence pack: enumerated inputs, forbidden hidden facts, priority vector, opponent-count adjustments, and deterministic tie-breaks.

## Files to Touch

- `games/river_ledger/docs/COMPETENT-PLAYER.md` (new)
- `games/river_ledger/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)

## Out of Scope

- The bot implementation, `AI.md`, and bot tests (GAT15RIVLEDTEX-013).
- Any bot search/learning class (forbidden by §8).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — passes with the new docs linked.
2. Every documented L2 input maps to an authorized projection field; no forbidden hidden fact appears.
3. Manual review confirms the policy is deterministic, explainable, and beatable (§8).

### Invariants

1. The documented bot uses only authorized views + public state (§8/§11).
2. No MCTS/ISMCTS/Monte Carlo/ML/RL/solver is proposed (§8).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -n 'priority' games/river_ledger/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
3. A doc-link + manual review is the correct boundary; the bot's authorized-view-only behavior is enforced by tests in GAT15RIVLEDTEX-013.
