# Vow Tide Benchmarks

Game ID: `vow_tide`

Variant: `vow_tide_standard`

Rules version: `vow-tide-rules-v1`

Date: 2026-06-21

## Scope

Native Rust is the benchmark source of truth. Browser and WASM checks remain
product smoke evidence only.

The benchmark harness covers each supported seat count, from 3 through 7 seats:

- setup, deterministic shuffle, and deal;
- first bidder legal-action generation;
- dealer-hook legal-action generation;
- lead-card and follow/void legal-action generation;
- bid validation plus apply;
- card validation plus apply;
- final-card trick resolution;
- hand scoring;
- observer projection and all selected seat-private projections;
- observer plus all seat effect filtering;
- replay snapshot and viewer-scoped export/import;
- Level 0 and Level 1 bot decisions through legal action APIs;
- complete seeded matches through terminal outcome.

The operation names are seat-suffixed (`_3p` through `_7p`) wherever seat count
changes the surface area. The largest fixture uses seven seats, the maximum
supported schedule for that seat count, all seven seat viewers, dealer-hook
bidding, constrained trick play, and a full deterministic match driven only
through validated legal commands.

Thresholds in [thresholds.json](../benches/thresholds.json) are provisional
native smoke floors under ADR 0002 and ADR 0003 benchmark policy until repeated
CI-runner baselines are available. The complete-match lanes carry the Gate 17
provisional floor of at least 75 completed matches per second for each supported
seat count. Calibration may replace a provisional floor only from measured
evidence, and may not remove an operation, bypass visibility filtering, weaken
dealer-hook or follow-suit validation, or shortcut bot legality.

## Native Evidence

Run:

```bash
cargo bench -p vow_tide
```

The benchmark executable prints a CSV-style summary plus a JSON block delimited
by `BEGIN_VOW_TIDE_BENCHMARK_JSON` and `END_VOW_TIDE_BENCHMARK_JSON`.

Current native lane families:

- `setup_deal_{3p,4p,5p,6p,7p}`
- `bid_legal_first_{3p,4p,5p,6p,7p}`
- `bid_legal_dealer_hook_{3p,4p,5p,6p,7p}`
- `play_legal_lead_{3p,4p,5p,6p,7p}`
- `play_legal_follow_{3p,4p,5p,6p,7p}`
- `validate_apply_bid_{3p,4p,5p,6p,7p}`
- `validate_apply_play_{3p,4p,5p,6p,7p}`
- `trick_resolution_{3p,4p,5p,6p,7p}`
- `score_hand_{3p,4p,5p,6p,7p}`
- `project_observer_{3p,4p,5p,6p,7p}`
- `project_all_seats_{3p,4p,5p,6p,7p}`
- `effect_filter_all_viewers_{3p,4p,5p,6p,7p}`
- `replay_snapshot_export_import_{3p,4p,5p,6p,7p}`
- `l0_decision_{3p,4p,5p,6p,7p}`
- `l1_decision_{3p,4p,5p,6p,7p}`
- `full_seeded_match_{3p,4p,5p,6p,7p}`

## Helper Evidence

The Vow Tide legal-card path uses the promoted
`game_stdlib::trick_taking::follow_suit_indices` helper. Earlier helper
admission and back-port tickets established the before/after conformance seam
for Plain Tricks and Briar Circuit. This gate keeps that evidence connected by
requiring the sibling helper/game benchmark lanes to remain materially stable:

```bash
cargo bench -p game-stdlib
cargo bench -p plain_tricks
cargo bench -p briar_circuit
```

Those comparison runs are performance evidence only. They do not change
deterministic replay, hashes, visibility, or bot legality.

## CI Strategy

Pull requests may run a filtered smoke lane for compilation and basic execution:

```bash
cargo bench -p vow_tide -- full_seeded_match_7p
```

Scheduled, workflow-dispatch, and main-branch performance gates can run the full
native lane:

```bash
cargo bench -p vow_tide
cargo run -p bench-report -- --input <report> --thresholds games/vow_tide/benches/thresholds.json
```

The threshold file records benchmark policy data consumed by bench tooling. It
is not rule behavior and does not affect deterministic game state.
