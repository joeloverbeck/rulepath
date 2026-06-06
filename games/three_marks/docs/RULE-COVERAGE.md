# Three Marks Rule Coverage Matrix

Game ID: `three_marks`

Rules version: `three_marks-rules-v1`

Data/manifest version: `1`

Engine version: `0.1.0`

Prepared by: `Codex`

Last updated: 2026-06-06

## Purpose

This matrix maps every stable rule ID in `RULES.md` to implementation and evidence. Rust tests, traces, replay checks, and serialization tests are the rule authority; UI smoke proves browser integration only.

## Status labels

| Status | Meaning |
|---|---|
| covered | Implementation and required evidence exist. |
| covered-by-trace | Golden trace or replay evidence is the primary proof. |
| not-applicable | The surface truly does not apply, with rationale. |
| intentionally-deferred | Deferred by a documented gate/stage decision. |
| unsupported | Explicitly not implemented for this variant. |

## Primary Evidence Files

| Evidence | Coverage role |
|---|---|
| `games/three_marks/src/setup.rs` | deterministic setup and fixed variant |
| `games/three_marks/src/actions.rs` | Rust legal placement action tree |
| `games/three_marks/src/rules.rs` | validation, placement, turn change, terminal outcome |
| `games/three_marks/src/effects.rs` | viewer-safe semantic effects |
| `games/three_marks/src/visibility.rs` | Rust-projected public board view |
| `games/three_marks/src/bots.rs` | Level 0 and Level 1 bot policies |
| `games/three_marks/src/replay_support.rs` | replay hashes and board projections |
| `games/three_marks/tests/rule_tests.rs` | named rule behavior tests |
| `games/three_marks/tests/property_tests.rs` | action and sequence invariants |
| `games/three_marks/tests/replay_tests.rs` | golden trace and replay hash checks |
| `games/three_marks/tests/serialization_tests.rs` | JSON/stable serialization checks |
| `games/three_marks/tests/visibility_tests.rs` | public-view and no-leak checks |
| `games/three_marks/tests/bot_tests.rs` | bot legality, determinism, explanations |
| `games/three_marks/tests/golden_traces/*.trace.json` | normal, terminal, draw, diagnostic, bot, and WASM-exported replay evidence |
| `apps/web/e2e/three-marks.smoke.mjs` | browser board/replay/a11y/no-leak integration |
| `games/three_marks/benches/three_marks.rs` | native hot-path benchmark evidence |

## Rule Coverage Matrix

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `TM-SCOPE-001` | Deterministic two-seat fixed-board perfect-information game. | `ThreeMarksState`, `setup_match`, `validate_command`, `apply_action`. | `cargo test -p three_marks`; golden traces; browser smoke. | covered | No rule behavior uses wall-clock or OS entropy. |
| `TM-SCOPE-002` | Public name is `Three Marks`. | manifest, docs, catalog/WASM metadata. | `static_data_parses_and_rejects_unknown_fields`; `three-marks.smoke.mjs`. | covered | Neutral Rulepath-owned naming. |
| `TM-SCOPE-003` | Rust owns setup, legality, effects, bots, replay, views. | game crate APIs consumed by WASM/web/tools. | rule tests; WASM tests; browser smoke. | covered | TypeScript renders Rust output only. |
| `TM-VAR-001` | Only `three_marks_standard` ships. | `Variant::three_marks_standard`, `SetupOptions::default`. | variant tests; fixture/static data checks. | covered | No variant picker changes rule behavior. |
| `TM-VAR-002` | Fixed 3 by 3, two seats, alternating turns, line wins, draws. | `variants.rs`, `setup.rs`, `rules.rs`. | setup, rule, replay, UI smoke. | covered | Core Gate 4 variant. |
| `TM-COMP-001` | Public 3 by 3 board. | `ThreeMarksState.cells`, `PublicView.board_rows/columns`. | visibility tests; browser board smoke. | covered | Board vocabulary remains game-local. |
| `TM-COMP-002` | Stable cells `r1c1` through `r3c3`. | `CellId`, `actions.rs`, `visibility.rs`. | action-id property tests; rule tests. | covered | Test IDs expose public cell ids only. |
| `TM-COMP-003` | Public mark tokens. | `CellOccupancy`, `ui.rs`, `ThreeMarksBoard.tsx`. | visibility tests; browser smoke/checklist. | covered | Color plus shape, original assets. |
| `TM-COMP-004` | Public seats `seat_0` and `seat_1`. | `ThreeMarksSeat`, setup state. | setup tests; rule tests; public view tests. | covered | Public ids match action/replay surfaces. |
| `TM-COMP-005` | Eight public winning lines. | `winning_line`, `WinningLine`. | row/column/diagonal tests; terminal traces; browser win smoke. | covered | Rust reports exact ordered line. |
| `TM-SETUP-001` | New match starts empty. | `setup_match`, `CellOccupancy::empty_cells`. | setup tests; replay reset; UI smoke. | covered | No setup choices. |
| `TM-SETUP-002` | `seat_0` acts first. | `setup_match` active seat. | setup tests; browser start smoke. | covered | No first-player randomness. |
| `TM-SETUP-003` | All seats/cells visible from setup. | `project_view`, explicit empty private view. | visibility no-leak tests; a11y/no-leak smoke. | covered | Perfect information. |
| `TM-TURN-001` | Active seat submits one placement with freshness token. | `legal_action_tree`, `validate_command`. | rule tests; browser legal-cell click. | covered | UI dispatches Rust action ids. |
| `TM-TURN-002` | Non-terminal placement passes turn. | `apply_action` active-seat flip and effect. | `valid_action_places_mark_advances_turn_ply_and_token`; traces. | covered | Emits `ActivePlayerChanged`. |
| `TM-TURN-003` | Terminal placement stops normal turns. | terminal branch in `apply_action`. | terminal rule tests; terminal trace; win/draw smoke. | covered | Legal targets become empty. |
| `TM-ACTION-001` | Empty cells are legal placements for active actor. | `legal_cells`, `action_choice`, `validate_command`. | `legal_actions_are_empty_cells_for_active_actor`; property tests. | covered | Flat action ids `place/<cell>`. |
| `TM-ACTION-002` | Occupied cells are not legal targets. | `legal_cells` filters occupancy; validation rejects occupied. | occupied rule tests; occupied-diagnostic trace; browser inert check. | covered | UI inertness follows missing Rust target. |
| `TM-ACTION-003` | Terminal state has no placement actions. | `legal_action_tree`/validation terminal checks. | terminal rule tests; terminal trace. | covered | Replay preserves terminal state. |
| `TM-RESTRICT-001` | Unknown cell rejected without mutation. | `parse_place_segment`, validation diagnostic. | `validation_rejects_occupied_invalid_stale_wrong_actor_and_terminal_without_mutation`; diagnostic traces. | covered | Viewer-safe diagnostic. |
| `TM-RESTRICT-002` | Occupied cell rejected without mutation. | validation occupied check. | occupied rule test; `occupied-diagnostic.trace.json`. | covered | Guards API/dev paths. |
| `TM-RESTRICT-003` | Stale, malformed, wrong-seat, terminal submissions reject. | validation freshness/path/actor/terminal checks. | rule tests; stale trace; WASM stale diagnostic smoke. | covered | No mutation on rejection. |
| `TM-SCORE-001` | Terminal winner or draw only. | `TerminalOutcome`. | win/draw rule tests; traces; browser win/draw smoke. | covered | No score ledger. |
| `TM-SCORE-002` | Ply increments for each valid placement. | `apply_action` increments `ply_count`. | valid-action rule test; replay projection tests. | covered | Invalid submissions do not increment. |
| `TM-END-001` | Row/column/diagonal line wins immediately. | `winning_line`, terminal effects. | `row_column_and_diagonal_wins_report_ordered_line_cells`; terminal trace; browser win highlight. | covered | Win has precedence over draw. |
| `TM-END-002` | Full board with no line is draw. | draw branch in `apply_action`. | `full_board_without_line_is_draw`; draw trace; browser draw smoke. | covered | Draw wording is canonical. |
| `TM-END-003` | Terminal outcome remains unchanged. | terminal validation rejection. | terminal rule tests; terminal trace. | covered | Later normal actions rejected. |
| `TM-VIS-001` | Public view/effects/replay/dom expose only public data. | `project_view`, public effects, replay export, board DOM. | visibility tests; no-leak smoke; not-applicable trace. | covered | Perfect information; no hidden state to redact. |
| `TM-RNG-001` | Setup/rules use no randomness. | setup/rules ignore seed for rule behavior. | setup determinism; replay tests; bot tests isolate bot seed. | covered | Bot policy may use deterministic bot seed. |
| `TM-RNG-002` | Replay binds command/effect/view/action-tree/board/outcome hashes. | `replay_support::replay_commands`, golden traces, WASM export. | `golden_traces_match_expected_hashes`; `wasm-exported.trace.json`. | covered-by-trace | Three Marks replay-check support lands in GAT4THRMARBOA-014. |
| `TM-AMB-001` | Public name resolution. | docs/catalog/UI copy. | SOURCES/RULES docs; browser smoke. | covered | Neutral name avoids source confusion. |
| `TM-AMB-002` | `seat_0` first. | setup. | setup tests and replay traces. | covered | Deterministic. |
| `TM-AMB-003` | Stable cell ids. | `CellId`, action ids, public view. | property tests; browser smoke. | covered | Layout may change without changing ids. |
| `TM-AMB-004` | Line win beats simultaneous full-board draw. | terminal precedence in `apply_action`. | terminal/draw rule tests. | covered | Avoids ambiguous terminal state. |
| `TM-VAR-003` | Placement-only deviation from broader related variants. | docs and absence of movement rules. | RULES/SOURCES; no movement code. | covered | Gate 4 scope. |
| `TM-VAR-004` | Movement/sliding variants unsupported. | no setup/action path exposes movement. | docs; grep/code review. | unsupported | Out of scope until accepted movement spec. |
| `TM-VAR-005` | Misere/wild/larger/configurable/random/multiplayer variants unsupported. | only standard setup and static data. | variant parser tests; docs. | unsupported | Requires future accepted spec. |

## Visibility Surface Matrix

| Surface | Rule IDs | Evidence | Must not reveal | Status |
|---|---|---|---|---|
| public view | `TM-VIS-001` | `visibility_tests.rs`; WASM/browser smoke | hidden/private state | covered |
| action tree | `TM-ACTION-001`, `TM-ACTION-002`, `TM-VIS-001` | rule/property tests; browser legal-cell smoke | illegal targets | covered |
| effect log | `TM-TURN-002`, `TM-END-*`, `TM-VIS-001` | traces, effect tests, browser smoke | private state | covered |
| diagnostics | `TM-RESTRICT-*`, `TM-VIS-001` | diagnostic traces and tests | hidden/private data | covered |
| DOM/test IDs | `TM-COMP-002`, `TM-VIS-001` | `three-marks.smoke.mjs` | state dumps/candidate rankings | covered |
| bot explanations | `TM-RNG-001`, `TM-VIS-001` | bot tests; smoke | candidate rankings/hidden data | covered |
| private views | `TM-VIS-001` | explicit `not_applicable` rows | not applicable | not-applicable |

## Simulation And Benchmark Coverage

| Surface | Command/evidence | Rule IDs stressed | Status |
|---|---|---|---|
| simulation | `cargo run -p simulate -- --game three_marks --games 1000` | `TM-TURN-*`, `TM-ACTION-*`, `TM-END-*` | covered after GAT4THRMARBOA-014 |
| replay drift gate | `cargo run -p replay-check -- --game three_marks --all` | `TM-RNG-002`, trace hashes | covered after GAT4THRMARBOA-014 |
| fixture/schema gate | `cargo run -p fixture-check -- --game three_marks` | trace/static-data integrity | covered after GAT4THRMARBOA-014 |
| rule coverage gate | `cargo run -p rule-coverage -- --game three_marks` | all rule IDs | covered after GAT4THRMARBOA-014 |
| native benchmarks | `cargo bench -p three_marks` | legal/apply/view/replay/bot/playout hot paths | covered |
| Stage-2 perf budget | `BENCHMARKS.md`; `games/three_marks/benches/thresholds.json` | random playout throughput | covered | Initial run missed the visible 300,000 games/sec target; BENCHMARKS records the ADR-followup requirement. |
