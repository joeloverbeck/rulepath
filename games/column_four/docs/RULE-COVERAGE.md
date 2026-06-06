# Column Four Rule Coverage Matrix

Game ID: `column_four`

Rules version: `column_four-rules-v1`

Data/manifest version: `1`

Engine version: `0.1.0`

Prepared by: `Codex`

Last updated: 2026-06-06

## Purpose

This matrix maps every stable rule ID in `RULES.md` to implementation and evidence. Rust tests, golden traces, replay checks, fixture checks, simulations, and benchmarks are primary evidence; browser smoke proves integration only.

## Status labels

| Status | Meaning |
|---|---|
| covered | Implementation and required evidence exist. |
| covered-by-trace | Golden trace or replay evidence is the primary proof. |
| not-applicable | The surface truly does not apply, with rationale. |
| intentionally-deferred | Deferred by a documented gate decision. |
| unsupported | Explicitly not implemented for this variant. |

## Primary Evidence Files

| Evidence | Coverage role |
|---|---|
| `games/column_four/src/setup.rs` | deterministic setup and fixed variant |
| `games/column_four/src/actions.rs` | Rust legal column action tree |
| `games/column_four/src/rules.rs` | validation, gravity placement, turn change, terminal outcome |
| `games/column_four/src/effects.rs` | viewer-safe semantic effects |
| `games/column_four/src/visibility.rs` | Rust-projected public board view |
| `games/column_four/src/bots.rs` | Level 0 and Level 2 bot policies |
| `games/column_four/src/replay_support.rs` | replay hashes and board projections |
| `games/column_four/tests/rules.rs` | named rule behavior tests |
| `games/column_four/tests/property.rs` | random legal sequence invariant |
| `games/column_four/tests/replay.rs` | golden trace and replay hash checks |
| `games/column_four/tests/visibility.rs` | public-view and no-leak checks |
| `games/column_four/tests/bots.rs` | bot legality, determinism, explanations |
| `games/column_four/tests/golden_traces/*.trace.json` | win, draw, diagnostic, bot, terminal, and WASM-exported replay evidence |
| `apps/web/scripts/smoke-load-wasm.mjs` | WASM bridge smoke for catalog, actions, bot, diagnostics, replay |
| `games/column_four/benches/column_four.rs` | native hot-path benchmark evidence |

## Rule Coverage Matrix

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `CF-ACTION-001` | Non-terminal active seat may choose a non-full column. | `actions.rs`, `rules.rs`. | `cargo test -p column_four`; `shortest-normal-win.trace.json`; simulation. | covered | Rust owns legal column generation. |
| `CF-ACTION-002` | Full columns are unavailable. | `landing_cell`, `legal_columns`. | `full-column-diagnostic.trace.json`; rule tests. | covered | UI follows missing Rust target. |
| `CF-ACTION-003` | Terminal states have no placement actions. | `legal_action_tree`, validation terminal check. | `terminal-replay.trace.json`; terminal view tests. | covered | Replay preserves empty terminal tree. |
| `CF-AMB-001` | Public name is Column Four. | docs, manifest, WASM catalog. | `SOURCES.md`; `smoke-load-wasm.mjs`. | covered | Neutral naming. |
| `CF-AMB-002` | `seat_0` starts. | `setup_match`. | setup tests; traces. | covered | Deterministic. |
| `CF-AMB-003` | Stable bottom-origin cell IDs. | `CellId`, public view, action metadata. | visibility tests; replay projections. | covered | IDs bind traces and UI. |
| `CF-AMB-004` | Win beats simultaneous full-board draw. | terminal precedence in `apply_action`. | `final_placement_win_takes_precedence_over_draw`. | covered | Draw requires no line. |
| `CF-AMB-005` | Multiple-line primary highlight is deterministic. | `winning_line` sort/tiebreak. | `multiple_line_completion_uses_documented_tiebreak`. | covered | Replay hash stability. |
| `CF-COMP-001` | Public 7 by 6 board. | `ColumnFourState.cells`, `PublicView`. | visibility tests; WASM smoke. | covered | Board terms stay game-local. |
| `CF-COMP-002` | Seven public columns. | `ColumnId`, action choices. | action tests; legal-target tests. | covered | Columns are left-to-right. |
| `CF-COMP-003` | Six public rows. | `RowId`, `CellId`, view cells. | visibility tests; gravity tests. | covered | Rows are bottom-to-top. |
| `CF-COMP-004` | Stable cell IDs. | `CellId::ALL`, `CellId::as_string`. | replay tests; visibility tests. | covered | IDs appear in effects and traces. |
| `CF-COMP-005` | Public pieces. | `CellOccupancy`, `ui.rs`, public view. | visibility tests; effect tests. | covered | Tokens are viewer-safe. |
| `CF-COMP-006` | Public seats. | `ColumnFourSeat`, setup state. | setup tests; rule tests. | covered | Seat IDs drive actor ownership. |
| `CF-COMP-007` | Public winning line. | `WinningLine`, terminal view, effects. | win-direction traces; terminal tests. | covered | Rust exposes ordered four cells. |
| `CF-END-001` | Horizontal four wins. | `winning_line` horizontal direction. | `horizontal-win.trace.json`; rule tests. | covered-by-trace | Bottom-row win covered. |
| `CF-END-002` | Vertical four wins. | `winning_line` vertical direction. | `vertical-win.trace.json`; rule tests. | covered-by-trace | Gravity stack covered. |
| `CF-END-003` | Rising diagonal wins. | `winning_line` rising diagonal. | `diagonal-win.trace.json`; rule tests. | covered-by-trace | r1c1 to r4c4 covered. |
| `CF-END-004` | Falling diagonal wins. | `winning_line` falling diagonal. | `horizontal_vertical_and_diagonal_wins_are_reported_from_real_moves`. | covered | Direction logic is unit-tested. |
| `CF-END-005` | Full board without line draws. | draw branch in `apply_action`. | `draw.trace.json`; draw rule tests. | covered-by-trace | Trace fills all 42 cells. |
| `CF-END-006` | Multiple-line tiebreak is deterministic. | candidate sorting in `winning_line`. | tiebreak rule test. | covered | Primary line is stable. |
| `CF-END-007` | Terminal outcome remains unchanged. | terminal validation rejection. | terminal rule tests; `terminal-replay.trace.json`. | covered | Later applies reject. |
| `CF-GRAVITY-001` | Piece lands in lowest empty row. | `landing_cell`, `apply_action`. | gravity rule tests; all traces. | covered | TypeScript does not compute landing. |
| `CF-PLACE-001` | Valid action creates one piece. | `apply_action`, effects. | effect tests; replay projections. | covered | Invalid submissions create none. |
| `CF-RESTRICT-001` | Unknown column rejects without mutation. | `parse_drop_segment`, validation. | `invalid-column-diagnostic.trace.json`; rule tests. | covered-by-trace | Viewer-safe diagnostic. |
| `CF-RESTRICT-002` | Full column rejects without mutation. | `landing_cell` validation. | `full-column-diagnostic.trace.json`; rule tests. | covered-by-trace | Guards API paths. |
| `CF-RESTRICT-003` | Stale, malformed, wrong-seat, terminal reject. | validation freshness, actor, path, terminal checks. | `stale-diagnostic.trace.json`; rule tests; WASM smoke. | covered | Diagnostics are public prose. |
| `CF-RESTRICT-004` | TypeScript is not behavior authority. | Rust APIs consumed by WASM/web. | WASM smoke; boundary review; no TS legality changes. | covered | Bridge delegates to Rust. |
| `CF-RNG-001` | Setup/rules use no randomness. | setup/rules ignore rule RNG. | setup determinism; replay tests. | covered | Bot seed is outside rules. |
| `CF-RNG-002` | Replay binds commands, hashes, outcome, projection. | `replay_support`, golden traces. | `cargo run -p replay-check -- --game column_four --all`. | covered-by-trace | All trace hashes checked. |
| `CF-SCOPE-001` | Deterministic two-seat perfect-information game. | state, setup, rules, visibility. | package tests; traces; simulation. | covered | No hidden information. |
| `CF-SCOPE-002` | Public name is neutral Column Four. | docs, manifest, WASM catalog. | static data tests; WASM smoke. | covered | No commercial name. |
| `CF-SCOPE-003` | Rust owns setup, legality, effects, bots, replay, views. | game crate APIs, wasm-api delegation. | Rust tests; WASM smoke; tool checks. | covered | TS renders output only. |
| `CF-SCORE-001` | No score total beyond winner/draw. | `TerminalOutcome`. | terminal rule tests; traces. | covered | Result is terminal outcome only. |
| `CF-SCORE-002` | Ply increments only for valid placement. | `apply_action`, validation rejection. | rule tests; replay traces. | covered | Invalid diagnostics do not advance ply. |
| `CF-SETUP-001` | New match starts empty. | `setup_match`. | setup tests; replay reset; WASM smoke. | covered | 42 empty cells. |
| `CF-SETUP-002` | `seat_0` first. | `setup_match`. | setup tests; traces. | covered | Deterministic first actor. |
| `CF-SETUP-003` | All public information visible from setup. | `project_view`, private status. | visibility tests; WASM smoke. | covered | Perfect-information status explicit. |
| `CF-TURN-001` | Active seat submits one column action with token. | `legal_action_tree`, `validate_command`. | rule tests; WASM smoke. | covered | Freshness token required. |
| `CF-TURN-002` | Non-terminal placement passes turn. | `apply_action` active-seat flip. | effect tests; replay traces. | covered | Emits active-player change. |
| `CF-TURN-003` | Terminal placement stops normal turns. | terminal branch in `apply_action`. | terminal tests; traces. | covered | No next active turn. |
| `CF-VAR-001` | Only `column_four_standard` ships. | `variants.rs`, static data. | variant tests; fixture-check. | covered | No variant picker. |
| `CF-VAR-002` | Standard 7 by 6 two-seat four-in-row draw variant. | `variants.rs`, setup, rules. | setup/rule/replay tests. | covered | Gate 5 variant. |
| `CF-VAR-003` | Neutral 7 by 6 implementation differs from broader related variants. | docs and absence of variant actions. | `SOURCES.md`; static data tests. | covered | Scope is documented. |
| `CF-VAR-004` | PopOut/removal variants unsupported. | no action path exposes removal. | docs; code review. | unsupported | Out of scope until accepted spec. |
| `CF-VAR-005` | Misere, custom size, random, multiplayer variants unsupported. | only standard setup and static data. | variant parser tests; docs. | unsupported | Requires future accepted spec. |
| `CF-VIS-001` | Public surfaces expose only public data. | `project_view`, public effects, replay export, WASM JSON. | visibility tests; fixture-check; WASM smoke. | covered | No hidden state exists. |
| `CF-VIS-002` | Private view status is not applicable. | `PrivateView` in public projection. | visibility tests; WASM smoke. | covered | `hidden_fields` is empty. |

## Golden Trace Catalog

| Trace file | Purpose | Rule IDs covered | Expected result/hash evidence | Diagnostic coverage |
|---|---|---|---|---|
| `shortest-normal-win.trace.json` | shortest vertical win | `CF-TURN-001`, `CF-END-002`, `CF-RNG-002` | terminal seat_0 win hashes | none |
| `vertical-win.trace.json` | center vertical win | `CF-END-002`, `CF-GRAVITY-001` | terminal seat_0 win hashes | none |
| `horizontal-win.trace.json` | bottom horizontal win | `CF-END-001` | terminal seat_0 win hashes | none |
| `diagonal-win.trace.json` | rising diagonal win | `CF-END-003` | terminal seat_0 win hashes | none |
| `draw.trace.json` | full-board draw | `CF-END-005`, `CF-RNG-002` | 42 occupied cells, draw hashes | none |
| `stale-diagnostic.trace.json` | stale action rejection | `CF-RESTRICT-003` | diagnostic hash | `stale_action` |
| `invalid-column-diagnostic.trace.json` | unknown column rejection | `CF-RESTRICT-001` | diagnostic hash | `unknown_column` |
| `full-column-diagnostic.trace.json` | full column rejection | `CF-RESTRICT-002` | diagnostic hash | `full_column` |
| `bot-action.trace.json` | bot-selected center drop | `CF-RNG-001`, `CF-ACTION-001` | replay hashes for Rust-selected command | none |
| `terminal-replay.trace.json` | terminal action tree replay | `CF-ACTION-003`, `CF-END-007` | terminal hashes | none |
| `wasm-exported.trace.json` | WASM-compatible command export shape | `CF-RNG-002`, `CF-RESTRICT-004` | replay hashes | none |

## Simulation And Benchmark Coverage

| Surface | Command/evidence | Rule IDs stressed | Status |
|---|---|---|---|
| simulation | `cargo run -p simulate -- --game column_four --games 1000` | `CF-TURN-*`, `CF-ACTION-*`, `CF-END-*` | covered |
| replay drift gate | `cargo run -p replay-check -- --game column_four --all` | `CF-RNG-002`, trace hashes | covered |
| fixture/schema gate | `cargo run -p fixture-check -- --game column_four` | trace/static-data integrity | covered |
| rule coverage gate | `cargo run -p rule-coverage -- --game column_four` | all rule IDs | covered |
| native benchmarks | `cargo bench -p column_four` | legal/apply/view/replay/bot/playout hot paths | covered |
| random-playout target | `BENCHMARKS.md`; `games/column_four/benches/thresholds.json` | playout throughput | covered | First local run missed provisional 100,000 games/sec target and records a measured floor. |
