# RIVLEDSHOWUX-002: Fail `seat_N` in runtime visible/a11y River Ledger copy

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes (audit tooling + e2e, no Rust/engine) — `scripts/check-outcome-explanations.mjs`, `scripts/check-presentation-copy.mjs`, `apps/web/e2e/river-ledger.smoke.mjs`, `apps/web/e2e/a11y-noleak.smoke.mjs`
**Deps**: RIVLEDSHOWUX-001

## Problem

The static-copy guard (`scripts/check-presentation-copy.mjs:24`) matches `seat_[0-9]+` only in TS/static literals; the leak is born in Rust at runtime and reaches the DOM. Once RIVLEDSHOWUX-001 cleans the authored strings, this ticket adds the guards that keep them clean: an outcome-explanation audit rule plus a runtime e2e sweep that fail `\bseat_\d+\b` in visible text and accessibility labels on River Ledger public surfaces.

## Assumption Reassessment (2026-06-16)

1. Verified: `scripts/check-presentation-copy.mjs:24` already carries a `seat_[0-9]+` regex (static scan); `scripts/check-outcome-explanations.mjs` has no equivalent; `apps/web/e2e/{river-ledger,a11y-noleak}.smoke.mjs` exist and drive the real browser/WASM path.
2. Verified against spec §8 WB2 + §2 row #1; this ticket consumes the clean strings RIVLEDSHOWUX-001 produces (hence `Deps`).
3. Boundary under audit: the canonical check is the *runtime* DOM text + `aria-label` sweep, not the static source scan — the static scan structurally cannot see Rust-born strings.
4. FOUNDATIONS §11 (no raw internal id in visible text / accessibility labels) motivates this ticket; the guard is its enforcement surface.
5. New enforcement surface: the guard must go red when a `seat_N` is re-introduced into River Ledger terminal/visible copy (negative test) — a check that has never failed is unproven and may be vacuously passing.

## Architecture Check

1. Runtime DOM/a11y assertion is the correct enforcement layer (the static scan cannot catch Rust-born runtime strings); extending the existing e2e + audit scripts beats a new bespoke harness.
2. No shims; the existing static `seat_[0-9]+` rule is retained, not replaced.
3. `engine-core` untouched; no Rust behavior change — audit/e2e only.

## Verification Layers

1. Runtime visible/a11y text carries no `seat_\d+` -> `node apps/web/e2e/river-ledger.smoke.mjs` DOM + aria-label sweep.
2. Guard is non-vacuous -> negative check: temporarily re-introduce `seat_N`, confirm the audit/e2e step fails, then restore.
3. Outcome-explanation terminal strings clean -> `node scripts/check-outcome-explanations.mjs`.

## What to Change

### 1. `scripts/check-outcome-explanations.mjs`

Add a rule failing `\bseat_\d+\b` in River Ledger terminal/outcome explanation strings (visible + accessibility).

### 2. `apps/web/e2e/river-ledger.smoke.mjs` / `a11y-noleak.smoke.mjs`

Extend the DOM/no-leak sweep to assert no `seat_\d+` in rendered text content or `aria-label` / `data-testid` on River Ledger surfaces.

## Files to Touch

- `scripts/check-outcome-explanations.mjs` (modify)
- `scripts/check-presentation-copy.mjs` (modify, if a River-terminal pattern is added)
- `apps/web/e2e/river-ledger.smoke.mjs` (modify)
- `apps/web/e2e/a11y-noleak.smoke.mjs` (modify)

## Out of Scope

- The Rust authoring-layer fix that makes the strings clean (RIVLEDSHOWUX-001).
- Any change to other games' copy audits.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-outcome-explanations.mjs` — passes on the clean tree; the new River-terminal rule is present.
2. `node apps/web/e2e/river-ledger.smoke.mjs` + `node apps/web/e2e/a11y-noleak.smoke.mjs` — no `seat_\d+` in visible text or accessibility labels.
3. Negative: re-introducing a `seat_N` into River Ledger terminal copy makes (1) or (2) fail.

### Invariants

1. The runtime guard fails on any `seat_\d+` in River Ledger visible/a11y text (§11).
2. No Rust/engine behavior changes; this is audit + e2e only (§2).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/river-ledger.smoke.mjs` — DOM text + aria-label `seat_\d+` sweep.
2. `apps/web/e2e/a11y-noleak.smoke.mjs` — accessibility-label `seat_\d+` assertion.

### Commands

1. `node scripts/check-outcome-explanations.mjs`
2. `node apps/web/e2e/river-ledger.smoke.mjs`
3. The runtime e2e sweep is the correct boundary — the static scan in `check-presentation-copy.mjs` cannot see Rust-born strings.

## Outcome

Completed: 2026-06-16

Changed:

- Added a raw `seat_N` outcome-copy rule to
  `scripts/check-outcome-explanations.mjs` so static outcome templates cannot
  reintroduce raw seat ids in terminal copy.
- Extended `apps/web/e2e/river-ledger.smoke.mjs` with a River-Ledger-scoped
  runtime sweep over visible text, accessibility labels, and `data-testid`
  values, plus an induced `seat_5` probe proving the guard is non-vacuous.
- Extended `apps/web/e2e/a11y-noleak.smoke.mjs` with the same raw-seat runtime
  guard for visible text and accessibility labels.

Deviations:

- `scripts/check-presentation-copy.mjs` was left unchanged because it already
  scans static source literals for `seat_[0-9]+`; this ticket's missing coverage
  was runtime Rust-born copy and outcome-template copy.

Verification:

- `node scripts/check-outcome-explanations.mjs`
- `npm --prefix apps/web run build`
- `node apps/web/e2e/river-ledger.smoke.mjs`
- `node apps/web/e2e/a11y-noleak.smoke.mjs`
- Negative guard proof: both browser scripts now inject a visible/a11y
  `seat_5` probe and assert the raw-seat collector catches it before removing
  the probe.
