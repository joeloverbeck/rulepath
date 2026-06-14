# GAT15RIVLEDTEX-002: Pre-coding admission spine — admission, mechanics, coverage plan, pressure ledger

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None (docs — `games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md`, `MECHANICS.md`, `RULE-COVERAGE.md`, `PRIMITIVE-PRESSURE-LEDGER.md`)
**Deps**: GAT15RIVLEDTEX-001

## Problem

Spec §11.2 forbids implementation until a reviewed admission spine exists: mechanic inventory, an initial primitive-pressure ledger, a planned rule-coverage matrix, and an implementation-admission receipt with explicit blockers. This ticket completes that pre-coding spine so the crate scaffold (003) and all downstream work can begin.

## Assumption Reassessment (2026-06-14)

1. The 13-doc set matches `games/poker_lite/docs/` exactly (verified present); this ticket authors the four admission-spine docs not produced by 001, leaving trailing docs (UI/ADMISSION-final/PUBLIC-RELEASE) to 019 and AI to 013.
2. `specs/gate-15-river-ledger-texas-holdem-base.md` §4.2, §8 mechanic-atlas stance, and §11.2 admission rule fix this content; `docs/MECHANIC-ATLAS.md` §10A open promotion-debt register currently reads `Current debt: _None_.`
3. Cross-artifact boundary under audit: `MECHANICS.md` and `RULE-COVERAGE.md` planned rows reference the `RL-*` IDs from 001; the final `RULE-COVERAGE.md` reconciliation co-lands with the rule-coverage tool (015) and `BENCHMARKS.md` (014), so coverage may read partial-until-code per spec §5 G15-RL-001 evidence.
4. FOUNDATIONS §6 (official games are evidence-heavy) motivates this ticket: the admission receipt enumerates the rules/source/mechanic/coverage/UI/bot/no-leak/bench prerequisites as explicit blockers before coding.
5. Third-use mechanic hard gate (§4) under audit: hidden card/private-hand and betting/showdown shapes recur across `high_card_duel`, `plain_tricks`, `secret_draft`, `poker_lite`; the initial ledger records pressure against that comparison set and decides `game-local / no promotion`, leaving §10A debt `_None_`. The final review is GAT15RIVLEDTEX-020; this ticket must not promote any helper into `game-stdlib`.

## Architecture Check

1. A single front-loaded admission-spine ticket makes §11.2's "no coding before review" gate a concrete reviewable artifact rather than scattered prose, matching the OGC §3 ordering.
2. No backwards-compatibility aliasing/shims — new docs only.
3. `engine-core` stays noun-free (§3); the ledger records pressure but authorizes no `game-stdlib` promotion (§4).

## Verification Layers

1. Admission receipt enumerates every prerequisite/blocker -> manual review against OGC + spec §11.2.
2. Coverage matrix names every `RL-*` rule with an honest status -> grep-proof that planned rows mirror `RULES.md` IDs.
3. Atlas pressure recorded against the named comparison set with `game-local / no promotion` -> FOUNDATIONS §4 alignment check + `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `games/river_ledger/docs/MECHANICS.md`

Game-local mechanic inventory across atlas categories: N-seat hidden information, betting/contribution accounting, deck/deal, seven-card evaluator, public/private projections, split-pot allocation.

### 2. `games/river_ledger/docs/RULE-COVERAGE.md`

Planned matrix from every `RL-*` rule to its implementation module, tests, golden traces, replay/serialization, UI, bots, and benchmarks, with honest "pending code" status where applicable.

### 3. `games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md`

Initial pressure ledger comparing against `high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims`, and existing accounting entries; decision `game-local / no promotion`.

### 4. `games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md`

Pre-coding admission receipt listing prerequisites and explicit blockers per spec §11.2.

## Files to Touch

- `games/river_ledger/docs/MECHANICS.md` (new)
- `games/river_ledger/docs/RULE-COVERAGE.md` (new)
- `games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new)
- `games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)

## Out of Scope

- Any Rust code (GAT15RIVLEDTEX-003+).
- Final `RULE-COVERAGE.md` reconciliation with the tool (GAT15RIVLEDTEX-015) and `BENCHMARKS.md` (014).
- Final atlas review and `docs/MECHANIC-ATLAS.md` row (GAT15RIVLEDTEX-020).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — passes with the four new docs linked.
2. Planned coverage rows cover every `RL-*` family present in `RULES.md`.
3. Manual review confirms the admission receipt names blockers and the ledger decides `game-local / no promotion`.

### Invariants

1. No `game-stdlib` promotion is authorized by Gate 15 (§4); §10A debt stays `_None_`.
2. Coverage status is honest (no claimed-green rows before code exists) (§6).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE 'RL-[A-Z]+-' games/river_ledger/docs/RULE-COVERAGE.md`
2. `node scripts/check-doc-links.mjs`
3. A narrower command suffices: `tools/rule-coverage` cannot pass until code + the prefix validator exist, so this ticket's coverage is the planned matrix only.
