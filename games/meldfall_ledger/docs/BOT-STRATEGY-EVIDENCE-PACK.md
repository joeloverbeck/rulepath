# Meldfall Ledger Bot Strategy Evidence Pack

Game ID: `meldfall_ledger`

Implemented variant: `classic_500_single_deck_v1`

Rules version: `meldfall-ledger-rules-v1`

Bot target: Level 2 authored policy

Policy name/version: not admitted

Date: 2026-06-26

## Purpose And Gate

This is the formal status record for a future Level 2 Meldfall Ledger bot.
Gate 19 does not admit Level 2. A Level 2 bot must not be coded until this
pack is completed, reviewed, and accepted in a later bounded task.

The shipped bot is Level 0 random legal only. [AI.md](AI.md) is the current bot
registry.

Decision: `L2 not admitted / intentionally deferred`

Level 1 status: `not_admitted_pending_strategy_evidence`

Level 3 status: `L3 not applicable`

Meldfall Ledger is an imperfect-information multi-opponent meld game because
opponent hands, hidden stock order, next stock card, and some settlement card
identities are not visible to every seat. Public v1/v2 shallow search over
hidden worlds is not allowed.

## Explicit Public V1/V2 Exclusions

No public Meldfall Ledger bot may use omniscient state, hidden-state shortcuts,
future random outcomes, unbounded weight soup, static-data tactical conditions,
random blunder injection, MCTS, ISMCTS, Monte Carlo rollouts or search,
Monte Carlo-style bots, determinization, sampled hidden worlds, machine
learning, reinforcement learning, runtime LLM policy, opponent hand
enumeration, stock-order peeking, future-card peeking, seed/deck
reconstruction, or hidden-state-derived candidate rankings.

## Source Documents Consumed

| Document/source | Path/reference | Required? | Status | Notes |
|---|---|---:|---|---|
| rules | [RULES.md](RULES.md) | yes | read | Stable `ML-*` rule IDs. |
| sources | [SOURCES.md](SOURCES.md) | yes | read | Source/IP and strategy-prior-art posture. |
| competent-player analysis | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | yes | informative only | Strategy candidates exist but are not accepted Level 2 policy. |
| bot registry | [AI.md](AI.md) | yes | read | L0 only; L1 and L2 not admitted. |
| implementation | `games/meldfall_ledger/src/bots.rs` | yes | read | Current bot is random legal over viewer-authorized actions. |
| tests | `games/meldfall_ledger/tests/bots.rs` | yes | read | L0 behavior, input scope, and L1-not-admitted trace inventory. |

## Evidence Pack Status

Decision: `L2 not admitted / intentionally deferred`

| Area | Status | Required future work |
|---|---|---|
| accepted competent-player taxonomy | partial | Review and accept [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) as Level 2 design input. |
| exact policy ID/version | not assigned | Freeze a Level 2 policy id and version before code. |
| authorized input/memory schema | not implemented | Freeze every input field and memory field with no-leak tests. |
| deterministic priority vector | not implemented | Define lexicographic priorities and stable tie-breaks by phase. |
| fixed scenario corpus | not complete | Cover draw choice, pickup commitment, meld timing, lay-off timing, discard risk, go-out, scoring, stock exhaustion, target/tie pressure, and variable seat counts. |
| L0 evaluation | incomplete | Compare against L0 with fixed seed manifests and by-seat summaries. |
| calibration by decision type | not complete | Separate draw, table-play, discard, go-out, and score-posture calibration. |
| no-leak proof | partial | Prove inputs, explanations, candidate rankings, replays, exports, simulator summaries, and browser surfaces. |
| legality/replay/hash evidence | incomplete | Decisions must validate through normal command path and replay deterministically. |
| benchmark evidence | not complete | Add Level 2 decision/ranking and full playout benchmark lanes. |
| code/dependency review | incomplete | Prove absence of forbidden methods and hidden-state peeking. |
| implementation admission update | not complete | Update admission/evidence docs after all rows pass. |

## Exact Bot Input View For Future L2

| Input item | Included? | Source | Visible to acting seat? | Notes/no-leak test |
|---|---:|---|---:|---|
| legal action paths | yes | Rust legal action API | yes | Candidate base; no constructed illegal moves. |
| public view | yes | Rust projection | yes | Public melds, discard pile, stock count, hand counts, scores, active seat, dealer, phase. |
| acting seat private hand | yes | Rust seat-private projection | yes | Own cards only. |
| pickup commitment state | yes | Rust projection/action tree | yes | Required to finish table play safely. |
| visible command/effect history | possible | viewer-filtered logs | yes | Must not include full trace or hidden cards. |
| public discard/meld memory summary | possible | public table and public actions only | yes | Must be reconstructible by the viewer. |
| policy seed/tie-break state | yes | bot framework | not game info | Deterministic only. |
| opponent private hands | no | forbidden | no | Mutation-invariance/no-leak tests. |
| hidden stock order or next stock card | no | forbidden | no | Stock-order exclusion tests. |
| actual sampled hidden state | no | forbidden | no | No Monte Carlo/determinization. |
| private diagnostics/dev state | no | forbidden | no | Dev/test-only surfaces cannot feed public bots. |

## Required Future Scenario Corpus

| Corpus area | Required cases | Rule IDs | Evidence status |
|---|---|---|---|
| draw source | stock available/unavailable, top discard useful/useless, deeper discard with low/high added burden | `ML-TURN-001` through `ML-TURN-004` | future |
| pickup commitment | selected discard used in new meld, selected discard laid off, invalid finish/discard while commitment remains | `ML-TURN-004` | future |
| meld timing | sets, low-ace runs, high-ace runs, no-wrap rejection, table-now versus hold | `ML-MELD-*` | future |
| lay-off timing | own meld extension, opponent-origin meld extension, score-credit owner check | `ML-LAYOFF-*`, `ML-SCORE-006` | future |
| discard risk | obvious public lay-off danger, high-card shedding, preserving near melds | `ML-TURN-006`, `ML-SCORE-*` | future |
| go-out and settlement | go out without discard, go out by discard, in-hand penalties, public settlement redaction | `ML-TURN-007`, `ML-TURN-008`, `ML-SCORE-*`, `ML-VIS-006` | future |
| stock exhaustion | empty stock and no legal/accepted discard draw settles round | `ML-TURN-009` | future |
| match target | below 500, unique highest at/above 500, equal-high tie continuation | `ML-MATCH-*` | future |
| variable seats | 2, 3, 4, 5, and 6 seat tables | `ML-SETUP-001`, `ML-SETUP-004` | future |

## Candidate Extraction Plan

Candidate extraction is not accepted yet. Future work may evaluate:

| Candidate group | Extraction rule | Rule IDs | Visible facts used | Hidden-info risk | Tests |
|---|---|---|---|---|---|
| draw candidates | legal `draw/stock` and `draw/discard/<index>` leaves annotated by pickup obligation, visible discard identity, discard depth, stock count, and own immediate-use paths | `ML-TURN-*` | own hand plus public discard/stock facts | medium | stock-order and opponent-hand mutation tests |
| meld candidates | legal new meld paths annotated by value, hand-shape improvement, pickup satisfaction, and go-out potential | `ML-MELD-*`, `ML-SCORE-*` | own hand plus public phase facts | low | own-view tests |
| lay-off candidates | legal lay-off paths annotated by score credit, public meld target, pickup satisfaction, and go-out potential | `ML-LAYOFF-*`, `ML-SCORE-006` | own hand plus public meld tableau | low | public-tableau tests |
| discard candidates | legal discard leaves annotated by own penalty relief, future hand shape, and public lay-off danger | `ML-TURN-006`, `ML-SCORE-*` | own hand plus public table/discard facts | medium | hidden-opponent no-leak tests |
| go-out candidates | legal empty-hand table/discard sequence posture | `ML-TURN-007`, `ML-TURN-008`, `ML-MATCH-*` | own hand plus public scores/counts | medium | settlement and target scenarios |

## Phase Model

| Phase/situation | Detection from allowed input | Policy status | Rule IDs | Notes |
|---|---|---|---|---|
| draw choice | legal draw leaves exist | future | `ML-TURN-001` through `ML-TURN-004` | L0 random only today. |
| table play | legal finish/meld/lay-off choices exist after draw | future | `ML-TURN-005` | Must satisfy pickup commitment. |
| discard | legal discard leaves exist | future | `ML-TURN-006` | Risk model cannot inspect opponent hands. |
| go-out | hand can become empty through legal table/discard play | future | `ML-TURN-007`, `ML-TURN-008` | Settlement swing must remain viewer-safe. |
| stock exhaustion | stock count is zero and no legal/accepted discard draw can continue | future | `ML-TURN-009` | No discard reshuffle. |
| terminal/endgame | public scores after settlement | future | `ML-MATCH-*` | Equal-high ties continue. |

## Lexicographic Priority Vector

Not admitted. Future Level 2 should prefer lexicographic priorities such as:

1. Stay legal and satisfy mandatory pickup commitment.
2. Go out when settlement and match posture make it favorable.
3. Table or lay off cards that reduce high penalty exposure and score safely.
4. Choose discard pickups only when the selected card has an immediate legal
   outlet and the added burden is justified.
5. Avoid discards that obviously extend public melds for the next players.
6. Preserve strong own-hand meld potential.
7. Account for public opponent proximity from hand counts and public actions.
8. Account for public target/score/tie posture.
9. Deterministic tie-break by candidate identity and seed.

No numeric vector is accepted by this document.

## Current L0 Values

The current Level 0 values are authored in code and described in [AI.md](AI.md);
they are not static game data and are not Level 2 evidence:

| Policy area | Current value | Source |
|---|---|---|
| action source | viewer-authorized Rust legal action tree | `games/meldfall_ledger/src/bots.rs` |
| choice method | deterministic random legal among available legal paths | `games/meldfall_ledger/src/bots.rs` |
| explanation | legal-choice count only | `games/meldfall_ledger/src/bots.rs` |
| strategy | none | `games/meldfall_ledger/src/bots.rs` |

## Forbidden Hidden Information

| Information | Why forbidden | Potential leak surface | Required future no-leak test |
|---|---|---|---|
| opponent hands | unavailable to acting seat | input view, ranking, explanation | opponent-hand mutation invariance |
| hidden stock order and next stock card | unavailable as public bot input | draw source, discard risk, replay export | stock-order exclusion test |
| other-seat private settlement cards | unavailable to public/other seats | settlement evaluation, explanation | settlement redaction tests |
| actual sampled hidden state | hidden-state shortcut | belief/determinization | source scan and policy audit |
| private logs/dev state | unauthorized | dev inspector, candidate ranking | viewer redaction tests |

## Memory And Belief Model

| Item | Decision | Allowed inputs | Forbidden inputs | Tests |
|---|---|---|---|---|
| memory model | not admitted | future public history and own private observations only | actual hidden state | future |
| belief model | none | not applicable | sampled hidden states, opponent hands, stock order, future cards | future |
| redaction model | not admitted | viewer-safe facts | hidden facts | future |

## Required Evaluation Before Level 2

| Evidence row | Required content | Status |
|---|---|---|
| accepted competent-player taxonomy | reviewed strategy principles and novice traps | incomplete |
| policy id/version | stable Level 2 id and version string | incomplete |
| authorized input/memory schema | exact fields and redaction model | incomplete |
| priority vector and tie-breaks | deterministic phase-specific order | incomplete |
| scenario corpus | draw, pickup commitment, meld, lay-off, discard, go-out, stock exhaustion, target/tie, and variable-seat cases | incomplete |
| L0 comparison | fixed seeds, balanced seats, recorded manifests | incomplete |
| calibration | separated draw, table-play, discard, go-out, and score posture | incomplete |
| no-leak proof | inputs, explanations, candidates, replays, simulator summaries, browser surfaces | incomplete |
| legality/replay/hash | normal validation and deterministic replay/hash proof | incomplete |
| benchmark evidence | native decision/ranking and full playout lanes | incomplete |
| code/dependency review | absence of MCTS, ISMCTS, Monte Carlo rollouts/search, determinization, sampled worlds, machine learning, reinforcement learning, runtime LLM, and hidden-state peeking | incomplete |
| admission update | accepted implementation-admission and evidence receipt changes | incomplete |

## Public UX Note

Until Level 1 or Level 2 exists, the public UI may show the shipped Level 0
explanation only as a modest legality/simulation reason. It must not market the
bot as expert, optimal, strategic authority, or competent-human equivalent.

## Review Checklist

- Level 1 is explicitly not admitted.
- Level 2 is explicitly not admitted / intentionally deferred.
- Level 3 is explicitly not applicable.
- Current L0 is not represented as Level 1 or Level 2.
- No forbidden AI/search method is allowed.
- Future work is testable and no-leak aware.
