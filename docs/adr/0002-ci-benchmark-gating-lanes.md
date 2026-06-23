# ADR: CI Benchmark Gating Lanes

Status: Accepted

Date: 2026-06-05

Decision owner: joeloverbeck

Related documents:

- `docs/FOUNDATIONS.md`
- `docs/TESTING-REPLAY-BENCHMARKING.md`
- `docs/adr/0001-stage-1-random-playout-budget.md`
- `specs/gate-2-trace-replay-benchmark-hardening.md`
- `games/race_to_n/benches/thresholds.json`
- `games/race_to_n/docs/BENCHMARKS.md`

## Context

The Gate 2 benchmark threshold check was added to CI in the "Implemented gate 2"
change and ran against a shared GitHub `ubuntu-latest` runner for the first time
on the `implemented-gate-2` pull request. It hard-failed: three operations fell
below thresholds that were calibrated from local WSL2 native runs.

- `random_playout`: runner measured ~66,058 games/sec against the
  [ADR 0001](0001-stage-1-random-playout-budget.md) floor of 100,000 games/sec.
  ADR 0001 records WSL2 evidence of 108,000-140,000 games/sec and chose 100,000
  as a floor "conservative for noisy local/CI contexts"; the shared runner is in
  fact ~34% slower than that floor.
- `serialization_roundtrip`: runner measured ~195,236 against a 200,000 floor.
- `replay_throughput`: runner measured ~233,478 against a 250,000 floor.

The four operations carrying `conservative_ci_floor` rationale were deliberately
set below the WSL2 baseline and passed. The three failing operations were set at
or near the WSL2 baseline (`measured_baseline` / `accepted_adr`) and do not
survive the slower, virtualized, shared PR runner. `thresholds.json` itself warns
that "WSL2 and local workstation runs can be noisy."

This is a calibration-environment mismatch, not a code regression: there is no
prior CI baseline to regress from, and the failing operations are the heavy
validated paths while the lighter paths pass.

`docs/TESTING-REPLAY-BENCHMARKING.md` §15 and §17, together with ADR 0001,
currently require the Gate 2 benchmark threshold check to hard-fail **CI** and
forbid lowering a threshold "only to make CI green." §17 also permits expensive
benchmarks to "run nightly or manually." An ADR is required because resolving the
failure changes where the benchmark gate is enforced, which is benchmark-gating
doctrine fixed by ADR 0001 and the testing law. This keeps public playable
Rulepath unblocked (PRs stop failing on environmental throughput noise) while
preserving honest, hard-failing performance gating.

## Decision

Benchmark threshold enforcement is split into two CI lanes.

- The pull-request lane MUST run a non-gating benchmark **smoke** only: it
  compiles and runs the native harness (`cargo bench -p race_to_n -- legal_actions`)
  and MUST NOT invoke `bench-report` in threshold mode. Shared PR runners are not
  a valid throughput-gating environment.
- The pull-request lane MAY invoke `bench-report --schema-only` on smoke output
  to validate benchmark-report shape: required metadata fields are present and
  non-empty, and `operations` entries are well formed. Schema validation is
  environment-independent and MUST NOT compare measured values against
  thresholds.
- The scheduled, manual, and `main`-push lane MUST run the full benchmark and
  hard-fail through `bench-report` against
  `games/race_to_n/benches/thresholds.json`.

Threshold **values** are unchanged. ADR 0001's 100,000 games/sec
`random_playout` floor and every other committed threshold remain exactly as
recorded; this ADR relocates the enforcement lane and does not weaken any number.

This ADR amends the enforcement-environment clause of
`docs/TESTING-REPLAY-BENCHMARKING.md` §15 and §17: the phrase "hard-fail CI"
means hard-fail the scheduled / manual / `main`-push benchmark lane, while the
pull-request lane runs the smoke described above.

## Alternatives considered

| Alternative | Why considered | Why accepted/rejected |
|---|---|---|
| Two-lane split: PR smoke + scheduled/manual/main gate | §17 already permits nightly/manual benchmarks; isolates the noisy environment without touching numbers. | Accepted. Keeps gating hard-failing on a controlled lane, unblocks PRs, hides no performance, and changes no threshold value. |
| Recalibrate the three thresholds to conservative runner floors and keep them blocking every PR | Mirrors the existing `conservative_ci_floor` pattern. | Rejected. Shared runners are noisy and variable, so even conservative floors risk flaking every PR; lowering ADR 0001's accepted floor edges toward "tuning to green," which is forbidden. |
| Hybrid: PR smoke plus a scheduled gate recalibrated to the CI environment | Most robust long term. | Rejected for this scope. Adds recalibration and a second ADR amendment without removing the core problem the two-lane split already solves; can be revisited if scheduled-runner noise appears. |
| Leave the gate on every PR | Smallest diff. | Rejected. It hard-fails honest PRs on environmental noise and contradicts §17's nightly/manual carve-out. |

## Consequences

Positive consequences:

- Pull requests stop failing on shared-runner throughput noise.
- Pull requests can fail on malformed benchmark JSON before drift reaches
  `main`, without turning shared-runner throughput into a PR gate.
- Benchmark gating stays hard-failing on the scheduled / manual / `main` lane; no
  performance is hidden.
- No threshold value changes, so no risk of silently weakening ADR 0001.
- The single CI workflow is split into gate-scoped workflows, isolating the
  benchmark concern.

Negative or risky consequences:

- A throughput regression introduced in a PR is caught on the post-merge or
  scheduled lane rather than on the PR itself. Mitigation: the PR smoke still
  proves the harness compiles and runs; `main` push runs the gate immediately
  after merge.
- The scheduled lane still runs on shared GitHub runners, so its absolute numbers
  remain environment-dependent; thresholds may need future recalibration to the
  scheduled-runner baseline. That would be a separate ADR.

Operational requirements:

- `.github/workflows/gate-2-benchmarks.yml` implements both lanes via
  `if: github.event_name == 'pull_request'` (smoke) and
  `if: github.event_name != 'pull_request'` (gate).
- `bench-report --schema-only` is allowed only for report-shape validation on
  the PR smoke lane; threshold comparison remains scheduled / manual /
  `main`-push only.
- `docs/TESTING-REPLAY-BENCHMARKING.md` §15 and §17 reference this ADR for the
  lane definition.
- `games/race_to_n/docs/BENCHMARKS.md` records the observed `ubuntu-latest`
  values and the lane split.

## Determinism impact

- No change to RNG, iteration order, clocks, floating point, parallelism,
  serialization order, replay, or hashes. Only CI orchestration changes.
- Determinism proof remains `cargo test -p race_to_n` plus replay and bot tests,
  run in Gate 0 and Gate 1.

## Replay/hash impact

- None. No command streams, state/effect/action-tree/view hashes, trace format,
  or migration rules change. Existing golden traces remain valid.

## Visibility impact

- None. No public/private view, payload, log, explanation, ranking, or replay
  export is affected. `race_to_n` remains perfect-information.

## Data/Rust boundary impact

- None. `thresholds.json` remains typed benchmark policy data and is unchanged.
  No new field, format, expression, selector, behavior ID, variant, or schema is
  introduced. Behavior stays in typed Rust.

## `engine-core` contamination risk

- None. No `engine-core` change. The decision is CI orchestration plus doc
  updates.

## `game-stdlib` / primitive-pressure impact

- None. No shared primitive is introduced or promoted.

## UI impact

- None. No action trees, previews, effect-log animation, renderer boundaries,
  accessibility, reduced motion, or inspector behavior change. TypeScript remains
  presentation-only.

## Bot impact

- None. Bot views, legal action APIs, candidate ranking, explanations, and bot
  benchmarks are unchanged. No v1/v2-excluded AI technique is introduced.

## IP impact

- None. No public naming, prose, assets, fonts, fixtures, traces, or
  browser-shipped bundles change.

## Benchmark impact

- No native benchmark is added, removed, or recalibrated. All committed
  thresholds keep their values.
- The Stage 1 `random_playout` floor remains 100,000 games/sec per ADR 0001,
  enforced on the scheduled / manual / `main` lane.
- No public latency budget changes. No WASM/browser smoke benchmark is added.

## Migration notes

Existing docs to update:

- `docs/TESTING-REPLAY-BENCHMARKING.md` §15 and §17 — point "hard-fail CI" at
  this ADR's lane definition.
- `games/race_to_n/docs/BENCHMARKS.md` — record the lane split and observed
  `ubuntu-latest` values.

Existing games to back-port:

- None.

Existing traces to preserve or update:

- None.

Existing data/schema versions to bump:

- None. `thresholds.json` is unchanged.

Existing public UI behavior to migrate:

- None.

## Review checklist

Before accepting this ADR, verify:

- the decision supports public playable Rulepath before engine research — yes, it
  unblocks PRs without weakening gating;
- Rust remains behavior authority — unchanged;
- TypeScript does not decide legality — unchanged;
- `engine-core` remains noun-free — unchanged;
- `game-stdlib` remains earned and narrow — unchanged;
- static data remains content/parameters, not behavior — `thresholds.json`
  unchanged;
- replay determinism is preserved or migration is explicit — preserved;
- visibility boundaries remain safe — unchanged;
- bots remain fair and explainable — unchanged;
- benchmarks exist for hot paths — unchanged; gating relocated, not removed;
- IP/public-private boundaries are preserved — unchanged;
- affected foundation docs and per-game docs are updated — TESTING §15/§17 and
  `race_to_n` BENCHMARKS.md updated.
