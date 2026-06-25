# Blackglass Pact Bot Strategy Evidence Pack

Game ID: `blackglass_pact`

Implemented variant: `blackglass_pact_standard`

Rules version: `blackglass-pact-rules-v1`

Bot target: Level 2 authored policy

Policy name/version: not admitted

Date: 2026-06-25

## Purpose And Gate

This is the formal status record for a future Level 2 Blackglass Pact bot.
Gate 18 does not admit Level 2. A Level 2 bot must not be coded until this
pack is completed, reviewed, and accepted in a later bounded task.

The shipped bots are Level 0 random legal and Level 1 bounded baseline only.
[AI.md](AI.md) is the current bot registry.

Decision: `L2 not admitted / intentionally deferred`

Level 3 status: `L3 not applicable`

Blackglass Pact is an imperfect-information partnership game because unplayed
hands, partner hands, opponent hands, and future deal identity are not visible
to every seat. Public v1/v2 shallow search over hidden worlds is not allowed.

## Explicit Public V1/V2 Exclusions

No public Blackglass Pact bot may use omniscient state, hidden-state shortcuts,
future random outcomes, unbounded weight soup, static-data tactical conditions,
random blunder injection, MCTS, ISMCTS, Monte Carlo-style bots,
determinization, sampled hidden worlds, ML, RL, runtime LLM policy,
partner/opponent hand enumeration, future-deal peeking, or seed/deck
reconstruction.

## Source Documents Consumed

| Document/source | Path/reference | Required? | Status | Notes |
|---|---|---:|---|---|
| rules | [RULES.md](RULES.md) | yes | read | Stable `BP-*` rule IDs. |
| sources | [SOURCES.md](SOURCES.md) | yes | read | Source/IP and strategy-prior-art posture. |
| competent-player analysis | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | yes | informative only | Strategy candidates exist but are not accepted Level 2 policy. |
| bot registry | [AI.md](AI.md) | yes | read | L0/L1 only. |
| implementation | `games/blackglass_pact/src/bots.rs` | yes | read | Current L1 is bounded baseline, not L2. |
| tests | `games/blackglass_pact/tests/bots.rs` | yes | read | L1 behavior and no-leak canaries. |

## Evidence Pack Status

Decision: `L2 not admitted / intentionally deferred`

| Area | Status | Required future work |
|---|---|---|
| accepted competent-player taxonomy | partial | Review and accept [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) as Level 2 design input. |
| exact policy ID/version | not assigned | Freeze a Level 2 policy id and version before code. |
| authorized input/memory schema | not implemented | Freeze every input field and memory field with no-leak tests. |
| deterministic priority vector | not implemented | Define lexicographic priorities and stable tie-breaks by phase. |
| fixed scenario corpus | not complete | Cover ordinary contracts, nil, blind nil, bags, target posture, partner states, and opponent states. |
| L0/L1 evaluation | incomplete | Compare against L0 and L1 with fixed seed manifests and team/seat balance. |
| calibration by decision type | not complete | Separate bidding, nil selection, play, and score-posture calibration. |
| no-leak proof | partial | Prove inputs, explanations, candidate rankings, replays, exports, and browser surfaces. |
| legality/replay/hash evidence | incomplete | Decisions must validate through normal command path and replay deterministically. |
| benchmark evidence | not complete | Add Level 2 decision/ranking and full playout benchmark lanes. |
| code/dependency review | incomplete | Prove absence of forbidden methods and hidden-state peeking. |
| implementation admission update | not complete | Update admission/evidence docs after all rows pass. |

## Exact Bot Input View For Future L2

| Input item | Included? | Source | Visible to acting seat? | Notes/no-leak test |
|---|---:|---|---:|---|
| legal action paths | yes | Rust legal action API | yes | Candidate base; no constructed illegal moves. |
| public view | yes | Rust projection | yes | Scores, bags, dealer, public bids, public plays, trick winners. |
| acting seat private hand | yes after deal | Rust seat-private projection | yes | Own cards only. |
| blind-phase score/order facts | yes before deal | public setup state | yes | No hand or future card fields exist. |
| visible command/effect history | possible | viewer-filtered logs | yes | Must not include full trace or hidden hands. |
| public void/card-memory summary | possible | public plays only | yes | Must be reconstructible by the viewer. |
| policy seed/tie-break state | yes | bot framework | not game info | Deterministic only. |
| partner private hand | no | forbidden | no | Mutation-invariance/no-leak tests. |
| opponent private hands | no | forbidden | no | Mutation-invariance/no-leak tests. |
| future deal/deck order | no | forbidden | no | Blind no-leak and seed/deck exclusion tests. |
| actual sampled hidden state | no | forbidden | no | No Monte Carlo/determinization. |

## Required Future Scenario Corpus

| Corpus area | Required cases | Rule IDs | Evidence status |
|---|---|---|---|
| ordinary contract | low/high contract, made exact, overtricks, set contract | `BP-BID-*`, `BP-SCORE-001` through `BP-SCORE-006` | future |
| nil | safe nil, unsafe high-spade nil, failed nil, partner cover | `BP-BID-003`, `BP-SCORE-007`, `BP-SCORE-009` | future |
| blind nil | 100/200/300 point deficits, partner already declared, target pressure | `BP-BLIND-*`, `BP-SCORE-008` | future |
| bags | near-threshold avoidance, deliberate pressure, multiple threshold rollover | `BP-SCORE-011` through `BP-SCORE-014` | future |
| target | near 500, exact tie, negative score recovery, unique higher team | `BP-END-*` | future |
| partner/opponent states | partner nil alive/failed, opponent nil alive, public voids | `BP-VIS-003`, `BP-BOT-002` | future |

## Candidate Extraction Plan

Candidate extraction is not accepted yet. Future work may evaluate:

| Candidate group | Extraction rule | Rule IDs | Visible facts used | Hidden-info risk | Tests |
|---|---|---|---|---|---|
| blind candidates | legal `blind_nil/*` leaves annotated by score deficit, partner public declaration, and target posture | `BP-BLIND-*` | public score/order only | low | pre-deal no-card tests |
| bid candidates | legal `bid/*` leaves annotated by own-hand controls, nil risk, partner public bid, score/bag posture | `BP-BID-*`, `BP-SCORE-*` | own hand plus public facts | medium | own-view and partner-hand mutation tests |
| lead candidates | legal `play/<card>` leaves annotated by spades-broken state, contract need, nil/bag posture | `BP-PLAY-*` | own hand plus public facts | medium | lead examples |
| follow candidates | legal cards annotated by current winner, cover/set/protect goals | `BP-PLAY-*`, `BP-SCORE-*` | own hand and public trick | medium | current-trick tests |
| endgame candidates | legal actions annotated by public scores, bags, target, and remaining public hand context | `BP-END-*` | public scores/history only | medium | target/bag examples |

## Phase Model

| Phase/situation | Detection from allowed input | Policy status | Rule IDs | Notes |
|---|---|---|---|---|
| blind commitment | legal blind leaves exist before deal | L1 shallow only | `BP-BLIND-*` | L2 not admitted. |
| bidding | legal bid leaves exist | L1 shallow only | `BP-BID-*` | Future L2 needs accepted bid calibration. |
| lead | current trick empty and legal play leaves exist | future | `BP-PLAY-001` through `BP-PLAY-003` | Lead-control value is not modeled by L1. |
| follow suit | current trick non-empty and legal set follows led suit | future | `BP-PLAY-005` | Current L1 only chooses lowest legal card. |
| void play | current trick non-empty and legal set includes off-suit cards | future | `BP-PLAY-006` | Ruff/shed choices need no-leak tests. |
| late hand | public completed tricks and remaining hand count | future | `BP-SCORE-*` | Requires public card memory. |
| terminal/endgame | public scores and bags near target | future | `BP-END-*` | No arbitrary tiebreak or hidden posture. |

## Lexicographic Priority Vector

Not admitted. Future Level 2 should prefer lexicographic priorities such as:

1. Stay legal and satisfy mandatory rules.
2. Protect live own nil when legally possible.
3. Cover live partner nil when public facts justify it and ordinary contract
   risk is bounded.
4. Set opponent nil when it does not sacrifice a higher team priority.
5. Make the ordinary team contract.
6. Avoid unnecessary bags once contract safety is public.
7. Account for public target/score posture.
8. Use public card-memory and void inference only from public play.
9. Deterministic tie-break by candidate identity and seed.

No numeric vector is accepted by this document.

## Current Bounded L1 Values

The current Level 1 values are authored in code and described in [AI.md](AI.md);
they are not static game data and are not Level 2 evidence:

| Policy area | Current L1 value | Source |
|---|---|---|
| blind threshold | declare at deficit >= 300, or deficit >= 200 when partner has not declared and opponent score is below 450 | `games/blackglass_pact/src/bots.rs` |
| nil screen | own hand has no spades and no ace/king/queen | `games/blackglass_pact/src/bots.rs` |
| numeric bid estimate | own aces/kings plus spade length over three, clamped 1-13 | `games/blackglass_pact/src/bots.rs` |
| play choice | lowest legal card by rank value then card index | `games/blackglass_pact/src/bots.rs` |

## Forbidden Hidden Information

| Information | Why forbidden | Potential leak surface | Required future no-leak test |
|---|---|---|---|
| partner hand | unavailable to acting seat | input view, ranking, explanation | partner-hand mutation invariance |
| opponent hands | unavailable to acting seat | input view, ranking, explanation | opponent-hand mutation invariance |
| future deal/deck order | unavailable before and after deal as public bot input | blind policy, bidding features, replay export | seed/deck exclusion test |
| actual sampled hidden state | hidden-state shortcut | belief/determinization | source scan and policy audit |
| private logs/dev state | unauthorized | dev inspector, candidate ranking | viewer redaction tests |

## Memory And Belief Model

| Item | Decision | Allowed inputs | Forbidden inputs | Tests |
|---|---|---|---|---|
| memory model | not admitted | future public history and own private observations only | actual hidden state | future |
| belief model | none | not applicable | sampled hidden states, partner/opponent hands, future deal identity | future |
| redaction model | not admitted | viewer-safe facts | hidden facts | future |

## Required Evaluation Before Level 2

| Evidence row | Required content | Status |
|---|---|---|
| accepted competent-player taxonomy | reviewed strategy principles and novice traps | incomplete |
| policy id/version | stable Level 2 id and version string | incomplete |
| authorized input/memory schema | exact fields and redaction model | incomplete |
| priority vector and tie-breaks | deterministic phase-specific order | incomplete |
| scenario corpus | ordinary, nil, blind, bags, target, partner, opponent cases | incomplete |
| L0/L1 comparison | fixed seeds, balanced seats/teams, recorded manifests | incomplete |
| calibration | separated bidding, nil, play, score posture | incomplete |
| no-leak proof | inputs, explanations, candidates, replays, browser surfaces | incomplete |
| legality/replay/hash | normal validation and deterministic replay/hash proof | incomplete |
| benchmark evidence | native decision/ranking and full playout lanes | incomplete |
| code/dependency review | absence of MCTS, ISMCTS, Monte Carlo, determinization, sampled worlds, ML, RL, runtime LLM, and hidden-state peeking | incomplete |
| admission update | accepted implementation-admission and evidence receipt changes | incomplete |

## Public UX Note

Until Level 2 exists, the public UI may show the shipped Level 1 explanation as
a modest safe reason. It must not market the bot as expert, optimal, strategic
authority, or competent-human equivalent.

## Review Checklist

- Level 2 is explicitly not admitted / intentionally deferred.
- Level 3 is explicitly not applicable.
- Current L1 is not represented as Level 2.
- No forbidden AI/search method is allowed.
- Future work is testable and no-leak aware.
