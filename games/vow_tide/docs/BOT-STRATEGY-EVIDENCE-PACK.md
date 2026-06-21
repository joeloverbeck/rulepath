# Vow Tide Bot Strategy Evidence Pack

Game ID: `vow_tide`

Implemented variant: `vow_tide_standard`

Rules version: `vow-tide-rules-v1`

Bot target: Level 2 authored policy

Policy name/version: not admitted

Date: 2026-06-21

## Purpose And Gate

This is the formal status record for a future Level 2 Vow Tide bot. Gate 17
does not admit Level 2. A Level 2 bot must not be coded until this pack is
completed, reviewed, and accepted in a later bounded task.

The shipped bots are Level 0 random legal and Level 1 bounded baseline only.
[AI.md](AI.md) is the current bot registry.

Decision: `L2 not admitted`

Level 3 status: `L3 not applicable`

Vow Tide is an imperfect-information game because unplayed hands and hidden
stock identity/order are not visible to every seat. Public v1/v2 shallow search
over hidden worlds is not allowed.

## Explicit Public V1/V2 Exclusions

No public Vow Tide bot may use omniscient state, hidden-state shortcuts, future
random outcomes, unbounded weight soup, static-data tactical conditions, random
blunder injection, MCTS, ISMCTS, Monte Carlo-style bots, ML, RL, runtime LLM
policy, opponent-hand enumeration, hidden-stock peeking, or seed/deck
reconstruction.

## Source Documents Consumed

| Document/source | Path/reference | Required? | Status | Notes |
|---|---|---:|---|---|
| rules | [RULES.md](RULES.md) | yes | read | Stable `VT-*` rule IDs. |
| sources | [SOURCES.md](SOURCES.md) | yes | read | Source/IP and strategy-prior-art posture. |
| competent-player analysis | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | yes | incomplete for L2 | Strategy candidates exist but are not accepted policy. |
| bot registry | [AI.md](AI.md) | yes | read | L0/L1 only. |
| implementation | `games/vow_tide/src/bots.rs` | yes | read | Current L1 is bounded baseline, not L2. |
| tests | `games/vow_tide/tests/bots.rs` | yes | read | L1 behavior and no-leak input canary. |

## Evidence Pack Status

Decision: `L2 not admitted`

| Area | Status | Required future work |
|---|---|---|
| exact bot input view | partial | Freeze any expanded allowed input struct/view and no-leak tests for every field. |
| candidate extraction | not implemented | Map legal actions to candidate features from own/public facts only. |
| phase model | partial | Separate bidding, lead, follow, void, late-hand, and endgame nodes. |
| lexicographic priority vector | not implemented | Define and test ordered priorities; avoid weight soup. |
| bounded scoring tie-breakers | not implemented | Name small ranges and explanation fragments. |
| deterministic tie-break | partial | Current L1 has deterministic ordering; future L2 needs candidate identity rules. |
| style profiles | not planned | One strong default first. |
| explanation contract | partial | Must cite visible facts and redact by viewer. |
| simulation evidence | incomplete for L2 | Existing simulator proves L1 completion only. |
| benchmark evidence | not complete for L2 | Future Level 2 requires own decision/ranking lanes. |

## Exact Bot Input View For Future L2

| Input item | Included? | Source | Visible to acting seat? | Notes/no-leak test |
|---|---:|---|---:|---|
| legal action paths | yes | Rust legal action API | yes | Must be the candidate base. |
| public view | yes | Rust projection | yes | Bids, trick counts, current trick, trump, scores, hand counts. |
| acting seat private hand | yes | Rust seat-private projection | yes | Own cards only. |
| visible command/effect history | possible | viewer-filtered logs | yes | Must not include hidden stock or other hands. |
| policy seed/tie-break state | yes | bot framework | not game info | Deterministic only. |
| hidden opponent hands | no | forbidden | no | Mutation-invariance/no-leak tests. |
| hidden stock identity/order | no | forbidden | no | Bot input and export canaries. |
| future deals/random outcomes | no | forbidden | no | No seed/deck reconstruction. |
| actual sampled hidden state | no | forbidden | no | No Monte Carlo/determinization. |

## Candidate Extraction Plan

Candidate extraction is not accepted yet. Future work may evaluate:

| Candidate group | Extraction rule | Rule IDs | Visible facts used | Hidden-info risk | Tests |
|---|---|---|---|---|---|
| bid candidates | legal `bid/<n>` leaves annotated by own-hand controls, public bid prefix, hand size, and hook legality | `VT-BID-RANGE-001`, `VT-HOOK-001` | own hand, public bids, public hand size | low | own-view and hook tests |
| lead candidates | legal `play/<card>` leaves annotated by whether the card is currently winning an empty/public trick and whether the contract needs tricks | `VT-FIRST-LEAD-001`, `VT-SCORE-001` | own card, own bid/tricks, public trump | low | play examples |
| follow candidates | legal follow-suit cards annotated by current-winning status | `VT-FOLLOW-001`, `VT-TRICK-WIN-001` | own card, public current trick | low | comparator tests |
| void candidates | legal off-suit cards annotated by trump/win possibility and contract posture | `VT-FOLLOW-001`, `VT-SCORE-001` | own card, public current trick | medium | no hidden-state tests |
| endgame candidates | legal actions annotated by public standings and remaining hand count | `VT-STANDINGS-001` | public scores/schedule only | medium | kingmaking examples |

## Phase Model

| Phase/situation | Detection from allowed input | Policy status | Rule IDs | Notes |
|---|---|---|---|---|
| bidding | `phase = bidding` and legal bid leaves exist | L1 shallow only | `VT-BID-*`, `VT-HOOK-001` | L2 not admitted. |
| lead | `phase = playing_trick`, current trick empty | future | `VT-FIRST-LEAD-001` | Lead-control value is not modeled by L1. |
| follow suit | current trick non-empty and legal set follows led suit | future | `VT-FOLLOW-001` | Current L1 only checks current-winning status. |
| void play | current trick non-empty and legal set includes off-suit cards | future | `VT-FOLLOW-001` | Trump/shed choices need no-leak tests. |
| late hand | own bid/tricks nearly exact | future | `VT-SCORE-001` | L1 uses `needed`, but not future risk. |
| terminal/endgame | public scores and remaining hands | future | `VT-STANDINGS-001` | No table-leader policy accepted. |

## Lexicographic Priority Vector

Not admitted. Future Level 2 should prefer lexicographic priorities such as:

1. Stay legal and satisfy mandatory rules.
2. Preserve exact-contract scoring opportunities.
3. Secure needed tricks with the cheapest current winner.
4. Avoid overtricks after contract is met.
5. Use public standings and remaining hands without kingmaking shortcuts.
6. Reduce own-hand risk using own private hand only.
7. Deterministic tie-break by candidate identity and seed.

No numeric vector is accepted by this document.

## Bounded L1 Weight Set

The current Level 1 weights are authored in code and described in [AI.md](AI.md);
they are not static game data and are not Level 2 evidence:

| Policy area | Current L1 value | Source |
|---|---|---|
| own-hand control estimate | count own aces plus trump jack-or-higher cards, clamp to hand size | `games/vow_tide/src/bots.rs` |
| bid choice | nearest legal bid to estimate; lower numeric tie-break | `games/vow_tide/src/bots.rs` |
| play while needing tricks | lowest legal card currently winning by public trick comparator; otherwise highest legal card | `games/vow_tide/src/bots.rs` |
| play after contract met | lowest legal losing card; otherwise lowest legal card | `games/vow_tide/src/bots.rs` |

## Forbidden Hidden Information

| Information | Why forbidden | Potential leak surface | Required future no-leak test |
|---|---|---|---|
| opponent hand | unavailable to acting seat | input view, ranking, explanation | opponent-hand mutation invariance |
| hidden stock identity/order | unavailable after trump indicator | bid features, explanation, replay export | stock canary scan |
| future deal/random outcome | hidden setup fact | policy features, replay export | seed/deck exclusion test |
| sampled hidden state | actual hidden shortcut | belief/determinization | source scan and policy audit |
| private logs/dev state | unauthorized | dev inspector, candidate ranking | viewer redaction tests |

## Memory And Belief Model

| Item | Decision | Allowed inputs | Forbidden inputs | Tests |
|---|---|---|---|---|
| memory model | not admitted | future public history and own private observations only | actual hidden state | future |
| belief model | none | not applicable | sampled hidden states, opponent hands, stock identity | future |
| redaction model | not admitted | viewer-safe facts | hidden facts | future |

## Explanation Contract

Future Level 2 explanations must name:

- policy name/version;
- chosen priority reason;
- visible fact used;
- tie-break note when relevant;
- redaction when a fact is seat-private;
- known weakness only as product-safe copy.

They must not name opponent cards, hidden stock identities, sampled
possibilities, future deck facts, actual hidden-state facts, or strategy advice
derived from hidden information.

## Decision Examples

Decision examples are intentionally not accepted as Level 2 policy. Candidate
examples in [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) must be converted into
tests before they can drive code.

## Test Plan Required Before Level 2

| Test area | Required? | Specific tests | Evidence |
|---|---:|---|---|
| legality | yes | chosen action validates through normal command path | future |
| determinism | yes | fixed seed/view/rules/policy produce fixed decision | future |
| no hidden-state access | yes | mutate opponent hands/stock and assert decision/explanation stability where hidden | future |
| candidate extraction | yes | candidate annotations match legal actions and visible facts | future |
| priority vector | yes | examples hit expected priority slots | future |
| explanations | yes | viewer-safe public and seat-private explanations | future |
| simulation/fuzz | yes | many seeded games with failure reports | future |
| replay/hash | yes | decisions reproduce in replay | future |
| benchmark | yes | `level2_action_selection` and full playout lanes | future |

## Public UX Note

Until Level 2 exists, the public UI may show the shipped Level 1 explanation as
a modest safe reason. It must not market the bot as expert, optimal, strategic
authority, or competent-human equivalent.

## Review Checklist

- Level 2 is explicitly not admitted.
- Level 3 is explicitly not applicable.
- Current L1 is not represented as Level 2.
- No forbidden AI/search method is allowed.
- Future work is testable and no-leak aware.
