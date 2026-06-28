# PLP1RDY-003: Author and accept ADR 0012 — Private Repository, CI Federation, and Catalog Overlay

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — new governance doc (`docs/adr/0012-private-repository-ci-catalog-overlay.md`)
**Deps**: None

## Problem

Private games must not contaminate public architecture (FOUNDATIONS §12: "private
monster-game work starts shaping public architecture"; §13 trigger: "allowing
private licensed experiments to influence public architecture"). The spec (WB-1c)
requires an **accepted** ADR that defaults private games to a *separate private
repository* pinning the public Rulepath commit, with a private WASM/catalog/
renderer overlay — recorded as doctrine + documented seam plans, with **no public
code/CI change in this unit** — before the Part C doctrine (PLP1RDY-007) lands.
This ticket authors and accepts that ADR.

## Assumption Reassessment (2026-06-28)

1. ADR template + numbering: `docs/adr/ADR-TEMPLATE.md` exists; `0012` is the
   next integer after `0011`. No `Deps`: the integer ordering is stable
   regardless of write order among 001–003.
2. Verbatim Decision is `specs/private-lane-foundation-readiness.md` §5.3, which
   ends "**Records doctrine + documented seam plans only — no public code/CI
   change in the readiness unit**" (the catalog/renderer/CI/drift seam
   implementations are seeded forward, spec §Out of scope).
3. Cross-artifact boundary under audit: the ADR amends FOUNDATIONS
   private-architecture trigger, `docs/IP-POLICY.md` private build/repo rules,
   `docs/WASM-CLIENT-BOUNDARY.md` catalog boundary, and `docs/ARCHITECTURE.md`
   overlay shape — all consumed by PLP1RDY-007. Confirmed those docs exist.
4. FOUNDATIONS principle under audit (§10 IP conservatism / §11 "Private licensed
   experiments remain isolated and non-architectural" / §13 private-architecture
   trigger): the ADR permits the public repo to gain only **generic, private-free
   extension seams**; public catalog contains only public games; private catalog
   entries appear only in private build artifacts; a public submodule/feature/
   optional dependency naming private games is rejected as the default.

## Architecture Check

1. A separate private repository with a pinned public checkout is the
   isolation-maximal default: it keeps private IP out of every public surface
   (§10/§11) while letting the public repo expose only generic seams. The
   rejected alternatives (public submodule / default optional dependency) are
   recorded with rationale, not silently dropped.
2. No backwards-compatibility shim: the ADR records doctrine + seam *plans*; it
   introduces no public code or CI change to alias or migrate.
3. `engine-core` and the public catalog stay private-free (§3, §12); the overlay
   lives entirely in the private build.

## Verification Layers

1. ADR exists with correct ID/status -> codebase grep-proof (`test -f` + status grep).
2. Decision matches verbatim §5.3, including the "doctrine + seam plans only, no
   public code/CI change" clause -> manual review against the spec.
3. No public code/CI change in this unit -> `git status --porcelain` shows only
   the new ADR markdown (no `.rs`/`.mjs`/`.yml`/`.toml` diff).
4. Isolation/no-leak invariants preserved -> FOUNDATIONS alignment check (§10,
   §11 isolation invariant, §12 private-architecture stop, §13 trigger).

## What to Change

### 1. Author `docs/adr/0012-private-repository-ci-catalog-overlay.md`

From `docs/adr/ADR-TEMPLATE.md`, full section set with all impact sections,
`Status: Accepted`, date 2026-06-28, ID `0012`. Decision text is the verbatim
block from spec §5.3. The Alternatives-considered section records the rejected
public-submodule / default-optional-dependency options. The ADR names the amended
FOUNDATIONS / IP-POLICY / WASM-CLIENT-BOUNDARY / ARCHITECTURE lines, flags the
constitution supersession, and states that it records doctrine + documented seam
plans only.

### 2. Acceptance

Record `Status: Accepted` (Assumption A3). PLP1RDY-007 gates on this status.

## Files to Touch

- `docs/adr/0012-private-repository-ci-catalog-overlay.md` (new)

## Out of Scope

- The Part C doctrine edits to IP-POLICY / ARCHITECTURE / WASM-CLIENT-BOUNDARY /
  apps/web/README and the new PRIVATE-RELEASE-CHECKLIST template (PLP1RDY-007).
- Any catalog/renderer/CI/drift **seam implementation** (seeded forward).
- Creating the private repository (later, out of scope per spec §Out of scope).

## Acceptance Criteria

### Tests That Must Pass

1. `test -f docs/adr/0012-private-repository-ci-catalog-overlay.md && grep -q '^Status: Accepted' docs/adr/0012-private-repository-ci-catalog-overlay.md`
2. `git status --porcelain -- '*.rs' '*.mjs' '*.yml' '*.toml'` — empty: this unit ships no public code/CI change.
3. `node scripts/check-doc-links.mjs` — no broken links introduced.

### Invariants

1. Public repo gains only generic, private-free extension seams (§11).
2. No public code, CI manifest, or catalog constant change lands in this unit.

## Test Plan

### New/Modified Tests

1. `None — governance/ADR doc; verification is command-based (doc-link gate + status grep + no-public-code-diff check) and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE '^Status: Accepted' docs/adr/0012-private-repository-ci-catalog-overlay.md`
2. `node scripts/check-doc-links.mjs`
3. A narrower command suffices: the ADR ships only markdown, so doc-link integrity, the status grep, and the no-public-code-diff guard are the correct verification boundary.
