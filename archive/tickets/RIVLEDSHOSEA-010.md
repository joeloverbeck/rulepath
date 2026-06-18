# RIVLEDSHOSEA-010: Documentation reconciliation and spec closeout

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None (docs/status-only)
**Deps**: RIVLEDSHOSEA-004, RIVLEDSHOSEA-007, RIVLEDSHOSEA-008, RIVLEDSHOSEA-009

## Problem

Once the correctness, active-seat, viewer, and card tickets land, the River Ledger docs and spec status must be reconciled atomically: `UI.md` and `RULE-COVERAGE.md` updated, a narrow `RULES.md` clarification (no rule change), the `specs/README.md` index row added and flipped to `Done`, and the spec's Outcome section populated with evidence. This is the docs-reconciliation + exit-verification capstone for the spec (§12 / §14 / §17): it exercises the prior tickets' acceptance suite and records completion, adding no production logic.

## Assumption Reassessment (2026-06-18)

1. `games/river_ledger/docs/UI.md`, `docs/RULE-COVERAGE.md`, and `docs/RULES.md` exist; `RULES.md` carries the `RL-*` IDs verbatim (confirmed). `RULE-COVERAGE.md` rows for the showdown/split/remainder/UI/visibility IDs were updated for new evidence in RIVLEDSHOSEA-004 — this ticket reconciles the remaining `UI.md` narrative and any residual coverage rows, avoiding overlap with 004's trace-row edits.
2. `specs/README.md` currently has no row for `river-ledger-showdown-and-seat-presentation-fixes` (the two archived River Ledger UX specs are present and `Done`); the spec file's Status is `PLANNED` and its §20 Outcome reads "Pending implementation". Spec §17 assigns both the `Planned` row creation and the `Done` flip to closeout. Confirmed.
3. Shared boundary under audit: the doc/status surfaces (`UI.md`, `RULE-COVERAGE.md`, `RULES.md`, `specs/README.md`, the spec Outcome) and the exit-criteria evidence they must agree with. End state: every §14 exit criterion has recorded evidence and no stale `Planned`/`PLANNED` status remains.
4. FOUNDATIONS §11 (evidence coverage) + the spec's authority order: docs reconcile to what shipped; `RULES.md` may clarify (button order selects remainder recipients but does not rank tied winners) but must not change a rule oracle or any stable `RL-*` ID. Restated before trusting the spec.

## Architecture Check

1. A single trailing reconciliation ticket lets the doc/status surfaces land atomically once all upstream surfaces exist coherently (status-line/index counts, cross-surface UI narrative), avoiding a staleness window from per-ticket doc edits.
2. No shim: no compatibility note is added; `RULES.md` clarification keeps all stable IDs.
3. Docs/status-only; no `engine-core`/Rust/TS change. The `Done` flip is gated on exit-criteria evidence passing.

## Verification Layers

1. Exit-criteria evidence present -> manual review mapping each §14 row to its recorded evidence (seed-`10018`/seed-`31` tests, 3/4/5/6 e2e, cross-catalog no-leak, card bounding-box) in the spec Outcome.
2. Docs agree with shipped surfaces -> grep-proof: `UI.md` describes internal-vs-public labels, canonical-vs-remainder order, one resolved-showdown source, active-match projection, and suit-word containment; `RULES.md` retains all `RL-*` IDs.
3. Status reconciled -> grep-proof that the `specs/README.md` row reads `Done` and the spec Status is `Done` with no residual `Planned`/`PLANNED`.
4. Doc gates green -> `node scripts/check-doc-links.mjs`, `node scripts/check-catalog-docs.mjs`, `node scripts/check-player-rules.mjs` pass.

## What to Change

### 1. River Ledger docs

Update `UI.md` to clarify internal seat IDs vs one-based public labels, canonical semantic winner order vs button-order remainder distribution, the one resolved-showdown source, the active-match seat-label projection / match-scoped surface rule, and the full suit-word containment requirement. Reconcile residual `RULE-COVERAGE.md` narrative (not 004's trace rows). Add the narrow `RULES.md` clarification without changing any `RL-*` ID.

### 2. Index + spec closeout

Add the `specs/README.md` row for this spec after the two archived River Ledger UX specs (as `Done` at closeout), flip the spec Status to `Done`, and populate the §20 Outcome with merged ticket range, final SHA, seed-`10018`/seed-`31` test names + output, changed-trace hashes + rationale, 3/4/5/6 e2e evidence, cross-catalog selector/no-leak evidence, card bounding-box/resize evidence, and Gate 0/1/2 command results.

## Files to Touch

- `games/river_ledger/docs/UI.md` (modify)
- `games/river_ledger/docs/RULE-COVERAGE.md` (modify)
- `games/river_ledger/docs/RULES.md` (modify)
- `specs/README.md` (modify)
- `specs/river-ledger-showdown-and-seat-presentation-fixes.md` (modify; Status + §20 Outcome)

## Out of Scope

- Any production logic, test, or trace change (owned by RIVLEDSHOSEA-001..009).
- Changing any `RL-*` rule oracle or stable ID.
- Archiving tickets/spec (follow `docs/archival-workflow.md` separately if the repo convention moves completed specs).

## Acceptance Criteria

### Tests That Must Pass

1. Full Gate 0 (`cargo fmt --all --check`, `clippy -D warnings`, `build`, `test`, `boundary-check.sh`) and the River Ledger Gate 1 set (native, simulate, replay-check, fixture-check, rule-coverage) green.
2. Full `npm --prefix apps/web run smoke:e2e` and the doc/presentation guards (`check-doc-links`, `check-catalog-docs`, `check-player-rules`, `check-presentation-copy`, `check-outcome-explanations`) green.
3. Every §14 exit criterion has recorded evidence in the spec Outcome; no `Planned`/`PLANNED` status remains for this spec.

### Invariants

1. Docs and status agree with the shipped surfaces; no stale public-numbering or capability-as-active-seat description remains.
2. No `RL-*` rule oracle or stable ID changed.

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + verification capstone; it reconciles the named docs/status surfaces and exercises the prior tickets' acceptance suite, adding no test file.`

### Commands

1. `node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && node scripts/check-player-rules.mjs && node scripts/check-presentation-copy.mjs && node scripts/check-outcome-explanations.mjs`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace && cargo test --workspace && bash scripts/boundary-check.sh`
3. `cargo run -p replay-check -- --game river_ledger --all && cargo run -p fixture-check -- --game river_ledger && cargo run -p rule-coverage -- --game river_ledger && npm --prefix apps/web run smoke:e2e`

## Outcome

Completed: 2026-06-18

What changed:
- Reconciled `games/river_ledger/docs/UI.md` with the shipped Rust-authored public label contract, active-match seat-label projection, canonical winner versus remainder order, pairwise viewer no-leak evidence, and full suit-word card containment.
- Clarified `games/river_ledger/docs/RULES.md` without changing any stable `RL-*` ID: button order assigns split remainders only and does not redefine tied winner order.
- Updated `games/river_ledger/docs/RULE-COVERAGE.md` evidence notes for remainder order, active-seat UI metadata, showdown presentation, and browser no-leak coverage.
- Added the completed spec row to `specs/README.md` and flipped `specs/river-ledger-showdown-and-seat-presentation-fixes.md` to `Done` with a filled Outcome section.

Deviations from original plan:
- None for the docs/status scope. The spec Outcome records earlier implementation deviations from RIVLEDSHOSEA-008 and RIVLEDSHOSEA-009.

Verification:
- `node scripts/check-doc-links.mjs` — passed.
- `node scripts/check-catalog-docs.mjs` — passed.
- `node scripts/check-player-rules.mjs` — passed.
- `node scripts/check-presentation-copy.mjs` — passed.
- `node scripts/check-outcome-explanations.mjs` — passed.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace` — passed.
- `cargo test --workspace` — passed.
- `bash scripts/boundary-check.sh` — passed.
- `cargo test -p river_ledger` — passed.
- `cargo run -p simulate -- --game river_ledger --games 1000 --seat-count 6 --action-cap 48` — passed.
- `cargo run -p replay-check -- --game river_ledger --all` — passed.
- `cargo run -p fixture-check -- --game river_ledger` — passed.
- `cargo run -p rule-coverage -- --game river_ledger` — passed.
- `npm --prefix apps/web ci` — passed; npm reported one low-severity audit item.
- `npm --prefix apps/web run smoke:wasm` — passed.
- `npm --prefix apps/web run build` — passed.
- `npm --prefix apps/web run smoke:ui` — passed.
- `npm --prefix apps/web run smoke:effects` — passed.
- `npm --prefix apps/web run smoke:e2e` — passed.
- `cargo bench -p river_ledger` — passed.
