# masked_claims AI

Game ID: `masked_claims`

Implemented variant: `masked_claims_standard`

Rules version: `masked-claims-rules-v1`

Last updated: 2026-06-11

Prepared by: `Codex`

## Purpose

This document is the per-game bot registry and status document for Masked
Claims. It records which bots exist, which information they may access, how
they explain decisions, and how they are tested.

Detailed strategy evidence lives in `BOT-STRATEGY-EVIDENCE-PACK.md` and
`COMPETENT-PLAYER.md`.

## Bot summary

| Bot | Level | Policy/version | Public default? | Information access | Status | Evidence |
|---|---:|---|---:|---|---|---|
| random legal | 0 | `masked-claims-random-legal-v1` | no | current Rust legal action tree only | implemented and tested | `src/bots.rs`; `tests/bots.rs` |
| rule-informed baseline | 1 | `masked-claims-level1-v1` | yes, constrained | allowed seat view plus public state | implemented and tested | `tests/bots.rs`; `bot-claim-and-response.trace.json`; benchmarks |
| authored policy | 2 | none | no | not implemented | intentionally deferred | evidence pack documents why Level 1 is the gate bot |
| shallow deterministic search | 3 | none | no | not allowed for this hidden-information game | not allowed | hidden-state search is forbidden |

Public v1/v2 bots must not use MCTS, ISMCTS, Monte Carlo-style bots, ML, RL,
hidden-state sampling, or pedestal peeking.

## Level 0: random legal bot

| Item | Decision/evidence |
|---|---|
| legal action API used | `legal_action_tree` and validated `CommandEnvelope` |
| deterministic seed behavior | `Seed` controls tie-breaking among legal leaves |
| action selection method | random legal among Rust legal action paths |
| simulation tests | `cargo run -p simulate -- --game masked_claims --games 1000` |
| legality tests | `tests/bots.rs`; `src/bots.rs` |
| replay/hash tests | `bot-claim-and-response.trace.json`; `tests/replay.rs` |
| known limitations | random and not competent |
| public explanation text | concise random-legal rationale only |
| benchmark evidence | legal action and bot decision smoke floors in `BENCHMARKS.md` |

## Level 1: rule-informed baseline bot

| Item | Decision/evidence |
|---|---|
| policy name/version | `masked-claims-level1-v1` |
| decision order summary | In claim phase, prefer safe high-value claims from own hand, with bounded deterministic bluff/underclaim choices. In response phase, challenge certain lies from public counting, otherwise use a deterministic challenge threshold. |
| immediate tactics | choose legal claims, accept plausible high claims, challenge impossible or sufficiently suspicious claims |
| mandatory rule handling | always acts through the legal action tree and validates through Rust |
| tie-break method | deterministic seeded tie-break |
| information access | own hand, public scores/counters/galleries, declared grade, legal tree |
| explanation examples | "Claimed Master from a held Master using the highest safe legal claim."; "Accepted: a Master claim remains plausible and is worth 5." |
| tests | `tests/bots.rs`; `src/bots.rs`; `certain-lie-challenge.trace.json` |
| benchmarks | `level1_bot_claim_decision`; `level1_bot_response_decision` |
| public suitability | suitable for Gate 11 proof, not claimed strategically optimal |

## Level 2: authored policy bot

Required evidence pack: `BOT-STRATEGY-EVIDENCE-PACK.md`

Competent-player analysis: `COMPETENT-PLAYER.md`

| Item | Summary only |
|---|---|
| policy name/version | none |
| evidence pack status | complete enough to explain deferral, not a Level 2 implementation authorization |
| phase model | claim phase and response window |
| candidate extraction | documented for future policy work |
| lexicographic priority vector | not implemented |
| deterministic seeded tie-break | Level 1 only |
| style profiles | none |
| explanation contract | viewer-safe public explanations only |
| known weaknesses | Level 1 does not solve mixed bluff equilibrium |
| public default suitability | no Level 2 bot exists |

Do not code Level 2 before a follow-up explicitly accepts the strategy pack and
updates this document.

## Level 3: shallow deterministic search

| Requirement | Status | Evidence |
|---|---|---|
| perfect-information game | no | Masked Claims has private hands, reserve, pedestal identity, and veiled accepted masks. |
| small enough search space | not evaluated | Search is forbidden before the information-rule issue. |
| deterministic limits | not applicable | No search bot. |
| documented evaluator | not applicable | No search bot. |
| fallback policy | not applicable | Level 1 is the baseline. |
| explanation says search was used | not applicable | No search bot. |
| no hidden-information search | yes | Policy forbids sampling and hidden-state peeking. |
| ADR required? | yes for any search proposal | Hidden-information search would need explicit governance. |

## Exact information access table

| Information | Human acting seat sees? | Bot sees? | Public observer sees? | Tests/notes |
|---|---:|---:|---:|---|
| legal action tree | yes, for acting seat | yes, for acting seat | no private leaves | `tests/visibility.rs`; wasm bridge test |
| public board/state | yes | yes | yes | `project_view` tests |
| own private hand | yes | yes | no | `tests/bots.rs`; `tests/visibility.rs` |
| opponent private hand | no | no | no | no-leak tests |
| reserve order/contents | no | no | no | `tests/visibility.rs` |
| pedestal tile identity before challenge | no public viewer; claimant no longer receives it after claim | no | no | `claim-pending-window.trace.json` |
| veiled accepted mask identities | no | no | no | `accepted-mask-never-revealed.trace.json` |
| exposed challenged masks | yes | yes | yes | challenge traces |
| dev/test full state | test authority only | no for public bot | no | source review |

## Decision order summary

| Bot | Decision order |
|---|---|
| random legal | Enumerate legal leaves from Rust, choose one deterministically from the bot seed. |
| Level 1 claim | Prefer highest safe truthful claim, allow bounded underclaim/bluff according to deterministic parameters, then tie-break by seed. |
| Level 1 response | Challenge if public counting proves impossibility, otherwise compare a deterministic threshold to declared grade and public exposure context, then accept/challenge legally. |

## Style profiles

| Profile | Applies to bot | Policy variation | Hidden-info safe? | Status | Tests |
|---|---|---|---:|---|---|
| default | Level 1 | strongest current public baseline | yes | implemented | `tests/bots.rs` |

## Explanation examples

| Bot | Situation | Example explanation | Hidden-info safe? | Test |
|---|---|---|---:|---|
| random legal | any legal phase | Random legal choice from the current Rust action tree. | yes | `src/bots.rs` |
| Level 1 claim | held Master available | Claimed Master from a held Master using the highest safe legal claim. | yes | `bot-claim-and-response.trace.json` |
| Level 1 response | impossible declared grade by public counting | Challenged: all three Master masks are already visible to me. | yes | `certain-lie-challenge.trace.json` |

## Known weaknesses

| Bot | Weakness | Why acceptable | Mitigation/future trigger |
|---|---|---|---|
| random legal | no competence | required legality baseline only | not a public default |
| Level 1 | does not solve bluffing equilibrium or model opponent tendencies | Gate 11 requires legal reaction-window behavior and safe explanations | future Level 2 ticket with accepted evidence pack |

## Tests

| Test | Purpose | Bot(s) | Status | Notes |
|---|---|---|---|---|
| legality over seeds | bot chooses only legal action paths | all | covered | `tests/bots.rs`; simulation |
| determinism | fixed seed/view/rules/policy produce fixed decision | Level 1 | covered | `tests/bots.rs` |
| no-leak input view | hidden-info game bot uses allowed view | all public bots | covered | `tests/bots.rs`; visibility tests |
| no-leak explanation/ranking | rationale omits hidden facts | Level 1 | covered | `certain-lie-challenge.trace.json` |
| replay/hash | bot decisions reproduce in trace surfaces | Level 1 | covered | `bot-claim-and-response.trace.json`; `tests/replay.rs` |
| explanation smoke | non-random bot explains decisions | Level 1 | covered | `tests/bots.rs` |
| decision examples | expected examples choose legal actions | Level 1 | covered | `src/bots.rs` |

## Benchmarks

| Benchmark | Target | Current baseline | Bot(s) | Status | Notes |
|---|---:|---:|---|---|---|
| legal action generation | smoke floor | recorded by bench output | all | covered | `legal_actions_claim_phase`; `legal_actions_reaction_window` |
| candidate extraction | smoke floor | recorded by bench output | Level 1 | covered | included in bot decision benchmarks |
| bot decision latency | smoke floor | recorded by bench output | Level 1 | covered | `level1_bot_claim_decision`; `level1_bot_response_decision` |
| playout throughput | no calibrated floor | simulation summary | Level 1 | covered | `simulate --game masked_claims --games 1000` |
| explanation generation | smoke floor | included in decision path | Level 1 | covered | rationale generated with decision |
| WASM/browser smoke | no calibrated floor | wasm-api bridge test | Level 1 | covered | ticket 014 bridge smoke |

## Simulation metrics

| Run | Bots | Seeds/games | Metrics recorded | Failures | Notes |
|---|---|---:|---|---|---|
| `cargo run -p simulate -- --game masked_claims --games 1000` | Level 1 both seats | 1000 | completed games, wins/draws, average length, throughput | none in ticket 015 verification | Every game reached terminal in 16 actions. |

## Public default suitability

| Check | Status | Notes |
|---|---|---|
| bot is legal and deterministic | yes | legal action API plus tests |
| bot does not look broken | yes | acts in both claim and response phases |
| bot is fair under information rules | yes | uses own/public information only |
| explanations are safe and useful | yes | concise viewer-safe rationales |
| latency fits public UX | yes | benchmark smoke floors pass |
| known weaknesses acceptable | yes | public proof bot, not optimal strategy claim |
| public default decision | constrained yes | acceptable for Gate 11 proof and local demo |
