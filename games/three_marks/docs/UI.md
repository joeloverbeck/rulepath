# Three Marks UI Notes

Game ID: `three_marks`

Rules version: `three_marks-rules-v1`

Last updated: 2026-06-06

## Rust/browser boundary

The browser renders Rust-projected public views, Rust legal targets, Rust semantic effects, and Rust replay projections. TypeScript selects the renderer by game/view discriminant, maps public cell strings to UI buttons, and dispatches Rust action ids. It does not decide legality, infer winners, reconstruct replay state from diffs, or store hidden state.

## Live board

`apps/web/src/components/ThreeMarksBoard.tsx` renders:

- nine stable public cell buttons (`r1c1` through `r3c3`);
- occupied/inert cell states from the Rust view;
- legal empty cells from `legal_targets`;
- color-plus-shape mark tokens;
- Rust-provided status, active seat, ply, legal count, winning line, draw/win terminal state;
- public Level 1 bot explanation effect;
- reduced-motion-safe transitions.

## Replay board

`ReplayViewer.tsx` reuses `ThreeMarksBoard` in read-only mode for Rust replay reset/step projections. The placement sequence is display metadata from the replay document; the board state itself comes from Rust replay projection.

## Accessibility and no-leak posture

- All board cells have accessible names.
- Keyboard focus and Enter activation are covered by `three-marks.smoke.mjs`.
- Mark meaning is not color-only.
- DOM/test IDs expose public cell ids only and never state dumps.
- Local storage is limited to the reduced-motion preference.
- Perfect-information private view surfaces are explicitly not applicable.

## Supported setup modes

The shell supports Human vs bot, Hotseat, and Bot vs bot for Three Marks. All modes still dispatch through Rust action trees and bot turns.

## Outcome / victory explanation

Rust includes terminal outcome rationale in the public view so the browser can render the result without recomputing line detection or draw logic.

Terminal result variants are `win` and `draw`. Decisive cause variants are `line_completed` and `full_board_no_line`.

| Terminal kind | Template key | Decisive cause | Public fields | Rule IDs |
|---|---|---|---|---|
| win | `three_marks.line_completed` | `line_completed` | `winning_seat`, ordered `winning_line`, `line_orientation`, `board_full=false` | `TM-SCORE-001`, `TM-END-001` |
| draw | `three_marks.full_board_draw` | `full_board_no_line` | no line cells, `board_full=true` | `TM-SCORE-001`, `TM-END-002` |

There are no per-player breakdown fields for Three Marks; the result is a perfect-information terminal winner or draw. The ordered line cells are the Rust-selected decisive cells and are the only cells the UI may highlight as the outcome explanation. Hidden-info redaction is still explicit: no hidden fields, no private view payload, and no state dumps in DOM attributes, logs, storage, replay export, or tests. TypeScript may map these public ids to labels, but it must not infer a winner, draw, line, or orientation from board occupancy.

Web smoke coverage must assert that win and draw explanations render from Rust fields, highlight only Rust-supplied line cells, and keep the no-leak scan green.

## Verification

- `npm --prefix apps/web run smoke:e2e`
- `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md`
