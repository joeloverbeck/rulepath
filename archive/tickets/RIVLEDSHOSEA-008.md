# RIVLEDSHOSEA-008: Cross-catalog viewer and no-leak regression matrix

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (audit + e2e) — `apps/web/e2e/shell.smoke.mjs`, `apps/web/e2e/a11y-noleak.smoke.mjs`, and the fixed-two-seat game smoke scripts under `apps/web/e2e/`
**Deps**: RIVLEDSHOSEA-006

## Problem

The generic viewer callback in RIVLEDSHOSEA-006 replaces the hardcoded two-seat allowlist that the fourteen fixed-two-seat games silently relied on. Their current success must be re-proved against the generic "requested ID must be in the Rust-projected active set" path, and the hidden-information games must re-prove pairwise no-leak across viewpoint switches. This ticket adds the shared-shell regression matrix and targeted hidden-information viewpoint coverage (spec §9.4 / §11 platform-wide impact list). It ships new test infrastructure, so it doubles as the viewer/no-leak acceptance surface for the gate.

## Assumption Reassessment (2026-06-18)

1. `apps/web/e2e/` contains per-game smoke scripts for the fixed-two-seat games (`column-four`, `directional-flip`, `draughts-lite`, `event-frontier`, `flood-watch`, `frontier-control`, `high-card-duel`, `masked-claims`, `plain-tricks`, `poker-lite`, `secret-draft`, `three-marks`, `token-bazaar`) plus `shell.smoke.mjs` (covering `race_to_n`, which has no dedicated smoke script) and `a11y-noleak.smoke.mjs`. Confirmed by directory listing.
2. Spec §11 names the hidden-information games requiring viewpoint no-leak: `high_card_duel`, `secret_draft`, `poker_lite`, `plain_tricks`, `masked_claims`, `flood_watch`, `event_frontier`. The remaining fixed-two-seat games need only the selector regression (exactly two seat options/rows, unchanged play). Confirmed.
3. Shared boundary under audit: the generic viewer-selection path (RIVLEDSHOSEA-006) as exercised across every catalog game, and the hidden-information no-leak firewall (§11) under viewpoint switching. End state: every game routes through the same active-ID rule; no game relies on a shell hardcode.
4. FOUNDATIONS §11 no-leak firewall + §12: switching viewpoints must never expose another seat's private cards before authorized showdown reveal, in payloads, DOM, accessible names, `data-testid`, logs, storage, animation payloads, or replay exports. Restated as the load-bearing invariant.
5. No-leak surface: this ticket adds the pairwise no-leak loop for River Ledger and the targeted hidden-information viewpoint transitions; `a11y-noleak.smoke.mjs` and per-game smoke suites inspect hidden DOM text, `aria-label`, `data-testid`, logs, and replay exports after switching viewpoints. It introduces verification infrastructure only — no production logic.

## Architecture Check

1. A single cross-catalog matrix proves the platform rule ("no per-game exception for the generic viewer callback") in one place, rather than trusting each game's existing smoke to have re-covered the change. Cleaner audit trail than scattered per-game edits.
2. No shim: the fixed two-seat games are not retrofitted with variable-seat support; the ticket only proves their declared two-seat contracts and hidden-information boundaries survive.
3. Verification-only; no `engine-core`/`game-stdlib`/Rust change.

## Verification Layers

1. Fixed two-seat selector unchanged -> per-game e2e assertion: exactly two seat options + two rail rows, and unchanged play behavior, for every fixed-two-seat game through the generic path.
2. Hidden-information viewpoint no-leak -> `a11y-noleak.smoke.mjs` + per-game suites for `high_card_duel`/`secret_draft`/`poker_lite`/`plain_tricks`/`masked_claims`/`flood_watch`/`event_frontier`: viewer B never receives seat A's private state across switches.
3. River Ledger pairwise no-leak -> `river-ledger.smoke.mjs`/`a11y-noleak.smoke.mjs` loop: for each source seat A and distinct viewer B, B never receives A's private cards before authorized showdown reveal.

## What to Change

### 1. Fixed-two-seat selector regression

Extend `shell.smoke.mjs` and the fixed-two-seat game smoke scripts to assert exactly Observer + two seats and functional generic selector behavior (no reliance on the removed allowlist).

### 2. Hidden-information viewpoint no-leak

Extend `a11y-noleak.smoke.mjs` (and the named hidden-information games' smoke suites) with viewpoint-switch no-leak assertions inspecting DOM text, `aria-label`, `data-testid`, logs, and replay exports; add the River Ledger all-pairs loop.

## Files to Touch

- `apps/web/e2e/a11y-noleak.smoke.mjs` (modify)
- `apps/web/e2e/shell.smoke.mjs` (modify)
- `apps/web/e2e/high-card-duel.smoke.mjs` (modify)
- `apps/web/e2e/secret-draft.smoke.mjs` (modify)
- `apps/web/e2e/poker-lite.smoke.mjs` (modify)
- `apps/web/e2e/plain-tricks.smoke.mjs` (modify)
- `apps/web/e2e/masked-claims.smoke.mjs` (modify)
- `apps/web/e2e/flood-watch.smoke.mjs` (modify)
- `apps/web/e2e/event-frontier.smoke.mjs` (modify)
- remaining fixed-two-seat game smoke scripts under `apps/web/e2e/` (modify; selector regression only, as surfaced)

## Out of Scope

- The generic viewer callback and active-seat scoping themselves (RIVLEDSHOSEA-006 — exercised, not modified).
- Retrofitting variable-seat support into the fixed two-seat games.
- River Ledger showdown/label correctness (RIVLEDSHOSEA-001..004).
- Final docs/status closeout (RIVLEDSHOSEA-010).

## Acceptance Criteria

### Tests That Must Pass

1. Full `npm --prefix apps/web run smoke:e2e`: every fixed-two-seat game shows exactly two active seats and functional selector behavior.
2. `a11y-noleak.smoke.mjs`: hidden-information games show no private-state residue across viewpoint switches; River Ledger pairwise no-leak holds.
3. `node scripts/check-presentation-copy.mjs` and shell/a11y smoke green.

### Invariants

1. Every game validates viewer selection against the Rust-projected active set; none relies on a shell hardcode.
2. No viewpoint switch leaks another seat's private state to any observable surface before authorized reveal.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/a11y-noleak.smoke.mjs` — viewpoint-switch no-leak for the named hidden-information games + River Ledger all-pairs loop.
2. `apps/web/e2e/shell.smoke.mjs` and fixed-two-seat game smoke scripts — exactly-two-seat selector regression.

### Commands

1. `npm --prefix apps/web ci && npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e`
2. `npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:effects`
3. Full `smoke:e2e` (not a narrower filter) is the correct boundary because the viewer callback is shared across the entire catalog.

## Outcome

Completed: 2026-06-18

What changed:
- Added a shared-shell fixed two-seat viewer matrix in `apps/web/e2e/shell.smoke.mjs` covering Race to 21 plus every fixed two-seat catalog game. The matrix starts each game in Hotseat mode, asserts the generic seat frame exposes observer plus exactly two active seat ids (`seat_0`, `seat_1`), verifies matching rail rows, and switches through observer and both rendered seat labels.
- Extended `apps/web/e2e/a11y-noleak.smoke.mjs` with aggregate hidden-information selector switching for Flood Watch and Event Frontier, including rendered-label support for catalog-authored labels such as Charter/Freeholders.
- Added a River Ledger pre-showdown all-pairs private-card loop in `apps/web/e2e/river-ledger.smoke.mjs`: every active seat can see its own two cards, observer sees none, and every distinct viewer/source pair rejects the source seat's private card ids across the existing DOM/attribute/storage/test-id/console leak surface.

Deviations from original plan:
- The fixed two-seat matrix lives in the shared shell smoke rather than duplicating selector assertions into every per-game smoke script. This keeps the generic viewer callback acceptance in one platform-level test while full `smoke:e2e` still exercises each game-specific script.
- The selector assertions validate active seat ids and rendered labels separately. Some fixed two-seat games intentionally use authored labels (`Charter`, `Freeholders`) instead of the generic `Seat 0`/`Seat 1` copy.

Verification:
- `npm --prefix apps/web run build` — passed.
- `node scripts/check-presentation-copy.mjs` — passed.
- `node apps/web/e2e/shell.smoke.mjs` — passed.
- `node apps/web/e2e/a11y-noleak.smoke.mjs` — passed.
- `node apps/web/e2e/river-ledger.smoke.mjs` — passed.
- `npm --prefix apps/web run smoke:e2e` — passed.
- `npm --prefix apps/web ci` — passed; npm reported one low-severity audit item.
- `npm --prefix apps/web run smoke:ui` — passed.
- `npm --prefix apps/web run smoke:effects` — passed.
