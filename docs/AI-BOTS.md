# Rulepath AI and Bots

Status: bot law and operational architecture.

Rulepath bots exist to make public games enjoyable, testable, and demonstrable. They are product opponents and simulation drivers, not a research-AI showcase.

The goal is competent, explainable, fair, human-plausible, non-superhuman play. The goal is not a generic bot that plays every game.

## 0. AI document owners

Each AI-related document has one owner/purpose. Do not duplicate full bot
doctrine across game docs; link to the owning document and record only the
game-specific status or evidence needed there.

| Document | Owner/purpose | Must not duplicate |
|---|---|---|
| `docs/AI-BOTS.md` | Repository bot law, allowed techniques, forbidden techniques, information boundary, explanation requirements, and public-bot acceptance. | Per-game bot registries, full strategy analysis, or policy evidence packs. |
| `COMPETENT-PLAYER.md` | Human-readable strategy analysis for one game/variant, checked against rules and used as Level 2 design input. | Repository bot law or implementation registry state. |
| `BOT-STRATEGY-EVIDENCE-PACK.md` | Formal Level 2 authored-policy design input: candidate extraction, priority order, information boundaries, tests, explanations, and benchmarks for one policy. | General bot doctrine or long-form player strategy prose. |
| `GAME-AI.md` | Per-game bot registry and status receipt: which bots exist, their levels/policy ids, public suitability, evidence links, tests, benchmarks, and known weaknesses. | Full Level 2 evidence pack or competent-player strategy prose. |
| `GAME-EVIDENCE.md` | Cross-template conformance receipt linking bot levels, bot policy ids, benchmark evidence, release state, and blockers. | Bot doctrine, policy design, or strategy prose. |

## 1. Core bot law

Bots MUST:

- consume the same legal action API as humans;
- choose an action path through normal validation;
- never mutate state directly;
- never choose illegal actions;
- be deterministic under seed, rules version, data version, view, policy version, and declared limits;
- never use information unavailable to a real player in that seat;
- produce viewer-safe explanations for non-random public bots;
- have legality tests, determinism tests, simulations, and latency benchmarks.

Testing tools may inspect internal state. Such tools are not public bots and MUST NOT implement the public bot trait.

## 2. Bot levels

| Level | Name | Required for | Allowed techniques | Required evidence |
|---:|---|---|---|---|
| 0 | Random legal | Every official game | Random choice among legal actions using deterministic seeded tie-break | many-seed legality tests, simulation support, minimal explanation. |
| 1 | Rule-informed baseline | Serious public demos | Obvious tactics: win now, block immediate loss, obey forced rules, avoid visibly nonsensical choices, prefer direct visible value | rule-informed tests, deterministic output, simple explanations, latency benchmark. |
| 2 | Authored policy | Polished public games | Candidate extraction, ordered tactical priorities, phase-aware policy nodes, lexicographic ranking, bounded tie-breakers, deterministic seeded tie-break | completed strategy evidence pack, explanation examples, simulations, benchmarks. |
| 3 | Shallow deterministic search | Small perfect-information games only | Minimax/alpha-beta/iterative deepening with strict deterministic limits and fallback | ADR only if broad; otherwise documented limits, evaluator tests, latency budgets, deterministic transposition behavior. |

Level 3 is not a default escalation path. It is allowed only where game size, perfect information, and benchmarks make it safe.

## 3. Explicit v1/v2 exclusions

Public v1/v2 MUST NOT use:

- MCTS;
- ISMCTS;
- Monte Carlo playout bots;
- ML policy/value models;
- reinforcement learning;
- LLM move selection at runtime.

Future use requires ADR addressing playout speed, action abstraction, deterministic seeding, memory, latency, hidden-information fairness, explanation quality, public UX, training/evaluation data, replay/hash implications, and why authored policy or shallow deterministic search is insufficient.

## 4. Information boundary

A bot MUST receive only the allowed bot view for its seat. It MUST NOT receive opponent hands, face-down identities, hidden commitments, unrevealed deck order, secret roles, private logs, future random outcomes, full-state shortcuts, or hidden-state-derived candidate rankings.

Hidden-information bots MAY use:

- public information;
- their own private information;
- rules and variant metadata;
- remembered observations from their own legal view;
- legal inference from public history;
- belief models over legal possibilities;
- sampled possibilities generated from legal information, not copied from actual hidden state.

They may infer. They may not peek.

No-leak tests are mandatory for hidden-information bots.

## 4A. N-player imperfect-information bots

N-player hidden-information games add multi-opponent pressure, but they do not
change public bot law. A bot for a 3+ seat imperfect-information game MAY use:

- its own authorized private view;
- public state and public history;
- history that was visible to that bot when it happened;
- rules and variant metadata;
- legal inference from public facts and that bot's own observations;
- broad rule-of-thumb belief categories, such as visible pressure, public pot
  odds, known card counts, or opponent posture inferred from public actions.

It MUST NOT use:

- actual hidden cards, commitments, roles, hands, melds, walls, or deck tails
  belonging to other seats;
- an unredacted replay or internal full trace as public bot input;
- DOM, storage, dev-panel, test fixture, or full-state peeking;
- candidate rankings derived from unauthorized hidden state;
- sampled hidden worlds copied from actual hidden state; or
- MCTS, ISMCTS, Monte Carlo playout/search, ML, RL, or runtime LLM move
  selection unless a later accepted ADR changes public bot law.

Multi-opponent policy notes are required for Level 1+ N-player hidden-info bots.
They must name the target opponent set, risk model, visible facts used for
inference, deterministic tie-breaks, and explanation redaction per viewer.
Belief is allowed as documented human-plausible inference over legal public
information; it is not a license to sample or inspect the real hidden world.

## 5. Game-specific strategy modules

Do not build “one bot plays every game.” The action-tree API is generic; strategy is game-specific.

Preferred flow:

```text
legal action tree
  -> candidate extraction
  -> ordered tactical priorities
  -> phase-aware policy nodes
  -> lexicographic ranking
  -> small bounded tie-breakers
  -> deterministic seeded tie-break
  -> chosen action path + viewer-safe explanation
```

`ai-core` may provide traits and policy composition utilities. `games/*` owns game-specific strategy.

## 6. Candidate model

A candidate is a legal action path plus policy annotations.

A candidate MAY include:

- action path;
- public label;
- phase;
- tactical tags;
- visible immediate effects;
- rule-mandatory flag;
- explanation fragments;
- priority vector;
- bounded tie-break score;
- deterministic tie-break key;
- timing;
- dev diagnostic notes.

A candidate MUST NOT include hidden information unavailable to the bot seat or viewer.

## 7. Policy node types

| Node | Purpose |
|---|---|
| Mandatory filter | Enforce forced rules before preference. |
| Immediate tactic | Win now, block immediate loss, avoid visible terminal loss. |
| Phase priority | Select different priorities by phase/round/state. |
| Positional preference | Prefer visible strategic structures. |
| Resource/accounting policy | Evaluate visible costs/gains. |
| Denial policy | Block visible opponent threats. |
| Risk posture | Cautious/aggressive style under allowed information. |
| Bounded evaluator | Small tie-break after higher priority categories. |
| Fallback | Ensure a legal choice exists, usually random legal among finalists. |

Nodes SHOULD be small enough to test and explain.

## 8. Lexicographic priorities over weight soup

Prefer lexicographic priority vectors over large weighted sums.

Example order:

```text
terminal win
avoid terminal loss
mandatory rule compliance
immediate tactical gain
opponent denial
position/tempo preference
style preference
small bounded tie-break
seeded random tie-break
```

A better earlier category beats later categories. This makes behavior testable and explanations intelligible.

Small scoring is allowed only as a bounded tie-break after higher priorities. It MUST be documented, game-specific, tested, explainable, and benchmarked if costly.

Forbidden weight soup:

- dozens of magic weights without priority rationale;
- style profiles implemented only by multiplying weights;
- tactical conditions hidden in static data;
- scores that cannot produce explanations;
- knob tuning without simulations and benchmarks.

## 9. Competent-player document intake

For games with meaningful strategy, a competent-player document SHOULD be produced before a Level 2 bot.

The bot author must convert it into a Level 2 strategy evidence pack. The pack MUST include:

```text
game id / rules version / variant
sources and play observations
competent-player principles
novice traps
phase model
tactical priority order
candidate extraction plan
candidate features and visible facts
lexicographic ranking plan
bounded tie-breakers
risk posture/personality knobs
forbidden hidden information
belief model if hidden information exists
sample decisions
sample explanations
known weaknesses
difficulty limits if any
test plan
benchmark plan
public UX note
policy versioning plan
```

No evidence pack, no Level 2 bot.

## 10. Explanation contract

Every non-random bot decision SHOULD produce a viewer-safe explanation with:

- bot policy name/version;
- chosen priority reason;
- relevant visible fact;
- tie-break note if applicable;
- hidden-info disclaimer when relevant;
- search/limit note if Level 3 was used;
- fallback note if fallback was used.

Good:

```text
Chose the center column because no immediate win or block exists and this policy prefers central threats.
```

Bad:

```text
Chose this because score = 37.42.
```

Forbidden:

```text
Folded because the opponent has the ace.
```

Unless the ace is publicly visible or legally known to that bot and viewer, that explanation leaks hidden information.

## 11. Personality and difficulty

One strong default bot personality comes first. Optional profiles MAY come later.

Profiles SHOULD vary:

- policy order;
- risk posture;
- candidate preferences;
- bounded tie-breakers;
- allowed search limits where Level 3 is legal;
- explanation tone.

Profiles MUST NOT cheat, inspect hidden state, inject arbitrary random blunders by default, hide huge weight soup, or bypass validation.

Difficulty MAY vary through reduced tactical layers, smaller limits, documented imperfect heuristics, or weaker priority ordering. Difficulty MUST NOT vary through hidden-state access.

## 12. Hidden-information bot policy

Hidden-information bot docs MUST state:

- exact input view;
- memory model;
- belief or sampling model if any;
- forbidden information;
- no-leak tests;
- explanation redaction rules;
- candidate-ranking redaction rules;
- replay/export safety checks.

Sampled determinizations, if ever allowed by ADR, must be generated from the bot’s legal information. They must not copy actual hidden state.

## 13. Public and dev affordances

Public mode SHOULD expose a small “why?” or recent-bot-action affordance for non-random bots. It should be concise and should not turn the game into a debug console.

Dev mode MAY show candidate rankings, priority vectors, selected node, timings, tie-break seeds/counters, filtered counts, and fallback flags.

Dev output MUST be viewer-safe. Candidate rankings MUST NOT contain hidden facts or hidden-state-derived labels.

## 14. Tests and benchmarks

Every bot needs:

- many-seed legality tests;
- deterministic output tests;
- normal validation path tests;
- explanation smoke tests for non-random bots;
- simulation tests;
- latency benchmarks.

Hidden-information bots additionally require no-leak tests for bot view, memory, belief model, explanations, candidate rankings, serialized payloads, logs, previews, and replay exports.

Non-trivial bots SHOULD benchmark legal action generation, candidate extraction, action selection latency, playout throughput, allocations where practical, and serialization/replay overhead where relevant.

Native Rust benchmarks are primary. Browser/WASM measurements are smoke evidence.

## 15. Acceptance check for public bots

Before a bot becomes a default public opponent, verify:

- it uses the normal legal action API;
- it validates through the normal engine path;
- it receives only the allowed view;
- it is deterministic under declared inputs;
- legality tests pass over many seeds;
- latency fits the public UX or uses documented thinking feedback;
- explanations are clear and viewer-safe;
- Level 2 has a strategy evidence pack;
- hidden-information games have no-leak tests;
- no weight soup exists;
- no public MCTS/ISMCTS/Monte Carlo/ML/RL slipped in;
- docs and known weaknesses are honest.
