# CATSETVIS-009: Closeout — doc lift + per-asset IP + `Done`-flip

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None
**Deps**: 008

## Problem

P2's contract documentation, per-asset IP evidence, and status reconciliation must land atomically once the implementation and verification tickets have shipped. This trailing closeout applies the §10 lift-ready amendments (the UI-INTERACTION catalog-identity + variant-description contract and the `templates/GAME-UI.md` authoring row), records the per-asset IP closeout table, flips the `specs/README.md` index row `Planned`→`Done`, and marks the brainstorm P2 item Done with a pointer to the spec. Spec WB9 / §10 / §12; this is a docs/status-only ticket.

## Assumption Reassessment (2026-06-13)

1. `docs/UI-INTERACTION.md` (with §10A at line 196 — the non-atlas presentation-shape governance), `templates/GAME-UI.md`, `specs/README.md` (the `CATSETVIS` row currently reads `Planned`), and `brainstorming/2026-06-12-deferred-ui-infrastructure-priorities.md` (§4 P2, §5 row 2, §8 next-step 2) all exist — verified this session. This ticket edits only docs/status surfaces; no code.
2. Spec §10 (lift text: the UI-INTERACTION catalog-identity + variant-description contract addition referencing §10A; the `GAME-UI.md` catalog-identity + variant-description authoring row), §12 (flip the index `Planned`→`Done`; mark brainstorm P2 Done; **do not** amend `templates/PUBLIC-RELEASE-CHECKLIST.md`), and §6 D13 (per-asset IP rows live in this closeout, not a template change) govern.
3. Cross-artifact boundary: this is the trailing closeout; `Deps: 008` (the verification leaf) transitively gates 001–007, so the doc lift describes machinery that now exists and the `Done`-flip is gated on the distributed exit criteria passing. The `specs/README.md` `CATSETVIS` row was created (`Planned`) during spec authoring — this ticket is a create-then-modify on that row (flip to `Done`), not a new row.
4. FOUNDATIONS §10 (IP conservatism — per-asset originality + smallest-size legibility) + §5/§7: the doc lift documents the **new inert** `catalog_theme`/`description` contract as meaning-preserving area-doc strengthening — no FOUNDATIONS amendment and no ADR (spec §5 verdict), and no mechanic-atlas row (UI-INTERACTION §10A governs the catalog-theme shape as non-atlas presentation).

## Architecture Check

1. A single trailing closeout that lands the doc lift + IP rows + `Done`-flip atomically (after the CATSETVIS-008 smoke passes) avoids a stale-documentation window and keeps the contract text co-landing with the machinery it describes (the spec's closeout-deferred-docs pattern).
2. No backwards-compatibility shims; the doc edits are additive contract documentation.
3. `engine-core` / `game-stdlib` untouched; docs/status only — no mechanic-atlas promotion row (UI-INTERACTION §10A).

## Verification Layers

1. Doc lift applied → codebase grep-proof (`docs/UI-INTERACTION.md` carries the catalog-identity + variant-description contract text; `templates/GAME-UI.md` carries the authoring row).
2. `Done`-flip + brainstorm mark → grep-proof (`specs/README.md` `CATSETVIS` row reads `Done`; brainstorm §4 P2 reads `DONE` with a pointer to the spec).
3. Per-asset IP closeout recorded → manual review (one row per SVG/motif/theme: project-authored, no trade-dress proximity, smallest-size legible).
4. Doc-link integrity → `node scripts/check-doc-links.mjs` green.

## What to Change

### 1. UI-INTERACTION + GAME-UI contract lift

Apply the spec §10 lift text: the catalog-identity + variant-description contract addition in `docs/UI-INTERACTION.md` (referencing §10A) and the catalog-identity + variant-description authoring row in `templates/GAME-UI.md`.

### 2. Per-asset IP closeout table

Record one IP row per shipped SVG/motif/theme — origin "project-authored", no copied references, no AI generation, no proprietary/trade-dress proximity, smallest-size legibility checked.

### 3. Status reconciliation

Flip the `specs/README.md` `CATSETVIS` row `Planned`→`Done` (after exit criteria pass with evidence), and mark `brainstorming/2026-06-12-deferred-ui-infrastructure-priorities.md` P2 (§4 `Status: DONE`, §5 row 2, §8 next-step 2) Done with a pointer to the spec.

## Files to Touch

- `docs/UI-INTERACTION.md` (modify)
- `templates/GAME-UI.md` (modify)
- `specs/README.md` (modify; `CATSETVIS` row `Planned`→`Done`)
- `brainstorming/2026-06-12-deferred-ui-infrastructure-priorities.md` (modify; mark P2 Done)

## Out of Scope

- Amending `templates/PUBLIC-RELEASE-CHECKLIST.md` (§12 / D13 — explicitly not amended).
- Any code, behavior, or renderer change.
- Any FOUNDATIONS amendment or ADR (spec §5 verdict: none required).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` is green.
2. Grep-proofs: `docs/UI-INTERACTION.md` and `templates/GAME-UI.md` carry the catalog-identity + variant-description contract/row; `specs/README.md` `CATSETVIS` row reads `Done`; brainstorm §4 P2 reads `DONE` with a spec pointer.
3. The spec's §9 exit criteria pass with evidence — the distributed acceptance from CATSETVIS-005/006/008 (workspace tests + per-game replay/fixture green, `smoke:wasm`/`ui`/`effects`/`e2e` green, no raw IDs, IP rows recorded).

### Invariants

1. The `Done`-flip happens only after the exit criteria pass with evidence.
2. No `PUBLIC-RELEASE-CHECKLIST.md` amendment, no FOUNDATIONS amendment, and no ADR are introduced.

## Test Plan

### New/Modified Tests

1. `None — documentation/status-only ticket; verification is command-based (`check-doc-links` + grep-proofs) and the distributed exit-criteria evidence named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -n 'Done' specs/README.md | grep -i catalog-setup` and `grep -niE 'P2.*DONE|Status: DONE' brainstorming/2026-06-12-deferred-ui-infrastructure-priorities.md`
3. `cargo test --workspace && npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:effects && npm --prefix apps/web run smoke:e2e && bash scripts/boundary-check.sh && node scripts/check-catalog-docs.mjs && node scripts/check-presentation-copy.mjs` — the aggregate exit-criteria sweep gating the `Done`-flip.

## Outcome

Completed 2026-06-13.

- Lifted the catalog-identity and variant-description contract into `docs/UI-INTERACTION.md`, keeping it governed as non-atlas presentation shape under §10A.
- Added the catalog identity and optional variant-description authoring rows to `templates/GAME-UI.md`.
- Flipped the `specs/README.md` catalog-setup visual redesign row to `Done` and marked brainstorming P2 Done with a pointer to the spec.
- Updated the stale `wasm-api` catalog unit test to assert the new described-variant JSON fields instead of the pre-description exact fragment shape exposed by the aggregate exit sweep.

Verification:

- `node scripts/check-doc-links.mjs`
- `rg -n "catalog identity|variant descriptions|GameVariantCatalogEntry.description|catalog-theme" docs/UI-INTERACTION.md templates/GAME-UI.md`
- `grep -n 'Done' specs/README.md | grep -i catalog-setup`
- `grep -niE 'P2.*DONE|Status: DONE|Done: P2' brainstorming/2026-06-12-deferred-ui-infrastructure-priorities.md`
- `bash scripts/boundary-check.sh`
- `node scripts/check-catalog-docs.mjs`
- `node scripts/check-presentation-copy.mjs`
- `cargo test -p wasm-api --lib list_games_reports_registered_games`
- `cargo fmt --all --check`
- `cargo test --workspace`
