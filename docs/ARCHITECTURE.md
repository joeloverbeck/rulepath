# Rulepath Architecture

Status: architectural law. Supersede only by ADR.

Rulepath uses a Rust-workspace-first architecture with a React/TypeScript public web app. Rust is the source of truth for rules, legality, visibility, replay, and bots. TypeScript is the presentation client.

This is not an engine-demo architecture. The public site must feel polished, but polish must be built on deterministic Rust contracts rather than browser-side rule guesses.

## 1. Recommended repository shape

```text
/
  Cargo.toml
  crates/
    engine-core/
    game-stdlib/
    ai-core/
    wasm-api/
  games/
    race_to_n/
    three_marks/
    column_four/
    directional_flip/
    draughts_lite/
    high_card_duel/
    token_bazaar/
    ...
  apps/
    web/
  tools/
    simulate/
    trace-viewer/
    replay-check/
    rule-coverage/
    bench-report/
    seed-reducer/
  benches/
  docs/
    FOUNDATIONS.md
    ARCHITECTURE.md
    DATA-RUST-BOUNDARY.md
    AUTHORING-MODEL.md
    GAME-LADDER.md
    IMPLEMENTATION-ORDER.md
    UI-DOCTRINE.md
    VISUAL-INTERACTION-CONTRACT.md
    AI-DOCTRINE.md
    BOT-POLICY-ARCHITECTURE.md
    TESTING-AND-BENCHMARKING.md
    MULTIPLAYER-POLICY.md
    IP-POLICY.md
    AGENT-DISCIPLINE.md
    SOURCES.md
    adr/
```

This shape MAY change by ADR. The ownership rule MUST NOT change: Rust owns behavior; TypeScript owns public presentation.

## 2. Dependency direction

Recommended dependency direction:

```text
apps/web  -> wasm-api package boundary
wasm-api  -> games registry + engine-core contracts
ai-core   -> engine-core contracts
games/*   -> engine-core + ai-core traits + optional game-stdlib
game-stdlib -> engine-core contracts only when necessary
engine-core -> no project crate with game mechanics
```

`engine-core` MUST NOT depend on `game-stdlib`, `ai-core`, `wasm-api`, `apps/web`, or `games/*`.

`game-stdlib` SHOULD remain optional. A game may implement local mechanics without extracting a shared primitive.

## 3. Ownership table

| Area | Owns | Must not own |
|---|---|---|
| `engine-core` | generic identities, seeds, action trees, commands, diagnostics, effects, replay, visibility contracts, serialization contracts, hashes | game nouns, rules, mechanics, bot strategy, UI, networking, storage |
| `game-stdlib` | earned reusable mechanics after repeated pressure | speculative abstractions, kernel law, game-specific exceptions |
| `ai-core` | bot traits, random legal bot, deterministic RNG use, policy helpers, instrumentation, bounded search helpers after ADR or gate | game strategy, hidden-state cheating, UI code |
| `games/*` | game nouns, rules, state, actions, visibility, effects, bots, UI metadata, docs, tests, traces, benchmarks | generic kernel contracts, browser shell, networking |
| `wasm-api` | thin batched API from browser to Rust | rule logic, chatty rule-loop calls, hidden-state leakage |
| `apps/web` | app shell, layout, menus, renderer integration, panels, settings, accessibility, replay UI | rule legality, hidden-state authority, bot decisions |
| `tools/*` | simulation, replay, trace, coverage, benchmark, seed reduction | public UI polish, game behavior not present in games |

## 4. `engine-core`

`engine-core` is deliberately small.

It MAY contain:

- `GameId`, `RulesVersion`, `GameManifestId`;
- `MatchId`, `PlayerId`, `SeatId`, viewer identity;
- deterministic seed and RNG contracts;
- action tree/action path/command contracts;
- diagnostics and stale-action errors;
- semantic effect log contracts;
- replay/checkpoint/hash contracts;
- visibility/public-view contracts;
- serialization/versioning hooks;
- generic capability tags that are not game nouns.

It MUST NOT contain:

- terms such as board, grid, card, deck, pile, hand, suit, faction, scenario, trick, pot, resource, market, role, combat, territory, movement, adjacency, line, capture, flip, promotion;
- helper functions for a single game;
- bot priorities or search evaluation;
- renderer metadata;
- TypeScript or web concepts;
- account, lobby, database, matchmaking, or server persistence;
- public or private licensed content.

### Kernel-change default

Default answer: do not change `engine-core`.

A kernel change MUST answer:

1. Which implemented games need it?
2. Why can this not live in `games/*`?
3. Why can this not live in `game-stdlib`?
4. Does the change introduce any game noun?
5. Does it preserve deterministic replay?
6. Does it preserve visibility boundaries?
7. Does it require ADR?

## 5. `game-stdlib`

`game-stdlib` contains reusable mechanics only after evidence.

It MAY eventually contain:

- grids and coordinates;
- line/pattern detection;
- zones and piles;
- deck helpers and deterministic shuffle helpers;
- tracks, counters, resources;
- graph maps and area-control helpers;
- simultaneous-choice helpers;
- drafting helpers;
- auction/betting accounting helpers;
- reaction-window helpers.

Extraction rule:

1. Implement the first game locally.
2. Implement the second game honestly.
3. Identify the repeated shape.
4. Extract a narrow tested helper.
5. Back-port both games.
6. Preserve traces or intentionally update them with notes.
7. Document the abstraction and its limits.

A helper extracted for one hypothetical future game is forbidden without ADR.

## 6. `ai-core`

`ai-core` contains reusable bot infrastructure, not game strategy.

It SHOULD contain:

- bot traits;
- random legal bot;
- deterministic bot RNG handling;
- policy-composition helpers;
- lexicographic priority helpers;
- candidate ranking/instrumentation structures;
- decision limits;
- simulation hooks;
- optional minimax/alpha-beta helpers for small perfect-information games after benchmarks;
- benchmark instrumentation.

It MUST NOT contain game-specific priorities, hidden-state peeking, UI calls, or monster-game logic.

## 7. `games/*`

Each game module owns its world.

Recommended shape:

```text
games/<game_id>/
  Cargo.toml
  src/
    lib.rs
    ids.rs
    state.rs
    setup.rs
    actions.rs
    rules.rs
    visibility.rs
    effects.rs
    bots.rs
    ui.rs
    variants.rs
  data/
    manifest.toml
    variants.toml
    fixtures/
  docs/
    RULES.md
    SOURCES.md
    RULE-COVERAGE.md
    AI.md
    UI.md
    BENCHMARKS.md
  tests/
    golden_traces/
    rule_tests.rs
    visibility_tests.rs
    simulation_tests.rs
    serialization_tests.rs
    bot_tests.rs
```

Game-specific types are not a smell inside a game. They are the point.

A game module MUST provide:

- setup;
- legal action tree or flat legal actions;
- validation from action path to command;
- transition application;
- terminal/outcome detection;
- effect emission;
- public/private view projection;
- serialization support;
- random legal bot;
- docs, tests, traces, benchmarks.

## 8. `wasm-api`

`wasm-api` is a thin batched browser-facing boundary.

It SHOULD expose operations conceptually like:

```text
list_games() -> GameCatalog
new_match(game_id, seed, seats, options) -> MatchHandle
load_match(snapshot_or_replay) -> MatchHandle
get_public_view(match, viewer) -> PublicViewPayload
get_action_tree(match, viewer) -> ActionTreePayload
preview_action(match, viewer, action_path, view_token) -> PreviewPayload
apply_action(match, viewer, action_path, view_token) -> ApplyResultPayload
run_bot_turn(match, bot_seat, limits) -> ApplyResultPayload
get_effects(match, since_cursor, viewer) -> EffectLogPayload
get_replay(match) -> ReplayPayload
serialize_match(match, mode, viewer) -> SnapshotPayload
```

It MUST avoid chatty hot-loop crossings. Do not call JavaScript for each rule step. Do not serialize full internal state on every animation frame.

Payloads sent to the browser MUST be viewer-safe.

## 9. `apps/web`

`apps/web` is the public playable shell.

It SHOULD contain:

- game picker;
- match setup;
- Rust/WASM loader;
- session controller;
- renderer boundary;
- action panels;
- effect log panel;
- replay viewer;
- bot/hotseat controls;
- settings;
- dev toggle and inspector panels;
- responsive layout;
- original assets.

It MUST NOT implement rule legality.

Debug panels MUST be behind a visible dev toggle or local-dev mode. They must not dominate the default public experience.

## 10. Core transition model

The architecture preserves this conceptual separation:

```text
setup(seed, seats, options) -> internal state
legal_action_tree(state, actor_viewer) -> legal action tree
preview(action_path, actor, state) -> safe preview or diagnostic
validate(action_path, actor, state, view_token) -> command or diagnostic
apply(command, state, rng) -> new state + semantic effects
public_view(state, viewer) -> viewer-safe projection
replay(seed, options, command stream) -> same states/effects/hashes
```

Concrete Rust APIs MAY differ. This separation MUST remain.

## 11. Action model

Simple games MAY expose flat legal actions. Compound games MUST expose action trees or progressive construction.

Core concepts:

| Concept | Meaning |
|---|---|
| `ActionTree` | legal choice structure at a decision point |
| `ActionNode` | point where the actor chooses among options |
| `ActionChoice` | selectable option with label, tags, accessibility text, and optional preview hook |
| `ActionPath` | selected path through the tree |
| `PartialAction` | accumulated choices before confirmation |
| `Command` | validated action ready to apply |
| `ActionDiagnostic` | reason a path is invalid, disabled, stale, hidden, or unavailable |
| `ViewToken` | freshness/version marker used to reject stale UI submissions gracefully |

The UI MAY hide illegal choices in normal mode. Developer mode SHOULD expose disabled choices and reasons when useful.

## 12. Effect model

Effects are semantic facts emitted by Rust. They are not animations.

Effects SHOULD support:

- replay;
- human-readable logs;
- animation scheduling;
- debugging;
- explanation templates;
- visibility filtering;
- trace hashing.

Examples of generic effect kinds are acceptable only when they remain generic:

```text
ActionStarted
ActorChosen
ItemPlaced
ItemMoved
ItemRemoved
ItemRevealed
CounterChanged
ScoreChanged
OwnershipChanged
PhaseChanged
TurnChanged
RandomSampled
ChoiceCommitted
ChoiceRevealed
ActionCompleted
GameEnded
```

Game modules MAY define game-specific semantic effects as long as they are emitted through generic contracts and filtered for viewers.

Effects MUST be deterministic. Effects MUST be visibility-filtered. Effects MUST NOT leak hidden information.

## 13. Replay and versioning

Replay is first-class.

A replay SHOULD include:

- game id;
- rules version;
- engine version;
- manifest/data version;
- seed;
- seats and player mapping;
- options/variants;
- ordered command stream;
- optional checkpoints;
- state, effect, and legal-action hashes at checkpoints;
- serialization version;
- source-build metadata when available.

Breaking replay compatibility REQUIRES ADR or explicit migration notes.

## 14. Visibility model

Internal state and viewer-safe views are different types.

Visibility contracts MUST cover:

- hidden hands;
- face-down cards;
- secret commitments;
- unrevealed random outcomes;
- private logs;
- previews;
- bot information access;
- serialization/export;
- dev inspectors;
- replay redaction.

A browser view MUST be safe to send to that viewer. Do not ship hidden state to the browser and rely on UI hiding.

## 15. Determinism model

The engine MUST produce identical results for identical game version, seed, seats, options, static data version, and command stream.

Rules:

- all randomness MUST pass through the engine RNG contract;
- random samples SHOULD be logged when player-visible or replay-relevant;
- floating-point logic MUST NOT determine rule outcomes unless an ADR defines exact constraints;
- wall-clock time, OS randomness, browser APIs, unordered-map iteration, and thread scheduling MUST NOT affect rules;
- serialization order for hashes MUST be stable.

## 16. Web deployment model

The initial web app SHOULD deploy as static files:

```text
index.html
assets/*.js
assets/*.css
assets/*.wasm
assets/game-data/*
```

No accounts, database, hosted multiplayer, matchmaking, or server deployment belong in the initial architecture.

Rust/WASM is a deployment boundary, not a rules boundary. Rust remains the source of truth.

## 17. Tools are not optional

Tooling SHOULD include:

- `simulate`: native bot/random playouts;
- `replay-check`: command log replay and hash comparison;
- `trace-viewer`: command/effect/state trace inspection;
- `rule-coverage`: rules-to-tests coverage report;
- `bench-report`: benchmark summaries;
- `seed-reducer`: failure seed minimization where practical;
- `fixture-check`: static data schema validation.

## 18. Architecture acceptance checklist

Before accepting a major change, verify:

- `engine-core` has no game nouns;
- Rust owns behavior;
- no behavior is hidden in untyped data;
- action trees and diagnostics are generated by Rust;
- effects are semantic, deterministic, and filterable;
- public views are viewer-safe;
- replay is deterministic;
- bots use legal action APIs and allowed views;
- WASM API is batched;
- TypeScript has no rule legality;
- tests/traces/benchmarks exist;
- no public licensed content is bundled;
- ADR exists for architecture changes.

## 19. Comparative-system lessons

- boardgame.io proves that a pragmatic web turn-based-game framework can expose state, logs, plugins, multiplayer, a lobby, AI hooks, and React bindings without requiring a universal tabletop language.
- VASSAL proves that a huge module ecosystem can scale partly by not enforcing rules. Rulepath chooses the harder path, so it must pay with tests, legal-action generation, replay, performance, and game-specific implementations.
- Ludii proves that broad generality requires formal ludemic language/tooling discipline.
- Regular Boardgames and Regular Games prove that efficient general descriptions require compiler, automata, optimization, and benchmark culture.
- OpenSpiel proves that serious AI/search infrastructure benefits from fast procedural game cores, but it is not a public UI architecture.

These precedents justify incrementalism. They do not justify building a DSL in v1.

## Source notes

See `SOURCES.md` for source details and links.
