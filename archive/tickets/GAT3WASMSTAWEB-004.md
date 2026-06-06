# GAT3WASMSTAWEB-004: Reducer/state-machine shell state model

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — TypeScript/presentation only (`apps/web`); no Rust crate touched.
**Deps**: 001

## Problem

The shell's match lifecycle is currently a pile of independent `useState` calls in
`apps/web/src/main.tsx` (`api`, `version`, `matchId`, `view`, `tree`, `effects`,
`lastCursor`, `diagnostic`, `staleToken`, `mode`). Gate 3 adds setup, multiple
play modes, replay, a dev panel, effect cursors, and reduced-motion preference on
top of this; spec §11 ("avoid an uncontrolled pile of scattered React `useState`")
and §23.2 require a reducer/state-machine model with a single authoritative app
state before those features are built. This ticket provides the state substrate
the shell/renderer/modes/replay tickets dispatch into.

## Assumption Reassessment (2026-06-06)

1. Current state is scattered `useState` in `main.tsx` (`App`, lines ~229–238) with
   ad-hoc transitions inside `refresh`/`start`/`playChoice`/`submitStale`. The
   `mode` union is only `"loading" | "ready" | "playing" | "error"` (line ~92).
   Spec §11 enumerates the full state set (WASM load, catalog, selected game, setup
   inputs, match handle, active mode setup/play/replay/dev-error, actor/viewer,
   view, action tree, selected path, pending op, diagnostics, effect cursor/queue,
   replay doc/session/cursor, autoplay state, dev-panel visibility, reduced-motion,
   safe prefs).
2. Spec §11 lists explicit example transitions (`wasmLoaded`→catalog,
   `gameSelected`→setup, `matchStarted`→reset cursor/replay, `actionApplied`→refresh,
   `staleDiagnostic`→refresh+display, `replayImported`→replay-only-after-validation,
   `autoplayPaused`→stop). §11 mandates one authoritative state representation with
   derived visual state allowed but no duplicate copies of Rust view/action/effect
   truth.
3. Cross-artifact boundary under audit: the reducer consumes the typed result types
   exported by `apps/web/src/wasm/client.ts` (created by GAT3WASMSTAWEB-001) —
   `PublicView`, `ActionTree`, `EffectEntry`, `ApiError`. The reducer stores Rust
   truth verbatim and never recomputes it (FOUNDATIONS §2): no legality/rule/bot
   logic in the reducer.

## Architecture Check

1. A single `useReducer` (or equivalent state machine) with a typed action union
   gives one authoritative app state and testable transitions, versus the current
   scattered `useState` where invariants (e.g. "match start resets effect cursor
   and replay state") are implicit and easy to break. This matches the spec's §24.2
   reducer guidance.
2. No backwards-compatibility shims: the scattered `useState` set is replaced, not
   shadowed; `App` reads from the reducer state after this ticket.
3. `engine-core` untouched (no Rust change); the reducer holds Rust-projected data
   as opaque typed values and introduces no mechanic logic; `game-stdlib` untouched.

## Verification Layers

1. Single authoritative state → manual review + `tsc`: one reducer state object;
   no duplicate independent copies of `view`/`tree`/`effects` survive as separate
   `useState`.
2. Transitions are explicit/testable → manual review against spec §11 transition
   list: each named transition (`matchStarted` resets cursor/replay, `staleDiagnostic`
   refreshes, `autoplayPaused` stops advancement) maps to a reducer case.
3. No Rust truth recomputed in TS → FOUNDATIONS §2 alignment check: reducer cases
   only store/clear Rust-supplied values; legality/effects/bot never derived here.
4. Single-layer beyond the above is N/A: this is a TS-internal state refactor with
   no schema/trace/bot surface; behavior is proven by the unchanged `smoke:ui` flow
   plus `tsc`.

## What to Change

### 1. New `apps/web/src/state/shellReducer.ts`

Define the typed shell state (covering the spec §11 set) and a reducer with a typed
action union for the §11 transitions. Keep Rust-projected data (`PublicView`,
`ActionTree`, `EffectEntry[]`, diagnostics, effect cursor) as the single copy of
that truth; model `mode` as `loading | ready | setup | play | replay | error` and
include dev-panel visibility, reduced-motion preference, and pending-operation flags.

### 2. `apps/web/src/main.tsx`

Replace the scattered `useState` calls in `App` with the reducer. Port the existing
`refresh`/`start`/`playChoice`/`submitStale` behavior to dispatch reducer actions,
preserving current observable behavior and the `render_game_to_text` smoke global
(it now serializes reducer-derived state).

## Files to Touch

- `apps/web/src/state/shellReducer.ts` (new)
- `apps/web/src/main.tsx` (modify) — adopt the reducer; port existing handlers

## Out of Scope

- New regions/components (picker, setup, renderer) — GAT3WASMSTAWEB-005/-006.
- New play modes / replay sessions — GAT3WASMSTAWEB-008/-009 (this ticket models the *state shape* they will use, not their flows).
- Persisting safe preferences to local storage — handled where reduced motion lands (GAT3WASMSTAWEB-007/-014).

## Acceptance Criteria

### Tests That Must Pass

1. `cd apps/web && npm run build` — `tsc --noEmit` + Vite build succeed with the reducer in place.
2. `cd apps/web && npm run smoke:ui` — existing start/play/effect flow still works through the reducer.
3. `grep -nE "useState<" apps/web/src/main.tsx | grep -iE "view|tree|effects|cursor"` — returns nothing (Rust-truth state lives only in the reducer).

### Invariants

1. Exactly one authoritative app-state representation; no duplicate independent copies of Rust view/action/effect truth.
2. Reducer cases store/clear Rust-supplied data only; no legality/rule/bot computation occurs in TypeScript.

## Test Plan

### New/Modified Tests

1. `None — state refactor with no TS test framework in the project (adding one is out of scope per spec §19.4 tooling-churn caution); verification is `tsc` + the existing `smoke:ui` flow named above.`

### Commands

1. `cd apps/web && npm run build`
2. `cd apps/web && npm run smoke:ui`
3. A unit-test framework is intentionally not introduced here: the reducer's behavior is exercised end-to-end by `smoke:ui` against the real WASM client, the correct verification boundary for a presentation-state change.

## Outcome

Completed: 2026-06-06

What changed:

- Added `apps/web/src/state/shellReducer.ts` with a single typed shell state, explicit reducer actions, mode coverage for load/setup/play/replay/error, pending operations, replay session state, autoplay state, dev-panel state, reduced-motion state, diagnostics, Rust views/action trees/effects, and effect cursor.
- Replaced the scattered Rust-truth `useState` calls in `apps/web/src/main.tsx` with `useReducer(shellReducer, initialShellState)`.
- Ported WASM load, match start, refresh, action apply, stale diagnostic, and stale submit flows to reducer dispatches while preserving the current rendered controls and `render_game_to_text` shape.

Deviations from original plan:

- No test framework was added; verification stayed with the existing TypeScript build and smoke flow as planned.

Verification results:

- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:ui` passed with version `rulepath-wasm-api/0.1.0`, match `race_to_n-1`, counter `2`, `8` effects, and stale-action diagnostic coverage.
- `grep -nE "useState<" apps/web/src/main.tsx | grep -iE "view|tree|effects|cursor"` returned no matches.
