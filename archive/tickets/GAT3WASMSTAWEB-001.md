# GAT3WASMSTAWEB-001: Extract typed TypeScript WASM client module

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — TypeScript/presentation only (`apps/web`); no Rust crate touched.
**Deps**: None

## Problem

The low-level WASM bridge currently lives inline in `apps/web/src/main.tsx`:
the `RulepathApi` class, the `WasmExports` type, `loadApi()`, memory/UTF-8
encode-decode, `last_output` reads, and JSON parsing all sit beside the React
`App` component. Gate 3 (spec §9.3, §10.1) requires the React tree to be kept
away from low-level exported memory functions behind a dedicated typed client
module. Without this extraction, every later Gate 3 ticket (reducer, shell,
modes, replay UI) would wire UI directly to raw pointers/exports, entrenching the
exact boundary violation §9.3 forbids. This is the foundation ticket for all
TypeScript work in the gate.

## Assumption Reassessment (2026-06-06)

1. The bridge is inline today: `apps/web/src/main.tsx` defines `class RulepathApi`
   (lines ~101–192), the `WasmExports` type (lines ~5–44), result types
   (`MatchCreated`, `PublicView`, `ActionTree`, `EffectEntry`, `ApiError`), and
   `async function loadApi()` fetching `/wasm_api.wasm` (line ~200). The React
   `App` consumes the class instance, not raw exports — so this is an extraction
   of working code into a module, not a rewrite (spec §3.4, §A-grounding A1).
2. Spec §9.3 enumerates the client boundary's responsibilities (load/instantiate,
   memory, encode/decode, `last_output`, JSON parse, response normalization,
   typed methods); §10.2 forbids components from touching `legal_additions`/raw
   exports. `tsconfig.json` and `apps/web/package.json` (`build` = `build:wasm &&
   tsc --noEmit && vite build`) confirm the typecheck gate this ticket relies on.
3. Cross-artifact boundary under audit: the TypeScript↔WASM boundary. The module
   is the single owner of `crates/wasm-api`'s raw ABI (`rulepath_alloc`,
   `rulepath_dealloc`, `rulepath_last_output_ptr/len`, the `rulepath_*` op
   exports). No other `apps/web` file may import `WasmExports` or read
   `memory.buffer` after this ticket.
4. FOUNDATIONS §2 (behavior authority) motivates the boundary: Rust owns all
   behavior; TypeScript is presentation/transport only. Restated before trusting
   the spec narrative — the client module MUST NOT add legality, rule, or bot
   logic; it only marshals bytes and normalizes Rust responses.

## Architecture Check

1. A dedicated `apps/web/src/wasm/client.ts` module with typed methods and typed
   result types gives one ABI owner and a stable surface the reducer/components
   import, versus the current inline coupling where any component could reach
   `instance.exports`. This is the cleanest seam for the later tickets to build on.
2. No backwards-compatibility shims: the inline bridge is **moved**, not
   duplicated; `main.tsx` imports the module and the old inline definitions are
   deleted in the same diff (no parallel bridge left behind).
3. `engine-core` is untouched (no Rust change); no mechanic noun enters any
   kernel; `game-stdlib` untouched.

## Verification Layers

1. Components no longer touch raw exports → codebase grep-proof: `rulepath_`,
   `memory.buffer`, and `WasmExports` appear only in `apps/web/src/wasm/`.
2. Boundary returns typed normalized results → schema/type validation: `tsc
   --noEmit` passes with the module's exported result types consumed by `main.tsx`.
3. Behavior authority unchanged → FOUNDATIONS §2 alignment check: the module adds
   no legality/rule/bot logic; it only marshals + normalizes.
4. App still works end-to-end → simulation/CLI run: `npm run smoke:ui` stays green
   (existing node smoke exercises the built app via `render_game_to_text`).

## What to Change

### 1. New `apps/web/src/wasm/client.ts`

Move out of `main.tsx`: the `WasmExports` type, the `EncodedArg` type, the result
types (`MatchCreated`, `PublicView`, `ActionTree`, `ActionChoice`, `EffectEntry`,
`ApiError`), the `RulepathApi` class, and `loadApi()`. Export `RulepathApi`,
`loadApi`, and every result type. Keep the alloc/encode → call → decode →
`JSON.parse` → dealloc lifecycle and the `status !== 0` → throw-typed-`ApiError`
normalization exactly as today (no behavior change).

### 2. `apps/web/src/main.tsx`

Replace the inline definitions with `import { RulepathApi, loadApi, type PublicView,
… } from "./wasm/client"`. Delete the moved code. The `App` component, its
`useState`/`useEffect` wiring, and the `render_game_to_text`/`advanceTime` smoke
globals stay unchanged in this ticket.

## Files to Touch

- `apps/web/src/wasm/client.ts` (new)
- `apps/web/src/main.tsx` (modify) — import from the module; delete inline bridge

## Out of Scope

- Adding new WASM operations (`list_games`, replay) — GAT3WASMSTAWEB-002/-003.
- Reducer/state-machine refactor of `App` — GAT3WASMSTAWEB-004.
- Base-aware artifact URL (still fetches `/wasm_api.wasm` here) — GAT3WASMSTAWEB-011.
- Any component/region restructure — GAT3WASMSTAWEB-005+.

## Acceptance Criteria

### Tests That Must Pass

1. `cd apps/web && npm run build` — `tsc --noEmit` + Vite build succeed.
2. `grep -rnE "rulepath_|memory\.buffer|WasmExports" apps/web/src --include=*.tsx --include=*.ts | grep -v "src/wasm/"` — returns nothing (raw ABI confined to the module).
3. `cd apps/web && npm run smoke:ui` — built app smoke stays green.

### Invariants

1. The `apps/web/src/wasm/` module is the only importer of the raw WASM exports.
2. The client returns typed normalized results/`ApiError`; no React component handles pointers/lengths or `last_output`.

## Test Plan

### New/Modified Tests

1. `None — refactor/extraction ticket; verification is the existing `smoke:ui` node smoke plus `tsc` typecheck named in Assumption Reassessment.`

### Commands

1. `cd apps/web && npm run build`
2. `cd apps/web && npm run smoke:ui`
3. A narrower unit test is not the correct boundary here: the behavior is unchanged and is proven by the existing build + smoke pipeline exercising the real WASM artifact.

## Outcome

Completed: 2026-06-06

What changed:

- Added `apps/web/src/wasm/client.ts` as the dedicated TypeScript WASM client module.
- Moved raw WASM exports, memory encode/decode, `last_output` handling, JSON parsing, `ApiError` normalization, `RulepathApi`, `loadApi`, and exported result types out of `apps/web/src/main.tsx`.
- Left `main.tsx` importing typed client surfaces only; no Rust crate or behavior logic changed.

Deviations from original plan:

- None.

Verification results:

- `npm --prefix apps/web run build` passed.
- `grep -rnE "rulepath_|memory\.buffer|WasmExports" apps/web/src --include=*.tsx --include=*.ts | grep -v "src/wasm/"` returned no matches.
- `npm --prefix apps/web run smoke:ui` passed with version `rulepath-wasm-api/0.1.0`, match `race_to_n-1`, counter `2`, `8` effects, and stale-action diagnostic coverage.
