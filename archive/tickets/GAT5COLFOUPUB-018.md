# GAT5COLFOUPUB-018: Column Four capstone — mechanic atlas, status hygiene & acceptance

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — modify `docs/MECHANIC-ATLAS.md`, `specs/README.md`, `README.md`, `docs/ROADMAP.md`, `progress.md`, `apps/web/README.md`, `specs/gate-5-column-four-public-polish.md` (no code surfaces)
**Deps**: 009, 016, 017

## Problem

Gate 5 is admitted only when its exit criteria pass with evidence (spec §20, §B). This capstone records the mechanic-atlas second-use pressure (spec §16), updates every status surface that would otherwise mislead a reader, flips the spec + index to `Done`, and serves as the gate's end-to-end acceptance checkpoint (spec §19, §B exit criteria).

## Assumption Reassessment (2026-06-06)

1. Status surfaces verified present: `specs/README.md` (Gate 5 row currently `column_four — not yet specced — Not started`), `README.md`, `docs/ROADMAP.md`, `progress.md`, `apps/web/README.md` (describes a Gate 3 shell), and `docs/MECHANIC-ATLAS.md` (records `fixed 2D occupancy` / `simple line/pattern detection` as `first-use local-only` for `three_marks`, `later column_four`; `gravity placement into a column` as `column_four` local-only). The spec `specs/gate-5-column-four-public-polish.md` Status is `Planned` (set during reassessment).
2. Spec §16 (atlas/pressure), §19 (status hygiene), and §20/§B (acceptance criteria) define the work: record `three_marks`→`column_four` second-use pressure (fixed 2D occupancy, coordinate placement, simple line detection) plus first-use gravity/column-action/terminal-line-highlight; flip statuses; verify §B exit criteria. All implementation/test/CI tickets (002–017) must have landed.
3. Cross-artifact boundary under audit: the mechanic-atlas pressure-ledger contract (`docs/MECHANIC-ATLAS.md`) and the `specs/README.md` index lifecycle (`Planned → Done`). This ticket records evidence and flips status; it changes no game behavior.
4. FOUNDATIONS §4 (`game-stdlib` is earned) and §16 of the spec (no extraction this gate) motivate the atlas update: pressure is recorded for a later gate's decision, not acted on.
5. The third-use mechanic hard gate (§4) is the enforcement surface under audit: `column_four` is the **second** use of fixed-2D-occupancy / line-detection, so the hard gate does not fire — the atlas must record the second-use pressure and explicitly defer extraction to a later gate (after `directional_flip`), confirming no premature promotion to `engine-core`/`game-stdlib` occurred.

## Architecture Check

1. Consolidating atlas + status + Done-flip + acceptance into one trailing capstone keeps the completion narrative atomic and gated on all prior tickets — cleaner than scattering status flips across implementation tickets (which would flip `Done` before exit criteria pass). The Done-flip defaults to the capstone per the §Ticket-shapes rule.
2. No backwards-compatibility aliasing/shims — status/doc edits only.
3. No code surfaces; `engine-core`/`game-stdlib` untouched. The atlas explicitly records no-extraction (FOUNDATIONS §4).

## Verification Layers

1. Atlas-pressure invariant -> codebase grep-proof: `docs/MECHANIC-ATLAS.md` records `column_four` second-use for fixed 2D occupancy + line detection and first-use gravity/column-action, with the explicit no-extraction decision.
2. Status-hygiene invariant -> codebase grep-proof: `specs/README.md` Gate 5 row reads `Done` with a link; `README.md`/`docs/ROADMAP.md`/`progress.md`/`apps/web/README.md` no longer present a pre-Gate-5 state; spec Status is `Done`.
3. Exit-criteria invariant -> simulation/CLI + UI smoke + benchmark check: the spec §B exit criteria are exercised across the gate's distributed acceptance surfaces (simulate/replay-check/fixture-check/rule-coverage from 013, wasm smoke from 012, browser/a11y/no-leak smoke from 015, benches from 011, boundary-check from 016).
4. Boundary invariant -> FOUNDATIONS alignment check (§3): `bash scripts/boundary-check.sh` green — no mechanic noun entered `engine-core`.

## What to Change

### 1. `docs/MECHANIC-ATLAS.md`

Update the ledger rows: `fixed 2D occupancy` and `simple line/pattern detection` move to second-use (`three_marks` + `column_four`) recorded-local-only; `gravity placement into a column`, `column actions`, and `terminal line highlighting` recorded as first/again-use; state the explicit decision to defer extraction past `directional_flip` (Gate 6).

### 2. Status surfaces

Flip `specs/README.md` Gate 5 row to `Done` with a link to the spec; flip the spec's own Status to `Done`; update `README.md` (available games include Column Four), `docs/ROADMAP.md` (Gate 5 status), `progress.md` (Gate 5 completion evidence), and `apps/web/README.md` (available games + smoke coverage, no longer a Gate 3-only description).

### 3. Acceptance verification

Run the gate's distributed acceptance surfaces and record the §B exit-criteria results (each ROADMAP exit line → passing evidence) in `progress.md`.

## Files to Touch

- `docs/MECHANIC-ATLAS.md` (modify)
- `specs/README.md` (modify)
- `README.md` (modify)
- `docs/ROADMAP.md` (modify)
- `progress.md` (modify)
- `apps/web/README.md` (modify)
- `specs/gate-5-column-four-public-polish.md` (modify)

## Out of Scope

- Any production code, game docs (GAT5COLFOUPUB-017), or CI workflow (016) — this ticket only records pressure, flips status, and verifies acceptance.
- Extraction of any board/grid/cell/line/gravity primitive (deferred past Gate 6, FOUNDATIONS §4).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "column_four" docs/MECHANIC-ATLAS.md && grep -n "Done" specs/README.md | grep -i "gate 5\|column_four"` — atlas pressure recorded and index flipped.
2. Full gate acceptance: `cargo test --workspace`, `cargo run -p simulate -- --game column_four --games 1000`, `cargo run -p replay-check -- --game column_four --all`, `cargo run -p fixture-check -- --game column_four`, `cargo run -p rule-coverage -- --game column_four`, `npm --prefix apps/web run smoke:wasm`, `npm --prefix apps/web run smoke:e2e`, `bash scripts/boundary-check.sh`, `node scripts/check-doc-links.mjs` — all pass.
3. `grep -nE "Status\*\*: Done|Status: Done" specs/gate-5-column-four-public-polish.md` — spec flipped to Done.

### Invariants

1. No status surface remains misleadingly stuck at a pre-Gate-5 state; the spec/index read `Done` only because §B exit criteria pass with evidence.
2. The mechanic atlas records second-use pressure with no extraction into `engine-core`/`game-stdlib`.

## Test Plan

### New/Modified Tests

1. `None — capstone/status ticket; verification re-runs the full gate pipeline (no new production logic) and records the §B exit-criteria evidence.`

### Commands

1. `cargo test --workspace && bash scripts/boundary-check.sh && node scripts/check-doc-links.mjs`
2. `cargo run -p replay-check -- --game column_four --all && cargo run -p fixture-check -- --game column_four && cargo run -p rule-coverage -- --game column_four && cargo run -p simulate -- --game column_four --games 1000`
3. `npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:e2e`
