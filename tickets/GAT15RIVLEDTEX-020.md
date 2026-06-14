# GAT15RIVLEDTEX-020: Mechanic-atlas final pressure review

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None (docs — `games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md`, `docs/MECHANIC-ATLAS.md`)
**Deps**: GAT15RIVLEDTEX-008

## Problem

After implementation, River Ledger must reassess repeated-shape pressure before public release: compare its standard-deck / hidden-hand / N-seat-projection / betting-ledger / seven-card-evaluator / split-allocation shapes against prior games, update the primitive-pressure ledger with implementation evidence, and record the atlas decision — `game-local / no promotion`, leaving §10A open debt `_None_`.

## Assumption Reassessment (2026-06-14)

1. `docs/MECHANIC-ATLAS.md` §10A open promotion-debt register reads `Current debt: _None_.` (verified); the initial River Ledger pressure ledger was authored in 002 and is finalized here with module/test/trace evidence from 003–013.
2. `specs/...-base.md` §8 (mechanic-atlas stance), §10.3, and §5 G15-RL-013 fix the comparison set (`high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims`, existing accounting entries) and the `game-local / no promotion` decision.
3. Cross-artifact boundary under audit: the final ledger cites the implemented `cards`/`betting`/`pot`/`evaluator`/`visibility` modules and their tests/traces; the `docs/MECHANIC-ATLAS.md` River Ledger pressure row is additive and must not change §10A debt.
4. FOUNDATIONS §4 (third-use mechanic hard gate) motivates this ticket: hidden card/private-hand and betting/showdown shapes recur, but Gate 15 records pressure and keeps helpers local; no `game-stdlib` promotion and no `engine-core` noun. If implementation had hit a hard third-use gate unhandleable by local recording, work would stop for an ADR/promotion gate — confirm it did not.

## Architecture Check

1. A post-implementation pressure review with cited evidence makes the `game-local / no promotion` decision auditable and keeps `game-stdlib` earned rather than speculative.
2. No backwards-compatibility aliasing/shims — docs only.
3. `engine-core` stays noun-free (§3); no `game-stdlib` promotion (§4); §10A debt remains `_None_`.

## Verification Layers

1. Ledger cites real modules/tests/traces for each pressured shape -> grep-proof of cited paths in the codebase.
2. Atlas decision is `game-local / no promotion`; §10A debt unchanged -> grep `Current debt: _None_.` in `docs/MECHANIC-ATLAS.md`.
3. Doc-link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md`

Finalize the pressure ledger with implementation evidence (modules/tests/traces) for standard deck, hidden hole cards, N-seat projections, fixed-limit contribution ledger, seven-card evaluator, and split allocation.

### 2. `docs/MECHANIC-ATLAS.md`

Add the River Ledger / Gate 15 pressure row (comparison set + `game-local / no promotion`); leave §10A open debt `_None_`.

## Files to Touch

- `games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md` (modify; created by 002)
- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- Any `game-stdlib` extraction or promotion (none authorized by Gate 15).
- Any `engine-core` change.
- The acceptance sweep + spec/index `Done`-flip (GAT15RIVLEDTEX-021).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — passes with the updated ledger/atlas.
2. `grep -F 'Current debt: _None_.' docs/MECHANIC-ATLAS.md` — §10A debt unchanged.
3. The ledger's cited module/test/trace paths resolve in the codebase.

### Invariants

1. Gate 15 promotes no helper into `game-stdlib`; §10A debt stays `_None_` (§4).
2. The decision is `game-local / no promotion` with cited evidence (§4).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -F 'Current debt: _None_.' docs/MECHANIC-ATLAS.md`
3. A doc-link + grep is the correct boundary; this ticket records a decision, it changes no behavior.
