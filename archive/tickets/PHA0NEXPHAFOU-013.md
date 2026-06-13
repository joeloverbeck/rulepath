# PHA0NEXPHAFOU-013: Templates — UI/benchmark/release/pressure + AGENT-TASK + README N-seat fields

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — `templates/GAME-UI.md`, `GAME-BENCHMARKS.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `PRIMITIVE-PRESSURE-LEDGER.md`, `AGENT-TASK.md`, `README.md` edits only.
**Deps**: PHA0NEXPHAFOU-002, PHA0NEXPHAFOU-005, PHA0NEXPHAFOU-007

## Problem

The remaining templates lack N-seat fields: `GAME-UI.md` has no multi-seat panel/turn-order/showdown render fields; `GAME-BENCHMARKS.md` has no seat-count/surface fields; `PUBLIC-RELEASE-CHECKLIST.md` has no N-seat/pairwise rows; `PRIMITIVE-PRESSURE-LEDGER.md` has no seat/topology pressure rows; `AGENT-TASK.md` has no seat/surface scope row or forbidden-generic-multiplayer-mega-task checkbox; and `templates/README.md` has no adoption note telling authors which templates need the N-seat additions.

## Assumption Reassessment (2026-06-13)

1. No code change. `templates/` holds `GAME-UI.md`, `GAME-BENCHMARKS.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `PRIMITIVE-PRESSURE-LEDGER.md`, `AGENT-TASK.md`, `README.md` (verified via `ls templates/`).
2. Docs: the doctrine sources are `docs/UI-INTERACTION.md` (PHA0NEXPHAFOU-005) for `GAME-UI`; `docs/TESTING-REPLAY-BENCHMARKING.md` (PHA0NEXPHAFOU-007) for `GAME-BENCHMARKS` + `PUBLIC-RELEASE-CHECKLIST`; `docs/AGENT-DISCIPLINE.md` (PHA0NEXPHAFOU-009, sibling) for `AGENT-TASK`; `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (PHA0NEXPHAFOU-002) throughout.
3. Cross-artifact boundary under audit: the remaining template set + the README adoption note; shared surface = the seat/surface/no-leak/outcome fields mirroring UI-INTERACTION and TESTING.
4. FOUNDATIONS principle restate: §11 (no-leak, benchmark coverage), §7 (public UI is play-first/legal-only), and §3/§12 (AGENT-TASK must forbid an unbounded "implement multiplayer" mega-task and kernel growth). The additions are meaning-preserving.
5. Enforcement surface: §11 no-leak + benchmark coverage. `PUBLIC-RELEASE-CHECKLIST.md`'s pairwise no-leak rows and `GAME-BENCHMARKS.md`'s seat-count fixtures are per-game enforcement inputs; `AGENT-TASK.md`'s forbidden-change checkbox guards §3/§12 (no generic multiplayer mega-task). No leakage path introduced.

## Architecture Check

1. Grouping the UI/benchmark/release/pressure templates plus `AGENT-TASK` and the README adoption note keeps the "remaining templates" diff coherent and reviewable in one pass.
2. No backwards-compatibility aliasing/shims introduced.
3. Templates keep typed mechanic nouns game-local (§3); `AGENT-TASK`'s forbidden-mega-task checkbox reinforces the bounded-task law (§12), not weakens it.

## Verification Layers

1. `AGENT-TASK.md` seat/surface scope row + forbidden generic-multiplayer-mega-task checkbox present → codebase grep-proof + FOUNDATIONS alignment check (§3/§12).
2. `PUBLIC-RELEASE-CHECKLIST.md` pairwise no-leak / per-seat outcome rows present → FOUNDATIONS alignment check (§11 no-leak firewall).
3. `GAME-BENCHMARKS.md` seat-count + max-surface fixture fields present → manual review (benchmark coverage per `docs/TESTING-REPLAY-BENCHMARKING.md`).
4. `GAME-UI.md` multi-seat layout + showdown render fields and `PRIMITIVE-PRESSURE-LEDGER.md` seat/topology rows and the `README.md` adoption note present → manual review.
5. Links resolve → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `templates/GAME-UI.md`

Add a multi-seat layout table (seat rail / card positions, active/pending seats, local seat selector, observer mode, team grouping, small-screen collapse), a showdown table (each contender's evaluated combo, used components, rank vector, decisive comparison, folded/non-revealed seats), and an object-count/render budget.

### 2. `templates/GAME-BENCHMARKS.md`

Add a benchmark matrix by seat count and max-surface fixture; require legal-action, preview, apply, project-view, serialize, replay-import, bot-turn, and WASM smoke benchmarks for the largest official variant.

### 3. `templates/PUBLIC-RELEASE-CHECKLIST.md`

Add rows: supported player-count smoke; all seat labels/roles safe; pairwise no-leak matrix complete; per-seat outcome explanation complete; multi-seat replay export/import; large-surface performance; small-screen seat-rail accessibility.

### 4. `templates/PRIMITIVE-PRESSURE-LEDGER.md`

Add comparison rows for seat count, topology size, data size, action fanout, view payload size, no-leak complexity, and benchmark pressure; add examples (graph topology, private-hand/deck, trick-taking, side-pot allocation).

### 5. `templates/AGENT-TASK.md`

Add a required "seat/surface scope" row and a forbidden-change checkbox (no generic multiplayer mega-task); hidden-info tasks must name the exact viewer pairs and surfaces tested.

### 6. `templates/README.md`

Add an adoption note: every game with >2 seats fills seat-range, turn-order, view matrix, no-leak matrix, outcome matrix, and surface-scale fields across the templates.

## Files to Touch

- `templates/GAME-UI.md` (modify)
- `templates/GAME-BENCHMARKS.md` (modify)
- `templates/PUBLIC-RELEASE-CHECKLIST.md` (modify)
- `templates/PRIMITIVE-PRESSURE-LEDGER.md` (modify)
- `templates/AGENT-TASK.md` (modify)
- `templates/README.md` (modify)

## Out of Scope

- The contract and bot template clusters (PHA0NEXPHAFOU-011 / PHA0NEXPHAFOU-012).
- Editing `docs/AGENT-DISCIPLINE.md` (PHA0NEXPHAFOU-009) or the UI/TESTING docs (PHA0NEXPHAFOU-005/007).
- Authoring the optional `templates/MULTI-SEAT-VIEW-MATRIX.md` (deferred per spec Assumption A4).
- Any `games/*`/web code.

## Acceptance Criteria

### Tests That Must Pass

1. Each of the six templates carries its N-seat additions per What to Change.
2. `AGENT-TASK.md` carries the seat/surface scope row and the forbidden generic-multiplayer-mega-task checkbox; `templates/README.md` carries the adoption note.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The §11 pairwise no-leak and per-seat outcome rows are present in `PUBLIC-RELEASE-CHECKLIST.md`.
2. `AGENT-TASK.md` reinforces the bounded-task law (§12) — it forbids an unbounded "implement multiplayer" mega-task.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -niE "seat/surface scope|multiplayer mega-task|pairwise no-leak|seat count" templates/AGENT-TASK.md templates/PUBLIC-RELEASE-CHECKLIST.md templates/GAME-BENCHMARKS.md templates/README.md`
3. `bash scripts/boundary-check.sh`

## Outcome

Completed: 2026-06-13

Summary:

- Added N-seat layout, showdown rendering, render-budget, benchmark fixture, pairwise no-leak, per-seat outcome, primitive-pressure, and bounded agent-task prompts to the remaining template set.
- Added the template README adoption note for games with more than two seats.

Deviations:

- None.

Verification:

- `node scripts/check-doc-links.mjs`
- `grep -niE "seat/surface scope|multiplayer mega-task|pairwise no-leak|seat count" templates/AGENT-TASK.md templates/PUBLIC-RELEASE-CHECKLIST.md templates/GAME-BENCHMARKS.md templates/README.md`
- `bash scripts/boundary-check.sh`
