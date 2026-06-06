# GAT3WASMSTAWEB-014: Accessibility, reduced-motion, and no-leak review + smoke

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — accessibility checks + a hidden-info no-leak review over the rendered shell (`apps/web`); no Rust/crate change.
**Deps**: 010, 013

## Problem

Gate 3 requires a practical accessibility baseline and a hidden-information no-leak
review across all browser surfaces, even though `race_to_n` is perfect-information
(spec §17, §19.5, §19.6; FOUNDATIONS §11 no-leak firewall). These are core shell
requirements, not optional polish (§24.5). This trailing ticket adds the
accessibility smoke and the no-leak checklist/negative checks over the full shell
built by the prior tickets, and applies any small a11y fixes they surface.

## Assumption Reassessment (2026-06-06)

1. The shell, modes, replay UI, and dev panel exist after GAT3WASMSTAWEB-005–010,
   with the rendered-browser harness from GAT3WASMSTAWEB-013 available to drive
   accessibility/no-leak assertions over served `dist`. Components use `data-testid`
   hooks and (per -005/-006/-007) landmarks, headings, labeled controls, accessible
   names, non-color cues, and reduced-motion handling — this ticket audits and
   hardens them as a whole rather than per-region.
2. Spec §17 (landmarks/structure, keyboard behavior for all Gate-3 flows, focus
   management, accessible names + screen-reader summaries, reduced motion, responsive
   layout, color/contrast not color-only); §19.5 (accessibility smoke: accessible
   names, keyboard reachable, focus visible, reduced-motion path, no color-only,
   keyboard-accessible dev/replay controls; axe-style scan optional); §19.6 (no-leak
   review across browser payloads, action tree, diagnostics, effect log, DOM attrs,
   test IDs, console logs, local/session storage, replay exports, bot explanations,
   candidate rankings, dev inspector).
3. Cross-artifact boundary under audit: the no-leak firewall spans every browser
   surface the prior tickets produced (DOM, test IDs, storage, console, replay
   export, dev panel). The review confirms no surface leaks state a viewer's
   deterministic view forbids; for `race_to_n` all facts are public, so the review
   establishes the *pattern* (whitelist-only dev fields, command/effect-only export)
   safe for later hidden-information games.
4. FOUNDATIONS §11 (no-leak; accessible play-first UI): restated before trusting the
   spec — hidden information must not reach payloads, DOM, storage, logs, previews,
   effect logs, bot explanations, candidate rankings, UI test IDs, or replay exports;
   the UI must be keyboard-operable with visible focus and no color-only cues.
5. §11 enforcement substrate (no-leak visibility firewall): name the surface — the
   set of browser outputs in §19.6. This ticket's negative checks are the Gate-3
   enforcement of that firewall for the perfect-information case; the deferred
   stronger enforcement is the hidden-information game gates (ROADMAP Gate 8+), which
   reuse this checklist. Confirm no Gate-3 surface introduces a leakage path those
   gates would have to undo (e.g. raw internal state in `data-testid`, full-state in
   local storage, internal fields in replay export).

## Architecture Check

1. A single trailing accessibility + no-leak pass (over the composed shell) with
   automatable negative checks is cleaner than scattering ad-hoc a11y assertions per
   region: it audits the whole surface set once and codifies the no-leak whitelist as
   reusable checks for later gates.
2. No backwards-compatibility shims: a11y fixes are applied in place; no parallel
   accessible/inaccessible paths.
3. `engine-core` untouched; review + small UI fixes only; `game-stdlib` untouched.

## Verification Layers

1. Keyboard-only critical path → simulation/manual review: the full start→play→bot→
   dev→replay flow is completable by keyboard with visible focus (extends the -013
   harness).
2. Accessible names + non-color cues → manual review + assertion: interactive
   controls have accessible names; state/seat/winner/error use a non-color cue.
3. Reduced motion → simulation: with reduced motion enabled/emulated, all
   information is preserved without animation.
4. No-leak firewall → no-leak visibility test: negative checks assert no hidden/
   internal/private fields appear in DOM attributes, `data-testid`s, console logs,
   local/session storage, replay exports, or the dev panel (§19.6 surface list).

## What to Change

### 1. New `apps/web/e2e/a11y-noleak.smoke.mjs` (or extend the -013 harness)

Accessibility smoke (keyboard path, focus-visible, accessible names, reduced-motion,
no color-only) + no-leak negative checks over each §19.6 surface (DOM/test-ids/
console/storage/replay-export/dev-panel). Optionally an axe-style scan if tooling
churn is acceptable (not mandatory, §19.5).

### 2. New `apps/web/docs` or checklist entry — no-leak/a11y review checklist

A repository checklist enumerating the §19.6 surfaces and §17 a11y baseline as a
re-runnable review record (content/doc home finalized in GAT3WASMSTAWEB-015).

### 3. Small a11y fixes across components (as surfaced)

Apply focus-management, accessible-name, label, contrast, or non-color-cue fixes the
smoke reveals in the existing region/renderer/panel components.

## Files to Touch

- `apps/web/e2e/a11y-noleak.smoke.mjs` (new) — accessibility + no-leak negative checks
- `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` (new) — re-runnable review record
- `apps/web/src/components/*.tsx` (modify) — small a11y fixes surfaced by the smoke

## Out of Scope

- Comprehensive WCAG audit beyond the Gate-3 baseline (deferred, §22).
- Full hidden-information renderer proof (deferred beyond no-leak design review, §22).
- Repo doc/index/status updates — GAT3WASMSTAWEB-015.

## Acceptance Criteria

### Tests That Must Pass

1. `cd apps/web && npm run smoke:e2e` (incl. the a11y/no-leak smoke) — keyboard path, focus-visible, accessible names, reduced-motion, and no-color-only checks pass.
2. No-leak negative checks pass: no hidden/internal/private field appears in DOM attributes, `data-testid`s, console logs, local/session storage, replay exports, or the dev panel.
3. `cd apps/web && npm run build` — typecheck + build remain green.

### Invariants

1. The Gate-3 critical flow is keyboard-operable with visible focus and no color-only state cues.
2. No browser surface leaks state a viewer's deterministic view forbids; the no-leak whitelist pattern is safe for later hidden-information games.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/a11y-noleak.smoke.mjs` (new) — a11y + no-leak negative checks; rationale: codifies the §17/§19.6 baseline as re-runnable enforcement reused by later hidden-info gates.
2. `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` (new) — manual review record for the parts not automatable.

### Commands

1. `cd apps/web && npm run smoke:e2e`
2. `cd apps/web && npm run build`
3. The no-leak negative test is the correct boundary: it asserts the *absence* of forbidden data across browser surfaces, which a positive functional smoke cannot prove.

## Outcome

Completed on 2026-06-06.

Changes:

- Added `a11y-noleak.smoke.mjs` and wired `smoke:e2e` to run it after the rendered-shell smoke.
- Added the Gate 3 no-leak/accessibility checklist under `apps/web/e2e`.
- Added visible focus styles for standard controls and custom mode radio labels.

Deviations:

- The no-leak storage check allows only the existing `rulepath.reducedMotion` UI preference with `reduce` or `motion`; it rejects other local/session storage.
- The replay export check whitelists `expected_private_view_hashes.not_applicable` as an explicit schema marker for this perfect-information game, while still rejecting private/hidden payload vocabulary.

Verification:

- `npm --prefix apps/web run smoke:e2e`
- `npm --prefix apps/web run build`
