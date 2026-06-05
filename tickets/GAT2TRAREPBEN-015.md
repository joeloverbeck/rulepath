# GAT2TRAREPBEN-015: Gate 2 capstone — exit-evidence runbook + status flip

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: No — verification-only capstone; the only file mutations are the Gate 2 spec Status flip and the `specs/README.md` index flip after exit evidence passes.
**Deps**: GAT2TRAREPBEN-013, GAT2TRAREPBEN-014

## Problem

Gate 2 is not done until every exit criterion and acceptance-evidence command passes
and the spec/index reflect `Done` truthfully. This capstone runs the full §6/§7
evidence set end-to-end across the tickets built earlier, then flips the spec Status
and the `specs/README.md` index — and only then (spec §11 Sequencing; §10).

## Assumption Reassessment (2026-06-05)

1. After GAT2TRAREPBEN-001–013 land, the real tools exist (replay-check, fixture-check,
   bench-report, seed-reducer, rule-coverage, trace-viewer), traces are Trace Schema v1
   JSON, CI gates on them, and GAT2TRAREPBEN-014 has finalized the docs. The spec file
   `specs/gate-2-trace-replay-benchmark-hardening.md` currently reads Status `Planned`;
   `specs/README.md` lists Gate 2 as `Not started` / "not yet specced" (it is updated to
   `Planned` when the spec lands, then to `Done` here).
2. Spec §6 (Exit criteria), §7 (Acceptance evidence), §10 (index flip), and §11
   (Sequencing: WB12 flips Gate 2 to `Done` only after all exit criteria + acceptance
   evidence pass; Gate 3 stays blocked until Gate 2 is `Done`) define this ticket's
   contract.
3. Cross-artifact boundary under audit: this capstone exercises the pipeline the prior
   tickets composed; it introduces no production logic. Its `Deps` are the leaf set
   {013 (CI gates), 014 (docs)} whose transitive closure covers every implementation
   ticket (002–012).
4. FOUNDATIONS §11 / §12 (exit invariants; no silent waiver): restate that Gate 2 may
   be marked `Done` only when `random_playout` meets the accepted threshold or a formal
   recalibration exists, golden-trace drift fails loudly, fixture-check rejects
   behavior/unknown fields, and no benchmark miss is silently waived.
5. §11 determinism / no-leak across the evidence set: confirm the exit run reproduces
   deterministic hashes, the not-applicable hidden-info/stochastic rationales are
   present, and no hidden information leaks through any trace, tool output, or replay
   export (perfect-information game; verified, not assumed).

## Architecture Check

1. A single verification-only capstone (rather than scattering exit checks across
   implementation tickets) gives one auditable closeout that runs the §7 command set as
   a whole — the status flip is gated on that whole, not on partial evidence.
2. No backwards-compatibility shims; no new production logic.
3. `engine-core` untouched; this ticket runs commands and edits two Markdown/spec files.

## Verification Layers

1. Replay/trace/fixture/coverage gates → golden-trace + schema validation + CLI run:
   replay-check `--all`, fixture-check, rule-coverage all pass; corrupted/malformed
   inputs fail (re-confirmed at closeout).
2. Benchmark gate → benchmark check: `bench-report` hard-fails the accepted Stage-1
   threshold; `random_playout` meets it or is formally recalibrated.
3. Seed reproducer → deterministic replay-hash check: an injected `simulate` failure
   normalizes into a reproducer that `replay-check` replays.
4. Smoke surfaces → simulation/CLI run + manual review: quick simulation, WASM smoke,
   web build, UI smoke, boundary check, and docs link check pass.

## What to Change

### 1. Gate 2 exit-evidence runbook (verification, no production code)

Run the spec §7 acceptance-evidence command set in order and record exact commands +
results in the spec's closeout: `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`,
`cargo test -p race_to_n`, `cargo run -p replay-check -- --game race_to_n --all` (plus
a corrupted-hash fail), `cargo run -p fixture-check -- --game race_to_n` (plus a
malformed fail), `cargo run -p rule-coverage -- --game race_to_n`,
`cargo run -p simulate -- --game race_to_n --games 1000`, the injected-failure
`seed-reducer` reproducer, `cargo bench -p race_to_n` + `bench-report` threshold gate,
the Stage-1 decision, `bash scripts/boundary-check.sh`,
`npm --prefix apps/web run smoke:wasm`, `npm --prefix apps/web run build`,
`npm --prefix apps/web run smoke:ui`, `node scripts/check-doc-links.mjs`, and the
no-YAML / no-private-content / not-applicable-evidence reviews.

### 2. Status flip (only after evidence passes)

Flip `specs/gate-2-trace-replay-benchmark-hardening.md` Status to `Done` with the
closeout evidence table, and flip the `specs/README.md` Gate 2 row to `Done`.

## Files to Touch

- `specs/gate-2-trace-replay-benchmark-hardening.md` (modify) — Status `Planned` → `Done`; populate closeout evidence
- `specs/README.md` (modify) — Gate 2 index row → `Done`

## Out of Scope

- Any production logic or new tool behavior (owned by GAT2TRAREPBEN-002–012).
- Re-running implementation; this ticket only exercises and records.
- Admitting Gate 3 (it remains blocked; the index merely stops blocking once Gate 2 is `Done`).

## Acceptance Criteria

### Tests That Must Pass

1. Every spec §7 acceptance-evidence command passes (or its negative case fails as required); the closeout table records each exact command + result.
2. `random_playout` meets the accepted Stage-1 threshold or a formal recalibration is recorded; no silent waiver.
3. After evidence passes, `grep -n "Done" specs/README.md` shows the Gate 2 row flipped and the spec Status reads `Done`.

### Invariants

1. Gate 2 is marked `Done` only after all exit criteria + acceptance evidence pass (§11 Sequencing).
2. Golden-trace drift fails loudly, fixtures reject behavior/unknown fields, and no benchmark miss is silently waived (§11/§12).

## Test Plan

### New/Modified Tests

1. `None — verification-only capstone; it exercises the pipeline composed by GAT2TRAREPBEN-002–013 and records results. No new production tests are added here.`

### Commands

1. `cargo run -p replay-check -- --game race_to_n --all && cargo run -p fixture-check -- --game race_to_n && cargo run -p rule-coverage -- --game race_to_n`
2. `cargo test --workspace && cargo bench -p race_to_n && cargo run -p bench-report -- --input <report> --thresholds games/race_to_n/benches/thresholds.json`
3. `npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run build && npm --prefix apps/web run smoke:ui && node scripts/check-doc-links.mjs && bash scripts/boundary-check.sh`
