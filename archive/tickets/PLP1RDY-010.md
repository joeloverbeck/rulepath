# PLP1RDY-010: SOURCES external prior-art notes (lessons + non-adoption)

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: None — sources doctrine doc (`docs/SOURCES.md`)
**Deps**: None

## Problem

The private-lane design draws on external prior art (event-engine and private-IP
patterns). The spec (WB-7, report `A-16`) records those lessons in the sources
doctrine — each with the Rulepath lesson and an explicit **non-adoption**
rationale, so future readers don't re-propose adopting them. This is an
independent docs edit with no dependency on the ADRs.

## Assumption Reassessment (2026-06-28)

1. Target verified present: `docs/SOURCES.md`. The prior-art set to record is
   enumerated in the spec WB-7 row: Rally the Troops/GMT, VASSAL, boardgame.io,
   OpenSpiel, GitHub reusable workflows/checkout, Cargo workspaces/git/registries.
2. Spec source: `specs/private-lane-foundation-readiness.md` WB-7 + Assumption A7
   ("No external research is needed … the change plan's already-cited prior art
   (report §2.4, Appendix B) is sufficient; the `B-*`/`A-16` work records those
   citations rather than re-researching them").
3. Cross-artifact boundary under audit: `docs/SOURCES.md` is a provenance/notes
   doc, not a behavior contract; the edit adds citation notes only and links no
   not-yet-created doc (no `check-doc-links` forward-reference risk).
4. FOUNDATIONS principle under audit (§1 "Complexity is earned" / §10 IP / §13
   ADR triggers): each prior-art note must record **why Rulepath does not adopt**
   the pattern (e.g. a declarative event DSL, a hosted runtime), keeping the
   non-adoption rationale explicit so the citation is doctrine, not a backlog item.

## Architecture Check

1. Recording lessons + non-adoption in the existing sources doc keeps prior-art
   provenance in one lawful home; it proposes no new mechanism.
2. No backwards-compatibility shim: pure additive notes.
3. `engine-core` untouched (§3); SOURCES is a provenance doc.

## Verification Layers

1. Each prior-art note carries a Rulepath lesson + non-adoption rationale ->
   manual review of `docs/SOURCES.md`.
2. Cross-artifact doc-link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `docs/SOURCES.md`

Add the external prior-art notes (Rally the Troops/GMT, VASSAL, boardgame.io,
OpenSpiel, GitHub reusable workflows/checkout, Cargo workspaces/git/registries),
each with the Rulepath lesson and the explicit non-adoption rationale (report `A-16`).

## Files to Touch

- `docs/SOURCES.md` (modify)

## Out of Scope

- Any new external research (Assumption A7: the change plan's cited prior art is
  sufficient).
- Adopting any cited pattern (the notes record non-adoption).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -qiE 'rally the troops|vassal|boardgame\.io|openspiel' docs/SOURCES.md` — the prior-art set is recorded.
2. `node scripts/check-doc-links.mjs` — no broken links.

### Invariants

1. Each note records a non-adoption rationale; no cited pattern is adopted.
2. No external IP/prose is copied — original notes only (§10).

## Test Plan

### New/Modified Tests

1. `None — sources doctrine doc; verification is command-based (prior-art greps + doc-link gate) and the non-adoption discipline is named in Assumption Reassessment.`

### Commands

1. `grep -niE 'non-adopt|not adopt|do(es)? not adopt' docs/SOURCES.md`
2. `node scripts/check-doc-links.mjs`
3. A narrower command suffices: a single docs edit, so the prior-art greps + doc-link gate are the correct verification boundary.

## Outcome

Completed the external prior-art notes in `docs/SOURCES.md`. Added a
private-lane prior-art section for Rally the Troops / GMT-style hosted modules,
VASSAL, boardgame.io, OpenSpiel, GitHub reusable workflows/checkout, and Cargo
workspaces/git/registries. Each row records the Rulepath lesson and an explicit
non-adoption rationale.

No external pattern was adopted, and no external prose was copied.

Verification:

- `grep -qiE 'rally the troops|vassal|boardgame\.io|openspiel' docs/SOURCES.md`
- `grep -niE 'non-adopt|not adopt|do(es)? not adopt' docs/SOURCES.md`
- `node scripts/check-doc-links.mjs`
- `git diff --check`
