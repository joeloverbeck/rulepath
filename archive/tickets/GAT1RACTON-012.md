# GAT1RACTON-012: Bare apps/web harness + UI.md + UI smoke

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `apps/web` gains a bare `race_to_n` harness page (human vs random bot) over the `wasm-api` boundary; `games/race_to_n/docs/UI.md`; a UI smoke test.
**Deps**: GAT1RACTON-011

## Problem

The spec §5 exit criterion "human vs random bot works in CLI and **web**"
requires a minimal browser path. This ticket adds a **bare** `apps/web` harness
(no picker, no stores, no polish — spec §2) that plays a human seat vs the Level 0
random bot through the `wasm-api` surface, plus a UI smoke covering the
OFFICIAL-GAME-CONTRACT §10 / TESTING §11 path. TypeScript presents only; it
invents no legality (FOUNDATIONS §2/§7).

## Assumption Reassessment (2026-06-05)

1. `apps/web` exists with `src/main.tsx`, `index.html`, `vite.config.ts`,
   `package.json`, and a `scripts/smoke-load-wasm.mjs` (verified `find apps/web`).
   CI already runs `npm run smoke:wasm` and `npm run build` (verified
   `.github/workflows/ci.yml`). The current page is a Gate 0 WASM-load smoke; this
   ticket adds gameplay UI.
2. The harness consumes the `wasm-api` batched surface from GAT1RACTON-011
   (`new_match`/`get_view`/`get_action_tree`/`apply_action`/`run_bot_turn`/
   `get_effects`). It builds controls from the Rust-supplied action tree and
   submits action paths with freshness tokens (AGENT-DISCIPLINE §10).
3. Cross-artifact boundary under audit: `apps/web` reaches Rust ONLY through the
   `wasm-api` package boundary (ARCHITECTURE §2); it MUST NOT import game Rust
   internals. The UI smoke is an integration test, not a rule test (TESTING §11).
4. FOUNDATIONS §7 (public UI is legal-only; TS invents no legality) and §2 (TS
   presents only) motivate this ticket. Controls come from the Rust action tree;
   illegal moves are simply absent. Animation (if any) is driven by semantic
   effects from `get_effects` (ARCHITECTURE §7) — but Gate 1 is bare, so a minimal
   effect display suffices.
5. No-leak / legality enforcement surface: the UI is the §11 "TypeScript does not
   decide legality" enforcement surface. Confirm: the harness renders only
   Rust-supplied legal choices, submits paths with freshness tokens, and shows the
   Rust diagnostic on a stale/invalid submission (the stale-submission path is a
   required smoke step). `race_to_n` is perfect-information, so no hidden-state
   DOM/payload leak is possible (recorded rationale; the no-leak negative test is
   n/a per TESTING §8).
6. Schema/contract: the harness consumes the public-view/action-tree/effects JSON
   from the `wasm-api` surface (additive consumption). `UI.md` follows
   `templates/GAME-UI.md`. No schema changed.

## Architecture Check

1. A bare harness (no picker/stores/polish) is the spec's Gate 1 scope; the
   polished shell is Gate 3. Building controls from the Rust action tree (not
   hardcoded buttons) is the only design that keeps TS presentation-only
   (FOUNDATIONS §7). Alternative (TS computing legal moves) is a §12 stop
   condition.
2. No backwards-compatibility shims — the Gate 0 smoke page is extended, not
   aliased.
3. `engine-core` untouched; no Rust internals imported by `apps/web`
   (ARCHITECTURE §2). `game-stdlib` untouched.

## Verification Layers

1. No TS legality -> codebase grep-proof + manual review (no legal-move
   computation in `apps/web/src`; controls derive from the action tree).
2. Boundary discipline -> codebase grep-proof (`apps/web` imports only the
   `wasm-api` package, no `games/*` internals — ARCHITECTURE §2).
3. UI smoke path -> UI smoke / skill-template dry-run (load, start, show legal
   actions, one human action, one bot turn, semantic effects, stale-submission
   diagnostic — TESTING §11).
4. Effects drive display -> FOUNDATIONS alignment check (§7: visible changes come
   from `get_effects`, then settle to the latest public view).

## What to Change

### 1. Bare harness (`apps/web/src/main.tsx` + supporting files)

Replace/extend the Gate 0 page with a minimal `race_to_n` harness: start a match,
render the public view, render legal-action controls from `get_action_tree`,
submit a human action (with freshness token) via `apply_action`, run a bot turn
via `run_bot_turn`, display semantic effects from `get_effects`, and show the Rust
diagnostic when a stale/invalid action is submitted. No picker, no stores, no
polish.

### 2. UI smoke test

Add a smoke test (extending the existing `apps/web` smoke harness pattern /
`scripts/`) covering: load, start, display legal actions, one human action, one
bot turn, semantic effects, and the stale-submission diagnostic path. Wire an
`npm run` script if needed.

### 3. UI.md

Author `games/race_to_n/docs/UI.md` from `templates/GAME-UI.md` (minimal — the
bare harness interaction pattern).

## Files to Touch

- `apps/web/src/main.tsx` (modify)
- `apps/web/src/styles.css` (modify) — minimal
- `apps/web/package.json` (modify) — add a `smoke:ui` script if needed
- `apps/web/scripts/smoke-ui.mjs` (new — UI smoke, following the existing smoke pattern)
- `games/race_to_n/docs/UI.md` (new)

## Out of Scope

- Gate 3 shell: picker, public-view/action/effect stores, effect queue, replay
  controls/viewer, dev toggle, local import/export UI (spec §2; Forbidden).
- Any TypeScript legality (forbidden).
- Visual polish / renderer detour (spec §2 Not allowed).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` — the harness builds.
2. The UI smoke (`npm --prefix apps/web run smoke:ui` or equivalent) covers load, start, legal actions, one human action, one bot turn, effects, and the stale-submission diagnostic path.
3. `npm --prefix apps/web run smoke:wasm` — existing WASM-load smoke still passes.

### Invariants

1. No legality is computed in TypeScript; controls derive from the Rust action tree (FOUNDATIONS §2/§7/§12).
2. `apps/web` reaches Rust only through the `wasm-api` boundary (ARCHITECTURE §2).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-ui.mjs` — the TESTING §11 click-path smoke (manual-runbook-style automation over the bare harness).
2. `games/race_to_n/docs/UI.md` — interaction-pattern doc (no test; manual review).

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui` (and `smoke:wasm`)
3. `grep -rniE 'legal|isValid|canPlay' apps/web/src` — expect no legality decisioning in TS (boundary proof; matches that only render Rust-supplied data are acceptable on review).
