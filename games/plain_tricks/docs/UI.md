# Plain Tricks UI

Game ID: `plain_tricks`

Implemented variant: `plain_tricks_standard`

Rules version: `plain-tricks-rules-v1`

Last updated: 2026-06-09

## Contract

The web UI presents Rust/WASM output only. It never computes legality,
follow-suit availability, trick winner, round scoring, deal rotation, terminal
outcome, hidden-info redaction, replay authority, stale diagnostics, or bot
choice. The board consumes `PlainTricksPublicView`, `ActionTree`, and effect
payloads from the WASM bridge.

## Board Layout

`PlainTricksBoard.tsx` renders:

| Surface | Rust payload source | UI behavior |
|---|---|---|
| active seat and terminal status | `active_seat`, `terminal`, `phase` | Turn pill, heading, and terminal panel. |
| round/trick metrics | `round_index`, `trick_index`, `current_leader`, `total_trick_counts` | Metric boxes with public counts. |
| own hand | `private_view.own_hand` | Owner seat sees only its own unplayed cards. Observer gets a hidden placeholder. |
| opponent hand | `hand_counts` | Face-down count only; no opponent card ids, suits, ranks, or labels. |
| current trick | `current_trick` | Played cards are public from the moment Rust applies them. |
| trick history | `trick_history` | Public resolved tricks with played cards and winner. |
| legal actions | `actionTree.choices` under the `play` node | Card buttons are enabled only when Rust exposes that card leaf. |
| recent effects | `latestEffect` and effect log entries | Text status for deal, play, trick, score, rotation, terminal, and bot events. |
| replay | viewer-scoped public export/import projection | Replay viewer shows public timeline with redacted command summaries. |

## UI Metadata

Rust `ui.rs` provides stable labels for table, own hand, opponent hand, current
trick, trick history, score, play action, observer-disabled reason, reduced
motion note, and rules summary. TypeScript uses these labels as presentation
copy; it does not derive rule behavior from them.

## Legal Action Mapping

| Rust action | Rule IDs | UI control | Accessibility label source | Notes |
|---|---|---|---|---|
| `play/<card-id>` | `PT-ACT-001`, `PT-ACT-002`, `PT-ACT-003` | Native card button in the owner's hand | `choice.accessibility_label` from Rust | The button is enabled only when that exact card id appears under the Rust `play` action node. |

Action `data-testid` values use
`choice-plain-tricks-trick-${trick}-${index}`. They intentionally do not include
raw card ids, suits, ranks, opponent facts, or tail facts.

## Accessibility And Motion

- Core choices are native buttons, reachable by keyboard tab order and
  activatable with Enter/Space.
- Board status is announced through a screen-reader-only live summary and a
  visible latest-effect status region.
- Own hand, current trick, opponent hand, trick history, and terminal panels use
  semantic sections and labels.
- Card labels and accessibility labels appear only for cards the viewer is
  authorized to see: own unplayed cards or public played cards.
- Color is not the only information channel; labels, headings, counts, and text
  carry the state.
- Reduced-motion mode adds `.reduced` and suppresses board animation while
  preserving text feedback and final state.
- Responsive layout keeps the trick table measurable on narrow viewports.

## No-Leak Requirements

Before a card is played, opponent hand card ids, suits, ranks, labels, and tail
card identities must not appear in observer/opponent browser payloads, DOM text,
attributes, `data-testid` values, local storage, session storage, console logs,
public replay export/import, dev-panel text, bot public rationale, or effect
logs. Owner seat view may show only that seat's own unplayed hand.

After play, that card is public in the current trick, history, effects, and
public replay. The tail remains hidden forever, including at terminal.

## Outcome / victory explanation

The shared outcome surface explains Plain Tricks terminal results from
Rust-owned `PlainTricksPublicView` data. TypeScript must render the supplied
rationale only; it must not compare trick totals, decide a winner, infer a
follow-suit fact, or reveal hidden cards.

### Terminal result variants

| Result variant | Rust source of truth | Player-facing summary | Rule IDs |
|---|---|---|---|
| `trick_win` | `TerminalView::TrickWin.rationale` | One seat won more total tricks across both rounds. | `PT-SCORE-002`, `PT-END-001` |
| `split` | `TerminalView::Split.rationale` | Final trick totals are tied 6-6. | `PT-SCORE-002`, `PT-END-002` |

### Decisive cause payload

| Cause variant | Rust payload field(s) | Static template key | Notes |
|---|---|---|---|
| `seat_0_total_tricks:*` | `OutcomeRationaleView.result_kind`, `.decisive_cause`, `.per_seat[].total_tricks` | `plain_tricks.trick_win` | Public count-only result. |
| `seat_1_total_tricks:*` | `OutcomeRationaleView.result_kind`, `.decisive_cause`, `.per_seat[].total_tricks` | `plain_tricks.trick_win` | Public count-only result. |
| `split:*` | `OutcomeRationaleView.result_kind`, `.decisive_cause`, `.per_seat[].total_tricks` | `plain_tricks.split` | Public count-only split. |

### Per-player final breakdown

| Breakdown value | Source | Visible to public observer? | Visible to seat viewer? | Hidden-info notes |
|---|---|---:|---:|---|
| result label | `OutcomeRationaleView.per_seat[].result` | yes | yes | `win`, `loss`, or `split`; no hidden card data. |
| total tricks | `OutcomeRationaleView.per_seat[].total_tricks` | yes | yes | Public trick totals only. |
| decisive rule ids | `OutcomeRationaleView.decisive_rule_ids` | yes | yes | Public rule references. |

### No-leak rules

- Visible text: terminal explanations may name trick totals and winner/split
  status only.
- Hidden DOM/accessibility attributes: no `aria-label`, `title`, hidden text,
  or CSS class may contain unplayed opponent or tail card facts.
- `data-testid`/selectors: selectors must not encode card ids or hidden facts.
- Storage/logs/dev panel: any outcome/debug display must use the same
  viewer-filtered count-only rationale payload as the public panel.
- Effect log/replay export: public exports may include played cards and public
  trick history only; they must not carry unplayed cards, seed evidence, or tail
  card identities.
- Bot explanations/candidate rankings: public bot effects may cite action
  family only; they must not reveal own-hand evaluation or opponent-hand
  guesses.

### Smoke and tests

| Test case | Required assertion |
|---|---|
| browser no-leak smoke | `plain-tricks.smoke.mjs` finds no unplayed card id in DOM, storage, export, replay, or dev panel. |
| WASM redaction test | `cargo test -p wasm-api` proves non-actor trees are empty and public export omits seed/unplayed cards. |
| outcome explanation smoke | e2e full match reaches terminal and renders trick-total explanation. |

## Evidence

- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `node apps/web/e2e/plain-tricks.smoke.mjs`
- `npm --prefix apps/web run smoke:e2e`
