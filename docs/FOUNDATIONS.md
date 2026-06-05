# Rulepath Foundations

Status: repository constitution. This document is project law until superseded by an ADR.

Rulepath is the project name. The repository name is `rulepath`.

Rulepath is first a public playable portfolio-quality web app for card and board games. It is second a route toward eventually stress-testing private complex tabletop implementations. It is third a long-term engine research project. When these goals compete, the public playable product wins.

Rulepath MUST NOT present itself as a near-term arbitrary tabletop engine. Complexity is earned by a staged game ladder, not claimed up front.

## 1. Product identity

Rulepath exists to make a visitor think: this is a polished playable game site, and the architecture behind it is serious.

The public app MUST provide:

- pleasing, responsive, consumer-polished presentation;
- clear feedback for every player action;
- informative and satisfying animation;
- deterministic Rust rule enforcement;
- legal-move UI that lets the player perform only legal moves in normal mode;
- competent, explainable, non-superhuman bots;
- replay, debug, and trace support;
- public/private visibility safety;
- original assets and neutral presentation;
- clean architecture suitable for a public portfolio project.

The public app MUST NOT feel like a thin engine demo with debug panels pasted on top. Debug tools are required early, but the default public experience is play first.

## 2. Priority order

Use this table when making tradeoffs.

| Priority | Goal | Consequence |
|---:|---|---|
| 1 | Polished public playable site | Presentation, onboarding, feedback, and bot quality matter. |
| 2 | Correct deterministic rules | Rust validates everything. Replay and tests are first-class. |
| 3 | Clean kernel boundary | `engine-core` stays generic and small. |
| 4 | Future multiplayer readiness | Command logs, views, validation, and serialization are preserved. Networking is deferred. |
| 5 | Long-term engine research | DSLs, generalized systems, MCTS, ML, and private stress tests wait for evidence. |

## 3. Core law

### 3.1 Rust owns behavior

Rust MUST own:

- setup;
- legal action generation;
- action validation;
- state transitions;
- scoring and terminal detection;
- deterministic RNG;
- effect emission;
- replay and trace hashing;
- public/private view projection;
- serialization contracts;
- bot hot loops and bot choices.

TypeScript MUST own only browser shell, layout, presentation, renderer integration, menus, replay controls, settings, and accessibility wrappers.

The UI MUST NOT decide legality.

### 3.2 Engine-core is not a game engine full of game concepts

`engine-core` MUST contain only generic contracts and infrastructure:

- identities: game, match, rules version, player, seat;
- deterministic RNG contracts;
- action trees, action paths, commands, diagnostics;
- semantic effect log contracts;
- replay, checkpoint, version, and hash contracts;
- visibility and public-view contracts;
- serialization boundaries;
- generic errors and diagnostics.

`engine-core` MUST NOT contain:

- game nouns;
- board, card, faction, scenario, pile, deck, coordinate, zone, resource, combat, trick, suit, pot, or role concepts;
- rule-specific helpers;
- bot strategy;
- UI layout or animation timing;
- networking, accounts, persistence, database, matchmaking;
- licensed data or private-game names.

### 3.3 Game-specific logic belongs in games

`games/*` owns:

- game nouns;
- setup;
- legal action generation;
- validation;
- transitions;
- scoring;
- terminal detection;
- visibility projection;
- semantic effect emission;
- game-specific bots;
- game-specific UI metadata;
- tests, traces, benchmarks, rules docs, source notes.

Game-specific code inside a game module is correct. Game-specific code inside `engine-core` is a boundary failure.

### 3.4 Game-stdlib is earned

`game-stdlib` may eventually contain reusable mechanics such as grids, coordinates, line detection, zones, piles, decks, tracks, resources, graph maps, simultaneous-choice helpers, and reaction-window helpers.

A primitive SHOULD enter `game-stdlib` only after at least two implemented games need the same shape, or after an ADR proves earlier extraction is necessary.

`game-stdlib` MUST NOT become a second kernel.

### 3.5 Static data is content, not behavior

Typed Rust owns behavior in v1.

Static data MAY include manifests, labels, icon IDs, theme tokens, board labels, coordinate labels, public/original card IDs, deck composition, initial setup constants, scoring tables, typed variant selections, localization strings, explanation templates, UI metadata, golden traces, and benchmark fixtures.

Static data MUST NOT include selectors as strings, rule branches, loops, triggers, conditional card effects, tactical AI conditions, exception logic, mandatory-action rules, or nested untyped objects interpreted as behavior.

Variant flags in data are allowed only when they deserialize into typed Rust enums whose behavior is implemented and tested in Rust.

Card or effect identity may be data-listed only when it maps to a compiled, documented, tested Rust enum or constructor. Data may say that a public original card has `EffectId::DrawTwo` only if `DrawTwo` is real Rust behavior. Data must not define what “draw two” means procedurally.

### 3.6 No DSL at project start

Rulepath starts with typed Rust game modules, not a DSL.

A future DSL MAY be proposed only after multiple Rust game modules show repeated, painful, stable behavior shapes that typed Rust plus `game-stdlib` cannot maintain cleanly.

A future DSL MUST be typed, compiled or lowered, source-span-aware, deterministic, formatted, linted, versioned, tested, benchmarked, documented with examples and anti-examples, and unable to silently depend on hidden defaults.

A DSL MUST NOT be introduced to make one monster game possible.

### 3.7 UI contract

React/TypeScript owns the application shell. Rust/WASM owns rules. The renderer owns visual representation and interaction mapping. The renderer MUST NOT decide legality.

V1 SHOULD use React + SVG for boards/cards because the early ladder has modest visual object counts, SVG is inspectable and scalable, and accessibility/debug overlays are easier. Canvas or PixiJS MAY replace or supplement SVG only after measured needs justify them.

Animation MUST be effect-log-driven:

```text
player chooses legal action path
  -> Rust validates and applies command
  -> Rust emits semantic effects
  -> UI schedules animations from effects
  -> renderer settles to the new public view
```

The UI MUST NOT infer causality from state diffs except in diagnostics.

### 3.8 Legal move UX

In normal player mode, illegal moves MUST NOT be clickable.

Simple games MAY highlight all legal cells, cards, or actions directly.

Compound games MUST use progressive construction:

```text
choose action type
  -> choose target
  -> choose pieces/cards/resources
  -> preview costs/effects
  -> confirm
```

Learning/debug mode MAY show disabled illegal choices with reasons. Normal mode SHOULD avoid flooding players with irrelevant illegal choices.

Stale actions MUST be rejected gracefully by Rust diagnostics.

Hidden information MUST NOT leak through UI metadata, previews, logs, debug panels, public views, serialized views, or bots.

### 3.9 Bot law

Bots MUST consume the same legal action API as human players. Bots MUST NOT mutate state directly. Bots MUST NOT choose illegal actions.

Bots MUST never choose actions using information that a real player in that seat could not know. Testing tools may inspect internal state, but such tools are not bots and MUST NOT implement the public bot trait.

Every game MUST have a Level 0 random legal bot. Public demos SHOULD have at least a Level 1 rule-informed bot. Polished public games SHOULD use Level 2 authored policy bots. Level 3 shallow deterministic search MAY be used only for small perfect-information games where benchmarks prove it fits.

MCTS, ISMCTS, Monte Carlo-style bots, ML, and RL are not part of the public v1/v2 plan. They require future ADRs, benchmarks, and a clear reason.

Bots SHOULD be explainable and non-superhuman. They should not be intentionally stupid by random blunders unless a future explicit learner-friendly mode says so.

Avoid weight soup. Prefer ordered tactical priorities, phase-aware decision trees, behavior-tree-like policy nodes, lexicographic priority evaluation, small scoped scoring tie-breakers, deterministic tie-breaking, and decision explanations.

### 3.10 Testing and benchmarking law

A game is not implemented when it appears to work in the UI. A game is implemented when rules, replay, visibility, bots, docs, traces, and benchmarks are covered.

Every game MUST have typed Rust rules, structured rules docs, source notes, a rule coverage matrix, unit tests, rule tests, golden traces, property/invariant tests, simulation/fuzz tests, deterministic replay tests, serialization tests, AI legal-action tests, CLI simulation support, benchmark coverage, UI metadata, and UI smoke tests once web-exposed.

Hidden-information games additionally MUST have public/private view tests and no-leak tests for logs, previews, serialization, bot views, and UI payloads.

Bot games additionally MUST have bot legality tests over many seeds, bot decision latency benchmarks, bot documentation, and explanation examples.

When tests fail, the protocol is:

1. determine whether the failing tests are still valid;
2. determine whether the issue is in the system under test or the test suite;
3. fix the issue;
4. add or update regression coverage;
5. report what changed.

Tests MUST NOT be deleted, weakened, or rewritten merely to get green output.

### 3.11 Multiplayer law

The initial public app is static and local-first:

- no accounts;
- no database;
- no hosted multiplayer;
- no matchmaking;
- human vs bot;
- local hotseat;
- bot vs bot replay;
- replay viewer.

Future hosted multiplayer MUST use an authoritative Rust server. Browser clients MUST NOT own authoritative state.

Rulepath preserves future multiplayer through deterministic command logs, public/private views, action validation, serialization, hashes, and replay, not by adding networking to v1.

### 3.12 IP law

Public games MUST be public-domain/classic, original, or permissioned.

Public implementations MUST use neutral names for trademark-risk classics, original rules summaries, original assets, and source notes.

Public files MUST NOT copy rulebook prose, proprietary card text, proprietary assets, board art, icons, screenshots, trade dress, licensed data, or hidden licensed modules.

Private complex-game stress tests may exist later only in private repositories, private submodules, or local-only folders. They MUST NOT drive public architecture, contaminate public naming, or appear in public builds.

If code or data ships to an unauthorized browser, it has shipped.

### 3.13 Agent law

Claude Code, Codex, and similar tools are accelerators, not unattended architects.

Agent tasks MUST be bounded, measurable, test-driven, explicit about non-goals and forbidden kernel changes, and should request complete files or coherent complete sections, not diffs.

Agents MUST NOT invent major architecture, generalize from one game, add game nouns to `engine-core`, add YAML behavior, create a DSL without ADR, implement private licensed content in public files, rewrite tests blindly, or optimize without benchmarks.

## 4. Foundation gates

| Gate | Must be true before moving on |
|---|---|
| Repository skeleton | Workspace, docs, ADR folder, empty crates, CI smoke, web shell smoke. |
| Tiny smoke game | CLI and web playable, random bot, replay, traces, benchmark, no kernel contamination. |
| First public polish game | `column_four` or equivalent has attractive SVG UI, legal-only interaction, effect-log animation, baseline bot, replay viewer. |
| Hidden information | Public/private views, filtered logs, filtered previews, serialization and bot-view no-leak tests pass. |
| Complex action trees | Progressive UI, diagnostics, benchmarks, replay, and action-tree inspector all work. |
| Research/private work | Public ladder already stands on its own. Private work is isolated and optional. |

## 5. Near-term non-goals

Rulepath v1/v2 does not include:

- arbitrary tabletop support;
- no-code game authoring;
- a DSL;
- YAML behavior;
- hosted multiplayer;
- accounts;
- matchmaking;
- server persistence;
- MCTS/ISMCTS public bots;
- ML/RL bots;
- private licensed games in public files;
- a public app that looks like a private monster-game project.

## Source notes

See `SOURCES.md`, especially boardgame.io, VASSAL, Ludii, Regular Boardgames / Regular Games, OpenSpiel, Board Game Arena bot and AI-development guidance, React/SVG/Canvas/PixiJS, Rust/WASM, Serde data-format documentation, and copyright/IP sources.
