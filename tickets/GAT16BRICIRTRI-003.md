# GAT16BRICIRTRI-003: Primitive-pressure ledger (second use) and mechanic-atlas update

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None (docs — `games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md`, `docs/MECHANIC-ATLAS.md`, `games/plain_tricks/docs/MECHANICS.md`, `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md`)
**Deps**: 001

## Problem

`plain_tricks` is the first official use of follow-suit legality, led-suit trick comparison, trick-winner-leads sequencing, and deterministic trick-round redeal (`docs/MECHANIC-ATLAS.md` §10 records them `local-only` first use). Briar Circuit is the **second close use**. FOUNDATIONS §4 requires the second use to implement locally, compare both implementations, update both game inventories and the atlas, and record a keep-local/defer decision **before gameplay implementation**. This ticket authors that comparison so GAT16BRICIRTRI-004+ proceed against a recorded decision, and so Gate 17 (Oh Hell, the third-use hard gate) can compare real evidence.

## Assumption Reassessment (2026-06-20)

1. `docs/MECHANIC-ATLAS.md` currently records the four trick-taking rows (follow-suit legality, trick resolution / led-suit comparator, trick-winner-leads turn order, deal rotation / trick-round redeal) as `plain_tricks` `local-only` first official use; §9A names "Hearts, Oh Hell, and Spades" as the next pressure point; §10A debt register reads `Current debt: _None_` (Gate 15.1 closeout 2026-06-20). This ticket moves the four rows to second-use/repeated-shape candidate and leaves §10A `_None_`.
2. `specs/gate-16-briar-circuit-trick-taking.md` §8.2 (comparison table), §8.3, and §10.3 fix the decision (`defer-reject / keep local`) and the exact atlas/ledger edits; `games/plain_tricks/docs/MECHANICS.md` and `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md` gain the second-use comparison with **no Plain Tricks behavior change**.
3. Cross-artifact boundary under audit: the shared mechanic shapes (follow-suit, led-suit comparator, winner-leads, deal rotation, private-hand projection) compared across `plain_tricks` and `briar_circuit`; the decision is recorded in three docs that must stay mutually consistent (game-local ledger, Plain Tricks ledger/inventory, central atlas).
4. FOUNDATIONS §4 (`game-stdlib` is earned) and the §12 stop condition "a third repeated mechanic proceeds without a ledger decision" are the principles under audit. This is the **second** use, not the third: the rule is implement-locally + compare + record, not a hard gate blocking the game. No helper is promoted; no kernel noun is added; open promotion debt remains `_None_`. Gate 17 is named as the third-use hard-gate trigger.

## Architecture Check

1. Recording the keep-local/defer decision before code is cleaner than discovering duplicate trick logic at closeout: it documents *why* duplication is acceptable (small shared core, divergent behavior-bearing exceptions) so Gate 17 reasons from evidence, not memory.
2. No backwards-compatibility aliasing/shims — docs-only; Plain Tricks behavior, traces, and hashes are untouched.
3. `engine-core` stays free of mechanic nouns (§3); `game-stdlib` gains nothing (§4) — the decision is explicitly *defer/keep-local*. No trick/card helper is created.

## Verification Layers

1. Atlas rows moved to second-use and §10A debt still `_None_` -> grep-proof of the four rows + the debt register line in `docs/MECHANIC-ATLAS.md`.
2. Plain Tricks docs gain the second-use note with no code/behavior delta -> `git diff --stat games/plain_tricks/` shows only `docs/` changes.
3. Decision recorded as `keep local / defer` in all three ledgers -> grep cross-check + FOUNDATIONS §4 alignment review.

## What to Change

### 1. `games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md`

The Plain Tricks ↔ Briar Circuit comparison per spec §8.2 (repeated core vs material divergence per mechanic shape) and the decision `defer-reject / keep local`, naming Gate 17 Oh Hell as the next review trigger.

### 2. `docs/MECHANIC-ATLAS.md`

Move the follow-suit, led-suit comparator, winner-leads, and deal/redeal rows from first use to second-use/repeated-shape candidate (spec §8.2 suggested rows); update §9A to show the Hearts comparison completed with Oh Hell remaining the third-use trigger; leave §10A `Current debt: _None_`.

### 3. `games/plain_tricks/docs/MECHANICS.md` and `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md`

Add the second-use comparison reference (Briar Circuit) with no behavior change.

## Files to Touch

- `games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new)
- `docs/MECHANIC-ATLAS.md` (modify)
- `games/plain_tricks/docs/MECHANICS.md` (modify)
- `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md` (modify)

## Out of Scope

- Any `game-stdlib` extraction or helper module (the decision is defer/keep-local; an extraction ticket would only exist if the decision were *promote*).
- Any change to Plain Tricks behavior, traces, hashes, action paths, or renderer.
- The crate skeleton and typed card model (GAT16BRICIRTRI-004).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -nE 'second(-| )use|repeated-shape' docs/MECHANIC-ATLAS.md` — the four trick rows show the updated status.
2. `grep -n 'Current debt: _None_' docs/MECHANIC-ATLAS.md` — debt register unchanged.
3. `node scripts/check-doc-links.mjs` and `bash scripts/boundary-check.sh` — pass (no kernel nouns, links intact).

### Invariants

1. No `game-stdlib` promotion and no `engine-core` noun added (§3/§4); open debt remains `_None_`.
2. Plain Tricks behavior is unchanged; only its docs gain the comparison (§ forbidden-changes spec §9).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE 'follow-suit|led-suit comparator|winner-leads|deal rotation' docs/MECHANIC-ATLAS.md`
2. `node scripts/check-doc-links.mjs && bash scripts/boundary-check.sh`
3. `git diff --stat games/plain_tricks/` — confirms docs-only change to the sibling game.
