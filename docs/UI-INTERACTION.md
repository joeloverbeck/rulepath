# Rulepath UI and Interaction

Status: public web app and visual interaction law.

Rulepath must feel like a polished playable consumer game site, not a diagnostic harness. The UI is a client of Rust authority: it presents legal actions, previews, viewer-safe views, semantic effects, replay, and bot explanations. It does not decide rules.

## 1. Public UI target

The public app should provide:

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

Public visuals should feel like a cozy board-game table: warm, tactile, inviting, original, polished, readable, and slightly handcrafted without clutter.

Prefer:

- warm surfaces and soft depth;
- premium abstract components;
- clear hierarchy;
- satisfying restrained motion;
- original SVG icons and components;
- color plus shape, not color alone;
- readable typography;
- respectful empty states;
- concise help.

Avoid proprietary mimicry, casino vibes, SaaS dashboard coldness, debug-console dominance, pasted-on screenshots, aggressive skeuomorphism, and trade-dress imitation.

## 3. Ownership split

React/TypeScript owns app shell, menus, game picker, setup, settings, panels, replay controls, accessibility wrappers, local safe storage, WASM integration, and dev inspector UI.

Rust/WASM owns legal action trees, validation, state transitions, public/private views, safe previews, semantic effects, bot decisions, replay, diagnostics, and deterministic simulation.

The renderer owns visual mapping, hit targets derived from Rust legal choices, animation timelines, resize behavior, reduced-motion variants, and debug overlays.

The renderer must not decide legality.

## 4. React + SVG default

V1 uses React + SVG as the default renderer because early ladder games have modest object counts, SVG scales cleanly, SVG is inspectable, debug overlays and accessibility hooks are easier, and a cozy abstract board style fits SVG well.

Canvas may replace or supplement SVG only after measured SVG pressure such as high object count, heavy animation load, or DOM overhead. Canvas adoption requires profiling notes, accessibility plan, debug overlay plan, renderer-boundary preservation, and reduced-motion behavior.

PixiJS may be introduced later only after strong UI-performance evidence or ADR. It is a legitimate renderer for heavier GPU-accelerated 2D scenes, not a v1 default.

## 5. UI data inputs

The browser may receive only viewer-safe payloads.

| Payload | Produced by | Contains | Must not contain |
|---|---|---|---|
| Public view | Rust | visible state for one viewer | hidden state for other seats |
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

Stale submissions must be rejected gracefully with a safe diagnostic and a refreshed action tree.

## 7. Legal controls

Normal mode:

- illegal choices are absent, inert, or visually unavailable;
- legal choices are obvious;
- hidden information is not exposed through disabled controls or tooltips;
- large or compound consequences require confirmation;
- no raw command editing exists.

Learning/debug mode may show disabled choices and Rust-supplied reasons, but only when viewer-safe and clearly labeled.

## 8. Progressive construction

Compound actions must be built through staged legal choices, not raw command objects.

```text
choose action type
  -> choose source/origin
  -> choose target/destination
  -> choose extra pieces/cards/resources
  -> preview cost/effects
  -> confirm
```

At every stage Rust owns legal next choices; UI owns presentation, grouping, focus, and affordances.

## 9. Rust-generated previews

Previews may include visible cost, expected visible effects, next choices, disabled reason, confirmation requirement, and animation hint tags.

Previews must not include hidden card identities, hidden commitments, opponent private information, future random outcomes, bot-only internal facts, or TypeScript-guessed consequences.

## 10. Effect-log-driven animation

Animation must be driven by semantic effects emitted by Rust. Effects are the authoritative cause; visual timelines are presentation.

State diffs may diagnose missing effect coverage, but must not become the causal source for normal animation.

The scheduler must handle grouped effects, simultaneous effects, reveal batches, and reduced motion.

## 11. Settle-to-view rule

After animations complete, the renderer must settle to the latest viewer-safe public view.

A dev assertion mode should detect visual state mismatches, missing effect coverage, lingering objects, illegal hit targets, and hidden data appearing in DOM/logs/local storage.

## 12. Hidden information safety

The browser must not receive unauthorized hidden state in:

- public views;
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

Full-state inspectors are local-dev-only and must not ship hidden state to unauthorized public browsers.

## 13. Dev inspector boundary

Dev UI is required early, but public play must not be debug-first.

Dev toggle may show seed, versions, current actor/phase, legal action count, action tree, selected path, public view, effect log, command log, replay controls, timings, bot decision timing, bot candidate ranking, and visibility selector for local developer builds.

Any inspector that exposes internal state must be excluded from public builds or guarded at the data-source level. A CSS toggle is not a data boundary.

## 14. Replay UI

Replay UI should step command by command, show semantic effects, show action/bot explanations when safe, support speed control and pause, and allow export/import of public-safe replay data.

Replay rendering must use command/effect data and public views, not guessed diffs.

## 15. Bot explanation UI

Public mode should offer a small “why?” or recent-bot-action affordance for non-random bots. It should show a concise reason and relevant visible fact.

Dev mode may show candidate rankings, priority vectors, tie-break seeds, filtered candidate counts, and timing. Dev output must be viewer-safe and must not expose hidden information or hidden-state-derived evaluations.

## 16. Accessibility baseline

Public games should provide visible focus, keyboard-accessible action selection where practical, text labels for icons, sufficient contrast, no reliance on color alone, scalable layout, reduced-motion mode, screen-reader state summaries, and screen-reader legal-action summaries.

Accessibility is easier because Rust supplies explicit action trees and public views.

## 17. Responsiveness baseline

Public games should support desktop, tablet, and phone portrait for simple games where practical. Complex games may require larger screens, but Rulepath should say so clearly and degrade gracefully.

Side panels should collapse; logs and action panels should be readable; touch targets should be safe; board/card sizes should remain playable.

## 18. Public game acceptance checklist

A web-exposed game is acceptable only when:

- default view is playable, pleasant, and not debug-dominated;
- every gameplay click maps to a Rust legal choice;
- illegal moves are not clickable in normal mode;
- compound actions use progressive construction;
- previews and diagnostics come from Rust;
- animations are driven by semantic effects;
- renderer settles to public view;
- replay can step through actions;
- hidden information is not shipped or exposed;
- bot choices have public-safe explanations when non-random;
- dev inspectors are safe and secondary;
- reduced motion works;
- basic focus/keyboard behavior exists where practical;
- public assets are original and neutral.
