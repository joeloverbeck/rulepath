# GAT3WASMSTAWEB-009: Replay viewer and safe local import/export UI

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — TypeScript/presentation only (`apps/web`); all replay validation/projection comes from the Rust replay ops (GAT3WASMSTAWEB-003).
**Deps**: 003, 004, 008

## Problem

Gate 3 requires a replay viewer and safe local replay import/export (spec §8.4,
§8.5, §15.4). The viewer steps/resets through Rust-authoritative projected state;
import passes data to Rust validation before it becomes app state and never treats
imported JSON as authoritative (§10.2, §15.1, §15.3, FOUNDATIONS §2/§11). No replay
UI exists today. This ticket builds the UI on top of the Rust replay ops from
GAT3WASMSTAWEB-003 and the bot-vs-bot runs from GAT3WASMSTAWEB-008.

## Assumption Reassessment (2026-06-06)

1. No replay UI exists in `apps/web/src`. The Rust replay ops (export/import/step/
   reset) and their typed client wrappers are delivered by GAT3WASMSTAWEB-003
   (`crates/wasm-api/src/lib.rs` + `apps/web/src/wasm/client.ts`); this ticket
   consumes those wrappers and adds reducer replay state
   (`apps/web/src/state/shellReducer.ts`, GAT3WASMSTAWEB-004). Bot-vs-bot runs that
   produce replays come from GAT3WASMSTAWEB-008.
2. Spec §8.4 (view a generated replay, step ≥1 boundary, reset to start,
   Rust-projected state/effects, safe metadata); §8.5 (export safe local format,
   import back, Rust validates before user-visible state, clear diagnostics for
   invalid/mismatched/oversized/wrong-game, no hidden data); §15.4 (show cursor/
   progress, current view + effect summaries). §11 transition `replayImported` enters
   replay state only after Rust validation.
3. Cross-artifact boundary under audit: the replay-document JSON shape produced/
   consumed by the Rust replay ops (anchored on `docs/TRACE-SCHEMA-v1.md` per
   GAT3WASMSTAWEB-003). The UI treats the document as opaque input to Rust — it does
   not parse it as authoritative state or re-apply commands itself.
4. FOUNDATIONS §2/§11 (replay authority; fail-closed; no-leak): restated — Rust
   replays and validates; TypeScript drives only the UI cursor and renders returned
   views/effects. Imported JSON is input to Rust validation, never TS-owned truth.
5. §11 enforcement substrate (deterministic replay + no-leak + fail-closed import):
   the fail-closed validator is the Rust `import_replay` op (GAT3WASMSTAWEB-003);
   this UI MUST surface its typed diagnostics without bypassing them, MUST NOT
   auto-store imports as authoritative state, and MUST NOT render imported payload
   as HTML/JS (§15.3) — so no leakage/injection path is introduced at the UI layer.
6. Schema/contract consumption: the UI consumes the replay-document shape from
   GAT3WASMSTAWEB-003 (additive new surface). It is a consumer only — it adds no new
   fields; if it needs a field absent from the doc, that is a -003 change, not a UI
   workaround.

## Architecture Check

1. A replay-viewer component driving a Rust-projected cursor + an import/export
   component that delegates all validation to Rust is cleaner than any TS-side
   replay engine: it keeps one replay authority and reuses the same projection path
   as live play. Reset+replay-through-Rust covers navigation without TS snapshots
   (§15.4).
2. No backwards-compatibility shims: there is no prior replay UI to alias.
3. `engine-core` untouched; the UI never replays by mutating state; no mechanic
   logic in React; `game-stdlib` untouched.

## Verification Layers

1. Replay is Rust-projected → simulation/CLI run: stepping/resetting shows views/
   effects returned by the Rust replay ops; no TS command re-application
   (grep-proof: no replay loop mutating game state in `apps/web/src`).
2. Import is fail-closed at the UI → no-leak/fail-closed test: wrong-game,
   bad-version, malformed, and oversized imports show the Rust diagnostic and do not
   enter replay state (`replayImported` only after validation success).
3. Export round-trips → deterministic replay check: export a bot-vs-bot run, import
   it, step through, and confirm the projected state matches (leans on -003's Rust
   round-trip test; UI exercises it end-to-end).
4. No-leak / no-injection → no-leak visibility test: imported payload is never
   rendered as HTML/JS and is not auto-persisted as authoritative (§15.3).

## What to Change

### 1. New `apps/web/src/components/ReplayViewer.tsx`

Show initial state, step forward ≥1 boundary, reset to start, current cursor/
progress, current Rust public view + effect summaries; reduced-motion-friendly.

### 2. New `apps/web/src/components/ReplayImportExport.tsx`

Export the current/last run to a downloadable/copyable document (from the Rust
export op); import a pasted/file payload with a size bound, delegating validation to
the Rust import op and surfacing typed diagnostics; never auto-store as authoritative.

### 3. `apps/web/src/state/shellReducer.ts` + `apps/web/src/main.tsx`

Replay session/document/cursor state + `replayImported`/step/reset transitions
(replay mode entered only after Rust validation); mount the viewer + import/export
in the (secondary) replay surface.

## Files to Touch

- `apps/web/src/components/ReplayViewer.tsx` (new)
- `apps/web/src/components/ReplayImportExport.tsx` (new)
- `apps/web/src/state/shellReducer.ts` (modify) — replay session/cursor state + transitions
- `apps/web/src/main.tsx` (modify) — mount replay surface

## Out of Scope

- Rust replay ops + replay document format — GAT3WASMSTAWEB-003.
- Backward step via snapshots (reset+replay-through-Rust suffices, §15.4).
- Dev-panel replay-metadata inspector — GAT3WASMSTAWEB-010.
- Browser E2E coverage of the replay round trip — GAT3WASMSTAWEB-013.

## Acceptance Criteria

### Tests That Must Pass

1. `cd apps/web && npm run build` — typecheck + Vite build succeed.
2. `cd apps/web && npm run smoke:ui` — a generated run exports, re-imports, and steps/resets through Rust-projected state.
3. `grep -rnE "dangerouslySetInnerHTML|localStorage\\.setItem\\(.*replay|JSON\\.parse\\(.*authoritative" apps/web/src/components/ReplayImportExport.tsx` — returns nothing (no HTML injection, no auto-store of imports as authoritative).

### Invariants

1. Rust validates/projects replays; TypeScript drives only the UI cursor and never re-applies commands.
2. Import enters replay state only after Rust validation; invalid/oversized/wrong-game inputs show safe diagnostics and are not stored as authoritative.

## Test Plan

### New/Modified Tests

1. `None — UI ticket over the Rust replay ops; verification is `smoke:ui` round-trip + `tsc`; the byte-level replay round-trip is proven by GAT3WASMSTAWEB-003's Rust test, and browser E2E by GAT3WASMSTAWEB-013.`

### Commands

1. `cd apps/web && npm run build`
2. `cd apps/web && npm run smoke:ui`
3. The deterministic replay-hash round trip is verified in Rust (GAT3WASMSTAWEB-003); this ticket's correct boundary is the UI delegating to those ops, exercised by node smoke.

## Outcome

Completed: 2026-06-06

What changed:

- Added `ReplayImportExport` for exporting the current run through Rust and importing pasted replay documents through Rust validation.
- Added `ReplayViewer` for Rust-projected reset/step views, cursor progress, and effect summaries.
- Wired `main.tsx` to call `exportReplay`, `importReplay`, `replayReset`, and `replayStep`; replay state enters the reducer only after Rust import validation succeeds.
- Added replay UI styles.
- Extended `smoke-ui.mjs` to export a generated run, re-import it, reset it, and step through Rust-projected replay state.

Deviations from original plan:

- The imported replay document remains opaque in UI state (`document: null`) to avoid treating pasted JSON as authoritative TypeScript state.

Verification results:

- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:ui` passed and reported `replay_cursor: 1`.
- `grep -rnE "dangerouslySetInnerHTML|localStorage\.setItem\(.*replay|JSON\.parse\(.*authoritative" apps/web/src/components/ReplayImportExport.tsx` returned no matches.
