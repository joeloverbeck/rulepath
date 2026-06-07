# Draughts Lite Rule Coverage Matrix

Game ID: `draughts_lite`

Rules version: `draughts_lite-rules-v1`

Data/manifest version: `1`

Engine version: `0.1.0`

Prepared by: `Codex`

Last updated: 2026-06-07

## Purpose

This matrix maps every stable rule ID in `RULES.md` to implementation and evidence. Rust tests, golden traces, replay checks, fixture checks, simulations, and benchmarks are primary evidence; browser smoke proves integration only.

## Rule Coverage Matrix

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `DL-ACTION-001` | Quiet moves appear only when no capture exists. | `actions.rs`, `rules.rs`. | `shortest-quiet.trace.json`; rule tests. | covered-by-trace | Rust tree owns origin and landing choices. |
| `DL-ACTION-002` | Captures suppress quiet moves and expose complete capture paths. | `actions.rs`, `rules.rs`. | `mandatory-capture-suppresses-quiet.trace.json`; `single-capture.trace.json`; `multi-jump.trace.json`. | covered-by-trace | No TS legality. |
| `DL-ACTION-003` | Promotion during capture stops continuation. | `rules.rs`, `effects.rs`. | `promotion-during-capture-stop.trace.json`; `path-after-promotion-stop-diagnostic.trace.json`. | covered-by-trace | Newly crowned piece waits until a later turn. |
| `DL-ACTION-004` | Kings move and jump one diagonal square. | `rules.rs`. | `dl_men_kings_capture_and_mandatory_capture_rules_hold`; unit tests. | covered | No flying kings. |
| `DL-ACTION-005` | Terminal action tree has no choices. | `actions.rs`, `rules.rs`. | terminal tests; terminal traces. | covered | Terminal commands reject. |
| `DL-AMB-001` | Public name is Draughts Lite. | manifest, docs, WASM catalog. | `SOURCES.md`; fixture-check; WASM smoke. | covered | Neutral project naming. |
| `DL-AMB-002` | Coordinates and playable parity are fixed. | `ids.rs`, `game-stdlib::board_space`. | setup/id tests; fixture-check; traces. | covered | `row + column` odd is playable. |
| `DL-AMB-003` | Promotion during capture stops the move. | `rules.rs`. | promotion-stop tests and traces. | covered-by-trace | Matches scoped English draughts subset. |
| `DL-AMB-004` | No maximum-capture mandate. | `rules.rs`, legal tree generation. | `dl_multi_jump_same_piece_no_double_capture_and_no_maximum_capture_rule`; branch trace. | covered | Mandatory capture exists; maximum capture does not. |
| `DL-BOT-001` | Level 0 random legal bot validates through Rust. | `DraughtsLiteRandomBot`. | bot tests; simulation. | covered | Chooses complete legal paths only. |
| `DL-BOT-002` | Level 1 authored policy is deterministic and public-safe. | `DraughtsLiteLevel1Bot`. | bot tests; `bot-action.trace.json`; strategy evidence pack. | covered-by-trace | No search, learning, or hidden state. |
| `DL-BOT-003` | Search/ML/LLM bots are forbidden. | bot module scope, docs. | bot strategy evidence pack; no search dependencies. | unsupported | Explicitly out of public v1/v2 scope. |
| `DL-COMP-001` | Board is a public 8 by 8 play area. | `ids.rs`, `visibility.rs`. | fixture-check; setup tests. | covered | Board nouns stay game-local. |
| `DL-COMP-002` | Cells use stable public IDs. | `game-stdlib::Coord`, `ui.rs`. | id/ui tests; traces. | covered | `rNcM` IDs are stable. |
| `DL-COMP-003` | Pieces are public owned tokens. | `state.rs`, `visibility.rs`. | visibility tests; fixture-check. | covered | No hidden pieces. |
| `DL-COMP-004` | Men move and jump forward only. | `rules.rs`. | `men_move_and_capture_forward_only`; rule tests. | covered | Seat-relative forward direction. |
| `DL-COMP-005` | Kings move and jump one diagonal any direction. | `rules.rs`. | king rule tests. | covered | No long-range capture. |
| `DL-COMP-006` | Jump captures one adjacent opposing piece. | `rules.rs`, `effects.rs`. | capture traces; effect tests. | covered-by-trace | Captured pieces are removed in path order. |
| `DL-COMP-007` | Action paths are origin plus landing segments. | `actions.rs`, replay support, WASM bridge. | replay tests; WASM smoke; `wasm-exported.trace.json`. | covered-by-trace | Multi-segment list is preserved. |
| `DL-END-001` | Opponent with no pieces loses. | terminal logic in `rules.rs`. | `terminal-no-pieces.trace.json`; terminal tests. | covered-by-trace | Winner is public. |
| `DL-END-002` | Opponent with no legal move loses. | terminal logic in `rules.rs`. | `terminal-no-legal-moves.trace.json`; terminal tests. | covered-by-trace | No stalemate draw. |
| `DL-END-003` | Tournament draw/adjudication is omitted. | rules scope and simulator taxonomy. | simulation bounded-nonterminal bucket; docs. | not-applicable | Gate 7 does not implement draw claims, clocks, or repetition. |
| `DL-MOVE-001` | Quiet move transfers a piece without capture. | `rules.rs`, `effects.rs`. | `shortest-quiet.trace.json`; quiet move tests. | covered-by-trace | Invalid submissions do not mutate. |
| `DL-MOVE-002` | Jump removes exactly one opposing piece per step. | `rules.rs`, `effects.rs`. | `single-capture.trace.json`; `multi-jump.trace.json`. | covered-by-trace | Path order is deterministic. |
| `DL-MOVE-003` | Men promote on the opponent king row. | `rules.rs`, `effects.rs`. | `promotion-quiet.trace.json`; promotion capture trace. | covered-by-trace | Capture promotion stops the sequence. |
| `DL-MOVE-004` | Completed moves check terminal then advance turn. | `apply_action`, terminal helpers. | rule tests; terminal traces; simulation. | covered | Rust owns outcome and active seat. |
| `DL-REPLAY-001` | Complete moves replay as multi-segment commands. | `replay_support.rs`, WASM export. | replay tests; replay-check; `wasm-exported.trace.json`. | covered-by-trace | Trace Schema v1 list shape is retained. |
| `DL-REPLAY-002` | Partial selections are not replay commands. | action tree and validation. | partial continuation diagnostics; replay tests. | covered | Only complete paths are applied. |
| `DL-RESTRICT-001` | Capture is mandatory when available. | `legal_moves_for`, validation. | mandatory-capture and quiet-while-capture traces. | covered-by-trace | Quiet commands reject. |
| `DL-RESTRICT-002` | Same-piece capture continuation is mandatory. | continuation search in `rules.rs`. | `multi-jump.trace.json`; `illegal-continuation-diagnostic.trace.json`. | covered-by-trace | Early ending rejects. |
| `DL-RESTRICT-003` | Invalid/stale/wrong-actor paths reject without mutation. | validation diagnostics. | diagnostic traces; rule tests; WASM stale smoke. | covered-by-trace | Viewer-safe diagnostics only. |
| `DL-RESTRICT-004` | Different capture lengths are all legal if Rust supplies them. | legal tree generation. | forced-continuation branch trace; no-maximum-capture test. | covered | No maximum-capture rule. |
| `DL-RNG-001` | Setup and rules contain no randomness. | `setup.rs`, `rules.rs`; bot seed isolated. | setup determinism tests; replay hashes. | covered | Bot randomness is recorded as resolved commands. |
| `DL-SCOPE-001` | Two-seat deterministic perfect-information scoped draughts subset. | crate modules and docs. | full crate tests; fixture-check; WASM smoke. | covered | Gate proves compound action trees. |
| `DL-SCOPE-002` | A complete move is one Rust-validated replay command. | actions, validation, replay support. | multi-jump trace; WASM-exported trace. | covered-by-trace | UI partial path is not a command. |
| `DL-SCOPE-003` | Tournament extras are out of scope. | rules/docs. | omission rows; simulation taxonomy. | not-applicable | No fake draw rule is added. |
| `DL-SETUP-001` | New match has exactly two seats. | `setup.rs`. | setup tests; standard fixture. | covered | `seat_0`, `seat_1`. |
| `DL-SETUP-002` | Each seat starts with 12 men. | `setup.rs`. | setup tests; standard fixture. | covered | Rows 1-3 and 6-8. |
| `DL-SETUP-003` | `seat_0` starts active. | `setup.rs`. | setup tests; fixture. | covered | Deterministic first player. |
| `DL-SETUP-004` | Setup state is public. | `visibility.rs`. | visibility tests; fixture-check. | covered | No hidden setup data. |
| `DL-TURN-001` | Active seat submits one complete path. | validation and action tree. | rule tests; replay tests. | covered | Non-active actor rejects. |
| `DL-TURN-002` | Quiet turn applies and advances if nonterminal. | `apply_action`. | quiet trace; rule tests. | covered-by-trace | Promotion handled if applicable. |
| `DL-TURN-003` | Capture turn applies atomically. | `apply_action`, capture validation. | single and multi-jump traces. | covered-by-trace | Complete sequence only. |
| `DL-TURN-004` | Forced continuation stays with same piece. | `actions.rs`, `rules.rs`. | forced-continuation branch and illegal-continuation traces. | covered-by-trace | Ends at no jump or promotion. |
| `DL-UI-001` | Browser consumes Rust legal tree/effects/outcomes only. | WASM bridge and web smoke. | `smoke-load-wasm.mjs`; WASM API tests. | covered | TypeScript forwards paths. |
| `DL-UI-002` | Full pointer/keyboard/a11y UI evidence lands later. | planned renderer tickets. | GAT7DRALITCOM-018/019. | intentionally-deferred | Board renderer and E2E/a11y smoke are separate tickets. |
| `DL-VAR-001` | Only standard variant ships. | `variants.rs`, static data. | static-data tests; fixture-check. | covered | `draughts_lite_standard`. |
| `DL-VAR-002` | Stable cells and dark-square parity. | `ids.rs`, `game-stdlib`. | id tests; fixture; traces. | covered | Public coordinate convention. |
| `DL-VAR-003` | Men forward, kings all diagonals. | `rules.rs`. | movement tests. | covered | English draughts subset. |
| `DL-VAR-004` | Larger/flying/backward variants excluded. | docs and tests absence. | source docs; variant table. | unsupported | Later spec required for variants. |
| `DL-VAR-005` | Other draughts/checkers variants excluded. | docs and variant catalog. | source docs. | unsupported | Gate 7 does not implement them. |
| `DL-VAR-006` | Tournament systems excluded. | docs and simulation taxonomy. | source docs; bounded-nonterminal simulation. | unsupported | Later spec/ADR required. |
| `DL-VIS-001` | Public view/effects/replay expose no hidden state. | `visibility.rs`, effects, WASM JSON. | visibility tests; fixture-check; WASM smoke. | covered | Perfect information; internals remain out. |
| `DL-VIS-002` | Bot choices/explanations are public-safe. | `bots.rs`, effects. | bot tests; bot trace; strategy pack. | covered-by-trace | No candidate rankings or hidden data. |

## Golden Trace Catalog

| Trace file | Purpose | Rule IDs covered | Diagnostic coverage |
|---|---|---|---|
| `shortest-quiet.trace.json` | quiet opening move | `DL-ACTION-001`, `DL-MOVE-001`, `DL-REPLAY-001` | none |
| `mandatory-capture-suppresses-quiet.trace.json` | quiet rejected when capture exists | `DL-RESTRICT-001` | `quiet_move_while_capture_available` |
| `single-capture.trace.json` | one capture applies | `DL-ACTION-002`, `DL-MOVE-002` | none |
| `multi-jump.trace.json` | forced multi-jump applies | `DL-TURN-003`, `DL-TURN-004`, `DL-REPLAY-001` | none |
| `forced-continuation-branch.trace.json` | branch choice in continuation | `DL-RESTRICT-004` | none |
| `promotion-quiet.trace.json` | quiet promotion | `DL-MOVE-003` | none |
| `promotion-during-capture-stop.trace.json` | capture promotion stop | `DL-ACTION-003`, `DL-AMB-003` | none |
| `terminal-no-pieces.trace.json` | win by no opposing pieces | `DL-END-001` | none |
| `terminal-no-legal-moves.trace.json` | win by no legal move | `DL-END-002` | none |
| `stale-diagnostic.trace.json` | stale rejection | `DL-RESTRICT-003` | `stale_action` |
| `non-active-seat-diagnostic.trace.json` | wrong actor rejection | `DL-TURN-001`, `DL-RESTRICT-003` | `not_active_seat` |
| `non-playable-cell-diagnostic.trace.json` | light-square rejection | `DL-VAR-002`, `DL-RESTRICT-003` | `non_playable_cell` |
| `occupied-destination-diagnostic.trace.json` | occupied landing rejection | `DL-RESTRICT-003` | `action_path_not_available` |
| `quiet-while-capture-diagnostic.trace.json` | mandatory capture diagnostic | `DL-RESTRICT-001` | `quiet_move_while_capture_available` |
| `illegal-continuation-diagnostic.trace.json` | incomplete continuation | `DL-RESTRICT-002` | `mandatory_continuation_incomplete` |
| `path-after-promotion-stop-diagnostic.trace.json` | illegal post-promotion continuation | `DL-ACTION-003` | `continues_after_promotion_stop` |
| `bot-action.trace.json` | Level 1 bot command | `DL-BOT-002`, `DL-VIS-002` | none |
| `wasm-exported.trace.json` | WASM multi-segment replay export | `DL-UI-001`, `DL-REPLAY-001` | none |

## Tool Coverage

| Surface | Command/evidence | Rule IDs stressed | Status |
|---|---|---|---|
| simulation | `cargo run -p simulate -- --game draughts_lite --games 1000` | turn, legality, bot, terminal, bounded nonterminal | covered |
| replay drift gate | `cargo run -p replay-check -- --game draughts_lite --all` | trace hashes and replay determinism | covered |
| fixture/schema gate | `cargo run -p fixture-check -- --game draughts_lite` | static-data and trace integrity | covered |
| rule coverage gate | `cargo run -p rule-coverage -- --game draughts_lite` | all rule IDs | covered |
| native benchmarks | `cargo bench -p draughts_lite` | legal/apply/view/replay/bot hot paths | covered |
| benchmark report | `cargo run -p bench-report -- --game draughts_lite --input <report>` | threshold mapping | covered |
