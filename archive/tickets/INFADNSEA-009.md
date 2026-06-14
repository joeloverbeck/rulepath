# INFADNSEA-009: Closeout — doc amendments + spec/index Done-flip (capstone)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None
**Deps**: INFADNSEA-003, INFADNSEA-004, INFADNSEA-006, INFADNSEA-008

## Problem

The spec defers its doc/law amendments to closeout (§10: "acceptance-clause amendments only", "WB9 only") because the amended doctrine describes machinery that must exist first. Once Infra A–D have landed (the leaf set 003/004/006/008 transitively covers 001–008), this capstone lifts the deferred acceptance-clause amendments into the contract/area docs, updates the web Shell-Surface doc, re-runs the spec's exit criteria as evidence, and flips the spec + `specs/README.md` index row to `Done`. Per the infrastructure-spec closeout-deferred-docs exception, the single capstone carries both the doc lift and the `Done`-flip.

## Assumption Reassessment (2026-06-14)

1. The amendment targets exist: `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (§13 simulator-summary deferral note to retire), `docs/TESTING-REPLAY-BENCHMARKING.md`, `docs/UI-INTERACTION.md`, `apps/web/README.md` (Shell Surface section), `specs/README.md` (the `15A` row, currently `Planned`). The spec is `specs/infra-a-d-n-seat-public-infrastructure.md` (Status `Planned`).
2. Grounded in `specs/infra-a-d-n-seat-public-infrastructure.md` §10 (Documentation updates required) + §6 (Exit criteria) + WB9; the index row was created at spec authoring (already `Planned`), so this is a create-then-modify on `specs/README.md` only in the status-flip sense.
3. Cross-artifact boundary under audit: the contract/area docs + web README + the `specs/README.md` index + the spec's own Status — all must land atomically once 001–008 ship, and the amendments are acceptance-clause only (no doctrine change that would need an ADR per the §10 constraint and FOUNDATIONS "supersede only by accepted ADR").

## Architecture Check

1. A single closeout capstone is cleaner than scattering doc edits across the implementation tickets: the amended doctrine describes machinery (seat frame, harness, seat-keyed summaries) that must exist coherently before the clauses are true, so one atomic lift avoids a window where docs describe absent machinery.
2. No backwards-compat shim: amendments are additive acceptance clauses; the `Done`-flip replaces `Planned`, not aliased.
3. No `engine-core`/`game-stdlib` impact; docs/status only. No FOUNDATIONS principle is amended (acceptance clauses only), so no ADR is triggered.

## Verification Layers

1. Exit criteria pass -> re-run the spec §6 exit commands (the leaf tickets' acceptance commands) and record pass/fail as evidence; flip `Done` only on green.
2. Doc amendments landed + links resolve -> `node scripts/check-doc-links.mjs` and grep-proofs for each new acceptance clause.
3. Catalog/boundary gates clean -> `node scripts/check-catalog-docs.mjs` and `bash scripts/boundary-check.sh`.
4. Index/spec status consistency -> grep `specs/README.md` `15A` row and the spec Header both read `Done`.

## What to Change

### 1. Lift the deferred acceptance-clause amendments

Apply the §10 amendments: `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md §13` (note the simulator-summary generalization is delivered, retiring the "later Infra B spec" deferral); `docs/TESTING-REPLAY-BENCHMARKING.md` (name the reusable N-player no-leak harness as a verification surface); `docs/UI-INTERACTION.md` (acceptance clause for the shared multi-seat shell frame); `apps/web/README.md` Shell Surface section (the seat frame).

### 2. Flip spec + index to Done

After the §6 exit commands pass with recorded evidence, set the spec Header Status and the `specs/README.md` `15A` row to `Done` with the evidence summary.

## Files to Touch

- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (modify)
- `docs/TESTING-REPLAY-BENCHMARKING.md` (modify)
- `docs/UI-INTERACTION.md` (modify)
- `apps/web/README.md` (modify)
- `specs/README.md` (modify)
- `specs/infra-a-d-n-seat-public-infrastructure.md` (modify — Status + Outcome/evidence)

## Out of Scope

- Any production logic — this capstone exercises 001–008 and reconciles docs/status; it modifies no implementation surface.
- Any doctrine change requiring an ADR (amendments are acceptance clauses only, per spec §10).
- Gate 15 work (the successor unit).

## Acceptance Criteria

### Tests That Must Pass

1. The spec §6 exit commands (the union of the leaf tickets' acceptance commands: `cargo test --workspace`, `cargo run -p simulate …`, `npm --prefix apps/web run smoke:ui|smoke:e2e`, harness tests) all pass and are recorded as evidence.
2. `node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && bash scripts/boundary-check.sh` — all pass.
3. `specs/README.md` `15A` row and the spec Header both read `Done` with evidence.

### Invariants

1. The `Done`-flip happens only after the §6 exit criteria pass with recorded, re-run evidence (not copied from memory).
2. Doc amendments are acceptance-clause additions only — no FOUNDATIONS/contract doctrine is changed without an ADR.

## Test Plan

### New/Modified Tests

1. `None — documentation/status-only ticket; verification is command-based and the implementation tests are owned by INFADNSEA-001–008 and re-run here as exit evidence.`

### Commands

1. `node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && bash scripts/boundary-check.sh`
2. `cargo test --workspace && npm --prefix apps/web run smoke:e2e`

## Outcome

Completed on 2026-06-14.

Lifted the deferred WB9 acceptance-clause amendments:

- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` now treats seat-keyed simulator summaries as delivered infrastructure and forbids fixed two-seat scalar summary regression.
- `docs/TESTING-REPLAY-BENCHMARKING.md` names the reusable `wasm-api` pairwise no-leak harness as an accepted verification surface.
- `docs/UI-INTERACTION.md` records the shared web shell seat frame as the default multi-seat orientation acceptance surface.
- `apps/web/README.md` documents `SeatFrame` in the shell surface and the High Card Duel seat-frame no-leak E2E coverage.
- `specs/README.md` and the spec header were flipped to `Done`; the completed spec carries final Outcome evidence.

Verification:

- `cargo test -p wasm-api`
- `cargo test --workspace`
- `npm --prefix apps/web run smoke:e2e`
- `cargo run -p simulate -- --game race_to_n --games 1000`
- `npm --prefix apps/web run smoke:ui`
- `node scripts/check-doc-links.mjs`
- `node scripts/check-catalog-docs.mjs`
- `bash scripts/boundary-check.sh`
