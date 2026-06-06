# GAT5COLFOUPUB-017: Column Four trailing game docs

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — new docs `games/column_four/docs/MECHANICS.md`, `UI.md`, `AI.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `PUBLIC-RELEASE-CHECKLIST.md` (no code surfaces)
**Deps**: 008, 011, 012, 014

## Problem

The remaining official-game docs cite implemented surfaces (bot registry, benchmark results, WASM/UI behavior) and so must land after those surfaces exist. They complete the `docs/OFFICIAL-GAME-CONTRACT.md` documentation set and record the local-only mechanic decision for `column_four` (spec §15, §16).

## Assumption Reassessment (2026-06-06)

1. `games/three_marks/docs/` ships MECHANICS, UI, AI, and GAME-IMPLEMENTATION-ADMISSION as filled docs (verified); `column_four` mirrors them plus `PUBLIC-RELEASE-CHECKLIST.md` (required because Column Four is surfaced as the public showcase, spec §15). RULES/SOURCES landed in 001, COMPETENT-PLAYER/EVIDENCE-PACK in 007, BENCHMARKS in 011, RULE-COVERAGE in 013 — this ticket covers the rest.
2. Spec §15 (doc requirements) and §16 (MECHANICS records repeated-shape comparison + explicit no-extraction decision) define content. Cited surfaces: bot registry (GAT5COLFOUPUB-008 `bots.rs`), benchmark results (011 `BENCHMARKS.md`), WASM ops (012), UI behavior (014 `ColumnFourBoard`).
3. Cross-artifact boundary under audit: the doc-governed contract in `docs/OFFICIAL-GAME-CONTRACT.md` (admission evidence) and `templates/GAME-*.md`. These docs conform to those templates; they assert evidence, they do not create contract surface.
4. FOUNDATIONS §6 (official games are evidence-heavy) and §4/§16 (record mechanic pressure, defer extraction) motivate this ticket: MECHANICS.md must state the local-only decision and the repeated-shape comparison with `three_marks`, and the admission checklist must show passing evidence before admission is considered complete.

## Architecture Check

1. A single trailing docs ticket that lands once the cited surfaces exist avoids a staleness window where docs reference not-yet-built APIs — cleaner than co-locating each doc with its implementing ticket when the doc cross-references multiple surfaces (MECHANICS, ADMISSION span the whole game).
2. No backwards-compatibility aliasing/shims — new docs.
3. No code surfaces; `engine-core`/`game-stdlib` untouched. MECHANICS.md explicitly records that no extraction occurs this gate (FOUNDATIONS §4).

## Verification Layers

1. Doc-completeness invariant -> codebase grep-proof: all five docs exist and follow their `templates/GAME-*.md` structure.
2. Surface-accuracy invariant -> manual review: AI.md names the implemented bot registry (008); BENCHMARKS cross-reference is consistent with 011; UI.md describes the shipped `ColumnFourBoard` (014); WASM behavior matches 012.
3. No-extraction-record invariant -> manual review (FOUNDATIONS §4/§16): MECHANICS.md records the `three_marks`→`column_four` repeated-shape comparison and the explicit decision not to extract to `engine-core`/`game-stdlib`.
4. Admission-evidence invariant -> manual review: GAME-IMPLEMENTATION-ADMISSION.md marks each official-game requirement with evidence; PUBLIC-RELEASE-CHECKLIST.md reviews IP/a11y/no-leak/polish.

## What to Change

### 1. `games/column_four/docs/MECHANICS.md`

From `templates/GAME-MECHANICS.md`: game-local mechanic inventory (coordinates, 7×6 occupancy, gravity/drop, line detection, column actions), action/effect shape, UI/bot pressure, the repeated-shape comparison vs. `three_marks`, the explicit no-extraction decision, and the mechanic-atlas updates required (applied in GAT5COLFOUPUB-018).

### 2. `games/column_four/docs/UI.md`, `AI.md`

UI.md (from `templates/GAME-UI.md`): public goal, layout, board anatomy, column controls, Rust/WASM-owned view/action/preview/effect boundary, visual/IP-safe asset policy, animation, terminal/replay/bot-explanation/accessibility/reduced-motion/responsive behavior, no-leak. AI.md (from `templates/GAME-AI.md`): bot registry (Level 0 + Level 2), determinism, legal-action authority, explanation format, tests, benchmarks, excluded AI/search approaches.

### 3. `games/column_four/docs/GAME-IMPLEMENTATION-ADMISSION.md`, `PUBLIC-RELEASE-CHECKLIST.md`

ADMISSION (from `templates/GAME-IMPLEMENTATION-ADMISSION.md`): the official-admission checklist with evidence per requirement. PUBLIC-RELEASE-CHECKLIST (from `templates/PUBLIC-RELEASE-CHECKLIST.md`): official-game contract, IP/trade-dress, UI polish, accessibility, reduced motion, no-leak, replay, bot explanations, static-bundle behavior, dev/debug boundaries, docs/status surfaces.

## Files to Touch

- `games/column_four/docs/MECHANICS.md` (new)
- `games/column_four/docs/UI.md` (new)
- `games/column_four/docs/AI.md` (new)
- `games/column_four/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)
- `games/column_four/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)

## Out of Scope

- Repo-level status/atlas updates `docs/MECHANIC-ATLAS.md`, `specs/README.md`, `README.md`, `docs/ROADMAP.md`, `progress.md`, `apps/web/README.md` (GAT5COLFOUPUB-018).
- RULES/SOURCES (001), COMPETENT-PLAYER/EVIDENCE-PACK (007), BENCHMARKS (011), RULE-COVERAGE (013) — already authored.

## Acceptance Criteria

### Tests That Must Pass

1. `for f in MECHANICS UI AI GAME-IMPLEMENTATION-ADMISSION PUBLIC-RELEASE-CHECKLIST; do test -f games/column_four/docs/$f.md || echo MISSING $f; done` — prints nothing.
2. `node scripts/check-doc-links.mjs` — doc links resolve.
3. Manual review: MECHANICS records the no-extraction decision; ADMISSION shows passing evidence; docs match shipped surfaces (008/011/012/014).

### Invariants

1. The full `docs/OFFICIAL-GAME-CONTRACT.md` doc set exists and is filled (not template shells).
2. MECHANICS.md records the second-use pressure and the explicit decision not to extract to `engine-core`/`game-stdlib`.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (test -f / check-doc-links) plus manual review against the shipped surfaces named in Assumption Reassessment.`

### Commands

1. `for f in MECHANICS UI AI GAME-IMPLEMENTATION-ADMISSION PUBLIC-RELEASE-CHECKLIST; do test -f games/column_four/docs/$f.md || echo MISSING $f; done`
2. `node scripts/check-doc-links.mjs`
3. `cargo run -p rule-coverage -- --game column_four` — confirms RULE-COVERAGE/BENCHMARKS doc consistency unaffected by the new docs.
