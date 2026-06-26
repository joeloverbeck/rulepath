# Meldfall Ledger AI

Game ID: `meldfall_ledger`

Implemented variant: `classic_500_single_deck_v1`

Rules version: `meldfall-ledger-rules-v1`

Last updated: 2026-06-26

## Purpose

This is the Meldfall Ledger bot registry. It records shipped bot policies,
information access, explanation posture, known weaknesses, and evidence. Rule
authority remains [RULES.md](RULES.md), and source posture remains
[SOURCES.md](SOURCES.md).

Level 1 is not admitted for Gate 19. Level 2 is not admitted for Gate 19. The
strategy evidence pack records future criteria but does not authorize coding a
Level 2 policy.

## Bot Summary

| Bot | Level | Policy/version | Supported seats | Public default? | Information access | Status | Evidence |
|---|---:|---|---|---:|---|---|---|
| random legal | 0 | `meldfall-ledger-l0-random-legal-v1` | 2-6 | no | Rust legal action tree projected for the acting seat and deterministic bot RNG | implemented and tested | `cargo test -p meldfall_ledger --test bots` |
| rule-informed baseline | 1 | `not_admitted_pending_strategy_evidence` | 2-6 | no | would require public facts, own private hand, Rust legal actions, and deterministic authored preferences | not admitted | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| authored policy | 2 | not admitted | 2-6 | no | would require completed evidence pack | blocked by evidence pack | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| shallow deterministic search | 3 | not applicable | not applicable | no | forbidden for this hidden-information game | not allowed | repository bot law |

Public v1/v2 bots must not use MCTS, ISMCTS, Monte Carlo rollouts or search,
Monte Carlo-style bots, machine learning, reinforcement learning, runtime LLM
move selection, sampled hidden worlds, determinization, hidden-card peeking,
opponent hand access, stock-order access, future-card access, seed/deck
reconstruction, or hidden-state-derived candidate rankings.

## Level 0: Random Legal Bot

`MeldfallL0Bot` projects the acting seat, reads the Rust legal action tree
authorized for that seat, flattens legal leaves, samples one legal path with the
declared deterministic seed, and returns that path for normal validation and
application by the caller. It never mutates state directly.

| Item | Decision/evidence |
|---|---|
| legal action API used | `legal_action_tree_for_seat`, `project_action_tree_for_viewer`, then normal game validation through callers |
| deterministic seed behavior | same seed and same legal set produce the same sampled legal path |
| action selection method | random legal among viewer-authorized legal leaves |
| legality tests | `l0_selects_deterministic_legal_actions_from_authorized_tree`, `selected_l0_actions_apply_through_rules_api_for_current_phases` |
| hidden-info tests | `l0_input_contains_own_hand_only_and_no_stock_order` |
| known limitations | intentionally not competent; baseline for legality and bounded simulation only |
| public explanation text | `Selected one seeded legal action from <N> viewer-authorized choices.` |

## Level 1: Rule-Informed Baseline Bot

No Level 1 Meldfall Ledger bot is admitted in Gate 19.

| Item | Decision/evidence |
|---|---|
| policy name/version | `not_admitted_pending_strategy_evidence` |
| reason not admitted | rummy competence needs a documented hand-shaping, meld-timing, discard-risk, pickup-risk, scoring, and public-inference model before authored preferences are safe to ship |
| allowed future inputs | public view, acting seat private hand, legal action tree, rules/variant metadata, and public history visible to that bot |
| forbidden inputs | opponent hands, hidden stock order, next stock card, internal shuffle tail, private diagnostics, full-state shortcuts, sampled hidden worlds, and hidden-state-derived candidate rankings |
| explanation status | not implemented |
| tests | `bot_trace_inventory_records_l1_not_admitted` |
| public suitability | no public Level 1 default exists |

## Exact Information Access Table

| Information | Acting seat sees? | Bot sees? | Public observer sees? | Tests/notes |
|---|---:|---:|---:|---|
| legal action tree | yes for actor | yes for actor only | no | legal path tests |
| public discard pile, public meld groups, stock count, scores, active seat, dealer, turn phase | yes | yes | yes | projected seat view |
| own private hand | yes | yes for acting bot seat | no | bot input no-leak test |
| opponent private hands | no | no | no | bot input no-leak test; `ML-VIS-003` |
| hidden stock order and next stock card | no | no | no | bot input no-leak test; `ML-VIS-003` |
| public tabled-card score-credit owners | yes | yes | yes | public tableau and scoring rules |
| private settlement cards for other seats | no | no | no | viewer export no-leak tests |
| seed or full replay state as strategy input | no | no | no | code review |
| dev/test full state | no for public bot | no for public bot | no | test harness only |

## Decision Order Summary

| Bot | Decision order |
|---|---|
| random legal | project authorized legal tree, flatten legal leaves, select one legal path with deterministic seeded random choice, return selected path |
| rule-informed baseline | not admitted |
| authored policy | not admitted |

## Explanation Examples

| Bot | Situation | Viewer class | Example explanation | Redaction needed? | Hidden-info safe? |
|---|---|---|---|---:|---:|
| random legal | any legal set exists | public summary | `Selected one seeded legal action from 3 viewer-authorized choices.` | no hidden facts named | yes |
| rule-informed baseline | not admitted | not applicable | not applicable | yes | not applicable |
| authored policy | not admitted | not applicable | not applicable | yes | not applicable |

## Known Weaknesses

| Bot | Weakness | Why acceptable | Mitigation/future trigger |
|---|---|---|---|
| random legal | ignores strategy, may draw/discard/table inefficiently, and does not pursue terminal scoring | Level 0 is required as a legality baseline, not a public competent opponent | none; not public default |
| rule-informed baseline | no admitted policy | Gate 19 requires the safety boundary before claiming strategic behavior | complete and accept the strategy evidence pack before implementing Level 1 or Level 2 preferences |

## Tests And Simulations

| Evidence | Purpose | Status |
|---|---|---|
| `cargo test -p meldfall_ledger --test bots` | L0 legality, determinism, authorized input scope, and L1-not-admitted trace inventory | covered |
| `cargo run -p simulate -- --game meldfall_ledger --seat-count 2 --games 1000 --start-seed 1900 --action-cap 4096` | seeded L0 bounded playout smoke for 2 seats | covered as bounded smoke |
| `cargo run -p simulate -- --game meldfall_ledger --seat-count 4 --games 1000 --start-seed 1901 --action-cap 4096` | seeded L0 bounded playout smoke for 4 seats | covered as bounded smoke |
| `cargo run -p simulate -- --game meldfall_ledger --seat-count 6 --games 1000 --start-seed 1902 --action-cap 8192` | seeded L0 bounded playout smoke for 6 seats | covered as bounded smoke |

The current simulator arm proves legal bounded L0 action selection and
seat-count coverage. It does not prove terminal rummy competence; the recorded
Gate 19 smoke runs ended as bounded nonterminal playouts at the action cap.

## Public Default Suitability

| Check | Status | Notes |
|---|---|---|
| bot is legal and deterministic | yes | Rust bot tests and legal-tree selection. |
| bot does not look broken | no claim | L0 is random legal and can make visibly weak rummy choices. |
| bot is fair under information rules | yes | No opponent hand, stock order, future card, or hidden-state input. |
| explanations are safe and useful | minimal | L0 explanation names only the legal-choice count. |
| latency fits public UX | native smoke only | Browser/WASM proof is later gate work. |
| known weaknesses acceptable | yes for Gate 19 L0 | Level 1 and Level 2 are not admitted. |
| public default decision | no | Use L0 for tests and simulation, not as a competent public opponent. |
