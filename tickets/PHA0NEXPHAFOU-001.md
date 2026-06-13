# PHA0NEXPHAFOU-001: Author ADR 0007 — admit the public scaling phase and move Gate P to the tail

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — new `docs/adr/0007-*.md` only; no crates/schemas/traces/code surfaces.
**Deps**: None

## Problem

`docs/ROADMAP.md` ends at Gate 14 with a Gate P appendix, and its header is law: *"A stage or gate may be skipped or reordered only by accepted ADR."* The next phase deliberately continues the public ladder (Gate 15+) to prove 3+ official seats and larger surfaces, and moves Gate P to the very tail. Both are gate-ladder changes that the header law forbids without an accepted ADR. Until this ADR is accepted, ROADMAP cannot be edited (PHA0NEXPHAFOU-014) and no Gate 15+ spec can be grounded.

## Assumption Reassessment (2026-06-13)

1. No code/skills change. `docs/adr/ADR-TEMPLATE.md` exists and is the canonical ADR section set; `docs/adr/` holds `0001`–`0006` + `ADR-TEMPLATE.md`, so the next free ID is **0007** (verified via `ls docs/adr/`).
2. Specs/docs: `docs/ROADMAP.md:3` header states the reorder-needs-ADR law; ROADMAP §15 is the Gate P appendix. `specs/README.md` active-epoch tracker marks every Gate 15+ row blocked pending this ADR.
3. Cross-artifact boundary under audit: the ROADMAP gate ladder + Gate P placement. This ADR is the sole authority that unblocks PHA0NEXPHAFOU-014 (ROADMAP edit) and the downstream Gate 15+ specs.
4. FOUNDATIONS principle: `ROADMAP.md`'s header law is the controlling authority for gate reorder; FOUNDATIONS §13 (architecture-changing decisions) is the general backstop, and §1 priority order keeps the public ladder ranked above the private Gate P. No principle's meaning is changed.

## Architecture Check

1. An accepted ADR is the only FOUNDATIONS/ROADMAP-sanctioned way to reorder the ladder; a bare ROADMAP edit would cross the header law and FOUNDATIONS §12.
2. No backwards-compatibility aliasing/shims introduced.
3. `engine-core` is untouched and stays free of mechanic nouns (§3); no `game-stdlib` promotion (§4).

## Verification Layers

1. ADR present and follows the template → codebase grep-proof (`## ` section presence) + manual review of decision prose.
2. ADR Status/Date fields populated → grep-proof.
3. Alignment with §13/§1 + ROADMAP header law → FOUNDATIONS alignment check.

## What to Change

### 1. Create `docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md`

Build from `docs/adr/ADR-TEMPLATE.md`, every section present:
- **Context**: Gate 14 (`event_frontier`) is done; the public mechanic ladder through Gate 14 is complete but has not proven 3+ official seats or substantially larger surfaces; ROADMAP header law requires an ADR to add/reorder a gate.
- **Decision**: add a public scaling phase after Gate 14 (the Gate 15+ ladder seeded in `specs/README.md`); move Gate P to the very tail, restated as last, private, optional, non-architectural.
- **Alternatives considered**: (a) leave Gate P as the only non-done tail and skip a public phase — rejected: never proves 3+ seats through public games; (b) let private Gate P drive the scaling architecture — rejected per §1 priority order and §10 IP conservatism.
- **Impact sections** (Determinism, Replay/hash, Visibility, Data/Rust boundary, `engine-core` contamination, UI, Bot, IP, Benchmark): each "no change — the phase is admitted and proven through public games; this ADR adds no schema/visibility/bot-class/kernel change."
- **Migration notes**: none beyond the ROADMAP edit (PHA0NEXPHAFOU-014) and the spec-index reconcile (PHA0NEXPHAFOU-015).
- **Review checklist**: per `ADR-TEMPLATE.md`.

### 2. Set front matter

`Status: Proposed`, `Date: 2026-06-13`. Flag in the ADR body that acceptance is a maintainer gate that must precede the ROADMAP edit.

## Files to Touch

- `docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md` (new)

## Out of Scope

- Editing `docs/ROADMAP.md` (PHA0NEXPHAFOU-014, gated on this ADR being Accepted).
- Writing any Gate 15+ or Infra spec.
- Changing the meaning of any FOUNDATIONS principle.
- Self-accepting the ADR — acceptance is a human maintainer review.

## Acceptance Criteria

### Tests That Must Pass

1. `docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md` exists and contains every `ADR-TEMPLATE.md` section.
2. `node scripts/check-doc-links.mjs` passes (any links in the ADR resolve).
3. The ADR carries a populated `Status` (`Proposed`) and `Date` field.

### Invariants

1. `docs/ROADMAP.md` is NOT edited by this ticket — the header-law sequencing holds (ROADMAP edit waits for `Status: Accepted`).
2. No FOUNDATIONS principle is superseded; the ADR adds a roadmap phase, it does not weaken a constitutional principle.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `ls docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md`
2. `grep -E "^## " docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md` (confirm template section set)
3. `node scripts/check-doc-links.mjs`
