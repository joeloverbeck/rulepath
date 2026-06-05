# Rulepath Testing, Replay, and Benchmarking

Status: correctness, determinism, replay, visibility, and performance law.

A game is not implemented when it appears to work in the UI. It is implemented when rules, replay, visibility, bots, docs, traces, and performance are covered.

## 1. Test taxonomy

| Category | Required | Purpose |
|---|---|---|
| Unit tests | yes | Pure helper correctness. |
| Rule tests | yes | Named rule scenarios tied to `RULES.md` and coverage rows. |
| Golden trace tests | yes | Detect unplanned rule/effect/view/hash drift. |
| Property/invariant tests | yes | Broad validity over many states/actions. |
| Simulation/fuzz tests | yes | Many legal playouts with reproducible failure seeds. |
| Replay tests | yes | Seed + options + commands reproduce hashes and outcomes. |
| Serialization tests | yes | Snapshots, public views, replays, versions, unknown/newer versions. |
| Visibility/no-leak tests | hidden-information games | Public/private safety across every payload and tool. |
| AI legal-action tests | games with bots | Bots choose legal action paths through normal validation. |
| UI smoke tests | web-exposed games | Browser integration and public UX wiring, not rule authority. |
| Mechanic primitive tests | promoted `game-stdlib` helpers | Shared helper examples, anti-examples, back-ported game cases. |
| Benchmarks | yes | Performance visibility and regression detection. |

## 2. Rule tests

Rule tests SHOULD reference stable rule section IDs or names.

Examples:

```text
rule_3_occupied_positions_are_not_legal
rule_4_forced_capture_must_continue
rule_6_opponent_private_cards_are_hidden
rule_9_minimum_raise_is_enforced
```

Rule tests live in Rust first. UI smoke tests do not replace rule tests.

## 3. Golden traces

Golden traces are executable historical evidence. They SHOULD include:

```text
game_id
rules_version
engine_version
data_version
schema_version
seed
seats/options/variant
action_stream
checkpoints
expected_state_hashes
expected_effect_hashes
expected_legal_action_hashes
expected_public_view_hashes for selected viewers
notes explaining why the trace exists
migration/update note when changed
```

Update golden traces only when rule behavior, effect contracts, view projection, serialization, hash format, or trace format intentionally changes. The update note MUST explain why.

Golden trace drift without explanation is a failure, not noise.

## 4. Replay determinism

Replay tests MUST prove that seed + options + command stream reproduce:

- final state hash;
- effect hashes;
- legal-action-tree hashes at checkpoints;
- public/private view hashes for selected viewers;
- outcome;
- terminal state;
- serialization round trips where applicable.

Each game SHOULD have at least:

- one short normal trace;
- one terminal trace;
- one bot-action trace;
- one invalid/stale diagnostic trace when applicable;
- one stochastic trace when random setup or random events exist;
- one redacted hidden-information trace when hidden information exists.

## 5. Determinism hazards

Rules MUST NOT depend on nondeterministic inputs.

Test or prohibit:

- wall-clock time in rules;
- OS randomness outside the RNG contract;
- browser API randomness;
- unordered map/set iteration in behavior or hashing;
- locale-sensitive ordering;
- thread scheduling;
- floating point in rule decisions without ADR constraints;
- unstable serialization order;
- hidden global state;
- nondeterministic bot tie-breakers.

The same game version, data version, seed, seats, options, and command stream must reproduce the same states, effects, views, action trees, and hashes.

## 6. Property and invariant tests

Property/invariant tests SHOULD assert:

- legal action generation never panics;
- legal actions never produce invalid states;
- terminal states do not expose normal gameplay actions unless explicitly defined;
- pieces/cards/resources are conserved where applicable;
- scores stay within expected bounds;
- mandatory moves are enforced;
- public/private visibility is safe;
- serialization round trips preserve state;
- replay hashes are deterministic;
- action trees use stable IDs and do not duplicate choices unexpectedly;
- effect logs cover visible consequences needed by UI/replay.

## 7. Simulation and fuzz tests

Every official game MUST support random legal simulation through a native CLI or equivalent tool.

Simulations SHOULD:

- run many seeds;
- enforce turn/action caps;
- check invariants after every action;
- validate bot actions through the normal path;
- record failing seed and command stream;
- export minimal reproducible traces where practical;
- measure terminal outcomes, average length, and throughput.

Failure output SHOULD include:

```text
game id
rules version
data version
seed
options/variant
bot policy versions
turn/action index
actor
chosen action path
command stream so far
state/effect/view hash at failure
failure reason
replay command
```

## 8. Visibility and no-leak tests

Hidden-information games MUST prove unauthorized viewers cannot see:

- opponent private components;
- face-down identities;
- hidden commitments before reveal;
- secret roles;
- hidden random order;
- private logs;
- hidden diagnostics;
- bot-only input data;
- hidden-state-derived candidate rankings;
- hidden facts in explanations;
- hidden information in serialized public views;
- hidden information in UI payload fixtures and DOM-safe attributes;
- hidden information in local storage;
- hidden information in replay exports.

Tests SHOULD serialize public payloads and search for known hidden IDs. This is blunt and valuable.

## 9. Serialization tests

Serialization tests MUST cover:

- internal snapshot round trip;
- public/private view JSON round trip;
- replay JSON round trip;
- compact snapshot round trip if used;
- version field presence;
- unknown/newer version behavior;
- stable hash serialization;
- public/private export separation;
- unknown-field rejection for hand-authored data.

Public replay interchange SHOULD remain readable JSON unless ADR says otherwise.

## 10. AI tests

For every bot:

- sample many states and seeds;
- request action from bot using allowed view;
- validate action through normal engine path;
- assert deterministic output for fixed seed/view/limits;
- assert explanation exists for non-random bots;
- reject direct state mutation or bypassed validation.

Hidden-information bots MUST receive a bot view, not internal full state. Their no-leak tests must cover explanations, candidate rankings, memory, belief models, and serialized debug output.

## 11. UI smoke tests

Once web-exposed, a game SHOULD cover:

- load game picker;
- start match;
- display public view;
- display legal actions;
- apply one human action;
- show semantic effects;
- run one bot turn where applicable;
- step replay;
- open dev toggle safely;
- reduced-motion mode does not block play;
- basic responsive layout.

UI smoke tests are integration tests. They MUST NOT become primary rule tests.

## 12. Mechanic primitive tests

Promoted `game-stdlib` helpers MUST have:

- unit tests;
- property tests where useful;
- examples;
- anti-examples;
- tests from each back-ported game;
- trace preservation or migration notes;
- benchmarks before and after extraction;
- documentation of limits.

A third official game MUST NOT proceed through duplicated mechanic code without ledger decision.

## 13. Failing-test protocol

When tests fail, humans and agents MUST:

1. determine whether the failing tests are still valid;
2. determine whether the issue is in the system under test or in the test suite;
3. fix the issue;
4. add or update regression coverage;
5. report what changed.

Tests MUST NOT be deleted, weakened, renamed away, or rewritten merely to get green output.

## 14. Native-first benchmark doctrine

Benchmark native Rust first. Browser/WASM measurements are useful smoke evidence after the native engine is correct and measurable.

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

Do not optimize without a benchmark target. Do not claim performance without benchmark evidence.

## 15. Provisional performance budgets

Initial native targets, to be replaced by measured baselines:

| Stage | Example | Native target |
|---:|---|---:|
| 1 | `race_to_n` / Nim | 500,000+ games/sec |
| 2 | `three_marks` | 300,000+ games/sec |
| 3 | `column_four` | 100,000+ games/sec |
| 4 | `directional_flip` | 30,000+ games/sec |
| 5 | `draughts_lite` | 10,000+ games/sec |
| 6 | `high_card_duel` / `blackjack_lite` | 20,000+ hands/sec |
| 9 | `poker_lite` | 2,000+ hands/sec |
| 14 | `event_frontier` | 100+ turns/sec |

Public latency targets:

| Operation | Early games | Complex games |
|---|---:|---:|
| legal action tree | under 16 ms | under 100 ms |
| preview | under 16 ms | under 100 ms |
| apply action | under 16 ms | under 100 ms |
| random legal bot | under 100 ms | under 250 ms |
| authored policy bot | under 250 ms | under 500 ms with UI thinking feedback |
| replay step | smooth at 1x | no dropped UI events in stepped mode |

If a game exceeds budget, document why and create benchmark work. Do not hide unknown performance.

## 16. Benchmark report contents

Each game benchmark note SHOULD include:

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

## 17. CI expectations

CI SHOULD run:

- formatting;
- linting;
- unit/rule tests;
- golden trace tests;
- replay tests;
- serialization tests;
- visibility/no-leak tests;
- quick simulations;
- static data validation;
- docs link checks where practical;
- WASM build smoke;
- UI smoke for exposed games.

Full fuzzing and expensive benchmarks MAY run nightly or manually.

## 18. IP-safe fixtures and traces

Public fixtures, traces, snapshots, benchmark data, and UI test artifacts MUST contain only public-domain/classic neutral data, original content, or permissioned content.

Private licensed traces MUST NOT be public CI dependencies or public artifacts.

Before public release, inspect traces and bundles for proprietary IDs, prose, card text, assets, screenshots, and private module names.
