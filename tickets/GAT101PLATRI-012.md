# GAT101PLATRI-012: Bot-strategy evidence docs (competent-player and evidence pack)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `games/plain_tricks/docs/COMPETENT-PLAYER.md`, `games/plain_tricks/docs/BOT-STRATEGY-EVIDENCE-PACK.md`. No Rust code.
**Deps**: GAT101PLATRI-011

## Problem

This gate requires a Level 2 authored-policy bot. Per `templates/README.md` and `docs/OFFICIAL-GAME-CONTRACT.md`, `COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md` MUST be authored **before** the Level 2 bot is coded — they are the design input that makes the policy explainable and fair. This ticket authors them so GAT101PLATRI-013 can implement against a recorded strategy.

## Assumption Reassessment (2026-06-09)

1. `templates/README.md` line "`BOT-STRATEGY-EVIDENCE-PACK.md` before Level 2 bot coding" and the template index confirm the precedence; `games/poker_lite/docs/{COMPETENT-PLAYER.md,BOT-STRATEGY-EVIDENCE-PACK.md}` exist as exemplars. The rules/legal-tree/views these docs reason about are stable as of GAT101PLATRI-006/007/009/011.
2. Spec §4 (docs are mandatory and precede Level 2 coding) and appendix B2 supply the allowed/forbidden bot input and the heuristic sketch (`plain-tricks-level2-v1`). Note this ordering **diverges** from the `poker_lite` ticket order (which trailed bot docs after the bot); the divergence is mandated by spec §4 and templates for a Level 2 policy.
3. Shared boundary under audit: the bot input contract (own seat, own hand, legal tree, public trick/score/void history) and the §8/§11 no-hidden-state rule the evidence pack must encode before code exists.
4. FOUNDATIONS §8 (public bots are competent, explainable, fair, deterministic, beatable; no MCTS/ISMCTS/ML/RL/hidden-state) is the principle under audit, restated before authoring.

## Architecture Check

1. Authoring strategy docs before the bot (vs. documenting after) forces the policy to be designed for fairness/explainability and gives GAT101PLATRI-013 a recorded contract to implement — the `templates/README.md` precedence rule.
2. No backwards-compatibility aliasing/shims; docs only.
3. `engine-core` untouched; no `game-stdlib` change.

## Verification Layers

1. Allowed/forbidden bot input recorded (own hand + public history only; no opponent hand/tail/seed/sampling) -> manual review against spec §B2 + FOUNDATIONS §8.
2. Policy is explainable, deterministic, beatable, no-solver -> manual review (bot-strategy audit); enforced in code by GAT101PLATRI-013.
3. Doc-link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `games/plain_tricks/docs/COMPETENT-PLAYER.md`

Describe what competent Plain Tricks play looks like for a fair authored opponent: when to win vs duck a trick, leading established winners, leading low from the longest suit, using public void information.

### 2. `games/plain_tricks/docs/BOT-STRATEGY-EVIDENCE-PACK.md`

Record the `plain-tricks-level2-v1` design: allowed input (own seat/hand, legal tree, current trick, public play history incl. revealed voids, trick counts/totals, round/trick index, terminal flag); forbidden input (opponent hand, tail, seed/reconstructed shuffle, full internal trace, hidden replay payloads, opponent private explanations, sampled/enumerated holdings); the heuristic priority sketch and deterministic tie-break; and the viewer-safe explanation policy (own-hand + public-history reasoning only).

## Files to Touch

- `games/plain_tricks/docs/COMPETENT-PLAYER.md` (new)
- `games/plain_tricks/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)

## Out of Scope

- The bot implementation and `AI.md` (GAT101PLATRI-013).
- Any code; this is design-input documentation.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with both docs present.
2. The evidence pack enumerates allowed and forbidden bot inputs consistent with FOUNDATIONS §8 and spec §B2.
3. Manual review confirms the policy is authored, deterministic, explainable, and beatable (no solver/sampling).

### Invariants

1. The documented policy uses only the seat's own hand and public information (FOUNDATIONS §8/§11; no hidden-state cheating).
2. No MCTS/ISMCTS/Monte Carlo/ML/RL is proposed (FOUNDATIONS §8).

## Test Plan

### New/Modified Tests

1. `None — documentation ticket; verification is command-based and named in Assumption Reassessment. Bot legality is enforced by tests in GAT101PLATRI-013.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `node scripts/check-catalog-docs.mjs`
3. A doc-link check is the correct boundary; the strategy's executable guarantees (legality, determinism) are verified in GAT101PLATRI-013's bot tests.
