# GAT3WASMSTAWEB-011: Base-aware WASM asset loading + preview/static-serve + dist smoke

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — build/config + TypeScript asset loading only (`apps/web`).
**Deps**: 001

## Problem

The app fetches the WASM artifact by absolute root path (`fetch("/wasm_api.wasm")`),
which works only when the site is root-mounted and can fail under nested/static
local serving (spec §18.2, §23.4). Gate 3 also requires a smoke path that serves
the built `dist` output, not only the dev server (§18.3). Today `apps/web` has no
`preview`/static-serve script and `vite.config.ts` sets no `base`. This ticket
makes asset loading base-aware and adds a dist-serving smoke.

## Assumption Reassessment (2026-06-06)

1. `apps/web/src/main.tsx` (now `apps/web/src/wasm/client.ts` after
   GAT3WASMSTAWEB-001) fetches `"/wasm_api.wasm"` (loadApi, ~line 200).
   `apps/web/vite.config.ts` is `defineConfig({ plugins: [react()] })` with no
   `base` (default `/`). `apps/web/package.json` scripts are `build:wasm`, `build`
   (`build:wasm && tsc --noEmit && vite build`), `smoke:wasm`, `smoke:ui`
   (`build && node scripts/smoke-ui.mjs`) — no `preview`. `build:wasm` installs the
   artifact to `apps/web/public/wasm_api.wasm`.
2. Spec §18.2 acceptable outcomes: document/enforce root-relative serving, or prefer
   a base-aware artifact URL that works with Vite build/preview and local static
   paths (preferred direction is base-aware/local-static robust loading). §18.3
   wants `npm run preview` or an equivalent local static server proving the
   production build loads + WASM fetch succeeds from built output.
3. Cross-artifact boundary under audit: the Vite `base` config ↔ the client's
   artifact URL resolution. The client must resolve the `.wasm` URL relative to the
   app base (e.g. via `import.meta.env.BASE_URL` / `new URL(..., import.meta.url)`)
   rather than a hardcoded `/`, so dev, `vite preview`, and a static file server all
   resolve the artifact.

## Architecture Check

1. Resolving the artifact URL from the Vite base (instead of a hardcoded `/`) plus a
   `preview`-served dist smoke is cleaner than documenting a root-only constraint: it
   makes the built app portable across local static serving and proves the *built*
   output, closing the §23.3 "smoke passes while the real shell is broken" gap.
2. No backwards-compatibility shims: the absolute `/wasm_api.wasm` fetch is replaced
   by base-aware resolution; no dual path remains.
3. `engine-core` untouched; build/config only; `game-stdlib` untouched.

## Verification Layers

1. Base-aware loading → codebase grep-proof: `client.ts` no longer hardcodes
   `"/wasm_api.wasm"`; it derives the URL from the Vite base.
2. Dist build serves + loads WASM → simulation/CLI run: a `preview`/static-serve
   smoke loads the built `dist`, the WASM fetch succeeds, and a match can start.
3. Existing smoke unaffected → simulation: `smoke:ui` still passes (dev/build path).
4. Single-layer beyond the above is N/A: this is build/config + asset-URL wiring;
   correctness is proven by the dist smoke against real built output.

## What to Change

### 1. `apps/web/src/wasm/client.ts`

Resolve the `.wasm` URL from the app base (e.g. `import.meta.env.BASE_URL` or
`new URL("wasm_api.wasm", import.meta.url)`), replacing the hardcoded `/wasm_api.wasm`.

### 2. `apps/web/vite.config.ts`

Set an explicit `base` consistent with the documented local static-serving target
(e.g. `base: "./"` for relative/static serving, or document root-mount), so built
asset paths match the chosen serving mode.

### 3. `apps/web/package.json` + dist smoke

Add a `preview` (or static-serve) script and a `smoke:preview` that serves the built
`dist` and runs a smoke proving: production build loads, WASM fetch succeeds from
built output, a match starts, legal actions render, human + bot turn apply.

## Files to Touch

- `apps/web/src/wasm/client.ts` (modify) — base-aware artifact URL
- `apps/web/vite.config.ts` (modify) — explicit `base`
- `apps/web/package.json` (modify) — `preview` + `smoke:preview` scripts
- `apps/web/scripts/smoke-preview.mjs` (new) — dist-serving smoke

## Out of Scope

- The Puppeteer rendered-browser E2E harness — GAT3WASMSTAWEB-013 (this ticket is the served-dist plumbing it can target).
- Hosted deployment (forbidden — §5, §18.4).
- WASM/API op-coverage smoke — GAT3WASMSTAWEB-012.

## Acceptance Criteria

### Tests That Must Pass

1. `cd apps/web && npm run build` — produces `dist` with the WASM artifact.
2. `cd apps/web && npm run smoke:preview` — the served `dist` loads, WASM fetches from built output, and a match starts with a human + bot turn.
3. `grep -rn "\"/wasm_api.wasm\"" apps/web/src` — returns nothing (no hardcoded root path).

### Invariants

1. The WASM artifact URL resolves from the Vite base, working under dev, `preview`, and static local serving.
2. The built `dist` is verified by a smoke that serves built output, not only the dev server; no backend is required.

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-preview.mjs` (new) — serves `dist` and exercises load → match → human/bot turn; rationale: proves the *built* shell, closing the dev-only-smoke gap (§23.3).

### Commands

1. `cd apps/web && npm run build`
2. `cd apps/web && npm run smoke:preview`
3. A served-dist smoke (not a unit test) is the correct boundary: the risk is asset-path resolution under static serving, which only appears against built output.
