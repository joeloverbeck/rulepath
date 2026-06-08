# GAT72GAT8HIG-020: Trailing game docs + mechanic-atlas pressure row

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — docs only (`games/high_card_duel/docs/{MECHANICS,AI,UI,GAME-IMPLEMENTATION-ADMISSION,PUBLIC-RELEASE-CHECKLIST,COMPETENT-PLAYER,BOT-STRATEGY-EVIDENCE-PACK}.md`, `docs/MECHANIC-ATLAS.md`)
**Deps**: GAT72GAT8HIG-010, GAT72GAT8HIG-013, GAT72GAT8HIG-018

## Problem

The official-game contract requires the full per-game doc set, and Gate 8 must
record card/deck/hand/commitment primitive pressure in the mechanic atlas
(without promotion). These docs must land coherently once the implementation
surfaces they describe exist.

## Assumption Reassessment (2026-06-07)

1. Verified the doc convention: sibling `games/draughts_lite/docs/` carries
   `MECHANICS.md AI.md UI.md GAME-IMPLEMENTATION-ADMISSION.md
   PUBLIC-RELEASE-CHECKLIST.md COMPETENT-PLAYER.md BOT-STRATEGY-EVIDENCE-PACK.md`
   (templates under `templates/GAME-*.md`). `RULES.md`/`SOURCES.md` (002) and
   `RULE-COVERAGE.md`/`BENCHMARKS.md` (013/014) are authored with their
   validators and are out of scope here.
2. Verified against the spec: §4.2.12 + §10.3 fix the doc set; COMPETENT-PLAYER
   and BOT-STRATEGY-EVIDENCE-PACK are `not applicable / deferred` (Level 2 not
   shipped, §4.2.1 note). §4.2.12 also directs extending the mechanic-atlas
   `high_card_duel` pressure row.
3. Cross-artifact boundary under audit: the mechanic-atlas primitive-pressure
   register (`docs/MECHANIC-ATLAS.md`) — the existing `high_card_duel`
   "deterministic shuffle and hidden draw → local-only" row (line ~201) is
   *extended*, not duplicated (per reassess finding M2).
4. FOUNDATIONS principle under audit (§4 game-stdlib earned): the atlas note
   records first official card/deck use as local-only and explicitly does not
   authorize promotion; names future pressure (`blackjack_lite`, poker-lite,
   trick-taking).

## Architecture Check

1. A trailing cross-cutting docs ticket (after the surfaces exist) avoids a
   staleness window where docs describe unbuilt behavior — cleaner than
   co-locating multi-surface docs early.
2. No backwards-compatibility shims — additive docs + one atlas-row extension.
3. `engine-core`/`game-stdlib` untouched; the atlas explicitly withholds
   promotion (cards remain local; §4).

## Verification Layers

1. Doc-set completeness -> codebase grep-proof: all seven trailing docs exist with no empty required sections; COMPETENT-PLAYER/BOT-STRATEGY read `not applicable`.
2. Atlas pressure (no promotion) -> FOUNDATIONS alignment check: the `high_card_duel` row is extended with local pressure notes and states no promotion is authorized (§4).
3. Doc links -> simulation/CLI run: `node scripts/check-doc-links.mjs` passes.

## What to Change

### 1. Trailing per-game docs

Author `MECHANICS.md` (card/deck/hand/commitment/reveal/chance/hidden-info
inventory + primitive-pressure notes), `AI.md` (Level 0 + hidden-info
boundaries; Level 1 deferred), `UI.md` (UX/a11y/viewer/reduced-motion/no-leak),
`GAME-IMPLEMENTATION-ADMISSION.md`, `PUBLIC-RELEASE-CHECKLIST.md`,
`COMPETENT-PLAYER.md` (`not applicable / deferred`),
`BOT-STRATEGY-EVIDENCE-PACK.md` (`not applicable / Level 2 not shipped`).

### 2. `docs/MECHANIC-ATLAS.md`

Extend the existing `high_card_duel` pressure row with local notes for
deterministic shuffle, private hand, hidden commitment, reveal, effect filtering,
and no-leak replay export; state first official local card/deck use without
promotion; name future pressure points.

## Files to Touch

- `games/high_card_duel/docs/MECHANICS.md` (new)
- `games/high_card_duel/docs/AI.md` (new)
- `games/high_card_duel/docs/UI.md` (new)
- `games/high_card_duel/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)
- `games/high_card_duel/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)
- `games/high_card_duel/docs/COMPETENT-PLAYER.md` (new)
- `games/high_card_duel/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)
- `docs/MECHANIC-ATLAS.md` (modify — extend the existing high_card_duel row)

## Out of Scope

- `RULES.md`/`SOURCES.md` (002), `RULE-COVERAGE.md` (013), `BENCHMARKS.md` (014).
- The `specs/README.md` Done-flip and blackjack resolution (GAT72GAT8HIG-021).
- Any `game-stdlib` promotion (explicitly withheld).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — passes.
2. `grep -i "not applicable" games/high_card_duel/docs/COMPETENT-PLAYER.md games/high_card_duel/docs/BOT-STRATEGY-EVIDENCE-PACK.md` — both marked not applicable.
3. `grep -i "high_card_duel" docs/MECHANIC-ATLAS.md` — the extended pressure row present (single row, not duplicated).

### Invariants

1. The full official-game doc set is present (explicit `not applicable` over silent omission).
2. The atlas records local pressure and authorizes no promotion (§4).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `bash scripts/boundary-check.sh`
3. Doc-link + boundary checks are the correct boundary — this ticket ships no code.
