# TESTING AND BENCHMARKING

Status: correctness and performance law.

A game is not implemented when it appears to work in the UI. It is implemented when its rules, replay, visibility, bots, docs, and performance are covered.

## 1. Definition of done for every game

Every implemented game MUST have:

- typed Rust rules;
- structured rules documentation;
- rules-source notes and citations/links where appropriate;
- rule coverage matrix;
- unit tests;
- rule tests;
- golden trace tests;
- property/invariant tests;
- fuzz/simulation tests;
- deterministic replay tests;
- serialization tests;
- AI legal-action tests;
- CLI simulation support;
- benchmark coverage;
- game module README or docs;
- UI metadata;
- replay support;
- UI smoke tests once exposed in the web app.

Games with hidden information MUST additionally have:

- public/private view tests;
- no-leak tests for logs, previews, serialization, and bot views;
- replay tests for hidden stochastic setup.

Games with bots MUST additionally have:

- bot legality tests over many seeds;
- bot decision latency benchmarks;
- bot documentation.

## 2. Test categories

### Unit tests

Small pure-function tests for:

- win detection;
- scoring;
- movement validity;
- pattern detection;
- selector helpers;
- visibility projection helpers;
- hand evaluators;
- resource accounting.

### Rule tests

Scenario tests for named rules.

Each rule test SHOULD cite or reference the structured rule summary section it covers.

### Golden trace tests

Golden traces MUST record known action sequences and expected hashes/effects.

A golden trace SHOULD include:

```text
game id
rules version
seed
players/seats
options/variant
action stream
checkpoints
expected final state hash
expected effect hash
expected legal-action hash at selected checkpoints
```

Golden traces catch accidental rule drift.

### Property/invariant tests

Property tests SHOULD assert invariants such as:

- legal actions never produce invalid states;
- total pieces/cards/resources are conserved where applicable;
- scores stay within expected bounds;
- terminal states have no normal legal actions;
- mandatory moves are enforced;
- hidden information is not visible to unauthorized viewers;
- serialization round-trips preserve state.

### Fuzz/simulation tests

Random legal simulations MUST exist for every game.

Simulation tests SHOULD:

- run many seeds;
- enforce turn caps where needed;
- record failing seed and command stream;
- check invariants after every action;
- verify bots choose only legal actions;
- generate minimal reproducible artifacts when practical.

### Deterministic replay tests

Replay tests MUST prove that seed + command stream reproduces:

- state hashes;
- effect hashes;
- outcome;
- legal action hashes at checkpoints.

### Visibility tests

Required for hidden-information games.

Test:

- opponent hands are hidden;
- face-down cards remain hidden;
- private choices are hidden before reveal;
- public logs do not reveal private identity;
- previews do not leak hidden state;
- serialized public views are safe.

### Serialization tests

Test:

- internal snapshot round trip;
- public view JSON round trip;
- replay JSON round trip;
- version fields present;
- unknown/newer version behavior is explicit.

### AI legal-action tests

For every bot:

- sample many states/seeds;
- request action from bot;
- validate action through normal engine path;
- reject if bot bypasses legal action tree.

### UI smoke tests

Once a game is public-web-exposed, UI smoke tests SHOULD cover:

- load game;
- start match;
- display board/state;
- display legal actions;
- apply action;
- display effect log;
- run bot turn;
- replay command stream;
- basic responsiveness.

## 3. Rule coverage matrix

Every game MUST maintain `docs/RULE-COVERAGE.md` or equivalent.

Suggested columns:

| Rule section | Summary | Implementation module | Unit tests | Rule tests | Golden traces | Notes |
|---|---|---|---|---|---|---|

Every omitted rule MUST be marked:

- not applicable to chosen variant;
- intentionally deferred;
- unsupported;
- open question.

No silent rule gaps.

## 4. Bug protocol

Every bug MUST receive regression coverage.

When tests fail, the fix protocol is:

1. determine whether the failing tests are still valid;
2. determine whether the issue is in the system under test or the test suite;
3. fix the issue;
4. add or update regression coverage;
5. report what changed.

Do not blindly rewrite tests to pass.

## 5. Native-first benchmark doctrine

Benchmark from native Rust first.

Browser measurements are useful only after the native engine is correct and measurable.

Measure at least:

- legal action generation;
- action validation;
- action application;
- random playout throughput;
- serialization/deserialization;
- replay throughput;
- public/private view generation;
- bot decision latency;
- WASM smoke performance for public games.

## 6. Provisional performance budgets

These are starting targets, not promises. Replace them with measured baselines once games exist.

| Stage | Example game type | Native random playout target | Notes |
|---|---:|---:|---|
| 0 | Nim / Tic-Tac-Toe | 500,000+ games/sec | Tiny smoke games should be extremely fast. |
| 1 | Four-in-a-Row | 100,000+ games/sec | Grid placement and pattern checks. |
| 2 | Directional flip | 30,000+ games/sec | Multi-piece updates increase cost. |
| 3 | Draughts-lite | 10,000+ games/sec | Action trees and forced continuations. |
| 4 | Blackjack / simple cards | 20,000+ hands/sec | Shuffling and hidden views. |
| 7 | Poker-lite / Hold 'Em | 2,000+ hands/sec | Hand evaluation and betting dominate. |
| 12 | asymmetric event game | 100+ turns/sec | Complex games need measured, realistic budgets. |

Latency budgets for public play:

| Operation | Initial target |
|---|---:|
| legal action tree for current player | under 16 ms for early games, under 100 ms for complex games |
| apply action | under 16 ms for early games, under 100 ms for complex games |
| random/legal bot decision | under 100 ms for public demo games |
| heuristic bot decision | under 250 ms unless UI explains thinking delay |
| replay step | fast enough for 1x and stepped playback without dropped UI events |

If a game exceeds the budget, document why and create a benchmark issue. Do not optimize blindly.

## 7. Benchmark reporting

Each game SHOULD publish benchmark notes:

```text
hardware
Rust version
build profile
engine version
game rules version
benchmark command
baseline numbers
known bottlenecks
regression threshold
```

Public docs MAY show benchmark summaries after numbers are stable.

## 8. CI expectations

CI SHOULD run:

- formatting;
- linting;
- unit/rule tests;
- golden trace tests;
- serialization tests;
- no-leak tests;
- quick simulations;
- docs link checks where practical;
- WASM build smoke;
- web UI smoke for exposed games.

Full fuzzing and expensive benchmarks MAY run nightly or manually.

## 9. Test data and traces

Golden traces and test fixtures MUST avoid licensed data unless they are private-only.

Public trace names SHOULD avoid trademark-forward game presentation.

## 10. Performance anti-patterns

MUST NOT:

- measure only browser speed;
- serialize JSON in inner playout loops;
- cross JS/WASM boundary per rule operation;
- clone huge states blindly without benchmark evidence;
- allow unordered-map iteration to affect deterministic output;
- hide slow behavior in generic interpreters;
- add a complex bot before legal/action/replay benchmarks exist.

## Source notes

See `SOURCES.md`, especially OpenSpiel, Regular Boardgames / Regular Games, deterministic lockstep, command-log replay, Rust/WASM, and Board Game Arena bot guidance.
