# River Ledger Benchmarks

Game ID: `river_ledger`

Variant: `river_ledger_standard`

Rules version: `river-ledger-rules-v2`

Date: 2026-06-20

## Scope

Native benchmarks cover the Gate 15 base paths and the Gate 15.1 all-in /
side-pot pressure paths expected to vary with seat count, stack profile, and
pot layer shape:

- 3-seat equal-stack setup;
- 6-seat asymmetric-stack setup;
- 6-seat setup/deal;
- initial legal-action generation;
- short-stack legal-action generation;
- apply/validate for a known call;
- apply/validate for a stack-capped short raise all-in;
- six-seat maximum-layer side-pot construction;
- six-seat multi-pot allocation with split winners;
- six-seat all-in showdown resolution;
- projection for observer plus all six seats;
- terminal multi-pot projection for observer plus all six seats;
- public replay export/import;
- six-seat multi-pot public replay serialization;
- evaluator batch equivalent to six seats each checking a 7-card hand;
- Level 2 policy selection under short-stack pressure;
- seeded Level 2 full playout;
- seeded six-seat full playout from asymmetric all-in pressure stacks.

The maximum-layer construction fixture is deterministic and exercises six
distinct contribution caps, folded money, at least three contestable pots, a
returned top layer, and a split pot allocation. The all-in showdown/projection
fixture uses a no-return sibling shape because terminal showdown conservation
excludes already-returned uncalled excess.

Thresholds in [thresholds.json](../benches/thresholds.json) are provisional
native smoke floors calibrated under the accepted ADR 0002 / ADR 0003 benchmark
process until repeated CI hardware baselines are available. They are not allowed
to justify lookup-table evaluators, hidden-state shortcuts, weaker explanations,
or reduced no-leak coverage. Accepted ADR 0005 governs variance-aware CI-floor
calibration once repeated runner measurements are available for these lanes.

## Native Evidence

Run:

```bash
cargo bench -p river_ledger
```

The benchmark executable prints a CSV-style summary plus a JSON block delimited
by `BEGIN_RIVER_LEDGER_BENCHMARK_JSON` and
`END_RIVER_LEDGER_BENCHMARK_JSON`.

Current native lane names:

- `setup_3p_equal_stacks`
- `setup_6p_asymmetric_stacks`
- `setup_deal_6p`
- `legal_actions_initial_6p`
- `legal_actions_short_stack`
- `apply_call_6p`
- `apply_short_all_in_raise`
- `construct_side_pots_6p_max_layers`
- `allocate_side_pots_6p_split_winners`
- `resolve_all_in_showdown_6p`
- `project_all_viewers_6p`
- `project_view_6p_multi_pot`
- `public_export_import_6p`
- `serialize_replay_6p_multi_pot`
- `evaluator_showdown_batch_6p`
- `bot_policy_6p_short_stack`
- `level2_full_playout_6p`
- `full_game_6p_all_in_pressure`

## Simulation Evidence

River Ledger is the first official 3-6 seat simulator consumer. With no
explicit `--seat-count`, the native simulator cycles 3, 4, 5, and 6 seats and
cycles the deterministic `default`, `asymmetric`, and `short_pressure` stack
profiles. Its summary reports `games_by_seat_count` and
`games_by_stack_profile` so the coverage is visible in CI output:

```bash
cargo run -p simulate -- --game river_ledger --games 1000
```

An explicit `--seat-count 3`, `--seat-count 4`, `--seat-count 5`, or
`--seat-count 6` pins the seat count while still cycling stack profiles. The
output uses seat-keyed `wins_by_seat` entries for all configured seats rather
than two-seat-only counters.

## Current Caveats

- Benchmark thresholds are smoke floors, not tuned release gates.
- WASM and browser performance evidence remains smoke evidence, not the native
  threshold source.
- `fixture-check` still validates River Ledger trace fixtures through the
  existing placeholder trace contract. The Gate 15.1 maximum-layer fixture
  invariants are asserted in the native benchmark harness.
