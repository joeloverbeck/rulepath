# Briar Circuit Bot Strategy Evidence Pack

Game ID: `briar_circuit`

Implemented variant: `briar_circuit_standard`

Rules version: `briar-circuit-rules-v1`

Bot target: Level 2 authored policy

Policy name/version: not admitted

Date: 2026-06-21

## Purpose And Gate

This is the formal status record for a future Level 2 Briar Circuit bot.
Gate 16 does not admit Level 2. A Level 2 bot must not be coded until this pack
is completed, reviewed, and accepted in a later bounded task.

The shipped bots are Level 0 random legal and Level 1 bounded baseline only.
[AI.md](AI.md) is the current bot registry.

## Explicit Public V1/V2 Exclusions

No public Briar Circuit bot may use omniscient state, hidden-state shortcuts,
future random outcomes, unbounded weight soup, static-data tactical conditions,
random blunder injection, MCTS, ISMCTS, Monte Carlo-style bots, ML, RL, runtime
LLM policy, opponent-hand enumeration, or seed/deck reconstruction.

## Source Documents Consumed

| Document/source | Path/reference | Required? | Status | Notes |
|---|---|---:|---|---|
| rules | [RULES.md](RULES.md) | yes | read | Stable `BC-*` rule IDs. |
| coverage | [RULE-COVERAGE.md](RULE-COVERAGE.md) | yes | read | Current implementation evidence. |
| competent-player analysis | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | yes | incomplete for L2 | Strategy candidates exist but are not accepted policy. |
| mechanics | [MECHANICS.md](MECHANICS.md) | yes | read | Trick-taking second-use pressure recorded. |
| bot registry | [AI.md](AI.md) | yes | read | L0/L1 only. |
| benchmarks | [BENCHMARKS.md](BENCHMARKS.md) | yes | read | L0/L1 decision and full-hand lanes exist. |

## Evidence Pack Status

Decision: `L2 not admitted`

| Area | Status | Required future work |
|---|---|---|
| exact bot input view | partial | Freeze an allowed input struct/view and no-leak tests for every field. |
| candidate extraction | not implemented | Map legal actions to candidate features from own/public facts only. |
| phase model | partial | Separate pass, early trick, mid hand, moon-risk, threshold/endgame nodes. |
| lexicographic priority vector | not implemented | Define and test ordered priorities; avoid weight soup. |
| bounded scoring tie-breakers | not implemented | Name small ranges and explanation fragments. |
| deterministic tie-break | not implemented | Define candidate identity and seed rule. |
| style profiles | not planned | One strong default first. |
| explanation contract | partial | Must cite visible facts and redact by viewer. |
| simulation evidence | not complete for L2 | Run many seeded games and compare to L1 safely. |
| benchmark evidence | partial | Existing L1 lanes; future Level 2 requires own decision/ranking lanes. |

## Exact Bot Input View For Future L2

| Input item | Included? | Source | Visible to acting seat? | Notes/no-leak test |
|---|---:|---|---:|---|
| legal action paths | yes | Rust legal action API | yes | Must be the candidate base. |
| public view | yes | Rust projection | yes | Scores, current trick, captured history, hand counts. |
| acting seat private hand | yes | Rust seat-private projection | yes | Own cards only. |
| visible command/effect history | possible | viewer-filtered logs | yes | Must not include pass provenance. |
| policy seed/tie-break state | yes | bot framework | not game info | Deterministic only. |
| hidden opponent hands | no | forbidden | no | Mutation-invariance/no-leak tests. |
| deck order/future deals | no | forbidden | no | Export and bot-input tests. |
| actual sampled hidden state | no | forbidden | no | No Monte Carlo/determinization. |

## Candidate Extraction Plan

Candidate extraction is not accepted yet. Future work may evaluate:

| Candidate group | Extraction rule | Rule IDs | Visible facts used | Hidden-info risk | Tests |
|---|---|---|---|---|---|
| pass candidates | legal pass select/confirm actions annotated with own-card point value and suit length | `BC-PASS-002` | own hand only | low | own-view candidate tests |
| follow candidates | legal play actions annotated with whether they win current trick and point exposure | `BC-PLAY-002`, `BC-TRICK-001` | own card, public trick | medium | no opponent-hand invariance |
| discard candidates | legal void/off-suit plays annotated by point burden and public moon risk | `BC-PLAY-003`, `BC-SCORE-003` | own card, public captured points | medium | public-history-only tests |
| endgame candidates | legal actions annotated by public standings near threshold | `BC-MATCH-002`, `BC-MATCH-003` | public scores/history | medium | kingmaking examples |

## Phase Model

| Phase/situation | Detection from allowed input | Policy status | Rule IDs | Notes |
|---|---|---|---|---|
| pass selection | `phase = passing`, own selected count | future | `BC-PASS-*` | Avoid keeping high-risk points; not accepted. |
| opening trick | current trick empty, trick index 0 | future | `BC-PLAY-001`, `BC-PLAY-004` | Mostly forced. |
| follow suit | led suit exists and legal set follows suit | future | `BC-PLAY-002` | Determine cheap win/duck. |
| void discard | legal set allows off-suit | future | `BC-PLAY-003` | Candidate moon-risk guard needed. |
| hearts lead | current trick empty, hearts state known | future | `BC-PLAY-005`, `BC-PLAY-006` | Own-hand only. |
| threshold/endgame | public score near/over 100 | future | `BC-MATCH-*` | Avoid hidden intent modeling. |

## Lexicographic Priority Vector

Not admitted. Future Level 2 should prefer lexicographic priorities such as:

1. Stay legal and satisfy mandatory rules.
2. Avoid self-capturing high public point load.
3. Prevent publicly visible moon when safe.
4. Manage threshold/endgame standing from public scores.
5. Reduce own hand risk using own private hand only.
6. Deterministic tie-break by candidate identity and seed.

No numeric vector is accepted by this document.

## Forbidden Hidden Information

| Information | Why forbidden | Potential leak surface | Required future no-leak test |
|---|---|---|---|
| opponent hand | unavailable to acting seat | input view, ranking, explanation | opponent-hand mutation invariance |
| pass provenance | private after exchange | ranking, explanation, export | pass-origin canary scan |
| deck order/future deal | hidden setup fact | policy features, replay export | seed/deck exclusion test |
| sampled hidden state | actual hidden shortcut | belief/determinization | source scan and policy audit |
| private logs/dev state | unauthorized | dev inspector, candidate ranking | viewer redaction tests |

## Memory And Belief Model

| Item | Decision | Allowed inputs | Forbidden inputs | Tests |
|---|---|---|---|---|
| memory model | not admitted | future visible public history only | actual hidden state | future |
| belief model | none | not applicable | sampled hidden states, opponent hands | future |
| redaction model | not admitted | viewer-safe facts | hidden facts | future |

## Explanation Contract

Future Level 2 explanations must name:

- policy name/version;
- chosen priority reason;
- visible fact used;
- tie-break note when relevant;
- redaction when a fact is seat-private;
- known weakness only as product-safe copy.

They must not name opponent cards, pass provenance, hidden deck material,
sampled possibilities, actual hidden-state facts, or strategy advice derived
from hidden information.

## Decision Examples

Decision examples are intentionally not accepted. Candidate examples in
[COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) must be converted into tests before
they can drive code.

## Test Plan Required Before Level 2

| Test area | Required? | Specific tests | Evidence |
|---|---:|---|---|
| legality | yes | chosen action validates through normal command path | future |
| determinism | yes | fixed seed/view/rules/policy produce fixed decision | future |
| no hidden-state access | yes | mutate opponent hands/pass provenance and assert decision/explanation stability where hidden | future |
| candidate extraction | yes | candidate annotations match legal actions and visible facts | future |
| priority vector | yes | examples hit expected priority slots | future |
| explanations | yes | viewer-safe public and seat-private explanations | future |
| simulation/fuzz | yes | many seeded games with failure reports | future |
| replay/hash | yes | decisions reproduce in replay | future |
| benchmark | yes | `level2_action_selection` and full playout lanes | future |

## Public UX Note

Until Level 2 exists, the public UI may show the shipped Level 1 explanation as
a modest safe reason. It must not market the bot as expert, strategic, or
competent-human equivalent.

## Review Checklist

- Level 2 is explicitly not admitted.
- Current L1 is not represented as Level 2.
- No forbidden AI/search method is allowed.
- Future work is testable and no-leak aware.
