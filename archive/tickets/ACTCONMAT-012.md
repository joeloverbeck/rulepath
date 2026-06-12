# ACTCONMAT-012: Lift-ready doc amendments

**Status**: DONE (2026-06-12)
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — documentation only (`docs/UI-INTERACTION.md`, `docs/OFFICIAL-GAME-CONTRACT.md`, `apps/web/README.md`).
**Deps**: ACTCONMAT-002, ACTCONMAT-003, ACTCONMAT-004, ACTCONMAT-005, ACTCONMAT-006, ACTCONMAT-008, ACTCONMAT-009, ACTCONMAT-010

## Problem

The spec's enforcement law is drafted across the implementation tickets but must land atomically in the area docs once the surfaces it cites exist: the reserved-metadata-key table and §19 UI-acceptance additions (`docs/UI-INTERACTION.md`), the player-rules authoring-contract additions (`docs/OFFICIAL-GAME-CONTRACT.md` §5), and the shell-surface / smoke-layer notes (`apps/web/README.md`). Per spec §10 these amendments are "applied at WB10, not before" — this is that lift.

## Assumption Reassessment (2026-06-12)

1. `docs/UI-INTERACTION.md` has 19 numbered sections; §9 already sanctions previews showing "visible cost"; §19 already bans raw internal identifiers. `docs/OFFICIAL-GAME-CONTRACT.md` has 12 sections; the player-rules contract is the "Player-facing rules document" subsection under §5 (lines 104-128), NOT a standalone clause. The surfaces these amendments cite are delivered by ACTCONMAT-002 (reserved keys), 003 (cost/consequence rendering), 004 (composer), 005 (faction), 006 (turn report), 008 (rules contract), 009 (variant), 010 (runtime guard).
2. Spec §10: the exact amendment text is drafted in the spec (UI-INTERACTION §5/§9 reserved-key block, §19 additions; OFFICIAL-GAME-CONTRACT §5 additions). This ticket applies that text verbatim plus the `apps/web/README.md` Shell-Surface / Smoke-Layers notes.
3. Cross-artifact boundary under audit: the area-doc contracts (`UI-INTERACTION.md` §5/§9/§19, `OFFICIAL-GAME-CONTRACT.md` §5) and `apps/web/README.md`. This is a cross-cutting docs ticket — it cites each implementation surface independently, so it `Deps` each surface-delivering ticket (not just a transitive head).
4. FOUNDATIONS §7 (public UI is central product work) / §2: the amendments formalize existing law (§9 visible-cost previews become a MUST when reserved keys are emitted; §19 gains the raw-identifier runtime-guard clause) — no constitutional principle changes meaning (spec §5 verdict: no FOUNDATIONS amendment required).

## Architecture Check

1. A single trailing docs ticket lands the contract atomically once every cited surface exists, avoiding a staleness window where the doc promises a surface that has not shipped. Citing each surface independently (per-ticket `Deps`) is correct because the doc references each by name.
2. No shim: amendments are additive clarifications to existing sections; no contract is rewritten or aliased.
3. `engine-core` untouched; docs only. No `game-stdlib` change.

## Verification Layers

1. UI-INTERACTION reserved-key table + §19 additions present -> codebase grep-proof on `docs/UI-INTERACTION.md`.
2. OFFICIAL-GAME-CONTRACT §5 player-rules additions present -> codebase grep-proof.
3. `apps/web/README.md` Shell-Surface + Smoke-Layers notes present, doc links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. UI-INTERACTION.md

Apply the §5/§9 reserved-metadata-key block and the §19 additions (cost/consequence display, raw-identifier runtime guard, multi-target composer, faction-first naming, non-interactive-advance narration) per spec §10.

### 2. OFFICIAL-GAME-CONTRACT.md §5

Apply the "Player-facing rules document" additions: no maintainer sections / internal seat IDs in the player file; required resource-economy section; every public variant exposed through setup with a typed display label.

### 3. apps/web/README.md

Add the Shell Surface entries (TurnReportPanel, composer, variant selector) and Smoke Layers entry (runtime identifier guard).

## Files to Touch

- `docs/UI-INTERACTION.md` (modify; §5/§9 reserved-key block, §19 additions)
- `docs/OFFICIAL-GAME-CONTRACT.md` (modify; §5 player-rules additions)
- `apps/web/README.md` (modify; Shell Surface + Smoke Layers)

## Out of Scope

- Any implementation surface (this is docs-only; the surfaces are delivered by the `Deps` tickets).
- `specs/README.md` index flip to Done (ACTCONMAT-013 capstone).
- The §15 bot-why doc note — folded into ACTCONMAT-011's outcome if the affordance lands.

## Acceptance Criteria

### Tests That Must Pass

1. Grep-proof: the reserved-key table, §19 additions, and OFFICIAL-GAME-CONTRACT §5 additions are present at the cited sections.
2. `node scripts/check-doc-links.mjs` passes (no broken links introduced).
3. `node scripts/check-catalog-docs.mjs` passes (README catalog surfaces intact).

### Invariants

1. Amendments match the spec §10 lift-ready text verbatim; no new constitutional principle is introduced (§13 — no ADR needed).
2. Docs-only: no implementation surface is modified here.

## Test Plan

### New/Modified Tests

1. None — documentation-only ticket; verification is command-based (`check-doc-links`, `check-catalog-docs`) and grep-proofs against the post-implementation docs.

### Commands

1. `node scripts/check-doc-links.mjs`
2. `node scripts/check-catalog-docs.mjs`
3. `grep -n "reserved" docs/UI-INTERACTION.md && grep -n "Costs and economy\|resource economy" docs/OFFICIAL-GAME-CONTRACT.md`

## Completion Notes (2026-06-12)

Applied the WB10 lift-ready amendments to `docs/UI-INTERACTION.md`: the reserved presentation metadata key contract and the §19 acceptance additions for cost/consequence display, runtime raw-identifier guard coverage, multi-target composition, faction-first naming, and near-board narration of non-interactive advances.

Applied the `docs/OFFICIAL-GAME-CONTRACT.md` §5 player-facing rules additions: no maintainer sections or internal seat ids in `HOW-TO-PLAY.md`, resource-economy documentation for spendable-resource games, and typed public variant labels in setup.

Updated `apps/web/README.md` with the shared action-affordance surface, `TurnReportPanel`, typed variant selector, and runtime raw-identifier DOM guard smoke-layer note.

Verification:

1. `grep -n "reserved" docs/UI-INTERACTION.md` — passed.
2. `grep -n "Costs and economy\|resource economy" docs/OFFICIAL-GAME-CONTRACT.md` — passed.
3. `node scripts/check-doc-links.mjs` — passed (`Checked 25 markdown files`).
4. `node scripts/check-catalog-docs.mjs` — passed (`catalog-docs check passed - 14 games reflected in intro, root, and smoke surfaces`).
