# Starbridge Crossing UI

Game ID: `starbridge_crossing`

Display name: `Starbridge Crossing`

Status: Gate 20 browser presentation contract.

## Surface

The browser renderer presents the Rust/WASM public projection for a 121-space
six-point star board. It shows every public space, coordinate label, zone label,
peg occupant, active seat, finish ranks, ply count, command count, and the
all-public audit note. Starbridge Crossing has no private hands, hidden deck,
commitment, team, or seat-private fact class.

The board uses fixed regions:

- 121 SVG board spaces from Rust-projected `ui.anchor` metadata;
- seat legend rows for the active 2, 3, 4, or 6 seats;
- selected action path status;
- legal peg, step, jump, jumped-over, continue, and stop affordances from the
  Rust action tree only;
- semantic effect feedback for step, jump-chain, finish, blocked-pass, and
  terminal effects.

## Interaction

Users build paths on the board: choose a Rust-marked peg, choose a Rust-marked
step or jump landing, and for jump chains choose Rust-provided `Continue` or
`Stop` controls. Keyboard operation follows the same path: focused legal spaces
submit with Enter or Space, Escape clears a pending path, and arrow keys move
through the stable space order.

TypeScript may normalize SVG coordinates, group Rust action choices for display,
and apply CSS classes for focus or animation. It must not compute adjacency,
jump midpoints, repeated-landing legality, finish ranks, blocked-pass legality,
terminal standings, or outcome math. Rust action trees, views, and effects are
the only behavioral authority.

## Effects

The renderer treats semantic effects as presentation cues only. `step` marks the
origin and landing, `jump_chain` marks origin, jumped-over spaces, and landings,
`finish_assigned` updates rank copy, `pass_blocked` narrates a Rust blocked-pass
decision, and `terminal` reports the Rust terminal reason. Reduced-motion mode
keeps textual feedback and removes animated-style dashed legal rings.

## Accessibility and no-leak rules

Every board space has an accessible name with its space id, coordinate label,
zone label, occupant, and legal status when Rust exposes one. Seat colors are
paired with text symbols, so color is not the only affordance. DOM text,
accessibility names, `data-starbridge-space` attributes, storage, console logs,
effect text, replay exports, and test ids must not include private-state,
hidden-state, candidate-ranking, or debug-only terms.

Hidden information is not applicable: Starbridge Crossing is fully public.
That does not waive no-leak proof; it means the proof asserts every exposed fact
is a public game fact.

## Outcome / victory explanation

Terminal result variants:

- `starbridge_crossing.finish_order_complete` when all but one active seat has
  finished and Rust assigns the final remaining rank.
- The deterministic turn-limit fallback is a terminal ranking surface with the
  same public-finish-order explanation class and an explicit terminal reason.

Decisive cause variants:

- `finish_order_complete`, backed by `RULES.md` rule IDs `SC-FINISH-001`,
  `SC-FINISH-002`, `SC-FINISH-003`, and `SC-FINISH-004`.
- `turn_limit_progress_vector`, backed by `RULES.md` rule IDs `SC-FINISH-005`
  and `SC-FINISH-006`.

Per-player breakdown fields:

- public seat label;
- home and target point;
- finish rank if assigned;
- active/unfinished status before terminal;
- deterministic progress-vector standing for the turn-limit fallback.

Hidden-info redaction rules:

- not applicable for game facts; all board occupancy, ranks, effects, and
  standings are public;
- public outcome explanations still must not expose debug-only state names,
  private framework fields, bot candidate rankings, or non-public diagnostics.

RULES.md rule IDs:

- scoring and accounting: `SC-SCORE-001` and `SC-SCORE-002`;
- terminal conditions: `SC-END-001`, `SC-END-002`, and `SC-FINISH-001`
  through `SC-FINISH-006`;
- UI no-leak and legal-only controls: `SC-UI-001`.

Web smoke coverage:

- `node scripts/check-outcome-explanations.mjs` validates this section, the
  `RULES.md` scoring/end IDs, the `wasm/client.ts` rationale mirror, and the
  `outcomeExplanationTemplates.ts` key.
- `npm --prefix apps/web run smoke:ui` validates catalog and view shape.
- `npm --prefix apps/web run smoke:effects` validates `step` feedback.
- `node apps/web/e2e/starbridge-crossing.smoke.mjs` validates 121-space render,
  Rust legal previews, jump-chain path building, replay import/export,
  reduced-motion behavior, responsive layout, and no-leak scans.
