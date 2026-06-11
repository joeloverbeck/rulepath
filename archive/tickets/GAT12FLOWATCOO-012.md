# GAT12FLOWATCOO-012: Benchmarks, thresholds, BENCHMARKS.md, and gate-2 CI

**Status**: ACCEPTED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/flood_watch/benches/flood_watch.rs`, `benches/thresholds.json`, `games/flood_watch/docs/BENCHMARKS.md`; `tools/bench-report/src/main.rs` (modify — register game); `.github/workflows/gate-2-benchmarks.yml` (modify — add bench smoke step)
**Deps**: GAT12FLOWATCOO-011

## Problem

`flood_watch` needs the benchmark identity list (including the environment-phase automation hot path and a full cooperative random playout), non-blocking smoke-floor thresholds matching the `masked_claims` pattern, the tool-validated `BENCHMARKS.md` doc, registration in `bench-report`, and a gate-2 CI smoke step. Thresholds start as `baseline_pending_non_blocking` with a named calibration follow-up under ADR 0002/0003/0005.

## Assumption Reassessment (2026-06-11)

1. `games/masked_claims/benches/` is the verified exemplar: `masked_claims.rs` + `thresholds.json`, every entry `"rationale_class": "baseline_pending_non_blocking"`. `tools/bench-report/src/main.rs` registers games via a `resolve_game()` match (verified, `"masked_claims" => RegisteredGame { thresholds_path: "games/masked_claims/benches/thresholds.json" }`). `.github/workflows/gate-2-benchmarks.yml` enumerates games by crate name (verified: `cargo bench -p masked_claims` + `bench-report --thresholds games/masked_claims/benches/thresholds.json`).
2. The spec (§Deliverables "Benchmarks", §Implementation reference "Benchmark operations", Work-breakdown item 10, FOUNDATIONS-alignment "Benchmark ADRs 0002/0003/0005") fixes the operation list: `legal_actions_action_phase`, `validate_action`, `apply_bail`, `apply_reinforce`, `apply_end_turn_environment_phase` (automation hot path), `project_public_view_midgame`, `state_hash_terminal`, `public_export_timeline`, `level1_bot_decision`, `random_playout`. Initial thresholds are non-blocking smoke floors; PR smoke stays non-gating.
3. Cross-artifact boundary under audit: `BENCHMARKS.md` is a tool-validated doc — `tools/rule-coverage` reads it alongside `RULES.md`/`RULE-COVERAGE.md`, so the rule-coverage registration (GAT12FLOWATCOO-015) depends on this `BENCHMARKS.md` existing to go fully green (flagged as a cross-ticket dependency; GAT12FLOWATCOO-015 `Deps` this ticket). `thresholds.json` is consumed by `bench-report`; its schema must match the existing `rationale_class` shape.
4. FOUNDATIONS §6 (benchmarks are part of the done contract) and §11 (benchmarks cover the change) motivate this ticket; ADR 0002 (gating lanes), 0003 (calibrated thresholds), 0005 (variance-aware floors, Proposed) govern the threshold posture. No §13 trigger fires — smoke floors are the established pattern.
5. Enforcement surface: not a no-leak or determinism surface, but `public_export_timeline` and `state_hash_terminal` benches exercise the redacted-export and deterministic-hash paths; the bench harness must not introduce nondeterminism into those measured paths.

## Architecture Check

1. Co-landing `BENCHMARKS.md` + `thresholds.json` + `bench-report` registration + the gate-2 step keeps the benchmark CI lane self-contained and avoids a red `bench-report` window — the lane is green the moment it is wired.
2. No backwards-compatibility aliasing/shims; additive bench harness + additive `resolve_game` arm.
3. `engine-core` untouched; benches live under `games/flood_watch/benches`.

## Verification Layers

1. Benchmark identity list present -> benchmark check: `cargo bench -p flood_watch` runs all ten named operations including the automation hot path and full random playout.
2. Non-blocking smoke floors -> schema validation: `thresholds.json` entries use `baseline_pending_non_blocking`; `bench-report` parses and reports without gating.
3. Tool-validated doc -> `BENCHMARKS.md` present and consistent (consumed by rule-coverage in GAT12FLOWATCOO-015).
4. CI registration -> the `gate-2-benchmarks.yml` step runs `cargo bench -p flood_watch` + `bench-report` against the thresholds.

## What to Change

### 1. Bench harness + thresholds

Author `games/flood_watch/benches/flood_watch.rs` covering the ten operations and `benches/thresholds.json` with `baseline_pending_non_blocking` entries. Name the variance-aware calibration follow-up under ADR 0002/0003/0005 in the doc/handoff.

### 2. `BENCHMARKS.md`

Instantiate from `templates/GAME-BENCHMARKS.md`; record the operation list, the smoke-floor posture, and the calibration follow-up.

### 3. Tool + CI registration

Add the `flood_watch` arm to `tools/bench-report/src/main.rs` `resolve_game()` (thresholds path). Add a `flood_watch bench smoke` step to `.github/workflows/gate-2-benchmarks.yml` mirroring the `masked_claims` step.

## Files to Touch

- `games/flood_watch/benches/flood_watch.rs` (new)
- `games/flood_watch/benches/thresholds.json` (new)
- `games/flood_watch/docs/BENCHMARKS.md` (new)
- `tools/bench-report/src/main.rs` (modify — `resolve_game` arm)
- `.github/workflows/gate-2-benchmarks.yml` (modify — bench smoke step)

## Out of Scope

- The other native tools (`simulate`/`replay-check`/`fixture-check`/`rule-coverage`) and gate-1 CI (GAT12FLOWATCOO-015).
- Benchmark calibration to real floors (named follow-up; out of this gate).
- `COMPETENT-PLAYER.md` balance evidence (GAT12FLOWATCOO-013).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p flood_watch` runs the full benchmark identity list.
2. `bench-report` parses `games/flood_watch/benches/thresholds.json` and reports non-blocking smoke floors.
3. The `gate-2-benchmarks.yml` `flood_watch` step is present and consistent with the `masked_claims` precedent.

### Invariants

1. Thresholds are non-blocking smoke floors (`baseline_pending_non_blocking`); PR smoke stays non-gating.
2. The measured export/hash benches introduce no nondeterminism into those paths.

## Test Plan

### New/Modified Tests

1. `games/flood_watch/benches/flood_watch.rs` — the ten benchmark operations.
2. `games/flood_watch/benches/thresholds.json` — smoke-floor thresholds.

### Commands

1. `cargo bench -p flood_watch -- legal_actions` (smoke subset)
2. `cargo bench -p flood_watch | tee /tmp/flood_watch-bench.txt && cargo run -p bench-report -- --input /tmp/flood_watch-bench.txt --thresholds games/flood_watch/benches/thresholds.json`
3. The narrower bench-filter smoke is the correct PR boundary (full calibration is a named non-blocking follow-up per ADR 0002/0003/0005).

## Outcome

Accepted on 2026-06-11. Added the Flood Watch native benchmark harness,
non-blocking smoke-floor thresholds, `BENCHMARKS.md`, `bench-report`
registration, and gate-2 benchmark workflow entries. The harness covers all
ten required operations, including environment automation, public export,
terminal hashing, Level 1 bot decisions, and a legal cooperative playout. The
threshold posture is `baseline_pending_non_blocking` pending variance-aware
calibration under ADR 0002/0003/0005.

Verification:

1. `cargo bench -p flood_watch -- legal_actions`
2. `cargo bench -p flood_watch > /tmp/flood_watch-bench-full.txt`
3. `cargo run -p bench-report -- --input /tmp/flood_watch-bench-full.txt --thresholds games/flood_watch/benches/thresholds.json`
4. `cargo run -p bench-report -- --game flood_watch --input /tmp/flood_watch-bench-full.txt`
5. `cargo fmt --all --check`
6. `cargo clippy -p flood_watch --all-targets -- -D warnings`
