# GAT13FROCONASY-002: Primitive-pressure ledger and mechanic-atlas reviews

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — documentation only (`games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md`, `docs/MECHANIC-ATLAS.md`)
**Deps**: None

## Problem

FOUNDATIONS §4 and `docs/OFFICIAL-GAME-CONTRACT.md` §7 require the primitive-pressure decisions to be recorded before implementation. Frontier Control introduces graph-map topology, area control, and faction asymmetry (first official uses) and reuses multi-action budgets and role/faction modifiers (second uses) — and it must run the mandatory `game-stdlib::board_space` promoted-primitive applicability audit. This ledger records each review and updates the atlas so the gate proceeds only if every expected outcome holds (spec Work-breakdown item 1, "blocks all implementation tasks").

## Assumption Reassessment (2026-06-11)

1. `templates/PRIMITIVE-PRESSURE-LEDGER.md` is the instantiation source; `games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md` is the exemplar recording first-use rows for shared-outcome, event-deck, role-modified-action-effects, and multi-action budgets, each naming `frontier_control`/`event_frontier` as the revisit trigger (verified).
2. `docs/MECHANIC-ATLAS.md` carries §10 (initial table — `fixed 2D occupancy / board-space identity` is a promoted primitive), §10A (open promotion-debt register, currently `Current debt: _None_`), and §10B (deferred/candidate register with the role-modified-action-effects, multi-action-turn-budgets, shared-outcome, reaction-window, and deterministic-shuffle rows) — verified present.
3. Cross-artifact boundary under audit: the atlas §10/§10A/§10B rows are the shared contract; this ticket adds first-use rows and records second-use comparisons without promoting anything, and must leave §10A reading `_None_` so GAT13FROCONASY-003 (crate skeleton) may proceed.
4. FOUNDATIONS §4 (`game-stdlib` is earned) is the principle under audit: frontier_control's graph/control/asymmetry shapes are **first** uses (record local-only) and budgets/modifiers are **second** uses (compare, keep local) — the third-use hard gate does **not** fire here, so this ticket spawns **no** `game-stdlib` extraction. If any review contradicts its expected outcome (esp. the `board_space` audit finding applicable scope, or a third-use-equivalent role/faction pressure), STOP per §12 and the spec's Work-item-1 instruction.

## Architecture Check

1. Recording the ledger + atlas before code keeps the §4 earning process auditable and prevents speculative promotion; the alternative (deciding promotion after implementation) is exactly the speculative-generalization §12 stop condition.
2. No backwards-compatibility aliasing/shims.
3. Confirms `engine-core` stays free of graph/control/faction nouns and `game-stdlib` gains nothing — the audits' explicit purpose; `board_space` is recorded not-applicable (graph sites have no rectangular coordinates).

## Verification Layers

1. §4 first/second-use discipline -> FOUNDATIONS alignment check (ledger records each shape local-only / keep-local with a named revisit trigger).
2. `board_space` applicability (OGC §7) -> manual review + codebase grep-proof (no `game-stdlib::board_space` import is planned; audit records not-applicable rationale).
3. Atlas debt invariant -> codebase grep-proof (`docs/MECHANIC-ATLAS.md` §10A still reads `_None_`; new §10/§10B rows are `local-only`).

## What to Change

### 1. Author `games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md`

Instantiate from the template; record the seven items from the spec Deliverables atlas row: (a) `board_space` applicability audit (not applicable); (b) role-modified-action-effects second-use comparison vs flood_watch (related but distinct — magnitude modifiers on a shared set vs disjoint faction sets; keep both local); (c) multi-action-turn-budget second-use comparison vs flood_watch (keep local; third-use hard gate arms for `event_frontier`); (d) shared-outcome comparison (not a second use — competitive winner); (e) reaction-window review (not reaction-capable); (f) deterministic-shuffle non-use note; (g) first-use records for graph-map topology / adjacency legality / connectivity scoring, site control / deterministic contest resolution, and faction-asymmetric action sets and scoring.

### 2. Update `docs/MECHANIC-ATLAS.md`

Record the audit outcomes per the spec §Documentation-updates atlas bullets: §10 `fixed 2D occupancy` audit note (frontier_control not applicable); §10B updates to the role-modified-action-effects, multi-action-turn-budgets, shared-outcome, reaction-window, and deterministic-shuffle rows; new §10/§10B `local-only` first-use rows for graph/control/asymmetry naming `event_frontier` as the second-use revisit. Confirm §10A stays `_None_`.

## Files to Touch

- `games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new)
- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- Any crate, code, or static data (GAT13FROCONASY-003+).
- Promoting any helper into `game-stdlib` (explicitly not earned this gate).
- Editing `docs/ROADMAP.md` as a progress diary.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n '_None_' docs/MECHANIC-ATLAS.md` confirms §10A still records no open promotion debt.
2. `node scripts/check-doc-links.mjs` passes.
3. Manual review confirms each of the seven ledger items records its expected outcome and the `board_space` audit reads not-applicable with rationale.

### Invariants

1. No `game-stdlib` promotion is recorded (first/second uses stay local).
2. §10A remains `_None_`, satisfying the spec's admission rule for GAT13FROCONASY-003.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE '_None_|frontier_control' docs/MECHANIC-ATLAS.md`
2. `node scripts/check-doc-links.mjs`
3. Grep + doc-link is the correct boundary: the atlas is prose/contract, validated by content presence, not by a compiler.
