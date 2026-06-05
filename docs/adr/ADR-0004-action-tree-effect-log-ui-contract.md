# ADR-0004: Action trees, semantic effects, and the UI legality contract

## Status

Accepted.

## Context

Rulepath must support simple legal moves and later compound actions without letting the UI become a second rules engine. It also needs pleasing animations that explain what happened. Inferring animation causality from state diffs is fragile, especially when one action causes multiple semantic consequences.

## Decision

Rust owns legality and exposes legal choices as action trees/action paths.

Simple games MAY present flat legal actions. Compound games MUST support progressive action construction through an action tree or equivalent typed progressive model.

The player-facing UI MUST make legal choices obvious and MUST NOT make illegal choices clickable in normal player mode.

Rust emits semantic effect logs after applying validated commands. The UI schedules animations from semantic effects and settles to the new public view.

The renderer owns visuals, hit targets, animation timelines, resize behavior, and mapping public views/effects to visuals. The renderer MUST NOT decide legality.

React/TypeScript owns shell, layout, panels, settings, replay controls, accessibility wrappers, and WASM integration. React/TypeScript MUST NOT implement rule legality.

## Consequences

Positive:

- legal-only UI is possible without duplicating rules;
- compound actions can be represented without giant flat action lists;
- replay and animation share semantic effects;
- debug tooling can inspect action trees and effect logs.

Negative:

- the action/effect APIs must be designed earlier than a purely visual prototype would require;
- every game must emit enough semantic effects for useful logs and animations;
- stale action diagnostics must be handled carefully.

Migration consequences:

- state-diff animation may exist only as temporary diagnostic tooling;
- games that expose web UI must supply UI metadata and explanation templates keyed to Rust actions/effects.

## Alternatives considered

### UI computes legal moves

Rejected. It duplicates rules and creates desync, hidden-info, and testability risks.

### UI infers animation from state diffs

Rejected as a primary model. Diffs are useful for diagnostics, but they do not reliably encode causality.

### PixiJS-first renderer

Rejected. SVG is the v1 default for modest early games; Canvas/PixiJS remain measured upgrade paths behind a renderer boundary.
