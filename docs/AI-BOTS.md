# Rulepath AI and Bots

Status: bot law and operational architecture.

Rulepath bots exist to make public games enjoyable, testable, and demonstrable. The goal is competent, explainable, fair, non-superhuman play. The goal is not generic superhuman AI.

## 1. Core bot law

Bots must:

- consume the same legal action API as humans;
- choose an action path through normal validation;
- never mutate state directly;
- never choose illegal actions;
- be deterministic under seed, rules version, view, policy version, and limits;
- never use information unavailable to a real player in that seat;
- produce viewer-safe explanations for non-random public bots;
- have legality tests, determinism tests, simulations, and latency benchmarks.

Testing tools may inspect internal state. Such tools are not public bots and must not implement the public bot trait.

## 2. No omniscient bots

A bot must not receive opponent hands, face-down identities, hidden commitments, unrevealed deck order, secret roles, private logs, future random outcomes, or full-state shortcuts unavailable to its seat.

Hidden-information bots may use:

- public information;
- their own private information;
- rules and variant metadata;
- remembered observations from their own legal view;
- belief models over legal possibilities;
- sampled possibilities generated from legal information, not copied from actual hidden state.

No-leak tests are mandatory for hidden-information bots.

## 3. Bot levels

### Level 0: random legal bot

Required for every official game.

Purpose: prove legal action generation, stress transitions, drive simulations, find crashes, and provide an immediate automated opponent.

Requirements: legal action API only, deterministic seeded tie-break, many-seed legality tests, simulation support, minimal explanation such as “random legal choice.”

### Level 1: rule-informed baseline bot

Required for serious public demos.

Use obvious rule knowledge: win immediately, block immediate loss, obey forced rules, avoid visibly nonsensical choices, prefer direct visible points/material/resources when the rule connection is clear.

This bot may be weak. It should not look broken.

### Level 2: authored policy bot

Preferred default for polished public games.

Use candidate extraction, ordered tactical priorities, phase-aware policy nodes, lexicographic ranking, small bounded scoring tie-breakers, deterministic seeded tie-break, explanation examples, and debug candidate rankings.

A Level 2 bot requires a completed strategy evidence pack before coding.

### Level 3: shallow deterministic search

Allowed only for small perfect-information games where benchmarks prove it fits.

Allowed techniques include minimax, alpha-beta, iterative deepening with strict deterministic limits, and controlled transposition tables. Requirements: documented evaluator, latency benchmarks, deterministic limits, fallback policy, explanation that search was used, and no hidden-information search unless future ADR defines a fair information-set approach.

## 4. Explicit v1/v2 exclusions

Public v1/v2 must not use MCTS, ISMCTS, Monte Carlo-style bots, ML, or RL. Future use requires ADR that addresses playout speed, action abstraction, deterministic seeding, memory, latency, hidden-information fairness, explanation quality, public UX, and why authored policy or shallow search is insufficient.

## 5. Game-specific policy modules

Do not build “one bot plays every game.” The legal action API is generic; strategy is game-specific.

Preferred flow:

```text
legal action tree
  -> candidate extraction
  -> ordered tactical priorities
  -> phase-aware policy nodes
  -> lexicographic ranking
  -> small bounded tie-breakers
  -> deterministic seeded tie-break
  -> chosen action path + explanation
```

## 6. Crate ownership

| Location | Owns |
|---|---|
| `ai-core` | bot traits, random legal bot, deterministic RNG utilities, policy-node helpers, lexicographic rank helpers, candidate/ranking structures, instrumentation, decision limits, simulation hooks |
| `games/*/bots.rs` or equivalent | game-specific tactical policies, phase policies, style profiles, scoring tie-breakers, explanation text, tests, benchmarks |
| `tools/simulate` | playout harness, seed failure reporting, simulation metrics, benchmark integration |
| `apps/web` | display of bot name, timing, explanation, and dev-mode candidate ranking |

`ai-core` must not contain game strategy.

## 7. Candidate model

A candidate is a legal action path plus policy annotations.

A candidate may include action path, public label, phase, tactical tags, visible immediate effects, rule-mandatory flag, explanation fragments, priority vector, bounded tie-break score, seed tie-break key, timing, and dev diagnostic notes.

A candidate must not include hidden information unavailable to the bot seat.

## 8. Policy node types

Useful node types:

| Node | Purpose |
|---|---|
| mandatory filter | obey forced rules before preference |
| immediate tactic | win now, block immediate loss, avoid immediate terminal loss |
| phase priority | select different priorities by phase/round/state |
| positional preference | prefer visible strategic positions or structures |
| resource policy | evaluate visible costs/gains |
| denial policy | block visible opponent threats |
| risk posture | cautious/aggressive style under allowed information |
| bounded evaluator | small tie-break after lexicographic categories |
| fallback | ensure legal choice exists, usually random legal among finalists |

Nodes should be small enough to test and explain.

## 9. Lexicographic priorities

Prefer lexicographic priority vectors over giant weighted sums.

Example priority order:

```text
terminal win
avoid terminal loss
mandatory rule compliance
immediate tactical gain
opponent denial
positional preference
style preference
small bounded tie-break
seeded random tie-break
```

A better earlier category beats later categories. This makes behavior testable and explanations intelligible.

## 10. Small bounded scoring tie-breakers

Small scoring is allowed as a tie-breaker after higher priorities. It must be documented, bounded, game-specific, tested, and explainable.

Forbidden weight soup:

- dozens of magic weights with no priority rationale;
- style profiles implemented only by multiplying weights;
- tactical conditions hidden in static data;
- scores that cannot produce clear explanations;
- knob tuning without simulations and benchmarks.

## 11. Strategy evidence pack

Before a Level 2 bot is coded, fill `templates/BOT-STRATEGY-EVIDENCE-PACK.md`.

The pack must include sources consulted, competent-player principles, tactical priorities, phase model, candidate features, lexicographic ranking plan, permitted tie-breakers, forbidden hidden information, decision examples, explanation examples, known weaknesses, test plan, benchmark plan, and public UX note.

No evidence pack, no Level 2 bot.

## 12. Explanation contract

Every non-random bot decision should produce a viewer-safe explanation with:

- policy name/version;
- chosen priority reason;
- relevant visible fact;
- tie-break note if applicable;
- hidden-info disclaimer when relevant;
- whether fallback or search was used.

Good: “Chose the center column because no immediate win/block exists and the policy prefers central threats.”

Bad: “Chose this because score = 37.42.”

Forbidden: “Folded because the opponent has a flush” unless that fact is publicly visible.

## 13. Public and debug affordances

Public mode should expose a small “why?” or recent-bot-action affordance. It should be concise and not turn the game into a debug console.

Dev mode may show candidate rankings, priority vectors, selected node, timing, tie-break seed/counter, filtered counts, and fallback flags.

Debug output must be viewer-safe.

## 14. Hidden-information policy

Hidden-information bot docs must state:

- exact input view;
- memory model;
- belief or sampling model if any;
- forbidden information;
- no-leak tests;
- explanation redaction rules;
- candidate-ranking redaction rules.

Sampled determinizations must be generated from the bot’s legal information. They must not peek at actual hidden state.

## 15. Style profiles

One strong default bot personality comes first. Optional profiles may come later.

Profiles should vary policy order, risk posture, tie-break preferences, explanation tone, and bounded evaluators. Profiles must not cheat, use hidden information, inject random blunders by default, or become weight dumps.

## 16. Simulation and benchmarks

Every game must support native CLI simulation. Simulations should record games completed, terminal outcomes, turn/action caps, illegal action attempts, invariant failures, average length, playout throughput, bot decision latency, seed, and command stream of failures.

Non-trivial bots need benchmarks for legal action generation, candidate extraction, action selection latency, playout throughput, allocations where practical, and serialization/replay overhead when relevant.

Native Rust benchmarks are primary. Browser/WASM smoke measurements are secondary.

## 17. Bot docs and tests

Every non-random bot must document strategy level, policy name/version, information access, decision order, style profiles if any, scoring terms, tie-break method, known weaknesses, benchmark numbers, legality tests, no-leak tests when relevant, explanation examples, and public suitability.

Every bot must have many-seed legality tests, determinism tests, explanation smoke tests, simulation tests, and latency benchmarks.

Hidden-information bots additionally require no-leak tests for bot view, explanations, candidate rankings, serialized payloads, logs, previews, and sampling code.

## 18. Acceptance checklist

Before a bot becomes a default public opponent, verify:

- it uses normal legal action API;
- it never receives unauthorized hidden state;
- it validates through normal engine path;
- legality tests pass over many seeds;
- determinism tests pass;
- latency benchmarks pass or documented UI thinking feedback exists;
- explanations are clear and viewer-safe;
- candidate rankings are dev-only and viewer-safe;
- Level 2 bot has strategy evidence pack;
- no weight soup exists;
- no public MCTS/ISMCTS/Monte Carlo/ML/RL slipped in;
- docs and known weaknesses are honest.
