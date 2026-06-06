# GAT4THRMARBOA-012: Web board-aware Three Marks replay view + controls

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/ReplayViewer.tsx` (reusing `ThreeMarksBoard`)
**Deps**: GAT4THRMARBOA-011

## Problem

Three Marks replay must be board-aware: playback reconstructs the board at each step from the Rust replay projection (not TypeScript diffs), shows the placement sequence and current step, displays step effects (mark placed, turn changed, line completed, draw, game ended), highlights the winning line / draw at the correct step, supports reset/step controls, and honours reduced motion. Generic JSON inspection may remain only as a secondary dev surface.

## Assumption Reassessment (2026-06-06)

1. `apps/web/src/components/ReplayViewer.tsx` is the existing Gate 3 replay surface; `ReplayImportExport.tsx` handles local export/import; the board renderer `ThreeMarksBoard.tsx` (GAT4THRMARBOA-011) is reused for replay rendering. The Rust replay projection (board at step + step effects + outcome) comes from GAT4THRMARBOA-007 via the WASM replay reset/step ops (009). Verified `ReplayViewer.tsx` exists.
2. Spec `specs/gate-4-three-marks-board-smoke.md` §13 (board reconstruction from Rust projection, placement sequence, step effects, winning line/draw at correct step, reset/step controls, reduced motion, JSON only as dev panel), §15.7 (replay-step browser smoke).
3. Cross-artifact boundary under audit: the Rust replay-projection contract (`docs/TESTING-REPLAY-BENCHMARKING.md`, `docs/WASM-CLIENT-BOUNDARY.md`) — the replay view renders the Rust projection and never uses guessed state diffs as authority.
4. FOUNDATIONS §11 (replay/hash deterministic; renderer diffs are diagnostics only, not authoritative causality) and §2 (Rust owns replay projection) motivate this ticket: the board at each replay step is the Rust projection; TypeScript never reconstructs board state from inferred diffs.
5. No-leak + determinism enforcement surface (§11): the replay export and the board projection are the firewall and deterministic surface — name them. Three Marks is perfect-information, so replay payloads carry no hidden state; stepping remains usable with animations disabled (reduced motion); local export/import stays viewer-safe through the existing operation group.

## Architecture Check

1. Reusing `ThreeMarksBoard` to render the Rust replay projection per step is cleaner than a parallel replay-only board and guarantees live and replay views share one presenter. Alternative (TS reconstructs board from diffs) is a §11/§12 stop condition and rejected.
2. No backwards-compatibility aliasing/shims — `ReplayViewer` is extended to render the board projection; generic JSON is demoted to a secondary dev surface, not aliased.
3. `engine-core` untouched; no `game-stdlib` change; TypeScript decides no rule state during replay.

## Verification Layers

1. Board-reconstruction invariant -> UI smoke (`smoke:e2e`: replay step shows the board state from the Rust projection, not generic JSON only).
2. Step-effect/winning-line invariant -> UI smoke (step effects render; winning-line highlight / draw appears at the correct replay step).
3. Controls + reduced-motion invariant -> UI smoke + manual review (reset/step work; stepping remains usable with animation disabled).
4. Replay-authority invariant -> FOUNDATIONS alignment check (§11: replay board comes from Rust projection; no guessed-diff authority).

## What to Change

### 1. `ReplayViewer.tsx`

For a Three Marks replay, render the board at the current step via `ThreeMarksBoard` from the Rust replay projection (reset/step ops); show the placement sequence and current step index; render step effects; highlight the Rust winning line / draw at the correct step; honour reduced motion; keep generic JSON as a secondary dev-panel affordance only. Preserve the existing Race to N replay behaviour.

## Files to Touch

- `apps/web/src/components/ReplayViewer.tsx` (modify)

## Out of Scope

- The live board renderer (GAT4THRMARBOA-011) and catalog/setup (010).
- WASM replay reset/step op generalization (done in 009).
- Browser smoke test authoring (GAT4THRMARBOA-013).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` — type-checks and builds.
2. `npm --prefix apps/web run smoke:e2e` — replay step renders a board (not JSON-only) with correct winning-line/draw at the right step.
3. Race to N replay path is non-regressed.

### Invariants

1. The replay board at each step is the Rust projection; TypeScript uses no guessed state diffs as replay authority.
2. Reset/step controls work and remain usable under reduced motion; replay payloads carry no hidden state.

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/ReplayViewer.tsx` — board-aware replay rendering (asserted by 013's `smoke:e2e`).

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:e2e`
3. The replay-step assertion is finalized in the 013 browser smoke; build + e2e step render is the correct boundary for the replay-view diff.

## Outcome

Completed: 2026-06-06

Changes:
- Extended `ReplayViewer.tsx` to render a Three Marks board for Rust replay reset/step projections using `ThreeMarksBoard`.
- Added a replay command sequence display from the imported/exported replay document while keeping the board state itself sourced from Rust replay projection.
- Added read-only mode to `ThreeMarksBoard` for replay rendering so replay cells are not interactive.
- Preserved imported replay documents in shell state for display-only sequence metadata.
- Added replay-board and placement-sequence styling.

Deviations:
- `ThreeMarksBoard.tsx`, `main.tsx`, and `styles.css` were touched in addition to `ReplayViewer.tsx` to support read-only board reuse, document preservation, and replay layout.
- Full persistent browser smoke assertions remain in GAT4THRMARBOA-013; this ticket used a one-off local Puppeteer probe for the new Three Marks replay board path.

Verification:
- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:e2e`
- One-off local Puppeteer probe: selected Three Marks, made a placement, exported/imported the replay, confirmed a replay board rendered at cursor 0, stepped to cursor 1, confirmed `r1c1` was occupied from the Rust replay projection, and confirmed a placement sequence rendered.
