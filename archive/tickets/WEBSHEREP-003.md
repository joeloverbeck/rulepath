# WEBSHEREP-003: Full-length export→import round-trip e2e smoke

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (e2e) — `apps/web/e2e/starbridge-crossing.smoke.mjs`
**Deps**: WEBSHEREP-001, WEBSHEREP-002

## Problem

The shipped Starbridge e2e smoke (`apps/web/e2e/starbridge-crossing.smoke.mjs:81-90`) already drives an Export→Import round-trip, but only on a **short 2-seat** match whose export fits under the old 128 KiB cap — which is why the latent defect went unnoticed. There is no regression covering the shell's own **full-length, maximum-seat** export, the exact case that broke (spec §4 Deliverable 3, §6 exit criterion 1). This ticket adds that coverage and asserts the imported surface stays leak-free.

## Assumption Reassessment (2026-06-28)

1. `apps/web/e2e/starbridge-crossing.smoke.mjs` is a Puppeteer node smoke (imports `./launch.mjs`), wired into `apps/web/package.json` `smoke:e2e` (`"test"` = `smoke:e2e`). It already has helpers `startStarbridge(page, baseUrl, modeLabel, seed, seatCount)` (line 125), `clickText`, `replayTextareaValue`, `waitForText`, and an `assertNoForbiddenTerms(text, label, forbiddenTerms)` no-leak helper with `forbiddenTerms` (`hidden_state`, `candidate_ranking`, …). The existing round-trip block is lines 85-90 on a 2-seat seed-22 match.
2. Spec `specs/web-shell-replay-import-size-roundtrip.md` §4 (Deliverable 3) requires a full-length, maximum-seat round-trip (Starbridge 6-seat to the turn-limit terminal, or an equivalent comparable-size fixture) with a no-leak assertion on the imported surface, plus retention of the short-game regression (§6 exit criterion 2).
3. Shared boundary under audit: the shell's Export→Import UI path (`Export Current Run` / `Import Replay` buttons in `ReplayImportExport.tsx`) over the real WASM bridge. This smoke exercises WEBSHEREP-001 (Rust bound) and WEBSHEREP-002 (TS shadow removed) end-to-end; it adds no production logic.
4. FOUNDATIONS §11 no-leak firewall: the imported Replay viewer surface must carry no hidden-information terms. The smoke reuses `assertNoForbiddenTerms` on the imported replay text / rendered surface.

## Architecture Check

1. Extending the existing wired smoke is cleaner and lower-blast-radius than a new smoke file: no `package.json` `smoke:e2e` chain edit, and the export/import helpers already exist. It keeps the short-game round-trip as the regression while adding the full-length case.
2. No backwards-compatibility shim: this is additive test coverage.
3. No `engine-core` / `game-stdlib` involvement; presentation-layer verification only.

## Verification Layers

1. Full-length 6-seat export re-imports (no `replay_too_large`) -> e2e assertion: drive a 6-seat match to a full-length export (autoplay to terminal, or inject a comparable-size valid fixture > 128 KiB), click Export then Import, assert the Replay viewer loads (cursor/standings render) and no `replay_too_large` diagnostic appears.
2. Imported surface is leak-free -> `assertNoForbiddenTerms` over the imported replay text and rendered viewer surface.
3. Short-game round-trip regression intact -> existing 2-seat round-trip block (lines 85-90) retained and passing.

## What to Change

### 1. Add a full-length 6-seat round-trip case

In `apps/web/e2e/starbridge-crossing.smoke.mjs`, add a round-trip that obtains a full-length 6-seat Starbridge export exceeding the old 128 KiB bound — by autoplaying a 6-seat bot-vs-bot match to its deterministic terminal then clicking **Export Current Run**, or (per spec §4) by injecting a comparable-size valid Starbridge export fixture into the replay textarea. Click **Import Replay** and assert: the Replay viewer loads (e.g. `waitForText(page, "Replay viewer")` / cursor/standings visible) and no `replay_too_large` diagnostic is shown. Apply `assertNoForbiddenTerms` to the imported surface. Keep the existing 2-seat round-trip block as the short-game regression.

## Files to Touch

- `apps/web/e2e/starbridge-crossing.smoke.mjs` (modify)

## Out of Scope

- Production code changes (WEBSHEREP-001 Rust bound, WEBSHEREP-002 TS component).
- Docs and the spec Status flip (WEBSHEREP-004).
- New e2e harness/runner or `package.json` changes — the existing wired smoke is extended in place.

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/e2e/starbridge-crossing.smoke.mjs` (via `npm --prefix apps/web run smoke:e2e`) green, including the new full-length 6-seat round-trip.
2. The new case asserts no `replay_too_large` on the shell's own full-length export and a leak-free imported surface.
3. The retained 2-seat short-game round-trip still passes.

### Invariants

1. The shell can re-import any replay it exports for a legitimate full-length match.
2. No hidden-information term reaches the imported Replay viewer surface (§11 no-leak).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/starbridge-crossing.smoke.mjs` — add a full-length 6-seat Export→Import round-trip with a no-leak assertion; retain the short-game round-trip.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `npm --prefix apps/web run build` (the smoke runs against `dist/`)

## Outcome

Completed: 2026-06-28

Extended `apps/web/e2e/starbridge-crossing.smoke.mjs` with a full-length
6-seat Starbridge Crossing export/import case. The new case drives a 6-seat
bot-vs-bot match to the deterministic 2000-ply turn-limit terminal, exports the
current run, asserts the document exceeds the old 128 KiB cap, confirms the
6-seat setup is represented, imports the unedited document, waits for the
Replay viewer to load, asserts no `replay_too_large` diagnostic appears, and
runs the existing forbidden-term no-leak scan over the export, imported
surface, and console. The existing short 2-seat replay round-trip remains in
the same smoke.

Deviation from plan: the imported replay progress assertion checks that the
Replay viewer loads at `Cursor 0 /` rather than requiring the denominator text
to be exactly `2000`; the full-length property is proven by the exported
document size and `seat_5` setup assertion, while the imported surface load and
absence of `replay_too_large` prove the round-trip regression.

Verification:

- `npm --prefix apps/web run build` passed.
- `node apps/web/e2e/starbridge-crossing.smoke.mjs` passed.
- `npm --prefix apps/web run smoke:e2e` passed, including the updated
  Starbridge smoke and the full catalog e2e chain.
