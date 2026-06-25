# GAT18BLAPACSPA-010: bot-strategy documents (AI, competent-player, deferred L2 evidence pack)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — game-local docs (`games/blackglass_pact/docs/{AI,COMPETENT-PLAYER,BOT-STRATEGY-EVIDENCE-PACK}.md`)
**Deps**: GAT18BLAPACSPA-009

## Problem

Author the bot-strategy doc set that documents the L0/L1 policies shipped in GAT18BLAPACSPA-009 and gates a future L2: `AI.md` (policy IDs, authorized observations, deterministic priorities/ties, memory limits, explanation safety, forbidden-method prohibition), `COMPETENT-PLAYER.md` (sourced, rules-checked strategy analysis), and `BOT-STRATEGY-EVIDENCE-PACK.md` (status `not admitted / intentionally deferred` with the complete evidence required before any L2 code) — spec §4.2, Appendix D.8–D.9, candidate task `GAT18-BLAPAC-009`.

## Assumption Reassessment (2026-06-25)

1. The L1 policy IDs/priorities these docs describe are produced by `games/blackglass_pact/src/bots.rs` + `tests/bots.rs` (GAT18BLAPACSPA-009); per the official-game pattern, a Level-1 evidence pack trails and depends on the bot ticket because the evidence is produced by the bot tests.
2. Appendix D pins the AI level posture, authorized inputs, and the L2 evidence-gate rows; the docs must not adopt the arXiv:1912.11323 learned/expected-utility method (out of scope for v1/v2).
3. Cross-artifact boundary under audit: `AI.md`/`COMPETENT-PLAYER.md`/`BOT-STRATEGY-EVIDENCE-PACK.md` must agree with `bots.rs` on L0/L1 behavior and on L2 being unadmitted.
4. FOUNDATIONS §8 (public bots, not research AI) motivates this ticket: the docs restate the prohibition of MCTS/ISMCTS/Monte Carlo/ML/RL/runtime-LLM and the requirement that bots use only authorized public/own-hand information.

## Architecture Check

1. Documenting the policies after the bot lands (vs. speculative pre-authoring) keeps the docs faithful to the shipped deterministic priorities; the deferred L2 pack prevents premature search-bot work.
2. No shims; no L2 code is authored.
3. `engine-core` untouched; no `game-stdlib` change; docs only.

## Verification Layers

1. `AI.md` policy IDs/priorities match `bots.rs` -> manual review cross-checking the L1 priority families against the code.
2. L2 status is `not admitted / intentionally deferred` with the full evidence-gate list -> grep-proof in `BOT-STRATEGY-EVIDENCE-PACK.md`.
3. No forbidden-method or hidden-state claim -> manual review + `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `AI.md`

L0/L1 policy IDs, authorized observations (Appendix D.2), deterministic priorities/tie-breaks, bounded memory, explanation safety, explicit prohibition of hidden-world/search/learning methods.

### 2. `COMPETENT-PLAYER.md`

Sourced, rules-checked analysis (Appendix D.8): trick counting, spade control, nil risk, partner coverage through public play, contract management, bag pressure, setting, score posture, common novice errors.

### 3. `BOT-STRATEGY-EVIDENCE-PACK.md`

Status `not admitted / intentionally deferred`; the complete Appendix D.9 evidence rows required before any authored L2 coding (no empty placeholder).

## Files to Touch

- `games/blackglass_pact/docs/AI.md` (new)
- `games/blackglass_pact/docs/COMPETENT-PLAYER.md` (new)
- `games/blackglass_pact/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)

## Out of Scope

- Any L2 bot code (deferred behind the evidence gate).
- `bots.rs` behavior changes (GAT18BLAPACSPA-009).

## Acceptance Criteria

### Tests That Must Pass

1. `AI.md` enumerates L0 + L1 policy IDs and the forbidden-method prohibition.
2. `BOT-STRATEGY-EVIDENCE-PACK.md` records L2 `not admitted` with the full evidence-gate list.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The docs match the shipped L0/L1 behavior and never describe hidden-state access.
2. L2/L3 and search/sampling/learning methods are documented as unadmitted/forbidden.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; the L0/L1 behavior the docs describe is tested in GAT18BLAPACSPA-009.`

### Commands

1. `grep -iE "not admitted|deferred" games/blackglass_pact/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
2. `node scripts/check-doc-links.mjs`
3. Doc-only ticket — review + doc-link check is the correct boundary; bot behavior is verified in its own ticket.
