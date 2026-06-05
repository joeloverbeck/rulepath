# Rulepath Game Ladder

Status: staged mechanic ladder and game-selection law.

The ladder is mechanics-first and product-aware. It exists to build a polished public playable site while earning complexity through observable, testable stages.

This ladder is not fan service. It is not a disguised path toward one private licensed game. It is a sequence of public games and original microgames chosen to prove mechanics, UI contracts, rule enforcement, bots, replay, visibility, and performance.

## 1. Global ladder law

A stage MUST NOT begin until the prior stage has:

- native tests passing;
- rule coverage notes;
- golden traces;
- deterministic replay from seed and command stream;
- random legal bot;
- CLI simulation;
- benchmark coverage;
- serialization coverage;
- UI smoke tests if web-exposed;
- no game-specific contamination of `engine-core`;
- source/IP notes for public games.

A stage MAY be skipped only by ADR.

A public stage MUST use public-domain/classic, original, or permissioned mechanics and content. Trademark-risk classics SHOULD use neutral names and original presentation.

## 2. Ladder overview

| Stage | Public ID / candidate | First proves | Public product role |
|---:|---|---|---|
| 0 | repository skeleton | workspace, CI, ADRs, WASM smoke | not public gameplay |
| 1 | `race_to_n` or Nim | tiny deterministic rule kernel | scaffolding, not impressive milestone |
| 2 | `three_marks` | flat grid placement and simple pattern detection | first board UI smoke |
| 3 | `column_four` | gravity/grid alignment, preview, polish | first “Rulepath is real” public target |
| 4 | `directional_flip` | directional scanning and multi-piece effects | richer animation proof |
| 5 | `draughts_lite` | movement, capture, mandatory continuation | action-tree/progressive UI proof |
| 6 | `high_card_duel`, then `blackjack_lite` if useful | decks, chance, private views | first hidden-info safety proof |
| 7 | `token_bazaar` or `resource_race` | resources, payments, score economy | original microgame portfolio piece |
| 8 | `secret_draft` or commitment/drafting microgame | simultaneous hidden choice, reveal, hand passing | waiting-state/private-bot proof |
| 9 | `poker_lite` | betting, pots, public/private cards | imperfect-info policy proof |
| 10 | `plain_tricks` / Whist-like | follow-suit, tricks, variants | classic card-game depth |
| 11 | `masked_claims` | bluffing, claims, challenges, reactions | reaction-window proof |
| 12 | `flood_watch` / disaster microgame | cooperative event pressure | shared win/loss and automation |
| 13 | `frontier_control` | asymmetric area control | faction/action asymmetry |
| 14 | `event_frontier` | event-driven asymmetric scenario | highest public complexity |
| Appendix | private red-team slice | private stress test only | not public architecture driver |

## 3. Stage 0: repository skeleton and kernel contracts

### Purpose

Create the repository shape without pretending a real game exists.

### Mechanics tested

- workspace structure;
- CI smoke;
- documentation placement;
- ADR process;
- empty `engine-core`, `ai-core`, `game-stdlib`, `wasm-api` crates;
- static web shell loading a placeholder WASM package;
- no game nouns in `engine-core`.

### Required build proof

- `cargo test` runs;
- TypeScript/web smoke runs;
- docs exist;
- ADR directory exists;
- formatter/linter checks run in CI;
- placeholder WASM import path exists.

### Exit criteria

- `FOUNDATIONS.md` and replacement docs are in the repo;
- ADR-0001 through initial boundary ADRs are accepted;
- no game implementation complexity has entered the kernel.

### Not allowed

- real game mechanics in `engine-core`;
- YAML behavior;
- network infrastructure;
- private-game naming.

## 4. Stage 1: tiny smoke game — `race_to_n` or Nim

### Purpose

Prove the rule kernel while the game is too small to hide architecture mistakes.

This is not the first impressive public milestone. It is scaffolding.

### Mechanics tested

- game identity;
- player/seat identity;
- turn order;
- legal actions;
- action validation;
- terminal state;
- deterministic replay;
- basic semantic effects;
- random legal bot;
- CLI simulation;
- WASM smoke path.

### Candidate implementations

- `race_to_n`: players add 1-3 to a counter; exact target or reach target wins depending on typed variant;
- Nim with one or a few heaps;
- another tiny public-domain take-away game.

### Exit criteria

- CLI and web can start a match;
- human vs random bot works;
- random bots complete 100,000 native games without crash;
- replay reproduces final state/effect/action-tree hashes;
- golden traces cover a normal win and a stale/invalid action diagnostic;
- UI displays legal actions and semantic effect log;
- benchmark harness runs;
- no static data contains behavior.

### Not allowed

- generalized pile/deck/board abstractions;
- polished renderer work beyond a minimal public-safe shell;
- multiplayer.

## 5. Stage 2: flat grid placement — `three_marks`

### Purpose

Prove board coordinates, occupancy, legal placement, simple terminal detection, and the first SVG board renderer.

### Mechanics tested

- fixed grid coordinates;
- occupancy;
- legal-only placement;
- simple pattern detection;
- draw detection;
- action previews;
- direct legal target highlighting;
- first keyboard-accessible board actions where practical.

### Candidate implementation

- Tic-Tac-Toe under a neutral/original public name such as `three_marks`.

### Exit criteria

- legal placement never allows occupied or out-of-board cells;
- pattern detection has unit tests and property tests;
- draw states tested;
- random legal bot exists;
- Level 1 baseline bot wins immediately and blocks immediate loss;
- SVG board renderer is clean and responsive;
- public UI uses original assets and neutral naming;
- no grid primitive enters `engine-core`.

### Extraction pressure

Do not extract a generic grid package yet unless an ADR proves it. One grid game is not pressure.

## 6. Stage 3: gravity/grid alignment — `column_four`

### Purpose

This is the first plausible public polish target: the moment Rulepath should feel like a real playable site.

### Mechanics tested

- gravity-constrained placement;
- legal-only column interaction;
- hover/drop preview;
- line detection under gravity;
- clear win/draw/turn feedback;
- effect-log-driven drop and win-line animation;
- simple heuristic bot;
- polished responsive SVG board.

### Candidate implementation

- Four-in-a-Row under a neutral name such as `column_four`.

### Exit criteria

- legal action UI exposes only columns with available space;
- previews show landing row and potential immediate result;
- every placement emits semantic effects sufficient for animation;
- win-line effects identify the winning cells;
- baseline bot can win immediately, block immediate wins, prefer center/control priorities, and explain choices;
- public page is pleasant, responsive, and not debug-dominated;
- replay viewer can step through a full game;
- native playout and bot latency benchmarks exist.

### Extraction pressure

After Stage 2 and Stage 3, repeated coordinate and line-detection shapes may be noted. Extraction still SHOULD wait for Stage 4 unless the shape is already obvious and documented by ADR.

## 7. Stage 4: directional multi-piece effects — `directional_flip`

### Purpose

Prove directional scanning, cascaded piece changes, grouped effect logs, multi-piece animation, greedy bots, and repeated grid pressure.

### Mechanics tested

- directional scanning;
- bracketed flips/captures;
- pass/no-move conditions if included;
- grouped semantic effects with child detail;
- multi-piece animations;
- greedy evaluation;
- repeated grid helper pressure.

### Candidate implementation

- Reversi-style directional flipping under a neutral name such as `directional_flip`.

### Exit criteria

- every flip is represented in effect data or a grouped effect with visible child detail;
- legal move generation matches directional capture rules;
- pass/no-legal-action behavior is tested if part of variant;
- random and greedy bots exist;
- bot explanation can name “gain flips”, “corner”, or other game-specific public concepts;
- replay visually reconstructs action consequences;
- native playout benchmark exists;
- after Stages 2-4, grid/coordinate/pattern helpers may be extracted to `game-stdlib` if the repeated shape is clear.

### Not allowed

- `engine-core` grid types;
- untyped directional selectors in data.

## 8. Stage 5: movement, capture, mandatory continuation — `draughts_lite`

### Purpose

Introduce movement paths, captures, forced actions, promotion if included, and compound action trees without jumping to chess-scale exceptions.

### Mechanics tested

- movement from origin to destination;
- capture;
- mandatory captures;
- forced continuation;
- multi-step action trees;
- progressive action construction;
- promotion if included;
- shallow deterministic search only if benchmark-safe.

### Candidate implementation

- simplified checkers/draughts-like original variant under `draughts_lite`;
- simplified chesslike movement microgame if it isolates the mechanics better.

### Exit criteria

- forced capture and continuation are correct and tested;
- action tree inspector is usable;
- UI guides origin, path, target, continuation, and confirmation;
- stale action diagnostics are graceful;
- Level 1 baseline bot follows forced rules and captures immediate material;
- Level 3 shallow search MAY exist only if latency benchmarks fit;
- rules variant is documented explicitly.

### Not allowed

- full chess exception load;
- generic movement framework in `engine-core`;
- search without benchmarks.

## 9. Stage 6: deck/chance/private view smoke — `high_card_duel`

### Purpose

Add deterministic chance and private/public views before betting, bluffing, or complex card games.

### Mechanics tested

- deterministic shuffle;
- deck/hand/discard zones;
- card draw/reveal effects;
- public/private view projection;
- viewer-filtered logs;
- no-leak previews;
- serialization safety;
- random/baseline bots from private views.

### Candidate implementations

- `high_card_duel`: a War-like deterministic-deck smoke game with original public presentation;
- `blackjack_lite` after the smoke game if it adds useful draw/stand/scoring pressure;
- a simple draw/discard original microgame if better for visibility tests.

### Exit criteria

- seed replay reproduces shuffle and draws;
- unauthorized public views cannot see private cards;
- logs and previews do not leak hidden identity;
- bots receive only allowed private view;
- no-leak tests cover logs, previews, serialization, bot views, and UI payloads;
- web UI handles private hands and public zones without shipping hidden state to unauthorized viewers.

### Extraction pressure

Deck/zone helpers MAY be local first. Extract to `game-stdlib` after a second card game proves the shape.

## 10. Stage 7: resources and score economy — `token_bazaar`

### Purpose

Introduce explicit resource effects and economic decisions before auctions, betting, and cooperative economies.

### Mechanics tested

- counters/resources;
- payments;
- gains;
- score tracks;
- purchase/take/pass choices;
- cleanup phases;
- invariant tests;
- valuation bot.

### Candidate implementation

- original `token_bazaar` or `resource_race` microgame.

### Exit criteria

- all resource changes are explicit semantic effects;
- cost previews are clear and legal-only;
- invariant tests preserve totals where applicable;
- Level 1 valuation bot exists;
- UI explains costs, gains, and score changes;
- benchmark covers legal action generation and resource transitions.

## 11. Stage 8: simultaneous hidden choice and drafting — `secret_draft`

### Purpose

Prove hidden commitments, reveal phases, waiting states, hand passing, ordered resolution, and private bot views.

### Mechanics tested

- simultaneous choices;
- hidden commitments;
- waiting states;
- reveal phases;
- hand passing/drafting;
- ordered resolution;
- public/private bot views.

### Candidate implementation

- original simultaneous card selection microgame;
- original drafting/commitment microgame;
- Rock-Paper-Scissors only if it is useful as a tiny commitment test and not mistaken for the public ladder's core.

### Exit criteria

- commitments remain hidden until reveal;
- reveal effects replay correctly;
- UI shows who is pending without leaking choices;
- bots act only from their allowed private views;
- no-leak tests cover commitments, logs, previews, serialization, and replay exports.

## 12. Stage 9: betting and richer public/private card state — `poker_lite`

### Purpose

Introduce betting after decks, hidden views, resources, and action trees are solid.

### Mechanics tested

- fold/check/call/bet/raise;
- public/private cards;
- pots;
- simple showdown;
- hand evaluator;
- imperfect-information fair bots;
- betting flow diagnostics;
- pot accounting.

### Candidate implementation

- `poker_lite` with deliberately narrow rules;
- Texas Hold ’Em only after `poker_lite` is correct, benchmarked, and documented.

### Exit criteria

- written variant scope exists before coding;
- betting flow is correct for chosen variant;
- pot accounting edge cases are tested;
- hand evaluator has exhaustive or high-coverage tests for scoped hands;
- bots never use hidden opponent cards;
- candidate ranking/explanations are available in dev mode;
- native hand/playout benchmarks exist.

### Not allowed

- casino-real-money features;
- unbounded betting variants;
- omniscient bots;
- ML/RL.

## 13. Stage 10: trick-taking and variant pressure — `plain_tricks`

### Purpose

Prove lead/follow constraints, trick resolution, trump/no-trump variants, partnerships if selected, deal rotation, and round scoring.

### Mechanics tested

- deal rotation;
- lead/follow-suit obligations;
- trick winner resolution;
- trump/no-trump variants;
- partnership scoring if included;
- mandatory follow-suit tests;
- round scoring;
- variant documentation.

### Candidate implementations

- Whist-like neutral implementation;
- Hearts/Spades-like neutral implementation if variant scope is carefully documented.

### Exit criteria

- follow-suit rules are tested against many hands;
- illegal plays are not clickable in normal mode;
- scoring variants are explicitly documented;
- rule coverage matrix maps every rule to tests;
- baseline bot is not purely random;
- hidden information remains safe.

## 14. Stage 11: bluffing, claims, challenges, reaction windows — `masked_claims`

### Purpose

Prove claims, challenges, response windows, conditional resolution, cancellation/replacement, pending response UI, and no-leak behavior.

### Mechanics tested

- hidden roles/cards;
- claims;
- challenges;
- reaction windows;
- pending responses;
- conditional resolution;
- cancellation/replacement;
- logs that explain who may respond and why.

### Candidate implementation

- original hidden-claim microgame;
- original reaction-window card microgame.

### Exit criteria

- reaction windows work in at least two games or have ADR approval;
- pending response UI is clear;
- hidden claim/challenge flow does not leak information;
- bots can respond legally;
- effect logs remain comprehensible under conditional resolution.

### Not allowed

- trademark-forward hidden-role names;
- proprietary role/card text;
- generic reaction window in `engine-core` without proof.

## 15. Stage 12: cooperative event-pressure game — `flood_watch`

### Purpose

Prove shared win/loss, event deck pressure, role powers, enemy automation, multi-action turn budgets, and scenario setup.

### Mechanics tested

- cooperative shared outcome;
- event deck pressure;
- role powers;
- enemy/environment automation;
- multi-action turn budgets;
- resource coordination;
- scenario setup;
- public explanation of automated events.

### Candidate implementation

- original fire/flood/infection/disaster microgame.

### Exit criteria

- role powers live in the game module;
- event automation is replayable and effect-log-driven;
- shared win/loss is tested;
- bot baseline can complete games legally;
- public UI explains event pressure without mimicking proprietary games.

## 16. Stage 13: asymmetric area-control microgame — `frontier_control`

### Purpose

Prove graph maps, area control, faction-specific actions, faction-specific scoring, per-faction UI affordances, and per-faction bots.

### Mechanics tested

- graph map;
- area control;
- asymmetric player powers;
- faction-specific legal actions;
- faction-specific scoring;
- per-faction UI affordances;
- per-faction bots and explanations.

### Candidate implementation

- original asymmetric area-control microgame;
- original two-to-four-faction graph-map conflict toy.

### Exit criteria

- faction-specific behavior remains in game module;
- no faction nouns enter `engine-core`;
- each faction has random and baseline policy;
- simulations produce useful metrics;
- UI makes asymmetry readable.

## 17. Stage 14: event-driven asymmetric scenario system — `event_frontier`

### Purpose

Prove the highest public complexity before any private monster-game red-team work.

### Mechanics tested

- event decks with exceptions;
- eligibility/initiative tracks;
- periodic scoring/reset;
- asymmetric victory;
- scenario setup;
- large action trees;
- scripted policy bots;
- effect-log readability under complexity;
- benchmark pressure for large legal action trees.

### Candidate implementation

- original scenario-based asymmetric event game;
- original micro-insurgency/political-control design with neutral presentation.

### Exit criteria

- action tree construction remains usable;
- effect logs remain understandable;
- scripted bots are coherent enough for demo play;
- replay/debug tools diagnose long games;
- engine-core remains noun-free;
- public portfolio demo stands without private experiments.

## 18. Appendix: private monster-game red-team experiments

Private monster-game experiments are not public ladder stages.

They MAY happen only after Stage 14 is complete and public Rulepath is coherent without them.

Scope limits for the first private experiment:

- private repository, private submodule, or local-only folder;
- one scenario or vertical slice;
- limited factions/sides;
- limited card/event subset;
- no full bot at first;
- no public build;
- no public licensed data;
- no public CI dependency;
- explicit rule coverage matrix;
- explicit kernel-contamination review.

Exit criteria:

- architecture survives without kernel contamination;
- missing abstractions are documented;
- performance remains measurable;
- public project can stop the experiment without damage.

## 19. Ladder anti-patterns

MUST NOT:

- choose a game because it is impressive rather than mechanically useful;
- jump from grid placement to a monster game;
- generalize from one game;
- add primitives for speculative future needs;
- treat the public ladder as preparation for one licensed target;
- copy proprietary presentation;
- hide private licensed work in public builds;
- use the ladder to justify a DSL before repeated Rust pressure exists.

## Source notes

See `SOURCES.md`, especially boardgame.io, VASSAL, Ludii, Regular Boardgames, Regular Games, OpenSpiel, Board Game Arena guidance, and IP sources.
