# CARACTPRES-008: Catalog action-presentation audit and residual migrations

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web` (audit record + audit-mandated board migrations); no Rust surface touched
**Deps**: CARACTPRES-007

## Problem

Eight games hand-roll action panels directly from `actionTree.choices` and the rest use the generic `ActionControls` fallback (`apps/web/src/main.tsx:388-545`, `apps/web/src/components/ActionControls.tsx:19-48`); nothing records which presentation each game *should* use. Spec D4 mandates a per-game adoption audit — every catalog game gets one explicit disposition: **adopt** (`ActionPathBuilder`), **board-native** (on-board affordances already map 1:1 to Rust choices), or **fallback** (`ActionControls` adequate for single-stage games) — plus implementation of any residual flat-leaf-list migrations the audit mandates. No game silently keeps a flat leaf dump (spec §9 exit criterion 4).

## Assumption Reassessment (2026-06-12)

1. The catalog is 14 games with per-game board components dispatched in `apps/web/src/main.tsx:388-523`; games with custom boards bypass `ActionControls` (`:525-545`). Known panel styles from this session's exploration: `PlainTricksBoard` (cards as buttons, board-native), `TokenBazaarBoard` (action-grid), `SecretDraftBoard` (pool items), `FloodWatchBoard` (district/family split), `EventFrontierBoard` (flat leaf dump — migrated by CARACTPRES-007). The full audit enumerates all 14 at implementation.
2. Spec authority: `specs/card-and-action-presentation-shared-surfaces.md` §6 D4 ("The audit is a deliverable, not optional"), §8 WB7, §9 exit criterion 4. The audit record lands in `apps/web/README.md`'s Shell Surface section (the catalog-shaped doc surface `scripts/check-catalog-docs.mjs` partially enforces; the audit table is process-enforced content beside the renderer list).
3. Cross-artifact boundary under audit: the `ActionTree` → control mapping per game. The audit's "board-native" criterion is concrete: every gameplay click maps to a Rust legal choice with no TS filtering or synthesis (`docs/UI-INTERACTION.md` §19); "adopt" applies where compound/multi-stage trees render as lists rather than staged choices.
4. FOUNDATIONS §7 restated: the audit enforces legal-only, progressive-construction doctrine catalog-wide; a "fallback" disposition is only valid where the game's tree is genuinely single-stage.

## Architecture Check

1. Audit-with-criteria beats force-migrating all panels: board-native affordances (clicking a card/column/site) are often better UX than any panel, and the spec explicitly rejects forced migration (§3.3); the recorded table makes the decision reviewable and future-binding per game.
2. No backwards-compatibility aliasing/shims: any migrated board loses its flat list in the same diff; no per-game toggles.
3. `engine-core`/`game-stdlib` untouched.

## Verification Layers

1. Audit completeness (14 rows, one disposition each, no game omitted) -> grep-proof against the `GAME_*` catalog in `crates/wasm-api/src/lib.rs` vs. the README table row count.
2. Residual flat-leaf-list absence after migrations -> codebase grep-proof: no remaining `collectLeaves`-style flatten-and-join rendering in any board component.
3. Migrated boards keep submission fidelity -> per-migrated-game e2e smoke additions mirroring CARACTPRES-007's byte-identical-path assertions.
4. If the audit mandates zero migrations: single-layer note — the deliverable is the recorded table; verification surfaces 1 and 2 still apply.

## What to Change

### 1. Audit

Classify each of the 14 catalog games' action presentation: adopt / board-native / fallback, with a one-line rationale citing the game's tree shape (single-stage vs. compound) and existing controls.

### 2. Record

Add the audit table to `apps/web/README.md` (Shell Surface section): game, disposition, rationale. Future games must add a row (spec D6; lift text lands via CARACTPRES-010).

### 3. Residual migrations

Implement every "adopt" row surfaced by the audit using `ActionPathBuilder` (per CARACTPRES-007's adoption pattern), including smoke additions per migrated game. Candidate set is implementation-discovered; known-likely candidates from session evidence: none confirmed beyond Event Frontier (already migrated in 007) — the audit decides.

## Files to Touch

- `apps/web/README.md` (modify)
- `apps/web/src/components/` board components (modify — audit-mandated set, implementation-discovered; parent verified; candidates named in the audit before edits)
- `apps/web/e2e/` per-game smoke files (modify — for migrated games only, as surfaced; parent verified)

## Out of Scope

- `ActionPathBuilder` feature work (variant gaps route to a 007 follow-up, not forked here).
- Copy hygiene of panel headings — CARACTPRES-009 (this ticket may neutralize headings only in boards it migrates anyway).
- Deck surfaces; Rust changes; the generic `ActionControls` fallback's internals.
- Re-litigating Event Frontier (done in 007).

## Acceptance Criteria

### Tests That Must Pass

1. README audit table has exactly one row per `GAME_*` catalog entry (count assertion against `crates/wasm-api/src/lib.rs`).
2. Grep-proof: no board component renders flattened leaf-path strings.
3. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:e2e` green after any migrations.

### Invariants

1. Every catalog game has an explicit, recorded action-presentation disposition (spec D4/D6).
2. Migrated games submit byte-identical action paths for identical selections (§11 determinism).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/<game>.smoke.mjs` — staged-construction + submission-fidelity assertions per migrated game (audit-discovered set).
2. `None beyond migrated games — audit rows are doc content; verification is the grep/count commands below.`

### Commands

1. `grep -c '^|' apps/web/README.md` region check vs. `grep -c 'GAME_' crates/wasm-api/src/lib.rs` (exact commands refined at implementation to the table's anchor).
2. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e`
3. Narrow boundary rationale: presentation + docs only; Rust untouched.

## Outcome

Completed on 2026-06-12.

- Added a 14-row action-presentation audit table to `apps/web/README.md`, one row per WASM catalog game.
- Recorded Event Frontier as the sole `ActionPathBuilder` adoption from this pass; all remaining custom boards are explicit `board-native` dispositions or the `race_to_n` generic fallback.
- Verified no residual Event Frontier-style flattened leaf-path renderer remains in web board components.
- No additional migrations were required by the audit.

Verification:

- `rg -N "^const GAME_[A-Z0-9_]+: &str =" crates/wasm-api/src/lib.rs | rg -v "DISPLAY_NAME" | wc -l` -> 14
- `sed -n '/### Action Presentation Audit/,/## Smoke Layers/p' apps/web/README.md | rg -N '^\| `.*` \|' | wc -l` -> 14
- `rg -n "collectLeaves|leaf\\.path|path\\.map\\(actionLabel\\)|function actionLabel" apps/web/src/components apps/web/src/main.tsx` -> no matches
- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:e2e`
