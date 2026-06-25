# GAT19MELLEDFIV-017: Benchmarks and BENCHMARKS.md

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — deterministic evidence + docs (`games/meldfall_ledger/benches/*`, `docs/BENCHMARKS.md`)
**Deps**: GAT19MELLEDFIV-014, GAT19MELLEDFIV-016

## Problem

Gate 19 needs native benchmarks and `BENCHMARKS.md`: per-seat-count profiles (2/4/6), action-fanout, view/export for all viewers, replay export/import, and L0/L1 bot decision budgets, plus the large-discard-tail and large-public-tableau profiles. `BENCHMARKS.md` is a tool-validated doc (`tools/rule-coverage` reads it), so it co-lands with the benches.

## Assumption Reassessment (2026-06-25)

1. `games/blackglass_pact/benches/blackglass_pact.rs` + `thresholds.json` are the shape (confirmed during reassessment); the game behavior (setup/actions/views/exports/bots) exists from GAT19MELLEDFIV-004–016. `benches/` is created here.
2. Spec §7.6 (benchmark expectations table) defines the eight profiles and provisional targets; §4.2 (BENCHMARKS.md row) defines the doc.
3. Cross-artifact: `tools/rule-coverage` reads `BENCHMARKS.md` (along with `RULES.md`/`RULE-COVERAGE.md`), so this doc must land before `rule-coverage` goes fully green (GAT19MELLEDFIV-018 `Deps` this ticket).
4. FOUNDATIONS §11 / ADR 0002/0003/0005: benchmark thresholds follow the CI benchmark-gating/calibration ADRs; provisional targets may be calibrated under that process but the profiles must not be removed.
5. FOUNDATIONS §11 determinism: benches run setup + N random-legal actions deterministically; they measure, they do not change behavior.

## Architecture Check

1. Co-landing `BENCHMARKS.md` with the benches keeps the rule-coverage-validated doc in sync with the actual profiles, avoiding a red `rule-coverage` window from a missing doc.
2. No backwards-compatibility shims.
3. `engine-core` untouched; benches are crate-local test harness.

## Verification Layers

1. All eight benchmark profiles run -> `cargo bench -p meldfall_ledger`.
2. `BENCHMARKS.md` documents each profile + provisional target -> grep for each profile name; `node scripts/check-doc-links.mjs`.
3. Thresholds conform to the CI benchmark lane format -> `thresholds.json` schema parse.

## What to Change

### 1. `benches/meldfall_ledger.rs` + `thresholds.json`

Profiles: `native_2p_short_round`, `native_4p_default`, `native_6p_large_surface`, `large_discard_tail`, `large_public_tableau`, `replay_export_import`, `l0_bot_decision`, `l1_bot_decision` (per spec §7.6), with provisional thresholds in `thresholds.json`.

### 2. `docs/BENCHMARKS.md`

Seat-count profiles, max-6-seat/action-surface budgets, bot-vs-bot simulation budgets, and the provisional thresholds, formatted for `tools/rule-coverage` consumption.

## Files to Touch

- `games/meldfall_ledger/benches/meldfall_ledger.rs` (new)
- `games/meldfall_ledger/benches/thresholds.json` (new)
- `games/meldfall_ledger/docs/BENCHMARKS.md` (new)
- `games/meldfall_ledger/Cargo.toml` (modify; created by GAT19MELLEDFIV-003 — add `[[bench]]`)

## Out of Scope

- `RULE-COVERAGE.md` + `rule-coverage` registration (GAT19MELLEDFIV-018).
- CI workflow wiring beyond the bench profiles (the CI receipt surfaces land in GAT19MELLEDFIV-021/022).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p meldfall_ledger` runs all eight profiles (smoke run acceptable per crate bench filters).
2. `BENCHMARKS.md` names every profile and provisional target.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. Benchmark profiles are not removed; thresholds follow the CI calibration ADRs (ADR 0002/0003/0005).
2. Benches measure without changing behavior (FOUNDATIONS §11 determinism).

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/benches/meldfall_ledger.rs` — eight benchmark profiles.
2. `games/meldfall_ledger/benches/thresholds.json` — provisional thresholds.

### Commands

1. `cargo bench -p meldfall_ledger`
2. `node scripts/check-doc-links.mjs`
3. `rule-coverage` consumption of `BENCHMARKS.md` is verified in GAT19MELLEDFIV-018 (its registration ticket).

## Outcome

Completed: 2026-06-26

What changed:

1. Added `games/meldfall_ledger/benches/meldfall_ledger.rs` and registered it in `games/meldfall_ledger/Cargo.toml` as a custom `harness = false` native benchmark target.
2. Added `games/meldfall_ledger/benches/thresholds.json` with all eight required profile names and provisional smoke thresholds.
3. Added `games/meldfall_ledger/docs/BENCHMARKS.md` documenting the seat-count profiles, large discard-tail/tableau profiles, replay export/import profile, L0 bot profile, and L1-not-admitted profile.

Deviations:

1. `l1_bot_decision` is implemented as a status-check profile over `not_admitted_pending_strategy_evidence`, not as a strategic decision benchmark, because Gate 19 has no admitted Level 1 policy.
2. Bench thresholds are baseline smoke floors pending repeated CI-runner calibration under ADR 0002/0003/0005. They are not tuned p95 performance gates yet.

Verification:

1. `cargo bench -p meldfall_ledger` passed and ran all eight profiles: `native_2p_short_round`, `native_4p_default`, `native_6p_large_surface`, `large_discard_tail`, `large_public_tableau`, `replay_export_import`, `l0_bot_decision`, and `l1_bot_decision`.
2. `cargo run -p bench-report -- --input /tmp/meldfall_ledger_bench_report.txt --thresholds games/meldfall_ledger/benches/thresholds.json` passed (`bench-report: 8 operations passed thresholds for meldfall_ledger`).
3. `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).
4. `rg -n "native_2p_short_round|native_4p_default|native_6p_large_surface|large_discard_tail|large_public_tableau|replay_export_import|l0_bot_decision|l1_bot_decision|thresholds\\.json|not_admitted_pending_strategy_evidence" games/meldfall_ledger/docs/BENCHMARKS.md games/meldfall_ledger/benches/thresholds.json` confirmed the doc and threshold profile names.
5. `cargo fmt --all --check` passed.
