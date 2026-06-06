# GAT4THRMARBOA-015: Three Marks cross-cutting game docs + mechanic atlas update

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (docs) — new `games/three_marks/docs/{RULE-COVERAGE,MECHANICS,AI,UI,BENCHMARKS,GAME-IMPLEMENTATION-ADMISSION}.md`; `docs/MECHANIC-ATLAS.md`, `docs/SOURCES.md`
**Deps**: GAT4THRMARBOA-003, GAT4THRMARBOA-004, GAT4THRMARBOA-005, GAT4THRMARBOA-006, GAT4THRMARBOA-007, GAT4THRMARBOA-008, GAT4THRMARBOA-009, GAT4THRMARBOA-011, GAT4THRMARBOA-012

## Problem

Three Marks needs the remaining official-game documentation set — rule-coverage matrix (mapping rules→tests/traces, consumed by `tools/rule-coverage`), mechanic inventory, AI/UI/benchmark notes, and the admission checklist — plus the repo-level mechanic-atlas and source-index updates. These cite implemented test/bench/UI surfaces, so they land after the implementation tickets and must stay coherent with them.

## Assumption Reassessment (2026-06-06)

1. `games/race_to_n/docs/{RULE-COVERAGE,MECHANICS,AI,UI,BENCHMARKS,GAME-IMPLEMENTATION-ADMISSION}.md` are the mirrors. `tools/rule-coverage/src/main.rs` validates the `RULE-COVERAGE.md` matrix format (status tokens like `code`/`covered-by-trace`/`not-applicable`/`unsupported`). `docs/MECHANIC-ATLAS.md` **already lists `three_marks`** at lines 167-168 ("fixed 2D occupancy", "simple line/pattern detection", labelled `repeated-shape candidate after Stage 3`) — this ticket edits those rows in place, not adds duplicates (reassessment finding M5). Verified the atlas rows and the race_to_n doc set.
2. Spec `specs/gate-4-three-marks-board-smoke.md` §16 (doc set), §17 (mechanic-atlas / primitive-pressure, no extraction), §5.2 (`docs/SOURCES.md` or game-level source discoverability — game-level SOURCES.md landed in GAT4THRMARBOA-001), §21 (docs acceptance rows). RULES.md/SOURCES.md from 001; tests/traces/benches/bots/UI from 003-012.
3. Cross-artifact boundary under audit: the doc-governed contract in `docs/OFFICIAL-GAME-CONTRACT.md` (§4-§7 doc requirements) and the `tools/rule-coverage` consumer of `RULE-COVERAGE.md` (the matrix must satisfy the tool's format, generalized in GAT4THRMARBOA-014).
4. FOUNDATIONS §4 (`game-stdlib` is earned; the atlas records primitive pressure) and §6 (official games are evidence-heavy) motivate this ticket: the docs must record fixed-2D-occupancy and line/pattern detection as first-use local-only mechanics with no extraction, and prove the full evidence set exists.
5. Third-use mechanic hard-gate surface (§4): the `docs/MECHANIC-ATLAS.md` primitive-pressure ledger is the gate — name it. Three Marks is the *first* implemented use of fixed-2D-occupancy and simple line/pattern detection; the atlas update records status local-only / first-use and explicitly defers the extraction decision (`column_four` second comparison point; `directional_flip`/Stage 4 the real pressure) — no helper is promoted to `game-stdlib` and no board noun enters `engine-core`.

## Architecture Check

1. A single cross-cutting docs ticket landing after implementation avoids a staleness window where docs cite not-yet-existing tests/benches/UI; it keeps the rule-coverage matrix, mechanic inventory, and admission checklist mutually coherent. Alternative (co-locating each doc with its implementing ticket) risks status-count/coverage drift across surfaces and is rejected for the matrix/atlas/admission docs.
2. No backwards-compatibility aliasing/shims — new docs; atlas/SOURCES edited in place.
3. The docs assert (and the boundary review in GAT4THRMARBOA-016 proves) that `engine-core` stays noun-free and no `game-stdlib` helper was extracted from one game.

## Verification Layers

1. Rule-coverage-matrix invariant -> schema/coverage validation (`rule-coverage --game three_marks` accepts `RULE-COVERAGE.md`; every rule maps to a real test/trace name).
2. Doc-completeness invariant -> manual review against `docs/OFFICIAL-GAME-CONTRACT.md` §4-§7 (MECHANICS/AI/UI/BENCHMARKS/ADMISSION cover the required categories).
3. Mechanic-atlas no-extraction invariant -> codebase grep-proof + FOUNDATIONS alignment (§4: atlas rows record `three_marks` first-use local-only; no `game-stdlib` board helper; `bash scripts/boundary-check.sh` clean).
4. Cross-surface coherence invariant -> manual review + grep (doc-cited test/bench/trace/UI names resolve to real files from 003-012).

## What to Change

### 1. `games/three_marks/docs/RULE-COVERAGE.md`

Matrix tying every rule requirement to real test names (rule/property/replay/serialization/bot), golden traces, replay checks, and UI smoke, with explicit not-applicable rows (hidden info, stochastic rule events) — in the format `tools/rule-coverage` validates.

### 2. `games/three_marks/docs/{MECHANICS,AI,UI,BENCHMARKS,GAME-IMPLEMENTATION-ADMISSION}.md`

MECHANICS (fixed 2D positions, occupancy, targeted placement, line/pattern detection, terminal win/draw, semantic effects, board interaction, bots, benchmark pressure — all local-only); AI (Level 0/1 policy, versions, determinism, tie-breaking, explanations, limitations, tests/sims/benches); UI (board behaviour, Rust/TS boundary, accessibility, reduced motion, original SVG/token posture, replay UI, dev-panel boundaries, any deferred setup mode from 010); BENCHMARKS (lane, 300,000 games/sec target + measured results + threshold decision per ADR discipline); ADMISSION (official-game checklist, foundation alignment, IP/source readiness, testing/traces/replay/benchmarks, UI smoke, deferrals).

### 3. `docs/MECHANIC-ATLAS.md` (modify) + `docs/SOURCES.md` (modify)

Edit the existing `three_marks` atlas rows (lines ~167-168) in place to record first implemented use, status local-only, extraction deferred; add a `three_marks` pointer to the repo-level `docs/SOURCES.md` index so source posture is discoverable.

## Files to Touch

- `games/three_marks/docs/RULE-COVERAGE.md` (new)
- `games/three_marks/docs/MECHANICS.md` (new)
- `games/three_marks/docs/AI.md` (new)
- `games/three_marks/docs/UI.md` (new)
- `games/three_marks/docs/BENCHMARKS.md` (new)
- `games/three_marks/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)
- `docs/MECHANIC-ATLAS.md` (modify)
- `docs/SOURCES.md` (modify)

## Out of Scope

- `RULES.md`/`SOURCES.md` game-level prose (GAT4THRMARBOA-001).
- The `tools/rule-coverage` generalization that consumes `RULE-COVERAGE.md` (GAT4THRMARBOA-014).
- Spec Status flip / `specs/README.md` index `Done` flip (GAT4THRMARBOA-016).
- Any helper extraction to `game-stdlib` (forbidden, spec §17).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p rule-coverage -- --game three_marks` — `RULE-COVERAGE.md` validates (depends on GAT4THRMARBOA-014).
2. `node scripts/check-doc-links.mjs` — doc links resolve across the new docs.
3. `bash scripts/boundary-check.sh` and grep prove no `game-stdlib` board-helper extraction and no `engine-core` board noun.

### Invariants

1. Every rule requirement in `RULE-COVERAGE.md` maps to a real test/trace; every doc-cited surface name resolves to a real file.
2. The mechanic atlas records `three_marks` as first-use local-only with extraction deferred; no helper was promoted and no kernel noun added.

## Test Plan

### New/Modified Tests

1. `None — documentation ticket; verification is command-based (`rule-coverage`, `check-doc-links.mjs`, `boundary-check.sh`) plus the manual contract-completeness review named in Assumption Reassessment.`

### Commands

1. `cargo run -p rule-coverage -- --game three_marks`
2. `node scripts/check-doc-links.mjs && bash scripts/boundary-check.sh`
3. The full evidence end-to-end (exit criteria) is the capstone GAT4THRMARBOA-016; doc-validation + link/boundary checks are the correct boundary for the docs diff.
