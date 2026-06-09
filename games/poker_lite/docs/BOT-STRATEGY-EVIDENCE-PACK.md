# Crest Ledger Bot Strategy Evidence Pack

Game ID: `poker_lite`

Implemented variant: `poker_lite_standard`

Rules version: `poker-lite-rules-v1`

Bot target: Level 2 authored policy

Policy name/version: `poker-lite-crest-ledger-level2-v1` / v1

Date: 2026-06-09

## Status

Level 2 is implemented for Gate 10 in `PokerLiteLevel2Bot`.

This pack consumes [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) and summarizes the
evidence for legality, determinism, fairness, no hidden-state access, and
beatable bounded play.

## Explicit Public v1/v2 Exclusions

The policy does not use and must not grow:

- omniscient state;
- opponent private crest access;
- hidden center access before reveal;
- deck-tail or future-order access;
- hidden-state sampling or determinization;
- MCTS, ISMCTS, Monte Carlo, ML, RL, or runtime LLM policy;
- TypeScript legality or TypeScript bot policy.

## Source Documents and Evidence

| Document/source | Path/reference | Status | Notes |
|---|---|---|---|
| Rules | [RULES.md](RULES.md) | read | Legal actions, reveal timing, terminal outcomes. |
| Competent player analysis | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | complete | Strategy input for priority order. |
| Bot registry | [AI.md](AI.md) | complete | Policy registry and explanation boundary. |
| Bot implementation | [src/bots.rs](../src/bots.rs) | implemented | `PokerLiteLevel2Bot`, `PokerLiteBotInput`. |
| Bot tests | [tests/bots.rs](../tests/bots.rs) | passing | Main evidence surface. |
| Bot trace | [bot-action.trace.json](../tests/golden_traces/bot-action.trace.json) | present | Deterministic policy/action fixture. |

## Exact Bot Input View

| Input item | Included? | Source | Visible to acting seat? | Evidence |
|---|---:|---|---:|---|
| legal action tree/action paths | yes | Rust `legal_action_tree` | yes | `random_and_level2_decisions_are_legal_and_do_not_mutate_state` |
| public pledge state | yes | Rust projection/state public fields | yes | `level2_input_whitelist_excludes_forbidden_hidden_material` |
| own private rank and strength bucket | yes | acting seat private view | yes | `bot_explanations_and_effects_do_not_leak_hidden_cards_or_sampling_claims` |
| public center rank after reveal | yes | public view | yes | `level2_policy_uses_authored_priority_and_stable_tie_break` |
| deterministic seed | yes | policy tie-break only | not game information | `seeded_bots_are_deterministic_on_same_allowed_state` |
| opponent private crest | no | forbidden | no | whitelist test |
| hidden center before reveal | no | forbidden | no | whitelist test |
| deck tail/future order | no | forbidden | no | whitelist test |

## Candidate Extraction

Candidates are exactly legal leaf paths from the Rust legal action tree for the
active bot seat. The policy annotates only those legal candidates and returns an
`ActionPath` that validates through `validate_command`.

## Lexicographic Priority Vector

Earlier slots dominate later slots:

| Slot | Priority | Higher/better value | Evidence | Explanation fragment |
|---:|---|---|---|---|
| 1 | survival through legal continuation | avoid yield unless price is poor | `level2_bots_finish_many_games_with_legal_actions_under_cap` | `bounded public price` |
| 2 | protect made pair | match/lift when own rank matches revealed center | `level2_policy_uses_authored_priority_and_stable_tie_break` | `made public pair` |
| 3 | respect public price | match affordable price or yield poor price | playout and legality tests | `public pledge options` |
| 4 | high-rank pressure pre-reveal | press with high private rank before center reveal | bot-action trace and priority test | `high private rank before center reveal` |
| 5 | avoid reckless lift | prefer non-lift without made pair | priority test | `legal public pledge options` |
| 6 | close uncertainty | prefer hold/match where appropriate | playout test | `bounded public price` |
| 7 | stable action id and deterministic seed tie-break | reproducible final ordering | determinism test | not surfaced unless needed |

No large weighted score, static-data tactical condition, search tree, or sampled
belief model is used.

## Explanation Contract

The bot emits:

- public `BotChoseActionPublic`: policy id and action family only;
- private-to-actor `BotChoseActionPrivate`: policy id, action family, and own
  strength bucket only.

Evidence: `bot_explanations_and_effects_do_not_leak_hidden_cards_or_sampling_claims`.

## Evidence Fixtures

| Evidence | Test / trace | Expected behavior |
|---|---|---|
| random and Level 2 validate | `random_and_level2_decisions_are_legal_and_do_not_mutate_state` | Choices are legal command paths; selection does not mutate state. |
| deterministic fixed-state decisions | `seeded_bots_are_deterministic_on_same_allowed_state` | Same state and seed produce identical decisions. |
| input whitelist | `level2_input_whitelist_excludes_forbidden_hidden_material` | Input summary excludes private card ids/labels, hidden center id, deck tail, seed, and opponent language. |
| priority example | `level2_policy_uses_authored_priority_and_stable_tie_break` | High private pre-reveal presses; made-pair facing price matches. |
| explanation no-leak | `bot_explanations_and_effects_do_not_leak_hidden_cards_or_sampling_claims` | Public/private bot effects omit hidden card ids and search/sampling claims. |
| repeated playout legality | `level2_bots_finish_many_games_with_legal_actions_under_cap` | Level 2 bots finish repeated games under the 16-action cap. |
| golden bot trace | [bot-action.trace.json](../tests/golden_traces/bot-action.trace.json) | Documents policy id, allowed input summary, and expected `press` action. |

## Known Weaknesses

| Weakness | Why acceptable for public Level 2 | Future trigger |
|---|---|---|
| No opponent belief model | Avoids hidden-state sampling and keeps the policy explainable. | Add only with an accepted ADR and no-leak tests. |
| No long-horizon betting trap analysis | Crest Ledger is tiny and bounded; current policy is competent but beatable. | Add documented heuristics if simulations show repeated obvious mistakes. |
| No style profiles | One safe default ships first. | Add after benchmark and UX evidence. |

## Verification Commands

- `cargo test -p poker_lite --test bots`
- `cargo test -p poker_lite`
- `node scripts/check-doc-links.mjs`

`simulate --game poker_lite` evidence is expected after native tool registration
lands in a later Gate 10 ticket.
