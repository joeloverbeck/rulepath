# ADR: Variance-aware CI benchmark floors

Status: Accepted

Status note: accepted on 2026-06-22 after the BENCICAL-003 and BENCICAL-004
implementation evidence showed the variance-aware floor rule had been applied to
the shipped benchmark manifests and testing doctrine from three representative
CI runs. This ADR is binding benchmark doctrine from that date forward.

Date: 2026-06-08

Decision owner: joeloverbeck

Related documents:

- `docs/FOUNDATIONS.md`
- `docs/TESTING-REPLAY-BENCHMARKING.md`
- `docs/adr/0001-stage-1-random-playout-budget.md`
- `docs/adr/0002-ci-benchmark-gating-lanes.md`
- `docs/adr/0003-ci-calibrated-benchmark-thresholds.md`
- `games/three_marks/benches/thresholds.json`
- `games/column_four/benches/thresholds.json`
- `games/three_marks/docs/BENCHMARKS.md`
- `games/column_four/docs/BENCHMARKS.md`

## Context

[ADR 0003](0003-ci-calibrated-benchmark-thresholds.md) recalibrated the Gate 2
benchmark floors to the shared `ubuntu-latest` runner that enforces them, on the
premise that "the runner is stable, not noisy" — its evidence was that
`race_to_n`'s `random_playout` landed at ~66,050 games/sec across several runs.
It set each floor as a conservative margin below a **single** observed CI value
captured in the `BENCICAL-001` calibration run (`27087214359`). ADR 0003 also
foresaw the present failure in its own Consequences: "The shared runner could
become noisier in future ... a future noise problem would be a separate ADR
(controlled-runner or regression-relative gating)." This is that ADR.

The `Gate 2 benchmarks / Benchmark threshold gate` job has hard-failed on every
`main`-push merge since the recalibration: runs `27101213584` (PR #10),
`27111487150` (PR #11), and `27114668009` (PR #12). `race_to_n`,
`directional_flip`, `draughts_lite`, and `high_card_duel` pass; `three_marks` and
`column_four` breach their floors, so the aggregating `set -euo pipefail` step
exits non-zero. The pull-request lane stays green because ADR 0002 makes it a
non-gating smoke.

The breaching operations, with the ADR 0003 calibration value and the three
subsequent steady-state merge runs:

| Game / operation | Floor | Calib. (`27087…`) | `27101…` | `27111…` | `27114…` |
|---|---:|---:|---:|---:|---:|
| column_four `level2_bot_decision` | 3,000 | 3,063 | 1,438 | 1,450 | 1,370 |
| column_four `random_playout` | 9,000 | 9,700 | 5,939 | 6,173 | 5,972 |
| column_four `apply_action` | 200,000 | (native baseline) | 146,481 | 159,442 | 152,166 |
| column_four `replay_throughput` | 3,200 | 3,334 | 2,885 | 2,896 | 2,879 |
| column_four `replay_step_projection` | 33,000 | 34,175 | 30,494 | 31,170 | 30,708 |
| three_marks `public_view_generation` | 200,000 | (native baseline) | 198,156 | 196,178 | 196,580 |
| three_marks `random_playout` | 35,000 | (native baseline) | 34,830 | — | 34,134 |
| three_marks `level1_bot_decision` | 35,000 | (native baseline) | — | — | 34,966 |

Two facts shape the decision:

1. **The ADR 0003 calibration sample was unrepresentative, not the steady
   state.** The three post-calibration merge runs are tightly clustered among
   themselves (`level2_bot_decision`: 1,438 / 1,450 / 1,370; `random_playout`:
   5,939 / 6,173 / 5,972), while the single `BENCICAL-001` calibration run was
   roughly **2× faster** on the compute-bound bot and playout operations. ADR
   0003 calibrated floors below that one fast sample, so every representative run
   on the typical (slower) runner instance falls below. Calibrating to a single
   CI observation is the defect; the steady-state runner is consistent enough to
   floor against once multiple runs are sampled.

2. **Some operations were never CI-recalibrated at all.** `column_four`
   `apply_action` and `three_marks` `public_view_generation` / `random_playout` /
   `level1_bot_decision` still carry native WSL2 `measured_baseline` floors,
   because `BENCICAL-001`/`BENCICAL-002` calibrated from the single fast run and
   left these heavy validated paths on their native numbers. They breach the
   slower steady-state runner by 2–27%.

`docs/TESTING-REPLAY-BENCHMARKING.md` §15 forbids lowering a threshold "only to
make CI green" and requires keeping a miss visible. An ADR is required because the
fix amends ADR 0003's calibration doctrine — single-CI-observation floors — which
is doctrine fixed by an accepted ADR and the testing law; FOUNDATIONS §13 says
that may be superseded only by an accepted ADR. This keeps public playable
Rulepath unblocked (a permanently red `main` is removed) while preserving honest,
hard-failing performance gating.

## Decision

Gate 2 benchmark floors MUST be **variance-aware**: calibrated below the
*minimum* observed value across multiple representative CI runs, not below a
single observation.

- Each `thresholds.json` `threshold` value for a blocking operation MUST be a
  conservative floor below the **minimum** `ubuntu-latest` measurement observed
  for that operation across **at least three** representative scheduled / manual /
  `main`-push gate runs. The committed `threshold` is the gate value.
- The safety margin below that observed minimum MUST be at least **15%**
  (`threshold` ≤ `0.85 × min_observed`, rounded down). This margin absorbs the
  residual run-to-run variance of the shared runner fleet.
- The unrepresentative single-sample calibration run (`BENCICAL-001`,
  `27087214359`) MUST NOT be used as the calibration minimum; it is recorded only
  as the historical fast outlier.
- Operations still carrying native `measured_baseline` floors that breach the CI
  runner MUST be recalibrated under the same rule; their native targets MUST stay
  documented in the game's `BENCHMARKS.md`.
- Each game's `BENCHMARKS.md` MUST record the native target, the observed CI
  minimum, and the committed variance-aware floor. No performance figure is
  hidden.
- The scheduled / manual / `main`-push lane keeps running **every** game and
  aggregating failures (ADR 0003 stands for the run-all-games hardening). The
  pull-request smoke lane is unchanged (ADR 0002 stands for the lane split).
- The CI workflow itself (`.github/workflows/gate-2-benchmarks.yml`) is
  **unchanged** by this ADR; only threshold values, rationales, and docs change.

This ADR amends ADR 0003: "a conservative floor below the stable `ubuntu-latest`
measurement" becomes "a conservative floor (≥15% margin) below the minimum across
at least three representative `ubuntu-latest` runs." ADR 0001's 100,000 games/sec
`random_playout` figure remains the documented **native** target. The `race_to_n`
audit required by the operational requirements found `random_playout`'s ADR 0003
floor (65,000) sitting only ~2.6% below the observed CI minimum (66,761), so under
this ADR's ≥15% rule it is widened to 56,000 (`0.85 × 66,761`, rounded down). Its
`rationale_class` therefore moves from `accepted_adr` to `conservative_ci_floor`,
because the enforced floor is now a variance-aware CI floor derived like the other
recalibrated operations rather than ADR 0001's budget figure; the 100,000 native
target stays recorded in `games/race_to_n/docs/BENCHMARKS.md`.

Regression-relative gating against a stored per-runner baseline — the most robust
fix, which removes absolute-throughput calibration entirely — is the **committed
next step**. It is deferred from this ADR's scope because it requires a
baseline-artifact mechanism and a `bench-report` change; it MUST be taken up in a
separate ADR when the project gains a controlled runner or baseline-artifact
store, and revisited promptly if the variance-aware floors flake again.

The exact recalibrated values are committed by the implementation tickets
(`BENCICAL-003`, `BENCICAL-004`) from CI evidence, not invented here.

## Alternatives considered

| Alternative | Why considered | Why accepted/rejected |
|---|---|---|
| Variance-aware floors: floor ≥15% below the min across ≥3 representative CI runs; recalibrate the un-migrated `measured_baseline` ops; name regression-relative gating as the next step | The three steady-state runs are tightly clustered, so a margin below their minimum is robust without new infrastructure; ADR 0003 named this class of fix as the planned follow-up. | Accepted. Minimal, honest, keeps the gate hard-failing, calibrates to representative CI evidence rather than one fast sample, keeps native baselines on the record, and changes no PR behavior or workflow file. |
| Regression-relative gating now (stored baseline + relative check) | Most robust; immune to fleet heterogeneity; named by ADR 0003 as the robust long-term fix. | Rejected for this scope. Requires a baseline-artifact mechanism and a `bench-report` parser/logic change before `main` goes green; recorded as the committed next step instead. |
| Re-floor below the single latest CI sample again (repeat ADR 0003's method) | Smallest data edit. | Rejected. It repeats the exact defect — single-observation calibration — and will breach again on the next slightly slower runner instance. |
| Make the `main`/scheduled lane non-gating smoke too | Turns `main` green immediately. | Rejected. Hides all throughput regressions and contradicts TESTING §15/§16 "do not hide unknown performance" and the hard-fail requirement; ADR 0003 rejected this for the same reason. |
| Leave the gate failing | Zero change. | Rejected. A permanently red `main` trains maintainers to ignore the gate, which is worse than no gate. |

## Consequences

Positive consequences:

- `main` stops failing on a calibration-sample artifact that ADR 0003 already
  predicted; the gate becomes meaningful again instead of permanently red.
- Benchmark gating stays hard-failing on the `main`/scheduled/manual lane,
  calibrated to a representative minimum of the environment that runs it.
- Native targets remain recorded in each game's `BENCHMARKS.md`, so no
  performance figure is hidden or lost.
- The calibration rule now requires multiple samples, so a single fast or slow
  runner instance can no longer set the gate.

Negative or risky consequences:

- A ≥15% margin below the observed minimum makes the floor catch only material
  regressions; a regression between the floor and the native target is not caught
  on CI. Mitigation: the native target stays documented in `BENCHMARKS.md` as the
  off-CI standard, and regression-relative gating is the committed next step.
- The shared runner fleet could still add a >15%-slower hardware generation,
  re-breaching the floors. Mitigation: the margin is conservative against the
  three-run spread; a recurrence triggers the committed regression-relative ADR
  rather than another absolute re-floor.

Operational requirements:

- `games/three_marks/benches/thresholds.json` and
  `games/column_four/benches/thresholds.json` recalibrated to variance-aware
  floors; `games/race_to_n/benches/thresholds.json` audited and widened only
  where margin is under 15% (`BENCICAL-003`).
- `games/three_marks/docs/BENCHMARKS.md` and
  `games/column_four/docs/BENCHMARKS.md` record native target, CI minimum, and
  committed floor (`BENCICAL-003`).
- `docs/TESTING-REPLAY-BENCHMARKING.md` §15 and §17 reference this ADR for the
  variance-aware calibration doctrine (`BENCICAL-004`).
- `.github/workflows/gate-2-benchmarks.yml` unchanged.

## Determinism impact

- No change to RNG, iteration order, clocks, floating point, parallelism,
  serialization order, replay, or hashes. Only benchmark policy data and docs
  change.
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
  (no new field, format, expression, selector, behavior ID, variant, or version);
  `schema_version` stays 1. Only `threshold` numeric values and
  `rationale`/`rationale_class` strings change. Behavior stays in typed Rust; no
  DSL pressure.

## `engine-core` contamination risk

- None. No `engine-core` change. The decision is benchmark policy data and docs.

## `game-stdlib` / primitive-pressure impact

- None. No shared primitive is introduced or promoted.

## UI impact

- None. No action trees, previews, effect-log animation, renderer boundaries,
  accessibility, reduced motion, or inspector behavior change. TypeScript remains
  presentation-only.

## Bot impact

- The `level1_bot_decision` / `level2_bot_decision` benchmark floors are
  recalibrated to variance-aware CI values, but bot views, legal action APIs,
  candidate ranking, explanations, and hidden-information safety are unchanged.
  No v1/v2-excluded AI technique is introduced.

## IP impact

- None. No public naming, prose, assets, fonts, fixtures, traces, or
  browser-shipped bundles change. Benchmark numbers are non-proprietary.

## Benchmark impact

- No native benchmark is added or removed. Threshold **values** for the breaching
  operations are recalibrated to variance-aware `ubuntu-latest` floors; native
  targets are preserved in `BENCHMARKS.md`.
- The Stage 1 `random_playout` native target remains 100,000 games/sec per ADR
  0001; its enforced CI floor is widened from 65,000 to 56,000 under this ADR's
  ≥15% margin rule (the ADR 0003 floor was only ~2.6% below the observed CI
  minimum), and its `rationale_class` becomes `conservative_ci_floor`.
- No public latency budget changes. No WASM/browser smoke benchmark is added.

## Migration notes

Existing docs to update:

- `docs/TESTING-REPLAY-BENCHMARKING.md` §15 and §17 — reference this ADR for the
  variance-aware calibration doctrine (extends the existing ADR 0003 reference).
- `games/three_marks/docs/BENCHMARKS.md`, `games/column_four/docs/BENCHMARKS.md` —
  record native target, CI minimum, and variance-aware floor.

Existing games to back-port:

- `three_marks`, `column_four` threshold files recalibrated. `race_to_n` audited
  and widened only where margin is under 15%. `directional_flip`,
  `draughts_lite`, and `high_card_duel` are non-blocking (`1`) and need no change
  until they have stable CI measurements.

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
  each affected game's `BENCHMARKS.md` updated.
