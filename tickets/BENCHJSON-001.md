# BENCHJSON-001: Restore Gate 2 benchmark JSON schema conformance for masked_claims and frontier_control

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `games/masked_claims/benches/masked_claims.rs` and `games/frontier_control/benches/frontier_control.rs` (benchmark harnesses only; no `engine-core`/`game-stdlib`/rules/trace/schema-of-record changes)
**Deps**: none

## Problem

The "Gate 2 benchmarks" CI workflow (`.github/workflows/gate-2-benchmarks.yml`, `bench-gate` job) has been hard-failing on every `main`/scheduled run (confirmed at merge `e06bdb0`, and on prior runs back through at least 2026-06-22). The failure is **not** a throughput regression: `bench-report` rejects the benchmark JSON of two games at *parse* time, before any threshold is compared.

From the CI log of run `28032927121` (13 of 15 games pass; these 2 fail):

- **masked_claims** → `missing array operations`
- **frontier_control** → `missing field build_profile`

`tools/bench-report/src/main.rs` `Report::parse` (line 209) and `validate_report` (the `for (field, value) in [("build_profile", …), ("command", …), ("os", …), ("rust_version", …), ("hardware_environment_notes", …)]` loop) require a report object with: `schema_version, game_id, rules_version, data_version, engine_version, build_profile, command, os, rust_version, hardware_environment_notes`, plus a non-empty `operations` array whose entries each carry `operation_name`, `unit`, `current_value`.

Both games predate the schema hardening that made those five metadata fields required and standardized the `operations`/`current_value` keys. Their `benchmark_report_json` functions emit an older, divergent shape:

- `masked_claims` (masked_claims.rs:383–403): emits `"results":[…]` (must be `"operations"`), per-op key `"value"` (must be `"current_value"`), and omits all five metadata fields.
- `frontier_control` (frontier_control.rs:342–356): emits `"operations"` correctly and per-op `operation_name`/`unit`/`current_value` correctly, but omits all five metadata fields.

Because ADR 0002 keeps `bench-report` off the pull-request lane (shared runners cannot validly gate throughput), this schema drift is invisible at PR time and only surfaces on the post-merge `main` gate — which is why it merged and has stayed red. (Prevention of future drift is BENCHJSON-002, out of scope here.)

## Assumption Reassessment (2026-06-23)

1. **`bench-report` required schema is as stated.** Verified in `tools/bench-report/src/main.rs`: `Report` struct fields (lines 154–166), `Report::parse` required-field extraction (lines 209–225), non-empty `operations` guard (lines 226–228), and `validate_report`'s metadata non-empty loop. `Operation` requires `operation_name`/`unit`/`current_value` (lines 169–173, 201–205).
2. **The two `thresholds.json` files are already conformant** and need no change: `games/masked_claims/benches/thresholds.json` and `games/frontier_control/benches/thresholds.json` both carry `schema_version/game_id/rules_version/data_version/engine_version` and a `thresholds[]` with `operation_name/unit/threshold/rationale_class/rationale`. `masked_claims`'s `OPERATIONS` units already include the `_per_second` suffix (masked_claims.rs:28–40), matching the threshold units, so once the report key is `operations` and the value key is `current_value`, `validate_report`'s unit comparison passes.
3. **Cross-artifact boundary under audit:** the benchmark-report JSON contract between each game's `[[bench]]` harness (the producer) and `tools/bench-report` (the consumer). The schema-of-record is `tools/bench-report/src/main.rs`'s `Report`/`Operation` structs plus `docs/TESTING-REPLAY-BENCHMARKING.md` and ADR 0003. This change makes the two stale producers conform to that existing contract; it does not change the contract.
4. **Conforming reference pattern exists and is the target shape.** `games/briar_circuit/benches/briar_circuit.rs` defines `const BUILD_PROFILE: &str = "bench"` (line 20) and a `ReportMetadata` struct + `ReportMetadata::new()` (lines 745–758) populating `command` (`env::args().join(" ")`), `os` (`format!("{} {}", env::consts::OS, env::consts::ARCH)`), `rust_version` (shells out to `rustc`), and `hardware_environment_notes` (a fixed descriptive string); `benchmark_report_json` (lines 776–813) emits all required fields plus `operations`. The 13 passing games follow this shape.
5. **Adjacent contradiction classification:** the lack of a PR-time schema check that let this drift in undetected is a *separate* structural gap, deferred to BENCHJSON-002 (future cleanup as its own ticket), not a consequence of this ticket.
6. **Required constants already present.** Both target files already reference `GAME_ID`, `RULES_VERSION_LABEL`, `ENGINE_VERSION`, `DATA_VERSION`, `REPORT_SCHEMA_VERSION` in their current `benchmark_report_json`, so only `BUILD_PROFILE` and the metadata-construction code are additive.

## Architecture Check

1. **Fix the producers, not the consumer.** 13 of 15 games already satisfy the `bench-report` schema; the two outliers are the defect. Relaxing `bench-report` to accept the old shape would weaken a provenance/reproducibility contract that ADR 0003 and `docs/TESTING-REPLAY-BENCHMARKING.md` deliberately enforce (build profile, host, toolchain, command are recorded so a threshold number is interpretable) and would diverge the two games further from the other 13. Aligning the two emitters converges the codebase on one schema.
2. **No backwards-compatibility shim.** The old `results`/`value` keys are replaced outright, not aliased; `bench-report` is not taught to accept both shapes.
3. **`engine-core` untouched** — changes are confined to two `games/*/benches/*.rs` harnesses; no mechanic nouns enter `engine-core`, and `game-stdlib` is not modified. Benchmark provenance is test infrastructure, not product behavior: rules, legal-action generation, state transitions, scoring, RNG, views, and traces are unchanged.

## Verification Layers

1. **masked_claims report parses + passes thresholds** -> benchmark check: `cargo bench -p masked_claims | cargo run -p bench-report -- --input /dev/stdin --thresholds games/masked_claims/benches/thresholds.json` prints `bench-report: N operations passed thresholds for masked_claims` (use the `tee`-to-file form from the workflow if `/dev/stdin` is awkward).
2. **frontier_control report parses + passes thresholds** -> benchmark check: same invocation for `frontier_control`.
3. **Schema-of-record conformance (metadata present + non-empty)** -> schema/serialization validation: emitted JSON contains `build_profile`, `command`, `os`, `rust_version`, `hardware_environment_notes` (all non-empty) and an `operations` array with per-op `operation_name`/`unit`/`current_value`.
4. **Full Gate 2 gate is green** -> CI/full-pipeline: the `bench-gate` step in `.github/workflows/gate-2-benchmarks.yml` exits 0 (all 15 games print a pass line; no `missing array operations` / `missing field build_profile`).

## What to Change

### 1. masked_claims benchmark JSON emitter (`games/masked_claims/benches/masked_claims.rs`)

- Adopt the conforming pattern from `briar_circuit.rs`: add `const BUILD_PROFILE: &str = "bench";` and a `ReportMetadata` struct + `ReportMetadata::new()` (command via `env::args`, os via `env::consts::OS`/`ARCH`, rust_version via a `rustc --version` invocation, a fixed `hardware_environment_notes` string), plus the needed `use std::env;` / `use std::process::Command;`.
- Rewrite `benchmark_report_json` (lines 383–403) to emit the top-level object with `schema_version, game_id, rules_version, data_version, engine_version, build_profile, command, os, rust_version, hardware_environment_notes, operations` — renaming the array key `results` → `operations`.
- In the per-op object (lines 387–395), rename `"value"` → `"current_value"`. Keep `operation_name`/`unit` as-is (units already carry `_per_second`). `iterations`/`elapsed_ms`/`threshold`/`pass` may stay (ignored by `bench-report`) or be trimmed for parity with the reference; prefer matching the reference shape.
- Update `main` (lines 67–69) to pass the constructed metadata into `benchmark_report_json`.

### 2. frontier_control benchmark JSON emitter (`games/frontier_control/benches/frontier_control.rs`)

- Same `BUILD_PROFILE` + `ReportMetadata` additions and imports.
- Rewrite `benchmark_report_json` (lines 342–356) to add the five missing metadata fields to the top-level object. The existing `operations` array and `bench_result_json` per-op shape (`operation_name`/`unit` (`{unit}_per_second`)/`current_value`) already conform and stay.
- Thread the metadata through `main` (around line 69) into `benchmark_report_json`.

## Files to Touch

- `games/masked_claims/benches/masked_claims.rs` (modify)
- `games/frontier_control/benches/frontier_control.rs` (modify)

## Out of Scope

- Any change to `tools/bench-report` (the consumer/schema-of-record).
- Any threshold value, `rationale_class`, lane, or budget change (no gating-doctrine change; ADR 0002/0003 untouched).
- The other 13 games' benches.
- PR-lane drift prevention (BENCHJSON-002).
- Factoring the per-game emitters into a shared bench-support helper (a larger refactor; would be its own ticket).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p masked_claims | tee /tmp/masked_claims-benchmark-report.txt && cargo run -p bench-report -- --input /tmp/masked_claims-benchmark-report.txt --thresholds games/masked_claims/benches/thresholds.json` → exit 0, prints `bench-report: N operations passed thresholds for masked_claims`.
2. `cargo bench -p frontier_control | tee /tmp/frontier_control-benchmark-report.txt && cargo run -p bench-report -- --input /tmp/frontier_control-benchmark-report.txt --thresholds games/frontier_control/benches/thresholds.json` → exit 0, prints the frontier_control pass line.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace` → clean (Gate 0 hygiene, since bench harnesses are part of `--all-targets`).

### Invariants

1. Every game's benchmark JSON satisfies the `bench-report` `Report`/`Operation` schema (all five metadata fields present and non-empty; `operations` array non-empty with `operation_name`/`unit`/`current_value` per op).
2. No threshold, lane, or rationale changes: the fix restores parseability only; pass/fail is still decided by the unchanged `thresholds.json` files.

## Test Plan

### New/Modified Tests

1. `None — benchmark-harness fix; verification is command-based via cargo bench + bench-report, which is the schema's only consumer.` The two emitters are inside `[[bench]]` harnesses (not the game lib), so the proof surface is running them through `bench-report`, exactly as the CI gate does (BENCHJSON-002 adds the standing PR-time guard).

### Commands

1. `cargo bench -p masked_claims | tee /tmp/masked_claims-benchmark-report.txt && cargo run -p bench-report -- --input /tmp/masked_claims-benchmark-report.txt --thresholds games/masked_claims/benches/thresholds.json`
2. `cargo bench -p frontier_control | tee /tmp/frontier_control-benchmark-report.txt && cargo run -p bench-report -- --input /tmp/frontier_control-benchmark-report.txt --thresholds games/frontier_control/benches/thresholds.json`
3. Narrower per-game `bench-report` invocation is the correct boundary because `bench-report` is the sole consumer of the benchmark JSON contract and reproduces exactly what the `bench-gate` CI step does per game.
