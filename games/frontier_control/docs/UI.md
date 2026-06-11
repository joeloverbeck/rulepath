# frontier_control UI

Game ID: `frontier_control`

Implemented variant: `frontier_control_standard`, `frontier_control_highlands`

Rules version: `frontier-control-rules-v1`

Renderer assumptions version: `frontier-control-ui-v1`

Prepared by: Codex

Last updated: 2026-06-11

## Purpose

This document defines the product-facing UI plan for Frontier Control. TypeScript
presents Rust/WASM output only. Rust owns legal action trees, validation,
adjacency, supply connectivity, clash resolution, scoring, terminal rationale,
effects, replay/export, and bot decisions.

## Product and visual target

| Field | Decision |
|---|---|
| public role | original graph-map and faction-asymmetry proof |
| desired feel | readable frontier trail map with warm board-game table presentation |
| visual risk to avoid | proprietary area-control trade dress, debug-console-first layout, clutter |
| public onboarding need | light to moderate |
| help/learning mode need | light |

## Renderer assumptions

| Assumption | Value | Evidence/notes |
|---|---|---|
| default renderer | React + SVG/HTML | Seven sites, ten trails, and small counters do not need canvas. |
| expected object count | low | Sites, trails, faction units, stake markers, score/budget panels, effect log. |
| animation pressure | medium | Movement, clash, scoring, and terminal feedback need ordered effect presentation. |
| SVG pressure expected? | yes | Graph map is naturally SVG. |
| Canvas/PixiJS needed? | no by default | No high object count or heavy particle animation. |
| WASM boundary | batched Rust calls | UI calls setup/action tree/apply/view/effects/bot/replay APIs. |

## Screen-size support

| Size/context | Supported? | Layout notes | Smoke test |
|---|---:|---|---|
| small phone portrait | yes | map stacks above action panel and score summary | GAT13FROCONASY-015/016 |
| phone landscape | yes | compact map with collapsible log | GAT13FROCONASY-015/016 |
| tablet | yes | map plus side action/log column | GAT13FROCONASY-015/016 |
| desktop | yes | full map, faction panels, score track, effect log | GAT13FROCONASY-015/016 |
| keyboard-only desktop | yes | actions are semantic buttons from Rust legal tree | GAT13FROCONASY-015/016 |
| reduced-motion user | yes | movement and clash animations become highlights/text summaries | GAT13FROCONASY-015/016 |

## Legal action mapping

| Rust action choice/tree node | Rule IDs | UI control/hit target | Normal mode behavior | Learning/debug behavior | Accessibility label | Notes |
|---|---|---|---|---|---|---|
| `march/<from>/<to>` | `FC-ACT-002`, `FC-CTRL-001`, `FC-CTRL-002` | Prospector action button or site-source/target affordance | enabled only when Rust returns the leaf | may show Rust diagnostic after rejected submit | `March crew from <from> to <to>` | UI must not compute adjacency. |
| `stake/<site>` | `FC-ACT-003`, `FC-SCORE-PROSPECTOR-SUPPLY` | site stake button | enabled only when Rust returns the leaf | may show public stake diagnostic | `Place stake at <site>` | Supply status comes from Rust view. |
| `muster` | `FC-ACT-004` | Prospector action button | enabled only when Rust returns the leaf | public diagnostic only | `Muster a crew at Base Camp` | Adds a crew at Base Camp. |
| `patrol/<from>/<to>` | `FC-ACT-006`, `FC-CTRL-001`, `FC-CTRL-003` | Garrison action button or site-source/target affordance | enabled only when Rust returns the leaf | may show Rust diagnostic after rejected submit | `Patrol guard from <from> to <to>` | UI must not compute clash results. |
| `reinforce/<fort>` | `FC-ACT-007` | fort reinforce button | enabled only when Rust returns the leaf | public diagnostic only | `Reinforce <fort>` | Fort legality is Rust-owned. |
| `dismantle/<site>` | `FC-ACT-008` | stake/site button | enabled only when Rust returns the leaf | public diagnostic only | `Dismantle stake at <site>` | Requires a guard, but UI does not decide that. |
| `end_turn` | `FC-ACT-009` | end-turn button | enabled only when Rust returns the leaf | public diagnostic only | `End turn` | Prevents stalls. |
| waiting/observer tree | `FC-ACT-010` | waiting state text and disabled action region | no gameplay buttons | safe active-faction metadata | `Waiting for <faction>` | No hidden state exists. |
| terminal tree | `FC-ACT-011`, `FC-TERM-NO-ACTIONS` | outcome panel | no gameplay buttons | terminal metadata only | `Match complete` | Outcome surface explains result. |

## Progressive construction flow

Not applicable for Gate 13. Frontier Control uses flat action paths. The UI may
group move actions by source site for readability, but every final action path
comes from Rust.

## Rust-generated previews

| Preview | Trigger | Viewer-safe contents | Must not contain | Tests |
|---|---|---|---|---|
| legal tree metadata | action tree request | legal labels, action paths, public budget/phase metadata | TypeScript-guessed adjacency, supply, clash, or score consequences | GAT13FROCONASY-015/016 |
| terminal view | terminal projection | winner, scores, tiebreak flag, summary | hidden state or counterfactual coaching | GAT13FROCONASY-015/016 |

## Semantic effect-to-animation mapping

| Semantic effect | Visual animation | Timing/priority | Reduced-motion replacement | Settle-to-view check | Rule IDs |
|---|---|---|---|---|---|
| `CrewMarched` | crew marker moves along trail | before clash | text summary and destination highlight | latest public view | `FC-ACT-002`, `FC-CTRL-001` |
| `GuardPatrolled` | guard marker moves along trail | before clash | text summary and destination highlight | latest public view | `FC-ACT-006`, `FC-CTRL-001` |
| `ClashResolved` | contested site flashes with removed-unit summary | after movement | text summary | latest public view | `FC-CTRL-002`, `FC-CTRL-003` |
| `StakePlaced` | stake marker appears | immediate | text summary | latest public view | `FC-ACT-003` |
| `StakeDismantled` | stake marker disappears | immediate | text summary | latest public view | `FC-ACT-008` |
| `CrewMustered` | Base Camp crew count increases | immediate | text summary | latest public view | `FC-ACT-004` |
| `GuardReinforced` | fort guard count increases | immediate | text summary | latest public view | `FC-ACT-007` |
| `TurnEnded` | faction turn banner changes | before next phase | text summary | latest public view | `FC-TURN-005` |
| `RoundScored` | score counters increment and supplied/cut stakes highlight | after Garrison turn | scoring text summary | score and supplied fields | `FC-TURN-006`, `FC-SCORE-*` |
| `Terminal` | outcome banner appears | final | text result | terminal view | `FC-TERM-*` |

## Replay UI

| Feature | Required? | Viewer-safe requirement | Tests/notes |
|---|---:|---|---|
| replay step forward/back | yes | one public projection for all viewers | GAT13FROCONASY-015/016 |
| effect log display | yes | public effects only | all effects are public |
| command log display | no by default | if shown, action paths only | no internal state dump |
| hash/version display | yes in dev | safe metadata only | no hidden state exists |
| local replay import/export | yes | public command/effect timeline | `replay-export-import.trace.json` |
| bot-vs-bot replay | yes | public bot rationales only | `bot-vs-bot-full-game.trace.json` |

## Bot explanation UI

| Surface | Public mode | Dev mode | Hidden-info safeguard | Tests |
|---|---|---|---|---|
| recent bot action | concise faction reason | policy id/version and timing may show | public facts only | `tests/bots.rs` |
| why affordance | optional short reason | expanded viewer-safe rationale | no hidden facts exist | `BOT-STRATEGY-EVIDENCE-PACK.md` |
| candidate ranking | not public by default | public facts only | no full internal state dump | future E2E/dev smoke |
| known weakness | not in normal play | docs may name retune note | no private state | `AI.md` |

## Outcome / victory explanation

The shared web outcome surface renders the Rust terminal view and terminal
effect only. TypeScript must not recompute score comparison, supply, fort
control, or tiebreaks.

### Terminal result variants

| Result variant | Rust source of truth | Player-facing summary | Rule IDs |
|---|---|---|---|
| Garrison score win | `TerminalView::Winner`, `FrontierControlEffect::Terminal` | The Garrison wins because its final score is higher after the last scheduled scoring round. | `FC-TERM-SCORE-COMPARE`, `FC-SCORE-GARRISON-FORT`, `FC-SCORE-COMPARABLE-TRACK` |
| Prospector score win | `TerminalView::Winner`, `FrontierControlEffect::Terminal` | The Prospectors win because supplied stakes produce the higher final score. | `FC-TERM-SCORE-COMPARE`, `FC-SCORE-PROSPECTOR-SUPPLY`, `FC-SCORE-STAKE-VALUE`, `FC-SCORE-COMPARABLE-TRACK` |
| Garrison tiebreak win | `TerminalView::Winner`, `FrontierControlEffect::Terminal` | The final scores are tied, so the Garrison wins the incumbent tiebreak. | `FC-TERM-GARRISON-TIEBREAK`, `FC-SCORE-COMPARABLE-TRACK` |

### Decisive cause payload

| Cause variant | Rust payload field(s) | Static template key | Notes |
|---|---|---|---|
| score comparison | `terminal.faction`, `terminal.scores`, `terminal.summary`, terminal effect totals | `frontier_control.score_compare` | Viewer-safe public totals only. |
| Garrison tiebreak | `terminal.garrison_tiebreak`, `terminal.scores`, `terminal.summary` | `frontier_control.garrison_tiebreak` | TypeScript renders the Rust flag only. |

### Per-player final breakdown

| Breakdown value | Source | Visible to public observer? | Visible to seat viewer? | Hidden-info notes |
|---|---|---:|---:|---|
| winner faction | `TerminalView::Winner.faction` | yes | yes | no hidden state |
| Garrison final score | `TerminalView::Winner.scores.garrison` | yes | yes | public score |
| Prospector final score | `TerminalView::Winner.scores.prospectors` | yes | yes | public score |
| Garrison tiebreak applied | `TerminalView::Winner.garrison_tiebreak` | yes | yes | public rule outcome |
| terminal summary | `TerminalView::Winner.summary` and terminal effect `summary` | yes | yes | Rust-authored public text |

### No-leak rules

- Visible text: no hidden state exists; text still must not include internal debug dumps.
- Hidden DOM/accessibility attributes: no internal state hashes, command freshness internals, or dev-only fields in public DOM.
- `data-testid`/selectors: no encoded command stream or internal state.
- Storage/logs/dev panel: public replay export only; no full internal state in public build.
- Effect log/replay export: public command/effect timeline only.
- Bot explanations/candidate rankings: public facts from the projected view and legal tree only.

### Player-facing copy contract

The outcome surface explains only the actual result. It must not include
coaching, counterfactuals, turning-point analysis, or strategy advice.

### Accessibility and reduced motion

Terminal summary is exposed as text. Decisive cause is not color-only or
animation-only. Faction standing uses labels plus color/shape. Reduced-motion
mode preserves winner, scores, and tiebreak facts.

### Smoke and tests

| Test case | Terminal path | Required assertion |
|---|---|---|
| Garrison score win | `standard-garrison-win.trace.json` or E2E scripted path | summary names Garrison and scores; no TypeScript recomputation |
| Prospector score win | `standard-prospector-win.trace.json` or E2E scripted path | summary names Prospectors and supplied-stake result |
| Garrison tiebreak | `tie-garrison-tiebreak.trace.json` | summary names the Garrison tiebreak |

## Dev inspector boundary

Dev tools are secondary. Public mode is play first, not a diagnostic harness.
Full internal state is test-harness-only; public dev panels may show only the
viewer-safe public view, public effects, legal action paths, versions, and
policy metadata.

## Accessibility labels and semantics

| Element/control | Accessible name/description | Role/semantic element | Keyboard path | Focus behavior | Notes |
|---|---|---|---|---|---|
| site node | site label, guards, crews, fort/stake/supplied state | SVG title/desc plus list item | tab to related action buttons | remains stable after action | no hidden data |
| trail | connection between two site labels | SVG desc | non-interactive | none | movement legality comes from buttons |
| faction panel | faction label, seat, score, active/waiting state | group | tab through actions | active panel first | public facts |
| action button | Rust-provided action label | button | tab/enter/space | focus returns to updated legal choices | no illegal raw command editing |
| score summary | current scores and round | status/group | readable text | updates after scoring | not color-only |
| effect log | ordered public effects | log | tab into entries | appends after effects | viewer-safe |
| outcome panel | winner, scores, cause | status/result | focus on terminal | remains keyboard accessible | no coaching copy |

## Keyboard and focus plan

| Interaction | Keyboard path | Focus movement | Escape/cancel behavior | Test |
|---|---|---|---|---|
| choose legal action | tab to Rust legal button, enter/space | focus returns to refreshed action panel | not applicable for flat actions | GAT13FROCONASY-015/016 |
| inspect site | tab or arrow through site summary list | focus stays on site/action region | escape returns to action panel if a popover exists | GAT13FROCONASY-015/016 |
| replay controls | buttons | focus remains in replay controls | escape closes replay detail panel | GAT13FROCONASY-015/016 |
| bot explanation/help | button/details | expanded text receives focus | close collapses | GAT13FROCONASY-015/016 |

## Responsive behavior

| UI region | Desktop behavior | Small-screen behavior | Minimum usable state | Test |
|---|---|---|---|---|
| graph map | centered SVG with site labels and trails | scales above controls with readable labels | all sites and trails visible | E2E screenshot review |
| controls/action panel | side panel grouped by faction/action type | below map, active faction first | legal actions remain reachable | E2E smoke |
| score/log | side column | collapsible below controls | scores and latest effect visible | E2E smoke |
| bot explanation/help | compact panel | disclosure under latest action | explanation text readable | E2E smoke |
