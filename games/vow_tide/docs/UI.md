# Vow Tide UI

Game ID: `vow_tide`

Implemented variant: `vow_tide_standard`

Rules version: `vow-tide-rules-v1`

Renderer assumptions version: `vow-tide-ui-v1`

Last updated: 2026-06-21

## Contract

The web UI presents Rust/WASM output only. It never computes bid legality,
dealer-hook filtering, follow-suit legality, trump comparison, trick winner,
scoring, terminal standings, hidden-info redaction, replay authority, or bot
choice.

The board consumes `VowTidePublicView`, `ActionTree`, and viewer-filtered
effects from the WASM bridge. TypeScript may format labels, group controls, and
render the shared outcome panel.

## Product And Visual Target

| Field | Decision |
|---|---|
| public role | Gate 17 hidden-information variable-seat exact-bid trick-taking proof |
| desired feel | quiet card-table surface with readable multi-seat rail |
| visual risk to avoid | debug-console-first UI, casino framing, cluttered seven-seat layout, private card leakage |
| public onboarding need | light; [HOW-TO-PLAY.md](HOW-TO-PLAY.md) is rendered through the shared rules surface |
| catalog identity | original Vow Tide metadata and icon from Rust/WASM catalog |

React + HTML/CSS remains the renderer. Canvas/PixiJS is not needed.

## Object Count And Render Budget

| Surface/region | Expected object count | Maximum official fixture count | Render/update budget | Evidence |
|---|---:|---:|---|---|
| seat rail | 3-7 seats | 7 | stable on phone and desktop | `vow-tide.smoke.mjs` |
| owner hand | 0-10 card buttons | 10 | no stale private hand after hotseat handoff | `vow-tide.smoke.mjs` |
| bid controls | 1-11 bid leaves, one omitted for dealer hook when applicable | 11 | keyboard usable | `vow-tide.smoke.mjs` |
| current trick | 0-7 public played cards | 7 | effect/status-driven update, reduced-motion safe | `VowTideBoard.tsx` |
| score/bid table | one row per active seat | 7 | readable standings and bids | `vow-tide.smoke.mjs` |
| outcome panel | 3-7 standing rows | 7 | shared outcome panel | `check-outcome-explanations.mjs` |

## Multi-Seat Layout

| UI element | Required behavior | Hidden-info safeguard | Small-screen behavior | Tests |
|---|---|---|---|---|
| seat rail | Stable `Tide 1` through selected seat count, active/viewing state, public bid/trick/score summaries. | Labels/classes/test IDs do not encode private cards. | Rail wraps/stacks with compact text. | `vow-tide.smoke.mjs` |
| active/pending seats | Active bidder/player is shown from Rust view. | Reason text uses public phase/seat only. | Turn pill and table row remain visible. | `vow-tide.smoke.mjs` |
| local seat selector | Shell can request observer or any selected seat viewer, including `seat_6`. | Only authorized seat view has `own_hand`; hotseat handoff replaces private hand. | Existing shell selector. | `vow-tide.smoke.mjs`, `seat-label-consistency.smoke.mjs` |
| observer mode | Observer sees public table, hand counts, trump indicator, bids, trick, scores, and hidden private-hand placeholder. | `own_hand` is empty; stock identity/order never renders. | Same board with hidden-hand placeholder. | Vow no-leak e2e |
| team grouping | Not applicable. | No teams or partnerships. | Not applicable. | docs review |

## Legal Action Mapping

| Rust action choice/tree node | Rule IDs | UI control/hit target | Normal mode behavior | Accessibility label | Notes |
|---|---|---|---|---|---|
| `bid/<n>` | `VT-BID-ORDER-001`, `VT-BID-RANGE-001`, `VT-HOOK-001` | Bid button in the Vow action grid. | Enabled only when Rust exposes the leaf. | `Bid <n>` from Rust action metadata. | Dealer hook omission is Rust-owned. |
| `play/<card_id>` | `VT-FIRST-LEAD-001`, `VT-FOLLOW-001`, `VT-TRICK-WIN-001` | Owner hand card button. | Enabled only when Rust exposes the card leaf. | Card label from viewer-authorized hand metadata. | UI does not compute follow suit or trump. |

Illegal choices must not appear as active controls. Learning/debug text may show
only Rust-supplied safe diagnostics.

## Progressive Construction Flow

Vow Tide has no staged action construction. Bids and plays are direct Rust
leaf paths. The UI owns presentation, grouping, focus, and disabled/pending
styling only.

## Semantic Effect-To-Animation Mapping

| Semantic effect | Visual animation/status | Reduced-motion replacement | Settle-to-view check | Rule IDs |
|---|---|---|---|---|
| `bid_accepted` | Latest status and public bid table update. | Text/table update only. | Public bid appears in Rust view. | `VT-BID-PUBLIC-001` |
| `dealer_hook_constrained` | Dealer action grid omits the forbidden bid. | Same omitted control and text status. | Legal tree remains Rust source. | `VT-HOOK-001` |
| `card_played` | Current trick card appears. | Card appears without animation. | Public trick includes played card. | `VT-FOLLOW-001` |
| `trick_captured` | Trick count and latest status update. | Text/table update only. | Captured trick count advances. | `VT-TRICK-WIN-001` |
| `hand_scored` | Scores table and latest status update. | Text/table update only. | Cumulative scores come from Rust. | `VT-SCORE-001` |
| `match_completed` | Shared outcome panel appears. | Outcome facts remain text. | Terminal standings match Rust view. | `VT-TERMINAL-001`, `VT-STANDINGS-001` |

Scheduler adoption status: generic-only plus board-local status copy. The Vow
e2e smoke covers reduced-motion behavior.

## Replay UI

| Feature | Required? | Viewer-safe requirement | Tests/notes |
|---|---:|---|---|
| replay step | yes | Uses viewer-scoped public replay summaries. | `vow-tide.smoke.mjs` |
| effect log display | yes | Viewer-filtered effects only. | WASM bridge and e2e no-leak scan. |
| command log display | redacted public summaries only | No hidden hand/stock data in public export. | viewer-scoped export/import smoke |
| local replay import/export | yes | Public export omits command stream and internal seed/state fields. | Vow e2e smoke |
| bot-vs-bot replay | yes | Public-safe effects and action summaries. | smoke and WASM tests |

## Bot Explanation UI

| Surface | Public mode | Dev mode | Hidden-info safeguard | Tests |
|---|---|---|---|---|
| recent bot action | Shows action family/short reason when available. | May show timing/policy id if viewer-safe. | No opponent hand, stock identity, or candidate hidden facts. | `tests/bots.rs`, e2e no-leak scan |
| "why?" affordance | Optional concise explanation. | Expanded viewer-safe details only. | Own hand facts are never shown to unauthorized viewers. | docs and no-leak tests |
| candidate ranking | Not public. | Only if redacted and viewer-safe. | No actual hidden state or sampled holdings. | future-only |

## Outcome / victory explanation

The shared outcome surface explains Vow Tide terminal results. The terminal
source of truth is Rust scoring and the Rust/WASM projected `VowTidePublicView`
fields rendered by `VowTideBoard.tsx`. TypeScript must not decide score
changes, winner set, ranking, co-winner status, hand count, or decisive cause.

### Terminal result variants

| Result variant | Rust source of truth | Player-facing summary | Rule IDs |
|---|---|---|---|
| `high_score_win` | `VowTideTerminalView.winners`, `standings`, `terminal_rationale` | One seat has the highest cumulative score after the final scheduled hand. | `VT-SCORE-001`, `VT-TERMINAL-001`, `VT-STANDINGS-001`, `VT-OUTCOME-001` |
| `shared_high_score` | `VowTideTerminalView.winners`, `standings`, `terminal_rationale` | Multiple seats share the highest cumulative score after the final scheduled hand. | `VT-SCORE-001`, `VT-TERMINAL-001`, `VT-STANDINGS-001`, `VT-OUTCOME-001` |

### Decisive cause payload

| Cause variant | Rust payload field(s) | Static template key | Notes |
|---|---|---|---|
| `final_schedule` / `high_score_win` | `terminal.winners`, `terminal.standings`, `terminal.hands_played`, `terminal_rationale` | `vow_tide.high_score_win` | Current web panel lists final ranks and scores. |
| `final_schedule` / `shared_high_score` | `terminal.winners`, `terminal.standings`, `terminal.hands_played`, `terminal_rationale` | `vow_tide.shared_high_score` | Co-winners remain public and score-based. |

### Per-player final breakdown

| Breakdown value | Source | Visible to public observer? | Visible to seat viewer? | Hidden-info notes |
|---|---|---:|---:|---|
| final cumulative score | `terminal.standings[].score` and `cumulative_scores` | yes | yes | Public score table. |
| final rank | `terminal.standings[].rank` | yes | yes | Rust-projected competition ranking. |
| winner/co-winner flag | `terminal.standings[].is_winner`, `terminal.winners` | yes | yes | UI may emphasize but not recompute. |
| hands played | `terminal.hands_played` | yes | yes | Public schedule fact. |
| rule IDs | `VT-SCORE-001`, `VT-TERMINAL-001`, `VT-STANDINGS-001`, `VT-OUTCOME-001` | yes | yes | Public rule references only. |

### Showdown and final-standing render

| Contender/seat | Evaluated combo | Used components | Rank vector | Decisive comparison | Folded/non-revealed handling | Visible to viewer? |
|---|---|---|---|---|---|---:|
| every seat | Not a showdown evaluator. | Public final score and completed public hand/trick accounting. | Higher cumulative score rank. | Highest score wins; tied top scores share. | No folded or no-reveal terminal outcome. | yes |

### No-leak rules

- Visible text: outcome text may name winners, ranks, scores, final schedule,
  and hands played only.
- Hidden DOM/accessibility attributes: no hidden text, `aria-label`, `title`,
  CSS class, or screen-reader string may contain unplayed opponent cards, stock
  identities, deck order, or future random facts.
- `data-testid`/selectors: selectors must not encode private card IDs or stock
  facts.
- Storage/logs/dev panel: terminal/debug display must use viewer-safe projected
  view/export data only.
- Effect log/replay export: public exports may include public played cards and
  final scores, not private hands, hidden stock, or internal command streams.
- Bot explanations/candidate rankings: explanations may mention legal action
  family and public/own-hand facts only for authorized viewers; rankings are
  not public.

### Player-facing copy contract

The outcome surface explains only the actual result. It must not include
coaching, counterfactuals, turning-point analysis, hidden-card inference, or
strategy advice.

### Accessibility and reduced motion

- Terminal summary is exposed through the shared outcome panel and status text.
- Decisive cause is text, not color-only or animation-only.
- Player standings include labels, ranks, and score values.
- Keyboard users can reach the terminal panel through normal document order.
- Reduced-motion mode preserves all facts and suppresses nonessential motion.
- Replay terminal renders the same viewer-safe outcome content for the same
  viewer.

### Smoke and tests

| Test case | Terminal path | Required assertion |
|---|---|---|
| `node apps/web/e2e/vow-tide.smoke.mjs` | built-app setup/action/replay no-leak smoke | Board renders 3 and 7 seats, cycles all seven viewers, exports/imports viewer-scoped replay, scans DOM/storage/console for forbidden private terms. |
| `node scripts/check-outcome-explanations.mjs` | static catalog/doc/type/template check | `UI.md`, [RULES.md](RULES.md), `client.ts`, and `outcomeExplanationTemplates.ts` expose the outcome contract and template keys. |
| `cargo test -p vow_tide --test rules` | scoring and terminal fixtures | Rust scoring and terminal outcome remain source of truth. |

## Dev Inspector Boundary

| Inspector item | Public build allowed? | Dev build allowed? | Must not contain | Tests |
|---|---:|---:|---|---|
| seed/rules/data version | yes | yes | hidden deck/order reconstruction | e2e no-leak scan |
| public view summary | yes | yes | private hands/stock identities | WASM no-leak and e2e |
| action tree | no by default | yes if actor/viewer-authorized | other seats' cards or hidden reasons | `wasm-api` tests |
| effect log | yes if viewer-filtered | yes if viewer-filtered | hidden stock or other hands | visibility tests |
| command log/export | redacted only | redacted only | private cards for unauthorized viewers | replay export tests |
| full internal state | no | test harness only | all hidden state | not shipped |

## Accessibility Labels And Focus

| Element/control | Accessible name/description | Role/semantic element | Keyboard path | Notes |
|---|---|---|---|---|
| board | `Vow Tide` heading/status. | section | document order | `aria-labelledby` on board. |
| seat rail | Seats group. | section/articles | document order | Scores, bids, hand counts, and active state are text. |
| bid button | `Bid <n>`. | button | Tab, Enter/Space | Only Rust legal bids render enabled. |
| hand card | Rust/WASM card accessibility label. | button | Tab, Enter/Space | Only owner-authorized cards render as private buttons. |
| outcome panel | Shared outcome labels. | region/panel | document order | Reduced motion does not remove facts. |

## Evidence

- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:e2e`
- `node apps/web/e2e/vow-tide.smoke.mjs`
- `node scripts/check-outcome-explanations.mjs`
- `cargo test -p vow_tide`
