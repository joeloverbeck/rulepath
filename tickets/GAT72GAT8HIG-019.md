# GAT72GAT8HIG-019: e2e no-leak/a11y smoke + gate-1 CI registration

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/e2e/high-card-duel.smoke.mjs` (new), `.github/workflows/gate-1-game-smoke.yml`
**Deps**: GAT72GAT8HIG-018

## Problem

Gate 8's browser-surface hidden-information firewall must be proven by an e2e
smoke that drives the High Card Duel UI and asserts no hidden token reaches the
DOM/attributes/CSS/test-ids/console/storage/replay export across observer and
seat viewers, plus accessibility and reduced-motion behavior — and the smoke must
run in CI alongside the existing per-game smokes.

## Assumption Reassessment (2026-06-07)

1. Verified the e2e convention: smokes are node scripts under `apps/web/e2e/`
   (`draughts-lite.smoke.mjs`, `a11y-noleak.smoke.mjs`, `shell.smoke.mjs`) run
   directly with `node`, registered per-line in
   `.github/workflows/gate-1-game-smoke.yml:107-112`; the no-leak/a11y checklist
   is `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md`.
2. Verified against the spec: §4.2.10 fixes the required scenarios (Seat 0/Seat 1
   private hands, observer counts/backs, lead/reply hidden until reveal, reveal
   flow, bot turn no-leak, public-safe replay default, dev-panel no-leak, DOM/
   attr/CSS/test-id/console/storage no-leak, keyboard/focus, reduced-motion,
   responsive, a11y scan) and the no-leak denylist tokens (`hcd:r`, `deck_order`,
   `bot_candidate`, etc.), scoped to browser-visible surfaces, not source text.
3. Cross-artifact boundary under audit: the browser-visible no-leak firewall —
   the smoke inspects DOM/storage/console/export payloads, not repository source.
4. FOUNDATIONS principle under audit (§11 no-leak firewall): hidden information
   must not reach any browser-visible surface for any viewer.
5. Enforcement surface named: the §11 no-leak firewall at the browser. Confirm
   the denylist targets browser-visible DOM/storage/console/export (avoiding
   false positives from public-safe source labels), covers observer + seat
   viewers + dev-panel-open, and the a11y scan passes or records bounded justified
   exceptions per the existing checklist.

## Architecture Check

1. A dedicated game smoke mirroring the sibling `*.smoke.mjs` + the shared
   a11y-noleak harness is cleaner than ad-hoc manual checks and plugs into the
   existing CI lane.
2. No backwards-compatibility shims — additive smoke + CI step.
3. No engine/`game-stdlib` change; this is browser verification.

## Verification Layers

1. Viewer-scoped DOM no-leak -> no-leak visibility test: Seat 0/Seat 1/observer DOM/attributes/CSS/test-ids carry no hidden token.
2. Reveal flow -> no-leak test: lead/reply hidden until the reveal flow exposes both simultaneously; bot turn leaks no candidates.
3. Storage/console/export no-leak -> no-leak test: localStorage/sessionStorage/console and the default replay export are clean.
4. Accessibility + reduced-motion -> a11y scan: passes or records bounded justified exceptions; keyboard/focus + reduced-motion paths work.

## What to Change

### 1. `apps/web/e2e/high-card-duel.smoke.mjs`

Drive the UI through the §4.2.10 scenarios; assert the denylist tokens are absent
from browser-visible DOM/attributes/CSS/test-ids/console/storage/replay-export
for observer + each seat, including dev-panel-open; run keyboard/focus, reduced-
motion, and responsive checks; run the a11y scan.

### 2. CI registration

Add `node apps/web/e2e/high-card-duel.smoke.mjs` to
`.github/workflows/gate-1-game-smoke.yml` alongside the existing smokes.

## Files to Touch

- `apps/web/e2e/high-card-duel.smoke.mjs` (new)
- `.github/workflows/gate-1-game-smoke.yml` (modify)

## Out of Scope

- Rust/WASM behavior and the board component (proven in 016/018).
- Native no-leak suite (GAT72GAT8HIG-011 — this is the browser counterpart).

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/e2e/high-card-duel.smoke.mjs` — passes (after `npm --prefix apps/web run build`).
2. `node apps/web/e2e/a11y-noleak.smoke.mjs` — passes.

### Invariants

1. No hidden token reaches any browser-visible surface for any viewer (§11 no-leak firewall).
2. The denylist targets browser payloads, not repository source text (no false positives).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/high-card-duel.smoke.mjs` — the browser no-leak/a11y/reduced-motion smoke.

### Commands

1. `npm --prefix apps/web run build && node apps/web/e2e/high-card-duel.smoke.mjs`
2. `node apps/web/e2e/a11y-noleak.smoke.mjs`
3. The browser smokes are the correct boundary — they exercise rendered surfaces the native suite (011) cannot reach.
