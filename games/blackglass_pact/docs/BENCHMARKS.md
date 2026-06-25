# Blackglass Pact Benchmarks

Game ID: `blackglass_pact`

Variant: `blackglass_pact_standard`

Rules version: `blackglass-pact-rules-v1`

Date: 2026-06-25

## Scope

Native Rust is the benchmark source of truth. Browser and WASM checks remain
product smoke evidence only until the web tickets land.

The benchmark harness covers the fixed-four Blackglass Pact surface:

- setup, blind commitment, deterministic shuffle, and deal;
- blind, bid, and play legal-action generation;
- bid validation plus apply;
- card validation plus apply;
- spades-trump trick resolution through the promoted helper seam;
- hand scoring, including team contracts, nils, blind nils, bags, and terminal posture;
- observer projection and all four seat-private projections;
- viewer-scoped replay export/import;
- Level 0 and bounded Level 1 bot decisions through legal action APIs;
- a fixed-four seeded bot-smoke match lane with stable seat/team output.

The operation names are fixed-four and end in `_4p` or `_smoke_4p`. Benchmark
output includes stable `seat_0..seat_3` and `team_0..team_1` order fields.

Thresholds in [thresholds.json](../benches/thresholds.json) are provisional
native smoke floors under ADR 0002, ADR 0003, and ADR 0005 benchmark policy
until repeated CI-runner baselines are available. The bot-smoke match lane
carries the Gate 18 provisional floor of at least 75 matches per second.
Calibration may replace a provisional floor only from measured evidence, and
may not remove a workload, bypass visibility filtering, weaken legal-action
validation, skip bot legality, or delete the promoted-helper trick lane.

## Native Evidence

Run:

```bash
cargo bench -p blackglass_pact
```

The benchmark executable prints a CSV-style summary plus a JSON block delimited
by `BEGIN_BLACKGLASS_PACT_BENCHMARK_JSON` and
`END_BLACKGLASS_PACT_BENCHMARK_JSON`.

Current native lanes:

- `setup_blind_deal_4p`
- `legal_tree_blind_4p`
- `legal_tree_bid_4p`
- `legal_tree_play_4p`
- `validate_apply_bid_4p`
- `validate_apply_play_4p`
- `promoted_helper_trick_resolution_4p`
- `score_hand_4p`
- `project_observer_4p`
- `project_all_seats_4p`
- `replay_export_import_4p`
- `l0_decision_4p`
- `l1_decision_4p`
- `full_seeded_match_smoke_4p`

## Seed And Fixture Manifest

| Workload family | Seed/fixture source | Notes |
|---|---|---|
| setup/blind/deal | seeds `180400+` | exercises blind decisions before deal. |
| legal trees | fixed seeds `180401` through `180403` | blind, bidding, and play states. |
| scoring | seed `180404` plus explicit bid/trick/bag state | deterministic hand-score lane. |
| exports/views | fixed bidding state seed `180402` | observer plus all four seats. |
| bots | fixed bidding state seed `180402`, bot seeds by iteration | L0/L1 legal-decision lanes. |
| match smoke | seeds `180400+` | setup, observer export, and L0/L1 decisions with team-summary output. |

## Provisional Posture

The target posture from Gate 18 remains:

- native p95 below 1 ms/op for setup, legal trees, scoring, views, exports, and bots;
- export p95 below 50 ms;
- at least 75 matches/s for the seeded match lane;
- zero unexplained cap breaches in the 1,000-match simulator corpus.

This ticket records provisional smoke thresholds rather than CI-calibrated
floors. Full terminal-match benchmarking is not claimed yet because the current
Blackglass simulator and replay-check registrations are bounded smoke surfaces,
not full terminal command replay drivers.

## Helper Evidence

Blackglass Pact uses `game-stdlib::trick_taking::follow_suit_indices` and
`game-stdlib::trick_taking::winning_play_index` through game-local rule code.
The `promoted_helper_trick_resolution_4p` lane keeps the spades-trump
resolution seam visible for later comparison with Plain Tricks, Vow Tide, and
Briar Circuit.

## CI Strategy

Pull requests may run a filtered smoke lane for compilation and basic execution:

```bash
cargo bench -p blackglass_pact -- full_seeded_match_smoke_4p
```

Scheduled, workflow-dispatch, and main-branch performance gates can run the full
native lane:

```bash
cargo bench -p blackglass_pact
cargo run -p bench-report -- --input <report> --thresholds games/blackglass_pact/benches/thresholds.json
```

The threshold file records benchmark policy data consumed by bench tooling. It
is not rule behavior and does not affect deterministic game state.
