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

## Verification

- `npm --prefix apps/web run smoke:e2e`
- `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md`
