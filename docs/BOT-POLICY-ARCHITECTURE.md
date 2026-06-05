# Bot Policy Architecture

Status: operational architecture for non-MCTS bots.

This document defines the preferred architecture for Rulepath's public bots: game-specific, explainable, deterministic, fair, and benchmarked.

## 1. Design target

Default public bots should feel competent without becoming inscrutable.

Preferred style:

```text
legal action tree
  -> candidate extraction
  -> ordered tactical priorities
  -> phase-aware policy nodes
  -> lexicographic ranking
  -> bounded scoring tie-breakers only when useful
  -> deterministic seeded tie-break
  -> chosen action path + explanation
```

Do not build a generic “one bot to play every game”. The legal action API is generic; strategy is game-specific.

## 2. Crate ownership

| Location | Owns |
|---|---|
| `ai-core` | bot traits, random legal bot, deterministic RNG utilities, policy-node interfaces, lexicographic rank helpers, instrumentation, candidate ranking structures, decision limits |
| `games/*/bots.rs` | game-specific tactical policies, phase policies, style profiles, scoring tie-breakers, explanation text, tests |
| `tools/simulate` | playout harness, seed failure reporting, benchmark integration |
| `apps/web` | display of bot name, explanation, timing, candidate ranking in dev mode |

`ai-core` MUST NOT contain game-specific strategy.

## 3. Candidate model

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
- tie-break score;
- seed tie-break key;
- diagnostic notes for dev mode.

A candidate MUST NOT include hidden information unavailable to the bot seat.

## 4. Policy node types

Useful policy node shapes:

| Node type | Purpose | Example |
|---|---|---|
| mandatory filter | obey forced rules | forced capture, follow suit |
| immediate tactic | tactical one-ply rule | win now, block immediate loss |
| phase priority | choose plan by phase | build, attack, cleanup |
| positional preference | prefer stable locations | center columns, corners |
| resource policy | value costs/gains | buy if net score improves |
| risk posture | style variation | cautious avoids risky reveal |
| denial policy | block opponent | deny winning column |
| bounded evaluator | small tie-break | material count after visible move |
| fallback | ensure action exists | random legal among remaining |

Nodes SHOULD be small enough to explain.

## 5. Lexicographic priorities

Prefer lexicographic priority vectors over large weighted sums.

Example shape:

```text
priority = [
  terminal_win_rank,
  avoid_terminal_loss_rank,
  mandatory_rule_rank,
  tactical_gain_rank,
  positional_rank,
  style_rank,
  random_tie_break_rank
]
```

A candidate with a better earlier category wins regardless of later categories.

This makes bots easier to test and explain.

## 6. Explanation contract

Every non-random bot decision SHOULD produce an explanation.

Explanation SHOULD include:

- policy name/version;
- chosen priority reason;
- relevant visible fact;
- tie-break note if applicable;
- hidden-info disclaimer if relevant.

Good:

```text
Chose column 4 because it wins immediately with a vertical line.
```

Good:

```text
Drew instead of standing because the visible total is 11 and the policy cannot see the next card.
```

Bad:

```text
Chose this because score = 37.42.
```

Bad:

```text
Folded because opponent has a flush.
```

unless the opponent's flush is publicly visible.

## 7. Style profiles

Style profiles are policy variants, not weight dumps.

| Profile | Implementation method |
|---|---|
| cautious | prefer safe lines, avoid high-variance choices, tie-break toward defensive actions |
| aggressive | prioritize threats and tempo after immediate safety rules |
| greedy | prefer immediate visible gain after mandatory/terminal checks |
| blocking/defensive | prioritize opponent threat denial |
| opportunistic | prefer actions that create multiple visible threats |
| risk-seeking | accept bounded risk under public/belief model |
| learner-friendly | simplified policy and explanation, explicit mode only |

Profiles MUST NOT cheat.

## 8. Hidden-information policy structure

For hidden-information games, a policy module MUST state its information model:

| Model | Allowed? | Notes |
|---|---|---|
| public + own private view | yes | default fair model |
| belief model from observed actions | yes | document memory and assumptions |
| sampled legal possibilities | yes | samples must be generated from legal information, not actual hidden state |
| actual hidden state | no | banned for bots |
| future deck order | no unless visible/known by rules | banned otherwise |

No-leak tests are mandatory.

## 9. Search policy

Shallow deterministic search is Level 3 and limited.

Search MAY be added only when:

- the game is small and perfect-information;
- action space is bounded;
- evaluator is documented;
- decision limits are deterministic;
- latency benchmarks pass;
- fallback policy exists;
- explanation can say search was used.

Search SHOULD NOT be used in hidden-information public bots in v1/v2.

MCTS/ISMCTS requires future ADR and is not part of public v1/v2.

## 10. Debug candidate ranking

Dev mode SHOULD show:

- all considered legal candidates or a capped representative set;
- priority vector;
- chosen policy node;
- tie-break seed/counter;
- timing;
- reason;
- filtered candidate count if large.

Debug output MUST be viewer-safe. It must not reveal hidden information or candidate evaluations based on hidden state.

## 11. Tests

Every bot MUST have:

- legal-action tests over many seeds;
- determinism test for fixed seed/state/view;
- explanation smoke test;
- benchmark coverage for decision latency;
- simulation test with failure seed output.

Hidden-information bots additionally MUST have:

- bot-view no-leak tests;
- explanation no-leak tests;
- candidate-ranking no-leak tests;
- belief/sampling no-read-actual-hidden-state tests.

## 12. Stage examples

### `column_four` Level 2

Priority order:

1. win immediately;
2. block opponent immediate win;
3. create two-way threat;
4. block visible two-way threat;
5. prefer center columns;
6. prefer lower-risk columns;
7. seeded tie-break.

### `directional_flip` Level 2

Priority order:

1. take stable corner if legal;
2. avoid visible corner giveaway if known by simple heuristic;
3. maximize flips as a tie-break;
4. prefer mobility-preserving move;
5. seeded tie-break.

### `poker_lite` Level 2

Priority order:

1. obey betting legality;
2. use own cards plus public cards only;
3. estimate hand strength from visible/own information;
4. apply risk posture;
5. avoid pot/accounting mistakes;
6. seeded tie-break.

No policy may inspect opponent hole cards.

## 13. Acceptance checklist

Before a bot becomes default public opponent:

- it uses normal legal action API;
- it never receives unauthorized hidden state;
- it has legality tests over many seeds;
- it has deterministic tests;
- it has latency benchmarks;
- it has explanation examples;
- it has documented known weaknesses;
- style profile behavior is bounded and understandable;
- candidate ranking is safe in dev mode;
- no giant weight soup exists.

## Source notes

See `SOURCES.md`, especially Board Game Arena bot guidance, OpenSpiel, Regular Boardgames, Regular Games, and testing/benchmark sources.
