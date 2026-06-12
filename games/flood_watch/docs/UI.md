# flood_watch UI

Game ID: `flood_watch`

Implemented variant: `flood_watch_standard`

Rules version: `flood-watch-rules-v1`

Renderer assumptions version: `flood-watch-ui-v1`

Prepared by: Codex

Last updated: 2026-06-11

## Purpose

This document defines the product-facing UI plan for Flood Watch. TypeScript presents Rust/WASM output only. Rust owns legality, public projection, effects, diagnostics, replay/export, and bot decisions.

## Product and visual target

| Field | Decision |
|---|---|
| public role | hidden-info cooperative proof and original portfolio game |
| desired feel | clear river-town emergency board, not a debug console |
| visual risk to avoid | proprietary cooperative-game mimicry, clutter, hidden deck leakage |
| public onboarding need | light |
| help/learning mode need | light |

## Renderer assumptions

| Assumption | Value | Evidence/notes |
|---|---|---|
| default renderer | React + SVG/HTML | Five districts and simple counters do not need canvas. |
| expected object count | low | Five district panels, deck/forecast, action bar, effect log. |
| animation pressure | medium | Event draw, flood rise, levee absorb, terminal. |
| SVG pressure expected? | yes | District map or schematic can be SVG. |
| Canvas/PixiJS needed? | no by default | No high object count. |
| WASM boundary | batched Rust calls | UI calls legal tree/apply/view/effects. |

## Screen-size support

| Size/context | Supported? | Layout notes | Smoke test |
|---|---:|---|---|
| small phone portrait | yes | stacked districts and action bar | GAT12FLOWATCOO-017 |
| phone landscape | yes | compact district grid | GAT12FLOWATCOO-017 |
| tablet | yes | district grid plus side log | GAT12FLOWATCOO-017 |
| desktop | yes | full board with log and bot panel | GAT12FLOWATCOO-017 |
| keyboard-only desktop | yes | actions are buttons from Rust legal tree | GAT12FLOWATCOO-017 |
| reduced-motion user | yes | event text summaries replace motion | GAT12FLOWATCOO-017 |

## Legal action mapping

| Rust action choice/tree node | Rule IDs | UI control/hit target | Normal mode behavior | Learning/debug behavior | Accessibility label | Notes |
|---|---|---|---|---|---|---|
| `bail/<district>` | `FW-ACT-002` | district action button | enabled only if Rust legal | may show public diagnostic | `Bail <district>` | Removes flood by role amount. |
| `reinforce/<district>` | `FW-ACT-003` | district action button | enabled only if Rust legal | may show public diagnostic | `Reinforce <district>` | Adds levees by role amount. |
| `forecast` | `FW-ACT-004` | forecast button | enabled only if Rust legal | may show public diagnostic | `Forecast next event` | Public reveal. |
| `end_turn` | `FW-ACT-005` | end-turn button | enabled only if Rust legal | may show public diagnostic | `End turn` | Starts environment phase. |

## Progressive construction flow

Not applicable. Flood Watch uses flat action paths.

## Rust-generated previews

| Preview | Trigger | Viewer-safe contents | Must not contain | Tests |
|---|---|---|---|---|
| action metadata | legal tree request | action labels, target district, public phase/budget | undrawn deck order | `cargo test -p flood_watch --test visibility` |
| terminal view | terminal projection | shared outcome, public reason, public district levels | undrawn event identities | `public-replay-export-import.trace.json` |

## Presentation metadata

Flood Watch event card faces are projected by Rust as viewer-safe component
display metadata: `id`, player-facing label, one-line summary, family tag, and
accessibility label. The authored source is
`games/flood_watch/data/cards_presentation.toml`, validated by the typed Rust
UI loader before exposure.

The web renderer presents forecast, drawn, discard, and face-down deck state
through `DeckFlowPanel`. Its public remainder badge uses Rust-projected
`undrawn_count`; the panel never names unrevealed event cards or reconstructs
hidden order.

## Semantic effect-to-animation mapping

| Semantic effect | Visual animation | Timing/priority | Reduced-motion replacement | Settle-to-view check | Rule IDs |
|---|---|---|---|---|---|
| `DistrictBailed` | flood gauge decreases | immediate | text summary | latest public view | `FW-ACT-002` |
| `LeveePlaced` | levee counter increases | immediate | text summary | latest public view | `FW-ACT-003` |
| `ForecastRevealed` | forecast card appears | before next draw | text summary | forecast field | `FW-ACT-004` |
| `EnvironmentPhaseBegan` | environment banner | before draws | text summary | effect log | `FW-ENV-001` |
| `EventDrawn` | drawn event appears | before consequences | text summary | drawn history | `FW-ENV-002` |
| `LeveeAbsorbed` | levee counter decreases | before rise | text summary | latest public view | `FW-ENV-003` |
| `FloodLevelRose` | flood gauge rises | after absorption | text summary | latest public view | `FW-ENV-003` |
| `DistrictInundated` | district danger state | before terminal | text summary | terminal view | `FW-ENV-006` |
| `DeckExhausted` | deck empty state | before win terminal | text summary | terminal view | `FW-ENV-007` |
| `Terminal` | outcome banner | final | text result | terminal view | `FW-END-*` |

## Replay UI

| Feature | Required? | Viewer-safe requirement | Tests/notes |
|---|---:|---|---|
| replay step forward/back | yes | public export only | GAT12FLOWATCOO-014/017 |
| effect log display | yes | viewer-filtered effects only | no hidden deck order |
| command log display | no by default | if shown, action paths only | no seed/deck reconstruction |
| local replay import/export | yes | redacted viewer-scoped export | `export_public_replay` |
| bot-vs-bot replay | yes | public-safe explanations | Level 1 rationale only |

## Bot explanation UI

| Surface | Public mode | Dev mode | Hidden-info safeguard | Tests |
|---|---|---|---|---|
| recent bot action | concise reason | policy id/version | no hidden deck facts | `tests/bots.rs` |
| why affordance | optional short reason | expanded public rationale | no candidate hidden facts | `BOT-STRATEGY-EVIDENCE-PACK.md` |
| candidate ranking | not public by default | public facts only | no hidden order | no-leak tests |

## Outcome / victory explanation

Describe the end-of-match explanation shown by the shared web outcome surface.

### Terminal result variants

| Result variant | Rust source of truth | Player-facing summary | Rule IDs |
|---|---|---|---|
| shared loss | `TerminalView::Complete`, `TerminalSummary` | The team lost because a named district reached flood level 3. | `FW-END-001`, `FW-ENV-006` |
| shared win | `TerminalView::Complete`, `TerminalSummary` | The team won because the final event resolved with all districts below level 3. | `FW-END-002`, `FW-ENV-007` |

### Decisive cause payload

| Cause variant | Rust payload field(s) | Static template key | Notes |
|---|---|---|---|
| district inundated | `terminal.summary.rule_id`, `terminal.summary.public_summary`, surviving levels | `flood_watch.shared_loss_inundation` | Public district and level facts only. |
| deck exhausted safely | `terminal.summary.rule_id`, `terminal.summary.public_summary`, surviving levels | `flood_watch.shared_win_deck_exhausted` | Does not reveal undrawn order. |

TypeScript MUST NOT compute these cause variants. It renders the Rust-projected value only.

### Per-player final breakdown

| Breakdown value | Source | Visible to public observer? | Visible to seat viewer? | Hidden-info notes |
|---|---|---:|---:|---|
| shared outcome | `TerminalView` | yes | yes | no seat winner |
| drawn card count | `TerminalSummary.drawn_card_count` | yes | yes | count only |
| surviving district levels | `TerminalSummary.surviving_levels` | yes | yes | public levels |

### No-leak rules

- Visible text: never names undrawn event cards.
- Hidden DOM/accessibility attributes: no full deck order or unrevealed card ids.
- `data-testid`/selectors: no encoded hidden card ids.
- Storage/logs/dev panel: no internal event deck.
- Effect log/replay export: drawn/forecast cards only; no future order.
- Bot explanations/candidate rankings: public levels, levees, forecast, and composition counts only.

### Player-facing copy contract

The outcome surface explains only the actual result. It must not include coaching, counterfactuals, turning-point analysis, or strategy advice.

### Accessibility and reduced motion

Terminal summary is exposed as text. Decisive cause is not color-only. Shared standing is color-independent. Reduced-motion mode preserves the same facts.

### Smoke and tests

| Test case | Terminal path | Required assertion |
|---|---|---|
| shared loss | `loss-by-inundation.trace.json` | names public district; no hidden deck |
| shared win | `standard-win.trace.json` | says shared win; no hidden deck |
| public export | `public-replay-export-import.trace.json` | replay export is redacted |

## Dev inspector boundary

Dev tools are secondary. Public mode is play first, not a diagnostic harness. Full internal event deck order is never allowed in public builds.

## Accessibility labels and semantics

| Element/control | Accessible name/description | Role/semantic element | Keyboard path | Focus behavior | Notes |
|---|---|---|---|---|---|
| district panel | district name, flood level, levee count | group | tab into actions | remains stable after action | no hidden data |
| bail button | `Bail <district>` | button | tab/enter | returns to district | Rust legal only |
| reinforce button | `Reinforce <district>` | button | tab/enter | returns to district | Rust legal only |
| forecast button | `Forecast next event` | button | tab/enter | focus stays in action bar | public reveal |
| end-turn button | `End turn` | button | tab/enter | moves to effect log | starts environment |

## Keyboard and focus plan

| Interaction | Keyboard path | Focus movement | Escape/cancel behavior | Test |
|---|---|---|---|---|
| choose legal action | tab/enter | focus returns to updated legal actions | not applicable | GAT12FLOWATCOO-017 |
| replay controls | buttons | focus remains in replay controls | not applicable | GAT12FLOWATCOO-017 |
| bot explanation/help | button/details | expanded text receives focus | close collapses | GAT12FLOWATCOO-017 |
