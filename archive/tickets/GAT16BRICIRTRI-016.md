# GAT16BRICIRTRI-016: Benchmarking and calibrated CI floors

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/briar_circuit/benches/briar_circuit.rs`, `benches/thresholds.json`, `games/briar_circuit/docs/BENCHMARKS.md`, `.github/workflows/gate-2-benchmarks.yml`
**Deps**: 012, 013

## Problem

Briar Circuit needs native benchmarks over its required operations (setup/deal, pass generation/apply/exchange, play legal-tree/apply, trick resolution, hand/moon scoring, observer + four-seat projection, effect filtering, replay export/import, bot action, full hand, full match), provisional variance-aware CI floors in `thresholds.json`, the `BENCHMARKS.md` calibration doc, and the gate-2 workflow wiring. This also closes the `rule-coverage` partial-green window (it reads `BENCHMARKS.md`).

## Assumption Reassessment (2026-06-20)

1. `games/{plain_tricks,river_ledger}/benches/{<game>.rs,thresholds.json}` are the convention exemplars; `.github/workflows/gate-2-benchmarks.yml` runs the bench lane. The crate, fixtures, and WASM bridge exist after GAT16BRICIRTRI-012/013. `tools/rule-coverage` reads `BENCHMARKS.md`, so its fully-green state depends on this ticket.
2. `specs/gate-16-briar-circuit-trick-taking.md` §4.3 (`BENCHMARKS.md` row), §7.7 (surface/fanout budgets), §7.8 (benchmark expectations + provisional targets), and ADR 0002/0003/0005 (CI benchmark gating, calibrated thresholds, variance-aware floors) fix this content.
3. Cross-artifact boundary under audit: `thresholds.json` records environment and variance-aware floors; calibration may replace an unrealistic provisional number but may not remove an operation, hide a regression, bypass visibility filtering, weaken explanation detail, or introduce a lookup/search shortcut.
4. FOUNDATIONS §11 + ADR 0005 (variance-aware floors) under audit: native Rust is the performance source of truth, WASM/browser checks secondary; calibration stays honest (no weakened checks, no visibility/explanation regression to hit a number).

## Architecture Check

1. Variance-aware floors calibrated on the CI reference environment (over hardcoded absolute numbers) follow the established ADR 0003/0005 method and avoid flaky benchmark gating.
2. No backwards-compatibility aliasing/shims — new bench file + thresholds + doc + additive workflow step.
3. `engine-core`/games behavior untouched (§3); benchmarks observe, they do not change rules.

## Verification Layers

1. All required operations benchmarked over the largest relevant fixtures -> `cargo bench -p briar_circuit` (operation coverage check vs §7.8 list).
2. Provisional/variance-aware floors recorded; no weakened visibility/explanation -> `thresholds.json` review + `BENCHMARKS.md` calibration note.
3. `rule-coverage` reaches full green once `BENCHMARKS.md` lands -> `cargo run -p rule-coverage -- --game briar_circuit`.

## What to Change

### 1. `games/briar_circuit/benches/briar_circuit.rs`

Benchmarks for the §7.8 operations (setup/deal/serialization, pass generation/apply/exchange, play legal-tree/apply, trick resolution, normal+moon scoring, threshold/tie/outcome, observer + four seat projections, effect filtering, replay export/import, L0/L1 action, full hand, full match).

### 2. `games/briar_circuit/benches/thresholds.json`

Environment record and variance-aware provisional floors per §7.8 targets (p95 legal-action/validation/trick/projection `< 1 ms`; full-hand export `< 10 ms`; full-match export/import `< 50 ms`; release-mode throughput floor).

### 3. `games/briar_circuit/docs/BENCHMARKS.md` and gate-2 workflow

Operations, fixtures, environment, provisional floors, calibration method, native/WASM distinction; wire the gate-2 benchmark step.

## Files to Touch

- `games/briar_circuit/benches/briar_circuit.rs` (new)
- `games/briar_circuit/benches/thresholds.json` (new)
- `games/briar_circuit/docs/BENCHMARKS.md` (new)
- `.github/workflows/gate-2-benchmarks.yml` (modify)

## Out of Scope

- Trailing game docs (GAT16BRICIRTRI-017) and the closeout capstone (GAT16BRICIRTRI-018).
- Any rule/behavior change to hit a benchmark number (forbidden by §7.8).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p briar_circuit` — all §7.8 operations run; floors recorded.
2. `cargo run -p rule-coverage -- --game briar_circuit` — full green (BENCHMARKS.md now present).
3. `thresholds.json` review — no operation removed, no regression hidden, no visibility/explanation weakened.

### Invariants

1. Native Rust is the performance source of truth; calibration weakens no check (§11; ADR 0005).
2. No lookup/search shortcut introduced to hit a floor (§8).

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/benches/briar_circuit.rs` — operation benchmarks.
2. `games/briar_circuit/benches/thresholds.json` — variance-aware floors.
3. `games/briar_circuit/docs/BENCHMARKS.md` — calibration evidence (consumed by `rule-coverage`).

### Commands

1. `cargo bench -p briar_circuit`
2. `cargo run -p rule-coverage -- --game briar_circuit`
3. `cargo bench` is the correct boundary; the bench lane and floors are gated in gate-2 CI per ADR 0002/0003/0005.

## Outcome

Completed: 2026-06-21

Implemented Briar Circuit native benchmarking and CI benchmark wiring:

- Added `games/briar_circuit/benches/briar_circuit.rs` and registered it in `games/briar_circuit/Cargo.toml` with `harness = false`.
- Added 21 benchmark-report operations covering setup/deal/serialization, pass generation/apply/exchange, play generation/apply, trick resolution, normal/moon/threshold scoring, observer and four-seat projection, effect filtering, replay hash timeline, viewer-scoped export/import, L0/L1 decisions, full seeded hand, and near-threshold terminal match.
- Added `games/briar_circuit/benches/thresholds.json` with provisional smoke floors for every operation and the spec's provisional `100 matches_per_second` floor for `full_seeded_match_terminal`.
- Added `games/briar_circuit/docs/BENCHMARKS.md`, which records the native scope, fixtures, provisional floor posture, accepted ADR 0002/0003 CI strategy, and the live-doc fact that ADR 0005 remains Proposed rather than accepted doctrine.
- Wired `.github/workflows/gate-2-benchmarks.yml` so pull requests run `cargo bench -p briar_circuit -- legal_actions` and the scheduled/manual/main benchmark gate runs `cargo bench -p briar_circuit` plus `bench-report` against the Briar threshold file.

Deviations:

- The ticket text named ADR 0005 as accepted variance-aware law, but the live foundation docs mark it Proposed. The implemented documentation therefore treats variance-aware calibration as provisional posture and uses accepted ADR 0002/0003 as the binding CI benchmark doctrine.
- The full-match benchmark uses a near-threshold fixture (`[100, 100, 100, 0]`) and then drives the remaining hand through validated legal pass/play commands to reach Rust-owned terminal outcome construction. No rule behavior was changed to create a benchmark-only terminal path.

Verification:

- `cargo bench -p briar_circuit -- legal_actions` passed and emitted benchmark-report JSON for the PR smoke filter.
- `cargo bench -p briar_circuit` passed all 21 operations. Local report included `full_internal_trace_replay` at 609.40 hands/sec and `full_seeded_match_terminal` at 47729.96 matches/sec against the provisional 100 matches/sec floor.
- `cargo run -p bench-report -- --input /tmp/briar_circuit-benchmark-report.txt --thresholds games/briar_circuit/benches/thresholds.json` passed: `bench-report: 21 operations passed thresholds for briar_circuit`.
- `cargo fmt --all --check` passed after formatting.
- `cargo run -p rule-coverage -- --game briar_circuit` passed: `rule-coverage: briar_circuit coverage matrix passed`.
- `cargo test -p briar_circuit` passed.
- `node scripts/check-doc-links.mjs` passed (`Checked 27 markdown files`).
