# ADR: CI-calibrated benchmark thresholds

Status: Accepted

Date: 2026-06-07

Decision owner: joeloverbeck

Related documents:

- `docs/FOUNDATIONS.md`
- `docs/TESTING-REPLAY-BENCHMARKING.md`
- `docs/adr/0001-stage-1-random-playout-budget.md`
- `docs/adr/0002-ci-benchmark-gating-lanes.md`
- `games/race_to_n/benches/thresholds.json`
- `games/race_to_n/docs/BENCHMARKS.md`

## Context

[ADR 0002](0002-ci-benchmark-gating-lanes.md) split the Gate 2 benchmark check
into two lanes: pull requests run a non-gating smoke, while the scheduled /
manual / `main`-push lane runs the hard-failing `bench-report` threshold gate.
That stopped honest PRs from failing on shared-runner throughput noise, but it
left the `main`/scheduled gate calibrated to local WSL2 baselines while running
on a shared GitHub `ubuntu-latest` runner. ADR 0002 explicitly foresaw this in
its Consequences: "the scheduled lane still runs on shared GitHub runners, so its
absolute numbers remain environment-dependent; thresholds may need future
recalibration to the scheduled-runner baseline. That would be a separate ADR."
This is that ADR.

The `main`-push lane has failed on every merge since Gate 3. The
`Gate 2 benchmarks / Benchmark threshold gate` job fails at `race_to_n` (the
first game in the sequential gate), so the `set -euo pipefail` step aborts before
`three_marks`, `column_four`, or `directional_flip` ever run. The four operations
that breach their WSL2-calibrated floors on `ubuntu-latest` are (from run
`27086098697`, commit `f27174a`):

| Operation | CI value | WSL2 floor | Rationale class | Under by |
|---|---:|---:|---|---:|
| `serialization_roundtrip` | 192,697 | 200,000 | `measured_baseline` | 3.7% |
| `replay_throughput` | 231,094 | 250,000 | `measured_baseline` | 7.6% |
| `random_playout` | 66,050 | 100,000 | `accepted_adr` (ADR 0001) | 34% |
| `bot_decision` | 985,488 | 1,000,000 | `measured_baseline` | 1.5% |

Two facts shape the decision:

1. **The runner is stable, not noisy.** ADR 0002 rejected per-PR gating fearing
   shared-runner flakiness. The empirical record across five consecutive runs
   contradicts that fear for these operations: `random_playout` lands at ~66,050
   every time (66,058 on the original `implemented-gate-2` PR run, 66,050 now).
   The shared runner is consistently ~34% slower than WSL2, not randomly noisy.
   Recalibrating to that stable CI baseline is principled, not cosmetic.

2. **We are blind to games 2-4 on CI.** Because the gate aborts at `race_to_n`,
   `three_marks` and `column_four` have never produced `ubuntu-latest` numbers.
   Their thresholds are mostly `measured_baseline` from local native runs and may
   also breach on the slower runner. The gate must run all games and aggregate
   failures before recalibration evidence for games 2-4 exists.

`docs/TESTING-REPLAY-BENCHMARKING.md` §15 forbids lowering a threshold "only to
make CI green" and requires keeping a miss visible. An ADR is required because the
fix changes ADR 0001's accepted Stage-1 floor and ADR 0002's lane-calibration
doctrine — both are doctrine fixed by accepted ADRs and the testing law, which
FOUNDATIONS §13 says supersede only by accepted ADR. This keeps public playable
Rulepath unblocked (a permanently red `main` is removed) while preserving honest,
hard-failing performance gating.

## Decision

The Gate 2 benchmark threshold values are calibrated to the **CI-runner
environment that enforces them**, not to a workstation that never runs the gate.

- Each `thresholds.json` `threshold` value MUST be a conservative floor below the
  stable `ubuntu-latest` measurement for that operation. The committed
  `threshold` is the **gate value**.
- Each game's `BENCHMARKS.md` MUST record the faster native/WSL2 baseline as a
  documented aspirational target alongside the CI floor. The native baseline is
  preserved on the record; it is not the gate value. No performance is hidden.
- The scheduled / manual / `main`-push benchmark lane MUST run **every** game's
  benchmark and `bench-report`, aggregate all failures, and fail non-zero if any
  game breaches a floor. It MUST NOT abort at the first failing game. This makes
  one CI run surface every game's numbers.
- The pull-request smoke lane is unchanged (ADR 0002 stands for the lane split).

This ADR amends ADR 0001 and ADR 0002:

- **ADR 0001** recorded 100,000 games/sec as the Stage-1 validated
  `random_playout` floor. That figure is retained as the documented **native**
  target. The **enforced** floor for the `random_playout` operation on the shared
  runner is recalibrated to a conservative value below the consistently observed
  ~66,050 games/sec. The native target is not weakened; the gate is moved to the
  environment it runs in.
- **ADR 0002** is amended so that "hard-fail the scheduled / manual / `main`-push
  benchmark lane" gates against CI-calibrated floors and aggregates across all
  games rather than aborting at the first.

Recalibrated operations keep their `rationale_class` semantics: re-floored
`measured_baseline` operations use `conservative_ci_floor`; `random_playout`
retains `accepted_adr` with its rationale re-pointed at this ADR. The exact
recalibrated values are committed by the implementation tickets
(`BENCICAL-001`, `BENCICAL-002`) from CI evidence, not invented here.

## Alternatives considered

| Alternative | Why considered | Why accepted/rejected |
|---|---|---|
| Recalibrate to a single CI-runner floor per operation; harden the gate to run all games | The runner is empirically stable (~66k every run), and ADR 0002 named this as the planned next step. | Accepted. Minimal, honest, calibrates the gate to its real environment, keeps native baselines on the record, and changes no PR behavior. |
| Dual-environment schema: add a `ci_threshold` field beside `threshold` | Preserves the WSL2 number as the committed value and adds a second calibrated number. | Rejected for this scope. Requires a `thresholds.json` schema bump (v2) and a `bench-report` parser change across all games for marginal benefit; BENCHMARKS.md already records the native baseline. |
| Controlled environment: self-hosted runner, or criterion regression-relative gating against a stored baseline | Most robust long term; removes the absolute WSL2-vs-CI mismatch entirely. | Rejected for this scope. Adds runner infrastructure or a baseline-artifact mechanism; ADR 0002 already deferred this as out-of-scope. Can be revisited if the shared runner later proves noisy. |
| Make the `main`/scheduled lane non-gating too | Smallest diff; turns `main` green immediately. | Rejected. It hides all throughput regressions and contradicts TESTING §15/§16 "do not hide unknown performance" and the hard-fail requirement. |
| Leave the gate failing | Zero change. | Rejected. A permanently red `main` trains maintainers to ignore the gate, which is worse than no gate. |

## Consequences

Positive consequences:

- `main` stops failing on an environment mismatch that ADR 0002 already
  predicted; the gate becomes meaningful again instead of permanently red.
- Benchmark gating stays hard-failing on the `main`/scheduled/manual lane,
  calibrated to the environment that actually runs it.
- Native WSL2 baselines remain recorded in each game's `BENCHMARKS.md`, so no
  performance figure is hidden or lost.
- One CI run now reports every game's numbers, ending the blind spot on
  `three_marks` and `column_four`.

Negative or risky consequences:

- CI floors are lower than the native targets, so a regression that lands between
  the CI floor and the native baseline is not caught by the gate. Mitigation:
  the native target stays documented in `BENCHMARKS.md` as the standard the game
  is held to off-CI, and the floor still catches material regressions.
- The shared runner could become noisier in future (different hardware
  generations behind `ubuntu-latest`). Mitigation: floors are set conservatively
  below the observed value; a future noise problem would be a separate ADR
  (controlled-runner or regression-relative gating).

Operational requirements:

- `.github/workflows/gate-2-benchmarks.yml` `bench-gate` job runs all games and
  aggregates failures (`BENCICAL-001`).
- The four `games/*/benches/thresholds.json` files carry CI-calibrated floors;
  `directional_flip` is already non-blocking (`1`) (`BENCICAL-002`).
- Each game's `BENCHMARKS.md` records the native baseline and the CI floor
  (`BENCICAL-002`).
- `docs/TESTING-REPLAY-BENCHMARKING.md` §15 and §17 reference this ADR for the
  calibration doctrine.

## Determinism impact

- No change to RNG, iteration order, clocks, floating point, parallelism,
  serialization order, replay, or hashes. Only CI orchestration and benchmark
  policy data change.
- Determinism proof remains the Gate 0 / Gate 1 `cargo test`, replay, and bot
  suites; unaffected.

## Replay/hash impact

- None. No command streams, state/effect/action-tree/view hashes, trace format,
  or migration rules change. Existing golden traces remain valid.

## Visibility impact

- None. No public/private view, payload, log, explanation, ranking, or replay
  export is affected. Benchmark numbers are public, non-secret performance data.

## Data/Rust boundary impact

- `thresholds.json` remains typed benchmark policy data. The schema is unchanged
  (no new field, format, expression, selector, behavior ID, variant, or version).
  Only `threshold` numeric values and `rationale`/`rationale_class` strings
  change. Behavior stays in typed Rust; no DSL pressure.

## `engine-core` contamination risk

- None. No `engine-core` change. The decision is CI orchestration, benchmark
  policy data, and docs.

## `game-stdlib` / primitive-pressure impact

- None. No shared primitive is introduced or promoted.

## UI impact

- None. No action trees, previews, effect-log animation, renderer boundaries,
  accessibility, reduced motion, or inspector behavior change. TypeScript remains
  presentation-only.

## Bot impact

- The `bot_decision` / `level0_bot_decision` / `level2_bot_decision` benchmark
  floors are recalibrated to CI values, but bot views, legal action APIs,
  candidate ranking, explanations, and hidden-information safety are unchanged.
  No v1/v2-excluded AI technique is introduced.

## IP impact

- None. No public naming, prose, assets, fonts, fixtures, traces, or
  browser-shipped bundles change. Benchmark numbers are non-proprietary.

## Benchmark impact

- No native benchmark is added or removed. Threshold **values** for the failing
  operations are recalibrated to conservative `ubuntu-latest` floors; the native
  targets are preserved in `BENCHMARKS.md`.
- The Stage 1 `random_playout` native target remains 100,000 games/sec per ADR
  0001; the enforced CI floor is recalibrated below the observed ~66,050.
- No public latency budget changes. No WASM/browser smoke benchmark is added.

## Migration notes

Existing docs to update:

- `docs/TESTING-REPLAY-BENCHMARKING.md` §15 and §17 — reference this ADR for the
  CI-calibration doctrine (extends the existing ADR 0002 lane reference).
- `games/race_to_n/docs/BENCHMARKS.md`, `games/three_marks/docs/BENCHMARKS.md`,
  `games/column_four/docs/BENCHMARKS.md` — record the native baseline and the CI
  floor.

Existing games to back-port:

- `race_to_n`, `three_marks`, `column_four` threshold files recalibrated.
  `directional_flip` is already non-blocking and needs no change until it has
  stable CI measurements.

Existing traces to preserve or update:

- None.

Existing data/schema versions to bump:

- None. `thresholds.json` `schema_version` stays 1; only values change.

Existing public UI behavior to migrate:

- None.

## Review checklist

Before accepting this ADR, verify:

- the decision supports public playable Rulepath before engine research — yes, it
  removes a permanently red `main` without weakening gating;
- Rust remains behavior authority — unchanged;
- TypeScript does not decide legality — unchanged;
- `engine-core` remains noun-free — unchanged;
- `game-stdlib` remains earned and narrow — unchanged;
- static data remains content/parameters, not behavior — `thresholds.json` schema
  unchanged, only values;
- replay determinism is preserved or migration is explicit — preserved;
- visibility boundaries remain safe — unchanged;
- bots remain fair and explainable — unchanged;
- benchmarks exist for hot paths — unchanged; floors recalibrated, not removed;
  native targets preserved in `BENCHMARKS.md`;
- IP/public-private boundaries are preserved — unchanged;
- affected foundation docs and per-game docs are updated — TESTING §15/§17 and
  each game's `BENCHMARKS.md` updated.
