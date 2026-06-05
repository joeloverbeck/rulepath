# ARCHITECTURE

Status: architectural law and workspace direction.

The architecture is Rust-workspace-first. TypeScript is a presentation client. The repository MUST optimize for deterministic rules, legal actions, replay, simulation, public/private views, and native performance before web polish.

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
    nim/
    tic_tac_toe/
    four_in_a_row/
    directional_flip/
    draughts_lite/
    ...
  apps/
    web/
  tools/
    simulate/
    trace-viewer/
    replay-check/
    rule-coverage/
    bench-report/
  benches/
  docs/
    FOUNDATIONS.md
    ARCHITECTURE.md
    AUTHORING-MODEL.md
    GAME-LADDER.md
    UI-DOCTRINE.md
    AI-DOCTRINE.md
    TESTING-AND-BENCHMARKING.md
    MULTIPLAYER-POLICY.md
    IP-POLICY.md
    AGENT-DISCIPLINE.md
    IMPLEMENTATION-ORDER.md
    SOURCES.md
    adr/
```

This shape MAY change through ADR, but the ownership rule MUST NOT change: Rust owns rules and hot loops; TypeScript owns UI and browser integration.

## 2. Crate responsibilities

### `engine-core`

`engine-core` MUST be small. It defines contracts and infrastructure only.

`engine-core` MAY contain:

- `GameId`, `RulesVersion`, `GameManifestId`;
- `PlayerId`, `SeatId`, `MatchId`;
- deterministic seed and RNG contracts;
- state transition protocol;
- command/action path/action tree model;
- effect log model;
- replay model;
- visibility/public-view contracts;
- serialization contracts;
- error and diagnostic types;
- versioning hooks;
- checksums and trace hashing;
- generic capability tags that are not game nouns.

`engine-core` MUST NOT contain:

- game-specific nouns;
- game-specific rules;
- card names, faction names, scenario names, or proprietary text;
- reusable mechanics merely because one game needs them;
- UI layout or animation timing;
- bot strategy;
- web framework concepts;
- networking, accounts, matchmaking, or persistence.

### `game-stdlib`

`game-stdlib` contains reusable mechanics extracted from real pressure.

It MAY contain primitives such as:

- grids and coordinates;
- line/pattern detection;
- zones and piles;
- standard deck/card helpers;
- dice and deterministic chance helpers;
- tracks, counters, resources;
- graph maps;
- drafting helpers;
- auction helpers;
- simultaneous-choice helpers;
- reaction-window helpers.

A primitive SHOULD enter `game-stdlib` only after at least two implemented games need the same shape, or after an ADR explains why earlier extraction is justified.

`game-stdlib` MUST NOT become a second kernel. It may be useful, but it is optional. Games may implement local logic without shame.

### `ai-core`

`ai-core` contains bot traits and reusable decision infrastructure.

It SHOULD contain:

- bot trait(s);
- random legal bot;
- rollout/simulation helpers;
- deterministic bot RNG handling;
- policy composition helpers;
- lexicographic priority helpers;
- optional minimax/alpha-beta helpers for perfect-information games;
- optional MCTS/ISMCTS infrastructure after ADR;
- benchmark instrumentation.

`ai-core` MUST NOT contain game-specific strategy. Game-specific policies live in `games/*`.

### `games/*`

Each game module owns its nouns, rules, static data, docs, tests, UI metadata, explanation templates, and bots.

A game module SHOULD contain:

```text
games/<game_id>/
  Cargo.toml
  src/
    lib.rs
    state.rs
    actions.rs
    rules.rs
    visibility.rs
    effects.rs
    bots.rs
    ui.rs
  data/
  docs/
    RULES.md
    RULE-COVERAGE.md
    AI.md
    UI.md
    BENCHMARKS.md
  tests/
    golden_traces/
    rule_tests.rs
    visibility_tests.rs
    simulation_tests.rs
```

Game-specific logic in a game module is correct. Game-specific logic in `engine-core` is a boundary failure.

### `wasm-api`

`wasm-api` exposes a batched browser-facing API. It MUST be thin and boring.

It SHOULD expose operations like:

```text
list_games() -> GameCatalog
new_match(game_id, seed, seats, options) -> MatchHandle
load_match(snapshot_or_replay) -> MatchHandle
get_public_view(match, viewer) -> PublicViewJson
get_action_tree(match, viewer) -> ActionTreeJson
preview_action(match, viewer, action_path) -> PreviewJson
apply_action(match, viewer, action_path) -> ApplyResultJson
run_bot_turn(match, bot_seat, limits) -> ApplyResultJson
get_replay(match) -> ReplayJson
get_effects(match, since_cursor, viewer) -> EffectLogJson
serialize_match(match) -> SnapshotBytesOrJson
```

The API MUST avoid chatty hot-loop crossings. Do not call JS for each rule step. Do not serialize full internal state on every animation frame.

### `apps/web`

`apps/web` is the public playable shell.

It SHOULD contain:

- game picker;
- match setup;
- Rust/WASM package loader;
- board renderer boundary;
- legal action UI;
- effect log panel;
- replay viewer;
- bot/hotseat controls;
- debug inspector panels;
- responsive layout and original visual assets.

It MUST NOT implement rule legality.

### `tools/*`

Tooling is not optional. Tools SHOULD include:

- `simulate`: run native bot/random playouts;
- `replay-check`: replay command logs and compare hashes;
- `trace-viewer`: inspect command/effect/state traces;
- `rule-coverage`: report rules-to-tests coverage;
- `bench-report`: summarize criterion/native benchmarks;
- `seed-reducer`: reduce failing random seeds when practical.

## 3. Core transition model

The engine SHOULD preserve this conceptual separation:

```text
setup(seed, players, options) -> state
legal_action_tree(state, viewer_or_actor) -> action tree
validate(action_path, actor, state) -> command or diagnostic
apply(command, state, rng) -> effects + new state
public_view(state, viewer) -> visible state projection
replay(seed, command stream) -> same states + same effects
```

Concrete Rust APIs MAY differ. The separation MUST remain.

## 4. Action model

Simple games MAY expose flat legal actions. Compound games MUST expose action trees or progressive construction.

Core concepts:

- `ActionTree`: the legal choice structure at a decision point;
- `ActionNode`: a point where the actor chooses among options;
- `ActionChoice`: one selectable option, with label, tags, and optional preview;
- `ActionPath`: the selected path through the tree;
- `PartialAction`: accumulated choices before commit;
- `Command`: validated action ready to apply;
- `ActionDiagnostic`: why a path is invalid, disabled, stale, or unavailable.

The UI MAY hide illegal choices in player mode. Developer mode SHOULD expose disabled choices and reasons when useful.

## 5. Effect model

Effects are semantic facts emitted by the engine. They are not animations.

Effects SHOULD include enough information for:

- replay;
- logs;
- animation scheduling;
- debugging;
- explanation templates;
- visibility filtering.

Examples:

```text
ActionStarted
PiecePlaced
PieceMoved
PieceRemoved
CardDrawn
CardRevealed
CardDiscarded
CounterChanged
ResourceSpent
ResourceGained
ScoreChanged
ControlChanged
PhaseChanged
TurnChanged
RandomSampled
ChoiceResolved
ActionCompleted
```

Effects MUST be deterministic. Effects MUST be filterable for visibility. An effect log MUST NOT leak hidden information to an unauthorized viewer.

## 6. Replay and versioning

Replay is a first-class architecture feature.

A replay SHOULD include:

- game id;
- rules version;
- engine version;
- seed;
- player/seat mapping;
- options/variants;
- ordered command stream;
- optional checkpoints;
- state/effect/action-tree hashes at checkpoints;
- serialization version.

Breaking replay compatibility REQUIRES ADR or explicit migration notes.

## 7. Visibility model

The engine MUST distinguish internal state from public/private views.

Visibility contracts MUST cover:

- hidden hands;
- face-down cards;
- secret choices;
- unrevealed random outcomes;
- private logs;
- previews;
- bot information access;
- serialization/export.

A public browser view MUST be safe to send to that viewer. Do not ship hidden state to the browser and rely on the UI to hide it.

## 8. Determinism model

The engine MUST produce identical results for identical game version, seed, players/options, and command stream.

Rules:

- All randomness MUST pass through the engine RNG contract.
- Random samples SHOULD be logged as semantic effects where player-visible or replay-relevant.
- Floating-point logic MUST NOT be used in deterministic rule outcomes unless an ADR defines exact constraints.
- Time, OS randomness, iteration-order nondeterminism, and browser APIs MUST NOT affect rules.

## 9. Web deployment model

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

## 10. Comparative-system lessons

- boardgame.io shows that a turn-based web-game framework can expose useful state, log, plugin, multiplayer, and React infrastructure while staying pragmatic.
- VASSAL shows that large tabletop module ecosystems can scale by not enforcing rules; because this project does enforce rules, the cost must be paid deliberately.
- Ludii shows that true general game systems require formal game-description work, tooling, and research discipline.
- OpenSpiel shows the value of keeping computational game transitions close to AI/search infrastructure.
- Regular Boardgames and Regular Games show why formal/compiled representations and benchmark culture matter if playout speed matters.

These precedents justify incrementalism. They do not justify overbuilding a universal language in v1.

## 11. Required ADRs

Create ADRs for at least:

- Rust core + WASM web shell;
- no YAML by default;
- Rust-first game behavior;
- no DSL until repeated implementation pressure;
- `engine-core` vs `game-stdlib` boundary;
- action tree model;
- effect log model;
- replay/seed model;
- visibility/public-view model;
- AI policy model;
- static site first;
- multiplayer deferral;
- future authoritative Rust server;
- IP/public-private module policy;
- game ladder selection.

ADR format:

```markdown
# ADR-0001: Title

## Status
Accepted | Proposed | Superseded

## Context
What pressure forced the decision?

## Decision
What is now project law?

## Consequences
Positive, negative, and migration consequences.

## Alternatives considered
What was rejected and why?
```

## Source notes

See `SOURCES.md`, especially boardgame.io, VASSAL, Ludii, OpenSpiel, Regular Boardgames / Regular Games, Rust/WASM, deterministic lockstep, client-server architecture, and command-log replay sources.
