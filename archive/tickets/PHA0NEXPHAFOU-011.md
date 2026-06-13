# PHA0NEXPHAFOU-011: Templates — game-contract cluster N-seat/surface fields

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — `templates/GAME-RULES.md`, `GAME-MECHANICS.md`, `GAME-RULE-COVERAGE.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `GAME-HOW-TO-PLAY.md`, `GAME-SOURCES.md` edits only.
**Deps**: PHA0NEXPHAFOU-002, PHA0NEXPHAFOU-005

## Problem

The game-contract templates assume "you vs opponent" / two seats. They lack seat-model, turn-order, per-seat outcome, pairwise no-leak, and common-game IP fields, so the first Gate 15+ spec would re-derive (or omit) them. The templates must carry these fields before any 3+ seat game spec is authored.

## Assumption Reassessment (2026-06-13)

1. No code change. `templates/` holds the six contract templates (verified via `ls templates/`): `GAME-RULES.md`, `GAME-MECHANICS.md`, `GAME-RULE-COVERAGE.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `GAME-HOW-TO-PLAY.md`, `GAME-SOURCES.md`.
2. Docs: the doctrine these templates must mirror lives in `docs/OFFICIAL-GAME-CONTRACT.md` (the N-seat acceptance + showdown rows added in PHA0NEXPHAFOU-005) and `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (PHA0NEXPHAFOU-002).
3. Cross-artifact boundary under audit: the per-game contract template set; shared surface = the seat-model / outcome / no-leak fields that mirror the OFFICIAL-GAME-CONTRACT rows.
4. FOUNDATIONS principle restate: §11 (per-seat outcome, pairwise no-leak) and §6 (official games are evidence-heavy). The template additions are meaning-preserving — they add fields games already owe under the contract.
5. Enforcement surface: §11 no-leak firewall. `GAME-RULE-COVERAGE.md`'s pairwise no-leak coverage table is the per-game input the Infra D harness and the OFFICIAL-GAME-CONTRACT consume; adding the field introduces no leakage path.

## Architecture Check

1. Putting seat/surface/no-leak fields in the contract templates makes every >2-seat game spec fill them, which is cleaner than per-game rediscovery.
2. No backwards-compatibility aliasing/shims introduced.
3. Templates keep typed mechanic nouns game-local (§3); no template prescribes TypeScript legality or a kernel noun.

## Verification Layers

1. Each template's new N-seat fields present → manual review.
2. `GAME-RULE-COVERAGE.md` pairwise (source-seat-private-datum × viewer × surface) table present → manual review + FOUNDATIONS alignment check (§11).
3. Links resolve → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `templates/GAME-RULES.md`

Add a mandatory "seat model" subsection: min/max seats, official seat IDs, seat labels, role/team assignment, order of play, setup-rejection rules, viewer classes. Add a showdown/evaluator subsection when applicable.

### 2. `templates/GAME-MECHANICS.md`

Add mechanic categories: N-seat model, turn-order policy, team/partnership/coalition, graph/track/topology size, hidden-hand/deck/wall model, evaluator/showdown/ranking, shared accounting/side-pot/split allocation, reaction/simultaneous windows.

### 3. `templates/GAME-RULE-COVERAGE.md`

Add required coverage sections per terminal result and per viewer class; for hidden-info N-seat games, a table of `source seat private datum` × `viewer` × `surface`.

### 4. `templates/GAME-IMPLEMENTATION-ADMISSION.md`

Add admission rows: min/max seats; wrong-seat-count diagnostics; stable seat labels; per-viewer projection proof; pairwise no-leak proof; topology/object-count inventory; atlas interlock status.

### 5. `templates/GAME-HOW-TO-PLAY.md`

Add fields: supported player count; seating/role assignment; turn-order visualization; simultaneous/pending response explanation; team/partnership explanation; how showdown/final standings are explained.

### 6. `templates/GAME-SOURCES.md`

Add rows: public-domain/common-system fact; source fact used; original prose/asset plan; name/trade-dress risk; casino/brand term avoided; variant/source conflict resolved.

## Files to Touch

- `templates/GAME-RULES.md` (modify)
- `templates/GAME-MECHANICS.md` (modify)
- `templates/GAME-RULE-COVERAGE.md` (modify)
- `templates/GAME-IMPLEMENTATION-ADMISSION.md` (modify)
- `templates/GAME-HOW-TO-PLAY.md` (modify)
- `templates/GAME-SOURCES.md` (modify)

## Out of Scope

- The bot templates `GAME-AI.md` / `BOT-STRATEGY-EVIDENCE-PACK.md` / `COMPETENT-PLAYER.md` (PHA0NEXPHAFOU-012).
- The UI/benchmark/release/pressure templates + `AGENT-TASK.md` + `templates/README.md` (PHA0NEXPHAFOU-013).
- Any `games/*` code or per-game doc instances.

## Acceptance Criteria

### Tests That Must Pass

1. Each of the six contract templates carries its N-seat/surface additions per What to Change.
2. `templates/GAME-RULE-COVERAGE.md` carries the pairwise source-seat-private-datum × viewer × surface table.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The §11 per-seat outcome and pairwise no-leak fields are present in the contract templates.
2. No template prescribes TypeScript legality (§2) or introduces a kernel noun (§3).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -niE "seat model|pairwise|min/max seats|public-domain/common" templates/GAME-RULES.md templates/GAME-RULE-COVERAGE.md templates/GAME-IMPLEMENTATION-ADMISSION.md templates/GAME-SOURCES.md`
3. `bash scripts/boundary-check.sh`

## Outcome

Completed: 2026-06-13

Summary:

- Added mandatory seat model, showdown/evaluator, terminal/viewer coverage, pairwise N-seat hidden-information, admission, player-facing, and source/IP prompts to the six game-contract templates.
- Kept the changes documentation-only and confined to the template files named by this ticket.

Deviations:

- None.

Verification:

- `node scripts/check-doc-links.mjs`
- `grep -niE "seat model|pairwise|min/max seats|public-domain/common" templates/GAME-RULES.md templates/GAME-RULE-COVERAGE.md templates/GAME-IMPLEMENTATION-ADMISSION.md templates/GAME-SOURCES.md`
- `bash scripts/boundary-check.sh`
