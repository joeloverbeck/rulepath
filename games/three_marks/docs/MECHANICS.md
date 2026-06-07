# Three Marks Mechanics Inventory

Game ID: `three_marks`

Rules version: `three_marks-rules-v1`

Last updated: 2026-06-07

## Purpose

This inventory records the mechanic shapes introduced by Three Marks and their extraction posture. It is evidence for `docs/MECHANIC-ATLAS.md`; it is not permission to generalize.

## Mechanic inventory

| Category | Game-local description | Evidence | Status | Notes |
|---|---|---|---|---|
| topology/spatial model | Fixed 3 by 3 public cell grid. | `CellId`, `game-stdlib::board_space`, `ThreeMarksState`, `PublicView`. | `promoted-primitive-conformant` | Coordinate/cell identity uses the promoted `board_space` primitive; no `engine-core` board noun. |
| component/zone model | Nine cells, two seats, public marks. | `ids.rs`, `state.rs`, `ui.rs`. | `local-only` | No zones, hands, decks, or hidden areas. |
| action shape | Flat targeted placement `place/<cell>` for Rust-provided empty cells. | `actions.rs`, rule/property tests. | `local-only` | UI renders action ids; it does not decide legality. |
| turn/phase model | Alternating single-action turns until win or draw. | `rules.rs`, traces. | `local-only` | No phases, reactions, simultaneous choices, or cleanup windows. |
| randomness/chance | No rule randomness; bot seed is outside rules. | setup/replay tests. | `local-only` | Replay command stream fully determines rule state. |
| visibility/hidden information | Perfect information; private view explicitly not applicable. | visibility tests; no-leak smoke. | `local-only` | Future hidden-info games must re-audit DOM/storage/replay surfaces. |
| placement/occupancy | Mark placement into empty fixed cells; occupied cells inert. | `rules.rs`, `three-marks.smoke.mjs`. | `first-use-local-only` | This is the first implemented fixed-2D occupancy pressure point. |
| pattern/line detection | Row, column, and diagonal three-in-a-row terminal scan. | `winning_line`, rule tests. | `first-use-local-only` | Similar to future `column_four`, but not extracted. |
| terminal outcome | Winner by line or draw by full board. | terminal tests/traces. | `local-only` | Win precedes draw if both would be true. |
| semantic effects | Placement, active-player change, rejection, line, draw, game ended, bot choice. | `effects.rs`, replay support, UI smoke. | `local-only` | Effects are viewer-safe and drive UI feedback. |
| UI interaction pattern | Board-first buttons from Rust legal targets; replay board from Rust projection. | `ThreeMarksBoard.tsx`, `ReplayViewer.tsx`. | `local-only` | Generic action list remains secondary. |
| bot policy pattern | Level 0 random legal; Level 1 deterministic priority policy. | `bots.rs`, bot tests. | `local-only` | No MCTS/ML/RL. |
| benchmark pressure | Legal tree, apply, public view, replay projection, serialization, playout, bot decisions. | `benches/three_marks.rs`, `thresholds.json`. | `local-only` | Random playout missed visible target; documented in `BENCHMARKS.md`. |

## Repeated-shape comparison

| Mechanic shape | Already appears in | Same shape? | Similarities | Differences | Required next step |
|---|---|---:|---|---|---|
| fixed 2D occupancy | first implemented in `three_marks`; planned `column_four` pressure | not yet | public grid, occupancy, targeted cells | no gravity, no variable height, no connect-four length | keep local; compare at `column_four`; no extraction now |
| simple line/pattern detection | first implemented in `three_marks`; planned `column_four`/`directional_flip` pressure | not yet | scanning ordered groups for terminal pattern | Three Marks has fixed eight lines only | keep local; Stage 4 review before any helper |

## Primitive candidates

| Candidate | Status | Games exerting pressure | Required next step | Blocker? |
|---|---|---|---|---:|
| fixed board occupancy helper | `deferred` | `three_marks` only implemented | wait for second implemented board and atlas review | yes, first-use only |
| line scan helper | `deferred` | `three_marks` only implemented | compare exact shape after `column_four` | yes, first-use only |

## Extraction rationale

Three Marks now conforms its 3 by 3 coordinate/cell identity to the promoted
`game-stdlib::board_space` primitive. Placement, occupancy, win-line detection,
effects, bot policy, and UI projection remain game-local; `engine-core` remains
noun-free.

## Review checklist

- Board/cell/line/mark vocabulary is game-local.
- Rust owns legal targets, validation, effects, bot choices, replay, and public views.
- TypeScript renders only public view/effect/action ids.
- First-use board and line-shape pressure is recorded but not extracted.
