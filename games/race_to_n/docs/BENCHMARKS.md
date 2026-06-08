# race_to_n Benchmarks

Game ID: `race_to_n`

Implemented variant: `race_to_21`

Rules version: `1`

Data/manifest version: `1`

Engine version: `0.1.0`

Benchmark report version: `1`

Prepared by: `Codex`

Last updated: 2026-06-08

## Benchmark doctrine

Native Rust benchmarks are primary for rule, replay, simulation, and bot hot paths. WASM/browser smoke benchmarks are later web-surface evidence and are not part of this native Gate 1 report.

Do not optimize without benchmark evidence. Do not accept regressions without rationale.

## Hardware and environment

| Field | Value |
|---|---|
| machine/device | WSL2 environment on `JOELOVERBECK` |
| CPU | 12th Gen Intel(R) Core(TM) i9-12900K, 12 vCPU exposed to WSL2 |
| RAM | 19 GiB exposed to WSL2 |
| OS/version | Linux 6.6.114.1-microsoft-standard-WSL2 x86_64 |
| Rust version | `rustc 1.93.0 (254b59607 2026-01-19)` |
| Cargo profile | bench |
| target | native |
| browser/version if applicable | not applicable |
| build artifact/hash | bench-profile local artifact from the GAT1RACTON-010 working tree |
| engine version/hash | engine-core `0.1.0`; source tree based on `fe763405ab83882e6936b211ae3d053b632fe7dd` plus this ticket |
| rules version | `1` |
| data/manifest version | `1` |
| benchmark date | 2026-06-05 |
| thermal/power notes | local WSL2 run; no CPU pinning or thermal isolation |

## Commands

| Benchmark/smoke | Command | Environment | Notes |
|---|---|---|---|
| full native benchmark set | `cargo bench -p race_to_n` | native | Runs `games/race_to_n/benches/race_to_n.rs` with `harness = false` and emits a human table plus marked JSON report. |
| single-case smoke | `cargo bench -p race_to_n -- legal_actions` | native | Cargo resolves the same bench target; the custom harness prints the filtered human table plus marked JSON report. |
| threshold gate | `cargo run -p bench-report -- --input <report> --thresholds games/race_to_n/benches/thresholds.json` | native/CI | Hard-fails if any stable operation is below its accepted threshold. |

## Structured report and threshold gate

The custom harness prints a human-readable table and a machine-readable JSON
block between `BEGIN_RACE_TO_N_BENCHMARK_JSON` and
`END_RACE_TO_N_BENCHMARK_JSON`.

The JSON report includes schema, game/rules/data/engine versions, build profile,
command, OS, Rust version, environment caveats, and one row per stable operation:
`operation_name`, `iterations`, `unit`, `current_value`, `threshold`, `pass`,
`rationale_class`, and `known_caveats`.

`games/race_to_n/benches/thresholds.json` is the committed threshold authority.
`tools/bench-report` reads either the marked harness output or the raw JSON block
and hard-fails when a report value is below the committed threshold. CI wires the
full native benchmark report into `bench-report`; this is the Gate 2 benchmark
acceptance surface.

CI runs this gate on two lanes per [ADR 0002](../../../docs/adr/0002-ci-benchmark-gating-lanes.md):
pull requests run a non-gating bench smoke (`cargo bench -p race_to_n -- legal_actions`,
no `bench-report`), while the scheduled / manual / `main`-push lane runs the full
report through `bench-report` and hard-fails. Per
[ADR 0003](../../../docs/adr/0003-ci-calibrated-benchmark-thresholds.md) and
[ADR 0005](../../../docs/adr/0005-variance-aware-ci-benchmark-floors.md), the
committed thresholds are the enforced variance-aware `ubuntu-latest` CI floors:
at least 15% below the minimum observed across representative CI runs. Faster
native WSL2 measurements remain documented below as aspirational targets and
baseline evidence.

The BENCICAL-003 audit used representative Gate 2 runs
`27101213584`/`27111487150`/`27114668009`. Existing floors for
`serialization_roundtrip`, `replay_throughput`, `random_playout`, and
`bot_decision` were within 15% of the observed minimum, so they were widened under
ADR 0005 while the native targets remain visible.

## Variance-aware CI floors

| Operation | Native target/baseline | Observed CI minimum | Enforced CI floor |
|---|---:|---:|---:|
| `serialization_roundtrip` | 316,309.13 roundtrips/sec native WSL2 target | 195,741.57 roundtrips/sec across runs `27101213584`/`27111487150`/`27114668009` | 165,000 roundtrips/sec |
| `replay_throughput` | 396,321.82 replays/sec native WSL2 target | 227,947.89 replays/sec across runs `27101213584`/`27111487150`/`27114668009` | 190,000 replays/sec |
| `random_playout` | 100,000 games/sec ADR 0001 native target; 109,870.94 games/sec native WSL2 baseline | 66,761.40 games/sec across runs `27101213584`/`27111487150`/`27114668009` | 56,000 games/sec |
| `bot_decision` | 1,736,308.07 decisions/sec native WSL2 target | 1,001,027.56 decisions/sec across runs `27101213584`/`27111487150`/`27114668009` | 850,000 decisions/sec |

## Native benchmark section

| Operation | Target | Baseline | Current | Regression threshold | Status | Notes |
|---|---:|---:|---:|---:|---|---|
| setup | no formal Gate 1 target | no previous baseline | covered inside playout/replay setup | 10% from first committed baseline | no standalone baseline | Setup is intentionally included in replay/playout costs for this tiny game. |
| legal action generation | measured | no previous baseline | 2,229,464.03 trees/sec | 1,000,000 trees/sec hard floor | pass | `legal_actions`, 1,000,000 iterations. |
| preview generation | not applicable | not applicable | not applicable | not applicable | not applicable | `race_to_n` has no separate preview engine beyond legal action metadata. |
| validation | measured with apply/replay | no previous baseline | covered inside apply/replay numbers | 10% from first committed baseline | no standalone baseline | Normal validation is executed before every apply and replay step. |
| apply action/state transition | measured | no previous baseline | 13,223,070.56 actions/sec | 5,000,000 actions/sec hard floor | pass | `apply_action`, 1,000,000 iterations, includes validation. |
| public/private view generation | measured | no previous baseline | 79,694,610.25 views/sec | 10,000,000 views/sec hard floor | pass | `public_view_generation`; private view is not applicable because the game is perfect information. |
| effect filtering | measured | no previous baseline | 68,173,296.52 filters/sec | 10,000,000 filters/sec hard floor | pass | `EffectLog::since` over public effects. |
| serialization/deserialization | measured | no previous baseline | native WSL2 target 316,309.13 roundtrips/sec; CI minimum 195,741.57 across runs `27101213584`/`27111487150`/`27114668009` | 165,000 roundtrips/sec CI floor | pass | `RaceSnapshot::to_json` fixture parsed with `RaceSnapshot::from_json` and hashed. |
| replay throughput | measured | no previous baseline | native WSL2 target 396,321.82 replays/sec; CI minimum 227,947.89 across runs `27101213584`/`27111487150`/`27114668009` | 190,000 replays/sec CI floor | pass | Seven-command deterministic terminal replay. |
| random playout throughput | 100,000 games/sec accepted Stage-1 validated-playout native target | no previous baseline | native WSL2 target 109,870.94 games/sec; CI minimum 66,761.40 across runs `27101213584`/`27111487150`/`27114668009` | 56,000 games/sec CI floor from ADR 0005 | pass | ADR 0001's 100,000 native target remains visible; ADR 0005 calibrates the enforced variance-aware CI floor. |
| bot decision latency | measured | no previous baseline | native WSL2 target 1,736,308.07 decisions/sec; CI minimum 1,001,027.56 across runs `27101213584`/`27111487150`/`27114668009` | 850,000 decisions/sec CI floor | pass | `RaceRandomBot::select_action` on the initial legal tree. |
| candidate extraction if Level 2 | not applicable | not applicable | not applicable | not applicable | not applicable | Gate 1 bot is Level 0 random legal only. |
| hidden-info view filtering if applicable | not applicable | not applicable | not applicable | not applicable | not applicable | `race_to_n` has no hidden state. |

## WASM/browser smoke benchmark section

| Operation | Target | Baseline | Current | Regression threshold | Status | Notes |
|---|---:|---:|---:|---:|---|---|
| WASM package load/init | later web ticket | not measured | not measured | not set | not applicable for this native report | GAT1RACTON-011/012 own browser-exposed behavior. |
| start match from browser | later web ticket | not measured | not measured | not set | not applicable for this native report | Native setup is exercised by replay/playout benchmarks. |
| fetch public view/action tree | later web ticket | not measured | not measured | not set | not applicable for this native report | Native view/action generation is measured above. |
| preview from browser | not applicable | not applicable | not applicable | not applicable | not applicable | No separate preview behavior. |
| apply one action through WASM | later web ticket | not measured | not measured | not set | not applicable for this native report | Native validation/apply is measured above. |
| render/effect smoke | later web ticket | not measured | not measured | not set | not applicable for this native report | UI smoke is separate from native rule benchmarks. |
| bot turn through browser shell | later web ticket | not measured | not measured | not set | not applicable for this native report | Native bot decision latency is measured above. |
| replay step smoke | later web ticket | not measured | not measured | not set | not applicable for this native report | Native replay throughput is measured above. |
| reduced-motion smoke | later web ticket | not measured | not measured | not set | not applicable for this native report | Browser accessibility smoke is outside this native report. |

## Benchmark validity notes

| Concern | Applies? | Notes/mitigation |
|---|---:|---|
| debug build accidentally measured | no | `cargo bench` used the bench profile and optimized target. |
| hardware differs from baseline | no baseline | This is the first committed benchmark report. |
| browser/device differs from baseline | not applicable | No browser benchmark was run. |
| benchmark data/rules version changed | no | Rules/data versions are both `1`. |
| trace format/hash changed | no | This ticket adds benchmarks only; replay/hash formats are unchanged. |
| sample size too small | no | Native cases use 100,000 to 1,000,000 iterations. |
| noisy measurements | yes | WSL2 run without CPU pinning; numbers are suitable as Gate 1 baseline, not publication-grade perf evidence. |
| hidden-info redaction path not included | not applicable | `race_to_n` is perfect information. |
| bot policy version changed | no | Level 0 random legal policy from GAT1RACTON-007. |

## Bottlenecks

| Bottleneck | Evidence | Affected operation | Planned response | Requires ADR/ledger? |
|---|---|---|---|---:|
| Random playout provisional target mismatch | `random_playout` measured around 108,000-140,000 validated games/sec vs the old 500,000 games/sec provisional target | random playout throughput | ADR 0001 accepts 100,000 games/sec as the Gate 2 hard floor for the correctness-preserving harness. | yes |
| CI runner throughput below native target | BENCICAL-003 runs `27101213584`/`27111487150`/`27114668009` measured lower `ubuntu-latest` throughput for serialization, replay, random playout, and bot decision than the WSL2 native targets. | committed benchmark thresholds | ADR 0005 makes the committed threshold the variance-aware CI floor while this file preserves the native target. | yes |
| Snapshot JSON parsing dominates serialization row | `serialization_roundtrip` = 314,686.27 roundtrips/sec, slower than view/apply rows | serialization/deserialization | Keep as baseline unless replay/storage usage proves this hot. | no |

## Comparison to previous release

| Operation | Previous | Current | Change | Accept? | Rationale |
|---|---:|---:|---:|---:|---|
| legal action generation | no previous baseline | 2,229,464.03 trees/sec | not applicable | yes | First Gate 2 structured benchmark report. |
| apply action/state transition | no previous baseline | 13,223,070.56 actions/sec | not applicable | yes | First Gate 2 structured benchmark report. |
| public/private view generation | no previous baseline | 79,694,610.25 views/sec | not applicable | yes | First Gate 2 structured benchmark report. |
| effect filtering | no previous baseline | 68,173,296.52 filters/sec | not applicable | yes | First Gate 2 structured benchmark report. |
| serialization/deserialization | no previous baseline | 316,309.13 roundtrips/sec | not applicable | yes | First Gate 2 structured benchmark report. |
| replay throughput | no previous baseline | 396,321.82 replays/sec | not applicable | yes | First Gate 2 structured benchmark report. |
| random playout throughput | no previous baseline | 109,870.94 games/sec native full report; 66,761.40 games/sec CI minimum across runs `27101213584`/`27111487150`/`27114668009` | not applicable | yes vs 56,000 CI floor; native target remains 100,000 | ADR 0005 recalibrates the enforced CI floor while ADR 0001's native target remains recorded. |
| bot decision latency | no previous baseline | 1,736,308.07 decisions/sec | not applicable | yes | First Gate 2 structured benchmark report. |

## Trace/data/hash compatibility notes

| Artifact | Version/hash | Compatible? | Notes/action |
|---|---|---:|---|
| golden traces | GAT1RACTON-008 trace fixtures | yes | Benchmarks do not alter traces. |
| replay export format | schema version `1`, rules version `1` | yes | Benchmarks replay in memory through existing command/state contracts. |
| serialized state/checkpoint | `RaceSnapshot` schema version `1` | yes | Serialization benchmark uses existing snapshot JSON. |
| data/manifest | data/manifest version `1` | yes | No data files changed. |
| bot policy | Level 0 random legal from GAT1RACTON-007 | yes | Bot benchmark uses the existing wired bot. |

## Accepted regressions

| Regression | Amount | Accepted? | Rationale | Follow-up |
|---|---:|---:|---|---|
| Stage-1 random playout budget recalibration | 109,870.94 native full-report current vs 100,000 accepted native target | yes | ADR 0001 replaces the provisional 500,000 target for validated random playouts after profiling showed the old target did not match the correctness-preserving harness. | Keep the native target visible. |
| CI floor below native random-playout target | 66,761.40 games/sec minimum across representative `ubuntu-latest` runs vs 100,000 native target | yes | ADR 0005 calibrates the enforced gate to the runner that executes it while preserving the native target in this file. | Keep `bench-report` hard-failing below 56,000 games/sec on CI. |

Regressions accepted for public polish, correctness, visibility safety, replay compatibility, or accessibility MUST be explicit. Silent regressions are not allowed.

## Benchmark TODOs that block public release

| TODO | Blocks public release? | Required evidence | Owner |
|---|---:|---|---|
| Maintain accepted Stage-1 random playout target | yes for native benchmark health | `random_playout` at or above 100,000 games/sec under native `cargo bench -p race_to_n`; CI gate enforces the ADR 0005 56,000 games/sec floor | Gate 2 |
| Add WASM/browser smoke benchmarks once the browser surface exists | no for this native ticket; yes before public web release | Browser smoke report for load, view/action tree, apply, bot turn, and render/effects | GAT1RACTON-011/012 follow-up |

## Review checklist

- Benchmark report records rules, data/manifest, and engine versions.
- Native benchmarks cover setup, legal actions, validation, apply action, view generation, effect filtering, serialization, replay, playout, and bot latency where applicable.
- Preview generation, hidden-info filtering, Level 2 candidate extraction, and WASM/browser smoke are marked not applicable or future-owned with rationale.
- Regression thresholds are explicit.
- Benchmark validity caveats are recorded.
- Trace/data/hash compatibility is recorded.
- Accepted regressions have rationale.
- Public-release-blocking benchmark TODOs are explicit.
