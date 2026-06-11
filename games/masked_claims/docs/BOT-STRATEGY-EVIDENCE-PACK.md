# Masked Claims Bot Strategy Evidence Pack

Game ID: `masked_claims`

Implemented variant: `masked_claims_standard`

Rules version: `masked-claims-rules-v1`

Bot target: Level 1 rule-informed policy

Policy name/version: `masked-claims-level1-v1` / v1

Date: 2026-06-11

## Status

Level 0 and Level 1 are implemented in [bots.rs](../src/bots.rs). This pack
records the implemented Level 1 evidence and consumes
[COMPETENT-PLAYER.md](COMPETENT-PLAYER.md).

The policy is legal, deterministic under declared inputs, explainable,
beatable, and viewer-safe. It is not a Level 2 authored-policy claim.

## Explicit Public v1/v2 Exclusions

The policy does not use and must not grow:

- omniscient state;
- opponent hand access;
- reserve access;
- accepted mask identities;
- pedestal tile identity before challenge reveal;
- sampled, enumerated, or determinized hidden holdings;
- MCTS, ISMCTS, Monte Carlo, ML, RL, or runtime LLM policy;
- TypeScript legality or TypeScript bot policy.

## Source Documents and Evidence

| Document/source | Path/reference | Status | Notes |
|---|---|---|---|
| Rules | [RULES.md](RULES.md) | read | Claim, reaction, resolution, visibility, and bot boundaries. |
| Competent player analysis | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | complete | Strategy posture and hidden-info boundary. |
| Bot implementation | [bots.rs](../src/bots.rs) | implemented | `MaskedClaimsRandomBot` and `MaskedClaimsLevel1Bot`. |
| Bot tests | [bots.rs tests](../tests/bots.rs) | passing | Legal actions, determinism, many completed games, no hidden terms in rationale. |
| Visibility tests | [visibility.rs](../tests/visibility.rs) | passing | Public/opponent/export no-leak surfaces. |
| Benchmarks | [BENCHMARKS.md](BENCHMARKS.md) | smoke floors | Level 1 claim/response decision latency is benchmarked; calibration remains follow-up. |

## Exact Bot Input View

| Input item | Included? | Source | Visible to acting seat? | Evidence |
|---|---:|---|---:|---|
| legal action tree/action paths | yes | Rust `legal_action_tree` | yes | bot legality tests |
| own seat id | yes | actor/viewer | yes | `MaskedClaimsBotInput` |
| own hand | yes | seat-private projection | yes | claim policy tests |
| public phase, turn, claimant | yes | public projection | yes | deterministic summary |
| public pedestal declared grade | yes | public projection | yes | response policy |
| public exposed rows | yes | public projection | yes | public counting guard |
| public scores | yes | public projection | yes | response threshold context |
| policy seed | yes | bot constructor | not game information | deterministic tie/posture gate |
| opponent hand | no | forbidden | no | no-leak tests |
| reserve identities | no | forbidden | no | no-leak tests |
| accepted mask identities | no | forbidden | no | no-leak tests |
| hidden pedestal identity | no | forbidden before reveal | no | hidden-state independence test |

## Candidate Extraction

Candidates are exactly legal leaf paths from the Rust legal action tree for the
active bot seat. Claim candidates are `claim/<own-mask>/<declared-grade>` paths
from the seat-private legal tree. Response candidates are the legal
`respond/accept` and `respond/challenge` paths from the reaction tree.

The bot returns an `ActionPath` that validates through `validate_command`. It
does not mutate state directly and does not synthesize out-of-tree paths.

## Implemented Claim Policy

| Policy piece | Implemented behavior | Visible facts used | Evidence |
|---|---|---|---|
| honest default | Prefer a legal honest claim with high declared grade and stable tile tie-break. | own hand, legal tree | `level1_is_deterministic_and_finishes_many_games_legally` |
| bounded bluff posture | For deterministic seed posture, prefer a legal overclaim candidate if it is not a certain lie by own/public counting. | own hand, exposed rows, legal tree, seed | `level1_bot_selects_legal_actions_in_claim_and_response_phases` |
| underclaim posture | For deterministic seed posture, prefer a legal underclaim candidate, falling back to honest if needed. | own hand, legal tree, seed | bot unit tests in `src/bots.rs` |
| counting guard | Do not choose a lie where all copies of the declared grade are known from own hand plus exposed rows. | own hand, exposed rows | source-level policy and no-leak tests |
| stable tie-break | Candidate identity and seed give deterministic ordering. | candidate path, seed | determinism tests |

## Implemented Response Policy

| Policy piece | Implemented behavior | Visible facts used | Evidence |
|---|---|---|---|
| certain-lie challenge | Challenge when own hand plus exposed rows account for all copies of the declared grade. | own hand, exposed rows, declared grade | `certain-lie-challenge.trace.json`; bot tests |
| threshold challenge | Challenge when remaining visible plausibility falls below the declared-grade threshold. | own hand, exposed rows, declared grade | `level1_bot_selects_legal_actions_in_claim_and_response_phases` |
| accept fallback | Accept when the claim remains plausible or challenge is unavailable. | declared grade, legal response tree | bot tests |
| rationale | Explain with public/own-view grade facts only. | viewer-safe input | rationale no-leak assertions |

## Explanation Contract

The Level 1 bot may explain decisions using only:

- policy id/version;
- action family;
- own held grade, without tile id;
- public declared grade;
- public exposed-row counting;
- public score value;
- deterministic policy posture.

It must not explain with opponent hand contents, reserve contents, accepted
mask identities, hidden pedestal identity, sampled hidden states, or search/ML
claims.

## Evidence Fixtures

| Evidence | Test / trace target | Expected behavior |
|---|---|---|
| legal Level 0 and Level 1 decisions | [tests/bots.rs](../tests/bots.rs) | Choices validate in claim and response phases. |
| deterministic Level 1 decision | [tests/bots.rs](../tests/bots.rs) | Same state and seed produce identical decisions. |
| repeated legal completions | [tests/bots.rs](../tests/bots.rs) | Level 1 finishes many games without illegal actions. |
| hidden-state independence | `src/bots.rs` unit tests | Identical allowed input gives identical response and rationale. |
| rationale no-leak | [tests/bots.rs](../tests/bots.rs), [tests/visibility.rs](../tests/visibility.rs) | Rationale omits `mask_g`, reserve, opponent-hand, and pedestal-tile terms. |
| golden bot trace | [bot-claim-and-response.trace.json](../tests/golden_traces/bot-claim-and-response.trace.json) | Records Level 1 policy ids and viewer-safe rationales. |
| benchmark operations | [BENCHMARKS.md](BENCHMARKS.md) | `level1_bot_claim_decision` and `level1_bot_response_decision` have smoke floors. |

## Known Weaknesses

| Weakness | Why acceptable for Level 1 | Future trigger |
|---|---|---|
| No calibrated balance report yet | The current gate has smoke benchmarks and legality/no-leak evidence; statistical calibration is named follow-up. | Mirrored Level 1 win rates outside roughly 40-60 after registration. |
| Simple threshold challenge | Keeps the policy explainable and beatable. | Repeated simulations show obvious under- or over-challenging. |
| No opponent belief model | Avoids hidden-state sampling and forbidden search classes. | Accepted ADR and new no-leak tests. |

## Verification Commands

- `node scripts/check-doc-links.mjs`
- `cargo test -p masked_claims --test bots`
- `cargo bench -p masked_claims -- --warm-up-time 1 --measurement-time 1`

## Review Checklist

- Legal action API and validation path are exact.
- Bot input view is explicit.
- No omniscient state, hidden-state shortcuts, or future random outcomes are used.
- Candidate extraction uses legal action paths and allowed views.
- Tie-breaks and posture gates are deterministic under seed and candidate identity.
- Hidden-information no-leak tests cover explanations, replay exports, and visibility surfaces.
- Public v1/v2 MCTS, ISMCTS, Monte Carlo bots, ML, and RL are absent.
