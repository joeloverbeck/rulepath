# GAT151RIVLED-020: Closeout capstone

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None (docs/status-only: public-release checklist, catalog/index reconciliation, status flips, archival; exercises the prior tickets' acceptance suite without modifying their files)
**Deps**: GAT151RIVLED-018, GAT151RIVLED-019

## Problem

Close out Gate 15.1: re-run the full acceptance command suite as exit evidence, complete the public-release checklist for the v2 delta, reconcile the catalog/README surfaces, flip statuses (`specs/README.md` index row and the spec's own Status to `Done`), confirm the mechanic-atlas debt register stays empty, and archive the completed spec. This ticket exercises the pipeline the prior tickets composed; it adds no production logic.

## Assumption Reassessment (2026-06-20)

1. Code/docs: behavior (GAT151RIVLED-003–012), evidence (-015–018), and per-game docs (-019) have landed. `specs/README.md` Order-6 row currently reads `_(seed; unwritten)_` / `Not started`; the spec Status reads `Not started`. `docs/MECHANIC-ATLAS.md` §10A debt register is empty. `apps/web/README.md` carries the intro catalog / Smoke Layers lists checked by `scripts/check-catalog-docs.mjs`; the wasm catalog const already lists river_ledger (no const change this gate).
2. Docs: spec §6 (three exit rows mapping ROADMAP §15.1), §7.1 (acceptance command suite), §10 (closeout surfaces: `PUBLIC-RELEASE-CHECKLIST.md`, `specs/README.md`, `apps/web/README.md`, atlas, archival per `docs/archival-workflow.md`).
3. Cross-artifact boundary under audit: the status/index/catalog reconciliation surfaces — `specs/README.md`, the spec file Status, `apps/web/README.md`, `PUBLIC-RELEASE-CHECKLIST.md`, and `docs/archival-workflow.md` — none of which are upstream production files this ticket modifies for behavior.
4. (§6 acceptance gate) Restate: the gate flips to `Done` only when all three §6 exit rows have current evidence and the §10 closeout surfaces are reconciled. Confirm the full §7.1 command suite passes before the status flip.

## Architecture Check

1. A single closeout capstone gated on the full evidence chain keeps the `Done`-flip honest: status changes only after the acceptance suite passes, not piecemeal.
2. No backwards-compatibility shims; this reconciles status/catalog surfaces only.
3. No behavior or `engine-core` change; archival follows the canonical `docs/archival-workflow.md`.

## Verification Layers

1. Full acceptance command suite green -> §7.1 command run (cargo fmt/clippy/build/test, simulate, replay-check, fixture-check, rule-coverage, boundary-check, doc/catalog/player-rules checks, web smokes, bench).
2. Catalog/README surfaces consistent -> `check-catalog-docs.mjs` clean.
3. Atlas debt register empty + spec/index `Done` -> grep-proof on `docs/MECHANIC-ATLAS.md` §10A and `specs/README.md` Order-6 row.
4. Public-release checklist complete -> manual checklist review against `PUBLIC-RELEASE-CHECKLIST.md`.

## What to Change

### 1. Public-release checklist + exit verification

Re-run all universal/N-seat/no-leak/rules/UI/bot/benchmark/source/IP/browser closeout rows in `PUBLIC-RELEASE-CHECKLIST.md`; record the §7.1 command transcript as exit evidence for the three §6 rows.

### 2. Status reconciliation + archival

Reconcile `apps/web/README.md` catalog/Smoke-Layers lists; flip `specs/README.md` Order-6 row and the spec Status to `Done`; confirm `docs/MECHANIC-ATLAS.md` §10A stays empty; archive the completed spec per `docs/archival-workflow.md`, leaving the living index pointing at the archived path; admit Gate 16.

## Files to Touch

- `games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md` (modify)
- `apps/web/README.md` (modify)
- `specs/README.md` (modify)
- `specs/gate-15-1-river-ledger-all-in-side-pots.md` (modify; Status → `Done`, then archive per workflow)
- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- Any production behavior, test, trace, or per-game rules/mechanics doc change (owned by GAT151RIVLED-003–019).
- Editing upstream tickets' files — this capstone exercises them, it does not modify them.

## Acceptance Criteria

### Tests That Must Pass

1. The full spec §7.1 command suite passes (cargo fmt/clippy/build/test, `simulate`, `replay-check --all`, `fixture-check`, `rule-coverage`, `boundary-check.sh`, `check-doc-links`/`check-catalog-docs`/`check-player-rules`/`check-outcome-explanations`, web `smoke:wasm`/`build`/`smoke:ui`/`smoke:effects`/`smoke:e2e`, `cargo bench -p river_ledger`).
2. `node scripts/check-catalog-docs.mjs` — catalog/README surfaces reconciled.
3. `node scripts/check-doc-links.mjs` — index/spec/archive links resolve after the `Done`-flip and archival.

### Invariants

1. The gate flips to `Done` only after all three §6 exit rows have current passing evidence.
2. `docs/MECHANIC-ATLAS.md` §10A open promotion-debt register remains empty; Gate 16 is admitted only after `Done`.

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + verification capstone; it reconciles the named status/catalog surfaces and exercises the prior tickets' acceptance suite, adding no test file.`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace && cargo test --workspace`
2. `cargo run -p simulate -- --game river_ledger --games 1000 && cargo run -p replay-check -- --game river_ledger --all && cargo run -p fixture-check -- --game river_ledger && cargo run -p rule-coverage -- --game river_ledger`
3. `node scripts/check-catalog-docs.mjs && node scripts/check-doc-links.mjs && node scripts/check-player-rules.mjs` — the closeout boundary is the full acceptance suite plus the doc/catalog gates, exercising every prior ticket without modifying it.

## Outcome

Completed 2026-06-20. Ran the full Gate 15.1 acceptance suite, reconciled the
public release checklist, web README River Ledger audit rows, mechanic-atlas
debt note, `specs/README.md` tracker row, and spec status/outcome, then archived
the completed Gate 15.1 spec. Gate 16 is admitted as the next not-started public
scaling unit.

The only source change was a mechanical Clippy fix in
`games/river_ledger/src/state.rs`: the setup ledger construction loop now uses
`starting_stacks.iter().copied().enumerate()` instead of range-indexing the
same vector. Behavior is unchanged; this was required for the mandated
`cargo clippy --workspace --all-targets -- -D warnings` lane.

Verification passed:

1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo build --workspace`
4. `cargo test --workspace`
5. `cargo test -p river_ledger`
6. `cargo test -p wasm-api`
7. `cargo run -p simulate -- --game river_ledger --games 1000`
8. `cargo run -p replay-check -- --game river_ledger --all`
9. `cargo run -p fixture-check -- --game river_ledger`
10. `cargo run -p rule-coverage -- --game river_ledger`
11. `bash scripts/boundary-check.sh`
12. `node scripts/check-doc-links.mjs`
13. `node scripts/check-catalog-docs.mjs`
14. `node scripts/check-player-rules.mjs`
15. `node scripts/check-outcome-explanations.mjs`
16. `npm --prefix apps/web ci` (passed with the existing one low-severity npm audit notice)
17. `npm --prefix apps/web run smoke:wasm`
18. `npm --prefix apps/web run build`
19. `npm --prefix apps/web run smoke:ui`
20. `npm --prefix apps/web run smoke:effects`
21. `npm --prefix apps/web run smoke:e2e`
22. `cargo bench -p river_ledger`
