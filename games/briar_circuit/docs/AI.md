# Briar Circuit AI

Game ID: `briar_circuit`

Implemented variant: `briar_circuit_standard`

Rules version: `briar-circuit-rules-v1`

Last updated: 2026-06-21

## Purpose

This is the per-game bot registry for Briar Circuit. It records shipped Rust bot
policies, their information boundary, explanation posture, and evidence. Rules
authority remains [RULES.md](RULES.md).

Level 2 is not admitted for Gate 16. The evidence pack records future criteria
but does not authorize coding a Level 2 policy.

## Bot Summary

| Bot | Level | Policy/version | Supported seat range | Public default? | Information access | Status | Evidence |
|---|---:|---|---|---:|---|---|---|
| random legal | 0 | `briar-circuit-l0-random-legal-v1` | exactly 4 seats | no | Rust legal action set and deterministic bot RNG | implemented and tested | `cargo test -p briar_circuit --test bots`, [BENCHMARKS.md](BENCHMARKS.md) |
| bounded baseline | 1 | `briar-circuit-l1-baseline-v1` | exactly 4 seats | yes, constrained | Rust legal action set, own hand, public state/history available through game API | implemented and tested | `l1_bot_uses_same_legal_action_api_for_pass_and_play`, no-leak tests |
| authored policy | 2 | not admitted | exactly 4 seats | no | would require completed evidence pack | blocked by evidence pack | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| shallow deterministic search | 3 | not allowed | not applicable | no | forbidden for this hidden-information game | not allowed | foundation bot law |

Public v1/v2 bots must not use MCTS, ISMCTS, Monte Carlo-style bots, ML, RL,
runtime LLM policy, hidden-state sampling, opponent-hand access, pass-provenance
peeking, or deck-order reconstruction.

## Level 0: Random Legal Bot

`BriarCircuitL0Bot` calls `legal_bot_actions`, samples one legal action with
`SeededRng`, and submits through the normal validation/apply path. It never
mutates state directly.

| Item | Decision/evidence |
|---|---|
| legal action API used | `legal_bot_actions`, then `validate_pass_command` or `validate_play_command` through callers |
| deterministic seed behavior | same seed and same legal set produce the same sampled index |
| action selection method | random legal among legal pass/play actions |
| legality tests | `l0_bot_selects_only_legal_pass_actions_and_is_seed_deterministic` |
| known limitations | intentionally not competent; baseline for simulations and legality |
| public explanation text | `Random legal choice from N actions.` |
| benchmark evidence | `l0_action_selection` in [BENCHMARKS.md](BENCHMARKS.md) |
| N-seat orchestration | callers advance legal seats deterministically; unsupported seat counts reject at setup |

## Level 1: Rule-Informed Baseline Bot

`BriarCircuitL1Bot` is a bounded baseline, not a competent-player claim.

| Item | Decision/evidence |
|---|---|
| policy name/version | `briar-circuit-l1-baseline-v1` |
| decision order summary | During pass, prefer high point-pressure cards; confirm after three selected cards. During play, lead or follow the led suit with the lowest-penalty, lowest-ranked legal card (duck); when void in the led suit, shed the highest-penalty legal card first (queen of spades, then high hearts, then high cards), since an off-suit card cannot win the trick. Own-hand information only. |
| mandatory rule handling | Legal set comes from Rust; invalid options are absent. |
| tie-break method | deterministic sort/order over legal candidates; no hidden random search |
| information access | own hand and public state/history via Rust state/view surfaces; no opponent hand/deck order |
| per-seat specialization | none |
| opponent set handling | table competitors only through public scores/history; no private inference model |
| explanation example | `Selected a bounded legal action from public state and own hand.` |
| tests | `l1_bot_uses_same_legal_action_api_for_pass_and_play`, `l1_bot_decision_is_invariant_to_opponent_hidden_hand_changes` |
| benchmarks | `l1_action_selection`, `full_seeded_hand`, `full_seeded_match_terminal` |
| public suitability | constrained; legal, deterministic, and safe, but not claimed strategically competent |

## Level 2: Authored Policy Bot

Required evidence pack: [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md)

Competent-player analysis: [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md)

| Item | Summary only |
|---|---|
| policy name/version | not admitted |
| evidence pack status | incomplete / blocked by deliberate Gate 16 scope |
| phase model | documented for future work only |
| candidate extraction | not implemented |
| lexicographic priority vector | not implemented |
| bounded scoring tie-breakers | not implemented |
| deterministic seeded tie-break | not implemented |
| style profiles | none |
| explanation contract | future Level 2 must be viewer-safe and cite visible facts only |
| known weaknesses | current L1 is shallow and not a competent-human proxy |
| public default suitability | no |

Do not code Level 2 before the evidence pack is completed and reviewed in a
separate bounded task.

## Exact Information Access Table

| Information | Acting seat sees? | Opponent seats see? | Bot sees? | Public observer sees? | Tests/notes |
|---|---:|---:|---:|---:|---|
| legal action tree | yes for actor | no for non-actors | yes for actor only | no | WASM/action tests |
| public trick/history/scores | yes | yes | yes | yes | projection tests |
| own private hand | yes | no | yes for acting bot seat | no | pairwise visibility tests |
| own pass selection | yes | no | yes for selecting seat | no | pass visibility tests |
| opponent private hand | no | no | no | no | `l1_bot_decision_is_invariant_to_opponent_hidden_hand_changes` |
| pass provenance | owner/receiver only as authorized | no | no unauthorized provenance | no | e2e export scan |
| unrevealed deck/order/future deal | no | no | no | no | replay/export tests |
| dev/test full state | no for public bot | no | no for public bot | no | test harness only |

## Decision Order Summary

| Bot | Decision order |
|---|---|
| random legal | enumerate legal actions, sample deterministic random index, return selected legal action |
| bounded baseline | pass: select high point-pressure cards, then confirm; play: duck with the lowest-penalty card when leading or following the led suit, and shed the highest-penalty card when void in the led suit |
| authored policy | not admitted |

## Explanation Examples

| Bot | Situation | Viewer class | Example explanation | Redaction needed? | Hidden-info safe? | Test |
|---|---|---|---|---:|---:|---|
| random legal | pass or play legal set exists | acting seat/public summary | `Random legal choice from 7 actions.` | no hidden card list beyond authorized action | yes | bot tests |
| bounded baseline | pass/play legal set exists | acting seat/public summary | `Selected a bounded legal action from public state and own hand.` | do not name opponent cards or deck facts | yes | bot no-leak tests |
| authored policy | not admitted | not applicable | not applicable | yes | not applicable | evidence pack |

## Known Weaknesses

| Bot | Weakness | Why acceptable | Mitigation/future trigger |
|---|---|---|---|
| random legal | ignores strategy | Level 0 is required legality baseline only | none; not public default |
| bounded baseline | shallow: no suit-memory model, no endgame table-leader policy, no opponent-aware moon prevention/planning | Gate 16 admits L1 only, and it is safe/deterministic. Its own-hand discard order (shed highest penalty when void) lowers the accidental shoot-the-moon rate in 1000-match self-play from ~9% to ~3% of hands, but it still reads no opponent state and does not actively defend against a moon. | complete Level 2 evidence pack before claiming competent play |

## Tests

| Test | Purpose | Bot(s) | Status | Notes |
|---|---|---|---|---|
| `cargo test -p briar_circuit --test bots` | legality, determinism, no hidden explanation names | all shipped bots | covered | Existing Rust bot tests. |
| `l1_bot_decision_is_invariant_to_opponent_hidden_hand_changes` | guard against opponent-hand peeking | L1 | covered | Mutating hidden opponent hands does not alter decision. |
| `cargo run -p simulate -- --game briar_circuit --games 1000` | end-to-end legal playouts | shipped bots/tool policies | covered in series evidence | Capstone re-runs final lane. |
| `node apps/web/e2e/briar-circuit.smoke.mjs` | browser no-leak around bot/replay surfaces | public bot surfaces | covered | No hidden terms in DOM/storage/logs/export. |

## Benchmarks

| Benchmark | Target | Current baseline | Bot(s) | Status | Notes |
|---|---:|---:|---|---|---|
| `l0_action_selection` | smoke floor | recorded in [BENCHMARKS.md](BENCHMARKS.md) | L0 | covered | Native bench-report lane. |
| `l1_action_selection` | smoke floor | recorded in [BENCHMARKS.md](BENCHMARKS.md) | L1 | covered | Native bench-report lane. |
| `full_seeded_hand` | smoke floor | recorded in [BENCHMARKS.md](BENCHMARKS.md) | L1-driven legal choices | covered | Complete hand benchmark. |
| `full_seeded_match_terminal` | 100 matches/sec provisional | recorded in [BENCHMARKS.md](BENCHMARKS.md) | L1-driven legal choices | covered | Near-threshold terminal fixture. |

## Public Default Suitability

| Check | Status | Notes |
|---|---|---|
| bot is legal and deterministic | yes | Rust tests and benchmarks. |
| bot does not look broken | constrained | It follows rules and basic penalty avoidance; it is shallow. |
| bot is fair under information rules | yes | No opponent-hand/deck access. |
| explanations are safe and useful | yes, minimal | Explanations are intentionally conservative. |
| latency fits public UX | yes | Native benchmark lane covers decision latency. |
| known weaknesses acceptable | yes for Gate 16 | Level 2 is not admitted. |
| public default decision | constrained Level 1 | Use as safe opponent, not as competent-player showcase. |
