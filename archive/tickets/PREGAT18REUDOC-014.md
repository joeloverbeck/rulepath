# PREGAT18REUDOC-014: AI-BOTS one-owner + UI-INTERACTION semantic scaffolding review

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — docs-only (`docs/AI-BOTS.md`, `docs/UI-INTERACTION.md`)
**Deps**: 004

## Problem

The AI doc set needs a single owner/purpose per doc, and `UI-INTERACTION.md` §10A still gates presentation-helper promotion on a raw game-count trigger ("official-game count above 20") rather than a semantic scaffolding review — out of step with the new scaffolding lane. Both edits align the AI/UI docs with the post-ADR-0008 doctrine.

## Assumption Reassessment (2026-06-22)

1. Verified `docs/UI-INTERACTION.md` §10A (line 227) reads "...a third structural divergence ... or an **official-game count above 20**" (exact wording confirmed via `/reassess-spec` finding M2 this session); `docs/AI-BOTS.md` exists as the bot doctrine doc. The UI scaffolding-review replacement depends on the scaffolding lane doctrine that ADR 0008 (ticket 004) establishes; hence `Deps: 004` + acceptance precondition for the UI portion.
2. Verified against spec D11: one owner/purpose per AI doc; replace the UI count trigger with a semantic scaffolding review.
3. Cross-artifact boundary under audit: the UI semantic scaffolding review references the scaffolding lane / register (ADR 0008, tickets 006/008/009).
4. FOUNDATIONS §7 (public UI is central) + §8 (public bots) motivate this: restating — AI one-owner keeps bot doctrine in one place; the UI scaffolding review keeps presentation-helper promotion **semantic** (tied to structural divergence + the scaffolding lane), not an arbitrary game-count threshold.
5. Touches the §4 scaffolding doctrine (UI portion): confirm the replacement routes through ADR 0008's semantic review (acceptance precondition) and opens no leak/nondeterminism path.

## Architecture Check

1. A semantic scaffolding-review trigger is more robust than a raw count threshold (which the corpus, at 17 games, is about to cross arbitrarily); one-owner AI docs remove cross-doc bot-doctrine duplication.
2. No backwards-compatibility shims.
3. `engine-core` (§3) / `game-stdlib` (§4) untouched; docs-only.

## Verification Layers

1. `AI-BOTS.md` has one clear owner/purpose (no duplicated bot doctrine) -> manual review + grep.
2. The UI "official-game count above 20" trigger is replaced by a semantic scaffolding review -> grep (old phrase gone, new present).
3. ADR 0008 `Accepted` precondition (UI portion) -> grep (`^Status: Accepted` on `docs/adr/0008-*.md`).
4. Links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. AI-BOTS one owner/purpose

Edit `docs/AI-BOTS.md` so each AI doc has a single owner/purpose, removing duplicated bot doctrine.

### 2. UI semantic scaffolding review

Replace the §10A "official-game count above 20" promotion trigger with a semantic scaffolding review tied to the scaffolding lane (structural divergence + register), per ADR 0008.

## Files to Touch

- `docs/AI-BOTS.md` (modify)
- `docs/UI-INTERACTION.md` (modify)

## Out of Scope

- The official-game-contract pointer (ticket 013) and the IP/SOURCES/AGENT-DISCIPLINE/archival docs (tickets 015/016).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "official-game count above 20" docs/UI-INTERACTION.md` returns nothing (trigger removed); `grep -niE "scaffolding review" docs/UI-INTERACTION.md` returns the replacement.
2. `grep -niE "owner|purpose" docs/AI-BOTS.md` reflects the one-owner framing (manual confirm).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. Presentation-helper promotion is semantic, not count-based.
2. Bot doctrine stays §8-compliant (no MCTS/ML/RL; legal-action API only) — unchanged by this edit.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (trigger-removal grep, link check) + manual one-owner review named in Assumption Reassessment.`

### Commands

1. `grep -niE "count above 20|scaffolding review" docs/UI-INTERACTION.md`
2. `node scripts/check-doc-links.mjs`
3. The old-phrase-gone / new-phrase-present grep pair is the correct boundary for the trigger swap.

## Outcome

Completed: 2026-06-22

Updated `docs/AI-BOTS.md` with a one-owner/purpose table for the AI-related doc
set. The repository bot law remains in `AI-BOTS.md`; per-game strategy,
Level 2 policy evidence, per-game bot registry/status, and cross-template
evidence links each point to their owning documents without duplicating full bot
doctrine.

Updated `docs/UI-INTERACTION.md` §10A to remove the raw
`official-game count above 20` presentation-helper promotion trigger. Repeated
presentation shapes now require semantic scaffolding review through ADR 0008 and
`MECHANICAL-SCAFFOLDING-REGISTER.md`, with behavior, legality, visibility,
renderer-policy, hidden-state, and game-rule candidates rejected from the
scaffolding lane.

Verification:

- `grep -niE "official-game count above 20" docs/UI-INTERACTION.md` returned no
  matches.
- `grep -niE "scaffolding review" docs/UI-INTERACTION.md` returned the
  replacement trigger.
- `grep -niE "owner|purpose" docs/AI-BOTS.md` returned the one-owner framing.
- `grep -n "^Status: Accepted" docs/adr/0008-mechanical-scaffolding-governance.md`
  returned `Status: Accepted`.
- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).

Deviation: none; bot doctrine and UI behavior remain documentation-only.
