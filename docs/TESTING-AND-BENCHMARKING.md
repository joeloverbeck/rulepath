# Rulepath Testing and Benchmarking

Status: correctness and performance law.

A game is not implemented when it appears to work in the UI. It is implemented when rules, replay, visibility, bots, docs, traces, and performance are covered.

## 1. Definition of done for every official game

Every official game must have:

- typed Rust rules;
- structured rules documentation;
- source notes;
- mechanic inventory;
- rule coverage matrix;
- unit tests;
- rule tests;
- golden trace tests;
- property/invariant tests;
- simulation/fuzz tests;
- deterministic replay tests;
- serialization tests;
- AI legal-action tests;
- CLI simulation support;
- benchmark coverage;
- UI metadata;
- replay support;
- UI smoke tests once web-exposed.

Hidden-information games additionally require public/private view tests and no-leak tests for logs, previews, serialization, bot views, explanations, candidate rankings, UI payloads, DOM-safe fixtures, local storage, and replay exports.

## 2. Test categories

| Category | Required | Purpose |
|---|---|---|
| Unit tests | yes | Pure helper correctness. |
| Rule tests | yes | Named rule scenarios tied to `RULES.md`. |
| Golden traces | yes | Detect unplanned rule/effect/view/hash drift. |
| Property/invariant tests | yes | Broad validity over many states/actions. |
| Simulation/fuzz tests | yes | Many legal playouts, failures with seeds. |
| Replay tests | yes | Seed + options + commands reproduce hashes. |
| Visibility/no-leak tests | hidden-info games | Public/private safety. |
| Serialization tests | yes | Internal snapshots, public views, replays round-trip safely. |
| AI legal-action tests | games with bots | Bots choose only legal action paths. |
| UI smoke tests | web-exposed games | Browser integration, not rule authority. |
| Mechanic atlas/primitive tests | promoted helpers | Shared helpers covered across examples/anti-examples. |
| Benchmarks | yes | Performance visibility and regression detection. |

## 3. Rule tests

Rule tests should reference rule sections by name or stable ID. They should fail with clear diagnostics.

Examples:

```text
rule_3_occupied_positions_are_not_legal
rule_4_forced_capture_must_continue
rule_6_opponent_private_cards_are_hidden
rule_9_minimum_raise_is_enforced
```

Rule tests live in Rust first. UI smoke tests do not replace rule tests.

## 4. Golden traces

A golden trace should include:

```text
game_id
rules_version
engine_version
data_version
seed
seats/options/variant
action_stream
checkpoints
expected_state_hashes
expected_effect_hashes
expected_legal_action_hashes
expected_public_view_hashes for selected viewers
notes explaining why the trace exists
```

Update golden traces only when rule behavior, effect contracts, view projection, or trace format intentionally changes. The update note must explain why.

## 5. Property and invariant tests

Property/invariant tests should assert:

- legal actions never produce invalid states;
- legal action generation never panics;
- terminal states do not expose normal gameplay actions unless explicitly defined;
- total pieces/cards/resources are conserved where applicable;
- scores stay within expected bounds;
- mandatory moves are enforced;
- public/private visibility is safe;
- serialization round trips preserve state;
- replay hashes are deterministic;
- action trees use stable IDs and do not duplicate choices unexpectedly.

## 6. Simulation and fuzz tests

Every game must support random legal simulation. Simulations should:

- run many seeds;
- enforce turn/action caps;
- check invariants after every action;
- validate bot actions through normal path;
- record failing seed and command stream;
- export minimal reproducible traces where practical;
- measure terminal outcomes, average length, and throughput.

Failure output should include game id, rules version, seed, options, bot policy versions, turn/action index, actor, chosen action path, command stream so far, hash at failure, failure reason, and replay command.

## 7. Replay tests

Replay tests must prove seed + options + command stream reproduces state hashes, effect hashes, legal-action hashes, public-view hashes, outcome, and terminal state.

Each game should have at least:

- one short normal trace;
- one terminal trace;
- one bot-action trace;
- one invalid/stale diagnostic trace when applicable;
- one stochastic trace when random setup exists;
- one redacted hidden-information trace when applicable.

## 8. Visibility and no-leak tests

Hidden-information games must prove unauthorized viewers cannot see:

- opponent private components;
- face-down identities;
- hidden commitments before reveal;
- secret roles;
- hidden random order;
- private logs;
- hidden diagnostics;
- bot-only input data;
- hidden information in serialized public views;
- hidden information in UI payload fixtures and DOM-safe attributes.

Tests should search serialized public payloads for known hidden IDs and fail if found.

## 9. Serialization tests

Test internal snapshot round trip, public view JSON round trip, replay JSON round trip, compact snapshot round trip if used, version field presence, unknown/newer version behavior, stable hash serialization, and public/private export separation.

Public replay interchange should remain readable JSON unless ADR says otherwise.

## 10. AI legal-action tests

For every bot:

- sample many states and seeds;
- request action from bot using allowed view;
- validate action through normal engine path;
- assert deterministic output for fixed seed/view/limits where applicable;
- assert explanation exists for non-random bots;
- reject direct state mutation or bypassed validation.

Hidden-information bots must receive bot view, not internal full state.

## 11. UI smoke tests

Once web-exposed, a game should cover:

- load game picker;
- start match;
- display public view;
- display legal actions;
- apply one human action;
- show semantic effects;
- run one bot turn;
- step replay;
- open dev toggle safely;
- reduced-motion mode does not block play;
- basic responsive layout.

UI smoke tests must not become primary rule tests.

## 12. Mechanic atlas and primitive tests

Promoted `game-stdlib` helpers must have:

- unit tests;
- property tests where useful;
- examples and anti-examples;
- tests from each back-ported game;
- trace preservation or migration notes;
- benchmarks before/after extraction;
- documentation of limits.

A third official game must not proceed through duplicated mechanic code without ledger decision.

## 13. Failing-test protocol

Follow the [failing-test protocol](INVARIANTS.md#1-failing-test-protocol). Humans and agents must not delete, weaken, or rewrite tests merely to get green output.

## 14. Native-first benchmark doctrine

Benchmark native Rust first. Browser measurements are useful after the native engine is correct and measurable.

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

Do not optimize without a benchmark target.

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
| random/legal bot | under 100 ms | under 250 ms |
| authored policy bot | under 250 ms | under 500 ms with UI thinking feedback |
| replay step | smooth at 1x | no dropped UI events in stepped mode |

If a game exceeds budget, document why and create benchmark work. Do not hide unknown performance.

## 16. Benchmark report template

Each game benchmark note should include:

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

Use `templates/GAME-BENCHMARKS.md` for per-game docs.

## 17. CI expectations

CI should run formatting, linting, unit/rule tests, golden trace tests, replay tests, serialization tests, visibility/no-leak tests, quick simulations, static data validation, docs link checks where practical, WASM build smoke, and UI smoke for exposed games.

Full fuzzing and expensive benchmarks may run nightly or manually.

## 18. IP-safe fixtures and traces

Public fixtures, traces, snapshots, and benchmark data must contain only public-domain/classic neutral data, original content, or permissioned content.

Private licensed traces must not be public CI dependencies or public artifacts.

Before public release, inspect traces and bundles for proprietary IDs, prose, card text, assets, screenshots, and private module names.
