# Draughts Lite Mechanics Inventory

Game ID: `draughts_lite`

Roadmap stage/gate: Gate 7 compound-action official game

Rules version: `draughts_lite-rules-v1`

Last updated: 2026-06-07

## Purpose

This inventory records Draughts Lite's game-local mechanic shapes and primitive-pressure posture. It is evidence for [../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md) and the local coordinate-helper review in [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md).

Draughts Lite is a deterministic, perfect-information two-seat board game that proves compound action trees: a single replay command can contain an origin plus one or more landing segments. Rust owns all legality, continuation, promotion, terminal, effect, replay, and bot behavior.

## Mechanic Inventory

| Category | Game-local description | Evidence | Current status | Notes |
|---|---|---|---|---|
| topology/spatial model | Fixed public 8 by 8 board, cells `r1c1` through `r8c8`, row 1 at the top, playable dark squares where `row + column` is odd. | [RULES.md](RULES.md), `ids.rs`, setup/rule tests | `local with helper reuse` | Uses behavior-free `game-stdlib::board_space`; dark-square policy stays game-local. |
| component/zone model | Sixty-four public cells, two seats, public pieces, no hands/decks/private zones. | `state.rs`, `visibility.rs`, [RULE-COVERAGE.md](RULE-COVERAGE.md) | `local-only` | Perfect-information board only. |
| action shape | Compound Rust action paths: `from/rNcM` then `to/rNcM` for quiet moves or one or more `jump/rNcM` segments for captures. | `actions.rs`, replay traces | `local-only` | Browser stores partial UI path only until a Rust leaf is submitted. |
| tree phases | `origin`, `quiet_landing`, `jump_landing`, `forced_continuation_landing`. | action metadata tests | `local-only` | Phase metadata drives UI cues; it is not TS legality. |
| turn/phase model | Alternating active seat; one complete path per command; forced continuation stays in the same command until Rust exposes a leaf. | `rules.rs`, multi-jump traces | `local-only` | No reaction windows or simultaneous choices. |
| randomness/chance | Rules and setup use no randomness; bot seed is outside game-rule state. | [RULES.md](RULES.md), replay tests | `local-only` | Replays record resolved commands. |
| visibility/hidden information | Perfect information; public view is complete, private view not applicable, hidden fields empty. | `visibility.rs`, WASM/browser no-leak smokes | `local-only` | Bot rationale and replay exports remain public-safe. |
| movement/capture/placement | Men move forward one diagonal; kings move one diagonal any direction; captures jump adjacent opposing pieces to an empty landing. | `rules.rs`, `DL-MOVE-*`, `DL-COMP-*` | `local-only` | No flying kings or backward captures by men. |
| mandatory action | If any capture exists, quiet moves are unavailable. If a capture step has Rust continuation children, the submitted command must continue. | `DL-RESTRICT-*`, diagnostic traces | `local-only` | No maximum-capture mandate. |
| promotion | Men promote to crown on the opponent king row; promotion during capture ends the path immediately. | promotion traces/tests | `local-only` | This is a replay-visible edge case. |
| scoring/outcome | Win when the opponent has no pieces or no legal move. | `DL-END-*`, terminal traces | `local-only` | Draw claims, clocks, repetition, and adjudication are out of scope. |
| semantic effects | Move committed, quiet step, capture step, promotion, forced capture, forced continuation, invalid command, terminal win, bot choice. | `effects.rs`, WASM serializer, browser smoke | `local-only` | Effects drive logs, static highlights, replay, and reduced-motion feedback. |
| UI interaction pattern | 8 by 8 grid, roving focus, Rust legal origins/destinations, pending path, cancel, replay projection, reduced motion. | [UI.md](UI.md), `DraughtsLiteBoard.tsx`, `draughts-lite.smoke.mjs` | `local-only` | TypeScript presents Rust/WASM output only. |
| bot policy pattern | Level 0 random legal and Level 1 authored public-feature policy. | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) | `local-only` | No MCTS, Monte Carlo, ML, RL, or runtime LLM move selection. |
| benchmark/performance pressure | Legal tree generation for opening, midgame, mandatory capture, and multi-jump positions. | [BENCHMARKS.md](BENCHMARKS.md), thresholds | `local-only` | Thresholds are smoke floors pending stable CI baselines. |

## Action Path And Metadata Contract

| Segment or metadata | Meaning | Evidence |
|---|---|---|
| `from/rNcM` | Select the moving piece origin. | `actions.rs`; `shortest-quiet.trace.json` |
| `to/rNcM` | Quiet landing. | `actions.rs`; quiet traces |
| `jump/rNcM` | Capture landing; captured cell/piece metadata is Rust-provided. | capture and multi-jump traces |
| `cell_id` | Cell controlled by this tree choice. | board UI smoke |
| `capture_mandatory` | Rust found at least one capture in the current position. | mandatory-capture tests/traces |
| `is_capture` | This legal move/step is a capture. | action metadata tests |
| `forced_by_continuation` | This landing is a later capture step in the same command. | multi-jump tests/traces |
| `would_promote` | This choice reaches crown status. | promotion tests/traces |
| `preview_origin`, `preview_landing`, `preview_captured_cell` | Viewer-safe UI cue cells. | `DraughtsLiteBoard.tsx` smoke |

## Diagnostics

Shipped validation diagnostics include `terminal_match`, `stale_action`, `unknown_actor`, `not_active_seat`, `empty_action_path`, `mandatory_continuation_incomplete`, `continues_after_promotion_stop`, `quiet_move_while_capture_available`, `action_path_not_available`, `malformed_segment`, `origin_not_playable`, `destination_not_playable`, `origin_outside_board`, and `destination_outside_board`.

Diagnostics are viewer-safe and do not mutate state. Golden traces cover stale, non-active, non-playable, occupied destination, quiet-while-capture, incomplete continuation, and post-promotion continuation cases.

## Extraction Or Defer Rationale

| Shape | Decision | Rationale | Trace impact |
|---|---|---|---|
| rectangular coordinate helper | reuse behavior-free `game-stdlib::board_space` | Gate 7 pressure justified coordinate parsing/iteration without moving draughts policy into shared code; Gate 7.1 back-ported the same subset to earlier matching board games. | no trace migration |
| movement/capture/promotion rules | local | They are core Draughts Lite legality. | protected by local traces |
| action-tree metadata phases | local | Compound path semantics are game-specific. | protected by local traces/WASM smoke |
| board UI grid | local | UI has unique pending-path and continuation behavior. | browser smoke only |
| Level 1 bot policy | local | Strategy and rationale are game-specific. | bot trace/effects stay local |

## Review Checklist

- `engine-core` remains generic and noun-free.
- TypeScript renders Rust choices, metadata, effects, and views only.
- `game-stdlib` reuse is limited to behavior-free board-space helpers.
- Replay command paths preserve ordered multi-segment lists.
- No draw/adjudication rule is invented by tools, UI, or docs.
