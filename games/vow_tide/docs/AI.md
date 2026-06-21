# Vow Tide AI

Game ID: `vow_tide`

Implemented variant: `vow_tide_standard`

Rules version: `vow-tide-rules-v1`

Last updated: 2026-06-21

## Purpose

This is the Vow Tide bot registry. It records shipped bot policies,
information access, explanation posture, and evidence. Rule authority remains
[RULES.md](RULES.md), and source posture remains [SOURCES.md](SOURCES.md).

Level 2 is not admitted for Gate 17. The evidence pack records future criteria
but does not authorize coding a Level 2 policy.

## Bot Summary

| Bot | Level | Policy/version | Supported seat range | Public default? | Information access | Status | Evidence |
|---|---:|---|---|---:|---|---|---|
| random legal | 0 | `vow-tide-random-legal-v0` | 3-7 seats | no | Rust legal action tree and deterministic bot RNG | implemented and tested | `cargo test -p vow_tide --test bots` |
| bounded baseline | 1 | `vow-tide-level1-v1` | 3-7 seats | yes, constrained | projected acting-seat view, own hand, public facts, legal action tree | implemented and tested | `cargo test -p vow_tide --test bots`; `cargo run -p simulate -- --game vow_tide --seat-count N --games 1000 --start-seed 170N00 --action-cap 2048` |
| authored policy | 2 | not admitted | 3-7 seats | no | would require completed evidence pack | blocked by evidence pack | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| shallow deterministic search | 3 | not applicable | not applicable | no | forbidden for this hidden-information game | not allowed | foundation bot law |

Public v1/v2 bots must not use MCTS, ISMCTS, Monte Carlo-style bots, ML, RL,
runtime LLM policy, hidden-state sampling, opponent-hand access, hidden stock
access, or deck-order reconstruction.

## Level 0: Random Legal Bot

`VowTideL0Bot` projects the acting seat, reads the normal Rust legal action
tree, samples one legal path with the declared deterministic seed, and returns
that path for normal validation/application by the caller. It never mutates
state directly.

| Item | Decision/evidence |
|---|---|
| legal action API used | `actions::legal_action_tree`, then `rules::validate_bid_command` or `rules::validate_play_command` through callers |
| deterministic seed behavior | same seed and same legal set produce the same sampled legal path |
| action selection method | random legal among flattened `bid/<n>` or `play/<card_id>` leaves |
| legality tests | `l0_selects_deterministic_legal_bid` |
| known limitations | intentionally not competent; baseline for legality and simulation |
| public explanation text | `Selected a seeded random legal Vow Tide action.` |
| N-seat orchestration | simulator fills all 3-7 seats deterministically; unsupported seat counts reject before setup |

## Level 1: Rule-Informed Baseline Bot

`VowTideL1Bot` is a bounded baseline, not a competent-player claim.

| Item | Decision/evidence |
|---|---|
| policy name/version | `vow-tide-level1-v1` |
| decision order summary | Bidding: estimate controls from own aces plus trump jack-or-higher cards, clamp to current hand size, choose the nearest legal bid with numeric low tie-break and dealer-hook legality. Play: compute `needed = bid - tricks_taken`; if `needed > 0`, prefer the lowest legal card that is currently winning against the public trick by the promoted trick comparator, otherwise play the highest legal card. If `needed == 0`, prefer the lowest legal losing card, otherwise the lowest legal card. |
| mandatory rule handling | Legal set comes from Rust; invalid bids/cards are absent and chosen paths still validate normally. |
| tie-break method | deterministic sort by rank value then card identity; bidding ties choose the lower numeric legal bid |
| information access | acting-seat private projection, public view facts, and legal action tree only |
| per-seat specialization | none in the bot; simulator uses a deterministic mixed L1/L0 seat fill for evidence runs |
| opponent set handling | no opponent private inference; public bids, public trick cards, trick counts, and scores only |
| explanation examples | `Estimated contract from own hand controls; chose legal bid 2.` and `Contract needs 1 more tricks; chose legal card queen_hearts.` |
| tests | `l1_hook_adjusts_to_nearest_legal_bid`, `l1_play_secures_lowest_currently_winning_card_when_contract_needs_trick`, `bot_input_contains_own_hand_only` |
| public suitability | constrained; legal, deterministic, and safe, but not claimed strategically competent |

## Exact Information Access Table

| Information | Acting seat sees? | Bot sees? | Public observer sees? | Tests/notes |
|---|---:|---:|---:|---|
| legal action tree | yes for actor | yes for actor only | no | legal path tests |
| public bids, trump indicator, current trick, trick counts, scores | yes | yes | yes | projected public view |
| own private hand | yes | yes for acting bot seat | no | `bot_input_contains_own_hand_only` |
| opponent private hands | no | no | no | bot input no-leak canary |
| hidden stock identity/order | no | no | no | bot input no-leak canary |
| future random outcome/deck order | no | no | no | source/code review |
| dev/test full state | no for public bot | no for public bot | no | test harness only |

## Decision Order Summary

| Bot | Decision order |
|---|---|
| random legal | flatten legal leaves, sample deterministic random index, return selected legal path |
| bounded baseline | bidding control estimate to nearest legal bid; playing contract-relative secure-or-shed using public trick comparator |
| authored policy | not admitted |

## Explanation Examples

| Bot | Situation | Viewer class | Example explanation | Redaction needed? | Hidden-info safe? |
|---|---|---|---|---:|---:|
| random legal | any legal set exists | public summary | `Selected a seeded random legal Vow Tide action.` | no hidden facts named | yes |
| bounded baseline | bidding | acting seat/public summary | `Estimated contract from own hand controls; chose legal bid 2.` | own-hand details are summarized, not listed for public viewers | yes |
| bounded baseline | trick play | acting seat/public summary | `Contract needs 1 more tricks; chose legal card queen_hearts.` | do not name opponent cards, hidden stock, or future cards | yes |
| authored policy | not admitted | not applicable | not applicable | yes | not applicable |

## Known Weaknesses

| Bot | Weakness | Why acceptable | Mitigation/future trigger |
|---|---|---|---|
| random legal | ignores strategy | Level 0 is required legality baseline only | none; not public default |
| bounded baseline | shallow: no probability model, no public card-memory model beyond current trick, no table-leader/kingmaking policy, no endgame risk planning | Gate 17 admits L1 only, and it is safe/deterministic. The simulator proves completion for 3-7 seats, not competence. | complete Level 2 evidence pack before claiming competent play |

## Tests And Simulations

| Evidence | Purpose | Status |
|---|---|---|
| `cargo test -p vow_tide --test bots` | legality, determinism, hook adjustment, contract-relative play, no-leak input canary | covered |
| `cargo run -p simulate -- --game vow_tide --seat-count 3 --games 1000 --start-seed 170300 --action-cap 2048` | 3-seat seeded completion and seat-keyed metrics | covered |
| `cargo run -p simulate -- --game vow_tide --seat-count 4 --games 1000 --start-seed 170400 --action-cap 2048` | 4-seat seeded completion and determinism check basis | covered |
| `cargo run -p simulate -- --game vow_tide --seat-count 5 --games 1000 --start-seed 170500 --action-cap 2048` | 5-seat seeded completion | covered |
| `cargo run -p simulate -- --game vow_tide --seat-count 6 --games 1000 --start-seed 170600 --action-cap 2048` | 6-seat seeded completion | covered |
| `cargo run -p simulate -- --game vow_tide --seat-count 7 --games 1000 --start-seed 170700 --action-cap 2048` | 7-seat seeded completion | covered |

## Simulation Metrics

The Vow Tide simulator summary records deterministic fields only:

- `wins_by_seat`;
- `co_wins_by_seat` and `co_win_games`;
- `exact_bid_rate_by_seat`;
- `total_actions` and `average_actions_per_match`;
- `total_hands` and `average_hands_per_match`;
- `hook_exclusions`;
- `action_cap_failures`;
- `completion_rate_percent`.

Wall-clock throughput is intentionally omitted from the Vow Tide summary so
identical seed, seat count, policy, and action cap produce identical stdout.

## Public Default Suitability

| Check | Status | Notes |
|---|---|---|
| bot is legal and deterministic | yes | Rust tests and simulator proof. |
| bot does not look broken | constrained | It bids from own controls and plays contract-relative legal cards. |
| bot is fair under information rules | yes | No opponent-hand, hidden-stock, deck-order, or future-random access. |
| explanations are safe and useful | yes, minimal | Explanations name only policy-level visible reasons. |
| latency fits public UX | yes for native baseline | Browser/WASM proof is later gate work. |
| known weaknesses acceptable | yes for Gate 17 | Level 2 is not admitted. |
| public default decision | constrained Level 1 | Use as a safe opponent, not as a competent-player showcase. |
