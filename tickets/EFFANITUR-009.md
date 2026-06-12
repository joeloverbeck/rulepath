# EFFANITUR-009: Animation smoke suite + smoke:e2e wiring

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — TypeScript/React presentation shell + test harness only (`apps/web`); Rust/WASM untouched. Adds browser-smoke test infrastructure (the gate's verification harness is itself this deliverable).
**Deps**: EFFANITUR-004, EFFANITUR-005, EFFANITUR-006, EFFANITUR-007, EFFANITUR-008

## Problem

The scheduler, orchestration, adopters, and catalog sweep need an end-to-end smoke proving the full behavior contract — animate-and-settle, skip mid-burst, act-mid-animation (input not blocked), replay-step interrupt, reduced-motion equivalence — and the existing game smokes must be updated for auto-advancing bot turns. Critically, `smoke:e2e` is a hand-maintained explicit chain in `apps/web/package.json`, so a new smoke file does not run until it is wired in (reassess finding M1) (spec WB9). This ticket is the gate's distributed-acceptance verification harness.

## Assumption Reassessment (2026-06-12)

1. `apps/web/package.json` `smoke:e2e` is a hand-maintained 16-command chain (`node e2e/shell.smoke.mjs && … && node e2e/event-frontier.smoke.mjs`) — confirmed; a new `e2e/animation.smoke.mjs` runs only once appended. `e2e/` holds the existing game smokes; the puppeteer dev-dependency is already present. The unit smokes from EFFANITUR-001/002/003/008 (`smoke-bursts.mjs`, `smoke-scheduler.mjs`, `smoke-presenters.mjs`, `smoke-catalog-sweep.mjs`) currently run only via direct `node`.
2. Spec WB9 + reassess M1: add `e2e/animation.smoke.mjs`; wire it into the `smoke:e2e` chain; consolidate the unit smokes under an npm script; update existing game smokes for auto-advancing bots. This is the deliverable-doubles-as-capstone for the gate's exit criteria (animate-and-settle, skip, input-not-blocked, replay-interrupt, reduced-motion).
3. Cross-artifact boundary under audit: this ticket exercises the surfaces produced by EFFANITUR-001–008 end-to-end without modifying their logic; `apps/web/package.json` is single-owner here (no other ticket edits it). `event-frontier.smoke.mjs` / `flood-watch.smoke.mjs` are shared with EFFANITUR-006/007 (this ticket `Deps` both, so their adopter assertions already exist).
4. FOUNDATIONS §11: the smoke proves reduced-motion equivalence (every animated fact present as text and play not blocked) and that acting mid-animation flushes-then-submits — the §11 "semantic effects drive animation" and play-first invariants in test form.
5. No-leak (FOUNDATIONS §11 firewall): the suite re-runs the no-leak sweeps (DOM / a11y / test-ID / storage / log) and asserts they pass unchanged-or-stronger — animation adds no leak surface; redacted effects animate generically.

## Architecture Check

1. One dedicated `animation.smoke.mjs` plus targeted updates to existing game smokes — wired into the single hand-maintained `smoke:e2e` chain — is the established repo verification shape (puppeteer e2e), cleaner than a parallel test runner, and closes the M1 gap that would otherwise leave the new smoke unrun.
2. No backwards-compatibility shim: existing game smokes are updated for auto-advance, never duplicated or branched on a legacy manual-trigger path.
3. `engine-core` untouched; all test infra is `apps/web`-local (§3). No `game-stdlib` promotion.

## Verification Layers

1. animate-and-settle, skip mid-burst, act-mid-animation (input not blocked + flush correctness), replay-step interrupt -> `e2e/animation.smoke.mjs` (the new smoke).
2. reduced-motion equivalence (all facts present as text; play not blocked) -> `animation.smoke.mjs` reduced-motion assertion.
3. `animation.smoke.mjs` actually runs in CI -> `smoke:e2e` chain grep-proof (the file appears in the `package.json` command) + a green `smoke:e2e` run.
4. existing game smokes pass under auto-advancing bots -> full `smoke:e2e` run.
5. no-leak unchanged-or-stronger -> no-leak visibility test (`a11y-noleak.smoke.mjs` in the chain).

## What to Change

### 1. Animation smoke

Add `apps/web/e2e/animation.smoke.mjs`: animate-and-settle, skip mid-burst, act-mid-animation (input not blocked, flush correctness), replay-step interrupt, and reduced-motion equivalence (every fact present as text, play not blocked).

### 2. Wire smoke:e2e + consolidate unit smokes

In `apps/web/package.json`: append `node e2e/animation.smoke.mjs` to the `smoke:e2e` chain (M1); add a `smoke:animation` script running the node unit smokes (`smoke-bursts`, `smoke-scheduler`, `smoke-presenters`, `smoke-catalog-sweep`) so the scheduler/burst/presenter/sweep tests run in CI.

### 3. Update existing game smokes

Update the existing `e2e/*.smoke.mjs` game smokes for auto-advancing bot turns (no manual "Run Bot Turn" click; bot turns auto-advance), without weakening any existing assertion.

## Files to Touch

- `apps/web/e2e/animation.smoke.mjs` (new)
- `apps/web/package.json` (modify; `smoke:e2e` wiring + `smoke:animation` script)
- `apps/web/e2e/*.smoke.mjs` (modify; existing game smokes updated for auto-advance — the human_vs_bot-involving smokes in the `smoke:e2e` chain; `event-frontier.smoke.mjs` / `flood-watch.smoke.mjs` shared with EFFANITUR-006/007)

## Out of Scope

- Any production scheduler/orchestration/adopter logic (EFFANITUR-001–008) — this ticket exercises, it does not modify.
- The §10/§10A/§19 doc amendments and `specs/README.md`/brainstorm status flips (EFFANITUR-010).
- Removing or weakening any existing smoke assertion (AGENT-DISCIPLINE §4).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e` green, including `e2e/animation.smoke.mjs` (animate-and-settle, skip, input-not-blocked, replay-interrupt, reduced-motion equivalence) and all existing game smokes under auto-advance.
2. `npm --prefix apps/web run smoke:animation` green (scheduler/burst/presenter/catalog-sweep node smokes).
3. Grep-proof: `e2e/animation.smoke.mjs` appears in the `package.json` `smoke:e2e` chain.

### Invariants

1. The new smoke actually runs in CI (wired into `smoke:e2e`); no created-but-unrun test (M1 closed).
2. No-leak sweeps pass unchanged-or-stronger; reduced-motion preserves every fact and play (§11).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/animation.smoke.mjs` — the full animation behavior contract.
2. `apps/web/package.json` — `smoke:e2e` wiring + `smoke:animation` script.
3. `apps/web/e2e/*.smoke.mjs` — existing game smokes updated for auto-advance.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `npm --prefix apps/web run smoke:animation`
3. `npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:effects` (full web smoke set, regression)
