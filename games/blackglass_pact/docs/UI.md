# Blackglass Pact UI

Game ID: `blackglass_pact`

Implemented variant: `blackglass_pact_standard`

Rules version: `blackglass-pact-rules-v1`

Renderer assumptions version: `blackglass-ui-v1`

Last updated: 2026-06-25

## Contract

The web UI presents Rust/WASM output only. It never computes blind-nil
eligibility, bid legality, follow-suit legality, broken-spades legality, trick
winner, team contract, nil result, bags, scoring, terminal outcome, hidden-info
redaction, replay authority, or bot choice.

The board consumes the Blackglass Pact public view, the Rust action tree, and
viewer-filtered effects from the WASM bridge. TypeScript may format labels,
group team and seat panels, and render shared outcome copy.

## Product And Visual Target

| Field | Decision |
|---|---|
| public role | Gate 18 fixed-four partnership trick-taking proof |
| desired feel | clear partnership table with readable team/accounting facts |
| visual risk to avoid | casino/trade-dress framing, color-only teams, hidden card leakage, debug-first controls |
| public onboarding need | moderate; [HOW-TO-PLAY.md](HOW-TO-PLAY.md) is rendered through the shared rules surface |
| catalog identity | original Blackglass Pact metadata and catalog entry from Rust/WASM |

React + HTML/CSS remains the renderer. Canvas/PixiJS is not needed.

## Object Count And Render Budget

| Surface/region | Expected object count | Maximum official fixture count | Render/update budget | Evidence |
|---|---:|---:|---|---|
| team summaries | 2 teams | 2 | stable on phone and desktop | `blackglass-pact.smoke.mjs` |
| seat rail | 4 seats | 4 | fixed-four layout with active/viewer styling | `blackglass-pact.smoke.mjs` |
| owner hand | 0-13 card buttons | 13 | no partner/opponent hand subtree mounted | `blackglass-pact.smoke.mjs` |
| blind/bid controls | 0-14 non-card buttons by phase | 14 | keyboard usable, Rust legal leaves only | `BlackglassPactBoard.tsx` |
| current trick | 0-4 public cards | 4 | effect-driven reveal class, reduced-motion safe | `BlackglassPactBoard.tsx` |
| last hand score | 0 or 2 team rows | 2 | compact accounting table from Rust score rows | `smoke:ui`, e2e |
| outcome panel | 2 final team rows | 2 | shared outcome panel | `check-outcome-explanations.mjs` |

## Partnership Layout

| UI element | Required behavior | Hidden-info safeguard | Small-screen behavior | Tests |
|---|---|---|---|---|
| team summaries | Shows North-South and East-West, score, bags, and ordinary contract. | Values are Rust-projected public fields. | Stacks into readable cards above the table. | `blackglass-pact.smoke.mjs` |
| seat frames | Shows North, East, South, West, team label, hand count, and bid label. | Counts and bids are public; no private card identity. | Compact grid. | `blackglass-pact.smoke.mjs` |
| current trick | Shows public played cards and the acting/waiting seat. | Only cards already played publicly render here. | Wraps below seat rail. | `blackglass-pact.smoke.mjs` |
| private hand | Shows only the authorized viewer seat hand. | Observer sees hidden placeholder; partner/opponent cards are not mounted. | Scrolls/wraps without overlapping actions. | `blackglass-pact.smoke.mjs` |
| viewer selector | App shell can request observer or one of four seats. | Viewer mode changes request a fresh Rust projection. | Existing shared shell selector. | `shell.smoke.mjs`, Blackglass e2e |

## Legal Action Mapping

| Rust action choice/tree node | Rule IDs | UI control/hit target | Normal mode behavior | Accessibility label | Notes |
|---|---|---|---|---|---|
| `blind_nil/declare` | `BP-BLIND-001` through `BP-BLIND-005` | Blind action button. | Enabled only when Rust exposes the leaf. | Rust leaf label. | No card preview exists in this phase. |
| `blind_nil/decline` | `BP-BLIND-001` through `BP-BLIND-005` | Blind action button. | Enabled only when Rust exposes the leaf. | Rust leaf label. | Does not alter deal bytes. |
| `bid/nil` | `BP-BID-*` | Bid action button. | Enabled only when Rust exposes the leaf. | Rust leaf label. | UI does not decide nil availability. |
| `bid/1` through `bid/13` | `BP-BID-*` | Bid action button. | Enabled only when Rust exposes the leaf. | Rust leaf label. | UI does not sum team contracts. |
| `play/<card-id>` | `BP-PLAY-*` | Owner hand card button. | Enabled only when Rust exposes the matching card leaf. | Viewer-authorized card label. | UI does not compute follow suit or broken spades. |

Illegal choices must not appear as active controls. Learning/debug text may show
only Rust-supplied safe diagnostics.

## Progressive Construction Flow

| Stage | Rust-owned input/output | UI presentation | Preview needed? | Confirmation needed? | Notes |
|---|---|---|---:|---:|---|
| blind commitment | Rust exposes declare/decline for eligible active seats. | Action buttons plus public phase status. | no | no | No card datum exists yet. |
| bidding | Rust exposes nil/numeric bid leaves for the active non-blind seat. | Action button grid. | no | no | Accepted bids immediately become public. |
| play | Rust exposes legal `play/<card-id>` leaves for the active owner. | Legal owner-hand card buttons. | no | no | Direct action; effects settle to latest view. |
| scoring | Rust emits completed-hand accounting and next phase/terminal state. | Last-hand table and outcome panel if terminal. | no | no | UI formats supplied score rows only. |

## Semantic Effect-To-Animation Mapping

| Semantic effect | Visual animation/status | Reduced-motion replacement | Settle-to-view check | Rule IDs |
|---|---|---|---|---|
| blind decision | Latest event/status copy and phase progress. | Text status only. | Public commitment visible, no cards. | `BP-BLIND-*` |
| deal completed | Private hand appears only for authorized seat; counts update. | Instant view update. | 13 owner cards only in seat view. | `BP-DEAL-*` |
| bid accepted | Bid label and contract fields update. | Text status only. | Team contract from Rust field. | `BP-BID-*` |
| spades broken | Spades metric changes. | Text metric changes. | Rust view controls metric. | `BP-PLAY-002` through `BP-PLAY-004` |
| card played | Current trick reveal class. | Card appears without motion. | Public trick contains played card only. | `BP-PLAY-*` |
| trick captured | Trick state clears/advances; latest event names winner. | Text status only. | Winner/next leader from Rust. | `BP-PLAY-007` through `BP-PLAY-010` |
| hand scored | Score/bag table updates. | Instant table update. | Score rows from Rust. | `BP-SCORE-*` |
| match complete | Shared outcome panel appears. | Static outcome panel. | Terminal team from Rust. | `BP-END-*` |

Scheduler adoption status: board-native mapping. Shared effect feedback and
reduced-motion paths are exercised by the Blackglass e2e smoke.

## Replay UI

| Feature | Required? | Viewer-safe requirement | Tests/notes |
|---|---:|---|---|
| replay step | yes | Uses viewer-safe replay projections from Rust/WASM. | `blackglass-pact.smoke.mjs` |
| effect log display | yes | Viewer-filtered public/private effects only. | WASM bridge and e2e no-leak scan. |
| command log display/export | viewer-safe only | No private hands, future deck, or unauthorized card facts. | export/import e2e no-leak scan |
| local replay import/export | yes | Public import cannot elevate viewer privilege. | Blackglass e2e |
| bot-vs-bot replay | yes | Public-safe action families/explanations. | traces and e2e smoke |

## Bot Explanation UI

| Surface | Public mode | Dev mode | Hidden-info safeguard | Tests |
|---|---|---|---|---|
| recent bot action | Shows action family/short reason when available. | May show policy id and timing if viewer-safe. | No partner/opponent hand, future deck, or hidden sampled world. | `tests/bots.rs`, e2e no-leak scan |
| why affordance | Optional concise explanation. | Expanded viewer-safe details only. | Candidate rankings remain redacted. | docs and no-leak tests |
| candidate ranking | Not public. | Only if redacted and viewer-safe. | No actual hidden state or sampled holdings. | future-only |

## Outcome / victory explanation

The shared outcome surface explains Blackglass Pact terminal results and the
documented threshold tie-continuation variant. The terminal source of truth is
Rust scoring and the Rust/WASM projected Blackglass Pact view fields rendered
by `BlackglassPactBoard.tsx`. TypeScript must not decide rule legality, team
contract, score changes, bag penalties, tie continuation, or terminal winner.

### Terminal result variants

| Result variant | Rust source of truth | Player-facing summary | Rule IDs |
|---|---|---|---|
| `team_score_win` | Terminal phase with `winning_team`, `team_scores`, `team_bags`, and Rust outcome rationale. | A team wins after a complete hand because it is uniquely higher at or above the 500-point target. | `BP-END-001`, `BP-END-002`, `BP-END-003` |
| `tied_threshold_continues` | Non-terminal post-hand evaluation when scores are tied at or above 500. | A tied threshold hand does not end the match; another complete hand is required. | `BP-END-002`, `BP-END-004`, `BP-END-006` |

### Decisive cause variants

| Cause variant | Rust payload field(s) | Static template key | Notes |
|---|---|---|---|
| `terminal_score_threshold` | `phase.winning_team`, `team_scores`, `team_bags`, `outcome_rationale` | `blackglass_pact.team_score_win` | Current web panel lists final team totals and emphasizes the winning team. |
| `tied_threshold_continues` | Current team scores after hand scoring and non-terminal phase | `blackglass_pact.tied_threshold_continues` | Documented for rule parity and static copy coverage. |

### Per-player final breakdown

| Breakdown value | Source | Visible to public observer? | Visible to seat viewer? | Hidden-info notes |
|---|---|---:|---:|---|
| final team score | `team_scores` | yes | yes | Public score table. |
| final team bags | `team_bags` | yes | yes | Public accounting field. |
| winner/loss emphasis | Rust terminal phase plus projected team standings | yes | yes | UI formats standings; it may not invent a different winner. |
| team contract | `team_contracts` and last-hand score rows | yes | yes | Public bids/contracts only. |
| seat bid/tricks/nil result | `last_hand_score.seats` when projected | yes after scoring | yes | Completed-hand facts are public score evidence. |
| rule IDs | `BP-SCORE-*`, `BP-END-*` | yes | yes | Public rule references only. |

### Showdown and final-standing render

| Contender/team | Evaluated combo | Used components | Rank vector | Decisive comparison | Folded/non-revealed handling | Visible to viewer? |
|---|---|---|---|---|---|---:|
| North-South | Team score at completed hand. | Public bids, tricks, nil results, bags, cumulative score. | Higher score rank after target predicate. | Unique higher team at or above 500 wins. | No folded or no-reveal terminal outcome. | yes |
| East-West | Team score at completed hand. | Public bids, tricks, nil results, bags, cumulative score. | Higher score rank after target predicate. | Exact ties continue; bags and seat order are not tiebreakers. | No folded or no-reveal terminal outcome. | yes |

### No-leak rules

- Visible text: outcome text may name final team scores, bags, winner, and
  tie-continuation facts only.
- Hidden DOM/accessibility attributes: no hidden text, `aria-label`, `title`,
  CSS class, or screen-reader string may contain unplayed partner/opponent
  cards, future deck order, blind-derived hidden facts, or bot candidate data.
- `data-testid`/selectors: selectors must not encode private card IDs or hidden
  hand facts.
- Storage/logs/dev panel: terminal/debug display must use viewer-safe projected
  view/export data only.
- Effect log/replay export: public exports may include public played cards,
  bids, team scores, and terminal status, not private hands or future deck
  material.
- Bot explanations/candidate rankings: explanations may mention legal action
  family and public or own-hand facts only; hidden-world sampling is forbidden.

### Player-facing copy contract

The outcome surface explains only the actual result. It must not include
coaching, counterfactuals, turning-point analysis, hidden-card inference, or
strategy advice.

### Accessibility and reduced motion

- Terminal summary is exposed through the shared outcome panel and status text.
- Decisive cause is text, not color-only or animation-only.
- Team standings include labels, score values, and bag values.
- Keyboard users can reach the terminal panel through normal document order.
- Reduced-motion mode preserves all facts and suppresses nonessential motion.
- Replay terminal renders the same viewer-safe outcome content for the same
  viewer.

### Smoke and tests

| Test case | Terminal path | Required assertion |
|---|---|---|
| `node apps/web/e2e/blackglass-pact.smoke.mjs` | built-app setup/play/replay no-leak smoke | Board renders safely, exports/imports replay, scans DOM/storage/console for forbidden private terms. |
| `node scripts/check-outcome-explanations.mjs` | static catalog/doc/type/template check | `UI.md`, [RULES.md](RULES.md), `client.ts`, and `outcomeExplanationTemplates.ts` expose the outcome contract and template keys. |
| `cargo test -p blackglass_pact --test rules` | scoring, target, and tie-continuation rules | Rust scoring and terminal outcome remain source of truth. |

## Dev Inspector Boundary

| Inspector item | Public build allowed? | Dev build allowed? | Must not contain | Tests |
|---|---:|---:|---|---|
| seed/rules/data version | yes | yes | hidden deck/order reconstruction | e2e no-leak scan |
| public view summary | yes | yes | private hands/future deck | WASM no-leak and e2e |
| action tree | no by default | yes if actor/viewer-authorized | partner/opponent cards, hidden reasons | `wasm-api` tests |
| effect log | yes if viewer-filtered | yes if viewer-filtered | private hand effects for other seats | visibility tests |
| command log/export | viewer-safe only | viewer-safe only | private hands or future deck material | replay export tests |
| full internal state | no | test harness only | all hidden state | not shipped |

## Accessibility Labels And Focus

| Element/control | Accessible name/description | Role/semantic element | Keyboard path | Notes |
|---|---|---|---|---|
| board | `Blackglass Pact` heading/status. | section | document order | `aria-labelledby` on board. |
| team summaries | North-South and East-West labels. | section/articles | document order | Scores, bags, contracts are text. |
| seat frames | North, East, South, West labels. | articles | document order | Hand counts and bids are text. |
| blind/bid action | Rust/WASM action label. | button | Tab, Enter/Space | Enabled only from Rust action tree. |
| hand card | Viewer-authorized card label. | button | Tab, Enter/Space | Only owner-authorized cards render as buttons. |
| outcome panel | Shared outcome labels. | region/panel | document order | Reduced motion does not remove facts. |

## Evidence

- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:effects`
- `npm --prefix apps/web run smoke:e2e`
- `node apps/web/e2e/blackglass-pact.smoke.mjs`
- `node scripts/check-outcome-explanations.mjs`
