# GAT91SECDRACOM-016: secret_draft e2e no-leak/a11y smoke + gate-1 CI + web-catalog README reconciliation

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only + CI/docs) — `apps/web/e2e/secret-draft.smoke.mjs` (new); `apps/web/package.json`, `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md`, `.github/workflows/gate-1-game-smoke.yml`, `apps/web/README.md`, `README.md` (modify). No Rust/engine change.
**Deps**: GAT91SECDRACOM-012, GAT91SECDRACOM-013, GAT91SECDRACOM-015

## Problem

The gate needs a browser E2E smoke proving human commit, pending-seat UI, bot commit, synchronized reveal, conflict fallback, replay step/export/import, reduced motion, and no hidden item ID in DOM/storage/logs/test-ids before reveal — plus gate-1 CI registration of all `secret_draft` native + browser checks. It also reconciles the three mechanically-checked web-catalog README surfaces so `check-catalog-docs.mjs` passes, closing the gap that forced a separate Gate 9 aftermath pass (spec reassessment finding I2).

## Assumption Reassessment (2026-06-08)

1. The e2e precedent is `apps/web/e2e/token-bazaar.smoke.mjs` (+ `high-card-duel.smoke.mjs`); `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` and `a11y-noleak.smoke.mjs` exist. `apps/web/package.json` `smoke:e2e` currently chains shell, a11y-noleak, three-marks, column-four, draughts-lite, high-card-duel, token-bazaar (verified) — `secret-draft` must be appended.
2. `.github/workflows/gate-1-game-smoke.yml` registers token_bazaar via native arms — `simulate` (l.48–49), `replay-check` (l.70), `fixture-check` (l.91), `rule-coverage` (l.112) — plus the E2E node line (l.129–138) and `check-catalog-docs.mjs` (l.144). `secret_draft` arms must be added to each, and the e2e step list extended.
3. `scripts/check-catalog-docs.mjs` mechanically checks three surfaces against the wasm-api `GAME_*` source of truth (verified from its header): (1) `apps/web/README.md` intro catalog list, (2) root `README.md` "current official games are" list, (3) `apps/web/README.md` Smoke Layers `smoke:e2e` bullet. The Shell Surface renderer bullet is process-enforced, not mechanically checked. Since GAT91SECDRACOM-013 added `GAME_SECRET_DRAFT`, these three README surfaces are now due — this ticket lands them so `check-catalog-docs` is green.
4. §11 no-leak + §7 public-UI are the motivating principles: restate before trusting spec — the e2e must assert no committed item ID appears in DOM text/attributes/`data-testid`/local+session storage/console/effect log before reveal (A6 includes the committing seat), and reveal renders as a grouped batch with reduced motion preserved.
5. Cross-artifact ordering: `check-catalog-docs` runs on every push, so between GAT91SECDRACOM-013 (catalog const) and this ticket the check is red — expected mid-gate, resolved here. This ticket is the "reconcile at the gate, not in an aftermath" closeout the spec's §Documentation-updates now mandates.
6. Schema/consumer note: `gate-1-game-smoke.yml` and `package.json smoke:e2e` are extended additively (new game arms/steps); no shared-schema break.

## Architecture Check

1. Folding the catalog README reconciliation into the ticket that adds the e2e smoke + gate-1 CI (the surfaces `check-catalog-docs` keys on) is cleaner than a trailing aftermath pass — the smoke:e2e bullet, the e2e file, and the README entries land atomically and CI goes green in one step.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched; e2e/CI/docs only. No `game-stdlib` change; TypeScript remains presentation-only.

## Verification Layers

1. Browser no-leak/a11y -> `node apps/web/e2e/secret-draft.smoke.mjs`: no pre-reveal committed ID in DOM/attributes/`data-testid`/storage/logs; a11y checks pass.
2. Full e2e suite -> `npm --prefix apps/web run smoke:e2e` (with secret-draft appended) passes.
3. Catalog-docs drift -> `node scripts/check-catalog-docs.mjs` passes (three README surfaces name `secret_draft`/`Veiled Draft`).
4. gate-1 CI -> workflow runs `secret_draft` native arms + e2e + check-catalog-docs (grep-proof in the yml).

## What to Change

### 1. `apps/web/e2e/secret-draft.smoke.mjs`

Puppeteer smoke covering human commit, waiting/pending state, bot commit, synchronized reveal, conflict fallback, replay step/export/import, reduced motion, and the DOM/storage/log/test-id no-leak negative assertions.

### 2. `apps/web/package.json`

Append `node e2e/secret-draft.smoke.mjs` to the `smoke:e2e` script.

### 3. `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md`

Add the `secret_draft` no-leak/a11y checklist rows.

### 4. `.github/workflows/gate-1-game-smoke.yml`

Add `secret_draft` arms to the simulate / replay-check / fixture-check / rule-coverage steps and append `node apps/web/e2e/secret-draft.smoke.mjs` to the E2E step (and its step title).

### 5. Web-catalog README reconciliation

- `apps/web/README.md`: add `secret_draft` / `Veiled Draft` to the intro catalog list and the Smoke Layers `smoke:e2e` bullet; add the board to the Shell Surface renderer list (process-enforced).
- `README.md`: add `Veiled Draft` to the "current official games are" list.

## Files to Touch

- `apps/web/e2e/secret-draft.smoke.mjs` (new)
- `apps/web/package.json` (modify)
- `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` (modify)
- `.github/workflows/gate-1-game-smoke.yml` (modify)
- `apps/web/README.md` (modify)
- `README.md` (modify)

## Out of Scope

- The `specs/README.md` index `Done` flip, MECHANIC-ATLAS first-use, `progress.md`, and final acceptance evidence (GAT91SECDRACOM-018 capstone).
- gate-2 benchmark CI (GAT91SECDRACOM-011).
- Renderer/binding code (GAT91SECDRACOM-014/015).

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/e2e/secret-draft.smoke.mjs` passes (incl. pre-reveal no-leak negative assertions).
2. `npm --prefix apps/web run smoke:e2e` passes with secret-draft appended.
3. `node scripts/check-catalog-docs.mjs` and `node scripts/check-doc-links.mjs` pass; gate-1 workflow includes all `secret_draft` arms.

### Invariants

1. No committed item ID in DOM/attributes/`data-testid`/storage/logs before reveal, including the committing seat (§11 no-leak, A6).
2. The three mechanically-checked catalog surfaces name the game — no catalog-docs drift left to an aftermath pass (spec I2).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/secret-draft.smoke.mjs` — human/bot commit, pending UI, reveal batch, conflict fallback, replay, reduced motion, no-leak.

### Commands

1. `node apps/web/e2e/secret-draft.smoke.mjs`
2. `npm --prefix apps/web run smoke:e2e && node scripts/check-catalog-docs.mjs && node scripts/check-doc-links.mjs`
3. The browser e2e + catalog-docs checks are the correct boundary for web registration; the gate-wide acceptance roll-up is GAT91SECDRACOM-018.
