# ACTCONMAT-010: Runtime raw-identifier DOM guard + negative test

**Status**: DONE (2026-06-12)
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — test infrastructure only (`apps/web/e2e/a11y-noleak.smoke.mjs`); `scripts/check-presentation-copy.mjs` gets a scope note.
**Deps**: ACTCONMAT-001, ACTCONMAT-005, ACTCONMAT-008, ACTCONMAT-009

## Problem

The presentation-copy guard (`scripts/check-presentation-copy.mjs`) scans TypeScript component *source* and cannot see Rust-supplied runtime strings, so raw internal IDs in player-facing labels (e.g. "Survey site_charterhouse,site_crossing") passed CI despite violating `docs/UI-INTERACTION.md` §19. A runtime DOM sweep must fail on raw snake_case identifiers in normal-mode visible text and accessibility labels, closing the hole the source-scan guard cannot see — with a negative test proving the guard trips on induced drift.

## Assumption Reassessment (2026-06-12)

1. `scripts/check-presentation-copy.mjs` scans component source only (regex over `.tsx` literals for `seat_0`, `site_xyz`, debug vocabulary); it never inspects runtime DOM. The smoke layer `apps/web/e2e/a11y-noleak.smoke.mjs` is the existing no-leak/accessibility DOM sweep — the right home for a runtime identifier assertion. This guard only becomes meaningful once labels/faction/rules/variant copy are resolved (ACTCONMAT-001/005/008/009), hence the Deps.
2. Spec D3 / §4.2: the smoke-level DOM sweep gains a normal-mode raw-identifier assertion (visible text + aria attributes must not match `[a-z0-9]+_[a-z0-9_]+` token patterns, allowlisting dev panels), with a negative test proving the guard trips on induced drift.
3. Cross-artifact boundary under audit: the runtime DOM (rendered visible text + ARIA) vs. the source-scan guard. The two are complementary; this ticket documents the source guard's scope and adds the runtime assertion in the smoke.
4. FOUNDATIONS §12 (stop conditions): the runtime guard *reduces* §12 exposure by catching raw-identifier regressions that reach the DOM — it strengthens the no-debug-vocabulary / no-raw-identifier invariant, never weakens it.
5. No-leak / identifier enforcement surface: the assertion runs over normal-mode DOM only (dev panels allowlisted), so it does not falsely flag the dev inspector (which legitimately shows seat IDs); the negative test confirms an induced `site_`/`seat_` string in normal-mode DOM fails the sweep.

## Architecture Check

1. Adding the assertion to the runtime DOM smoke (not the source guard) closes the exact hole — Rust-supplied strings only exist at runtime. A negative test makes the guard self-verifying, so a future regression in the guard itself is caught.
2. No shim: the source guard stays; the runtime sweep is an additive, stronger layer. The dev-panel allowlist is an explicit scope boundary, not a bypass.
3. No engine change; `engine-core` untouched. No `game-stdlib` change.

## Verification Layers

1. Runtime DOM has no raw snake_case identifiers in normal mode -> no-leak visibility test (`apps/web/e2e/a11y-noleak.smoke.mjs`) across the catalog.
2. The guard trips on induced drift -> negative test (inject a `site_`/`seat_` string; assert the sweep fails).
3. Dev panels are allowlisted, not falsely flagged -> UI smoke assertion.

## What to Change

### 1. Runtime identifier assertion

In `apps/web/e2e/a11y-noleak.smoke.mjs`, sweep normal-mode visible text and ARIA attributes for `[a-z0-9]+_[a-z0-9_]+` token patterns; fail on any match outside the allowlisted dev panels. Run catalog-wide.

### 2. Negative test

Add a negative-test path that induces a raw-identifier string into normal-mode DOM and asserts the sweep fails (guard self-verification).

### 3. Source-guard scope note

Add a comment in `scripts/check-presentation-copy.mjs` documenting that it scans source only and the runtime guard lives in the smoke sweep.

## Files to Touch

- `apps/web/e2e/a11y-noleak.smoke.mjs` (modify; runtime identifier assertion + negative test)
- `scripts/check-presentation-copy.mjs` (modify; scope note)

## Out of Scope

- Fixing any specific game's labels (done in ACTCONMAT-001/005/008/009) — this ticket is the guard that proves they stay fixed.
- The source-scan guard's existing checks — only a scope note is added.

## Acceptance Criteria

### Tests That Must Pass

1. `apps/web/e2e/a11y-noleak.smoke.mjs` passes catalog-wide with no raw snake_case identifier in normal-mode visible text or ARIA.
2. The negative test demonstrably fails when a raw identifier is induced into normal-mode DOM (recorded).
3. `npm --prefix apps/web run smoke:e2e` green.

### Invariants

1. The runtime sweep covers normal-mode DOM only; dev panels are allowlisted (§7 play-first, dev vocabulary quarantined).
2. The guard is self-verifying via the negative test (no silent pass on a broken guard).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/a11y-noleak.smoke.mjs` — runtime raw-identifier assertion + induced-drift negative test.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `node scripts/check-presentation-copy.mjs` (source guard still green)
3. `npm --prefix apps/web run smoke:ui`

## Completion Notes (2026-06-12)

Implemented the runtime raw-identifier DOM sweep in `apps/web/e2e/a11y-noleak.smoke.mjs`, covering normal-mode visible text and accessibility labels while excluding diagnostic/dev/replay/effect surfaces. Added an induced-drift negative check that injects a raw identifier into normal DOM and proves the guard catches it before the catalog sweep continues.

Documented the complementary source/runtime guard scope in `scripts/check-presentation-copy.mjs`. The new guard found existing public-copy leaks, so the ticket also normalized player-facing labels in Race to N, Directional Flip, and Flood Watch, adjusted Event Frontier waiting copy, and updated the Flood Watch smoke to assert the public seat label.

Verification:

1. `node scripts/check-presentation-copy.mjs` — passed (`presentation-copy check passed - 17 play-surface files scanned`).
2. `npm --prefix apps/web run smoke:ui` — passed.
3. `node apps/web/e2e/flood-watch.smoke.mjs` — passed after updating the public label assertion.
4. `npm --prefix apps/web run smoke:e2e` — passed.
