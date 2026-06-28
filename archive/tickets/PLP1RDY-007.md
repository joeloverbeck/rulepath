# PLP1RDY-007: Part C VCS/CI/catalog doctrine + seam plans (no code)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — doctrine docs + templates (`docs/IP-POLICY.md`, `docs/ARCHITECTURE.md`, `docs/WASM-CLIENT-BOUNDARY.md`, `docs/UI-INTERACTION.md`, `apps/web/README.md`, `templates/GAME-UI.md`, `templates/PUBLIC-RELEASE-CHECKLIST.md`, `templates/PRIVATE-RELEASE-CHECKLIST.md`)
**Deps**: PLP1RDY-003, PLP1RDY-005

## Problem

ADR 0012 defaults private games to a separate private repository with a private
WASM/catalog/renderer overlay. The spec records the Part C VCS/CI/catalog design
as **written doctrine + documented seam plans** (report `C-01`–`C-08`, plus
`A-08`, `A-11`, `A-12`, `B-08`, `B-16`) — explicitly **no `.rs`/`.mjs`/`.yml`/
`.toml` source change in this unit; the seam *implementations* are seeded forward.
This is WB-4.

## Assumption Reassessment (2026-06-28)

1. Targets verified present: `docs/IP-POLICY.md`, `docs/ARCHITECTURE.md`,
   `docs/WASM-CLIENT-BOUNDARY.md`, `docs/UI-INTERACTION.md`, `apps/web/README.md`,
   `templates/GAME-UI.md`, `templates/PUBLIC-RELEASE-CHECKLIST.md`. The new
   `templates/PRIVATE-RELEASE-CHECKLIST.md` does not yet exist (confirmed; it is
   the `B-16` deliverable).
2. Spec source: `specs/private-lane-foundation-readiness.md` WB-4 + §Scope "Part C
   VCS/CI/catalog doctrine + seam plans (report `C-01`–`C-08`)" + §Forbidden
   changes ("No edit to any `.rs`, `.mjs`, `.yml`, `.toml`, `.json`, or web/source
   file — Part C is doctrine + seam plans only").
3. Cross-artifact boundary under audit: `docs/IP-POLICY.md` is **also** edited by
   PLP1RDY-005 (sanctioned-lane section). This ticket `Deps` PLP1RDY-005 so the
   two IP-POLICY edits serialize (no merge conflict); this ticket adds the Part C
   repo-doctrine section only. ADR 0012 (PLP1RDY-003) is the authorizing
   supersession — this ticket gates on its `Status: Accepted`.
4. FOUNDATIONS principle under audit (§10 IP / §11 isolation / §13
   private-architecture trigger): the doctrine records the **default separate-
   private-repo** decision, the rejected alternatives (public submodule / default
   optional dependency), and the catalog/renderer/CI-federation/drift **seam
   plans** — the public repo gains only generic, private-free extension seams.
5. §11/§12 no-public-contamination touched: `apps/web/README.md` gets a
   **doctrine** note (private renderer overlay; public-catalog-only), NOT a catalog
   row — so `node scripts/check-catalog-docs.mjs` must still see zero private
   games. The edit ships no public code/CI change; enforcement of "no private game
   in public catalog" stays with the existing `check-catalog-docs` gate.

## Architecture Check

1. Recording Part C as doctrine + seam plans (not code) is exactly the ADR-0012
   scope: it documents the isolation-maximal default and the extraction seams
   without prematurely refactoring the public catalog/renderer/CI.
2. No backwards-compatibility shim: the public catalog/renderer/CI are unchanged;
   the seam plans describe additive generic extension points, seeded forward.
3. `engine-core` and the public catalog stay private-free (§3, §12); the overlay
   is private-build-only.

## Verification Layers

1. Default-private-repo decision + rejected-alternatives table + seam plans present
   -> codebase grep-proof across `docs/IP-POLICY.md` and `docs/ARCHITECTURE.md`.
2. apps/web/README note is doctrine, not a catalog row -> `node scripts/check-catalog-docs.mjs`
   passes (zero private games in public catalog/README/smoke surfaces).
3. No public code/CI change in this unit -> `git status --porcelain -- '*.rs' '*.mjs' '*.yml' '*.toml' '*.json'` empty.
4. Acceptance precondition + new template -> grep `^Status: Accepted` in
   `docs/adr/0012-*.md`; `test -f templates/PRIVATE-RELEASE-CHECKLIST.md`.

## What to Change

### 1. `docs/IP-POLICY.md` + `docs/ARCHITECTURE.md`

Add the default separate-private-repo decision, the rejected-alternatives table,
and the catalog / renderer / CI-federation / drift-check seam plans (report
`C-01`–`C-08`, `A-08` large-action-tree + private-overlay lane).

### 2. `docs/WASM-CLIENT-BOUNDARY.md` + `docs/UI-INTERACTION.md` + `apps/web/README.md`

Private catalog semantics + catalog seam plan (`A-11`); private web-overlay +
large asymmetric UI (`A-12`); apps/web/README private renderer-overlay +
public-only catalog **doctrine** note.

### 3. Templates

`templates/GAME-UI.md` private overlay / large-map (`B-08`); new
`templates/PRIVATE-RELEASE-CHECKLIST.md` + a cross-link from
`templates/PUBLIC-RELEASE-CHECKLIST.md` (`B-16`).

## Files to Touch

- `docs/IP-POLICY.md` (modify; Part C section only — serialized after PLP1RDY-005)
- `docs/ARCHITECTURE.md` (modify)
- `docs/WASM-CLIENT-BOUNDARY.md` (modify)
- `docs/UI-INTERACTION.md` (modify)
- `apps/web/README.md` (modify)
- `templates/GAME-UI.md` (modify)
- `templates/PUBLIC-RELEASE-CHECKLIST.md` (modify)
- `templates/PRIVATE-RELEASE-CHECKLIST.md` (new)

## Out of Scope

- Any catalog registry/adapter extraction, renderer-registry seam, reusable-workflow
  CI federation, or drift/boundary-check split **implementation** (seeded forward).
- The IP-POLICY sanctioned-lane section (PLP1RDY-005).
- Creating the private repository.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -qiE 'separate private repositor|rejected alternativ|seam plan' docs/IP-POLICY.md docs/ARCHITECTURE.md`.
2. `node scripts/check-catalog-docs.mjs` — zero private games in public catalog/README/smoke surfaces.
3. `test -f templates/PRIVATE-RELEASE-CHECKLIST.md && grep -q '^Status: Accepted' docs/adr/0012-*.md && node scripts/check-doc-links.mjs`.

### Invariants

1. No `.rs`/`.mjs`/`.yml`/`.toml`/`.json`/web source change lands in this unit.
2. The apps/web/README note adds no public catalog row; the public catalog stays
   private-free.

## Test Plan

### New/Modified Tests

1. `None — doctrine docs + templates; verification is command-based (doctrine greps + catalog-docs gate + no-public-code-diff check) and the no-contamination invariant set is named in Assumption Reassessment.`

### Commands

1. `git status --porcelain -- '*.rs' '*.mjs' '*.yml' '*.toml' '*.json'`
2. `node scripts/check-catalog-docs.mjs && node scripts/check-doc-links.mjs`
3. A narrower command suffices: doctrine/templates only, so the catalog-docs + doc-link gates plus the no-public-code-diff guard are the correct verification boundary.

## Outcome

Completed the Part C doctrine and seam-plan pass without source/config changes.
Added the default separate-private-repository decision, rejected alternatives,
and catalog/renderer/CI/drift seam plans to `docs/IP-POLICY.md` and
`docs/ARCHITECTURE.md`. Added public-catalog/private-overlay semantics to
`docs/WASM-CLIENT-BOUNDARY.md`, private renderer-overlay and large asymmetric UI
guidance to `docs/UI-INTERACTION.md`, and a public-only catalog doctrine note to
`apps/web/README.md`.

Updated `templates/GAME-UI.md` for private overlay / large-map planning, linked
the public release checklist to the private counterpart, and added
`templates/PRIVATE-RELEASE-CHECKLIST.md` with authority, build separation,
public back-leak, viewer-safety, and release-decision sections.

Verification:

- `grep -qiE 'separate private repositor|rejected alternativ|seam plan' docs/IP-POLICY.md docs/ARCHITECTURE.md`
- `test -f templates/PRIVATE-RELEASE-CHECKLIST.md && grep -q '^Status: Accepted' docs/adr/0012-*.md`
- `git status --porcelain -- '*.rs' '*.mjs' '*.yml' '*.toml' '*.json'`
- `node scripts/check-catalog-docs.mjs`
- `node scripts/check-doc-links.mjs`
- `git diff --check`
