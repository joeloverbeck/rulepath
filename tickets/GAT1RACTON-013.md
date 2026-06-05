# GAT1RACTON-013: CI wiring for race_to_n checks

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `.github/workflows/ci.yml` is extended with `race_to_n` rule/golden/replay/serialization/quick-sim/UI-smoke/bench-smoke checks.
**Deps**: GAT1RACTON-008, GAT1RACTON-009, GAT1RACTON-010, GAT1RACTON-012

## Problem

Gate 1's evidence must run in CI (TESTING §17; FOUNDATIONS §6). The current
workflow runs Gate 0 smoke only. This ticket extends CI so the `race_to_n`
rule/golden/replay/serialization tests, a quick simulation, the UI smoke, and a
bench smoke run on every PR — keeping the evidence live and drift-loud.

## Assumption Reassessment (2026-06-05)

1. `.github/workflows/ci.yml` has a single `smoke` job ("Gate 0 smoke") running
   `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D
   warnings`, `cargo build --workspace`, `cargo test --workspace`,
   `bash scripts/boundary-check.sh`, `npm --prefix apps/web run smoke:wasm`,
   `npm --prefix apps/web run build`, `node scripts/check-doc-links.mjs`
   (verified by reading the file). `cargo test --workspace` already picks up
   `race_to_n` tests once they exist; this ticket adds the game-specific + quick
   commands not covered by the blanket workspace test.
2. The checks this ticket wires exist after their upstream tickets: replay/golden/
   serialization tests (008), quick simulation (009, `--games 1000` boundary),
   bench smoke (010, a single `cargo bench` case), UI smoke (012, `smoke:ui`).
3. Cross-artifact boundary under audit: CI is the aggregation surface; each added
   step must invoke a command an upstream ticket made runnable (no inventing new
   verification). Steps are added to the existing job (or a new `race_to_n` job)
   keyed to the real commands.
4. FOUNDATIONS §6 (evidence-heavy; CI runs the evidence) and §11 (tests/traces/
   simulations/benchmarks cover the change) motivate this ticket. Golden-trace
   drift must fail CI loudly (TESTING §3).
5. Determinism note: CI runs the deterministic replay/golden tests; the quick-sim
   uses a fixed seed range so failures are reproducible. The full 100k run is the
   exit-criteria command (GAT1RACTON-015), not the per-PR quick-sim (TESTING §17:
   expensive runs may be nightly/manual). No leakage (perfect-info; CI artifacts
   are public-safe, TESTING §18).
6. Schema/contract: this edits a CI workflow (no code/schema). Steps map 1:1 to
   `cargo`/`npm` commands defined by upstream tickets.

## Architecture Check

1. Extending the existing workflow with explicit game-specific steps (quick-sim,
   bench-smoke, UI-smoke) makes the Gate 1 evidence first-class and drift-loud,
   beyond what the blanket `cargo test --workspace` covers. A separate
   `race_to_n` job is optional; reusing the `smoke` job keeps setup (Node, WASM
   target) shared. Alternative (relying solely on `cargo test --workspace`) would
   silently skip sim/bench/UI evidence.
2. No backwards-compatibility shims — steps are added, not aliased.
3. No code touched; `engine-core`/`game-stdlib` unaffected.

## Verification Layers

1. CI runs game evidence -> CI config grep-proof (the workflow invokes the quick-sim,
   bench-smoke, and UI-smoke commands).
2. Drift loudness -> golden trace check (golden-trace test failure fails the job —
   inherited from `cargo test`).
3. Command resolvability -> simulation/CLI run (each added command resolves
   against the post-008/009/010/012 tree; dry-run locally).
4. Quick-sim scoping -> manual review (per-PR uses `--games 1000`; the 100k run is
   reserved for GAT1RACTON-015 / nightly per TESTING §17).

## What to Change

### 1. Extend `.github/workflows/ci.yml`

Add steps (in the existing `smoke` job or a new `race_to_n` job sharing setup):
- quick simulation: `cargo run -p simulate -- --game race_to_n --games 1000`
- bench smoke: a single fast `cargo bench -p race_to_n -- <one-case>` (or `cargo build` of the bench target if a full bench is too slow for PR CI)
- UI smoke: `npm --prefix apps/web run smoke:ui`

Rule/golden/replay/serialization tests are covered by the existing
`cargo test --workspace`; add an explicit comment naming them, or a targeted
`cargo test -p race_to_n` step for legibility.

## Files to Touch

- `.github/workflows/ci.yml` (modify)

## Out of Scope

- Nightly full 100k simulation + full benchmark runs (TESTING §17 — may be a
  separate nightly workflow later; not Gate 1 PR CI).
- Gate 2 drift-loud trace tooling / `bench-report` CI generalization (spec §2).
- Any code change (this is CI config only).

## Acceptance Criteria

### Tests That Must Pass

1. The workflow YAML is valid and the added steps invoke commands that resolve against the post-upstream tree.
2. On a branch with all upstream tickets merged, the CI job passes: rule/golden/replay/serialization tests, quick-sim, bench-smoke, UI-smoke.
3. A deliberately broken golden trace fails the CI job (drift is loud) — verified once, then reverted.

### Invariants

1. Every Gate 1 evidence category (rule/golden/replay/serialization/sim/UI/bench) is exercised by CI (FOUNDATIONS §6; TESTING §17).
2. Per-PR CI uses the quick-sim boundary; the 100k exit-criteria run is not in per-PR CI (TESTING §17).

## Test Plan

### New/Modified Tests

1. `None — CI-configuration ticket; verification is the workflow run itself and the upstream tickets' tests/commands named in Assumption Reassessment.`

### Commands

1. `cargo run -p simulate -- --game race_to_n --games 1000`
2. `cargo test -p race_to_n && npm --prefix apps/web run smoke:ui`
3. The full `--games 100000` run is intentionally excluded from per-PR CI (TESTING §17); it is GAT1RACTON-015's exit-criteria command.
