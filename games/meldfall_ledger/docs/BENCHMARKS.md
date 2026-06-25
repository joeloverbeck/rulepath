# Meldfall Ledger Benchmarks

Game ID: `meldfall_ledger`

Variant: `classic_500_single_deck_v1`

Rules version: `meldfall-ledger-rules-v1`

Date: 2026-06-26

## Scope

Native Rust is the benchmark source of truth. Browser and WASM checks remain
later-ticket smoke evidence only until the web tickets land.

The benchmark harness covers the variable-seat Meldfall Ledger surface:

- setup and deterministic random-legal actions at 2, 4, and 6 seats;
- legal action tree generation, apply, and all-viewer projection at max seat count;
- long public discard-tail action generation;
- larger public meld tableau projection for public plus all six seat-private viewers;
- viewer-scoped replay export/import for public plus all six seat-private viewers;
- Level 0 random-legal bot decisions through the Rust legal action tree;
- the Level 1 profile as an explicit not-admitted status check.

Thresholds in [thresholds.json](../benches/thresholds.json) are provisional
native smoke floors under ADR 0002, ADR 0003, and ADR 0005 benchmark policy
until repeated CI-runner baselines are available. Calibration may replace a
provisional floor only from measured evidence, and may not remove a workload,
bypass visibility filtering, weaken legal-action validation, skip bot legality,
or turn the Level 1 not-admitted status into an implied strategy claim.

## Native Evidence

Run:

```bash
cargo bench -p meldfall_ledger
```

The benchmark executable prints a CSV-style summary plus a JSON block delimited
by `BEGIN_MELDFALL_LEDGER_BENCHMARK_JSON` and
`END_MELDFALL_LEDGER_BENCHMARK_JSON`.

Current native lanes:

- `native_2p_short_round`
- `native_4p_default`
- `native_6p_large_surface`
- `large_discard_tail`
- `large_public_tableau`
- `replay_export_import`
- `l0_bot_decision`
- `l1_bot_decision`

## Profile Targets

| Profile | Operation | Provisional target | Current threshold file posture |
|---|---|---:|---|
| `native_2p_short_round` | Setup plus 200 deterministic random-legal actions. | p95 under 2 ms/action after calibration. | Smoke floor: 1 action/s. |
| `native_4p_default` | Setup plus 500 deterministic random-legal actions. | p95 under 3 ms/action after calibration. | Smoke floor: 1 action/s. |
| `native_6p_large_surface` | Legal action tree, apply, and view projection for public plus all six seat viewers. | p95 under 8 ms/action after calibration. | Smoke floor: 1 action/s. |
| `large_discard_tail` | Generate legal discard-pickup choices for a long visible discard pile. | p95 under 5 ms/action-tree generation after calibration. | Smoke floor: 1 tree/s. |
| `large_public_tableau` | Project public plus all six seat-private views for a larger meld tableau. | p95 under 12 ms total after calibration. | Smoke floor: 1 viewer set/s. |
| `replay_export_import` | Export/import public plus all six seat-private views. | p95 under 20 ms per fixture after calibration. | Smoke floor: 1 export set/s. |
| `l0_bot_decision` | Random legal selection from the Rust legal action tree. | p95 under 1 ms after calibration. | Smoke floor: 1 decision/s. |
| `l1_bot_decision` | Level 1 profile receipt. | p95 under 10 ms only after an L1 policy is admitted. | Smoke floor: 1 status check/s; L1 is not admitted. |

## Seed And Fixture Manifest

| Workload family | Seed/fixture source | Notes |
|---|---|---|
| 2-seat native actions | seed `19200` | setup plus deterministic L0-driven draw/table/discard transitions. |
| 4-seat native actions | seed `19400` | default seat count with 500 deterministic L0-driven actions. |
| 6-seat large surface | seed `19600` | max-seat legal tree, apply, and all-viewer projection. |
| discard tail | seed `19610` plus fixed public discard tail | action-tree generation only; stock order remains hidden. |
| public tableau | seed `19621` plus fixed public meld groups | projection lane for public and six seat-private viewers. |
| replay export/import | same large-tableau state | viewer-scoped export/import and stable-string hashing. |
| L0 bot | seed `19620`, bot seed by iteration | legal random action selection only. |
| L1 profile | `not_admitted_pending_strategy_evidence` | status check only; no Level 1 policy is admitted. |

## Provisional Posture

The target posture from Gate 19 is:

- p95 below 2 ms/action for `native_2p_short_round`;
- p95 below 3 ms/action for `native_4p_default`;
- p95 below 8 ms/action for `native_6p_large_surface`;
- p95 below 5 ms/action-tree generation for `large_discard_tail`;
- p95 below 12 ms total for `large_public_tableau`;
- p95 below 20 ms per fixture for `replay_export_import`;
- p95 below 1 ms for `l0_bot_decision`;
- p95 below 10 ms for `l1_bot_decision` only after Level 1 is admitted.

This ticket records smoke thresholds rather than CI-calibrated floors. The
current L0 playout evidence is bounded and nonterminal; terminal strategy
benchmarking remains blocked until a competent policy is admitted.

## CI Strategy

Pull requests may run a filtered smoke lane for compilation and basic execution:

```bash
cargo bench -p meldfall_ledger -- native_4p_default
```

Scheduled, workflow-dispatch, and main-branch performance gates can run the full
native lane:

```bash
cargo bench -p meldfall_ledger
cargo run -p bench-report -- --input <report> --thresholds games/meldfall_ledger/benches/thresholds.json
```

The threshold file records benchmark policy data consumed by bench tooling. It
is not rule behavior and does not affect deterministic game state.
