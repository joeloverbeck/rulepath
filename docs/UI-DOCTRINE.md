# Rulepath UI Doctrine

Status: public web app and rendering law.

The public UI matters from the beginning. It must feel like a polished playable consumer site, not a diagnostic harness. The UI is still a client of the Rust engine: it presents legal actions and semantic effects; it does not decide rules.

## 1. Product target

The public web app SHOULD provide:

- polished game picker;
- clear match setup;
- readable board/card presentation;
- obvious legal moves;
- progressive action construction for compound moves;
- effect-log-driven animation;
- replay viewer;
- human vs bot;
- local hotseat;
- bot vs bot replay;
- local replay save/load;
- dev/debug toggle;
- responsive layout;
- original assets;
- accessible interaction where practical.

The default mode SHOULD be play mode. Debug tools are visible when intentionally enabled.

## 2. Ownership split

React/TypeScript owns:

- app shell;
- routes/pages if any;
- menus;
- game picker;
- match setup;
- panels;
- settings;
- replay controls;
- accessibility wrappers;
- WASM integration;
- local storage of safe replay artifacts;
- dev inspector UI.

Rust/WASM owns:

- legal actions;
- validation;
- state transitions;
- public/private views;
- previews;
- semantic effects;
- bot decisions;
- replay;
- deterministic simulation.

The renderer owns:

- visual representation;
- hit targets mapped from legal action choices;
- animation timelines;
- resize behavior;
- mapping public view/effects to visuals;
- reduced-motion variants;
- debug overlays when enabled.

The renderer MUST NOT decide legality.

## 3. Recommended v1 architecture

```text
React app shell
  -> session controller
  -> batched WASM API
  -> public-view store
  -> legal-action tree store
  -> effect queue
  -> renderer adapter
  -> panels: actions, log, replay, help, settings, dev inspectors
```

The session controller coordinates calls to Rust. It should be boring.

The renderer adapter receives only viewer-safe public view data, action affordances derived from Rust legal action trees, and viewer-filtered semantic effects.

## 4. V1 renderer choice: React + SVG

Use React + SVG as the default v1 board/card renderer.

Reasons:

- early ladder games have modest object counts;
- SVG scales cleanly;
- SVG is inspectable in the DOM;
- SVG supports crisp abstract boards, icons, coordinates, highlights, and overlays;
- accessibility labels and debug overlays are easier than with pure canvas;
- it avoids adopting a heavier game renderer before measured pressure exists.

This is a default, not a religion. Keep the renderer boundary clean.

## 5. Canvas policy

Canvas MAY replace or supplement SVG only when measured needs justify it, such as:

- hundreds/thousands of moving objects;
- heavy animation load;
- particle-like effects;
- custom low-level drawing;
- measurable SVG DOM overhead;
- visual effects that SVG cannot deliver cleanly.

Canvas SHOULD NOT be adopted merely because it feels more “game-like”.

Canvas adoption requires:

- benchmark or profiling note;
- accessibility plan;
- debug overlay plan;
- renderer-boundary preservation;
- reduced-motion behavior.

## 6. PixiJS policy

PixiJS MAY be considered after SVG/Canvas pressure is real.

Use PixiJS only if:

- public presentation genuinely needs heavier graphics;
- WebGL/WebGPU acceleration is useful;
- object counts or animation load justify bundle complexity;
- the renderer boundary already exists;
- accessibility/debug tradeoffs are accepted explicitly;
- ADR or UI technical note records the reason.

Do not start with PixiJS before the early ladder proves the UI contract.

## 7. Effect-log-driven animation

The UI MUST animate semantic effects emitted by Rust.

Correct pipeline:

```text
player chooses action path
  -> Rust validates and applies command
  -> Rust emits semantic effects
  -> UI receives viewer-filtered effects
  -> renderer schedules animations
  -> renderer settles to new viewer-safe public view
```

Forbidden pipeline:

```text
old view + new view
  -> UI diffs state
  -> UI guesses causality
  -> animations gradually become wrong
```

State diffs MAY be used in diagnostics to detect missing effect coverage. They MUST NOT be the primary animation source.

## 8. Legal action UI

Normal player mode:

- illegal choices are not clickable;
- legal choices are visible and understandable;
- the player is not flooded with irrelevant illegal choices;
- stale submissions are rejected gracefully;
- confirmation is used when consequences are large or compound;
- hidden information is not exposed through disabled controls or tooltips.

Learning/debug mode:

- MAY show disabled illegal choices;
- SHOULD show Rust-provided reasons;
- MUST NOT leak hidden information;
- SHOULD make clear it is not the default play experience.

Simple actions may be direct: click a legal cell, column, card, or button.

Compound actions MUST use progressive construction:

```text
choose action type
  -> choose target/source
  -> choose additional pieces/cards/resources
  -> preview cost/effects
  -> confirm
```

## 9. Previews

Previews are Rust-generated and viewer-safe.

Previews MAY include:

- cost summary;
- legal next choices;
- visible expected effects;
- disabled reason for the selected partial action;
- whether confirmation is required;
- animation hint tags.

Previews MUST NOT include:

- hidden card identities;
- unrevealed commitments;
- opponent private information;
- real hidden state used by bots;
- guessed rule consequences from TypeScript.

## 10. Effect log and explanation panel

The UI SHOULD show a human-readable log derived from semantic effects and explanation templates.

Every log entry SHOULD answer:

- who acted;
- what action was chosen;
- what changed;
- why a forced transition occurred when relevant;
- what choice is pending now;
- what information is private/redacted when relevant.

The UI MUST NOT generate authoritative rules explanations from arbitrary state diffs.

## 11. Debug/developer UI

Debug UI is required early but must not dominate public play.

The web app SHOULD include a visible dev toggle with:

- seed display;
- game/rules/data version display;
- current actor/phase display;
- legal action count;
- action tree inspector;
- selected action path inspector;
- public view inspector;
- effect log inspector;
- command log inspector;
- replay controls;
- performance timings;
- bot decision timing;
- bot candidate ranking when available;
- visibility mode selector in local developer builds only.

State inspectors that expose internal state MUST be local/dev-only. They MUST NOT ship hidden private state to unauthorized remote browsers.

## 12. Visual doctrine

Public visuals MUST be original.

Prefer:

- clean abstract premium style;
- readable typography;
- responsive layouts;
- clear state hierarchy;
- restrained but satisfying motion;
- original SVG icons;
- shape plus color, not color alone;
- colorblind-safe palettes;
- meaningful hover/focus/selection states;
- reduced-motion support.

Avoid:

- mimicking proprietary boards, cards, components, iconography, colors, or trade dress;
- trademark-forward presentation;
- noisy animations that obscure rules;
- skeuomorphic clutter that hurts readability;
- beautiful boards with broken replay/debug tools;
- debug-first layouts in public mode.

## 13. Accessibility baseline

The UI SHOULD provide:

- keyboard-accessible action selection where practical;
- visible focus states;
- sufficient contrast;
- text labels for icons;
- scalable layout;
- reduced-motion mode;
- screen-reader-friendly state summaries;
- screen-reader-friendly legal-action summaries;
- no reliance on color alone.

Accessibility is easier because Rust provides explicit public views and action trees.

## 14. Responsiveness policy

Every public game SHOULD support:

- desktop landscape;
- tablet landscape/portrait;
- phone portrait for simple games where practical;
- flexible side panels;
- collapsible logs/action panels;
- readable board/card sizes;
- touch-safe target sizes.

The public app SHOULD degrade gracefully. A complex game may require a larger screen, but the site should say so cleanly.

## 15. Public site modes

Initial modes:

- human vs bot;
- local hotseat;
- bot vs bot replay;
- replay viewer.

Not initial modes:

- accounts;
- matchmaking;
- hosted multiplayer;
- chat;
- ranked play;
- cloud saves.

## 16. UI acceptance checklist

A public web game is acceptable only when:

- the default view is playable and pleasant;
- legal moves are obvious;
- illegal moves are not clickable in normal mode;
- compound moves are progressively constructed;
- Rust generates previews and diagnostics;
- animations are driven by semantic effects;
- replay can step through actions;
- hidden information is not shipped to unauthorized viewers;
- bot choices can be inspected in dev mode;
- reduced-motion mode exists;
- basic keyboard/focus behavior exists where practical;
- public assets are original and neutral.

## Source notes

See `SOURCES.md`, especially React, MDN SVG, MDN Canvas API, PixiJS, boardgame.io, Rust/WASM, command-log replay, and IP sources.
