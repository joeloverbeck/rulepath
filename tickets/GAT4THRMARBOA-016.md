# GAT4THRMARBOA-016: Gate 4 capstone — exit criteria, boundary review, CI wiring, status flip

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (CI + docs) — `.github/workflows/gate-1-game-smoke.yml`, `specs/gate-4-three-marks-board-smoke.md`, `specs/README.md`
**Deps**: GAT4THRMARBOA-008, GAT4THRMARBOA-013, GAT4THRMARBOA-014, GAT4THRMARBOA-015

## Problem

Gate 4 is accepted only when every ROADMAP Gate 4 exit clause is proven end-to-end, the boundary review confirms no architecture law was crossed, Race to N is non-regressed, the CI gate exercises Three Marks, and the gate is recorded `Done`. This capstone exercises the pipeline the prior tickets composed, wires the `three_marks` CI steps, and flips the spec/index status — it introduces no new game logic.

## Assumption Reassessment (2026-06-06)

1. `.github/workflows/gate-1-game-smoke.yml` runs the four CLI checks + a `race_to_n UI smoke` only for `race_to_n` (steps at lines 30-51); the four native tools now accept `--game three_marks` (GAT4THRMARBOA-014) and the web `smoke:e2e` chains the Three Marks browser smoke (013). The spec `specs/gate-4-three-marks-board-smoke.md` Status is `Planned` and `specs/README.md:30` lists Gate 4 `Planned` (flipped from `Not started` during the in-session `/reassess-spec`). Verified the workflow steps and both status lines.
2. Spec §20 (exit criteria mapped row-for-row to ROADMAP Gate 4), §21 (acceptance evidence), §22 (FOUNDATIONS/boundary alignment), Appendix E (boundary review checklist), and `docs/ROADMAP.md` §6 (Gate 4 exit clauses). The `specs/README.md` lifecycle flips to `Done` only after exit criteria pass with evidence.
3. Cross-artifact boundary under audit: the exit-criteria contract spanning every upstream ticket (002-015) — the capstone is the leaf set whose transitive `Deps` cover the full chain (008 benchmarks, 013 UI smoke, 014 CLI tools, 015 docs/atlas each head distinct branches).
4. FOUNDATIONS §11 (universal acceptance invariants), §12 (no stop condition crossed), and §3 (kernel noun-free) motivate this ticket: the boundary review proves Rust behaviour authority, TypeScript-no-legality, noun-free `engine-core`, no `game-stdlib` extraction, deterministic replay/hash, viewer-safe no-leak, and IP cleanliness for the whole gate.
5. Boundary-review enforcement surfaces (§3/§4/§11): `scripts/boundary-check.sh` (kernel noun-free), the `game-stdlib` no-extraction review, the no-leak visibility tests, and the deterministic replay/hash checks are the surfaces — name them. The capstone runs them aggregate-green and records the Appendix E result; it changes the spec/index *status* only (a documentation state flip), introducing no rule logic and no schema change.

## Architecture Check

1. A single capstone that enumerates the exit criteria as test sub-cases and gates the `Done` flip on aggregate evidence is cleaner than scattering completion claims across tickets — it gives one reviewable acceptance surface. It is a capstone-double: it also ships the CI workflow config that makes the gate re-runnable.
2. No backwards-compatibility aliasing/shims — CI steps added; status lines flipped; no production logic added.
3. The review confirms `engine-core` stayed noun-free and no `game-stdlib` helper was extracted (re-running `scripts/boundary-check.sh` + grep), upholding §3/§4.

## Verification Layers

1. Occupied-never-legal exit invariant -> rule/property test + golden trace + UI/WASM smoke (rule tests, occupied-diagnostic trace, board inert smoke, WASM rejection).
2. Win/draw-covered exit invariant -> rule test + golden trace + replay-hash check + UI smoke (4 line kinds + draw; win/draw traces; replay projection; win-highlight/draw smoke).
3. Bots-exist exit invariant -> bot legality check + benchmark check (Level 0/1 tests, explanations, latency benches).
4. UI-pleasant/accessible exit invariant -> UI smoke + manual review (board-first render, a11y, reduced motion, dev-panel secondary per §15.7 + Appendix D).
5. Recorded-not-extracted exit invariant -> codebase grep-proof + FOUNDATIONS alignment (`boundary-check.sh` clean; atlas records first-use local-only; no kernel noun, no helper extraction).
6. Non-regression + CI invariant -> simulation/CLI run (Race to N tests/smoke green; `gate-1-game-smoke.yml` runs the `three_marks` CLI + UI steps).

## What to Change

### 1. `.github/workflows/gate-1-game-smoke.yml`

Add `three_marks` steps mirroring the `race_to_n` ones: `simulate --game three_marks --games 1000`, `replay-check --game three_marks --all`, `fixture-check --game three_marks`, `rule-coverage --game three_marks`, and ensure the web smoke job runs the Three Marks browser smoke (via `smoke:e2e`). Do not broaden CI beyond Gate 4 needs.

### 2. Boundary review + exit evidence (runbook)

Execute the Appendix E boundary-review checklist and the §20 exit-criteria matrix end-to-end (commands below), recording pass/fail. This is an implementer runbook plus the CI-runnable assertions; any failure routes back to the owning ticket, not a silent waiver.

### 3. Status flip

On aggregate-green evidence: flip `specs/gate-4-three-marks-board-smoke.md` Status to `Done` and populate its Acceptance-evidence/Outcome with the re-run command results; flip the `specs/README.md` Gate 4 index row from `Planned` to `Done`.

## Files to Touch

- `.github/workflows/gate-1-game-smoke.yml` (modify)
- `specs/gate-4-three-marks-board-smoke.md` (modify)
- `specs/README.md` (modify)

## Out of Scope

- Any production game/UI/tool logic (delivered in 002-014) — the capstone exercises, it does not implement.
- `tools/trace-viewer`/`tools/seed-reducer` wiring (deferred, spec §25).
- Gate 5 (`column_four`) admission (next gate).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace && bash scripts/boundary-check.sh && node scripts/check-doc-links.mjs` — Rust suite, boundary, and doc links green.
2. `cargo run -p simulate -- --game three_marks --games 1000 && cargo run -p replay-check -- --game three_marks --all && cargo run -p fixture-check -- --game three_marks && cargo run -p rule-coverage -- --game three_marks` — all four CLI evidence commands pass.
3. `npm --prefix apps/web run smoke:e2e` — Three Marks browser smoke passes; Race to N smoke non-regressed.

### Invariants

1. Every ROADMAP Gate 4 exit clause (occupied-never-legal, win/draw covered, random+Level 1 bots, pleasant/accessible UI, recorded-not-extracted) has passing evidence; no §12 stop condition was crossed.
2. The spec Status and `specs/README.md` index read `Done` only after the evidence above is green; Race to N is non-regressed.

## Test Plan

### New/Modified Tests

1. `None — capstone/verification ticket; it exercises existing tests/smokes/traces and adds CI config + status flips. New test surfaces live in 002-015.`

### Commands

1. `cargo test --workspace && bash scripts/boundary-check.sh`
2. `cargo run -p simulate -- --game three_marks --games 1000 && cargo run -p replay-check -- --game three_marks --all && cargo run -p fixture-check -- --game three_marks && cargo run -p rule-coverage -- --game three_marks && npm --prefix apps/web run smoke:e2e`
3. These aggregate commands ARE the exit-criteria boundary; per-surface narrower checks live in their owning tickets (002-015).
