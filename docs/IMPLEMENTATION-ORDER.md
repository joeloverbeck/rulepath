# IMPLEMENTATION ORDER

Status: staged build order with gates.

This is not a ticket plan. It is the order in which complexity is allowed to enter the project.

Online multiplayer is not part of this order.

## Gate 0: repository skeleton

### Build

- Create Rust workspace.
- Add `engine-core`.
- Add `game-stdlib` as mostly empty placeholder.
- Add `ai-core`.
- Add `wasm-api`.
- Add `apps/web` TypeScript shell.
- Add `tools/simulate` or equivalent.
- Add `docs/adr`.
- Add this document set.

### Required ADRs

- Rust core + WASM shell.
- No YAML by default.
- Rust-first game behavior.
- Static site first.
- Multiplayer deferral.
- IP/public-private module policy.

### Exit gate

- `cargo test` runs.
- web shell can load a placeholder WASM package.
- CI can run formatting and test smoke.
- docs are present.

## Gate 1: tiny kernel smoke game

### Build

Implement one tiny game extremely well, preferably Nim or Tic-Tac-Toe.

Required:

- setup;
- legal actions;
- action application;
- effect log;
- terminal outcome;
- deterministic seed handling even if no randomness is used;
- command log;
- replay;
- random legal bot;
- CLI simulation;
- golden trace;
- native benchmark;
- minimal web display;
- debug panels.

### Exit gate

- random bots complete 100,000 games native without crash;
- replay reproduces state/effect hashes;
- browser displays legal actions and effect log;
- no game-specific logic in `engine-core` beyond contracts.

## Gate 2: CLI, trace, and benchmark hardening

### Build

- Improve CLI runner.
- Add trace serialization.
- Add replay checker.
- Add state/effect/action-tree hashes.
- Add benchmark harness.
- Add failure seed output.

### Exit gate

- a failing simulation can be replayed from seed and command log;
- benchmark command produces stable report;
- trace format includes game/rules version.

## Gate 3: WASM API and static web shell

### Build

- Expose batched WASM API.
- Implement game picker.
- Implement match setup.
- Implement public view rendering for the smoke game.
- Implement legal action selection.
- Implement effect log panel.
- Implement replay controls.

### Exit gate

- static site can be served with no backend;
- smoke game can be played human vs random bot;
- hotseat works if applicable;
- bot vs bot replay works;
- no rule legality exists in TypeScript.

## Gate 4: first abstract grid games

### Build

Implement Stage 1 and Stage 2 games:

- Four-in-a-Row under neutral naming;
- directional flip/Reversi-style game.

Add or extract grid helpers only after repeated pressure.

### Exit gate

- grid placement and directional effects tested;
- legal move UI is clear;
- effect logs animate multi-piece consequences;
- random and baseline heuristic bots exist;
- native benchmarks exist;
- extraction into `game-stdlib` is justified by two games or ADR.

## Gate 5: movement and action trees

### Build

Implement a draughts/checkers-like variant or simplified movement/capture game.

Required:

- action tree ADR accepted;
- progressive action construction;
- forced continuation support;
- promotion if included;
- movement/capture tests;
- UI action wizard;
- shallow search experiment if appropriate.

### Exit gate

- action trees work in CLI and web;
- forced multi-step actions replay correctly;
- developer inspector shows tree and selected path;
- shallow search does not violate latency budgets.

## Gate 6: cards, chance, and hidden information

### Build

Implement Stage 4 card/chance games:

- War or simple card smoke game;
- Blackjack;
- simple draw/discard or trick-taking toy.

Required:

- deterministic shuffle;
- deck/hand zones;
- public/private views;
- no-leak tests;
- replay from seed;
- serialization tests;
- random bots.

### Exit gate

- hidden information never leaks in public views/logs/previews;
- seed replay reproduces draws;
- bots can simulate many hands;
- web UI handles private hands and public zones.

## Gate 7: resources, drafting, and simultaneous choice

### Build

Implement original microgames for:

- resources/score tracks;
- simultaneous hidden choice;
- drafting/hand passing;
- reveal phases.

### Exit gate

- waiting/reveal UI works;
- resource changes are explicit effects;
- simultaneous choices remain hidden until reveal;
- bots act from allowed information only.

## Gate 8: betting/trick-taking/variants

### Build

Implement richer card mechanics:

- poker-lite;
- Texas Hold 'Em only after poker-lite;
- Whist/Hearts/Spades-style trick-taking where appropriate.

Required:

- written variant scope before coding;
- pot/accounting tests for betting games;
- hand evaluator tests;
- follow-suit/trick tests for trick-taking games;
- rule coverage matrix.

### Exit gate

- betting/trick rules are correct for chosen variants;
- bots finish games legally;
- hidden information remains safe;
- native benchmarks exist;
- UI remains understandable.

## Gate 9: bluffing, claims, and reaction windows

### Build

Implement original bluffing/reaction microgames.

Required:

- reaction-window model;
- claims/challenges;
- pending response UI;
- hidden-role/card no-leak tests;
- baseline policy bot.

### Exit gate

- reaction windows work for at least two games or ADR approves model;
- logs explain pending responses;
- bots can respond legally;
- kernel remains generic.

## Gate 10: cooperative pressure and enemy automation

### Build

Implement an original cooperative event-pressure game.

Required:

- shared win/loss;
- role powers;
- enemy automation;
- event deck pressure;
- multi-action turn budget;
- scenario setup.

### Exit gate

- role powers live in game module;
- enemy automation is replayable;
- cooperative bot baseline exists;
- public UI is playable and clear.

## Gate 11: asymmetric area control

### Build

Implement an original asymmetric microgame.

Required:

- graph map;
- area control;
- faction-specific actions;
- faction-specific scoring;
- faction-specific UI affordances;
- per-faction policies.

### Exit gate

- no faction nouns in `engine-core`;
- each faction has random and baseline bot;
- simulations produce metrics;
- effect logs remain readable.

## Gate 12: event-driven asymmetric scenario game

### Build

Implement an original scenario/event-driven asymmetric game.

Required:

- event deck;
- eligibility/initiative track;
- periodic scoring/reset;
- asymmetric victory;
- scripted policy bots;
- large action tree benchmarks;
- robust rule coverage.

### Exit gate

- action tree UI remains usable;
- scripted bots are coherent enough for demos;
- replay/debug tools diagnose long games;
- engine-core is still clean;
- public portfolio demo can stand without private monster experiments.

## Gate 13: private monster-game red-team experiment

### Build

Only after Gate 12, a private licensed monster-game experiment MAY be attempted as a local/private vertical slice.

Required:

- private repository/submodule/local folder;
- public CI exclusion;
- no public assets/data/modules;
- no public build bundling;
- IP review;
- strict vertical slice scope.

### Exit gate

- no kernel contamination;
- missing abstractions documented;
- performance measurable;
- project remains coherent if this experiment stops.

## Cross-gate rules

At every gate:

- update docs;
- add tests before trusting behavior;
- add golden traces;
- run simulations;
- benchmark native first;
- keep public/private IP boundaries;
- do not add YAML by default;
- do not introduce a DSL without ADR;
- do not let agents make architecture unattended.

## Stop conditions

Stop and reassess if:

- `engine-core` gains game nouns;
- static data becomes procedural;
- browser UI starts deciding legality;
- bots bypass legal action API;
- replay stops being deterministic;
- performance is unknown;
- public builds contain licensed data;
- a monster game starts driving architecture.

## Source notes

See `SOURCES.md`, especially Rust/WASM, boardgame.io, OpenSpiel, Regular Boardgames / Regular Games, Board Game Arena bot/AI guidance, deterministic replay, and IP sources.
