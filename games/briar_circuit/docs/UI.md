# Briar Circuit UI

Game ID: `briar_circuit`

Implemented variant: `briar_circuit_standard`

Rules version: `briar-circuit-rules-v1`

Renderer assumptions version: `briar-ui-v1`

Last updated: 2026-06-21

## Contract

The web UI presents Rust/WASM output only. It never computes pass legality,
follow-suit legality, hearts-broken legality, trick winner, scoring, terminal
winner, hidden-info redaction, replay authority, or bot choice.

The board consumes `BriarCircuitPublicView`, `ActionTree`, and viewer-filtered
effects from the WASM bridge. TypeScript may format labels, group controls, and
render the shared outcome panel.

## Product And Visual Target

| Field | Decision |
|---|---|
| public role | Gate 16 hidden-information four-seat trick-taking proof |
| desired feel | cozy neutral card-table, readable seat rail, no casino framing |
| visual risk to avoid | debug-console-first UI, trade-dress imitation, private card leakage, cluttered 52-card history |
| public onboarding need | light; [HOW-TO-PLAY.md](HOW-TO-PLAY.md) is rendered through the shared rules surface |
| catalog identity | original Briar Circuit metadata and icon from Rust/WASM catalog |

React + SVG/HTML remains the renderer. Canvas/PixiJS is not needed.

## Object Count And Render Budget

| Surface/region | Expected object count | Maximum official fixture count | Render/update budget | Evidence |
|---|---:|---:|---|---|
| seat rail | 4 seats | 4 | stable on phone and desktop | `briar-circuit.smoke.mjs` |
| owner hand | 0-13 card buttons | 13 | no layout shift on pass/play | `briar-circuit.smoke.mjs` |
| pass controls | up to 13 select/unselect controls plus confirm | 14 controls | keyboard usable | `briar-circuit.smoke.mjs` |
| current trick | 0-4 public cards | 4 | effect-driven reveal class, reduced-motion safe | `BriarCircuitBoard.tsx` |
| captured tricks | 0-13 rows | 13 rows / 52 public card ids after play | scroll/readable history | `briar-circuit.smoke.mjs` |
| between-hands summary | 4 score rows when a hand just closed | 4 | dismissible panel from the projected `last_hand_summary` (public scoring only) | `BriarCircuitBoard.tsx` |
| outcome panel | 4 standing rows | 4 | shared outcome panel | `check-outcome-explanations.mjs` |

## Multi-Seat Layout

| UI element | Required behavior | Hidden-info safeguard | Small-screen behavior | Tests |
|---|---|---|---|---|
| seat rail | Stable four-seat labels, active/viewer styling, hand counts, cumulative scores. | Labels and classes do not encode private cards. | Stacks/flows in compact table. | `briar-circuit.smoke.mjs` |
| active/pending seats | Active seat during play; pass committed/pending counts during pass. | Pending status exposes counts/direction only. | Compact turn pill and pass meter. | `briar-circuit.smoke.mjs` |
| local seat selector | App shell can request observer or a seat-private view. | Only authorized seat view has `own_hand`. | Existing shell selector. | `shell.smoke.mjs`, Briar e2e |
| observer mode | Observer sees public table, hand counts, no owner hand. | `own_hand` is empty; observer placeholder only. | Same board with hidden-hand placeholder. | Briar no-leak e2e |
| team grouping | Not applicable. | No teams/partnerships. | Not applicable. | docs review |

## Legal Action Mapping

| Rust action choice/tree node | Rule IDs | UI control/hit target | Normal mode behavior | Accessibility label | Notes |
|---|---|---|---|---|---|
| `pass/select/<card>` | `BC-PASS-002` | Owner hand card button. | Enabled only when Rust exposes the leaf. | Card label from viewer-authorized card metadata. | Selects a card for pass. |
| `pass/unselect/<card>` | `BC-PASS-002` | Selected owner hand card button. | Enabled only when Rust exposes the leaf. | Card label plus selected state. | Removes staged pass card. |
| `pass/confirm` | `BC-PASS-002`, `BC-PASS-003` | Confirm pass button. | Enabled only when Rust exposes confirm after exactly three selections. | "Confirm pass". | No TypeScript count validation authority. |
| `play/<card>` | `BC-PLAY-*`, `BC-TRICK-*` | Owner hand card button. | Enabled only when Rust exposes the card leaf. | Card label from Rust/WASM view. | UI does not compute follow suit, first trick, or hearts-broken rules. |

Illegal choices must not appear as active controls. Learning/debug text may show
only Rust-supplied safe diagnostics.

## Progressive Construction Flow

| Stage | Rust-owned input/output | UI presentation | Preview needed? | Confirmation needed? | Notes |
|---|---|---|---:|---:|---|
| pass selection | Rust legal pass select/unselect leaves. | Toggle legal owner-hand cards. | no | no | Selection identities are owner-only. |
| pass confirm | Rust exposes `pass/confirm` only at exactly three selected cards. | Confirm button. | no | yes | Confirm may wait for other seats. |
| play | Rust exposes legal `play/<card>` leaves. | Legal owner-hand card buttons. | no | no | Direct action, then effects settle to latest view. |

## Semantic Effect-To-Animation Mapping

| Semantic effect | Visual animation/status | Reduced-motion replacement | Settle-to-view check | Rule IDs |
|---|---|---|---|---|
| `pass_selection_updated` | Selected-card styling and pass count. | Instant selected state. | View refresh after action. | `BC-PASS-002` |
| `pass_commitment_public` | Pass meter/turn status. | Text status only. | Counts from Rust view. | `BC-PASS-003` |
| `pass_exchange_public` | Pass status clears, playing phase appears. | Instant phase update. | Owner hand reflects authorized received cards only. | `BC-PASS-003` |
| `card_played` | Current trick reveal class. | Text/card appears without animation. | Card appears in public trick. | `BC-PLAY-*` |
| `hearts_broken` | Hearts metric changes. | Text metric changes. | Rust view controls metric. | `BC-PLAY-007` |
| `trick_captured` | Captured-trick history row. | Row appears without animation. | Latest view/history after effect. | `BC-TRICK-002` |

Scheduler adoption status: board-native mapping. Shared effect feedback and
reduced-motion paths are exercised by the Briar e2e smoke.

## Replay UI

| Feature | Required? | Viewer-safe requirement | Tests/notes |
|---|---:|---|---|
| replay step | yes | Uses viewer-scoped public replay summaries. | `briar-circuit.smoke.mjs` |
| effect log display | yes | Viewer-filtered public/private effects only. | WASM bridge and e2e no-leak scan. |
| command log display | redacted public summaries only | No private card IDs or pass provenance in public export. | viewer-scoped export/import smoke |
| local replay import/export | yes | Observer export is public; seat-private exports are authorized only. | GAT16BRICIRTRI-015 e2e |
| bot-vs-bot replay | yes | Public-safe action families/explanations. | traces and e2e smoke |

## Bot Explanation UI

| Surface | Public mode | Dev mode | Hidden-info safeguard | Tests |
|---|---|---|---|---|
| recent bot action | Shows action family/short reason when available. | May show timing/policy id if viewer-safe. | No opponent hand, pass provenance, deck order, or candidate hidden facts. | `tests/bots.rs`, e2e no-leak scan |
| "why?" affordance | Optional concise explanation. | Expanded viewer-safe details only. | Candidate rankings remain redacted. | docs and no-leak tests |
| candidate ranking | Not public. | Only if redacted and viewer-safe. | No actual hidden state or sampled holdings. | future-only |

## Outcome / victory explanation

The shared outcome surface explains Briar Circuit terminal results. The
terminal source of truth is Rust scoring and the Rust/WASM projected
`BriarCircuitPublicView` fields rendered by `BriarCircuitBoard.tsx`.
TypeScript must not decide rule legality, score changes, moon status, low-score
tie continuation, or terminal winner. The current board formats the Rust
projected terminal view into the shared `OutcomeExplanationPanel`.

### Terminal result variants

| Result variant | Rust source of truth | Player-facing summary | Rule IDs |
|---|---|---|---|
| `unique_low_score_win` | `TerminalOutcome::UniqueLowScoreWin`, `OutcomeBreakdown.status`, projected `phase`, `cumulative_scores` | A unique lowest score wins after a threshold hand. | `BC-MATCH-002`, `BC-MATCH-003`, `BC-OUTCOME-001` |
| `tied_low_continuation` | `OutcomeStatus::TiedLowContinuation` while scoring a threshold hand | The lowest score is tied, so another complete hand is required. | `BC-MATCH-003`, `BC-OUTCOME-001` |
| `moon_adjustment` | `HandScoreBreakdown.moon_shooter`, `SeatOutcomeBreakdown.moon_status`, `adjusted_hand_addition` | Capturing all 26 points changes hand additions. | `BC-SCORE-003`, `BC-OUTCOME-001` |

### Decisive cause payload

| Cause variant | Rust payload field(s) | Static template key | Notes |
|---|---|---|---|
| `match_threshold` / `unique_low_score_win` | `phase`, `cumulative_scores`; Rust terminal state is `TerminalOutcome::UniqueLowScoreWin` | `briar_circuit.low_score_win` | Current web panel lists final scores and emphasizes the lowest score. |
| `moon_adjustment` | `HandScoreBreakdown.moon_shooter`, `SeatOutcomeBreakdown.moon_status`, `adjusted_hand_addition` | `briar_circuit.moon_adjustment` | Reserved template key; no hidden card facts are needed. |
| `tied_low_continuation` | `OutcomeStatus::TiedLowContinuation.tied_low_score`, `tied_seats` | `briar_circuit.tied_low_continuation` | Non-terminal threshold continuation; documented for rule parity. |

### Per-player final breakdown

| Breakdown value | Source | Visible to public observer? | Visible to seat viewer? | Hidden-info notes |
|---|---|---:|---:|---|
| final cumulative score | `BriarCircuitPublicView.cumulative_scores` | yes | yes | Public score table. |
| winner/loss emphasis | Rust terminal status plus projected final scores | yes | yes | UI formats standings; it may not invent a different winner. |
| raw hand points | `SeatOutcomeBreakdown.raw_hand_points` | yes when projected in terminal/scoring rationale | yes | Raw point counts do not reveal unplayed private cards after hand close. |
| moon status/addition | `SeatOutcomeBreakdown.moon_status`, `adjusted_hand_addition` | yes when projected | yes | No hidden provenance or pass-origin facts. |
| rule IDs | `BC-SCORE-003`, `BC-MATCH-002`, `BC-MATCH-003`, `BC-OUTCOME-001` | yes | yes | Public rule references only. |

### Showdown and final-standing render

| Contender/seat | Evaluated combo | Used components | Rank vector | Decisive comparison | Folded/non-revealed handling | Visible to viewer? |
|---|---|---|---|---|---|---:|
| every seat | Not a showdown evaluator. | Public final score and completed public trick history. | Lower cumulative score rank. | Unique lowest score wins; tied low continues. | No folded or no-reveal terminal outcome. | yes |

### No-leak rules

- Visible text: outcome text may name final scores, winner, moon adjustment, and
  tie-continuation facts only.
- Hidden DOM/accessibility attributes: no hidden text, `aria-label`, `title`,
  CSS class, or screen-reader string may contain unplayed opponent cards, pass
  provenance, deck order, or seed-derived future facts.
- `data-testid`/selectors: selectors must not encode private card IDs or hidden
  pass facts.
- Storage/logs/dev panel: terminal/debug display must use viewer-safe projected
  view/export data only.
- Effect log/replay export: public exports may include public played cards and
  final scores, not private hands, pass provenance, or deck material.
- Bot explanations/candidate rankings: explanations may mention legal action
  family and public/own-hand facts only; rankings are not public.

### Player-facing copy contract

The outcome surface explains only the actual result. It must not include
coaching, counterfactuals, turning-point analysis, hidden-card inference, or
strategy advice.

### Accessibility and reduced motion

- Terminal summary is exposed through the shared outcome panel and status text.
- Decisive cause is text, not color-only or animation-only.
- Player standings include labels and score values.
- Keyboard users can reach the terminal panel through normal document order.
- Reduced-motion mode preserves all facts and suppresses nonessential motion.
- Replay terminal renders the same viewer-safe outcome content for the same
  viewer.

### Smoke and tests

| Test case | Terminal path | Required assertion |
|---|---|---|
| `node apps/web/e2e/briar-circuit.smoke.mjs` | built-app play/replay/import no-leak smoke | Board renders safely, exports/imports viewer-scoped replay, scans DOM/storage/console for forbidden private terms. |
| `node scripts/check-outcome-explanations.mjs` | static catalog/doc/type/template check | `UI.md`, [RULES.md](RULES.md), `client.ts`, and `outcomeExplanationTemplates.ts` expose the outcome contract and template keys. |
| `cargo test -p briar_circuit --test rules` | threshold and moon rule fixtures | Rust scoring and terminal outcome remain source of truth. |

## Dev Inspector Boundary

| Inspector item | Public build allowed? | Dev build allowed? | Must not contain | Tests |
|---|---:|---:|---|---|
| seed/rules/data version | yes | yes | hidden deck/order reconstruction | e2e no-leak scan |
| public view summary | yes | yes | private hands/pass provenance | WASM no-leak and e2e |
| action tree | no by default | yes if actor/viewer-authorized | opponent cards, hidden reasons | `wasm-api` tests |
| effect log | yes if viewer-filtered | yes if viewer-filtered | private pass effects for other seats | visibility tests |
| command log/export | redacted only | redacted only | private selected/pass cards for unauthorized viewers | replay export tests |
| full internal state | no | test harness only | all hidden state | not shipped |

## Accessibility Labels And Focus

| Element/control | Accessible name/description | Role/semantic element | Keyboard path | Notes |
|---|---|---|---|---|
| board | `Briar Circuit` heading/status. | section | document order | `aria-labelledby` on board. |
| seat rail | Seats group. | section/articles | document order | Scores and hand counts are text. |
| hand card | Rust/WASM card accessibility label. | button | Tab, Enter/Space | Only owner-authorized cards render as buttons. |
| pass confirm | `Confirm pass`. | button | Tab, Enter/Space | Enabled only from Rust action tree. |
| outcome panel | Shared outcome labels. | region/panel | document order | Reduced motion does not remove facts. |

## Evidence

- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:e2e`
- `node apps/web/e2e/briar-circuit.smoke.mjs`
- `node scripts/check-outcome-explanations.mjs`
