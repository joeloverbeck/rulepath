# River Ledger Benchmarks

Game ID: `river_ledger`

Variant: `river_ledger_standard`

Rules version: `river-ledger-rules-v1`

Date: 2026-06-14

## Scope

Native benchmarks cover the Gate 15 paths expected to vary with seat count:

- 6-seat setup/deal;
- initial legal-action generation;
- apply/validate for a known call;
- projection for observer plus all six seats;
- public replay export/import;
- evaluator batch equivalent to six seats each checking a 7-card hand;
- seeded Level 2 full playout.

Thresholds in [thresholds.json](../benches/thresholds.json) are provisional
native smoke floors until repeated CI hardware baselines are available. They are
not allowed to justify lookup-table evaluators, hidden-state shortcuts, weaker
explanations, or reduced no-leak coverage.

## Native Evidence

Run:

```bash
cargo bench -p river_ledger
```

The benchmark executable prints a CSV-style summary plus a JSON block delimited
by `BEGIN_RIVER_LEDGER_BENCHMARK_JSON` and
`END_RIVER_LEDGER_BENCHMARK_JSON`.

## Simulation Evidence

River Ledger is the first official 3-6 seat simulator consumer. The native
simulator accepts:

```bash
cargo run -p simulate -- --game river_ledger --seat-count 6 --games 1000 --start-seed 1506
```

The output uses seat-keyed `wins_by_seat` entries for all configured seats
rather than two-seat-only counters.

## Current Caveats

- Benchmark thresholds are smoke floors, not tuned release gates.
- WASM and browser performance evidence is owned by later WASM/web tickets.
- Tool registration for replay/fixture/rule coverage is owned by
  GAT15RIVLEDTEX-015.
