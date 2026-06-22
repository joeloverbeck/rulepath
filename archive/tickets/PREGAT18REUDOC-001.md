# PREGAT18REUDOC-001: docs/README authority-map hygiene + ADR status index

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — docs-only (`docs/README.md`)
**Deps**: None

## Problem

`docs/README.md` is the ordered authority map the foundation set subordinates to, but it omits `docs/TRACE-SCHEMA-v1.md` (a governing trace/replay-fixture doc), carries no central ADR status index, and never states that `Proposed` ADRs are informative only. Readers cannot tell, from the authority map, which ADRs are binding law versus advisory — a gap the pre-Gate-18 ADR work (ADR 0008/0009 authoring, ADR 0005 disposition) depends on.

## Assumption Reassessment (2026-06-22)

1. Verified `docs/README.md` carries a 15-item ordered authority map (lines 9–25) that lists `docs/adr/ADR-TEMPLATE.md` but **not** `docs/TRACE-SCHEMA-v1.md`, and has no ADR status index and no Proposed-ADR policy statement (confirmed via the `/reassess-spec` pass this session; spec §Assumptions A9 "confirmed absent / edit needed").
2. Verified against spec D6 / WB1: the register (`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`) and fixture-contract (`docs/EVIDENCE-FIXTURE-CONTRACT.md`) authority-map lines are owned by WB5 (tickets 006/007), which co-land them with the files they create — adding them here would link not-yet-created docs and turn `check-doc-links.mjs` red.
3. Cross-artifact boundary under audit: `docs/README.md` is the authority index ADRs and area docs subordinate to; the ADR status index added here is the surface ticket 003 (ADR 0005 disposition) updates when 0005's status changes.
4. FOUNDATIONS L3 ("supersede only by accepted ADR") motivates the Proposed-ADR statement: a `Proposed` ADR carries no binding authority and active docs must not cite it as accepted law.

## Architecture Check

1. Listing `TRACE-SCHEMA-v1.md` at its correct layer (subordinate to FOUNDATIONS / ARCHITECTURE / ADR 0004) and adding a single status index is cleaner than scattering per-ADR status notes across area docs.
2. No backwards-compatibility shims or alias paths introduced.
3. Docs-only: `engine-core` stays free of mechanic nouns (§3); no `game-stdlib` promotion (§4).

## Verification Layers

1. `TRACE-SCHEMA-v1.md` listed at the correct authority layer -> codebase grep-proof (`grep -n TRACE-SCHEMA-v1 docs/README.md`).
2. ADR status index lists every ADR 0001–0007 with its current status -> manual review + grep.
3. "Proposed ADRs are informative only" statement present -> grep.
4. All authority-map and ADR links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Authority map

Insert `docs/TRACE-SCHEMA-v1.md` into the ordered list at its correct layer (below FOUNDATIONS / ARCHITECTURE / ADR 0004), with a one-line role description consistent with the existing entries.

### 2. Central ADR status index

Add a new section listing every ADR (0001–0007 today) with its `Status` value and one-line subject, so a reader sees binding-vs-advisory at a glance.

### 3. Proposed-ADR policy

State that `Proposed` ADRs are informative only and must not be cited as accepted law (FOUNDATIONS L3).

## Files to Touch

- `docs/README.md` (modify)

## Out of Scope

- Register / fixture-contract authority-map lines (tickets 006/007 co-land them).
- ADR-TEMPLATE strengthening (ticket 002).
- ADR 0005 disposition / status-index row update (ticket 003).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n "TRACE-SCHEMA-v1" docs/README.md` returns the new authority-map entry.
2. `grep -niE "proposed adr" docs/README.md` returns the informative-only statement.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The authority map stays ordered by layer (constitution → architecture → boundary → ADRs → area docs).
2. No `Proposed` ADR is presented as binding law anywhere in `docs/README.md`.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline coverage (`check-doc-links.mjs`) is named in Assumption Reassessment.`

### Commands

1. `grep -nE "TRACE-SCHEMA-v1|[Pp]roposed ADR" docs/README.md`
2. `node scripts/check-doc-links.mjs`
3. A narrower grep is the correct boundary for the new entries; `check-doc-links.mjs` is the full-pipeline link-integrity gate.

## Outcome

Completed: 2026-06-22

Changed `docs/README.md` to add `TRACE-SCHEMA-v1.md` to the ordered authority
map, add a central ADR status index for ADRs 0001 through 0007, and state that
Proposed ADRs are informative only until accepted and paired with the named
foundation updates.

Deviations: the live authority map is a table rather than the numbered list
wording in the ticket, so the table format was preserved and later rows were
renumbered.

Verification:

- `grep -nE "TRACE-SCHEMA-v1|[Pp]roposed ADR" docs/README.md` returned the
  Trace Schema authority-map row and the Proposed ADR policy statement.
- `node scripts/check-doc-links.mjs` passed (`Checked 27 markdown files`).
