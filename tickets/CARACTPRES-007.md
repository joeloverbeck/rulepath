# CARACTPRES-007: Shared ActionPathBuilder and Event Frontier adoption

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web` (shared progressive-construction component, Event Frontier action panel replacement, smoke); no Rust surface touched
**Deps**: None (consumes the pre-existing `ActionTree` contract; shares `EventFrontierBoard.tsx` with CARACTPRES-005 — different section, coordinate merges)

## Problem

Event Frontier's action panel is a flat tree dump: `collectLeaves()` flattens the Rust `ActionTree` to leaves and renders raw path segments (`operation / survey / operation / survey / site charterhouse,site crossing`), discarding the human-readable `label`/`accessibility_label` Rust attaches to every choice (`apps/web/src/components/EventFrontierBoard.tsx:226-246,279-285,311-313`; labels built at `games/event_frontier/src/actions.rs:427-476,772-778`). `docs/UI-INTERACTION.md` §8 requires compound actions to be built through staged legal choices. Spec Workstream C (D4): one shared `ActionPathBuilder` that walks the tree stage by stage — grouped choices, Rust labels, Back/Cancel, leaf Confirm — adopted first by Event Frontier.

## Assumption Reassessment (2026-06-12)

1. `ActionChoice` carries `segment`, `label`, `accessibility_label`, and nested `next.choices` in the generic contract (`crates/engine-core/src/action.rs:35-44`); the web client types mirror this (`apps/web/src/wasm/client.ts` `ActionChoice`/`ActionTree`, consumed by `ActionControls.tsx:19-48` and the per-game panels). Event Frontier supplies stage labels like "Choose {kind} operation targets" / "Apply {kind} to {payload}" — verified at `actions.rs:427-476`.
2. Spec authority: `specs/card-and-action-presentation-shared-surfaces.md` §6 D4, §9 exit criterion 3 (staged family → targets → confirm; Back/Cancel; **submitted paths byte-identical to the old panel's**). `docs/UI-INTERACTION.md` §6 (action lifecycle with freshness token) and §8 (progressive construction) govern the interaction shape; cancel is free because no Rust state is committed until submission.
3. Cross-artifact boundary under audit: the action-path submission contract. The current panel submits `eventFrontierSubmitPath(leaf)` — `leaf.choice.segment.includes("/") ? [leaf.choice.segment] : leaf.path` (`EventFrontierBoard.tsx:287-289`), a compound-segment encoding quirk the builder MUST reproduce exactly so replay/command logs stay identical for identical selections.
4. FOUNDATIONS §2/§7 restated (this ticket implements them): TS renders Rust's staged legal choices and invents no legality — every stage's options come from the tree as supplied; grouping/order/affordance is presentation. Stage rendering uses `choice.label`; no segment-string munging (`actionLabel`'s `replaceAll` dies with its last caller).
5. Determinism/replay surface (§11): the builder changes *presentation* of selection, not the submitted command content — the byte-identical-path invariant is the enforcement surface, proven by smoke comparing submitted paths and by the unchanged Rust validation path (stale submissions still rejected gracefully with refreshed tree per UI-INTERACTION §6).

## Architecture Check

1. A shared stage-walking component beats per-game leaf dumps and per-game staged forms: the tree already encodes the stages (Rust owns structure), so one renderer with grouping-by-stage + path state covers every compound-action game; board-native affordances remain legitimate alternatives where clicks map 1:1 to choices (the CARACTPRES-008 audit decides per game).
2. No backwards-compatibility aliasing/shims: the flat leaf list and its helpers (`collectLeaves`, `actionLabel`) are removed from `EventFrontierBoard.tsx` in the same diff; no toggle between old/new panels.
3. `engine-core`/`game-stdlib` untouched; the component lives in `apps/web` (§3 governs Rust crates).

## Verification Layers

1. Submitted paths byte-identical to the old panel for equivalent selections -> e2e smoke driving both a single-stage action (`pass`) and a multi-target compound (`operation/survey` + two sites) and asserting the submitted path arrays (including the compound-segment quirk).
2. No TS legality: every rendered control maps to a tree-supplied choice -> codebase grep-proof (no filtering/synthesis of choices beyond grouping) + smoke asserting option sets equal tree contents per stage.
3. Stale-tree rejection still graceful -> smoke exercising a stale submission (the existing freshness-token path) with the builder active.
4. Accessibility (labels from `accessibility_label`, keyboard stage navigation, visible focus, Back/Cancel reachable) -> UI smoke + manual review per `docs/UI-INTERACTION.md` §17.

## What to Change

### 1. Shared `ActionPathBuilder` component

`apps/web/src/components/ActionPathBuilder.tsx`: walks `ActionTree` stage by stage; renders the current stage's `choices` as grouped controls using `label`/`accessibility_label`; maintains the selected path; Back (pop one stage), Cancel (reset to root); Confirm on leaves with a Rust-label path summary; submits via the same `onPathSubmit` contract, reproducing the compound-segment encoding. Player-facing heading copy is neutral ("Actions") — no debug vocabulary. Styles in `apps/web/src/styles.css`.

### 2. Event Frontier adoption

Replace the "Rust legal choices" section in `EventFrontierBoard.tsx` with `ActionPathBuilder`; delete `collectLeaves`/`actionLabel` and the leaf-dump markup; keep `eventFrontierSubmitPath`'s encoding inside the submission adapter.

### 3. Smoke coverage

Extend `apps/web/e2e/event-frontier.smoke.mjs`: stage walk, Back/Cancel, Confirm summary, byte-identical submission, stale-tree rejection.

## Files to Touch

- `apps/web/src/components/ActionPathBuilder.tsx` (new)
- `apps/web/src/components/EventFrontierBoard.tsx` (modify)
- `apps/web/src/styles.css` (modify)
- `apps/web/e2e/event-frontier.smoke.mjs` (modify)

## Out of Scope

- Adopting the builder in other games — CARACTPRES-008 audit decides per game.
- Deck surfaces — CARACTPRES-005 (same file, different section; flag merge coordination).
- The generic `ActionControls.tsx` fallback (stays for single-stage games).
- Any Rust change: tree shape, labels, validation, freshness semantics all unchanged.
- Catalog copy hygiene beyond this panel's own heading — CARACTPRES-009.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:ui` green with the builder active.
2. Extended e2e: staged construction works end-to-end; submitted paths for scripted selections equal the pre-change recordings; stale submission yields the graceful diagnostic + refreshed tree.
3. `npm --prefix apps/web run smoke:effects` green (submission → effect → settle unchanged).

### Invariants

1. Every gameplay control maps to a Rust-supplied legal choice; illegal choices are absent (FOUNDATIONS §7; UI-INTERACTION §7).
2. Submitted action paths are byte-identical to the previous panel's for identical selections (replay/command-log determinism, §11).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/event-frontier.smoke.mjs` — stage walk, Back/Cancel, Confirm, submission fidelity, stale-tree rejection.

### Commands

1. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run smoke:e2e`
3. Narrow boundary rationale: presentation-only; Rust action generation/validation untouched (no crate tests affected).
