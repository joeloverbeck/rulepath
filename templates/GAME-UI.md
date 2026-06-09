# <game_id> UI

Game ID: `<game_id>`

Implemented variant: `<variant>`

Rules version: `<rules_version>`

Renderer assumptions version: `<ui_version>`

Prepared by: `<name/agent>`

Last updated: YYYY-MM-DD

## Purpose

This document defines the product-facing web UI plan for the game. It applies `docs/UI-INTERACTION.md` to one game.

TypeScript never decides legality. Rust/WASM owns legal action trees, validation, state transitions, public/private views, safe previews, semantic effects, diagnostics, replay, serialization, and bot decisions. TypeScript maps Rust-provided legal choices to controls and renders viewer-safe payloads.

## Product and visual target

| Field | Decision |
|---|---|
| public role | scaffolding / UI smoke / showcase / hidden-info proof / original portfolio game / other: `<role>` |
| desired feel | cozy board-game table / minimal abstract / card-table / other: `<feel>` |
| visual risk to avoid | debug-console-first / casino vibe / SaaS dashboard / proprietary mimicry / clutter / `<risk>` |
| public onboarding need | none / light / substantial |
| help/learning mode need | none / light / substantial |

## Renderer assumptions

React + SVG is the default renderer unless profiling evidence or ADR says otherwise.

| Assumption | Value | Evidence/notes |
|---|---|---|
| default renderer | React + SVG | `<notes>` |
| expected object count | `<count/range>` | `<notes>` |
| animation pressure | low / medium / high | `<notes>` |
| SVG pressure expected? | yes/no | `<notes>` |
| Canvas/PixiJS needed? | no by default / profiling required / ADR required | `<notes>` |
| WASM boundary | batched Rust calls; no chatty rule hot loops | `<notes>` |

## Screen-size support

| Size/context | Supported? | Layout notes | Smoke test |
|---|---:|---|---|
| small phone portrait | yes/no | `<notes>` | `<test>` |
| phone landscape | yes/no | `<notes>` | `<test>` |
| tablet | yes/no | `<notes>` | `<test>` |
| desktop | yes/no | `<notes>` | `<test>` |
| keyboard-only desktop | yes/no | `<notes>` | `<test>` |
| reduced-motion user | yes/no | `<notes>` | `<test>` |

## Legal action mapping

| Rust action choice/tree node | Rule IDs | UI control/hit target | Normal mode behavior | Learning/debug behavior | Accessibility label | Notes |
|---|---|---|---|---|---|---|
| `<choice>` | `<rule_ids>` | cell/button/card/zone/menu/etc. | enabled only if Rust says legal; absent or inert if illegal | Rust-supplied safe diagnostics only | `<label>` | `<notes>` |

Illegal choices MUST NOT become legal through UI bugs. Hidden information MUST NOT leak through disabled controls, tooltip text, CSS classes, DOM attributes, test IDs, or diagnostics.

## Progressive construction flow

Use for compound actions. Mark `not applicable` for flat games.

| Stage | Rust-owned input/output | UI presentation | Preview needed? | Confirmation needed? | Notes |
|---|---|---|---:|---:|---|
| 1 | `<Rust legal choices>` | `<UI grouping/focus>` | yes/no | yes/no | `<notes>` |
| 2 | `<Rust next legal choices>` | `<UI grouping/focus>` | yes/no | yes/no | `<notes>` |
| 3 | `<Rust safe preview>` | `<preview presentation>` | yes/no | yes/no | `<notes>` |
| 4 | `<validated action path>` | confirm/cancel | yes/no | yes/no | `<notes>` |

At every stage, next choices come from Rust. The UI owns presentation, grouping, focus, and affordances only.

## Rust-generated previews

| Preview | Trigger | Viewer-safe contents | Must not contain | Tests |
|---|---|---|---|---|
| `<preview>` | `<partial_action>` | visible cost/effects/next choices/confirmation hint | hidden state, hidden identities, future random outcomes, bot-only facts, TypeScript-guessed consequences | `<tests>` |

## Semantic effect-to-animation mapping

Animation is driven by semantic effects emitted by Rust. State diffs may diagnose missing effect coverage; they are not normal animation authority.

| Semantic effect | Visual animation | Timing/priority | Reduced-motion replacement | Settle-to-view check | Rule IDs |
|---|---|---|---|---|---|
| `<effect>` | `<animation>` | `<timing>` | instant state change / fade / highlight / text summary / none | after animation, render latest viewer-safe public view | `<rule_ids>` |

## Settle-to-view checks

| Scenario | Required check | Test |
|---|---|---|
| after legal action animation | renderer settles to Rust public view | `<test>` |
| after replay step | renderer settles to Rust replayed public view | `<test>` |
| after stale/invalid diagnostic | action tree refreshed from Rust | `<test>` |
| after bot action | effect animation and explanation match Rust result | `<test>` |
| after reduced-motion path | no essential information is animation-only | `<test>` |

## Replay UI

| Feature | Required? | Viewer-safe requirement | Tests/notes |
|---|---:|---|---|
| replay step forward/back | yes/no | no hidden state leak | `<test>` |
| effect log display | yes/no | viewer-filtered effects only | `<test>` |
| command log display | yes/no | safe commands only | `<test>` |
| hash/version display | yes/no | safe metadata only | `<test>` |
| local replay import/export | yes/no | redacted/export-safe payload | `<test>` |
| bot-vs-bot replay | yes/no | public-safe explanations/rankings | `<test>` |

## Bot explanation UI

| Surface | Public mode | Dev mode | Hidden-info safeguard | Tests |
|---|---|---|---|---|
| recent bot action | concise reason | timing and policy version may show | no hidden facts | `<test>` |
| “why?” affordance | optional short explanation | optional expanded viewer-safe details | no candidate hidden facts | `<test>` |
| candidate ranking | not public by default | viewer-safe/redacted only | no forbidden info | `<test>` |
| known weakness | optional concise note | detailed notes okay if safe | no private state | `<test>` |

## Outcome / victory explanation

Describe the end-of-match explanation shown by the shared web outcome surface.

This section is mandatory for every web-exposed official game. Tiny games may provide a tiny explanation, but they may not omit the section.

### Terminal result variants

List every terminal result variant the game can produce.

| Result variant | Rust source of truth | Player-facing summary | Rule IDs |
|---|---|---|---|
| `<win/draw/split/yield/etc.>` | `<TerminalView field / PublicView field / terminal effect>` | `<one sentence>` | `<R-END-*/R-SCORE-*>` |

### Decisive cause payload

Name the Rust-owned public/terminal view fields and/or terminal semantic effects that carry the decisive cause.

| Cause variant | Rust payload field(s) | Static template key | Notes |
|---|---|---|---|
| `<line completed / score comparison / tiebreaker / showdown strength / no legal move / exact target>` | `<field names>` | `<game_id.template_key>` | `<viewer-safe notes>` |

TypeScript MUST NOT compute these cause variants. It renders the Rust-projected value only.

### Per-player final breakdown

List every value shown for every player at terminal.

| Breakdown value | Source | Visible to public observer? | Visible to seat viewer? | Hidden-info notes |
|---|---|---:|---:|---|
| `<score/line/strength/piece count/allocation/etc.>` | `<Rust field/effect>` | `<yes/no>` | `<yes/no>` | `<redaction/reveal rule>` |

### No-leak rules

State what the outcome surface must never reveal.

- Visible text:
- Hidden DOM/accessibility attributes:
- `data-testid`/selectors:
- Storage/logs/dev panel:
- Effect log/replay export:
- Bot explanations/candidate rankings:

For hidden-information games, explicitly cover no-reveal terminal outcomes. Example: a yielded private card/crest remains hidden and the outcome surface says the result resolved without private reveal.

### Player-facing copy contract

The outcome surface explains only the actual result. It must not include coaching, counterfactuals, "what would have changed it," turning-point analysis, or strategy advice.

### Accessibility and reduced motion

Confirm:

- terminal summary is exposed as a status/result message;
- decisive cause is text, not color-only or animation-only;
- player standing is color-independent;
- expanded breakdown is keyboard accessible;
- reduced-motion mode preserves all facts; and
- replay terminal renders the same outcome content for the same viewer.

### Smoke and tests

List the terminal smoke/no-leak cases required for this game.

| Test case | Terminal path | Required assertion |
|---|---|---|
| `<case>` | `<fixture/trace/scripted path>` | `<summary, breakdown, no-leak assertion>` |

## Dev inspector boundary

Dev tools are secondary. Public mode is play first, not a diagnostic harness.

| Inspector item | Public build allowed? | Dev build allowed? | Must not contain | Tests |
|---|---:|---:|---|---|
| seed/rules/data version | yes/no | yes | hidden state | `<test>` |
| public view inspector | yes/no | yes | hidden state | `<test>` |
| action tree inspector | no by default | yes if viewer-safe | hidden reasons/state | `<test>` |
| selected action path | yes/no | yes | hidden state | `<test>` |
| effect log | yes/no | yes if viewer-filtered | hidden outcomes | `<test>` |
| command log | no by default | yes if safe | private data | `<test>` |
| bot timing | yes/no | yes | hidden facts | `<test>` |
| candidate ranking | no | yes if redacted | hidden facts | `<test>` |
| full internal state | no | test harness only, not public build | all hidden state | `<test>` |

## Accessibility labels and semantics

| Element/control | Accessible name/description | Role/semantic element | Keyboard path | Focus behavior | Notes |
|---|---|---|---|---|---|
| `<element>` | `<label/description>` | button / list / grid / group / SVG title-desc / other | `<keys/tab order>` | `<focus behavior>` | `<notes>` |

Prefer semantic HTML controls where possible. SVG elements need accessible names or equivalent controls when interactive.

## Keyboard and focus plan

| Interaction | Keyboard path | Focus movement | Escape/cancel behavior | Test |
|---|---|---|---|---|
| choose legal action | `<keys>` | `<focus>` | `<cancel>` | `<test>` |
| progressive action stage | `<keys>` | `<focus>` | `<cancel>` | `<test>` |
| confirm/cancel | `<keys>` | `<focus>` | `<cancel>` | `<test>` |
| replay controls | `<keys>` | `<focus>` | `<cancel>` | `<test>` |
| bot explanation/help | `<keys>` | `<focus>` | `<cancel>` | `<test>` |

Focus indicators MUST remain visible. Do not remove outlines unless a replacement focus indicator exists.

## Screen-reader summaries where practical

| Summary | Trigger | Contents | Must not contain | Test/notes |
|---|---|---|---|---|
| current position/state | on load/action/replay step | concise viewer-safe state | hidden/private info | `<test>` |
| legal actions | on actor turn | count and labels of legal choices | hidden reasons | `<test>` |
| action result | after effect settle | viewer-safe semantic result | hidden outcomes | `<test>` |
| bot explanation | after bot action | viewer-safe reason | hidden facts | `<test>` |

## Contrast and color/shape notes

| Item | Color use | Non-color cue | Contrast concern | Test/review |
|---|---|---|---|---|
| legal choice | `<color>` | shape/outline/label/motion | `<concern>` | `<test>` |
| selected item | `<color>` | shape/outline/label | `<concern>` | `<test>` |
| player identity | `<color>` | icon/shape/label | `<concern>` | `<test>` |
| warnings/errors | `<color>` | text/icon | `<concern>` | `<test>` |

Do not rely on color alone.

## Reduced-motion behavior

| Motion/effect | Default behavior | Reduced-motion behavior | Information preserved? | Test |
|---|---|---|---:|---|
| `<animation>` | `<default>` | instant/fade/highlight/text summary/none | yes/no | `<test>` |

Reduced-motion mode MUST reduce, replace, or remove non-essential motion while preserving feedback.

## Responsive behavior

| UI region | Desktop behavior | Small-screen behavior | Minimum usable state | Test |
|---|---|---|---|---|
| board/table | `<behavior>` | `<behavior>` | `<minimum>` | `<test>` |
| controls/action panel | `<behavior>` | `<behavior>` | `<minimum>` | `<test>` |
| log/replay | `<behavior>` | `<behavior>` | `<minimum>` | `<test>` |
| bot explanation/help | `<behavior>` | `<behavior>` | `<minimum>` | `<test>` |

## Hidden-information safeguards

Fill every surface. Perfect-information games may use explicit `not applicable` with rationale.

| Surface | Safeguard | Test |
|---|---|---|
| browser payload/public view | `<safeguard>` | `<test>` |
| action tree | `<safeguard>` | `<test>` |
| Rust-generated preview | `<safeguard>` | `<test>` |
| effect log | `<safeguard>` | `<test>` |
| diagnostics/disabled reasons | `<safeguard>` | `<test>` |
| DOM attributes | `<safeguard>` | `<test>` |
| test IDs | `<safeguard>` | `<test>` |
| browser console/logs | `<safeguard>` | `<test>` |
| local storage/session storage | `<safeguard>` | `<test>` |
| replay export/import | `<safeguard>` | `<test>` |
| bot explanations | `<safeguard>` | `<test>` |
| candidate rankings | `<safeguard>` | `<test>` |
| dev inspector | `<safeguard>` | `<test>` |

## UI smoke tests

| Smoke test | Required? | Notes |
|---|---:|---|
| load game picker | yes/no | `<notes>` |
| start match | yes/no | `<notes>` |
| show public view | yes/no | `<notes>` |
| show legal actions | yes/no | `<notes>` |
| apply one human action | yes/no | `<notes>` |
| run one bot turn | yes/no | `<notes>` |
| show safe diagnostics for stale/invalid action | yes/no | `<notes>` |
| show effect log | yes/no | `<notes>` |
| replay at least one step | yes/no | `<notes>` |
| reduced-motion smoke | yes/no | `<notes>` |
| responsive smoke | yes/no | `<notes>` |
| keyboard/focus smoke | yes/no | `<notes>` |
| accessibility scan where practical | yes/no | `<notes>` |
| hidden-info no-leak smoke if applicable | yes/no/not applicable | `<notes>` |

## Review checklist

- UI is product-facing, not debug-console-first.
- React + SVG remains the default unless profiling/ADR justifies otherwise.
- TypeScript never decides legality.
- Legal controls map to Rust action choices.
- Progressive construction uses Rust next choices at every stage.
- Previews are Rust-generated and viewer-safe.
- Animation is effect-driven and settles to Rust public view.
- Accessibility labels, keyboard/focus, screen-reader summaries, contrast, reduced motion, and responsive behavior are explicit.
- Hidden-information safeguards cover browser payloads, action tree, preview, effect log, diagnostics, DOM attributes, test IDs, logs, local storage, replay export, bot explanations, candidate rankings, and dev inspector.
- UI smoke tests are not treated as proof of rule correctness.
