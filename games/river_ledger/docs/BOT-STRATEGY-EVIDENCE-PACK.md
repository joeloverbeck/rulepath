# River Ledger Bot Strategy Evidence Pack

Game ID: `river_ledger`

Implemented variant: `river_ledger_standard`

Rules version: `river-ledger-rules-v2`

Bot target: Level 2 authored policy

Policy name/version: `river-ledger-level2-v1` / v1

Date: 2026-06-20

## Status

Level 2 is implemented and updated for Gate 15.1 stack/all-in pressure. This
pack consumes [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) and defines the
legality, determinism, no-hidden-state, and beatability contract the bot must
preserve.

## Explicit Public v1/v2 Exclusions

The policy does not use and must not grow:

- omniscient state;
- opponent hole cards;
- future community cards, burn cards, deck tail, or deck order;
- hidden-state sampling, determinization, equity rollouts, or opponent hand
  enumeration;
- MCTS, ISMCTS, Monte Carlo, ML, RL, external solvers, or runtime LLM policy;
- TypeScript legality or TypeScript bot policy;
- private diagnostics, raw internal traces, or unrevealed showdown records.

## Source Documents and Evidence

| Document/source | Path/reference | Status | Notes |
|---|---|---|---|
| Rules | [RULES.md](RULES.md) | read | Legal actions, betting rounds, showdown, allocation. |
| Competent player analysis | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | complete | Strategy input for priority order. |
| Visibility projection | `../src/visibility.rs` | implemented | Authorized public and seat-private fields. |
| Legal actions | `../src/actions.rs` | implemented | Candidate source and action metadata. |
| Bot implementation | `../src/bots.rs` | pending | GAT15RIVLEDTEX-013. |
| Bot tests | `../tests/bots.rs` | pending | GAT15RIVLEDTEX-013. |

## Exact Bot Input View

| Input item | Included? | Authorized source | Visible to acting seat? | Evidence target |
|---|---:|---|---:|---|
| legal action tree/action paths | yes | Rust `legal_action_tree` | yes | bot legality tests |
| action metadata call price/adds-to-pot/raises remaining | yes | Rust action metadata | yes | input whitelist test |
| phase/street and active seat | yes | `PublicView.phase`, `active_seat` | yes | input summary test |
| button, blinds, seat status | yes | `PublicView.button`, `small_blind`, `big_blind`, `seats` | yes | position/context tests |
| public pot and contribution ledger | yes | `PublicView.pot_total`, `SeatView` contributions | yes | price posture tests |
| starting/remaining stack and all-in status | yes | `PublicView.seats`, action metadata | yes | stack-aware bot tests |
| public pot tiers, eligibility, and returns | yes | `PublicView` pot fields | yes | no-leak and explanation tests |
| revealed board cards | yes | `PublicView.board` | yes | board texture tests |
| own hole cards | yes | `PrivateView::Seat.hole_cards` | yes | no-leak whitelist test |
| deterministic seed | yes | policy tie-break only | not game information | determinism test |
| opponent hole cards | no | forbidden | no | whitelist/no-leak test |
| future community/deck-tail/burn/order | no | forbidden | no | whitelist/no-leak test |
| raw internal trace/private diagnostics | no | forbidden | no | no-leak test |

## Candidate Extraction

Candidates are exactly legal leaf paths from the Rust legal action tree for the
active bot seat. The policy annotates only those legal candidates and returns an
`ActionPath` that validates through `validate_command`. If the legal action tree
is empty, the bot reports no decision rather than inventing an action.

## Lexicographic Priority Vector

Earlier priority slots dominate later slots:

| Slot | Priority | Higher/better value | Explanation fragment |
|---:|---|---|---|
| 1 | terminal safety and legality | choose only available legal actions; avoid voluntary fold when free continuation exists | `legal River Ledger action` |
| 2 | fold/call/check obligation | check free weak hands; call affordable prices including call all-in when justified; fold poor big-unit prices | `public contribution price` |
| 3 | own-hole class | pairs, high connected cards, and suited high cards rank above weak disconnected lows | `own authorized hole class` |
| 4 | revealed board texture | made hand fit and strong draws rank above air | `revealed board texture` |
| 5 | live-opponent count | tighten with more live opponents; pressure more with fewer live opponents | `live opponent count` |
| 6 | street unit and price pressure | big-unit turn/river calls require stronger reasons than small-unit preflop/flop calls | `street contribution unit` |
| 7 | raise-cap and all-in pressure | prefer raises only when cap remains and strength/price justify it; distinguish short raise all-in from full raise all-in | `bounded raise cap` / `stack pressure` |
| 8 | side-pot eligibility awareness | preserve public eligibility when short-stack pressure makes aggregate pot totals misleading | `public pot eligibility` |
| 9 | deterministic tie-break | stable action id, seat id, and seed tie-break order | not surfaced unless needed |

No weighted hidden equity score, static-data tactical condition, search tree, or
sampled belief model is used.

## Opponent-Count Adjustments

| Live opponents | Adjustment | Reason |
|---:|---|---|
| 1 | Loosen value bets and raises with made or high-potential own classes. | Fewer hands can beat the bot at showdown. |
| 2-3 | Use neutral fixed-limit posture. | Balance contribution price against multiway risk. |
| 4-5 | Tighten calls and raises unless own hand/board fit is strong. | More live hands increase showdown collision risk. |

The adjustment uses only `SeatView.status` counts and never identifies or
estimates opponent private cards.

## Explanation Contract

The bot may explain:

- policy id;
- action family;
- public price posture;
- public stack/all-in posture;
- own authorized hole class bucket;
- revealed board texture bucket;
- live-opponent count bucket;
- street/cap pressure bucket.
- public side-pot eligibility facts when relevant.

The bot must not explain or imply opponent hole cards, future board cards,
deck-tail facts, hidden-state sampling, equity rollout percentages, solver
recommendations, or private diagnostics.

## Evidence Fixtures for GAT15RIVLEDTEX-013

| Evidence | Test / trace target | Expected behavior |
|---|---|---|
| legal decisions | `random_l1_and_l2_decisions_are_legal_and_do_not_mutate_state` | Choices are legal command paths; selection does not mutate state. |
| deterministic decisions | `seeded_bots_are_deterministic_on_same_allowed_state` | Same allowed input and seed produce identical choices. |
| input whitelist | `level2_input_whitelist_excludes_forbidden_hidden_material` | Input summary excludes other hole cards, future board, deck tail, raw trace, and hidden diagnostics. |
| priority examples | `level2_policy_uses_authored_priority_and_stable_tie_break` | Strong own class pressures; poor price folds; free weak hand checks. |
| explanation no-leak | `bot_explanations_and_effects_do_not_leak_hidden_cards_or_sampling_claims` | Public/private bot effects omit hidden card ids and search/sampling claims. |
| repeated playout legality | `level2_bots_finish_many_games_with_legal_actions_under_cap` | Level 2 bots finish repeated games under action cap. |
| all-in explanations | `bot_explanations_distinguish_call_all_in_and_short_raise_all_in` | Public explanations distinguish ordinary, call all-in, short raise all-in, and full raise all-in action classes. |

## Known Weaknesses

| Weakness | Why acceptable for public Level 2 | Future trigger |
|---|---|---|
| No opponent belief model | Avoids hidden-state sampling and keeps the policy explainable. | Add only with an accepted ADR and no-leak proof. |
| No long-horizon betting trap analysis | Fixed-limit cap and contribution bounds keep the policy competent but beatable. | Add documented heuristics if simulations show repeated obvious mistakes. |
| Coarse hand buckets | Simpler to explain and safer for public v2. | Refine only from authorized own/board facts and tests. |

## Verification Commands

- `cargo test -p river_ledger --test bots`.
- `cargo test -p river_ledger`.
- `node scripts/check-doc-links.mjs`.

Tool-level simulation evidence is covered by `cargo run -p simulate -- --game
river_ledger --games 1000`.
