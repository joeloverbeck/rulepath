# Column Four Mechanics Inventory

Game ID: `column_four`

Roadmap stage/gate: Gate 5 public showcase

Rules version: `column_four-rules-v1`

Last updated: 2026-06-07

## Purpose

This inventory records Column Four's local mechanic shapes and primitive-pressure posture. It is evidence for [../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md); it is not permission to generalize.

Column Four is the second official public board game after `three_marks`, so the repeated fixed-grid, occupancy, and line-detection shapes are recorded honestly. Gate 7.1 back-ported the promoted `game-stdlib::board_space` coordinate/cell identity primitive; no extraction to `engine-core` occurs.

## Mechanic Inventory

| Category | Game-local description | Evidence in rules | Current status | Notes |
|---|---|---|---|---|
| topology/spatial model | Fixed 7 by 6 public grid with cells `r1c1` through `r6c7`, rows counted bottom to top. | `game-stdlib::board_space`; `CF-COMP-001` through `CF-COMP-005`, `CF-AMB-003` in [RULES.md](RULES.md) | `promoted-primitive-conformant` | Coordinate/cell identity uses `board_space`; columns, gravity, and line scans are game-local. |
| component/zone model | Forty-two public cells, seven public columns, two seats, no hands/decks/zones. | `CF-SETUP-001` through `CF-SETUP-003` | `local-only` | Perfect-information board only. |
| action shape | Flat Rust action choices `drop/c1` through `drop/c7`; UI exposes columns, not cells. | `CF-ACTION-001`, `CF-RESTRICT-001` through `CF-RESTRICT-004` | `local-only` | TypeScript maps Rust legal targets to seven controls. |
| turn/phase model | Alternating one-drop turns until win or draw. | `CF-TURN-001` through `CF-TURN-003` | `local-only` | No phases, reactions, forced windows, or simultaneous choices. |
| randomness/chance | Rules use no randomness; bot seed is outside rule state. | `CF-RNG-001`, `CF-RNG-002` | `local-only` | Replay commands fully reproduce rule state. |
| visibility/hidden information | Perfect information; private view is explicitly not applicable. | `CF-VIS-001` | `local-only` | No hidden state, but browser no-leak checks still run. |
| resource/accounting | No resource total or score beyond terminal winner/draw. | `CF-SCORE-001` | `local-only` | Outcome is winner or draw only. |
| movement/capture/placement | Gravity placement into the lowest empty cell of a selected column. | `CF-GRAVITY-001`, `CF-GRAVITY-002` | `local-only` | This differs materially from Three Marks fixed-cell placement. |
| pattern/line/directional scanning | Four contiguous owned cells in horizontal, vertical, rising diagonal, or falling diagonal directions. | `CF-END-001` through `CF-END-006` | `repeated-shape candidate` | Similar terminal pattern shape to Three Marks, but line length and gravity support differ. |
| commitment/reveal | Not applicable; all choices are public and immediate. | `CF-VIS-001` | `local-only` | No secret commitments. |
| reaction/window/pending response | Not applicable; one actor acts at a time. | `CF-TURN-001` | `local-only` | No response windows. |
| scoring/outcome | Immediate win by line; full-board draw only with no line. | `CF-END-001` through `CF-END-007`, `CF-SCORE-001` | `local-only` | Win precedence over draw is explicit. |
| semantic effect shape | Drop accepted, piece landed, active player changed, win/draw, game ended, bot chose action. | `CF-TURN-002`, `CF-END-*`, `CF-VIS-001` | `local-only` | Effects drive logs, animation, replay, and bot rationale. |
| UI interaction pattern | Seven column controls, Rust landing preview, SVG board, replay projection, reduced-motion support. | `CF-ACTION-001`, `CF-GRAVITY-001`, `CF-VIS-001` | `local-only` | Implemented in [../../../apps/web/src/components/ColumnFourBoard.tsx](../../../apps/web/src/components/ColumnFourBoard.tsx). |
| bot policy pattern | Level 0 random legal and Level 2 authored tactical policy. | `CF-BOT-*`, [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | `local-only` | No MCTS, search-heavy policy, ML, or RL. |
| benchmark/performance pressure | Legal tree, apply, public view, replay, serialization, random playout, Level 0/2 bot decisions. | [BENCHMARKS.md](BENCHMARKS.md) | `local-only` | Native benchmark floor is recorded; random playout provisional target miss remains visible. |

## Repeated-Shape Comparison

| Mechanic shape | Already appears in | Same shape? | Similarities | Differences | Required next step |
|---|---|---:|---|---|---|
| fixed public grid occupancy | `three_marks` | yes, as a broad shape | public cells, two seats, occupied/empty view, replayed board projection | 7 by 6 instead of 3 by 3; columns and gravity change legal targeting | record atlas pressure in GAT5COLFOUPUB-018; defer extraction |
| line/pattern terminal detection | `three_marks` | similar, not identical | owned contiguous public cells produce terminal win and winning-line UI highlight | length four, directional scan over variable board, gravity support, deterministic primary-line tie-break | record repeated-shape candidate; defer extraction |
| flat legal target controls | `race_to_n`, `three_marks` | similar UI/API shape | Rust action tree produces public choices; UI dispatches action id | Column Four groups choices as columns and uses landing previews | keep local; generic action tree already belongs to `engine-core` |
| perfect-information no-leak posture | `race_to_n`, `three_marks` | yes | private view not applicable; replay/browser no-leak still checked | Column Four has richer DOM/SVG board and bot rationale | keep no-leak checklist updated |

## Second-Use Note

| Shape | First game | Second game | Candidate? | Rationale | Ledger/atlas update needed? |
|---|---|---|---:|---|---:|
| fixed public grid occupancy | `three_marks` | `column_four` | yes | Second implemented board with public cell occupancy. | yes |
| terminal line/pattern detection | `three_marks` | `column_four` | yes | Second implemented line-win game, but geometry differs enough to keep local. | yes |
| board-first React renderer | `three_marks` | `column_four` | yes | Same UI posture: Rust public view plus legal controls. | yes |

## Third-Use Hard-Gate Warning

| Shape | Games exerting pressure | Third-use? | Gate cleared? | Evidence |
|---|---|---:|---:|---|
| fixed public grid occupancy | `three_marks`, `column_four` | no | not applicable | This file; atlas update in GAT5COLFOUPUB-018. |
| terminal line/pattern detection | `three_marks`, `column_four` | no | not applicable | This file; atlas update in GAT5COLFOUPUB-018. |

## Primitives Reused

| Primitive | Source | Why reused | Rule IDs covered | Tests proving compatibility | Notes |
|---|---|---|---|---|---|
| action tree, command envelope, actor/viewer, seed, stable serialization | `engine-core` | Generic contracts already own action dispatch, replay, actor/viewer identity, and deterministic serialization. | `CF-ACTION-*`, `CF-RNG-*`, `CF-VIS-*` | `cargo test -p column_four`, replay golden traces | No game noun enters `engine-core`. |
| benchmark report tool | tools | Existing report/threshold lane handles native operation floors. | benchmark evidence | `cargo bench -p column_four -- legal_actions`; Gate 2 workflow | No new benchmark DSL. |

## Local Mechanics

| Local mechanic | Why local | Extraction risk | Rule IDs | Tests/traces | Notes |
|---|---|---|---|---|---|
| column/drop gravity | Specific to this vertical-drop game. | medium | `CF-GRAVITY-*`, `CF-ACTION-*` | rule tests; `vertical-win.trace.json`; `full-column-diagnostic.trace.json` | Do not promote until another game needs the same exact gravity model. |
| four-direction line scan | Similar to Three Marks but not identical. | medium | `CF-END-001` through `CF-END-006` | win-direction traces; rule tests | Defer extraction; third-use review required. |
| Level 2 tactical priorities | Game-specific strategy and public rationale. | low | bot docs/evidence | bot tests; `BOT-STRATEGY-EVIDENCE-PACK.md` | No shared search primitive. |
| SVG column board presentation | Product-facing renderer for this game. | low | UI rules | `column-four.smoke.mjs` | TypeScript stays presentation-only. |

## Extraction Or Defer Rationale

| Shape | Decision | Rationale | Back-port needed? | Trace impact | Benchmark impact |
|---|---|---|---:|---|---|
| fixed public grid occupancy | partial promoted-primitive conformance | Coordinate/cell identity conforms to `game-stdlib::board_space`; occupancy, targeting, and gravity remain local because they differ materially from other games. | complete for coordinate identity | none | none |
| line/pattern detection | defer | Similar terminal shape exists in Three Marks, but Column Four needs length-four directional scans and primary-line tie-breaks. | no | none | none |
| column gravity | local | First official use of gravity placement. | no | none | Column Four benchmarks cover local hot paths. |
| bot tactical policy | local | Strategy is game-specific and documented in the evidence pack. | no | none | bot-decision benches stay game-local. |

## Effects, UI, And Bot Notes

| Area | Requirement | Rule IDs | Notes |
|---|---|---|---|
| semantic effects | Emit accepted drop, landed piece, turn change, terminal, and bot rationale effects. | `CF-TURN-*`, `CF-END-*`, `CF-VIS-001` | WASM serializes viewer-safe effects for logs, replay, and UI animation. |
| UI interaction pattern | Seven column controls, not 42 cell controls. | `CF-ACTION-001`, `CF-RESTRICT-004` | Prevents TypeScript from becoming a legality authority. |
| Rust-generated previews | Landing preview comes from Rust public view/legal targets. | `CF-GRAVITY-001`, `CF-VIS-001` | Hover/focus preview is display-only. |
| bot policy pattern | Level 0 random legal; Level 2 authored public tactical policy. | bot docs/evidence | Explanations are public prose only; no rankings or scores. |
| visibility/no-leak | Perfect-information game with explicit no-leak smoke. | `CF-VIS-001` | Browser smoke checks DOM, attrs, storage, console, and replay export. |
| benchmark pressure | Legal/action/view/replay/serialization/playout/bot operations. | benchmark docs | `random_playout` target miss remains documented, not hidden. |

## Required Repo Atlas Update

| Update target | Required? | Reason | Owner |
|---|---:|---|---|
| `docs/MECHANIC-ATLAS.md` | yes | Record second-use fixed-grid/line-scan pressure and no-extraction decision. | GAT5COLFOUPUB-018 |
| `PRIMITIVE-PRESSURE-LEDGER.md` | no | Repo currently uses atlas-level pressure record for this gate; no separate ledger file exists for Column Four. | none |
| ADR | no | No shared primitive, boundary change, DSL, YAML, or engine-core noun is introduced. | none |

## Review Checklist

- Grid, column, cell, gravity, and line vocabulary remains game-local.
- Coordinate/cell identity conforms to `game-stdlib::board_space`.
- Rust owns legality, validation, effects, bot decisions, replay, and public views.
- TypeScript presents Rust/WASM output only.
- Second-use board and line pressure is recorded and deferred.
- No extraction to `engine-core` or `game-stdlib` occurs in Gate 5.
