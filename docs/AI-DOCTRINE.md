# Rulepath AI Doctrine

Status: bot and game-AI law.

Rulepath bots should make public demos enjoyable. The goal is competent, explainable, non-superhuman play that still tries to play well. The goal is not generic superhuman AI.

## 1. Core law

1. Bots MUST consume the same legal action API as human players.
2. Bots MUST choose an `ActionPath` or equivalent legal command through the normal validation path.
3. Bots MUST NOT mutate state directly.
4. Bots MUST NOT choose illegal actions.
5. Bots MUST be deterministic under bot seed, decision limits, rules version, and view.
6. Bots MUST never choose actions using information unavailable to a real player in that seat.
7. Testing tools MAY inspect internal state, but testing tools are not bots and MUST NOT implement the public bot trait.
8. Every game MUST have a Level 0 random legal bot.
9. Every serious public web game SHOULD have at least a Level 1 rule-informed baseline bot.
10. Polished public games SHOULD have Level 2 authored policy bots.
11. ML/RL MUST NOT be used in v1/v2 unless a future ADR explicitly overturns this.
12. MCTS/ISMCTS/Monte Carlo-style bots are not part of the public v1/v2 plan.

## 2. No omniscient bots

Omniscient bots are banned.

A bot MUST NOT receive actual hidden information unless that information is visible to its seat.

This includes:

- opponent hands;
- face-down card identities;
- hidden commitments;
- unrevealed deck order;
- secret roles;
- private logs;
- future random outcomes;
- internal simulation shortcuts that expose true hidden state.

Diagnostics may inspect internal state in local tools. Those diagnostics MUST be labeled as tools, not bots, and MUST NOT choose public-game actions.

## 3. Bot ladder

### Level 0: random legal bot

Required for every game.

Purpose:

- proves legal action generation;
- stress-tests transitions;
- finds crashes;
- creates baseline playouts;
- enables simulation benchmarks;
- gives the UI an automated opponent immediately.

Requirements:

- chooses only from legal action tree/action paths;
- deterministic under bot seed;
- can run native simulations;
- has tests proving no illegal actions over many seeds;
- produces minimal explanation such as “random legal choice”.

### Level 1: rule-informed baseline bot

Required for serious public demos.

Use obvious rule knowledge:

- win immediately if possible;
- block immediate loss;
- obey mandatory rule priorities;
- avoid strategically nonsensical choices that are obviously bad under public rules;
- prefer material/points/resources where the rule connection is direct.

This bot may be weak. It should not look broken.

### Level 2: authored policy bot

Preferred default for polished public games.

Use:

- ordered tactical priorities;
- phase-aware decision trees;
- behavior-tree-like policy nodes where useful;
- lexicographic priorities;
- small scoped scoring functions as tie-breakers;
- game-specific strategy modules;
- deterministic tie-breaking;
- explainable decision reasons;
- debug candidate ranking.

Do not create giant weighted scoring objects that become endless tuning knobs.

### Level 3: shallow deterministic search

MAY be used only for small perfect-information games where benchmarks prove it fits.

Good fits:

- deterministic perfect-information games;
- small action spaces;
- strong terminal/evaluation functions;
- short tactical horizons;
- stable latency under public web budgets.

Allowed techniques:

- minimax;
- alpha-beta;
- iterative deepening with strict deterministic limits;
- transposition tables only if determinism and memory are controlled.

Requirements:

- benchmarked decision latency;
- deterministic limits;
- fallback policy;
- explanation that search was used;
- no hidden-information games unless a future ADR defines a fair information-set approach.

### Future research: MCTS/ISMCTS/Monte Carlo

Not part of public v1/v2.

MAY be explored later only with ADR, benchmarks, and clear reason.

A proposal MUST address:

- playout speed;
- action abstraction;
- deterministic seeding;
- memory and latency;
- hidden-information fairness;
- explanation quality;
- why authored policy or shallow search is insufficient;
- public UX implications.

### Future research: ML/RL

Out of scope for v1/v2.

A future ADR MUST justify:

- why authored/search bots are insufficient;
- training data source;
- reproducibility;
- inference cost;
- model storage;
- browser/server deployment;
- explainability tradeoffs;
- maintenance burden;
- public portfolio value.

## 4. Preferred non-MCTS architecture

Use game-specific policy modules in `games/*` and reusable infrastructure in `ai-core`.

`ai-core` SHOULD provide:

- bot traits;
- random legal bot;
- deterministic bot RNG;
- policy-composition helpers;
- lexicographic priority helpers;
- candidate ranking structures;
- instrumentation;
- decision limits;
- simulation hooks.

`games/*` SHOULD provide:

- game-specific policies;
- phase-aware decision rules;
- tactical priorities;
- style profiles;
- game-specific scoring tie-breakers;
- explanation templates/examples;
- bot tests and benchmarks.

## 5. Bot decision call contract

A bot decision call SHOULD conceptually receive:

```text
choose_action(
  game_id,
  rules_version,
  actor_seat,
  allowed_view_for_actor,
  legal_action_tree,
  deterministic_bot_rng,
  decision_limits,
  instrumentation_sink,
  explanation_sink
) -> bot_decision
```

It MUST NOT receive internal full state for hidden-information games.

Decision output SHOULD include:

- chosen action path;
- policy name/version;
- bot style profile if any;
- seed/counter used for tie-breaking;
- decision timing;
- explanation;
- optional candidate ranking in dev/debug mode;
- whether fallback policy was used.

## 6. Hidden-information bots

Hidden-information bots may use only:

- public information;
- their own private information;
- rules and variant metadata available to all players;
- remembered observations from their own legal view if the bot is stateful;
- belief models over legal possibilities;
- sampled hidden states that are generated from the bot's legal information, not copied from actual hidden state.

They MUST document information access.

They MUST have no-leak tests proving:

- bot views exclude unauthorized hidden state;
- explanations do not reveal hidden state;
- candidate ranking does not reveal hidden state;
- logs/previews/serialization used by bots are safe;
- sampled determinizations do not read actual hidden state.

## 7. Human-looking does not mean stupid

Human-looking means:

- explainable;
- non-superhuman;
- bounded by realistic information;
- occasionally style-varied;
- not perfect under deep tactics;
- still trying to choose good moves.

Bots SHOULD NOT intentionally blunder randomly merely to look human.

A future learner-friendly mode MAY include deliberate simplification, but it must be explicit, documented, and separated from default public bots.

## 8. Style profiles

Style profiles are desired.

They MUST be implemented through:

- policy variation;
- priority ordering;
- risk posture;
- tie-break preferences;
- explanation tone;
- bounded evaluators;
- optional depth/limit adjustments for perfect-information search.

They MUST NOT be implemented through:

- cheating;
- hidden information;
- huge unbounded weights;
- random blunder injection by default;
- static data conditions that become behavior.

Example profiles:

- cautious;
- aggressive;
- greedy;
- blocking/defensive;
- opportunistic;
- risk-seeking;
- learner-friendly if explicitly supported.

## 9. Weight-soup ban

Avoid giant weighted score objects.

Allowed scoring:

- small local tie-breakers;
- bounded evaluators for shallow search;
- explicit lexicographic categories;
- documented coefficients with tests and explanation;
- game-specific heuristics with benchmark coverage.

Forbidden scoring:

- dozens of magic weights with no ordering rationale;
- style profiles that merely multiply weights;
- hidden tactical conditions in static data;
- scoring that cannot produce clear decision explanations.

## 10. Simulation requirements

Every game MUST support native CLI simulation.

Simulation SHOULD measure:

- games completed;
- terminal outcomes;
- games ended by turn cap;
- illegal action attempts by bots;
- invariant violations;
- average turn count;
- playout throughput;
- bot decision latency;
- seed and command stream of failures.

Simulation failures MUST emit enough data to replay and reduce the failing seed.

## 11. Benchmark requirements

Every non-trivial bot SHOULD have benchmarks for:

- legal action generation time;
- action selection latency;
- playout throughput;
- allocations if practical;
- serialization/replay overhead where relevant.

Benchmark from native Rust first. WASM smoke benchmarks are secondary.

## 12. Bot documentation

Every non-random bot MUST document:

- strategy level;
- policy name/version;
- information access;
- decision order;
- style profiles;
- scoring terms if any;
- tie-break method;
- known weaknesses;
- benchmark numbers;
- legality tests;
- no-leak tests if hidden information exists;
- explanation examples;
- whether it is suitable for public play.

## 13. Anti-patterns

MUST NOT:

- bypass legal action generation;
- mutate state directly;
- use actual hidden state;
- call a diagnostic full-state inspector a bot;
- tune endless weights with no tests;
- hide tactical logic in static data;
- call browser code from bot hot loops;
- add ML because it sounds impressive;
- benchmark only in the browser;
- make a bot for a private monster game before smaller policy bots work;
- present a cheating bot as fair;
- introduce MCTS/ISMCTS in public v1/v2 by stealth.

## Source notes

See `SOURCES.md`, especially OpenSpiel, Board Game Arena bots and AI guidance, Board Game Arena zombie mode, Regular Boardgames, Regular Games, and testing/benchmark sources.
