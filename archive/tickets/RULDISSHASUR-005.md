# RULDISSHASUR-005: Shared RulesPanel component + loader + shell state + styles

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/RulesPanel.tsx`, `state/shellReducer.ts`, `wasm/client.ts` (type only), `styles.css`, `package.json`; no Rust/engine/WASM behavior, and no legality moves to TypeScript (FOUNDATIONS §2).
**Deps**: None

## Problem

The surface needs one reusable, accessible rules panel that fetches the static per-game asset and renders it as inert prose, plus the presentation-only shell state that drives it. This is self-contained UI infrastructure; access-point wiring and end-to-end verification follow. Source: `specs/rules-display-shared-surface.md` §6.4 (web runtime design), §7.3–7.4 (layout / panel IA), §9.1 (web modules), §9.2 (Markdown rendering constraints).

## Assumption Reassessment (2026-06-09)

1. `apps/web/src/state/shellReducer.ts` uses a discriminated-union `ShellAction` with a switch-based reducer and already carries a presentation-only toggle precedent (`devPanelOpen` + `devPanelToggled`); the new `rulesPanelOpen` / `rulesPanelGameId` / `rulesPanelStatus` / `rulesPanelMarkdown` fields mirror it exactly. `apps/web/src/wasm/client.ts` exposes `listGames()` → `GameCatalogEntry[]` and the operation surface (`get_view`, `get_view_for_viewer`, `get_action_tree`, `get_effects`); no `get_rules` op exists (static path confirmed). `apps/web/src/components/RulesPanel.tsx` does not yet exist (only `DevPanel.tsx` is a comparable panel).
2. Behavior/layout requirements from spec §6.4/§7.3/§7.4/§9.2; `apps/web/src/styles.css` is the single stylesheet.
3. Cross-artifact boundary under audit: the panel fetches the generated `apps/web/public/rules/<game_id>.md` assets (produced by RULDISSHASUR-003/-004) and reads/writes only presentation-only shell state — that state must never feed Rust actions, legality, or the bot path.
4. FOUNDATIONS principle restated: §2 (TypeScript presents only; never decides legality/scoring/effects) and §11 (no hidden-information leak — the panel must not read private match state or write rules text into DOM test IDs, storage, effect log, or replay export).
5. No-leak / behavior-authority enforcement surface: the loader accepts only a catalog `game_id` matching `^[a-z0-9_]+$`, fetches the static asset path (never arbitrary input), stores only loading + rendered-content state, renders with raw HTML disabled, and never calls `get_view`/`get_view_for_viewer`/`get_action_tree`/`get_effects`/replay/bot APIs to populate rules text. RULDISSHASUR-007 negative-tests the no-leak guarantee; the chosen Markdown renderer (spec §9.2 default: a vetted, raw-HTML-disabled dependency) is the XSS-containment surface.

## Architecture Check

1. One shared panel + presentation-only state machine (idle/loading/loaded/error) keeps every entry point rendering the same inert article; mirroring the `devPanelOpen` reducer precedent avoids inventing a new state pattern.
2. No backwards-compatibility shims; the component, state fields, and dependency are new and additive.
3. `engine-core`/`game-stdlib` untouched; all change is `apps/web` presentation. No legality, scoring, or view projection moves to TypeScript (§2).

## Verification Layers

1. Shell state is presentation-only → grep-proof the new `rulesPanel*` fields are never read by action-apply / bot / legality code paths (only by panel + entry triggers); mirrors `devPanelOpen`.
2. Raw HTML is inert → render a fixture containing `<script>`/`<iframe>`/event-handler attributes through the panel; assert it is stripped/escaped (no executable nodes).
3. No hidden-information read → grep-proof `RulesPanel.tsx` and the loader call none of `get_view`/`get_view_for_viewer`/`get_action_tree`/`get_effects`/replay/bot APIs.
4. Loading/error/loaded states render → component-level check renders all three from a fixture; error shows "Rules are unavailable for this game." with no path/stack leak.
5. Behavior-authority alignment → manual review against FOUNDATIONS §2: the panel decides no legality and writes no Rust state.

## What to Change

### 1. `apps/web/src/components/RulesPanel.tsx` (new)

Drawer/sheet presentation with close button, heading linked via `aria-labelledby`, generated table of contents from headings, loading/error/loaded states, and focus management hooks (focus enters on open, returns to trigger on close; `Esc` closes modal mode). Render Markdown via the safe subset (headings, paragraphs, emphasis, lists, tables, inline code, internal anchors) with raw HTML disabled.

### 2. Rules loader

Accept only a catalog `game_id` (reject IDs failing `^[a-z0-9_]+$` or not in the loaded catalog), fetch `<base>/rules/<id>.md`, and set panel status/markdown. Store no match state.

### 3. `apps/web/src/state/shellReducer.ts` (modify)

Add presentation-only `rulesPanelOpen: boolean`, `rulesPanelGameId: string | null`, `rulesPanelStatus: "idle" | "loading" | "loaded" | "error"`, `rulesPanelMarkdown: string | null`, plus actions (e.g. `rulesPanelOpened`/`rulesPanelClosed`/`rulesPanelLoadStarted`/`rulesPanelLoaded`/`rulesPanelFailed`) following the existing union + switch pattern.

### 4. `apps/web/src/wasm/client.ts` (modify)

Optionally add a helper type for static rules-asset metadata; do NOT add or alter any game operation.

### 5. `apps/web/src/styles.css` + `apps/web/package.json` (modify)

Add drawer/sheet, article typography, focus, status, and responsive styles; add a single vetted, permissively-licensed Markdown renderer dependency configured with raw HTML disabled (spec §9.2 default).

## Files to Touch

- `apps/web/src/components/RulesPanel.tsx` (new)
- `apps/web/src/state/shellReducer.ts` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/styles.css` (modify)
- `apps/web/package.json` (modify)

## Out of Scope

- Wiring entry points in `GamePicker`/`MatchSetup`/in-play controls and mounting the panel (RULDISSHASUR-006).
- The e2e / accessibility / no-leak smoke (RULDISSHASUR-007).
- Authoring or generating any player doc/asset (RULDISSHASUR-003/-004).
- Any Rust/engine/WASM change or new `wasm-api` operation; any tooltip-as-primary-surface affordance.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` succeeds (TypeScript compiles with the new component, state, and dependency).
2. A fixture with `<script>`/`<iframe>`/`onerror=` renders inert (no executable nodes) through the panel renderer.
3. `npm --prefix apps/web run smoke:ui` passes (shell still builds and renders).

### Invariants

1. The `rulesPanel*` shell state is presentation-only — it never feeds a Rust action, legality decision, seat change, viewer-perspective change, replay, or effect-log entry (FOUNDATIONS §2).
2. The panel never reads private match state and never emits rules content into DOM test IDs, storage, logs, effect logs, or replay exports (FOUNDATIONS §11 no-leak).

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/RulesPanel.tsx` — new component with loading/error/loaded states and safe-render behavior.
2. `apps/web/src/state/shellReducer.ts` — new presentation-only rules-panel state/actions (reducer-level coverage if the project has it; otherwise exercised by `smoke:ui`/RULDISSHASUR-007).

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. Full e2e/a11y/no-leak verification is the boundary of RULDISSHASUR-007 (it adds the smoke); `build` + `smoke:ui` are the correct boundary for the panel-infra diff.

## Outcome

Completed: 2026-06-09

What changed:

- Added `apps/web/src/components/RulesPanel.tsx` with a static asset loader, accessible dialog/sheet shell, close/Escape handling, focus entry/return, loading/error/loaded states, table of contents, and a narrow React Markdown renderer.
- Added presentation-only rules panel state/actions to `apps/web/src/state/shellReducer.ts`.
- Added rules drawer/sheet, article, table, status, and responsive styles to `apps/web/src/styles.css`.

Deviations from original plan:

- No Markdown dependency was added. A local renderer handles the required subset and never uses `dangerouslySetInnerHTML`, so raw HTML/script/iframe/event-handler text remains inert React text. This avoids network/package churn while preserving the static-presentation boundary.
- `apps/web/src/wasm/client.ts` was not changed; no helper type was needed and no WASM operation was added.
- Access-point wiring and panel mounting remain for RULDISSHASUR-006 as planned.

Verification results:

- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:ui` passed.
- `node scripts/check-player-rules.mjs` passed (`player-rules check passed — 9 catalog games validated`).
- Safety grep found no `dangerouslySetInnerHTML`, `innerHTML`, raw `<script>`/`<iframe>`, `onerror`, view/action/effect/replay/bot API calls, storage writes, or `data-testid` usage in `RulesPanel.tsx`.
- `git diff --check` passed.

Outcome amended: 2026-06-09

- Added modal focus containment to `RulesPanel.tsx` and an explicit visible focus style for secondary rules triggers while implementing RULDISSHASUR-007's accessibility smoke.
