# <game_id> AI

Game ID: `<game_id>`

Implemented variant: `<variant>`

Rules version: `<rules_version>`

Last updated: YYYY-MM-DD

Prepared by: `<name/agent>`

## Purpose

This document is the per-game bot registry and status document. It records what bots exist, what information they access, how they explain decisions, how they are tested, and whether they are suitable for the public default.

It MUST NOT duplicate the full Level 2 strategy evidence pack. Link to the pack instead.

A Level 2 authored-policy bot requires completed `COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md` before coding.

## Bot summary

| Bot | Level | Policy/version | Supported seat range | Public default? | Information access | Status | Evidence |
|---|---:|---|---|---:|---|---|---|
| random legal | 0 | `<policy_version>` | `<min..max seats>` | no/yes | legal action tree only | required / implemented / tested | `<tests/benchmarks>` |
| baseline | 1 | `<policy_version>` | `<min..max seats>` | no/yes | allowed seat view | not planned / planned / implemented / tested | `<tests/benchmarks>` |
| authored policy | 2 | `<policy_version>` | `<min..max seats>` | no/yes | allowed seat view | blocked by evidence pack / planned / implemented / tested | `<pack_path>` |
| shallow deterministic search | 3 | `<policy_version>` | `<min..max seats>` | no/yes | allowed seat view | not allowed / ADR-needed / planned / implemented | `<evidence>` |

Public v1/v2 bots MUST NOT use MCTS, ISMCTS, Monte Carlo-style bots, ML, or RL.

## Level 0: random legal bot

Required for every official game.

| Item | Decision/evidence |
|---|---|
| legal action API used | `<api>` |
| deterministic seed behavior | `<seed_rule>` |
| action selection method | random legal among legal action paths using deterministic seed |
| simulation tests | `<tests/runs>` |
| legality tests | `<tests>` |
| replay/hash tests | `<tests>` |
| known limitations | random; not competent |
| public explanation text | `random legal choice` / `<text>` |
| benchmark evidence | `<benchmarks>` |
| N-seat orchestration | `<bot-vs-bot seat fill, skipped/eliminated seat handling, deterministic advance order>` |

## Level 1: rule-informed baseline bot

Required for serious public demos unless explicitly deferred by roadmap gate.

| Item | Decision/evidence |
|---|---|
| policy name/version | `<policy>` |
| decision order summary | `<summary>` |
| immediate tactics | win/block/forced compliance/material/resource gain/avoid immediate loss / `<game_specific>` |
| mandatory rule handling | `<handling>` |
| tie-break method | deterministic seeded tie-break / bounded tie-break |
| information access | allowed seat view only |
| per-seat policy specialization | none / `<seat/role/team-specific differences>` |
| opponent set handling | `<all opponents independently / nearest threat / table leader / team opponent logic>` |
| explanation examples | `<examples>` |
| tests | `<tests>` |
| benchmarks | `<benchmarks>` |
| public suitability | suitable / not suitable / constrained |

## Level 2: authored policy bot

Level 2 is the preferred public default for polished games when strategy matters.

Required evidence pack: `<path/to/BOT-STRATEGY-EVIDENCE-PACK.md>`

Competent-player analysis: `<path/to/COMPETENT-PLAYER.md>`

| Item | Summary only |
|---|---|
| policy name/version | `<policy>` |
| evidence pack status | missing / incomplete / complete / reviewed |
| phase model | `<summary>` |
| candidate extraction | `<summary>` |
| per-seat policy specialization | none / `<seat/role/team-specific differences>` |
| bot-vs-bot orchestration for N seats | `<seat assignment, deterministic skip/advance order, mirror-match handling>` |
| lexicographic priority vector | `<summary>` |
| bounded scoring tie-breakers | none / `<summary>` |
| deterministic seeded tie-break | `<summary>` |
| multi-winner/split metrics | none / `<standing, team result, split, side-pot, coalition, elimination metrics>` |
| style profiles | none / `<summary>` |
| explanation contract | `<summary>` |
| known weaknesses | `<summary>` |
| public default suitability | yes/no/constrained |

Do not code Level 2 before the evidence pack is complete.

## Level 3: shallow deterministic search

Allowed only for small perfect-information games where the foundation docs and benchmarks permit it.

| Requirement | Status | Evidence |
|---|---|---|
| perfect-information game | yes/no | `<evidence>` |
| small enough search space | yes/no | `<benchmarks>` |
| deterministic limits | yes/no | `<limits>` |
| documented evaluator | yes/no | `<doc>` |
| fallback policy | yes/no | `<policy>` |
| explanation says search was used | yes/no | `<example>` |
| no hidden-information search | yes/no/not applicable | `<tests>` |
| ADR required? | yes/no | `<reason>` |

## Exact information access table

| Information | Acting seat sees? | Teammate sees? | Opponent set sees? | Bot sees? | Public observer sees? | Tests/notes |
|---|---:|---:|---:|---:|---:|---|
| legal action tree | yes/no | yes/no/not applicable | yes/no | yes/no | yes/no | `<tests>` |
| public board/state | yes/no | yes/no/not applicable | yes/no | yes/no | yes/no | `<tests>` |
| own private hand/role/zone | yes/no/not applicable | yes/no/not applicable | no/not applicable | yes/no/not applicable | no/not applicable | `<tests>` |
| opponent private hand/role/zone | no/not applicable | no/not applicable | no/not applicable | no/not applicable | no/not applicable | `<tests>` |
| unrevealed deck/order/future random outcome | no/not applicable | no/not applicable | no | no | no | `<tests>` |
| private logs | yes/no/not applicable | yes/no/not applicable | no/not applicable | yes/no/not applicable | no/not applicable | `<tests>` |
| dev/test full state | no for public bot | no for public bot | no | no for public bot | no | `<tests>` |
| `<information>` | yes/no | yes/no | yes/no | yes/no | yes/no | `<tests>` |

Bots MUST NOT receive actual hidden information unavailable to the acting seat.

## Decision order summary

For each non-random bot, summarize order without duplicating full evidence.

| Bot | Decision order |
|---|---|
| baseline | 1. `<priority>` 2. `<priority>` 3. deterministic fallback |
| authored policy | see evidence pack; summary: `<summary>` |
| shallow search | search under strict limits, then fallback: `<summary>` |

## Style profiles

One strong default bot comes first. Optional profiles MAY be added later.

| Profile | Applies to bot | Policy variation | Hidden-info safe? | Status | Tests |
|---|---|---|---:|---|---|
| default | `<bot>` | strongest public policy | yes/no | `<status>` | `<tests>` |
| `<profile>` | `<bot>` | `<variation>` | yes/no | `<status>` | `<tests>` |

## Explanation examples

| Bot | Situation | Viewer class | Example explanation | Redaction needed? | Hidden-info safe? | Test |
|---|---|---|---|---:|---:|---|
| random legal | `<situation>` | `<viewer class>` | `<example>` | yes/no | yes/no | `<test>` |
| baseline | `<situation>` | `<viewer class>` | `<example>` | yes/no | yes/no | `<test>` |
| authored policy | `<situation>` | `<viewer class>` | `<example>` | yes/no | yes/no | `<test>` |

Public mode MAY show a small “why?” affordance or recent-bot-action explanation. Full candidate ranking is dev-mode only and MUST be viewer-safe.

## Known weaknesses

| Bot | Weakness | Why acceptable | Mitigation/future trigger |
|---|---|---|---|
| `<bot>` | `<weakness>` | `<rationale>` | `<mitigation>` |

Do not hide weaknesses behind magic weights.

## Tests

| Test | Purpose | Bot(s) | Status | Notes |
|---|---|---|---|---|
| legality over seeds | bot chooses only legal action paths | all | not started / partial / covered | `<notes>` |
| determinism | fixed seed/view/rules/policy/limits produce fixed decision | non-random | not started / partial / covered | `<notes>` |
| no-leak input view | hidden-info games only | all public bots | not applicable / not started / covered | `<notes>` |
| no-leak explanation/ranking | hidden-info games only | non-random | not applicable / not started / covered | `<notes>` |
| explanation redaction per viewer | hidden-info or asymmetric-view games only | non-random | not applicable / not started / covered | `<notes>` |
| N-seat orchestration determinism | bot-vs-bot or 3+ seat games only | all | not applicable / not started / covered | `<notes>` |
| replay/hash | bot decisions reproduce in replay | all | not started / partial / covered | `<notes>` |
| explanation smoke | non-random bots explain decisions | baseline/Level 2/Level 3 | not started / partial / covered | `<notes>` |
| decision examples | policy examples choose expected actions | Level 2 | not applicable / not started / covered | `<notes>` |

## Benchmarks

| Benchmark | Target | Current baseline | Bot(s) | Status | Notes |
|---|---:|---:|---|---|---|
| legal action generation | `<target>` | `<baseline>` | all | `<status>` | `<notes>` |
| candidate extraction | `<target>` | `<baseline>` | Level 2 | `<status>` | `<notes>` |
| bot decision latency | `<target>` | `<baseline>` | non-random | `<status>` | `<notes>` |
| playout throughput | `<target>` | `<baseline>` | all | `<status>` | `<notes>` |
| explanation generation | `<target>` | `<baseline>` | non-random | `<status>` | `<notes>` |
| WASM/browser smoke | `<target>` | `<baseline>` | public bots | `<status>` | `<notes>` |

## Simulation metrics

| Run | Bots | Seeds/games | Metrics recorded | Failures | Notes |
|---|---|---:|---|---|---|
| `<run>` | `<bots and seat assignments>` | `<count>` | completed games, terminal outcomes, per-seat standings, multi-winner/split outcomes, turn/action caps, illegal attempts, invariant failures, average length, playout throughput, bot latency, failing seed command streams | `<failures>` | `<notes>` |

## Public default suitability

| Check | Status | Notes |
|---|---|---|
| bot is legal and deterministic | yes/no | `<notes>` |
| bot does not look broken | yes/no | `<notes>` |
| bot is fair under information rules | yes/no/not applicable | `<notes>` |
| explanations are safe and useful | yes/no | `<notes>` |
| latency fits public UX | yes/no | `<notes>` |
| known weaknesses acceptable | yes/no | `<notes>` |
| public default decision | yes/no/constrained | `<notes>` |
