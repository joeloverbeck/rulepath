# PHA0NEXPHAFOU-010: docs/adr edits — 0004/0006 cross-reference notes, ADR-TEMPLATE fields, resolve 0005 status

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — `docs/adr/0004-*.md`, `docs/adr/0006-*.md`, `docs/adr/ADR-TEMPLATE.md`, `docs/adr/0005-*.md` edits only.
**Deps**: PHA0NEXPHAFOU-001

## Problem

Several `docs/adr/` artifacts need next-phase reconciliation: ADR 0004's hidden-info export examples are heads-up and need a pairwise N-player cross-reference; ADR 0006 is a useful casino-adjacent placement precedent to cite from the Hold'Em scope; `ADR-TEMPLATE.md` lacks fields a scaling ADR needs; and ADR 0005's status is ambiguous (`Proposed` in the file while some planning text treats it as accepted), which must not be carried into a phase that authors larger, noisier benchmarks.

## Assumption Reassessment (2026-06-13)

1. No code change. `docs/adr/` holds `0004-hidden-info-replay-export-taxonomy.md`, `0005-variance-aware-ci-benchmark-floors.md`, `0006-blackjack-lite-roadmap-placement.md`, and `ADR-TEMPLATE.md` (verified via `ls docs/adr/`).
2. Docs: the four ADR artifacts above; ADR 0007 (PHA0NEXPHAFOU-001) is the new sibling these notes coexist with; `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (PHA0NEXPHAFOU-002) is the cross-reference target for the 0004 note.
3. Cross-artifact boundary under audit: the `docs/adr/` directory; shared surface = the export no-leak taxonomy (0004), the placement precedent (0006), the ADR authoring template, and 0005's status field.
4. FOUNDATIONS principle restate: §13 ADR discipline — *"Supersede only by accepted ADR."* The 0004/0006 edits are **notes only** and must not weaken or supersede those decisions; the 0005 resolution upholds "do not rely on a `Proposed` ADR as law."
5. Enforcement surface: §11 no-leak firewall (ADR 0004 viewer-scoped export). The pairwise note clarifies the existing decision; it introduces no leakage path and does not change 0004's normative content.
6. The ADR 0005 status change touches a doc-governed contract's status field: grep the repo (`docs/`, `specs/`, `.claude/skills/`, `archive/`) for citations of ADR 0005 / "variance-aware" as *accepted* and reconcile the blast radius so no surface cites a `Proposed` ADR as law.

## Architecture Check

1. Grouping all `docs/adr/` edits into one reviewable diff is cleaner than scattering ADR-directory changes across unrelated tickets; the 0005 resolution removes the "Proposed-cited-as-accepted" hazard before the noisier-benchmark phase.
2. No backwards-compatibility aliasing/shims introduced.
3. The 0004/0006 edits add cross-reference notes only — they do not supersede an accepted ADR (FOUNDATIONS: supersede only by accepted ADR); `engine-core` is untouched.

## Verification Layers

1. ADR 0004 pairwise note added without altering its decision → manual review + grep-proof (0004 Status/Decision text unchanged).
2. ADR 0006 cross-reference note added without amendment → manual review.
3. ADR-TEMPLATE optional scaling fields present → codebase grep-proof.
4. ADR 0005 status consistent everywhere → grep-proof (no surface cites 0005 as accepted unless its Status is flipped to Accepted).

## What to Change

### 1. ADR 0004 (`0004-hidden-info-replay-export-taxonomy.md`)

Add a non-normative note / cross-reference to `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`: viewer-scoped export is per authorized viewer; seat A's export must not leak B/C/D private state; public export remains the default. Do not weaken or supersede the decision.

### 2. ADR 0006 (`0006-blackjack-lite-roadmap-placement.md`)

Add a cross-reference note that it is a precedent for casino-adjacent public-presentation restraint, to be cited from the future Hold'Em spec scope while distinguishing Hold'Em from deferred Blackjack. Do not amend the decision.

### 3. `ADR-TEMPLATE.md`

Add optional rows for scaling ADRs: affected foundation sections, superseded decision (if any), compatibility with hidden-info no-leak, compatibility with no-DSL/no-YAML, migration plan, rollback/contamination risk.

### 4. ADR 0005 status resolution (decision)

**Default (recommended): stop citing ADR 0005 as accepted.** Grep for surfaces that treat 0005 as accepted and make them consistent with its actual `Proposed` status, so no new-phase spec relies on a `Proposed` ADR as law. **Alternative branch** (if maintainers choose): accept ADR 0005 through the normal process — flip its `Status` to `Accepted` and update index/citation references. The ticket records which branch was taken.

## Files to Touch

- `docs/adr/0004-hidden-info-replay-export-taxonomy.md` (modify)
- `docs/adr/0006-blackjack-lite-roadmap-placement.md` (modify)
- `docs/adr/ADR-TEMPLATE.md` (modify)
- `docs/adr/0005-variance-aware-ci-benchmark-floors.md` (modify)

## Out of Scope

- Weakening or superseding ADR 0004 or 0006 (notes/cross-references only).
- Authoring ADR 0007 (PHA0NEXPHAFOU-001 owns it).
- Recalibrating the benchmark floors that are 0005's actual subject.

## Acceptance Criteria

### Tests That Must Pass

1. ADR 0004 carries the pairwise N-player export note with its decision text unchanged; ADR 0006 carries the casino-adjacent precedent cross-reference unchanged in decision.
2. `ADR-TEMPLATE.md` carries the optional scaling-ADR rows.
3. ADR 0005's status is consistent across the repo (no surface cites it as accepted unless its `Status` was flipped to `Accepted`); `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The ADR 0004 and 0006 decisions are unchanged in meaning — the edits are explanatory notes, not supersessions.
2. No `Proposed` ADR is cited anywhere as law after this ticket.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -rniE "0005|variance-aware" docs/ specs/ .claude/skills/ | grep -i accept` (expect no "accepted" citation unless 0005 was flipped)
3. `grep -niE "pairwise|precedent|affected foundation sections" docs/adr/0004-*.md docs/adr/0006-*.md docs/adr/ADR-TEMPLATE.md`

## Outcome

Completed: 2026-06-13

Resolved the ADR 0005 branch by keeping `docs/adr/0005-variance-aware-ci-benchmark-floors.md`
`Status: Proposed` and stopping active doctrine from citing it as binding law.
Added a status note to ADR 0005, adjusted its alternatives wording so it does not
look accepted inside a Proposed ADR, and updated
`docs/TESTING-REPLAY-BENCHMARKING.md` to treat ADR 0005 as a proposal that
applies only if maintainers accept it. The active Phase 0 spec wording was also
updated from "accepted" language to the chosen "binding" consistency branch.

Added a non-normative N-player pairwise export note to ADR 0004, pointing to the
multi-seat contract while leaving the ADR decision unchanged. Added a
non-normative casino-adjacent placement precedent note to ADR 0006 for future
Hold'Em-family scope work, also without changing the ADR decision. Added optional
scaling/supersession fields to `docs/adr/ADR-TEMPLATE.md`.

Deviations from plan: touched `docs/TESTING-REPLAY-BENCHMARKING.md` and the
active Phase 0 spec to complete the required ADR 0005 consistency cleanup across
active doctrine. No code, benchmark thresholds, schemas, or accepted ADR
decisions changed.

Verification:

- `node scripts/check-doc-links.mjs` passed (`Checked 27 markdown files`).
- `grep -rniE "0005|variance-aware" docs/ specs/ .claude/skills/ 2>/dev/null | grep -i accept`
  returned no matches, proving no active surface cites ADR 0005 as accepted.
- `grep -niE "pairwise|precedent|affected foundation sections" docs/adr/0004-*.md docs/adr/0006-*.md docs/adr/ADR-TEMPLATE.md`
  found the ADR 0004 note, ADR 0006 note, and ADR template field.
