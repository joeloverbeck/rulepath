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

## 16. Accessibility baseline

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

## 17. Responsive behavior

Simple public games SHOULD support desktop, tablet, and phone portrait where practical.

Complex games MAY require larger screens, but Rulepath should say so clearly and degrade gracefully.

Side panels should collapse. Logs and action panels should remain readable. Touch targets should be safe. Board/card sizes should remain playable.

## 18. UI acceptance check

A web-exposed game is acceptable only when:

- default view is playable, pleasant, and not debug-dominated;
- every gameplay click maps to a Rust legal choice;
- illegal moves are not clickable in normal mode;
- compound actions use progressive construction;
- previews and diagnostics come from Rust;
- semantic effects drive animation;
- renderer settles to latest public view;
- replay can step through actions;
- bot choices have public-safe explanations when non-random;
- dev inspectors are safe and secondary;
- reduced motion works;
- basic focus/keyboard behavior exists where practical;
- hidden information is absent from browser payloads, DOM, local storage, test IDs, logs, and replay exports;
- visuals are original and avoid proprietary presentation.
