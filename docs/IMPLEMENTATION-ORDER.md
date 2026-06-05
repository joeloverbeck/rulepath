# Rulepath Implementation Order

Status: staged build order with gates.

This is not a ticket plan. It is the order in which complexity is allowed to enter Rulepath.

The initial public app is static and local-first. Online multiplayer, accounts, databases, matchmaking, public server deployment, DSL work, MCTS/ISMCTS, ML/RL, and private monster-game work are not part of the v1/v2 implementation order.

## Gate 0: repository skeleton

### Build

- Create Rust workspace.
- Add `engine-core` with only placeholder generic contracts.
- Add `game-stdlib` as an empty or near-empty placeholder.
- Add `ai-core` with only trait placeholders and no game strategy.
- Add `wasm-api` placeholder.
- Add `apps/web` React/TypeScript shell.
- Add `tools/simulate` placeholder.
- Add `docs/adr`.
- Add this foundation document set.
- Add CI smoke for formatting, tests, and web build placeholder.

### Required ADRs

- ADR-0001 Rust core + WASM web shell.
- ADR-0002 engine/data/game boundary.
- ADR-0003 typed Rust behavior, no DSL, no YAML by default.
- ADR-0004 action tree + effect-log UI contract.
- ADR-0005 bot policy and no omniscient bots.
- ADR-0006 static local-first app and multiplayer deferral.
- ADR-0007 public game ladder and IP/private isolation.

### Exit gate

- `cargo test` runs.
- Web shell can load a placeholder WASM package.
- CI runs format/test/build smoke.
- Docs are present.
- `engine-core` has no game nouns.

## Gate 1: tiny kernel smoke game

### Build

Implement Stage 1 `race_to_n` or Nim extremely well.

Required:

- setup;
- typed state;
- legal actions;
- validation;
- command application;
- semantic effects;
- terminal outcome;
- deterministic seed handling even if no randomness is used;
- command log;
- replay;
- random legal bot;
- CLI simulation;
- golden traces;
- native benchmark;
- minimal web display;
- dev action/effect panels.

### Exit gate

- random bots complete 100,000 native games without crash;
- replay reproduces state/effect/action-tree hashes;
- browser displays legal actions and effect log;
- human vs random bot works;
- no game-specific logic enters `engine-core` beyond contracts.

## Gate 2: trace, replay, and benchmark hardening

### Build

- Improve CLI runner.
- Add trace serialization.
- Add replay checker.
- Add state/effect/action-tree hashes.
- Add benchmark harness.
- Add failure seed and command-stream output.
- Add seed-reduction plan, even if primitive.

### Exit gate

- failing simulation can be replayed from seed and command log;
- benchmark command produces a stable report;
- trace format includes game id, rules version, engine version, seed, options, command stream, and hashes;
- golden traces fail loudly on rule drift.

## Gate 3: WASM API and static web shell

### Build

- Expose batched WASM API.
- Implement game picker.
- Implement match setup.
- Implement public view rendering for the smoke game.
- Implement legal action selection.
- Implement effect log panel.
- Implement replay controls.
- Add visible dev toggle for inspectors.

### Exit gate

- static site can be served with no backend;
- smoke game can be played human vs random bot;
- hotseat works if applicable;
- bot vs bot replay works;
- dev toggle shows seed, version, legal action count, command/effect log;
- no rule legality exists in TypeScript.

## Gate 4: flat grid placement — `three_marks`

### Build

Implement `three_marks` / Tic-Tac-Toe-style neutral game.

Required:

- local grid coordinates in game module;
- occupancy rules;
- pattern detection;
- draw detection;
- direct legal cell highlighting;
- SVG board renderer;
- random legal bot;
- rule-informed baseline bot;
- golden traces and property tests.

### Exit gate

- occupied cells are never legal;
- win/draw detection is covered;
- baseline bot wins/blocks immediate threats;
- keyboard action selection exists where practical;
- no grid primitive enters `engine-core`.

## Gate 5: first public polish target — `column_four`

### Build

Implement `column_four` as the first serious public showcase.

Required:

- gravity-constrained placement;
- legal-only column interaction;
- hover/drop preview;
- line detection;
- effect-log-driven drop/win animation;
- attractive responsive SVG renderer;
- Level 1 and preferably Level 2 policy bot;
- replay viewer polish;
- UI smoke tests.

### Exit gate

- public page looks like a polished playable game, not an engine demo;
- legal columns only are clickable;
- win-line animation comes from semantic effects;
- bot explanations are visible in dev mode;
- native benchmarks and UI smoke tests pass;
- no grid abstractions are extracted unless justified.

## Gate 6: directional flip and earned grid extraction

### Build

Implement `directional_flip`.

Then decide whether to extract grid helpers from `three_marks`, `column_four`, and `directional_flip` into `game-stdlib`.

Required:

- directional scanning tests;
- multi-piece effect logs;
- grouped animation support;
- greedy bot;
- trace/replay coverage;
- extraction ADR or two-game/three-game pressure note if helpers move.

### Exit gate

- flip/capture effects replay and animate correctly;
- benchmarks exist;
- any extracted helper is narrow, tested, documented, and back-ported;
- `engine-core` remains noun-free.

## Gate 7: movement and action trees — `draughts_lite`

### Build

Implement movement/capture/mandatory-continuation game.

Required:

- action tree model finalized enough for compound moves;
- progressive action construction UI;
- forced capture and continuation tests;
- promotion tests if included;
- stale action diagnostics;
- action tree inspector;
- shallow search only if benchmark-safe.

### Exit gate

- action trees work in CLI and web;
- forced multi-step actions replay correctly;
- UI shows construction path and confirmation;
- developer inspector shows tree and selected path;
- shallow search, if present, fits latency budget.

## Gate 8: cards, chance, and hidden information

### Build

Implement `high_card_duel`; add `blackjack_lite` only if the smoke game is solid and adds useful pressure.

Required:

- deterministic shuffle;
- deck/hand/discard zones in game module or earned stdlib helper;
- draw/reveal/discard effects;
- public/private views;
- viewer-filtered logs;
- no-leak tests;
- serialization tests;
- bot-view tests;
- web UI for private hands and public zones.

### Exit gate

- hidden information never leaks through public views, logs, previews, serialization, bot views, or UI payloads;
- seed replay reproduces shuffle and draws;
- bots simulate many hands legally;
- web UI handles hidden and visible cards safely.

## Gate 9: resources, simultaneous choice, and drafting

### Build

Implement original microgames for:

- resources and score economy (`token_bazaar` / `resource_race`);
- simultaneous hidden choice;
- drafting/hand passing;
- reveal phases.

### Exit gate

- resource effects are explicit;
- waiting/reveal UI works;
- simultaneous choices remain hidden until reveal;
- bots act from allowed views only;
- invariant tests and no-leak tests pass;
- valuation/policy bots have explanations.

## Gate 10: richer card games — betting and trick-taking

### Build

Implement:

- `poker_lite` before any Hold ’Em variant;
- one Whist-like or Hearts/Spades-like trick-taking neutral implementation when ready.

Required:

- written variant scope before coding;
- pot/accounting tests for betting;
- hand evaluator tests;
- follow-suit/trick tests;
- public/private visibility tests;
- bot legality and latency benchmarks;
- rule coverage matrix.

### Exit gate

- betting/trick rules are correct for chosen variants;
- bots finish games legally without hidden-state cheating;
- hidden information remains safe;
- UI remains understandable;
- native benchmarks exist.

## Gate 11: bluffing, claims, challenges, reaction windows

### Build

Implement original bluffing/reaction microgames.

Required:

- claim/challenge model;
- pending response UI;
- conditional resolution;
- cancellation/replacement if scoped;
- no-leak tests;
- baseline policy bot;
- reaction-window extraction only after repeated pressure or ADR.

### Exit gate

- logs explain who may respond and why;
- bots respond legally;
- hidden claims do not leak;
- reaction-window model is backed by at least two games or ADR;
- kernel remains generic.

## Gate 12: cooperative pressure and enemy automation

### Build

Implement original cooperative event-pressure game.

Required:

- shared win/loss;
- event deck;
- role powers;
- enemy/environment automation;
- multi-action turn budget;
- scenario setup;
- cooperative bot baseline;
- replayable automation effects.

### Exit gate

- role powers live in game module;
- automation is deterministic and replayable;
- cooperative bot baseline completes games legally;
- public UI explains events and shared outcome clearly.

## Gate 13: asymmetric area control

### Build

Implement original asymmetric graph-map/area-control microgame.

Required:

- graph map;
- area control;
- faction-specific actions;
- faction-specific scoring;
- per-faction UI affordances;
- per-faction bots;
- metrics from simulations.

### Exit gate

- no faction nouns in `engine-core`;
- each faction has random and baseline bot;
- effect logs remain readable;
- simulations produce useful balance and length metrics.

## Gate 14: event-driven asymmetric scenario game

### Build

Implement original public scenario/event-driven asymmetric game.

Required:

- event deck with exceptions;
- eligibility/initiative tracks;
- periodic scoring/reset;
- asymmetric victory;
- scenario setup;
- scripted policy bots;
- large action tree benchmarks;
- robust rule coverage;
- replay/debug tools for long games.

### Exit gate

- action tree UI remains usable;
- scripted bots are coherent enough for demos;
- replay/debug tools diagnose long games;
- engine-core is still clean;
- public portfolio demo stands without private experiments.

## Gate P: private monster-game red-team experiment

This gate is optional, private, and post-public-ladder.

### Preconditions

- Gate 14 complete;
- public app is coherent without private work;
- IP/public-private policy ADR accepted;
- private repo/submodule/local folder configured;
- public CI excludes private content;
- public builds do not bundle private code/data/assets.

### Scope

- vertical slice only;
- one scenario or small subset;
- limited factions/cards/events;
- no public build;
- no full bot at first;
- strict kernel-contamination review.

### Exit gate

- no kernel contamination;
- missing abstractions documented;
- performance measurable;
- public project remains coherent if private work stops.

## Cross-gate rules

At every gate:

- update docs;
- validate tests before fixing failures;
- add regression coverage for bugs;
- add/update golden traces intentionally;
- run simulations;
- benchmark native first;
- preserve public/private IP boundaries;
- do not add YAML by default;
- do not introduce a DSL without ADR;
- do not let agents make architecture unattended;
- keep public UI pleasant and default-debug-free.

## Stop conditions

Stop and reassess if:

- `engine-core` gains game nouns;
- static data becomes procedural;
- TypeScript starts deciding legality;
- bots bypass legal action API;
- bots use hidden information unavailable to their seat;
- replay stops being deterministic;
- performance is unknown;
- public builds contain licensed data;
- private monster-game work starts driving public architecture;
- the public app feels like a debug console rather than a game site.

## Source notes

See `SOURCES.md`, especially boardgame.io, VASSAL, Ludii, Regular Boardgames, Regular Games, OpenSpiel, Board Game Arena guidance, Rust/WASM, deterministic replay, and IP sources.
