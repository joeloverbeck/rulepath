# AI DOCTRINE

Status: bot and game-AI law.

The goal is competent human-looking play that makes public demos enjoyable and proves the engine supports simulation and decision-making. The goal is not superhuman AI.

## 1. Core law

1. Bots MUST consume the same legal action API as human players.
2. Bots MUST NOT apply state changes directly.
3. Bots MUST NOT choose illegal actions.
4. Bots MUST respect public/private information boundaries unless explicitly marked as diagnostic omniscient bots in local-only tools.
5. Every game MUST have a random legal bot.
6. Every public web game SHOULD have at least a rule-informed baseline bot once the random bot is stable.
7. ML/RL MUST NOT be used in v1 or v2 unless a future ADR explicitly overturns this.

## 2. AI ladder

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

- chooses only from legal actions/action paths;
- deterministic under bot seed;
- can run native simulations;
- has tests proving no illegal actions over many seeds.

### Level 1: rule-informed baseline bot

Required for serious public demos.

Use simple game knowledge:

- win immediately if possible;
- block immediate loss;
- follow mandatory rule priorities;
- avoid obviously illegal/impossible strategic choices;
- prefer material/points/resources when rules clearly support it.

This bot may still be weak. It should stop looking broken.

### Level 2: authored heuristic/policy bot

Preferred default for polished games.

Use:

- ordered tactical priorities;
- phase-aware decision trees;
- lexicographic priorities;
- explicit strategy modules;
- small scoped scoring functions;
- explainable decision reasons;
- game-specific policy code.

Do not create a giant weighted scoring soup. If a policy cannot be explained in a short note, it is probably becoming unmaintainable.

### Level 3: shallow search

MAY be used where appropriate.

Good fits:

- deterministic perfect-information games;
- small action spaces;
- strong terminal/evaluation functions;
- tactical games with short horizons.

Use minimax/alpha-beta or similar only after benchmarks show it fits the latency budget.

### Level 4: MCTS/ISMCTS/search experiments

MAY be explored only after the engine is fast and stable.

Use when:

- random playouts are fast enough;
- action abstraction exists where needed;
- hidden-information handling is explicit;
- benchmark harness can compare policies;
- the game benefits from search rather than authored tactical rules.

Information Set MCTS or other imperfect-information approaches require ADR before public reliance.

### Level 5: ML/RL research

Out of scope for v1 and v2.

A future ADR must justify:

- why authored/search bots are insufficient;
- training data source;
- reproducibility;
- inference cost;
- model storage;
- browser/server deployment;
- explainability tradeoffs;
- maintenance burden.

## 3. Bot API expectations

Bot traits SHOULD support:

```text
choose_action(game, state_or_view, player, legal_action_tree, rng, limits) -> ActionPath
```

Bot decision calls SHOULD receive:

- current legal action tree;
- public/private view allowed for that bot;
- deterministic RNG;
- time/depth/node limits;
- optional explanation sink;
- instrumentation sink.

Bot decision outputs SHOULD include:

- chosen action path;
- policy name/version;
- random seed/counter where relevant;
- decision timing;
- optional explanation;
- optional candidate ranking in debug builds.

## 4. Hidden-information doctrine

Bots for hidden-information games MUST be explicit about what they know.

Allowed bot modes:

- `PublicViewBot`: sees only legal public information plus its own private hand/role.
- `DeterminizationBot`: samples hidden state from legal possibilities using a documented model.
- `DiagnosticOmniscientBot`: sees full state only in local/dev testing and MUST be labeled as cheating.

Public demos SHOULD use non-cheating bots unless the UI clearly labels a diagnostic mode.

## 5. Simulation requirements

Every game MUST support native CLI simulation.

Simulation should measure:

- games completed;
- games ended by terminal condition;
- games ended by turn cap;
- illegal action attempts by bots;
- invariant violations;
- average turn count;
- playout throughput;
- bot decision latency;
- seed of failures.

Simulation failures MUST emit enough data to replay and reduce the failing seed.

## 6. Benchmark requirements

Every bot SHOULD have benchmarks for:

- legal action generation time;
- action selection latency;
- playout throughput;
- allocations if practical;
- serialization/replay overhead where relevant.

Benchmark from native Rust first. WASM smoke benchmarks are secondary.

## 7. Bot documentation

Every non-random bot MUST document:

- strategy level;
- information access;
- decision order;
- scoring terms if any;
- known weaknesses;
- benchmark numbers;
- tests proving legal-action use;
- whether it is suitable for public play.

## 8. Anti-patterns

MUST NOT:

- bypass legal action generation;
- tune endless weights with no tests;
- hide tactical logic in static data;
- call browser code from bot hot loops;
- add ML because it sounds impressive;
- benchmark only in the browser;
- make a bot for a monster game before smaller policy bots work;
- present a cheating bot as fair.

## 9. Why this doctrine is strict

Major public board-game platforms still treat bots as custom work, not a solved generic framework problem. Research frameworks such as OpenSpiel show how serious game AI work depends on a fast game core, but they are not public web-app UI frameworks. The project should use that lesson: build fast, deterministic, testable decision infrastructure; then write game-specific bots honestly.

## Source notes

See `SOURCES.md`, especially OpenSpiel, Board Game Arena Bots and Artificial Intelligence, Board Game Arena Zombie Mode, Regular Boardgames / Regular Games, and native benchmark sources.
