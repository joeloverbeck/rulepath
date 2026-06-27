# Meldfall Ledger UI

Game ID: `meldfall_ledger`

Display name: `Meldfall Ledger`

Status: Gate 19 browser presentation contract.

## Surface

The browser renderer presents the Rust/WASM viewer projection for the current
viewer. It shows the public stock count, ordered public discard pile, public
meld tableau, score ledger, active seat, dealer, phase, and the authorized
viewer's own private hand when the viewer is a seat. Opponent hands and stock
order are never browser-side data.

The main board uses fixed zones:

- stock and discard draw zones;
- public meld tableau grouped by Rust meld id, kind, origin seat, and public
  score-credit owners;
- seat score ledger with hand counts, cumulative score, and round played score;
- persistent last-settlement panel sourced from `view.last_settlement`;
- private-hand rail for the authorized seat viewer;
- legal-action panel fed only by Rust action choices.

## Interaction

All legal actions are rendered as ordinary buttons. Drag is not required. The
discard pile buttons submit only Rust-provided `draw-discard-*` choices, the
stock button submits only Rust-provided `draw-stock`, and the grouped action
panel submits the remaining Rust-provided table, discard, and turn choices.

TypeScript may group choices by segment prefix for presentation, but it does not
validate melds, lay-offs, pickup commitments, scoring, turn progression, or
terminal state. The Rust action tree is the only legality source.

The last-settlement panel renders Rust-authored `view.last_settlement` values
only. It may label and order the fields for presentation, but it does not
recompute settlement math. The panel persists across the next round until Rust
projects a newer settlement, and shows:

- round index;
- round-end reason (`go_out_without_discard`, `go_out_by_final_discard:seat=N`,
  or `stock_exhausted`);
- per-seat tabled-positive total;
- per-seat in-hand penalty total and held-card count;
- per-seat round delta;
- per-seat cumulative score;
- per-seat rank and winner flag.

Keyboard operation follows the browser's native button focus order: draw zones,
tableau, private hand, action groups, status, and shared shell controls.

## Effects

The renderer treats Meldfall Ledger semantic effects as presentation cues only.
Draw, meld, lay-off, discard, round-score, and match-terminal payloads update
status copy and target the public draw/tableau/status regions for animation.
Reduced-motion mode keeps the same textual feedback while disabling motion.

## Accessibility and no-leak rules

DOM text, accessibility names, `data-testid` values, storage, console logs, and
effect text must not reveal unauthorized hidden card identities or stock order.
Public discard cards and tabled meld cards may be named because Rust projects
them as public. Own-hand cards may be named only in a seat-authorized view.
Observer views and other seats receive counts only.

## Outcome / victory explanation

Terminal result variants:

- `meldfall_ledger.high_score_win` for the unique winner after a settled round.
- A tied-at-target continuation is non-terminal and does not render an outcome
  panel.

Decisive cause variants:

- `unique_high_score_at_target`, backed by `RULES.md` rule IDs `ML-MATCH-001`,
  `ML-MATCH-002`, and `ML-MATCH-005`.
- `match_tie_continue`, backed by `RULES.md` rule IDs `ML-MATCH-001`,
  `ML-MATCH-003`, and `ML-MATCH-004`, remains a non-terminal state.

Per-player breakdown fields:

- tabled-positive total;
- in-hand penalty total;
- held-card count;
- cumulative score;
- latest round delta;
- rank;
- winner flag.

Hidden-info redaction rules:

- public outcome and last-settlement explanations may show score totals,
  tabled-positive totals, in-hand penalty totals, held counts, round deltas,
  ranks, and winner flags;
- public outcome explanations must not show opponent unmelded card identities,
  stock order, hidden draw identities, or private bot rankings;
- own remaining private cards stay seat-scoped and are not part of the public
  outcome panel or shared last-settlement panel.

RULES.md rule IDs:

- scoring and accounting: `ML-SCORE-001` through `ML-SCORE-007`;
- terminal conditions: `ML-MATCH-001` through `ML-MATCH-005`;
- UI no-leak and legal-only controls: `ML-UI-001` through `ML-UI-003`.

Web smoke coverage:

- `node scripts/check-outcome-explanations.mjs` validates this section, the
  `RULES.md` scoring/end IDs, the `wasm/client.ts` rationale mirror, and the
  `outcomeExplanationTemplates.ts` key.
- `npm --prefix apps/web run smoke:ui` validates the Rust/WASM catalog, view,
  action-tree, and replay surfaces used by the board.
- `npm --prefix apps/web run smoke:effects` validates effect feedback copy for
  Meldfall Ledger draw effects plus shared animation no-leak behavior.
- `node apps/web/e2e/meldfall-ledger.smoke.mjs` validates that the
  `view.last_settlement` breakdown renders during Bot-vs-bot play.
- `node apps/web/e2e/a11y-noleak.smoke.mjs` validates that the settlement panel
  keeps unauthorized hidden card identities out of DOM, accessible names,
  storage, and logs.
