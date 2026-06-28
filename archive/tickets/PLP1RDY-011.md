# PLP1RDY-011: Roadmap & specs placement — Private Lane P1 + tracker + M1 note

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — roadmap/contract/index docs (`docs/ROADMAP.md`, `docs/OFFICIAL-GAME-CONTRACT.md`, `specs/README.md`)
**Deps**: PLP1RDY-004, PLP1RDY-005, PLP1RDY-008

## Problem

Private Lane P1 needs a roadmap placement beside the public gate order, a public
specs-index tracker with an opaque `P1-M1` row, and an OFFICIAL-GAME-CONTRACT
milestone-1 capability/non-goals note + required private-spec field set. The spec
bundles this as WB-8 (report `E-01`, `E-02`, `E-03`).

## Assumption Reassessment (2026-06-28)

1. Targets verified present: `docs/ROADMAP.md`, `docs/OFFICIAL-GAME-CONTRACT.md`,
   `specs/README.md`. **Partial pre-landed**: `specs/README.md` already carries
   the `PLP1-RDY` index row (`Planned`) and the Gate P interlock note pointing at
   this readiness unit + ADRs 0010/0011/0012 (landed during the `/reassess-spec`
   session, 2026-06-28). Those are **verify-only**; this ticket lands the **new
   Private lane tracker section + the opaque `P1-M1` row** (`Doctrine pending`,
   linking only to the accepted ADRs).
2. Spec source: `specs/private-lane-foundation-readiness.md` WB-8 +
   §Documentation-updates ("a new Private lane tracker section with the opaque
   `P1-M1` row at `Doctrine pending`") + §Exit-criteria item 6.
3. Cross-artifact boundary under audit: `docs/ROADMAP.md` is **also** edited by
   PLP1RDY-005 (ADR-0007-limited note) and `docs/OFFICIAL-GAME-CONTRACT.md` by
   PLP1RDY-008 (completion profiles). This ticket `Deps` both so the shared-file
   edits serialize; it adds the Private Lane P1 ROADMAP section + the OGC M1 note.
4. FOUNDATIONS principle under audit (§10 IP / opaque-identifier rule): the
   roadmap shows Private Lane P1 **beside (not inside)** the public gate order and
   uses the opaque identifier; the `P1-M1` tracker row names no licensed title and
   links only to the accepted ADRs.

## Architecture Check

1. Placing Private Lane P1 beside the public ladder (not reordering Gates 21–23)
   realizes the ADR-0010 parallel-lane intent without disturbing public sequencing.
2. No backwards-compatibility shim: the tracker is a new opaque row; the existing
   PLP1-RDY row + Gate P note are reused, not duplicated.
3. `engine-core` untouched (§3); index/roadmap/contract docs only.

## Verification Layers

1. Private Lane P1 ROADMAP section + opaque `P1-M1` tracker row present ->
   codebase grep-proof (`docs/ROADMAP.md`, `specs/README.md`); the `P1-M1` row
   reads `Doctrine pending` and links only to ADRs.
2. OGC M1 capability/non-goals + required private-spec field set present -> grep
   `docs/OFFICIAL-GAME-CONTRACT.md`.
3. No licensed title leaked -> grep the added text carries the opaque identifier
   only; `node scripts/check-catalog-docs.mjs` still sees zero private games.
4. Cross-artifact doc-link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `docs/ROADMAP.md`

Add the Private Lane P1 section beside the public gate order (report `E-01`).

### 2. `specs/README.md`

Add the new **Private lane tracker** section + the opaque `P1-M1` row at
`Doctrine pending`, linking only to the accepted ADRs (report `E-02`). Verify the
pre-landed `PLP1-RDY` row + Gate P interlock note are intact (do not duplicate).

### 3. `docs/OFFICIAL-GAME-CONTRACT.md`

Add the milestone-1 capability target + explicit non-goals note and the required
private-spec field set (report `E-03`).

## Files to Touch

- `docs/ROADMAP.md` (modify; Private Lane P1 section — serialized after PLP1RDY-005)
- `specs/README.md` (modify; new tracker section + `P1-M1` row — `PLP1-RDY` row & Gate P note verify-only)
- `docs/OFFICIAL-GAME-CONTRACT.md` (modify; M1 note — serialized after PLP1RDY-008)

## Out of Scope

- Flipping the `PLP1-RDY` row `Planned` → `Done` (PLP1RDY-012 closeout).
- Authoring the private implementation spec (private repo, out of scope).
- The OFFICIAL-GAME-CONTRACT completion profiles (PLP1RDY-008).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -qi 'Private Lane P1' docs/ROADMAP.md && grep -q 'P1-M1' specs/README.md`.
2. `node scripts/check-catalog-docs.mjs` — zero private games in public catalog surfaces.
3. `node scripts/check-doc-links.mjs` — tracker/ADR links resolve.

### Invariants

1. Private Lane P1 sits beside, not inside, the public gate order; Gates 21–23 are
   unreordered.
2. No licensed title or private ID appears — opaque identifier only (§10).

## Test Plan

### New/Modified Tests

1. `None — roadmap/contract/index docs; verification is command-based (placement greps + catalog-docs + doc-link gates) and the opaque-identifier invariant is named in Assumption Reassessment.`

### Commands

1. `grep -nE 'Private Lane P1|P1-M1|Doctrine pending' docs/ROADMAP.md specs/README.md`
2. `node scripts/check-catalog-docs.mjs && node scripts/check-doc-links.mjs`
3. A narrower command suffices: index/roadmap/contract docs only, so the placement greps + the catalog-docs and doc-link gates are the correct verification boundary.

## Outcome

Completed the roadmap, spec-index, and official-contract placement pass. Added a
Private Lane P1 roadmap section beside the public gate order without reordering
Gates 21-23. Added the `P1-M1` opaque private-lane tracker row at `Doctrine
pending` in `specs/README.md`, linking only to accepted ADRs 0010, 0011, and
0012. Added the Private Lane P1 milestone 1 capability target, explicit
non-goals, and required private-spec field set to
`docs/OFFICIAL-GAME-CONTRACT.md`.

Verification:

- `grep -nE 'Private Lane P1|P1-M1|Doctrine pending' docs/ROADMAP.md specs/README.md docs/OFFICIAL-GAME-CONTRACT.md`
- `node scripts/check-catalog-docs.mjs`
- `node scripts/check-doc-links.mjs`
- `git diff --check`
- diff review confirmed the added lines use opaque identifiers only; the existing public PLP1-RDY report filename context was not introduced by this ticket.
