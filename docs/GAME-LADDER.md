# GAME LADDER

Status: staged mechanic ladder and game-selection law.

The ladder is mechanics-first, not fan-service-first. Each stage exists to pressure one or a few new mechanics while keeping the rest of the system observable, testable, replayable, and benchmarkable.

Public implementations SHOULD use public-domain/classic games, original games, or permissioned games. Trademark-risk games SHOULD use neutral names. Known-game examples below are candidates, not promises.

## Global entry rules for every stage

A stage MUST NOT begin until the prior stage has:

- native tests passing;
- golden traces;
- replay from seed and command stream;
- random legal bot;
- CLI simulation;
- benchmark coverage;
- UI smoke if the game is exposed in the web app;
- rule coverage notes;
- no game-specific contamination of `engine-core`.

A stage MAY be skipped only by ADR.

## Stage 0: kernel smoke games

### Why this stage exists

Prove the repository skeleton while the games are too small to hide architectural mistakes.

### Mechanics tested

- game identity;
- player identity;
- turn order;
- legal actions;
- action application;
- terminal states;
- deterministic replay;
- basic effects;
- random legal bot;
- CLI runner;
- WASM smoke path.

### Candidate implementations

- Nim;
- Tic-Tac-Toe;
- simple race-to-N counter game.

Rock-Paper-Scissors MAY be used here only if simultaneous choice is intentionally tested; otherwise save simultaneous hidden choice for a later stage.

### Preconditions

- Rust workspace exists;
- `engine-core`, `ai-core`, `wasm-api`, `apps/web`, and docs skeleton exist;
- ADR-0001 for Rust core + WASM shell exists.

### Exit criteria

- at least one tiny game runs in CLI and web;
- random bots complete 100,000 native games without crash;
- replay reproduces final state/effect hashes;
- action/effect logs are visible in developer UI;
- benchmark harness runs.

## Stage 1: deterministic grid placement

### Why this stage exists

Test board coordinates, legal placement constraints, win-pattern detection, and clean legal-move UI.

### Mechanics tested

- grid coordinates;
- occupancy;
- legal placement;
- pattern detection;
- draw detection;
- UI board renderer;
- action previews.

### Candidate implementations

- Four-in-a-Row under a neutral name;
- Gomoku-lite;
- small-board alignment game.

### Preconditions

- Stage 0 exit criteria met;
- effect log can express placements and terminal outcomes.

### Exit criteria

- legal placement never allows occupied/invalid cells;
- pattern detection has unit and property tests;
- simple heuristic bot exists for at least one game;
- web UI makes legal cells obvious;
- no grid primitive enters `engine-core` unless ADR-approved.

## Stage 2: directional effects and multi-piece changes

### Why this stage exists

Test scanning, cascaded state changes, and effect-log granularity.

### Mechanics tested

- directional scanning;
- line captures/flips;
- multi-piece state changes;
- state-delta effects;
- replay animation from semantic effects;
- greedy evaluation.

### Candidate implementations

- Reversi-style directional flip game;
- Ataxx-like original toy if desired.

### Preconditions

- Stage 1 grid model is stable;
- effect log can represent one action causing many piece changes.

### Exit criteria

- every flip/capture appears as explicit effect data or grouped semantic effect with child detail;
- replay visually reconstructs consequences;
- random and greedy bots exist;
- native playout benchmark exists;
- directional logic is tested independently from UI.

## Stage 3: movement, capture, mandatory moves, promotion

### Why this stage exists

Introduce movement and forced continuation without jumping to a full exception-heavy game.

### Mechanics tested

- movement paths;
- capture;
- mandatory actions;
- multi-step actions;
- forced continuation;
- promotion;
- action trees;
- shallow perfect-information search.

### Candidate implementations

- Draughts/checkers variant with documented rule variant;
- simplified chesslike movement microgame.

Chess itself SHOULD wait until the engine is ready for its exception load.

### Preconditions

- action tree model ADR exists;
- UI can render progressive action construction;
- replay can record compound actions.

### Exit criteria

- forced capture/continuation works;
- promotion is tested;
- action tree inspector is usable;
- shallow search works on small positions;
- rules variant is explicitly documented.

## Stage 4: chance, decks, and hidden information

### Why this stage exists

Add deterministic randomness and private views before betting, bluffing, or complex card games.

### Mechanics tested

- shuffled decks;
- deterministic seed;
- draw/reveal/discard zones;
- private hands;
- public/private views;
- hidden-information serialization;
- chance effects;
- no-leak testing.

### Candidate implementations

- War as a deterministic-deck smoke game;
- Blackjack;
- simple draw/discard original game;
- simple trick-taking toy.

### Preconditions

- replay/seed ADR exists;
- visibility/public-view ADR exists;
- serialization tests exist.

### Exit criteria

- seed replay reproduces shuffle and draws;
- unauthorized public views cannot see private cards;
- logs and previews do not leak hidden card identity;
- random bots complete many hands;
- serialization round trips hidden and visible state correctly.

## Stage 5: score tracks, counters, resources, and simple economy

### Why this stage exists

Introduce explicit resource effects and economic decisions before auctions and betting.

### Mechanics tested

- resource counters;
- payments;
- gains;
- score tracks;
- pass/take decisions;
- round cleanup;
- simple valuation bots.

### Candidate implementations

- token-taking auction microgame;
- simple push-your-luck counter game;
- original resource-race game.

### Preconditions

- effect log can represent score/resource changes;
- UI can explain costs and gains.

### Exit criteria

- all resource changes are explicit effects;
- invariant tests preserve total resources where applicable;
- greedy heuristic bot exists;
- UI previews cost and result.

## Stage 6: simultaneous hidden choice and drafting

### Why this stage exists

Test commitments, waiting states, reveal phases, and multiple actors before reaction windows.

### Mechanics tested

- simultaneous choices;
- hidden commitments;
- reveal phases;
- hand passing;
- drafting;
- ordered resolution;
- waiting UI.

### Candidate implementations

- Rock-Paper-Scissors if not used earlier;
- simultaneous card selection toy;
- drafting microgame inspired by common drafting mechanics, with original content.

### Preconditions

- hidden information and private views are proven;
- action log can record multiple commitments before reveal.

### Exit criteria

- simultaneous choices remain hidden until reveal;
- reveal effects replay correctly;
- UI handles waiting states;
- bots can act from their private views only.

## Stage 7: betting and richer public/private card state

### Why this stage exists

Introduce betting only after decks, hidden views, resources, and action trees are solid.

### Mechanics tested

- betting rounds;
- public/private cards;
- pots;
- fold/call/check/bet/raise;
- optional all-in logic;
- hand evaluation;
- imperfect-information bots;
- stochastic simulation.

### Candidate implementations

- poker-lite;
- Texas Hold 'Em as a medium-complexity target after poker-lite.

### Preconditions

- Stages 4 and 5 complete;
- hand evaluator testing strategy exists;
- betting variant scope is written before coding.

### Exit criteria

- betting flow is correct for scoped variant;
- pot accounting has edge-case tests;
- hand evaluator has exhaustive or high-coverage tests;
- random and baseline policy bots exist;
- native hand/playout benchmarks exist.

## Stage 8: bluffing, claims, challenges, and reaction windows

### Why this stage exists

Test hidden claims and interrupt-like windows in smaller games before asymmetric event systems.

### Mechanics tested

- hidden roles/cards;
- claims;
- challenges;
- responses;
- reaction windows;
- conditional resolution;
- action cancellation/replacement;
- public explanation of pending responses.

### Candidate implementations

- original hidden-role bluffing microgame;
- original reaction-window card microgame.

Avoid trademark-forward names and proprietary role/card text.

### Preconditions

- action tree and hidden-info models are stable;
- UI can show pending response windows.

### Exit criteria

- reaction windows are generic enough for at least two games or ADR-approved;
- hidden claim/challenge flow does not leak information;
- bots can respond legally;
- logs explain who may respond and why.

## Stage 9: trick-taking and partnership/variant pressure

### Why this stage exists

Classic card games add following-suit obligations, trick resolution, partnerships, variants, and scoring contracts without requiring proprietary content.

### Mechanics tested

- lead/follow constraints;
- trick capture;
- trump/no-trump variants;
- partnership scoring;
- deal rotation;
- round scoring;
- variant documentation.

### Candidate implementations

- Whist;
- Hearts;
- Spades;
- Euchre only if the chosen variant is documented carefully.

### Preconditions

- Stage 4 decks/hidden information complete;
- Stage 6 simultaneous/ordered reveal concepts are stable.

### Exit criteria

- mandatory follow-suit rules tested;
- scoring variants explicitly documented;
- rule coverage matrix maps every rule to tests;
- baseline policy bot is not purely random.

## Stage 10: cooperative pressure and enemy automation

### Why this stage exists

Cooperative games test shared loss pressure, event decks, role powers, enemy automation, and multi-action turns.

### Mechanics tested

- shared win/loss;
- event deck pressure;
- role powers;
- enemy automation;
- multi-action turn budgets;
- resource coordination;
- scenario setup.

### Candidate implementations

- original cooperative infection/fire/flood microgame;
- original disaster-management grid game.

Do not mimic proprietary board layouts, names, card text, or iconography.

### Preconditions

- decks, resources, and multi-action turns exist;
- private/public view rules are mature.

### Exit criteria

- role powers live in game module;
- event deck automation is replayable;
- shared loss/win tested;
- baseline cooperative bot can complete games legally.

## Stage 11: asymmetric factions and area control

### Why this stage exists

Introduce per-faction action sets and victory conditions without full monster-game scope.

### Mechanics tested

- area control;
- graph maps;
- asymmetric player powers;
- faction-specific actions;
- faction-specific scoring;
- differentiated bots;
- per-faction UI affordances.

### Candidate implementations

- original asymmetric area-control microgame;
- original two-to-four-faction graph-map conflict toy.

### Preconditions

- movement, resources, hidden info, and reaction windows are stable;
- action tree UI can handle faction-specific action menus.

### Exit criteria

- faction-specific behavior remains in game module;
- `engine-core` remains noun-free;
- each faction has random and baseline heuristic policy;
- simulations produce useful metrics.

## Stage 12: event-driven asymmetric campaign/scenario systems

### Why this stage exists

This is the last public ladder stage before private monster-game red-team work.

### Mechanics tested

- event decks with exceptions;
- eligibility/initiative tracks;
- periodic scoring/reset;
- asymmetric victory;
- scenario setup;
- campaign/scenario state;
- scripted policy bots;
- large action trees;
- effect-log readability under complexity.

### Candidate implementations

- original micro-insurgency/political-control design;
- original scenario-based asymmetric event game.

### Preconditions

- Stage 11 complete;
- scripted policy-bot framework exists;
- benchmarks exist for large action trees;
- rule coverage tools exist.

### Exit criteria

- no stage-specific nouns in `engine-core`;
- action tree construction remains usable;
- effect logs remain understandable;
- scripted bots are coherent enough for demo play;
- replay/debug tools can diagnose long games.

## Stage 13: private monster-game red-team experiments

### Why this stage exists

Only after the public ladder succeeds may the architecture be stressed against private licensed monster-game complexity.

### Scope limits

The first private monster-game experiment MUST be a vertical slice, not a full adaptation.

Allowed slice shape:

- one scenario;
- limited factions/sides if needed;
- limited card/event subset;
- no full bot at first;
- no public build;
- no public licensed data;
- explicit rule coverage matrix.

### Preconditions

- Stage 12 complete;
- IP/public-private module policy ADR accepted;
- private repository or private submodule configured;
- public CI does not require private content.

### Exit criteria

- architecture survives without kernel contamination;
- missing abstractions are documented;
- performance remains measurable;
- the project can stop without becoming a ruin.

## Ladder anti-patterns

MUST NOT:

- choose a game because it is impressive rather than mechanically useful;
- jump from grid placement to a monster game;
- generalize from one game;
- add primitives for speculative future needs;
- treat the public ladder as preparation for one specific licensed target;
- copy proprietary presentation;
- hide private licensed work in public builds.

## Source notes

See `SOURCES.md`, especially Ludii, Regular Boardgames / Regular Games, VASSAL, Board Game Arena, public rules sources, and IP sources.
