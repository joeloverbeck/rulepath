# Crest Ledger UI

Game ID: `poker_lite`

Implemented variant: `poker_lite_standard`

Rules version: `poker-lite-rules-v1`

Last updated: 2026-06-09

## Contract

The web UI presents Rust/WASM output only. It never computes legality,
availability, pledge accounting, reveal timing, terminal allocation, showdown
comparison, hidden-info redaction, replay authority, stale diagnostics, or bot
choice. The board consumes `PokerLitePublicView`, `ActionTree`, and effect
payloads from the WASM bridge.

## Board Layout

`PokerLiteBoard.tsx` renders:

| Surface | Rust payload source | UI behavior |
|---|---|---|
| active seat and terminal status | `active_seat`, `terminal`, `round` | Turn pill, heading, and terminal panel. |
| shared pool and round facts | `shared_pool`, `round.round_index`, `round.round_unit`, `round.outstanding_actor` | Metric boxes with viewer-safe labels. |
| seat ledgers | `contributions`, `private_counts`, `active_seat` | One panel per seat with public contribution count and hidden private count. |
| center crest | `center` | Hidden placeholder before center reveal; `CrestCard` after Rust marks it revealed. |
| contribution track | `contributions` | Public bar visualization of per-seat contribution totals. |
| private view | `private_view` | Owner seat view shows only the owner's private crest; observer/opponent get a hidden placeholder. |
| legal actions | `actionTree.choices` | Native buttons for Rust-legal choices only. |
| grouped showdown | `showdown` | One public grouped reveal panel after showdown terminal. |
| terminal outcome | `terminal` | Yield terminal states that the match resolved without private reveal; showdown states show shared-pool result. |
| recent effects | `latestEffect` and effect log entries | Text status for setup, pledge, reveal, allocation, terminal, and bot events. |
| replay | viewer-scoped public export/import projection | Replay viewer shows public timeline with redacted command summaries. |

## UI Metadata

Rust `ui.rs` provides the stable labels:

| Field | Current value |
|---|---|
| `display_name` | `Crest Ledger` |
| `surface_label` | `Crest Ledger board` |
| `shared_pool_label` | `Shared pool` |
| `hidden_center_label` | `Center crest hidden` |
| `hidden_private_label` | `Private crest hidden` |
| `hold_label` | `Hold` |
| `press_label` | `Press` |
| `lift_label` | `Lift` |
| `match_label` | `Match` |
| `yield_label` | `Yield` |
| `reduced_motion_note` | `Use simple reveal changes when reduced motion is enabled` |

## Legal Action Mapping

| Rust action | Rule IDs | UI control | Accessibility label source | Notes |
|---|---|---|---|---|
| `hold` | `CL-ACT-001` | Native action button | `choice.accessibility_label` from Rust | Adds no markers. |
| `press` | `CL-ACT-001` | Native action button | `choice.accessibility_label` from Rust | Shows Rust metadata such as added markers. |
| `lift` | `CL-ACT-002`, `CL-ACT-003` | Native action button only while Rust exposes it | `choice.accessibility_label` from Rust | Hidden after the round lift cap is consumed. |
| `match` | `CL-ACT-002` | Native action button | `choice.accessibility_label` from Rust | Uses Rust required-to-match metadata. |
| `yield` | `CL-ACT-002`, `CL-END-001` | Native action button | `choice.accessibility_label` from Rust | Ends without private reveal. |

Action `data-testid` values use `choice-poker-lite-round-${round}-${index}`.
They intentionally do not include action segments that could later carry hidden
ids if the action tree evolves.

## Accessibility And Motion

- Core choices are native buttons, reachable by keyboard tab order and
  activatable with Enter/Space.
- Board status is announced through a screen-reader-only live summary and a
  visible latest-effect status region.
- Seat ledgers, center crest, private view, actions, grouped showdown, and
  terminal panels use semantic sections and labels.
- Crest cards include accessible labels from Rust card metadata only after the
  viewer is authorized to see that crest.
- Color is not the only information channel; labels, headings, counts, and
  visible card text carry the state.
- Reduced-motion mode adds `.reduced` and suppresses crest reveal animation
  while preserving all text feedback and final state.
- Responsive layout keeps the table grid measurable on narrow viewports.

## No-Leak Requirements

Before the rule-defined reveal point, hidden crest ids, ranks, copy names, and
labels must not appear in observer/opponent browser payloads, DOM text,
attributes, `data-testid` values, local storage, session storage, console logs,
public replay export/import, dev-panel text, bot public rationale, or effect
logs. Owner seat view may show only that seat's own private crest.

After center reveal, the center crest is public. After showdown, both private
crests and the center crest are public as a grouped reveal. After yield,
private crests remain hidden and the terminal panel says the match resolved
without a private reveal.

## Outcome / victory explanation

The shared outcome surface explains Crest Ledger terminal results from Rust-owned
`PokerLitePublicView` data. TypeScript must render the supplied rationale only;
it must not compare crest ranks, decide pair strength, allocate the shared pool,
or infer why the match ended.

### Terminal result variants

| Result variant | Rust source of truth | Player-facing summary | Rule IDs |
|---|---|---|---|
| `yield_win` | `TerminalView::YieldWin.rationale` | The non-yielding seat wins the shared pool because the other seat yielded; no private reveal occurred. | `CL-PLEDGE-005`, `CL-SCORE-006`, `CL-END-001`, `CL-VIS-007` |
| `showdown_win` | `TerminalView::ShowdownWin.rationale` and `ShowdownView.rationale` | One seat wins the shared pool from the revealed showdown comparison. | `CL-REVEAL-002`, `CL-SCORE-004`, `CL-END-002` |
| `split` | `TerminalView::Split.rationale` and `ShowdownView.rationale` | Equal revealed showdown strength splits the shared pool exactly. | `CL-REVEAL-002`, `CL-SCORE-004`, `CL-SCORE-005`, `CL-END-003` |

### Decisive cause payload

| Cause variant | Rust payload field(s) | Static template key | Notes |
|---|---|---|---|
| `opponent_yielded` | `OutcomeRationaleView.result_kind`, `.decisive_cause`, `.per_seat[].allocation` | `poker_lite.yield_win_no_reveal` | Carries no private crest or strength fields. |
| `pair_beats_high_card` | `OutcomeRationaleView.decisive_cause`, `.per_seat[].strength.pair_bucket`, `.decisive_rule_ids` | `poker_lite.pair_beats_high_card` | Lawful only after showdown reveals both private crests. |
| `higher_private_rank` | `OutcomeRationaleView.decisive_cause`, `.per_seat[].strength.private_rank_value`, `.decisive_rule_ids` | `poker_lite.private_rank_tiebreak` | Lawful only after showdown reveals both private crests. |
| `equal_strength_split` | `OutcomeRationaleView.decisive_cause`, `.per_seat[].allocation`, `.decisive_rule_ids` | `poker_lite.equal_strength_split` | Lawful only after showdown reveals both private crests. |

### Per-player final breakdown

| Breakdown value | Source | Visible to public observer? | Visible to seat viewer? | Hidden-info notes |
|---|---|---:|---:|---|
| result label | `OutcomeRationaleView.per_seat[].result` | yes | yes | `win`, `loss`, `yield_loss`, or `split`; no hidden data. |
| final allocation | `OutcomeRationaleView.per_seat[].allocation` | yes | yes | Public shared-pool accounting. |
| final contribution | `OutcomeRationaleView.per_seat[].contribution` | yes | yes | Public contribution totals. |
| showdown strength | `OutcomeRationaleView.per_seat[].strength` | showdown/split only | showdown/split only | Omitted on yield. Present only after both private crests have been revealed by Rust. |

### No-leak rules

- Visible text: yield explanations must say the result resolved without private reveal and must not name either unrevealed private crest.
- Hidden DOM/accessibility attributes: no `aria-label`, `title`, `alt`, hidden text, or CSS class may contain unrevealed crest IDs, ranks, copies, labels, or strength buckets.
- `data-testid`/selectors: selectors must not encode private crest IDs, rank values, pair buckets, or yielded-hand facts.
- Storage/logs/dev panel: any outcome debug display must use the same viewer-filtered rationale payload as the public panel.
- Effect log/replay export: yield terminal effects and public exports may name winner, loser, pool, and already public center state only; they must not carry private reveal or inferred strength.
- Bot explanations/candidate rankings: bot explanations may not add outcome speculation or opponent hidden-strength facts.

For `YieldWin`, the rationale carries `strength: None` for both seats and uses
`poker_lite.yield_win_no_reveal`. The yielded private crest remains unrevealed
to the public observer and to the winning seat. The losing seat may still see
only its own private crest through the ordinary owner-private view; the outcome
rationale itself does not reveal it or compute a would-have-won comparison.

### Player-facing copy contract

The outcome surface explains only the actual result: yield, pair-vs-high-card,
private-rank tiebreak, or equal-strength split. It must not include coaching,
counterfactuals, turning-point analysis, or strategy advice.

### Accessibility and reduced motion

- The terminal summary must be exposed as a status/result message.
- The decisive cause must be present as text, not only a card highlight or color.
- Player standing and allocation must be color-independent.
- Expanded showdown breakdown must be keyboard accessible.
- Reduced-motion mode must preserve all result facts without requiring reveal
  animation.
- Replaying to terminal must render the same rationale for the same viewer.

### Smoke and tests

| Test case | Terminal path | Required assertion |
|---|---|---|
| yield no reveal | `seat_0 press`, `seat_1 yield` | `poker_lite.yield_win_no_reveal`; no private strength or yielded-loser crest in the rationale. |
| pair beats high card | showdown trace with one paired private crest | `poker_lite.pair_beats_high_card`; paired seat wins. |
| private rank tiebreak | showdown trace with no pair and unequal private ranks | `poker_lite.private_rank_tiebreak`; higher private rank wins. |
| equal strength split | showdown trace with equal strength | `poker_lite.equal_strength_split`; both seats receive equal allocation. |

## Evidence

- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:wasm`
- `node apps/web/e2e/poker-lite.smoke.mjs`
- `npm --prefix apps/web run smoke:e2e`
