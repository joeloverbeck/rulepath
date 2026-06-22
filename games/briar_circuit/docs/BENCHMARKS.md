# Briar Circuit Benchmarks

Game ID: `briar_circuit`

Variant: `briar_circuit_standard`

Rules version: `briar-circuit-rules-v1`

Date: 2026-06-21

## Scope

Native Rust is the benchmark source of truth. Browser and WASM checks remain
secondary product-latency smoke evidence.

The benchmark harness covers the Gate 16 required operations:

- setup, shuffle, deal, canonical summary, and replay hash snapshot;
- pass legal-action generation, pass selection apply, fourth confirmation, and
  atomic exchange;
- play legal-action generation from a maximum hand and a constrained follow
  state;
- command validation plus play apply;
- fourth-card trick resolution;
- normal scoring, moon scoring, threshold/tie/outcome construction;
- observer projection plus all four seat-private projections;
- public plus seat-private effect filtering;
- full-hand replay hash timeline;
- viewer-scoped public and seat-private export/import;
- Level 0 and Level 1 bot action selection through legal action APIs;
- complete seeded 13-trick hand;
- complete near-threshold seeded terminal match.

The largest native fixture uses four seats, 13-card private hands, pass
commitments pending until atomic exchange, all four seat viewers, a full
13-trick hand, and a near-threshold terminal match fixture. The terminal-match
fixture sets cumulative scores near the match threshold, then drives the
remaining hand only through validated legal pass/play commands.

Thresholds in [thresholds.json](../benches/thresholds.json) are provisional
native smoke floors under accepted ADR 0002 and ADR 0003 benchmark policy until
repeated CI-runner baselines are available. Accepted ADR 0005 governs
variance-aware CI-floor calibration once repeated runner measurements are
available. Calibration may replace a provisional floor only from measured
evidence, and may not remove an operation, bypass visibility filtering, weaken
outcome explanation detail, or introduce lookup/search shortcuts.

## Native Evidence

Run:

```bash
cargo bench -p briar_circuit
```

The benchmark executable prints a CSV-style summary plus a JSON block delimited
by `BEGIN_BRIAR_CIRCUIT_BENCHMARK_JSON` and
`END_BRIAR_CIRCUIT_BENCHMARK_JSON`.

Current native lane names:

- `setup_shuffle_deal_serialize`
- `pass_legal_actions`
- `pass_select_apply`
- `pass_commit_exchange`
- `play_legal_actions_max_hand`
- `play_legal_actions_follow`
- `validate_apply_play`
- `trick_resolution`
- `normal_hand_scoring`
- `moon_hand_scoring`
- `threshold_outcome`
- `project_observer_view`
- `project_four_seat_views`
- `effect_filter_all_viewers`
- `full_internal_trace_replay`
- `viewer_scoped_export_import_public`
- `viewer_scoped_export_import_seat`
- `l0_action_selection`
- `l1_action_selection`
- `full_seeded_hand`
- `full_seeded_match_terminal`

`full_seeded_match_terminal` carries the provisional native floor of at least
100 completed near-threshold matches per second. The other operations currently
use non-blocking smoke floors until repeated runner measurements support tighter
CI-calibrated thresholds.

## CI Strategy

Pull requests run a non-gating bench smoke for compilation and basic execution:

```bash
cargo bench -p briar_circuit -- legal_actions
```

The scheduled, workflow-dispatch, and push-to-main gate runs `cargo bench -p
briar_circuit`, captures the native report, and validates it with:

```bash
cargo run -p bench-report -- --input <report> --thresholds games/briar_circuit/benches/thresholds.json
```

The threshold file records the benchmark environment policy data consumed by
`bench-report`; it is not rule behavior and does not affect deterministic
replay, hashes, visibility, or bot legality.
