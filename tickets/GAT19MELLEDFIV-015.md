# GAT19MELLEDFIV-015: Bot-strategy, competent-player, and AI documentation

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — game-local docs (`AI.md`, `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`)
**Deps**: GAT19MELLEDFIV-014

## Problem

Gate 19 must ship the bot/strategy documentation: `AI.md` (L0 required, L1 if implemented, L2 deferred, hidden-info bot fields, no MCTS/ISMCTS/ML/RL), `COMPETENT-PLAYER.md` (rummy competence — hand-shaping, meld timing, discard risk, discard-pile pickup risk/reward, high-card penalty management), and `BOT-STRATEGY-EVIDENCE-PACK.md` (required before any L2 admission; for the initial ship it may mark L2 `not admitted`). These trail the bot ticket because they document the bot's behavior and fixtures.

## Assumption Reassessment (2026-06-25)

1. `games/blackglass_pact/docs/{AI.md,COMPETENT-PLAYER.md,BOT-STRATEGY-EVIDENCE-PACK.md}` and `games/river_ledger/docs/AI.md` are the house-style exemplars (confirmed during reassessment); the L0/L1 bots they describe exist from GAT19MELLEDFIV-014.
2. Spec §4.2 (AI.md / COMPETENT-PLAYER.md / BOT-STRATEGY-EVIDENCE-PACK.md rows) and Appendix A.3 (bot-policy research notes) define the content; L2 is deferred at initial ship.
3. Cross-artifact: these docs describe the bot behavior implemented in `bots.rs` (GAT19MELLEDFIV-014) — they must match the admitted tiers, not over-claim an L2 policy.
4. FOUNDATIONS §8: the docs must record the hidden-information boundary (opponents' hands / stock order are forbidden bot inputs) and the MCTS/ISMCTS/Monte-Carlo/ML/RL exclusion; L2 stays blocked until the evidence pack is accepted.

## Architecture Check

1. Authoring strategy docs after the bot exists keeps them truthful to the shipped policy rather than aspirational, and keeps the L2-deferral explicit so no later ticket silently admits L2.
2. No backwards-compatibility shims — new docs.
3. `engine-core`/`game-stdlib` untouched (docs only).

## Verification Layers

1. Docs name the admitted bot tiers and the L2-deferred state -> manual review against `bots.rs` (GAT19MELLEDFIV-014).
2. Hidden-info boundary + search-class exclusion recorded -> grep for the no-MCTS/ISMCTS/ML/RL statement and the forbidden-inputs note.
3. Doc links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `AI.md`

L0 required; L1 allowed if implemented (public + own-private features); L2 deferred; hidden-info bot fields; explicit no MCTS/ISMCTS/Monte Carlo/ML/RL.

### 2. `COMPETENT-PLAYER.md`

Rummy competence: hand-shaping, meld timing, discard risk, discard-pile pickup risk/reward, high-card penalty management, opponent-proximity inference from public facts only.

### 3. `BOT-STRATEGY-EVIDENCE-PACK.md`

Evidence-pack scaffold required before L2 admission; mark L2 `not admitted` for the initial ship, with the acceptance bar named.

## Files to Touch

- `games/meldfall_ledger/docs/AI.md` (new)
- `games/meldfall_ledger/docs/COMPETENT-PLAYER.md` (new)
- `games/meldfall_ledger/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)

## Out of Scope

- Any L2 bot implementation (deferred behind the evidence pack).
- `MECHANICS.md`/`UI.md` (GAT19MELLEDFIV-020/023), `RULES.md`/`SOURCES.md` (GAT19MELLEDFIV-001).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes.
2. `AI.md` records the MCTS/ISMCTS/Monte-Carlo/ML/RL exclusion and the L2-deferred state.
3. The docs match the bot tiers admitted in `bots.rs` (no L2 over-claim).

### Invariants

1. The hidden-information bot boundary is documented (opponents' hands / stock order are forbidden inputs) (FOUNDATIONS §8).
2. L2 stays blocked until `BOT-STRATEGY-EVIDENCE-PACK.md` is accepted.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is `check-doc-links` plus manual review against the shipped bot tiers.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -niE "mcts|ismcts|monte carlo|machine learning|reinforcement" games/meldfall_ledger/docs/AI.md` (confirm the exclusion is stated)
3. A narrower doc-link + grep check is the correct boundary; no code changes here.
