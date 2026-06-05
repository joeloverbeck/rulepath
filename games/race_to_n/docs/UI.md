# race_to_n UI

Game ID: `race_to_n`

Implemented variant: `race_to_21`

Rules version: `1`

Renderer assumptions version: `1`

Prepared by: `Codex`

Last updated: 2026-06-05

## Purpose

This document defines the Gate 1 browser harness for `race_to_n`.

TypeScript never decides legality. Rust/WASM owns action trees, validation, state transitions, public views, semantic effects, diagnostics, and bot decisions. TypeScript maps Rust-provided choices to controls and renders viewer-safe payloads.

## Product and visual target

| Field | Decision |
|---|---|
| public role | UI smoke harness |
| desired feel | minimal abstract counter race |
| visual risk to avoid | debug-console-first, clutter, proprietary mimicry |
| public onboarding need | none for Gate 1 |
| help/learning mode need | none for Gate 1 |

## Renderer assumptions

| Assumption | Value | Evidence/notes |
|---|---|---|
| default renderer | React + CSS | The Gate 1 harness is a small control surface and counter track. |
| expected object count | under 20 visible elements | One counter track, three score fields, action buttons, effects list. |
| animation pressure | low | Track width transition only; state is authoritative from Rust view. |
| SVG pressure expected? | no | CSS track is sufficient. |
| Canvas/PixiJS needed? | no by default | No profiling evidence requires canvas. |
| WASM boundary | batched Rust calls; no chatty rule hot loops | UI calls `new_match`, `get_view`, `get_action_tree`, `apply_action`, `run_bot_turn`, and `get_effects`. |

## Screen-size support

| Size/context | Supported? | Layout notes | Smoke test |
|---|---:|---|---|
| small phone portrait | yes | Single-column score and two-column controls. | visual smoke/manual resize |
| phone landscape | yes | Same responsive grid; no hover dependency. | visual smoke/manual resize |
| tablet | yes | Desktop grid fits with larger touch targets. | visual smoke |
| desktop | yes | Constrained 980px harness. | `npm --prefix apps/web run smoke:ui` plus Playwright visual check |
| keyboard-only desktop | yes | Native buttons in tab order. | manual/browser smoke |
| reduced-motion user | yes | Motion is nonessential; track transition may be disabled later without information loss. | manual review |

## Action Mapping

| Rust action choice/tree node | Rule IDs | UI control/hit target | Normal mode behavior | Learning/debug behavior | Accessibility label | Notes |
|---|---|---|---|---|---|---|
| `add-1` | `R-TURN-001`, `R-ACT-001` | button from Rust action tree | submits `add-1` with freshness token | Rust diagnostic shown on stale token | Rust-provided label | Rendered only when present in the Rust tree. |
| `add-2` | `R-TURN-001`, `R-ACT-001` | button from Rust action tree | submits `add-2` with freshness token | Rust diagnostic shown on stale token | Rust-provided label | Rendered only when present in the Rust tree. |
| `add-3` | `R-TURN-001`, `R-ACT-001` | button from Rust action tree | submits `add-3` with freshness token | Rust diagnostic shown on stale token | Rust-provided label | Rendered only when present in the Rust tree. |

Choices not returned by Rust are absent from the UI. Hidden information is not applicable because `race_to_n` is perfect information.

## Progressive construction flow

Not applicable. `race_to_n` actions are flat one-segment paths.

## Rust-generated previews

Not applicable for Gate 1. The harness shows the latest public view and semantic effects after committed actions.

## Semantic effect-to-animation mapping

| Semantic effect | Visual animation | Timing/priority | Reduced-motion replacement | Settle-to-view check | Rule IDs |
|---|---|---|---|---|---|
| `ActionStarted` | Text row in effect log | immediate | same text row | Latest public view remains visible. | `R-TURN-001` |
| `CounterAdvanced` | Counter track width settles to Rust view | 180ms CSS transition | instant width update acceptable | Counter text and track match view. | `R-ACT-001` |
| `TurnChanged` | Turn field updates | immediate | same | Turn field matches view. | `R-TURN-002` |
| `GameEnded` | Turn field shows winner | immediate | same | Winner matches view. | `R-WIN-001` |
| `ActionCompleted` | Text row in effect log | immediate | same text row | Latest public view remains visible. | `R-TURN-001` |

## Settle-to-view checks

| Scenario | Required check | Test |
|---|---|---|
| after human action | counter and turn settle to Rust public view | UI smoke |
| after bot action | effect list and public view update from Rust | UI smoke |
| after stale diagnostic | diagnostic is shown and counter does not mutate | UI smoke |
| after reduced-motion path | no essential information is animation-only | manual review |

## Replay UI

Replay controls are out of scope for Gate 1. The harness displays the viewer-filtered effect log only.

## Bot explanation UI

| Surface | Public mode | Dev mode | Hidden-info safeguard | Tests |
|---|---|---|---|---|
| recent bot action | effect log and public view update | not implemented | Bot is random legal over public tree; no hidden state exists. | UI smoke |
| candidate ranking | not implemented | not implemented | no candidate UI exists | not applicable |

## Dev inspector boundary

| Inspector item | Public build allowed? | Dev build allowed? | Must not contain | Tests |
|---|---:|---:|---|---|
| seed/rules/data version | no | yes | hidden state | not implemented |
| public view inspector | no | yes | hidden state | `render_game_to_text` exposes only concise public state for smoke automation. |
| action tree inspector | no | yes if viewer-safe | hidden reasons/state | not implemented |
| effect log | yes | yes | hidden outcomes | UI smoke |
| full internal state | no | no | all hidden state | code review |

## Accessibility labels and semantics

| Element/control | Accessible name/description | Role/semantic element | Keyboard path | Focus behavior | Notes |
|---|---|---|---|---|---|
| Start Match | `Start Match` | button | Tab, Enter/Space | stays in normal document order | Starts a seeded Gate 1 match. |
| Add buttons | Rust-provided label | button | Tab, Enter/Space | native focus ring | Rendered from Rust action tree. |
| Submit Stale | `Submit Stale` | button | Tab, Enter/Space | native focus ring | Smoke-only diagnostic path. |
| Effects | `semantic effects` | ordered list | read-only | none | Viewer-filtered effect rows. |

## Keyboard and focus plan

| Interaction | Keyboard path | Focus movement | Escape/cancel behavior | Test |
|---|---|---|---|---|
| choose Rust action | Tab to button, Enter/Space | remains in document order | none | manual/browser smoke |
| restart | Tab to Restart, Enter/Space | new controls render from Rust tree | none | manual/browser smoke |
| stale diagnostic | Tab to Submit Stale, Enter/Space | diagnostic appears in status region | none | UI smoke |

## Screen-reader summaries where practical

| Summary | Trigger | Contents | Must not contain | Test/notes |
|---|---|---|---|---|
| current state | on render | counter, turn, freshness token | internal state | visible scoreboard |
| action result | after effect settle | viewer-safe semantic effect rows | hidden outcomes | UI smoke |
| diagnostic | stale submission | Rust diagnostic code and message | hidden state | UI smoke |

## Contrast and color/shape notes

| Item | Color use | Non-color cue | Contrast concern | Test/review |
|---|---|---|---|---|
| action choice | green/white button states | text label and border | sufficient on light background | visual review |
| counter progress | green fill | numeric counter text | fill is not sole state cue | visual review |
| warnings/errors | warm background | diagnostic text/code | sufficient on light background | visual review |

## Reduced-motion behavior

| Motion/effect | Default behavior | Reduced-motion behavior | Information preserved? | Test |
|---|---|---|---:|---|
| counter fill transition | 180ms width transition | instant width update acceptable | yes | manual review |

## Responsive behavior

| UI region | Desktop behavior | Small-screen behavior | Minimum usable state | Test |
|---|---|---|---|---|
| board/table | full-width track below scoreboard | same track with stacked scoreboard | counter text remains visible | visual smoke |
| controls/action panel | five-column grid | two-column grid | buttons stay at least 42px high | visual smoke |
| effect log | full-width list | full-width list | latest effects visible | visual smoke |

## Hidden-information safeguards

| Surface | Safeguard | Test |
|---|---|---|
| browser payload/public view | `get_view` returns Rust public view only | wasm-api tests and UI smoke |
| action tree | controls derive from Rust tree only | code review and UI smoke |
| Rust-generated preview | not implemented | not applicable |
| effect log | `get_effects` uses viewer-filtered Rust effect log | wasm-api tests and UI smoke |
| diagnostics/disabled reasons | Rust diagnostic shown; TS does not infer rule failure | stale smoke |
| DOM attributes | no hidden state attributes | code review |
| test IDs | generic control IDs only | code review |
| browser console/logs | no internal state logs | smoke console review |
| local storage/session storage | not used | code review |
| replay export/import | not implemented | not applicable |
| bot explanations | not implemented beyond public effects | not applicable |
| candidate rankings | not implemented | not applicable |
| dev inspector | not implemented | not applicable |

## UI smoke tests

| Smoke test | Required? | Notes |
|---|---:|---|
| load game picker | no | No picker in Gate 1. |
| start match | yes | `smoke:ui` starts `race_to_n`. |
| show public view | yes | Counter and active seat are checked. |
| show Rust action choices | yes | `add-1` is checked from the action tree. |
| submit human action | yes | `apply_action` advances the counter. |
| run bot turn | yes | `run_bot_turn` returns to human turn or terminal state. |
| show semantic effects | yes | `get_effects` returns effect rows. |
| stale-submission diagnostic | yes | stale freshness token returns `stale_action`. |
