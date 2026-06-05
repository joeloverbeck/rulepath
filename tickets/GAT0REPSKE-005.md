# GAT0REPSKE-005: Gate-0 exit verification and index status flip

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — verification capstone plus a `specs/README.md` index status edit.
**Deps**: GAT0REPSKE-004

## Problem

Gate 0 is not done until every exit criterion passes with evidence and the spec index records it. This trailing capstone performs the boundary-review sign-off and the exit-criteria runbook against the integrated tree (crates from 001, WASM/web path from 002, tools from 003, CI from 004), then flips the `specs/README.md` Gate 0 row Status from `Planned` to `Done`. Keeping the index flip in its own trailing ticket ensures the `Done` marker is atomic with passing evidence.

## Assumption Reassessment (2026-06-05)

1. `specs/README.md` exists (verified `test -f` 2026-06-05); the Gate 0 row currently reads Status `Planned` (`specs/README.md:26`). The verifiable surfaces are delivered by upstream tickets 001–004 (declared transitively via `Deps: GAT0REPSKE-004`, whose own `Deps` reach 001–003).
2. Spec §5 (exit criteria), §6 (acceptance evidence), §9 (documentation updates — flip the index to `Done` only after exit criteria pass), and §10 (sequencing / admission rule). The `docs/ROADMAP.md` and `docs/README.md` pointers in §9 already landed at spec authoring (verified `docs/ROADMAP.md:32`, `docs/README.md:37`) and are NOT re-done here.
3. Cross-artifact tie-together: this ticket validates the crate boundary (001), the WASM/web path (002), the tool placeholders (003), and the CI gates (004) against the spec's exit-criteria contract; the only mutated artifact is the `specs/README.md` index row.
4. §3 / §11: the boundary review (`engine-core` noun-free + dependency direction per `docs/ARCHITECTURE.md` §2) and the exit-criteria verification are the FOUNDATIONS acceptance surface for Gate 0; restate them here before trusting the index flip rather than trusting the spec narrative.

## Architecture Check

1. A single trailing verification-and-flip ticket keeps the `Done` marker atomic with passing evidence — cleaner than flipping the index inside an implementation ticket before exit criteria are met.
2. No backwards-compatibility shims; no production logic introduced.
3. `engine-core` is untouched; no `game-stdlib` change; the change is a status-line edit only.

## Verification Layers

1. All §5 exit criteria pass -> simulation/CLI run + manual runbook (CI green from 004 + the WASM-load browser/headless smoke).
2. Boundary review (noun-free + dependency direction) -> codebase grep-proof + schema/serialization validation (mechanic-noun grep + `cargo tree -p engine-core`).
3. Index records `Done` -> codebase grep-proof (`specs/README.md` Gate 0 row reads `Done`).

## What to Change

### 1. Exit-criteria runbook (performed in this ticket)

Run CI (or the local mirror) and confirm each §5 exit criterion: workspace smoke tests run, `apps/web` builds, the placeholder WASM loads, foundation docs are present, and `engine-core` is noun-free with correct dependency direction. The WASM-load browser smoke is a **manual runbook step** — the project has no browser-automation harness yet: load the built shell, confirm the placeholder call returns the expected version/string, and record the result.

### 2. Index status flip

Edit `specs/README.md` to flip the Gate 0 row Status from `Planned` to `Done`.

## Files to Touch

- `specs/README.md` (modify)

## Out of Scope

- Any production code change (all in GAT0REPSKE-001…004).
- Editing `docs/ROADMAP.md` / `docs/README.md` — their pointers already landed, and ROADMAP is law, not a progress tracker (spec §9).
- Admitting Gate 1 / writing the next spec (separate work).

## Acceptance Criteria

### Tests That Must Pass

1. The CI pipeline (GAT0REPSKE-004) is green on the integrated tree.
2. Boundary review passes: the mechanic-noun grep returns 0 matches, and `cargo tree -p engine-core` shows no Rulepath-crate dependency.
3. The WASM-load smoke succeeds (the web shell loads the artifact and the placeholder call returns) per the runbook.

### Invariants

1. `specs/README.md` Gate 0 Status reads `Done` only after acceptance criteria 1–3 hold.
2. `docs/ROADMAP.md` remains unedited for progress (ROADMAP is law; the index tracks progress — spec §9).

## Test Plan

### New/Modified Tests

1. `None — verification-only capstone; verification is command/runbook-based and exercises the pipeline that tickets 001–004 composed.`

### Commands

1. `grep -rniE "board|card|deck|grid|suit|resource|capture" crates/engine-core/src; cargo tree -p engine-core` — boundary review.
2. `cargo test` plus the WASM and `apps/web` build commands — exit-criteria mirror.
3. `grep -n "gate-0-repository-skeleton" specs/README.md` — narrow post-edit proof that the Gate 0 row Status reads `Done`.
