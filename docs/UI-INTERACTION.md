# Rulepath UI and Interaction

Status: public web app and visual interaction law.

Rulepath must feel like a polished playable consumer game site, not a diagnostic harness. The UI is a client of Rust authority: it presents legal actions, previews, viewer-safe views, semantic effects, replay, and bot explanations. It does not decide rules.

## 1. Public UI target

The public app SHOULD provide:

- polished game picker;
- clear match setup;
- readable board/card/table presentation;
- obvious legal moves;
- progressive construction for compound actions;
- Rust-generated previews;
- effect-log-driven animation;
- human vs bot;
- local hotseat;
- bot vs bot replay;
- replay viewer;
- safe local replay import/export;
- small bot “why?” affordance;
- dev/debug toggle;
- responsive layout;
- original assets;
- accessibility baseline.

Default mode is play mode. Debug tools are deliberate, secondary, and non-dominant.

## 2. Visual direction

Public visuals should feel like a cozy premium board-game table: warm, tactile, inviting, original, polished, readable, and slightly handcrafted without clutter.

Prefer:

- warm surfaces and soft depth;
- premium abstract components;
- clear hierarchy;
- restrained satisfying motion;
- original SVG icons and components;
- color plus shape, not color alone;
- readable typography;
- respectful empty states;
- concise help.

Avoid:

- proprietary mimicry;
- casino vibes;
- SaaS-dashboard coldness;
- debug-console dominance;
- pasted screenshots/scans;
- aggressive skeuomorphism;
- trade-dress imitation;
- raw JSON as public UX.

## 3. Ownership split

| Area | Owner |
|---|---|
| Legal action trees | Rust/WASM |
| Validation and diagnostics | Rust/WASM |
| Public/private views | Rust/WASM |
| Safe previews | Rust/WASM |
| Semantic effects | Rust/WASM |
| Bot decisions and explanations | Rust/WASM |
| Replay authority | Rust/WASM |
| App shell, layout, panels | React/TypeScript |
| Renderer mapping and animation timelines | React/TypeScript renderer, driven by Rust effects |
| Accessibility wrappers and focus management | React/TypeScript, using Rust labels/actions |
| Local safe import/export UI | React/TypeScript, using Rust serialization contracts |

The renderer MUST NOT decide legality.

## 4. React + SVG default

V1 uses React + SVG as the default renderer because early ladder games have modest object counts, SVG scales cleanly, SVG is inspectable, debug overlays and accessibility hooks are easier, and the cozy abstract board style fits vector presentation.

Canvas MAY supplement or replace SVG only after profiling shows SVG pressure such as high object count, heavy animation load, or DOM overhead. Canvas adoption requires profiling notes, accessibility plan, debug overlay plan, renderer-boundary preservation, and reduced-motion behavior.

PixiJS MAY be introduced later only after strong UI-performance evidence or ADR. It is a legitimate renderer for heavier GPU-accelerated 2D scenes, not a v1 default.

## 5. Browser payload rules

The browser may receive only viewer-safe payloads.

| Payload | Produced by | Contains | Must not contain |
|---|---|---|---|
| Public/private view | Rust | visible state for one viewer | hidden state for other seats |
| Action tree | Rust | legal choices/action paths for actor/viewer | unsafe illegal branches or hidden reasons |
| Preview | Rust | viewer-safe cost/effect estimate and next choices | hidden identities or actual hidden state |
| Effect log | Rust | viewer-filtered semantic effects | unauthorized hidden outcomes |
| Diagnostics | Rust | stale/invalid/unavailable reasons | hidden information disguised as reasons |
| UI metadata | Rust/static typed content | labels, icons, layout hints, accessibility text | legality, rule behavior, hidden state |
| Bot explanation | Rust bot policy | viewer-safe reason summary | hidden facts unavailable to bot/viewer |

Action-tree leaves MAY carry reserved presentation metadata keys with fixed
meaning: `cost` (viewer-visible cost in the acting seat's primary resource),
`cost_rule` (stable rule-reference tag), `eligibility_consequence` (stable
consequence tag resolved through authored explanation templates). When a game
emits reserved keys, shared action surfaces MUST render them at choice and
confirmation time. Reserved keys are documented here, not typed into
engine-core; the kernel treats metadata as opaque.

For N-seat games, browser payloads must already identify the public observer,
authorized seat viewer, active seat or active set, pending responders, and any
viewer-safe role/team labels needed for presentation. TypeScript may visualize
these facts; it must not infer turn order, pending response rights, or legal
seat counts.

## 6. Action lifecycle

```text
UI requests public view and legal action tree
Rust returns view, choices, and freshness token
UI maps legal choices to controls/hit targets
player selects choices
UI requests Rust preview for partial/compound choices
Rust returns safe preview and next legal choices
player confirms full action path
UI submits action path plus freshness token
Rust validates freshness and legality
Rust applies command or returns diagnostic
UI receives new public view and semantic effects
renderer animates effects
renderer settles to public view
```

Stale submissions MUST be rejected gracefully with a safe diagnostic and refreshed action tree.

## 7. Legal-only controls

Normal mode:

- illegal choices are absent, inert, or visually unavailable;
- legal choices are obvious;
- hidden information is not exposed through disabled controls or tooltips;
- large or compound consequences require confirmation;
- no raw command editing exists.

Learning/debug mode MAY show disabled choices and Rust-supplied reasons, but only when viewer-safe and clearly labeled.

Bad:

```text
TypeScript computes that a column is full and disables it.
```

Good:

```text
Rust returns only legal columns; TypeScript renders those controls.
```

## 8. Progressive construction

Compound actions MUST be built through staged legal choices, not raw command objects.

```text
choose action type
  -> choose source/origin
  -> choose target/destination
  -> choose extra pieces/cards/resources
  -> preview cost/effects
  -> confirm
```

At every stage, Rust owns legal next choices. UI owns presentation, grouping, focus, and affordance quality.

## 9. Rust-generated previews

Previews MAY include visible cost, expected visible effects, next choices, disabled reason, confirmation requirement, and animation hint tags.

Previews MUST NOT include hidden card identities, hidden commitments, opponent private information, future random outcomes, bot-only internal facts, or TypeScript-guessed consequences.

## 10. Effect-log-driven animation

Animation MUST be driven by semantic effects emitted by Rust. Effects are authoritative cause; animation timelines are presentation.

The scheduler MUST handle:

- ordered effects;
- grouped effects;
- simultaneous/reveal batches;
- redacted effects;
- reduced-motion mode;
- interruption by replay stepping;
- settle-to-view reconciliation.

State diffs MAY diagnose missing effect coverage in dev mode. They MUST NOT become normal animation authority.

The shared scheduler is the single owner of effect-presentation timing: all
play-path animation and pacing flows through it, and ad-hoc timers outside it
are defects. Skip, acting during animation, and replay stepping share one
flush-and-settle path that finishes (never discards) in-flight animation and
renders the latest viewer-safe public view. Reduced-motion mode replaces
motion with instant transitions plus non-motion feedback while preserving
every conveyed fact as text. Redacted effects receive generic viewer-safe
presentation; animation introduces no new payload or leak surface.

## 10A. Turn orchestration and pacing

Non-interactive advances (bot turns, automated phases, autoplay, replay
playback) play out on the shared animation timeline with authored per-effect
dwell, not as instant state swaps and not behind manual advance triggers.

- Bot turns auto-advance: after the human's effects settle, the bot's turn
  runs and animates; bot-first starts and consecutive bot turns need no click.
- Skip/fast-forward is always available and instantaneous.
- Input never hard-blocks on animation: acting mid-animation flushes the
  timeline to the settled view, then submits.
- Auto-playing sequences expose pause/stop and speed control.
- Reduced-motion mode preserves pacing comprehension through the fast path
  and text narration; it never removes feedback or blocks play.
- Orchestration is presentation policy in TypeScript. It changes when bot and
  automation APIs are called and how results render - never what they decide.
  Wall-clock time stays out of Rust; command logs, traces, replays, and
  hashes are unaffected by pacing.

Repeated presentation shapes across games (per-game `ui.rs` display-metadata
modules, board adapters, effect-to-animation registrations, presentation-TOML
layouts) are governed by this document and the official-game contract; they
are not mechanic-atlas promotion pressure. Promotion of presentation helpers
into `game-stdlib` is deferred until a third structural divergence between
implementations of the same presentation shape, or an official-game count
above 20, and routes through the atlas ledger at that time.

Each official game carries an inert per-game catalog identity in
`games/<id>/src/ui.rs` (icon id, theme key, accent/shape token names, and an
accessibility label). This metadata selects presentation tokens and an original
SVG icon only; it MUST NOT encode legality, selectors, hidden identities,
action availability, rule branches, or behavior-by-naming. Catalog identity is
rendered with color plus shape and visible text, never color alone.

A variant MAY carry an optional one-line `description` (`Option<String>` in
Rust, projected as `GameVariantCatalogEntry.description?` in TypeScript). It is
inert choice-support prose: one line, <=120 characters, neutral and original.
It MUST NOT contain hidden information, rule procedure, conditionals,
selectors, triggers, legality, scoring, strategy advice, trademarks, copied
prose, casino terms, or raw IDs. It is omitted entirely when absent (never
`null`), never synthesized in TypeScript, and never parsed for behavior.
Repeated per-game catalog-theme and variant-description shapes are governed
here and are not mechanic-atlas promotion pressure.

## 10B. Multi-seat layout

Multi-seat layouts must avoid fixed two-column or left/right assumptions.

For games with 3+ seats, the UI SHOULD provide a seat rail or equivalent
orientation surface that can handle the game's declared range, starting with
3-7 seats unless a game spec documents a narrower range. The layout must show
viewer-safe seat labels, active seats, pending responders, and turn-order or
phase cues from Rust/WASM view data.

Local hotseat and replay views need an explicit viewer/observer selector when
multiple viewer projections are supported. Seat indices are dev-panel
vocabulary; normal public UI uses display names, roles, or team labels supplied
by Rust/static typed metadata.

Small-screen layouts may collapse the seat rail, but must preserve active seat,
pending responder, local viewer, and outcome information without relying on
color alone.

## 10C. Showdown and comparison explanations

Showdown, allocation, ranking, and comparison surfaces are presentation of
Rust-owned facts.

When a game exposes a showdown or equivalent terminal comparison, the UI must be
able to render:

- every revealed contender;
- the evaluated combination or equivalent result;
- used components when public or viewer-authorized;
- comparison vector or ranking facts;
- split/tie reason;
- decisive comparison reason; and
- redaction for folded, non-revealed, or unauthorized private data.

The existing shared outcome explanation surface is the preferred rendering
target. TypeScript may format, group, collapse, and label Rust-supplied safe
facts. It must not compute hand strength, score comparisons, allocation,
tiebreakers, winners, or redaction rules.

## 11. Settle-to-view rule

After animations complete, the renderer MUST settle to the latest viewer-safe public view.

Dev assertion mode SHOULD detect:

- visual state mismatch;
- missing effect coverage;
- lingering objects;
- illegal hit targets;
- hidden data in DOM/logs/local storage;
- unsafe test IDs;
- stale action controls after a rejected command.

## 12. Hidden-information safety

The browser MUST NOT receive unauthorized hidden state in:

- public/private views;
- action trees;
- previews;
- effect logs;
- diagnostics;
- UI metadata;
- DOM attributes;
- `data-testid` values;
- local storage;
- replay exports;
- debug panels;
- bot explanations;
- candidate rankings.

Bad:

```text
<div data-card-id="secret_ace" class="card back">
```

Good:

```text
<div data-visible-kind="face_down_card" class="card back">
```

Full-state inspectors are local-dev-only and must be excluded from public builds or guarded at the data-source level. A CSS toggle is not a data boundary.

## 13. Dev inspector boundary

Dev UI is required early, but public play MUST NOT be debug-first.

Dev toggle MAY show seed, versions, current actor/phase, legal action count, action tree, selected path, public view, effect log, command log, replay controls, timings, bot decision timing, bot candidate ranking, and visibility selector for local developer builds.

Any inspector that exposes internal state MUST be excluded from public builds or must receive data only through the same viewer-safe projection as public UI.

## 14. Replay UI

Replay UI SHOULD:

- step command by command;
- show semantic effects;
- show action/bot explanations when safe;
- support pause and speed control;
- support reduced motion;
- expose seed/version/variant metadata;
- allow export/import of public-safe replay data.

Replay rendering MUST use command/effect data and public views, not guessed diffs.

## 15. Bot explanation UI

Public mode SHOULD offer a small “why?” or recent-bot-action affordance for non-random bots. It should show a concise reason and relevant visible fact.

Dev mode MAY show candidate rankings, priority vectors, tie-break seeds, filtered candidate counts, and timing. Dev output MUST be viewer-safe and MUST NOT expose hidden information or hidden-state-derived evaluations.

## 16. Outcome / victory explanation surface

Every official web-exposed game MUST render a shared outcome explanation surface when a match becomes terminal.

The outcome surface answers "why did this result happen?" in player-facing terms. It is mandatory for every catalog game, including tiny games. Small games may have small explanations, but they may not omit the surface.

The surface MUST show:

1. the final result: winner, draw, split, or game-specific terminal result;
2. the decisive cause of that actual result, such as a completed line, exact target reached, terminal no-move/no-piece reason, final score comparison, showdown strength comparison, or terminal tiebreaker rung;
3. a viewer-safe final standing for every player;
4. a viewer-safe per-player breakdown sufficient to understand the result; and
5. stable rule references back to the game's scoring and terminal rules.

The surface MUST be driven by Rust-owned public/terminal view data and/or Rust-owned terminal semantic effects. TypeScript may render, lay out, interpolate safe template parameters, and manage disclosure/focus state. TypeScript MUST NOT compute the winner, score comparison, showdown strength, winning line, terminal tiebreaker, or decisive cause.

The surface MUST explain only the actual result. It MUST NOT provide coaching, counterfactual "what would have changed it" advice, hidden turning-point analysis, or AI strategy commentary.

Hidden-information games MUST use the same viewer-safe projection discipline as the rest of the UI. The outcome surface MUST NOT expose hidden information in visible text, hidden DOM text, accessibility labels, `data-testid`s, CSS classes, storage, logs, effect logs, replay exports, dev panels, or bot explanation surfaces. If a result resolves without reveal, the explanation must say so without revealing or implying unrevealed private facts.

The surface MUST be accessible:

- the terminal summary is programmatically exposed as a status/result message;
- the decisive cause is available as text and not only by color, animation, icon, or board highlight;
- player identity and standing are color-independent;
- expandable breakdowns use accessible disclosure controls;
- score/tiebreak tables or lists have readable labels; and
- reduced-motion mode preserves the same information without relying on animation.

## 17. Accessibility baseline

Public games SHOULD provide:

- visible focus;
- keyboard-accessible action selection where practical;
- accessible names for controls and SVG elements;
- text labels for icons;
- sufficient contrast;
- no reliance on color alone;
- scalable layout;
- reduced-motion mode;
- screen-reader state summaries;
- screen-reader legal-action summaries.

Accessibility is easier because Rust supplies explicit action trees and public views. Use that structure rather than trying to make arbitrary SVG clicks understandable after the fact.

Primary rules help MUST be reachable without hover. The public web UI MUST expose
a shared How to Play / Rules surface for every catalog game from the game picker,
match setup, and in-play controls. The surface renders authored
`HOW-TO-PLAY.md` content only.

The rules surface is presentation-only. It MUST NOT decide legality, inspect
private state, write replay/effect data, or generate runtime rules text.

The surface must meet the accessibility baseline: keyboard access, visible
focus, accessible names, focus management for modal/sheet mode, and
screen-reader navigable headings.

## 18. Responsive behavior

Simple public games SHOULD support desktop, tablet, and phone portrait where practical.

Complex games MAY require larger screens, but Rulepath should say so clearly and degrade gracefully.

Side panels should collapse. Logs and action panels should remain readable. Touch targets should be safe. Board/card sizes should remain playable.

## 19. UI acceptance check

A web-exposed game is acceptable only when:

- default view is playable, pleasant, and not debug-dominated;
- every gameplay click maps to a Rust legal choice;
- illegal moves are not clickable in normal mode;
- compound actions use progressive construction;
- previews and diagnostics come from Rust;
- semantic effects drive animation;
- semantic effects animate through the shared scheduler (or a recorded
  board-native/not-applicable adoption row), and the renderer settles to the
  latest viewer-safe public view after every burst, skip, or interruption;
- renderer settles to latest public view;
- bot turns and automated phases auto-advance on the animation timeline with
  always-available skip and pause; no manual advance trigger exists in normal
  mode;
- acting during animation is never blocked; it flushes to the settled view
  and submits;
- reduced-motion mode conveys every animated fact through non-motion
  presentation and never blocks play;
- replay can step through actions;
- bot choices have public-safe explanations when non-random;
- choices carrying reserved cost/consequence metadata display them before
  selection and in the confirmation summary, interpolated only with
  Rust-supplied numbers and viewer-safe balances;
- action labels and accessibility labels contain display names, never raw
  internal identifiers; a runtime DOM sweep enforces this in normal mode;
- multi-target stages render as composed target selection or a recorded
  board-native mapping, never one control per combination;
- normal-mode surfaces name the local viewer's faction/role and the acting
  faction in display terms; seat indices are dev-panel vocabulary;
- non-interactive advances (bot turns, automated phases) are narrated near
  the board from viewer-filtered semantic effects in authored vocabulary.
- dev inspectors are safe and secondary;
- reduced motion works;
- basic focus/keyboard behavior exists where practical;
- hidden information is absent from browser payloads, DOM, local storage, test IDs, logs, and replay exports;
- visuals are original and avoid proprietary presentation.
- terminal matches render the shared outcome explanation surface;
- the surface shows the final result, decisive cause, every player's final standing, and a viewer-safe per-player breakdown;
- the decisive cause is supplied by Rust public/terminal view data and/or Rust terminal semantic effects; TypeScript does not compute the result explanation;
- the surface explains only the actual result and contains no coaching, counterfactuals, or strategy advice;
- hidden-information games prove that outcome explanations do not leak unrevealed private state through text, DOM attributes, accessibility labels, test IDs, storage, logs, effect logs, replay export, dev panels, or bot explanation surfaces;
- the terminal summary is accessible to screen readers as a status/result message; expanded breakdowns are keyboard-accessible and color-independent; reduced-motion mode preserves all outcome facts.
- components with ordered card/component flows display Rust/static-supplied labels and short effect summaries at a glance; full text sits in an accessible detail tier; TypeScript derives no display text from IDs;
- face-down/redacted piles use the shared deck presentation with Rust/static-supplied copy; no hardcoded redaction prose;
- compound actions route through the shared progressive construction surface or a recorded board-native mapping; no flat leaf-path dumps;
- normal-mode surfaces contain no engine/debug vocabulary or raw internal identifiers; the presentation-copy CI guard passes.
