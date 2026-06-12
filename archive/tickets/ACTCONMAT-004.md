# ACTCONMAT-004: Multi-target composer in the shared action surface

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/ActionPathBuilder.tsx`, `apps/web/src/components/EventFrontierBoard.tsx`. No Rust/engine behavior; submitted command bytes unchanged.
**Deps**: ACTCONMAT-003

## Problem

Choosing Survey with ops 2 over 3 legal sites renders 6 pre-joined combination buttons instead of per-site staged picking; the confirm summary is the raw path string; targets are not selectable by clicking the map sites they name. Worst case is C(6,1)+C(6,2)+C(6,3)=41 buttons; future games scale worse. The fix is a presentation-only composer that groups the legal leaf set into per-target toggles — never inventing a combination Rust did not emit.

## Assumption Reassessment (2026-06-12)

1. `ActionPathBuilder.tsx` currently renders multi-target leaves as individual pre-joined combination buttons (one per legal leaf). The legal leaf set is supplied by Rust (`ActionTree`); `EventFrontierBoard.tsx` renders the map sites (`<text>{site.label}` with the resolved labels from ACTCONMAT-001) but has no linkage between map sites and action targets.
2. Spec D4 / §4.2: render per-target toggles + a live summary derived purely from the legal leaf set (a combination is selectable iff its leaf exists); confirm submits the exact matching leaf path — byte-identical encoding. Boards MAY register target-highlight hooks; `EventFrontierBoard` adopts them.
3. Cross-artifact boundary under audit: the action-tree leaf set (read side) and the command-path encoding (submit side). TS only groups/filters leaves Rust emitted; it never synthesizes a path.
4. FOUNDATIONS §2 / §12 ("TypeScript decides legality"): selecting targets without a matching legal leaf is impossible — the composer disables any toggle combination whose leaf Rust did not emit. No legality invented in TS.
5. Deterministic replay/serialization surface: the submitted path must be byte-identical to the pre-composer leaf-button encoding (the same `OperationSelection` path Rust accepts), so replay/serialization are untouched. Restructuring the action tree into per-site stages would change command encoding — explicitly deferred behind an ADR (spec §4.3); this ticket does NOT restructure the tree.

## Architecture Check

1. A presentation composer over the existing legal leaf set fixes the combinatorial dump without an ADR-gated tree restructuring — it is the replay-safe path (spec A3). Deriving toggle enabled/disabled states from leaf existence keeps Rust the single source of legality.
2. No shim: the composer replaces the combination-button rendering for multi-target leaves; the single-target flow is unchanged.
3. No engine change; `engine-core` untouched. No `game-stdlib` addition.

## Verification Layers

1. Submitted action paths byte-identical to the pre-composer leaf-button flow -> golden/replay check via the e2e smoke comparing submitted path bytes.
2. A combination with no matching legal leaf is non-selectable -> UI smoke (disabled-state assertion).
3. Map-target highlight reflects toggle state -> UI smoke / manual review.

## What to Change

### 1. Composer rendering

In `ActionPathBuilder.tsx`, when a stage's choices are multi-target leaves sharing one operation, render per-target toggles + a live summary instead of one button per combination. Derive the toggle set and enabled/disabled states purely from the legal leaf set.

### 2. Byte-identical submission

On confirm, submit the exact matching leaf path — identical bytes to the pre-composer encoding.

### 3. Board-target highlight hooks

Add optional target-highlight hooks boards can register; `EventFrontierBoard.tsx` adopts them so map sites light up as toggles change.

## Files to Touch

- `apps/web/src/components/ActionPathBuilder.tsx` (modify)
- `apps/web/src/components/EventFrontierBoard.tsx` (modify)
- `apps/web/e2e/event-frontier.smoke.mjs` (modify; byte-identical submission + composer smoke)

## Out of Scope

- Action-tree restructuring into per-site stages (ADR-gated; spec §4.3) — submitted bytes stay identical.
- Cost/consequence rendering (ACTCONMAT-003) — this ticket consumes that flow.
- Match-context / board identity copy (ACTCONMAT-005).

## Acceptance Criteria

### Tests That Must Pass

1. `apps/web/e2e/event-frontier.smoke.mjs` proves the composer never renders more controls than targets + confirm, and that the submitted path bytes equal the pre-composer leaf-button flow for the same target set.
2. A toggle combination with no matching legal leaf is disabled/absent.
3. `npm --prefix apps/web run smoke:e2e` green.

### Invariants

1. Every submittable target combination corresponds to a legal leaf Rust emitted (§2/§12).
2. Submitted command bytes are identical to the pre-composer encoding (§11 determinism; replay/serialization untouched).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/event-frontier.smoke.mjs` — composer control-count, disabled-combination, and byte-identical-submission assertions.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `npm --prefix apps/web run build`
3. `cargo run -p replay-check -- --game event_frontier --all` (confirms no submitted-path drift reaches replay)

## Outcome

Completed: 2026-06-12

Implemented a presentation-only multi-target composer in
`ActionPathBuilder`. When a legal leaf set contains site-target combinations
for one operation, the builder now renders per-target toggles, derives enabled
states only from Rust-emitted legal leaves, and confirms by submitting the
exact matching Rust leaf segment. Single-target and route-style leaves remain
on the ordinary leaf-button path. Event Frontier now receives selected target
ids through a highlight hook and marks matching map sites while a composed
selection is active.

The composer does not restructure the action tree and does not synthesize
command bytes. A target set is confirmable only when an emitted leaf has the
same target set; adding targets without a Rust leaf is disabled.

Verification:

- `npm --prefix apps/web run build` passed.
- `node apps/web/e2e/event-frontier.smoke.mjs` passed.
- `npm --prefix apps/web run smoke:e2e` passed.
- `cargo run -p replay-check -- --game event_frontier --all` passed.
