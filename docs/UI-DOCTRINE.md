# UI DOCTRINE

Status: public web app and rendering law.

The public web app matters from the beginning, but it MUST NOT drive engine architecture. The UI is a client of the Rust engine.

## 1. Product goal

The UI should make a public playable site feel clear, pleasant, and trustworthy.

It SHOULD provide:

- game picker;
- readable board/card presentation;
- clear legal moves;
- progressive action construction for compound moves;
- effect-log-driven animation;
- replay viewer;
- human vs bot;
- local hotseat;
- bot vs bot replay;
- debug/developer tools;
- responsive layout;
- original assets.

It MUST NOT implement rules or infer legality.

## 2. Initial app architecture

Recommended v1 shape:

```text
React/TypeScript shell
  -> WASM engine package
  -> game session controller
  -> board renderer boundary
  -> panels: actions, log, replay, inspectors, settings
```

React SHOULD own:

- app layout;
- menus;
- match setup;
- game picker;
- action panels;
- inspector panels;
- replay controls;
- settings;
- logs;
- accessibility wrappers.

The renderer SHOULD own:

- board coordinates;
- piece/card visuals;
- animation timelines;
- pointer hit targets;
- resize behavior;
- visual state derived from public views and effect logs.

The renderer MUST NOT decide whether an action is legal.

## 3. Rendering recommendation

### v1 default: React + SVG board renderer

Use a React shell with a clearly separated SVG board renderer for the first playable version.

Why:

- SVG is a web-standard vector format designed for clean rendering at any size.
- SVG elements are scriptable and inspectable through the DOM.
- Board games usually have modest object counts in early ladder stages.
- SVG supports crisp abstract boards, icons, coordinates, highlights, and simple animations without committing early to a game-rendering engine.
- SVG makes debug overlays and accessibility labels easier than a pure canvas from day one.

This is a recommendation, not a religion. Keep the renderer boundary clean so the project can swap renderer technology later.

### When to consider Canvas

Canvas MAY replace or supplement SVG when measured needs justify it:

- hundreds/thousands of moving objects;
- heavy animation load;
- particle/visual effects;
- custom low-level drawing;
- SVG DOM overhead becomes measurable.

Canvas should not be adopted merely because it feels more “game-like.”

### When to consider PixiJS

PixiJS MAY be considered after SVG/Canvas pressure is real.

Use PixiJS only if:

- board rendering has become graphically demanding;
- WebGL/WebGPU acceleration is actually useful;
- the renderer boundary already exists;
- debug and accessibility costs are accepted;
- bundle complexity is justified by measured benefit.

Do not start with PixiJS before simple games prove the UI contract.

## 4. Effect-log-driven animation

The UI MUST animate semantic effects emitted by Rust.

Correct pipeline:

```text
Player chooses action
  -> Rust validates and applies command
  -> Rust emits effect log
  -> UI receives effects
  -> renderer schedules animations
  -> rendered state settles to new public view
```

Forbidden pipeline:

```text
Rust mutates state
  -> UI diffs old/new public views
  -> UI guesses causality
  -> animation bugs accumulate
```

State diffs MAY be used for debugging, not primary animation causality.

## 5. Legal action UI

For simple games, the board MAY directly expose all legal cells/moves.

For compound games, the UI MUST use progressive construction:

```text
choose action type
  -> choose target
  -> choose pieces/cards/resources
  -> preview cost/effects
  -> confirm
```

Required UI behavior:

- legal choices are obvious;
- illegal choices are not clickable in player mode;
- learning/debug mode MAY show disabled choices with reasons;
- stale actions are rejected gracefully;
- pending reaction/waiting states are clear;
- hidden information remains hidden.

## 6. Effect log and explanation panel

The UI SHOULD show a human-readable log derived from semantic effects and explanation templates.

Every log entry SHOULD answer:

- who acted;
- what action was chosen;
- what changed;
- why a forced transition occurred when relevant;
- what choice is now pending.

The UI MUST NOT generate authoritative rules explanations from arbitrary state diffs. It should render authored templates supplied by game modules and effect types.

## 7. Debug/developer UI

Debug UI is required early.

The web app SHOULD include:

- seed display;
- game/rules version display;
- current player/phase display;
- legal action count;
- action tree inspector;
- public view inspector;
- state inspector in local/dev builds only;
- effect log inspector;
- command log inspector;
- replay controls;
- performance timings;
- bot decision timing;
- visibility mode selector for local debugging.

Debug UI MUST NOT ship hidden private state to unauthorized remote browsers. Local developer builds may expose internal state.

## 8. Visual doctrine

Public visuals MUST be original.

Prefer:

- clean abstract boards;
- readable typography;
- responsive layouts;
- simple motion with clear timing;
- original SVG icons;
- colorblind-safe encodings;
- shape plus color, not color alone;
- pleasant but restrained effects.

Avoid:

- mimicking proprietary boards/cards/components;
- trademark-forward presentation;
- noisy animations that obscure rules;
- clever UI that hides legal choices;
- beautiful boards with broken replay/debug tools.

## 9. Accessibility baseline

The UI SHOULD provide:

- keyboard-accessible action selection where practical;
- sufficient contrast;
- text labels for icons;
- scalable layout;
- reduced-motion mode;
- screen-reader-friendly summaries for state and legal actions;
- no reliance on color alone.

Accessibility is easier when public views and action trees are explicit.

## 10. Initial public site policy

Initial public web app:

- static site first;
- no accounts;
- no database;
- no hosted multiplayer;
- load Rust/WASM engine package;
- support local play and bots;
- save/load local replays;
- expose game picker and replay viewer.

Do not add network infrastructure to make the UI seem more serious. A correct static demo beats a fragile multiplayer shell.

## Source notes

See `SOURCES.md`, especially React, MDN SVG, MDN Canvas API, PixiJS, boardgame.io, Rust/WASM, command-log replay, and IP sources.
