# Event Frontier UI

Game ID: `event_frontier`

Implemented variants: `event_frontier_standard`, `event_frontier_hard_winter`,
and `event_frontier_land_rush`

Rules version: `event-frontier-rules-v1`

Renderer assumptions version: `event-frontier-ui-v1`

Prepared by: `Codex`

Last updated: 2026-06-12

## Purpose

This document defines the viewer-facing presentation contract for Event
Frontier. TypeScript never decides legality. Rust/WASM owns action trees,
validation, effects, public views, replay/export redaction, bot decisions, and
terminal rationale. The browser maps those safe payloads to controls and text.

## Product and visual target

| Field | Decision |
|---|---|
| public role | Gate 14 original portfolio game and hidden-order proof |
| desired feel | readable strategy board with a public event-card rail |
| visual risk to avoid | debug-console-first, proprietary mimicry, cluttered dashboard |
| public onboarding need | light |
| help/learning mode need | light |

## Renderer assumptions

| Assumption | Value | Evidence/notes |
|---|---|---|
| default renderer | React + SVG/HTML | Six sites, cards, counters, panels, and logs fit the existing shell. |
| expected object count | low to medium | Six sites, eight trails, two factions, one current card, one next card. |
| animation pressure | medium | Effects need clear feedback but no physics renderer. |
| SVG pressure expected? | yes | Graph map and trails are natural SVG/HTML overlays. |
| Canvas/PixiJS needed? | no | Current object count is modest. |
| WASM boundary | batched Rust calls | Legal choices and views come from existing WASM APIs. |

## Screen-size support

| Size/context | Supported? | Layout notes | Smoke test |
|---|---:|---|---|
| small phone portrait | yes | Stack card rail, map, controls, and log vertically. | ticket 018 browser smoke |
| phone landscape | yes | Keep controls below or beside the compact map. | ticket 018 browser smoke |
| tablet | yes | Two-column map/control layout. | ticket 018 browser smoke |
| desktop | yes | Map, event rail, action panel, and log visible without horizontal scroll. | ticket 018 browser smoke |
| keyboard-only desktop | yes | Legal controls are buttons/menu choices with visible focus. | ticket 018 browser smoke |
| reduced-motion user | yes | Replace movement animations with highlights and text summaries. | ticket 018 browser smoke |

## Legal action mapping

| Rust action choice/tree node | Rule IDs | UI control/hit target | Normal mode behavior | Learning/debug behavior | Accessibility label | Notes |
|---|---|---|---|---|---|---|
| `event` | `EF-ACT-001` through `EF-ACT-004`, `EF-EVENT-*` | current-card action button | Enabled only when Rust exposes the leaf. | Show Rust-safe card summary and selected path. | Resolve current event | Does not expose future deck order. |
| `operation/<kind>/...` | `EF-ACT-005`, `EF-ACT-006`, `EF-OP-*` | operation segmented control plus site choices | Progressive choices mirror Rust tree. | Show path, cost metadata, and safe diagnostics. | Choose operation and sites | TypeScript does not compute cost or legality. |
| `limited_operation/<kind>/...` | `EF-ACT-003`, `EF-OP-003` | same controls, one-site limit | Available only after first operation. | Show limited badge from path context. | Choose limited operation | Exactly one site comes from Rust path leaves. |
| `pass` | `EF-SCORE-002` | pass button | Gives resource and preserves eligibility when legal. | Show Rust effect text after apply. | Pass and gain resource | Double pass discards the card. |
| empty tree | `EF-ACT-007`, `EF-ACT-008`, `EF-END-005` | no gameplay controls | Waiting, Reckoning automation, or terminal. | Safe phase/status text only. | Waiting for Rust state | No disabled hidden-reason controls. |

## Progressive construction flow

| Stage | Rust-owned input/output | UI presentation | Preview needed? | Confirmation needed? | Notes |
|---|---|---|---:|---:|---|
| 1 | top-level legal choices | event/operation/pass command group | no | no | Choices are absent if Rust omits them. |
| 2 | operation kind branches | Charter or Freeholder operation menu | no | no | Branch labels are display only. |
| 3 | site and sub-choice leaves | site chips/highlights on public map | no | optional | Cost and legality come from Rust metadata/path. |
| 4 | validated action path | submit selected legal leaf | no | yes for multi-site operations | Invalid/stale diagnostics come from Rust. |

## Rust-generated previews

| Preview | Trigger | Viewer-safe contents | Must not contain | Tests |
|---|---|---|---|---|
| legal-tree metadata | current action tree | labels, public metadata, safe disabled/waiting reason | hidden deck tail, guessed future effects | WASM smoke, ticket 018 browser smoke |
| effect feedback | after apply/bot/replay step | viewer-filtered semantic effect text | hidden order or nonpublic diagnostics | smoke effects, no-leak smoke |

## Presentation metadata

Event card faces are projected by Rust as viewer-safe component display
metadata: `id`, player-facing label, one-line summary, family tag, and
accessibility label. The authored source is
`games/event_frontier/data/cards_presentation.toml`, validated by the typed
Rust UI loader before exposure.

The web renderer presents current, next, resolved, and discarded public cards
through `DeckFlowPanel`; the face-down remainder uses Rust/static-supplied deck
copy and never exposes hidden order. Compound Event Frontier action trees use
`ActionPathBuilder` for staged Event/Operation/Pass selection, Back/Cancel, and
leaf confirmation while submitting Rust action paths unchanged.

## Semantic effect-to-animation mapping

| Semantic effect | Visual animation | Timing/priority | Reduced-motion replacement | Settle-to-view check | Rule IDs |
|---|---|---|---|---|---|
| `CardRevealed` | update current/next card rail | before next action controls | instant text/card change | latest public view | `EF-VIS-003` |
| `ChoiceTaken` | pulse acting faction and choice | before consequences | text summary | latest public view | `EF-TURN-*` |
| `EventResolved` / `EdictActivated` / `EdictExpired` | card/effect banner | high | text summary | active-edict panel matches view | `EF-EVENT-*`, `EF-EDICT-*` |
| operation component effects | site/counter highlights | medium | highlight plus log entry | map counters match view | `EF-OP-*` |
| `ResourcesChanged` | resource counter update | medium | instant counter and log | resource pools match view | `EF-SCORE-*` |
| `ReckoningResolved` | Reckoning breakdown panel | high | static panel | scores and edicts match view | `EF-SCORE-*` |
| `Terminal` | outcome panel | highest | static result | terminal controls absent | `EF-END-*` |

## Replay UI

| Feature | Required? | Viewer-safe requirement | Tests/notes |
|---|---:|---|---|
| replay step forward/back | yes | public view/effects only | ticket 017/018 replay ergonomics |
| effect log display | yes | viewer-filtered effects only | no hidden deck tail |
| command log display | yes | redacted command summaries only | public export mirrors this |
| hash/version display | dev only | safe metadata only | no internal full trace in browser |
| local replay import/export | yes | public export redacts undrawn order | `replay-export-import-no-deck-leak.trace.json` |
| bot-vs-bot replay | yes | public explanations only | bot evidence pack |

## Bot explanation UI

| Surface | Public mode | Dev mode | Hidden-info safeguard | Tests |
|---|---|---|---|---|
| recent bot action | concise public rationale | policy/version and timing may show | no hidden deck order | `tests/bots.rs` |
| why affordance | optional public reason | expanded public details | no future random outcomes | ticket 018 smoke |
| candidate ranking | not public by default | viewer-safe/redacted only | no forbidden facts | `tests/bots.rs` |
| known weakness | not needed in play surface | docs only | no private state | `AI.md` |

## Outcome / victory explanation

The outcome surface explains the actual Rust terminal result and nothing else.
It must cite the stable `RULES.md` rule IDs that decided the match. TypeScript
must render the Rust-projected terminal rationale and static template text only;
it must not compute the winner, victory type, tiebreak, score comparison, or
decisive cause.

### Terminal result variants

| Result variant | Rust source of truth | Player-facing summary | Rule IDs |
|---|---|---|---|
| Charter instant victory | `PublicView.terminal` / terminal effect with `victory_type=charter_instant` | Charter wins by holding enough majority sites at a Reckoning victory check. | `EF-END-001`, `EF-SCORE-004` |
| Freeholder instant victory | `PublicView.terminal` / terminal effect with `victory_type=freeholder_instant` | Freeholders win by meeting the public cache threshold at a Reckoning victory check. | `EF-END-002` |
| Both-met instant victory | `PublicView.terminal` / terminal effect with Freeholder winner and both public distances met | Freeholders win because both instant conditions were true at the same Reckoning. | `EF-END-003` |
| Final fallback score win | `PublicView.terminal` / terminal effect with `victory_type=final_fallback` | After the third Reckoning, the higher cumulative score wins. | `EF-END-004`, `EF-SCORE-006` |
| Final fallback tiebreak | `PublicView.terminal` / terminal effect with tied scores and Freeholder winner | Freeholders win tied final fallback scores. | `EF-END-004`, `EF-SCORE-006` |

### Decisive cause payload

| Cause variant | Rust payload field(s) | Static template key | Notes |
|---|---|---|---|
| Charter site-majority instant | terminal winner, victory type, final scores, decisive rule, public site-majority count | `event_frontier.charter_instant` | Viewer-safe because site presence is public. |
| Freeholder cache instant | terminal winner, victory type, final scores, decisive rule, public cache count and threshold | `event_frontier.freeholder_instant` | Viewer-safe because caches are public. |
| Both-met instant | terminal winner, victory type, decisive rule, both public victory distances | `event_frontier.both_met_freeholder` | Must not imply hidden card order. |
| Final fallback score comparison | terminal winner, victory type, final scores, decisive rule | `event_frontier.final_fallback_score` | TypeScript does not compare scores. |
| Final fallback tiebreak | terminal winner, victory type, tied final scores, decisive rule | `event_frontier.final_fallback_tiebreak` | TypeScript does not resolve the tiebreak. |

### Per-player final breakdown

| Breakdown value | Source | Visible to public observer? | Visible to seat viewer? | Hidden-info notes |
|---|---|---:|---:|---|
| winner faction | Rust terminal view/effect | yes | yes | public terminal fact |
| victory type | Rust terminal view/effect | yes | yes | public terminal fact |
| Charter cumulative score | Rust public view/terminal rationale | yes | yes | public score |
| Freeholder cumulative score | Rust public view/terminal rationale | yes | yes | public score |
| Charter majority-site count | Rust public victory-distance/terminal rationale | yes | yes | public site state |
| Freeholder cache count and threshold | Rust public victory-distance/terminal rationale | yes | yes | public cache state |
| decisive rule ID | Rust terminal rationale or static mapping for Rust cause | yes | yes | references `RULES.md` only |

### No-leak rules

- Visible text: must not reveal undrawn deck order beyond the next public card.
- Hidden DOM/accessibility attributes: must not include hidden card IDs, deck
  tail values, or seed-derived future order.
- `data-testid`/selectors: must use stable generic labels, never hidden card IDs.
- Storage/logs/dev panel: must not persist internal full trace or deck tail.
- Effect log/replay export: must show only viewer-filtered public effects and
  redacted command summaries.
- Bot explanations/candidate rankings: must cite public view facts only.

Terminal state does not reveal the rest of the deck. The outcome surface may
show current, next, discarded, and resolved card facts only if they are already
public in the Rust projection.

### Player-facing copy contract

The outcome copy explains only the actual result: winner, victory type,
decisive public facts, final public scores, and rule IDs. It must not include
strategy advice, counterfactuals, turning-point analysis, or hidden-order hints.

### Accessibility and reduced motion

The terminal summary is a status/result message. The decisive cause is text,
not color-only or animation-only. Player standing is color-independent.
Expanded breakdowns are keyboard accessible. Reduced-motion mode preserves all
facts. Replay terminal renders the same outcome content for the same viewer.

### Smoke and tests

| Test case | Terminal path | Required assertion |
|---|---|---|
| Charter instant | `standard-charter-instant-win.trace.json` | summary names Charter, site-majority cause, and `EF-END-001`; no hidden deck order |
| Freeholder cache instant | `standard-freeholder-cache-win.trace.json` | summary names Freeholders, cache-threshold cause, and `EF-END-002`; no hidden deck order |
| Final fallback/tiebreak | `final-reckoning-fallback.trace.json` | summary names fallback score/tiebreak cause and `EF-END-004`; no hidden deck order |
| Browser smoke | ticket 018 Event Frontier smoke | outcome panel renders after terminal and no-leak assertions pass |

## Dev inspector boundary

| Inspector item | Public build allowed? | Dev build allowed? | Must not contain | Tests |
|---|---:|---:|---|---|
| seed/rules/data version | yes | yes | hidden state | smoke |
| public view inspector | no by default | yes | undrawn deck order | no-leak smoke |
| action tree inspector | no by default | yes if viewer-safe | hidden reasons/state | no-leak smoke |
| effect log | yes | yes if viewer-filtered | hidden outcomes | smoke effects |
| command log | no by default | yes if redacted | private/internal data | replay smoke |
| bot timing | no by default | yes | hidden facts | bot tests |
| candidate ranking | no | yes if redacted | hidden facts | bot tests |
| full internal state | no | test harness only | all hidden state | not public |

## Accessibility labels and semantics

| Element/control | Accessible name/description | Role/semantic element | Keyboard path | Focus behavior | Notes |
|---|---|---|---|---|---|
| current event card | current public event and ops value | region/button group | tab to action controls | focus remains on selected action | next card is public separately |
| site control | site name and public component counts | button/list item | tab/arrow within map group | visible outline | only legal site leaves are actionable |
| operation selector | operation kind and legal site count | button group/menu | tab and activate | moves to site choices | no TypeScript legality |
| pass/event buttons | legal action labels | button | tab and activate | returns to status after apply | absent when illegal |
| outcome panel | winner and decisive cause | status/region | tab to breakdown | focusable breakdown toggle | no hidden order |

## Reduced-motion behavior

| Motion/effect | Default behavior | Reduced-motion behavior | Information preserved? | Test |
|---|---|---|---:|---|
| card reveal | card rail transition | instant update plus text log | yes | ticket 018 |
| site component change | highlight/pulse site | static highlight and log | yes | ticket 018 |
| Reckoning | panel transition | static panel | yes | ticket 018 |
| terminal outcome | outcome panel emphasis | static result | yes | ticket 018 |

## Responsive behavior

| UI region | Desktop behavior | Small-screen behavior | Minimum usable state | Test |
|---|---|---|---|---|
| map | central graph with site counters | compact stacked graph/list | all site facts readable | ticket 018 |
| event rail | current/next/discard summary near map | above controls | current and next visible | ticket 018 |
| controls | side panel | stacked below map | legal leaves reachable | ticket 018 |
| log/replay | side/bottom panel | collapsible below controls | latest effect visible | ticket 018 |
| bot explanation | compact note | below controls/log | public reason visible | ticket 018 |
