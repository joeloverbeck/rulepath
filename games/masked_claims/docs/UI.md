# masked_claims UI

Game ID: `masked_claims`

Implemented variant: `masked_claims_standard`

Rules version: `masked-claims-rules-v1`

Renderer assumptions version: `masked-claims-ui-v1`

Prepared by: `Codex`

Last updated: 2026-06-11

## Purpose

This document defines the product-facing web UI plan for Masked Claims.
TypeScript never decides legality. Rust/WASM owns legal action trees,
validation, state transitions, views, semantic effects, diagnostics, replay,
serialization, and bot decisions. TypeScript renders viewer-safe payloads.

## Product and visual target

| Field | Decision |
|---|---|
| public role | hidden-info proof and original Gate 11 portfolio game |
| desired feel | compact abstract table with clear private/public zones |
| visual risk to avoid | debug-console-first, casino vibe, proprietary bluffing-game mimicry, clutter |
| public onboarding need | light |
| help/learning mode need | light, rules drawer sourced from `HOW-TO-PLAY.md` |

## Renderer assumptions

| Assumption | Value | Evidence/notes |
|---|---|---|
| default renderer | React + semantic HTML/SVG accents | Small object count and ordinary controls. |
| expected object count | 15 masks plus public counters/galleries/log rows | Low enough for React. |
| animation pressure | medium | Reveal/accept/challenge feedback matters. |
| SVG pressure expected? | no | Cards/tiles can be HTML elements with labels. |
| Canvas/PixiJS needed? | no by default | No high-volume render loop. |
| WASM boundary | batched Rust calls | UI calls Rust for view/action/apply/bot/replay only. |

## Screen-size support

| Size/context | Supported? | Layout notes | Smoke test |
|---|---:|---|---|
| small phone portrait | yes | Stack public state, action panel, and log; own hand scrolls horizontally. | `masked-claims.smoke.mjs` in ticket 017 |
| phone landscape | yes | Two-column compact table if width allows. | ticket 017 |
| tablet | yes | Table plus side action/log rail. | ticket 017 |
| desktop | yes | Public board, own hand, action panel, and log visible without scrolling. | ticket 017 |
| keyboard-only desktop | yes | All legal choices are buttons with visible focus. | a11y smoke |
| reduced-motion user | yes | Replace motion with instant state update and text highlight. | reduced-motion smoke |

## Legal action mapping

| Rust action choice/tree node | Rule IDs | UI control/hit target | Normal mode behavior | Learning/debug behavior | Accessibility label | Notes |
|---|---|---|---|---|---|---|
| `claim` then own mask then grade | `MC-ACT-001`, `MC-VIS-007` | own-hand mask buttons plus grade segmented control/buttons | enabled only from Rust tree for the claimant | show Rust metadata and safe declared-grade summary | "Claim this mask as <grade>" | The mask ID must not appear outside the owning seat view. |
| `respond/accept` | `MC-ACT-003`, `MC-SCORE-001` | primary response button | enabled only for responder in reaction window | safe explanation: accept scores declared grade | "Accept the claim" | Does not reveal the mask. |
| `respond/challenge` | `MC-ACT-003`, `MC-SCORE-002`, `MC-SCORE-003` | secondary response button | enabled only for responder in reaction window | safe explanation: challenge reveals and resolves | "Challenge the claim" | Reveal appears only after Rust effect. |
| empty claimant tree during reaction | `MC-ACT-004` | waiting state, no gameplay button | render waiting text from safe phase metadata | safe diagnostics only | "Waiting for response" | No disabled hidden controls. |
| terminal empty tree | `MC-ACT-005` | outcome panel and replay controls | no gameplay controls | show public rationale | "Game complete" | Terminal never reveals hidden residue. |

## Progressive construction flow

| Stage | Rust-owned input/output | UI presentation | Preview needed? | Confirmation needed? | Notes |
|---|---|---|---:|---:|---|
| 1 | Rust returns claim family and own mask choices | own-hand tile selection | no separate preview | no |
| 2 | Rust returns legal declared grades for selected mask | grade buttons with labels | yes, declared grade copy only | yes, submit claim |
| 3 | Rust validates `claim/<mask>/<grade>` | pending pedestal shows declared grade only | yes | no |
| 4 | Rust returns responder tree | accept/challenge buttons | no | button click applies response |

At every stage, next choices come from Rust. The UI owns grouping, focus, and
affordance only.

## Rust-generated previews

| Preview | Trigger | Viewer-safe contents | Must not contain | Tests |
|---|---|---|---|---|
| claim preview | selected own mask and grade | declared grade label, claimant, turn | opponent hand, reserve, hidden pedestal in public view | ticket 017 smoke |
| response prompt | pending reaction window | claimant, responder, declared grade, accept/challenge options | pedestal mask ID | ticket 017 smoke |

## Semantic effect-to-animation mapping

| Semantic effect | Visual animation | Timing/priority | Reduced-motion replacement | Settle-to-view check | Rule IDs |
|---|---|---|---|---|---|
| `ClaimPlaced` | own tile moves to pedestal placeholder | immediate after apply | instant placeholder update | render latest Rust view | `MC-TURN-001` |
| `ReactionWindowOpened` | response panel highlights responder | after claim | text/status highlight | action tree matches responder | `MC-TURN-002` |
| `ClaimAccepted` | pedestal moves to veiled gallery | after response | instant gallery count update | accepted mask remains hidden | `MC-SCORE-001` |
| `ChallengeDeclared` | challenge button result pulse | before reveal | text log entry | effect order preserved | `MC-TURN-004` |
| `MaskRevealed` | reveal the challenged mask in exposed row | after challenge declaration | instant exposed row update | only challenged tile is public | `MC-VIS-004` |
| `ScoreChanged` | score counter increments | after resolution | text update | score equals Rust view | `MC-SCORE-*` |
| `Terminal` | outcome panel opens | end of turn 8 | static result panel | rationale matches Rust terminal view | `MC-END-*` |

## Settle-to-view checks

| Scenario | Required check | Test |
|---|---|---|
| after legal claim | pedestal shows declared grade and no tile ID | `masked-claims.smoke.mjs` |
| after accept | veiled gallery count/grade updates and tile ID stays hidden | `masked-claims.smoke.mjs` |
| after challenge | revealed tile appears only after reveal effect | `masked-claims.smoke.mjs` |
| after replay step | renderer settles to replayed public view | `masked-claims.smoke.mjs` |
| after bot action | effect animation and explanation match Rust result | `masked-claims.smoke.mjs` |
| after reduced-motion path | no essential information is animation-only | `masked-claims.smoke.mjs` |

## Replay UI

| Feature | Required? | Viewer-safe requirement | Tests/notes |
|---|---:|---|---|
| replay step forward/back | yes | no hidden state leak | ticket 017/019 |
| effect log display | yes | viewer-filtered effects only | `MC-VIS-*` |
| command log display | yes, public summary only | claim paths redacted to grade | public export test |
| hash/version display | yes | safe metadata only | existing shell patterns |
| local replay import/export | yes | viewer-scoped public export | ticket 014 bridge |
| bot-vs-bot replay | optional | public-safe explanations | no hidden candidate ranks |

## Bot explanation UI

| Surface | Public mode | Dev mode | Hidden-info safeguard | Tests |
|---|---|---|---|---|
| recent bot action | concise Level 1 rationale | policy id/version may show | rationale uses own/public facts only | `tests/bots.rs`; smoke |
| "why?" affordance | optional short explanation | expanded safe metadata | no hidden tile facts | smoke |
| candidate ranking | not public | redacted only if added later | no opponent/reserve/pedestal facts | future |
| known weakness | optional concise note | detailed docs link okay | no private state | manual review |

## Outcome / victory explanation

### Terminal result variants

| Result variant | Rust source of truth | Player-facing summary | Rule IDs |
|---|---|---|---|
| `score_win` | `TerminalView::Complete` / `TerminalOutcome::ScoreWin` | The higher final score wins. | `MC-END-001` |
| `tiebreak_win` | `TerminalOutcome::TiebreakWin` | Scores tied, so the public tiebreak ladder decided the winner. | `MC-END-002` through `MC-END-004` |
| `draw` | `TerminalOutcome::Draw` | Scores and all tiebreakers tied. | `MC-END-005` |

### Decisive cause payload

| Cause variant | Rust payload field(s) | Static template key | Notes |
|---|---|---|---|
| final score comparison | `terminal`, `terminal_rationale.final_scores`, `decisive_rule_ids` | `masked_claims.score_win` | Viewer-safe public scores only. |
| exposed-lie tiebreak | `terminal.tiebreak`, `terminal_rationale.decisive_cause` | `masked_claims.tiebreak_exposed_lies` | Public counters only. |
| successful-challenge tiebreak | same | `masked_claims.tiebreak_successful_challenges` | Public counters only. |
| challenge-discipline tiebreak | same | `masked_claims.tiebreak_challenges_declared` | Public counters only. |
| draw | `terminal.draw` | `masked_claims.draw` | Does not imply hidden facts. |

TypeScript must not compute these cause variants. It renders the Rust-projected
value only.

### Per-player final breakdown

| Breakdown value | Source | Visible to public observer? | Visible to seat viewer? | Hidden-info notes |
|---|---|---:|---:|---|
| final score | `scores` | yes | yes | public |
| exposed lies | `counters` | yes | yes | public |
| successful challenges | `counters` | yes | yes | public |
| challenges declared | `counters` | yes | yes | public |
| veiled gallery declared grades/count | `veiled_gallery` | yes | yes | no tile IDs |
| exposed row | `exposed_rows` | yes | yes | only challenged masks |

### No-leak rules

- Visible text: no unplayed hand IDs, reserve IDs, veiled accepted IDs, or unchallenged pedestal IDs.
- Hidden DOM/accessibility attributes: same no hidden IDs or labels.
- `data-testid`/selectors: stable generic IDs only, not tile IDs for hidden masks.
- Storage/logs/dev panel: viewer-safe public export and filtered effects only.
- Effect log/replay export: claim command summaries redact tile IDs to declared grades.
- Bot explanations/candidate rankings: no opponent hand, reserve, accepted-mask, or hidden pedestal facts.

### Player-facing copy contract

The outcome surface explains only the actual result. It must not include
coaching, counterfactuals, "what would have changed it", turning-point analysis,
or strategy advice.

### Accessibility and reduced motion

- Terminal summary is exposed as a status/result message.
- Decisive cause is text, not color-only or animation-only.
- Player standing is color-independent.
- Expanded breakdown is keyboard accessible.
- Reduced-motion mode preserves all facts.
- Replay terminal renders the same outcome content for the same viewer.

### Smoke and tests

| Test case | Terminal path | Required assertion |
|---|---|---|
| score win | scripted accept/challenge mix | summary cites scores and no hidden residue |
| tiebreak win | terminal tiebreak trace | summary cites public tiebreak |
| draw | draw trace | draw text and no hidden residue |
| accept terminal no-leak | accepted-mask trace | veiled masks remain redacted |

## Dev inspector boundary

| Inspector item | Public build allowed? | Dev build allowed? | Must not contain | Tests |
|---|---:|---:|---|---|
| seed/rules/data version | yes | yes | hidden state | smoke |
| public view inspector | yes | yes | hidden state | no-leak smoke |
| action tree inspector | no by default | yes if viewer-safe | hidden reasons/state | no-leak smoke |
| selected action path | no public raw claim path | yes if actor-owned and safe | hidden public leak | no-leak smoke |
| effect log | yes | yes if viewer-filtered | hidden outcomes | smoke |
| command log | public summaries only | redacted summaries only | private paths | replay smoke |
| bot timing | yes | yes | hidden facts | smoke |
| candidate ranking | no | only if redacted in future | hidden facts | future |
| full internal state | no | test harness only | all hidden state | source review |

## Accessibility labels and semantics

| Element/control | Accessible name/description | Role/semantic element | Keyboard path | Focus behavior | Notes |
|---|---|---|---|---|---|
| own hand mask | "Mask in your hand: <grade label>" | button | tab/arrow then enter/space | remains on selected tile until grade chosen | owner view only |
| grade claim | "Declare <grade label>" | button/group | tab/arrow | moves to submit/confirmation | no hidden facts |
| accept response | "Accept the claim" | button | tab then enter/space | returns to board/log | responder only |
| challenge response | "Challenge the claim" | button | tab then enter/space | returns to board/log | responder only |
| score/counter summary | "Seat score and challenge counters" | region/list | read-only | no focus unless expandable | public |
| effect log | "Recent events" | list | optional focus for entries | newest event announced | viewer-filtered |
| replay controls | "Replay step" controls | button group | tab/arrow | focus remains in controls | public export only |

## Keyboard and focus plan

| Interaction | Keyboard path | Focus movement | Escape/cancel behavior | Test |
|---|---|---|---|---|
| choose claim mask | tab or arrow to own mask, enter | focus moves to grade choices | escape clears selection | smoke |
| choose declared grade | arrow/tab to grade, enter | focus moves to submit/result | escape returns to mask choices | smoke |
| accept/challenge | tab to response button, enter | focus moves to event/result | escape does nothing unless no modal | smoke |
| replay controls | tab to control, enter | focus remains on control | escape exits expanded replay panel | smoke |
| bot explanation/help | tab to details button, enter | focus enters details | escape closes details | a11y smoke |

## Screen-reader summaries where practical

| Summary | Trigger | Contents | Must not contain | Test/notes |
|---|---|---|---|---|
| current position/state | on load/action/replay step | turn, claimant/responder, declared grade, scores, public galleries | hidden tile IDs | smoke |
| legal actions | on actor turn | count and labels of legal choices | hidden reasons | smoke |
| action result | after effect settle | claim/window/accept/challenge/reveal/score result | unrevealed IDs | smoke |
| bot explanation | after bot action | policy id and viewer-safe rationale | hidden facts | smoke |

## Contrast and color/shape notes

| Item | Color use | Non-color cue | Contrast concern | Test/review |
|---|---|---|---|---|
| legal choice | accent color | button label and outline | meet WCAG AA | a11y smoke |
| selected item | stronger border | selected text and icon | visible focus ring | a11y smoke |
| player identity | two colors | seat label and position | color-independent text | manual/a11y |
| warnings/errors | alert color | text and icon | status contrast | a11y smoke |

Do not rely on color alone.

## Reduced-motion behavior

| Motion/effect | Default behavior | Reduced-motion behavior | Information preserved? | Test |
|---|---|---|---:|---|
| claim to pedestal | short movement/fade | instant update plus log highlight | yes | smoke |
| accept to veiled gallery | short movement/fade | instant count/grade update | yes | smoke |
| challenge reveal | flip/reveal emphasis | instant reveal plus text announcement | yes | smoke |
| score increment | brief count highlight | static changed score highlight | yes | smoke |

## Responsive behavior

| UI region | Desktop behavior | Small-screen behavior | Minimum usable state | Test |
|---|---|---|---|---|
| board/table | public pedestal, galleries, exposed rows, scores visible together | stacked public sections | declared grade and current responder visible | smoke |
| controls/action panel | side or lower rail with legal buttons | sticky lower panel | legal actions fit without overlap | smoke |
| log/replay | side rail | collapsible below board | latest event visible | smoke |
| bot explanation/help | inline details | collapsible details | rationale readable | smoke |
