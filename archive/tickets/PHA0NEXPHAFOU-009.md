# PHA0NEXPHAFOU-009: IP-POLICY + SOURCES + AGENT-DISCIPLINE + archival-workflow next-phase notes

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — `docs/IP-POLICY.md`, `docs/SOURCES.md`, `docs/AGENT-DISCIPLINE.md`, `docs/archival-workflow.md` edits only.
**Deps**: PHA0NEXPHAFOU-002

## Problem

Four policy/process docs need small additive notes before the public scaling phase executes: `IP-POLICY.md` lacks a Texas Hold'Em casino-trade-dress note; `SOURCES.md` lacks the next-phase scaling sources; `AGENT-DISCIPLINE.md` lacks canonical bounded N-seat task examples; and `archival-workflow.md` has no spec-index rollover section (the convention this very phase used must be recorded).

## Assumption Reassessment (2026-06-13)

1. No code change. The convention to be documented in `archival-workflow.md` is the one already exercised this session: `specs/README.md` was archived to `archive/specs/README-2026-06-13.md` and a new index written.
2. Docs: `docs/IP-POLICY.md` (public-domain rules, casino-adjacent terms, original presentation), `docs/SOURCES.md` (bibliography), `docs/AGENT-DISCIPLINE.md` (bad-task examples, bounded-task law), `docs/archival-workflow.md` (archival conventions). `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (PHA0NEXPHAFOU-002) is the cross-reference target.
3. Cross-artifact boundary under audit: four independent policy/process docs; shared surface = IP posture, source provenance, bounded-task examples, and the index-rollover convention. Each edit is additive and self-contained.
4. FOUNDATIONS principle restate: §10 IP conservatism (public games are public-domain/original; original prose/assets; neutral IDs where trademark/trade-dress risk exists). The IP note is a meaning-preserving clarification for the Hold'Em case.

## Architecture Check

1. Grouping four small additive notes into one cross-cutting docs ticket is cleaner than four micro-tickets — they share no code surface and each is a self-contained note.
2. No backwards-compatibility aliasing/shims introduced.
3. No code or boundary is touched; `engine-core` is unaffected.

## Verification Layers

1. IP casino-trade-dress note present → manual review (IP-conservatism audit).
2. SOURCES next-phase scaling section present with resolving links → manual review + `node scripts/check-doc-links.mjs`.
3. AGENT-DISCIPLINE bounded N-seat task examples present → manual review.
4. archival-workflow index-rollover section present → manual review.

## What to Change

### 1. `docs/IP-POLICY.md`

Add a note: public-domain/common card systems may be implemented from researched rule facts, but Rulepath must write original prose, neutral display names, and original card art/icons, and avoid casino product framing. Distinguish the existing `poker_lite` / Crest Ledger from proper Texas Hold'Em.

### 2. `docs/SOURCES.md`

Add a "Next-phase scaling sources" section: Pagat Texas Hold'em and poker hand-ranking, Hearts/Oh Hell/Spades, Chinese Checkers, Rummy 500, OpenSpiel, and boardgame.io, with the note that sources support facts only and Rulepath prose remains original. Mark any user-to-select source (Rummy/Pachisi/Mahjong) as TBD.

### 3. `docs/AGENT-DISCIPLINE.md`

Add good bounded-task examples: "generalize the simulator summary map from fixed two seats to a winner-id map," "add a 3-seat setup fixture for one game," "add a pairwise no-leak matrix harness," "add Rust-owned showdown rationale for Hold'Em," "add a multi-seat seat-rail UI fed by the Rust view."

### 4. `docs/archival-workflow.md`

Add a "roadmap phase rollover" section: archive the old living spec index date-suffixed under `archive/specs/` with a date/commit note, create a new `specs/README.md` seeded from the accepted roadmap/ADR, preserve old archive links, and record the authority commit/manifest used.

## Files to Touch

- `docs/IP-POLICY.md` (modify)
- `docs/SOURCES.md` (modify)
- `docs/AGENT-DISCIPLINE.md` (modify)
- `docs/archival-workflow.md` (modify)

## Out of Scope

- Editing `templates/AGENT-TASK.md` (PHA0NEXPHAFOU-013 owns the templates).
- Editing the ADRs (PHA0NEXPHAFOU-010).
- Re-archiving `specs/README.md` — the rollover already landed this session; this ticket only documents the convention.
- Any code change.

## Acceptance Criteria

### Tests That Must Pass

1. Each of the four docs carries its new note/section (IP casino note; SOURCES scaling sources; AGENT-DISCIPLINE N-seat examples; archival-workflow rollover section).
2. `node scripts/check-doc-links.mjs` passes (SOURCES links resolve).
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. §10 IP conservatism is reaffirmed — sources support facts only; Rulepath prose/assets stay original; no licensed content is added.
2. No code or contract surface is modified.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -niE "casino|next-phase scaling|seat-rail|phase rollover" docs/IP-POLICY.md docs/SOURCES.md docs/AGENT-DISCIPLINE.md docs/archival-workflow.md`
3. `bash scripts/boundary-check.sh`

## Outcome

Completed: 2026-06-13

Updated four policy/process docs for the public scaling phase:

- `docs/IP-POLICY.md` now has a Texas Hold'Em / common card-system note requiring
  original Rulepath prose, neutral display names where useful, original
  cards/icons, and no casino product framing; it distinguishes existing
  `poker_lite` / Crest Ledger from a future proper Hold'Em-family gate.
- `docs/SOURCES.md` now has a `Next-phase scaling sources` section covering
  Pagat Texas Hold'Em, hand ranking, Hearts, Oh Hell, Spades, 500 Rum, Chinese
  Checkers / Star Halma, OpenSpiel, boardgame.io, plus TBD rows for Pachisi and
  Mahjong-family source selection.
- `docs/AGENT-DISCIPLINE.md` now includes bounded N-seat task examples for
  simulator summary maps, 3-seat setup fixtures, pairwise no-leak harnesses,
  Rust-owned Hold'Em showdown rationale, and Rust-view-fed seat-rail UI.
- `docs/archival-workflow.md` now has a `Roadmap Phase Rollover` section for
  date-suffixed spec-index archival, authority commit/manifest notes, new index
  seeding, archive links, and bounded rollover commits.

Deviations from plan: none. No code, contract, kernel, schema, or archived spec
was changed.

Verification:

- `node scripts/check-doc-links.mjs` passed (`Checked 27 markdown files`).
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`).
- `grep -niE "casino|next-phase scaling|seat-rail|phase rollover" docs/IP-POLICY.md docs/SOURCES.md docs/AGENT-DISCIPLINE.md docs/archival-workflow.md`
  confirmed the four required note/section surfaces.
