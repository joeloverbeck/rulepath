# race_to_n Benchmarks

Game ID: `race_to_n`

Implemented variant: `race_to_21`

Rules version: `1`

Data/manifest version: `1`

Engine version: `0.1.0`

Benchmark report version: `1`

Prepared by: `Codex`

Last updated: 2026-06-05

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
| full native benchmark set | `cargo bench -p race_to_n` | native | Runs `games/race_to_n/benches/race_to_n.rs` with `harness = false`. |
| single-case smoke | `cargo bench -p race_to_n -- legal_actions` | native | Cargo resolves the same bench target; the custom harness still prints the full native table. |

## Native benchmark section

| Operation | Target | Baseline | Current | Regression threshold | Status | Notes |
|---|---:|---:|---:|---:|---|---|
| setup | no formal Gate 1 target | no previous baseline | covered inside playout/replay setup | 10% from first committed baseline | no standalone baseline | Setup is intentionally included in replay/playout costs for this tiny game. |
| legal action generation | measured | no previous baseline | 2,198,273.77 trees/sec | 10% from first committed baseline | no previous baseline | `legal_actions`, 1,000,000 iterations. |
| preview generation | not applicable | not applicable | not applicable | not applicable | not applicable | `race_to_n` has no separate preview engine beyond legal action metadata. |
| validation | measured with apply/replay | no previous baseline | covered inside apply/replay numbers | 10% from first committed baseline | no standalone baseline | Normal validation is executed before every apply and replay step. |
| apply action/state transition | measured | no previous baseline | 12,797,460.98 actions/sec | 10% from first committed baseline | no previous baseline | `apply_action`, 1,000,000 iterations, includes validation. |
| public/private view generation | measured | no previous baseline | 76,324,231.13 views/sec | 10% from first committed baseline | no previous baseline | `public_view_generation`; private view is not applicable because the game is perfect information. |
| effect filtering | measured | no previous baseline | 61,531,645.73 filters/sec | 10% from first committed baseline | no previous baseline | `EffectLog::since` over public effects. |
| serialization/deserialization | measured | no previous baseline | 314,686.27 roundtrips/sec | 10% from first committed baseline | no previous baseline | `RaceSnapshot::to_json` fixture parsed with `RaceSnapshot::from_json` and hashed. |
| replay throughput | measured | no previous baseline | 381,794.66 replays/sec | 10% from first committed baseline | no previous baseline | Seven-command deterministic terminal replay. |
| random playout throughput | 500,000 games/sec Stage-1 budget | no previous baseline | 134,277.09 games/sec | Stage-1 budget plus 10% from first committed baseline | fail vs Stage-1 budget | Below TESTING §15 Stage-1 budget on this WSL2 run. |
| bot decision latency | measured | no previous baseline | 1,681,023.74 decisions/sec | 10% from first committed baseline | no previous baseline | `RaceRandomBot::select_action` on the initial legal tree. |
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
| Random playout below Stage-1 budget | `random_playout` = 134,277.09 games/sec vs 500,000 games/sec target | random playout throughput | Track as Gate 1 perf follow-up; do not optimize without a targeted benchmark/profile. | no |
| Snapshot JSON parsing dominates serialization row | `serialization_roundtrip` = 314,686.27 roundtrips/sec, slower than view/apply rows | serialization/deserialization | Keep as baseline unless replay/storage usage proves this hot. | no |

## Comparison to previous release

| Operation | Previous | Current | Change | Accept? | Rationale |
|---|---:|---:|---:|---:|---|
| legal action generation | no previous baseline | 2,198,273.77 trees/sec | not applicable | yes | First benchmark report. |
| apply action/state transition | no previous baseline | 12,797,460.98 actions/sec | not applicable | yes | First benchmark report. |
| public/private view generation | no previous baseline | 76,324,231.13 views/sec | not applicable | yes | First benchmark report. |
| effect filtering | no previous baseline | 61,531,645.73 filters/sec | not applicable | yes | First benchmark report. |
| serialization/deserialization | no previous baseline | 314,686.27 roundtrips/sec | not applicable | yes | First benchmark report. |
| replay throughput | no previous baseline | 381,794.66 replays/sec | not applicable | yes | First benchmark report. |
| random playout throughput | no previous baseline | 134,277.09 games/sec | not applicable | no vs Stage-1 budget | First benchmark report, but below the stated Stage-1 target. |
| bot decision latency | no previous baseline | 1,681,023.74 decisions/sec | not applicable | yes | First benchmark report. |

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
| Stage-1 random playout budget miss | 134,277.09 current vs 500,000 target | no | This is a miss against the target, not an accepted regression. | Reassess in Gate 1 closeout or create a perf follow-up before claiming Stage-1 budget met. |

Regressions accepted for public polish, correctness, visibility safety, replay compatibility, or accessibility MUST be explicit. Silent regressions are not allowed.

## Benchmark TODOs that block public release

| TODO | Blocks public release? | Required evidence | Owner |
|---|---:|---|---|
| Resolve or explicitly waive Stage-1 random playout budget miss | yes for claiming Stage-1 budget met | `random_playout` at or above 500,000 games/sec, or an approved doctrine/spec adjustment | Gate 1 closeout |
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
