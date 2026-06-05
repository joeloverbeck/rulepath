# Rulepath Testing and Benchmarking

Status: correctness and performance law.

A game is not implemented when it appears to work in the UI. It is implemented when its rules, replay, visibility, bots, docs, traces, and performance are covered.

## 1. Definition of done for every game

Every implemented game MUST have:

- typed Rust rules;
- structured rules documentation;
- source notes;
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

Hidden-information games additionally MUST have:

- public/private view tests;
- no-leak tests for logs;
- no-leak tests for previews;
- no-leak tests for serialization;
- no-leak tests for bot views;
- no-leak tests for UI payloads and DOM-safe fixtures;
- replay tests for stochastic hidden setup.

Games with bots additionally MUST have:

- bot legality tests over many seeds;
- bot determinism tests;
- bot decision latency benchmarks;
- bot documentation;
- explanation examples;
- no-leak tests for bot explanations/candidate rankings when hidden information exists.

## 2. Test category table

| Category | Required? | Purpose |
|---|---|---|
| Unit tests | yes | pure helper correctness |
| Rule tests | yes | named rule scenarios |
| Golden traces | yes | accidental rule drift detection |
| Property/invariant tests | yes | broad state validity |
| Simulation/fuzz tests | yes | many legal playouts and failure seeds |
| Replay tests | yes | seed + commands reproduce hashes |
| Serialization tests | yes | snapshots/views/replays round trip safely |
| Visibility tests | hidden-info games | public/private safety |
| Bot legality tests | games with bots | bots only choose legal actions |
| Bot latency benchmarks | non-trivial bots | public responsiveness |
| UI smoke tests | web-exposed games | browser integration |
| Benchmark coverage | yes | performance visibility |

## 3. Unit tests

Use small pure-function tests for:

- win detection;
- scoring;
- movement validity;
- pattern detection;
- directional scanning;
- hand evaluation;
- resource accounting;
- visibility projection helpers;
- action-tree construction helpers;
- static data validation.

Unit tests SHOULD avoid browser/WASM setup.

## 4. Rule tests

Rule tests are scenario tests for named rules.

Each rule test SHOULD reference the `RULES.md` section it covers.

Example rule-test names:

```text
rule_3_2_occupied_cells_are_not_legal
rule_4_1_forced_capture_must_continue
rule_6_3_opponent_hand_is_hidden
rule_9_5_raise_must_meet_minimum
```

Rule tests SHOULD fail with clear diagnostics, not merely mismatched snapshots.

## 5. Golden trace tests

Golden traces record known action sequences and expected hashes/effects.

A golden trace SHOULD include:

```text
game_id
rules_version
engine_version
data_version
seed
players/seats
options/variant
action_stream
checkpoints
expected_state_hashes
expected_effect_hashes
expected_legal_action_hashes
expected_public_view_hashes for selected viewers
notes explaining why the trace exists
```

Golden traces MUST be updated only when rule behavior intentionally changes or trace format changes. The update note must say why.

## 6. Property/invariant tests

Property tests SHOULD assert invariants such as:

- legal actions never produce invalid states;
- legal action generation never panics;
- terminal states have no normal legal actions unless the game defines post-game actions;
- total pieces/cards/resources are conserved where applicable;
- scores stay within expected bounds;
- mandatory moves are enforced;
- hidden information is not visible to unauthorized viewers;
- serialization round trips preserve state;
- replay hashes are deterministic;
- action trees contain no duplicate unstable IDs unless intentionally allowed.

## 7. Simulation/fuzz tests

Random legal simulations MUST exist for every game.

Simulation SHOULD:

- run many seeds;
- enforce turn/action caps;
- check invariants after every action;
- validate bot actions through the normal path;
- record failing seed and command stream;
- export minimal reproducible trace where practical;
- measure average length and terminal outcomes.

Simulation failure output SHOULD include:

```text
game_id
rules_version
seed
bot_policy_versions
options
turn/action index
actor
chosen action path
command stream so far
state/effect hash
invariant failure or panic
replay command
```

## 8. Deterministic replay tests

Replay tests MUST prove that seed + options + command stream reproduces:

- state hashes;
- effect hashes;
- legal action hashes at checkpoints;
- public view hashes at checkpoints;
- outcome;
- terminal state.

Replay tests SHOULD include at least:

- one short normal game;
- one game reaching terminal condition;
- one trace involving bot action;
- one trace involving stochastic setup when applicable;
- one trace involving hidden-information redaction when applicable.

## 9. Visibility and no-leak tests

Required for hidden-information games.

Test that unauthorized viewers cannot see:

- opponent hand/card identities;
- face-down card identities;
- secret commitments before reveal;
- hidden roles;
- hidden deck order;
- private logs;
- hidden bot inputs;
- hidden reasons in diagnostics;
- hidden data in serialized public views;
- hidden data in UI payload fixtures.

No-leak tests SHOULD search serialized payloads for known hidden IDs and fail if found.

## 10. Serialization tests

Test:

- internal snapshot round trip;
- public view JSON round trip;
- replay JSON round trip;
- optional compact snapshot round trip if used;
- version fields present;
- unknown/newer version behavior explicit;
- stable hash serialization;
- public/private mode separation.

## 11. AI legal-action tests

For every bot:

- sample many states/seeds;
- request action from bot;
- validate action through normal engine path;
- reject if bot bypasses legal action tree;
- assert deterministic output for fixed seed/view/limits where applicable;
- assert explanation exists for non-random bots.

Hidden-information bot tests MUST pass bot view, not internal state.

## 12. UI smoke tests

Once a game is web-exposed, UI smoke tests SHOULD cover:

- load game picker;
- start match;
- display board/state;
- display legal actions;
- apply one human action;
- display semantic effect log;
- run one bot turn;
- replay command stream;
- dev toggle opens without breaking play;
- reduced-motion mode does not block play;
- basic responsiveness.

UI smoke tests MUST NOT become the primary rule tests. Rules are tested in Rust first.

## 13. Rule coverage matrix

Every game MUST maintain `docs/RULE-COVERAGE.md` or equivalent.

Suggested columns:

| Rule section | Summary | Implementation module | Unit tests | Rule tests | Golden traces | Property/simulation coverage | Notes |
|---|---|---|---|---|---|---|---|

Every omitted rule MUST be marked:

- not applicable to chosen variant;
- intentionally deferred;
- unsupported;
- open question.

No silent rule gaps.

## 14. Failing-test protocol

When tests fail:

1. Determine whether the failing tests are still valid.
2. Determine whether the issue is in the system under test or the test suite.
3. Fix the issue.
4. Add or update regression coverage.
5. Report what changed.

Agents and humans MUST NOT delete, weaken, or rewrite tests merely to get green output.

## 15. Native-first benchmark doctrine

Benchmark from native Rust first.

Browser measurements are useful only after the native engine is correct and measurable.

Measure at least:

- setup time where relevant;
- legal action generation;
- action validation;
- action application;
- public/private view generation;
- effect filtering;
- serialization/deserialization;
- replay throughput;
- random playout throughput;
- bot decision latency;
- WASM smoke performance for public games.

## 16. Provisional performance budgets

These are starting targets, not promises. Replace them with measured baselines once games exist.

| Stage | Example | Native target | Notes |
|---:|---|---:|---|
| 1 | `race_to_n` / Nim | 500,000+ games/sec | tiny smoke games should be extremely fast |
| 2 | `three_marks` | 300,000+ games/sec | tiny grid |
| 3 | `column_four` | 100,000+ games/sec | gravity and line checks |
| 4 | `directional_flip` | 30,000+ games/sec | directional scanning and flips |
| 5 | `draughts_lite` | 10,000+ games/sec | action trees and continuations |
| 6 | `high_card_duel` / `blackjack_lite` | 20,000+ hands/sec | shuffle and hidden views |
| 9 | `poker_lite` | 2,000+ hands/sec | hand evaluation and betting dominate |
| 14 | event-driven asymmetric game | 100+ turns/sec | complex games need realistic budgets |

Latency budgets for public play:

| Operation | Early games | Complex games |
|---|---:|---:|
| legal action tree | under 16 ms | under 100 ms |
| preview | under 16 ms | under 100 ms |
| apply action | under 16 ms | under 100 ms |
| random/legal bot | under 100 ms | under 250 ms |
| heuristic policy bot | under 250 ms | under 500 ms with UI thinking feedback |
| replay step | smooth at 1x | no dropped UI events in stepped mode |

If a game exceeds budget, document why and create a benchmark issue. Do not optimize blindly.

## 17. Benchmark reporting

Each game SHOULD publish benchmark notes:

```text
hardware
OS
Rust version
build profile
engine version
game rules version
data version
benchmark command
baseline numbers
regression threshold
known bottlenecks
comparison to prior release
```

CI MAY run quick benchmarks or regression smoke. Full benchmarks may run nightly or manually.

## 18. CI expectations

CI SHOULD run:

- formatting;
- linting;
- unit/rule tests;
- golden trace tests;
- serialization tests;
- visibility/no-leak tests;
- quick simulations;
- docs link checks where practical;
- static data schema validation;
- WASM build smoke;
- web UI smoke for exposed games.

Full fuzzing and expensive benchmarks MAY run nightly or manually.

## 19. Test data and IP

Golden traces and fixtures MUST avoid licensed data unless they are private-only.

Public traces SHOULD use neutral game IDs and original content.

Private licensed traces MUST NOT be public CI dependencies or public artifacts.

## 20. Performance anti-patterns

MUST NOT:

- measure only browser speed;
- serialize JSON in inner playout loops;
- cross JS/WASM boundary per rule operation;
- clone huge states blindly without benchmark evidence;
- allow unordered-map iteration to affect deterministic output;
- hide slow behavior in generic interpreters;
- add complex bots before legal/action/replay benchmarks exist;
- optimize without a benchmark target;
- accept “it feels fast” as evidence.

## Source notes

See `SOURCES.md`, especially OpenSpiel, Regular Boardgames, Regular Games, deterministic replay/command sources, Rust/WASM, and Board Game Arena guidance.
