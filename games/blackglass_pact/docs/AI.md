# Blackglass Pact AI

Game ID: `blackglass_pact`

Implemented variant: `blackglass_pact_standard`

Rules version: `blackglass-pact-rules-v1`

Last updated: 2026-06-25

## Purpose

This is the Blackglass Pact bot registry. It records shipped bot policies,
information access, explanation posture, known weaknesses, and evidence. Rule
authority remains [RULES.md](RULES.md), and source posture remains
[SOURCES.md](SOURCES.md).

Level 2 is not admitted for Gate 18. The evidence pack records future criteria
but does not authorize coding a Level 2 policy.

## Bot Summary

| Bot | Level | Policy/version | Supported seats | Public default? | Information access | Status | Evidence |
|---|---:|---|---|---:|---|---|---|
| random legal | 0 | `blackglass-pact-l0-random-legal-v1` | fixed four | no | Rust legal action tree and deterministic bot RNG | implemented and tested | `cargo test -p blackglass_pact --test bots` |
| bounded baseline | 1 | `blackglass-pact-l1-bounded-v1` | fixed four | constrained | authorized seat view, own hand after deal, public team/score/bid/play facts, legal action tree | implemented and tested | `cargo test -p blackglass_pact --test bots`; `cargo run -p simulate -- --game blackglass_pact --seat-count 4 --games 1000 --start-seed 180400 --action-cap 4096` |
| authored policy | 2 | not admitted | fixed four | no | would require completed evidence pack | blocked by evidence pack | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| shallow deterministic search | 3 | not applicable | not applicable | no | forbidden for this hidden-information game | not allowed | repository bot law |

Public v1/v2 bots must not use MCTS, ISMCTS, Monte Carlo-style bots, ML, RL,
runtime LLM policy, sampled hidden worlds, determinization, hidden-card
peeking, partner/opponent hand access, future deal access, or seed/deck
reconstruction.

## Level 0: Random Legal Bot

`BlackglassL0Bot` projects the acting seat, reads the normal Rust legal action
tree, flattens two-segment legal leaves, sorts them canonically, samples one
legal path with the declared deterministic seed, and returns that path for
normal validation/application by the caller. It never mutates state directly.

| Item | Decision/evidence |
|---|---|
| legal action API used | `legal_action_tree`, then the normal game validator through callers |
| deterministic seed behavior | same seed and same legal set produce the same sampled legal path |
| action selection method | random legal among sorted `blind_nil/*`, `bid/*`, or `play/<card_id>` leaves |
| legality tests | `l0_selects_seeded_legal_action_deterministically` |
| known limitations | intentionally not competent; baseline for legality and simulation |
| public explanation text | `Selected one seeded legal action from the authorized action tree.` |

## Level 1: Rule-Informed Baseline Bot

`BlackglassL1Bot` is a bounded baseline, not a competent-player claim.

| Item | Decision/evidence |
|---|---|
| policy name/version | `blackglass-pact-l1-bounded-v1` |
| blind-nil order | If the public team deficit is at least 300, declare. If the deficit is at least 200, partner has not declared, and the opponent score is below 450, declare. Otherwise decline. |
| bidding order | If own authorized hand has no spades and no ace/king/queen, prefer legal nil. Otherwise estimate from own aces/kings plus spade length over three, clamp to 1-13, and choose the nearest legal numeric bid with lower numeric tie-break. |
| play order | Choose the lowest legal card by rank value and card index from the Rust legal leaves. |
| mandatory rule handling | Legal set comes from Rust; invalid bids/cards are absent and chosen paths still validate normally. |
| tie-break method | canonical action-path sort; numeric bidding ties prefer lower legal bid; play ties use rank then card index |
| information access | acting-seat private projection, public team/score/bid/play facts, and legal action tree only |
| partner/opponent handling | may use public partner declaration/bid/play state; never reads partner or opponent unplayed hands |
| explanation examples | `Applied public-score blind-nil thresholds to a legal blind action.`; `Estimated an own-hand bounded bid and selected a legal bid leaf.`; `Selected a legal play using own authorized hand and public trick context.` |
| tests | `l1_blind_nil_uses_public_score_thresholds_only`, `l1_bids_nil_for_low_safe_own_hand_and_numeric_for_controls`, `l1_play_selects_lowest_legal_card_without_leaking_other_hands` |
| public suitability | constrained; legal, deterministic, and safe, but not claimed strategically competent |

## Exact Information Access Table

| Information | Acting seat sees? | Bot sees? | Public observer sees? | Tests/notes |
|---|---:|---:|---:|---|
| legal action tree | yes for actor | yes for actor only | no | legal path tests |
| public scores, bags, dealer, phase, bids, plays, trick winners | yes | yes | yes | projected seat view |
| own private hand after deal | yes | yes for acting bot seat | no | bid/play tests |
| partner private hand | no | no | no | partner no-leak rule |
| opponent private hands | no | no | no | explanation no-leak canary |
| future deal/deck order before blind decision | no | no | no | blind no-card tests |
| seed or full replay state as strategy input | no | no | no | code review |
| dev/test full state | no for public bot | no for public bot | no | test harness only |

## Decision Order Summary

| Bot | Decision order |
|---|---|
| random legal | flatten legal leaves, sort, sample deterministic random index, return selected legal path |
| bounded baseline | blind public-score thresholds; own-hand nil screen or numeric estimate; lowest legal play |
| authored policy | not admitted |

## Explanation Examples

| Bot | Situation | Viewer class | Example explanation | Redaction needed? | Hidden-info safe? |
|---|---|---|---|---:|---:|
| random legal | any legal set exists | public summary | `Selected one seeded legal action from the authorized action tree.` | no hidden facts named | yes |
| bounded baseline | blind decision | public summary | `Applied public-score blind-nil thresholds to a legal blind action.` | no hand facts exist yet | yes |
| bounded baseline | bidding | acting seat/public summary | `Estimated an own-hand bounded bid and selected a legal bid leaf.` | own-card identities are not listed for public viewers | yes |
| bounded baseline | play | acting seat/public summary | `Selected a legal play using own authorized hand and public trick context.` | do not name partner/opponent unplayed cards | yes |
| authored policy | not admitted | not applicable | not applicable | yes | not applicable |

## Known Weaknesses

| Bot | Weakness | Why acceptable | Mitigation/future trigger |
|---|---|---|---|
| random legal | ignores strategy | Level 0 is required legality baseline only | none; not public default |
| bounded baseline | shallow: no trick-count goal model, no partner nil cover logic beyond public declaration state, no card-memory void inference, no endgame bag/target posture | Gate 18 admits L1 only, and it is safe/deterministic. The simulator proves the bot-smoke arm, not strategic strength. | complete Level 2 evidence pack before claiming competent play |

## Tests And Simulations

| Evidence | Purpose | Status |
|---|---|---|
| `cargo test -p blackglass_pact --test bots` | legality, determinism, blind/bid/play priorities, explanation no-leak, bot trace inventory | covered |
| `cargo run -p simulate -- --game blackglass_pact --seat-count 4 --games 1000 --start-seed 180400 --action-cap 4096` | fixed-four seeded bot-smoke summary with seat/team metrics | covered as smoke |

The current simulator arm exercises setup, observer export, and one L0/L1
decision per seat/seed. Full terminal match replay validation remains a later
tool/replay registration obligation.

## Public Default Suitability

| Check | Status | Notes |
|---|---|---|
| bot is legal and deterministic | yes | Rust bot tests and legal-tree selection. |
| bot does not look broken | constrained | Blind and bidding heuristics are plausible; play is intentionally shallow. |
| bot is fair under information rules | yes | No partner/opponent hand, future deal, or hidden-state input. |
| explanations are safe and useful | minimal | Explanations name only policy-level visible reasons. |
| latency fits public UX | native smoke only | Browser/WASM proof is later gate work. |
| known weaknesses acceptable | yes for Gate 18 | Level 2 is not admitted. |
| public default decision | constrained Level 1 | Use as a safe opponent, not as a competent-player showcase. |
