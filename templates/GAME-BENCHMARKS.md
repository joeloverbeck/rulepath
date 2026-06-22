# <game_id> Benchmarks

Game ID: `<game_id>`

Implemented variant: `<variant>`

Rules version: `<rules_version>`

Data/manifest version: `<data_or_manifest_version>`

Engine version: `<engine_version>`

Benchmark report version: `<benchmark_report_version>`

Prepared by: `<name/agent>`

Last updated: YYYY-MM-DD

Evidence receipt: [`GAME-EVIDENCE.md`](GAME-EVIDENCE.md)

Template realignment mapping: report `B-14 -> GAME-BENCHMARKS.md`. This
template owns benchmark workload definitions, compatibility anchors,
measurement environments, commands, and results. `GAME-EVIDENCE.md` owns the
benchmark workload ID rollup, bot policy IDs, release status, and blockers.

## Benchmark doctrine

Native Rust benchmarks are primary for rule, replay, simulation, and bot hot paths. WASM/browser smoke benchmarks are required when web-exposed behavior, renderer integration, or JS/WASM boundary behavior is relevant.

Do not optimize without benchmark evidence. Do not accept regressions without rationale.

## Hardware and environment

| Field | Value |
|---|---|
| machine/device | `<machine>` |
| CPU | `<cpu>` |
| RAM | `<ram>` |
| OS/version | `<os>` |
| Rust version | `<rust_version>` |
| Cargo profile | debug / release / bench |
| target | native / wasm32 / browser smoke |
| browser/version if applicable | `<browser>` |
| build artifact/hash | `<artifact_or_hash>` |
| engine version/hash | `<engine_version_or_hash>` |
| rules version | `<rules_version>` |
| data/manifest version | `<data_or_manifest_version>` |
| benchmark date | YYYY-MM-DD |
| thermal/power notes | `<notes>` |

## Commands

| Workload ID | Benchmark/smoke | Command | Environment | Fixture/profile ID | Notes |
|---|---|---|---|---|---|
| `BENCH-001` | `<benchmark>` | `<command>` | native / wasm / browser | `<fixture_or_profile_id>` | `<notes>` |

## Seat count and surface fixture matrix

Every official seat count and the largest official surface fixture MUST be represented. The largest official variant requires legal-action, preview, apply, project-view, serialize, replay-import, bot-turn, and WASM smoke coverage.

| Seat count | Surface fixture | Workload IDs | Legal-action benchmark | Preview benchmark | Apply benchmark | Project-view benchmark | Serialize benchmark | Replay-import benchmark | Bot-turn benchmark | WASM smoke benchmark | Status |
|---:|---|---|---|---|---|---|---|---|---|---|---|
| `<seat_count>` | `<min/typical/max-surface fixture>` | `BENCH-*` | `<command/evidence>` | `<command/evidence>` | `<command/evidence>` | `<command/evidence>` | `<command/evidence>` | `<command/evidence>` | `<command/evidence>` | `<command/evidence>` | not started / partial / covered |

## Native benchmark section

| Operation | Target | Baseline | Current | Regression threshold | Status | Notes |
|---|---:|---:|---:|---:|---|---|
| setup | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| legal action generation | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| preview generation | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| validation | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| apply action/state transition | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| public/private view generation | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| effect filtering | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| serialization/deserialization | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| replay throughput | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| random playout throughput | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| bot decision latency | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| candidate extraction if Level 2 | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline/not applicable | `<notes>` |
| hidden-info view filtering if applicable | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline/not applicable | `<notes>` |
| largest official variant legal actions | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline/not applicable | `<seat count + max-surface fixture>` |
| largest official variant project-view | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline/not applicable | `<seat count + max-surface fixture>` |
| largest official variant replay import | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline/not applicable | `<seat count + max-surface fixture>` |

## WASM/browser smoke benchmark section

| Operation | Target | Baseline | Current | Regression threshold | Status | Notes |
|---|---:|---:|---:|---:|---|---|
| WASM package load/init | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| start match from browser | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| fetch public view/action tree | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| preview from browser | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| apply one action through WASM | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| render/effect smoke | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| bot turn through browser shell | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline/not applicable | `<notes>` |
| replay step smoke | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |
| reduced-motion smoke | `<target>` | `<baseline>` | `<current>` | `<threshold>` | pass/fail/no baseline | `<notes>` |

## Benchmark validity notes

| Concern | Applies? | Notes/mitigation |
|---|---:|---|
| debug build accidentally measured | yes/no | `<notes>` |
| hardware differs from baseline | yes/no | `<notes>` |
| browser/device differs from baseline | yes/no | `<notes>` |
| benchmark data/rules version changed | yes/no | `<notes>` |
| trace format/hash changed | yes/no | `<notes>` |
| sample size too small | yes/no | `<notes>` |
| noisy measurements | yes/no | `<notes>` |
| hidden-info redaction path not included | yes/no/not applicable | `<notes>` |
| bot policy version changed | yes/no/not applicable | `<notes>` |

## Bottlenecks

| Bottleneck | Evidence | Affected operation | Planned response | Requires ADR/ledger? |
|---|---|---|---|---:|
| `<bottleneck>` | `<benchmark/profile>` | `<operation>` | `<response>` | yes/no |

## Comparison to previous release

| Operation | Previous | Current | Change | Accept? | Rationale |
|---|---:|---:|---:|---:|---|
| `<operation>` | `<previous>` | `<current>` | `<change>` | yes/no | `<rationale>` |

## Trace/data/hash compatibility notes

| Workload ID | Artifact | Fixture/profile ID | Version/hash | Compatible? | Notes/action |
|---|---|---|---|---:|---|
| `BENCH-001` | golden traces | `<profile_id>` | `<version/hash>` | yes/no | `<notes>` |
| `BENCH-002` | replay export format | `<profile_id>` | `<version/hash>` | yes/no | `<notes>` |
| `BENCH-003` | serialized state/checkpoint | `<profile_id>` | `<version/hash>` | yes/no | `<notes>` |
| `BENCH-004` | data/manifest | `<profile_id>` | `<version/hash>` | yes/no | `<notes>` |
| `BENCH-005` | bot policy | `<policy_id>` | `<version/hash>` | yes/no | `<notes>` |

## Accepted regressions

| Regression | Amount | Accepted? | Rationale | Follow-up |
|---|---:|---:|---|---|
| `<regression>` | `<amount>` | yes/no | `<rationale>` | `<follow_up>` |

Regressions accepted for public polish, correctness, visibility safety, replay compatibility, or accessibility MUST be explicit. Silent regressions are not allowed.

## Benchmark TODOs that block public release

| TODO | Blocks public release? | Required evidence | Owner |
|---|---:|---|---|
| `<todo>` | yes/no | `<evidence>` | `<owner>` |

## Review checklist

- Benchmark report records rules, data/manifest, and engine versions.
- Benchmark workload IDs are stable and linked from `GAME-EVIDENCE.md`.
- Native benchmarks cover setup, legal actions, preview, validation, apply action, view generation, effect filtering, serialization, replay, playout, and bot latency where applicable.
- WASM/browser smoke covers public web hot paths where relevant.
- Regression thresholds are explicit.
- Benchmark validity caveats are recorded.
- Trace/data/hash compatibility is recorded.
- Accepted regressions have rationale.
- Public-release-blocking benchmark TODOs are explicit.
