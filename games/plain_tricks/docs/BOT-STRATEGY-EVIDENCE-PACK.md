# Plain Tricks Bot Strategy Evidence Pack

Game ID: `plain_tricks`

Implemented variant: `plain_tricks_standard`

Rules version: `plain-tricks-rules-v1`

Bot target: Level 2 authored policy

Policy name/version: `plain-tricks-level2-v1` / v1

Date: 2026-06-09

## Status

Level 2 is not implemented yet. This pack is the design input for
GAT101PLATRI-013 and consumes [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md).

The policy must be legal, deterministic, explainable, fair, beatable, and
viewer-safe before it can be shipped.

## Explicit Public v1/v2 Exclusions

The policy does not use and must not grow:

- omniscient state;
- opponent hand access;
- tail access;
- seed, shuffle reconstruction, or future deal order;
- full internal replay payloads;
- opponent private explanations;
- sampled, enumerated, or determinized hidden holdings;
- MCTS, ISMCTS, Monte Carlo, ML, RL, or runtime LLM policy;
- TypeScript legality or TypeScript bot policy.

## Source Documents and Evidence

| Document/source | Path/reference | Status | Notes |
|---|---|---|---|
| Rules | [RULES.md](RULES.md) | read | Legal plays, trick resolution, scoring. |
| Competent player analysis | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | complete | Strategy input for priority order. |
| Replay fixtures | [replay.rs](../tests/replay.rs) | passing | Golden trace and export evidence. |
| Visibility tests | [visibility.rs](../tests/visibility.rs) | passing | No-leak projection evidence. |
| Bot implementation | `games/plain_tricks/src/bots.rs` | pending | Implemented in GAT101PLATRI-013. |
| Bot tests | `games/plain_tricks/tests/bots.rs` | pending | Implemented in GAT101PLATRI-013. |

## Exact Bot Input View

| Input item | Included? | Source | Visible to acting seat? | Evidence target |
|---|---:|---|---:|---|
| legal action tree/action paths | yes | Rust `legal_action_tree` | yes | legality test |
| own seat id | yes | actor/viewer | yes | input whitelist |
| own hand | yes | seat-private view | yes | input whitelist |
| current trick public surface | yes | public view | yes | priority test |
| public completed trick history | yes | public view | yes | priority test |
| trick counts/totals | yes | public view | yes | repeated playout test |
| round/trick index and active/leader seats | yes | public view | yes | deterministic state summary |
| terminal flag | yes | public view | yes | playout termination test |
| public off-suit evidence | yes | completed public plays | yes | priority test |
| explicit void flags/table | no | forbidden | no | no-leak test |
| opponent hand | no | forbidden | no | input whitelist |
| tail cards | no | forbidden | no | input whitelist |
| seed/shuffle order | no | forbidden | no | input whitelist |
| full internal trace/export with hidden payloads | no | forbidden | no | no-leak test |

## Candidate Extraction

Candidates are exactly legal leaf paths from the Rust legal action tree for the
active bot seat. The policy annotates only those legal candidates and returns an
`ActionPath` that validates through `validate_command`.

The bot must not synthesize card ids outside the legal tree, inspect rejected
diagnostics to learn hidden state, or request TypeScript-side filtering.

## Lexicographic Priority Vector

Earlier slots dominate later slots:

| Slot | Priority | Higher/better value | Explanation fragment |
|---:|---|---|---|
| 1 | rules legality | choose only Rust-supplied legal leaves | `legal play option` |
| 2 | cheapest winning follow | win current trick with the lowest legal led-suit card that beats the lead | `can win the led suit cheaply` |
| 3 | necessary control | prefer winning when behind or tied late in a round | `needs lead control` |
| 4 | conserve winners | avoid spending a higher winner when a lower winner is enough | `preserves higher card` |
| 5 | duck impossible tricks | if no legal card can win, play the lowest legal card | `cannot win this trick` |
| 6 | lead likely winner | when leading, prefer a high card whose suit has public lower cards spent | `likely winning lead` |
| 7 | lead low from length | when no clear winner exists, lead the lowest card from the longest own suit | `low lead from length` |
| 8 | stable card id tie-break | sort by suit/rank/card id for reproducibility | not surfaced unless needed |

No large weighted score, static-data tactical condition, search tree, sampled
belief model, or opponent-hand enumeration is used.

## Explanation Contract

The bot may explain decisions using only:

- policy id/version;
- action family;
- own card chosen;
- public current trick;
- public completed trick history;
- public trick counts and round/trick index;
- public off-suit evidence phrased as observed play history.

The bot must not explain with:

- opponent card ids still hidden;
- tail card ids;
- seed/shuffle claims;
- sampled opponent ranges;
- explicit hidden void flags;
- "I know" claims about private material.

## Evidence Fixtures To Add In GAT101PLATRI-013

| Evidence | Test / trace target | Expected behavior |
|---|---|---|
| legal decisions | `random_and_level2_decisions_are_legal_and_do_not_mutate_state` | Choices are legal command paths; selection does not mutate state. |
| deterministic decisions | `seeded_level2_decisions_are_deterministic` | Same allowed state and seed produce identical decisions. |
| input whitelist | `level2_input_whitelist_excludes_forbidden_hidden_material` | Summary excludes opponent hand, tail, seed, and explicit void flags. |
| priority example | `level2_policy_uses_authored_priority_and_stable_tie_break` | Cheap winning follow, duck, and lead priorities match this pack. |
| explanation no-leak | `bot_explanations_and_effects_do_not_leak_hidden_cards_or_sampling_claims` | Public/private bot effects omit hidden cards and search/sampling claims. |
| repeated playout legality | `level2_bots_finish_many_games_with_legal_actions_under_cap` | Level 2 bots finish repeated games under the 24-play cap. |
| golden bot trace | `bot-action.trace.json` | Refreshed with policy id and deterministic selected action. |

## Known Weaknesses

| Weakness | Why acceptable for public Level 2 | Future trigger |
|---|---|---|
| No opponent belief model | Avoids hidden-state sampling and keeps the policy explainable. | Add only with an accepted ADR and no-leak tests. |
| No long-horizon endgame search | Plain Tricks is small but hidden-info; authored heuristics are competent and beatable. | Add documented heuristics if simulations show repeated obvious mistakes. |
| No style profiles | One safe default ships first. | Add after benchmark and UX evidence. |

## Verification Commands

- `node scripts/check-doc-links.mjs`
- `node scripts/check-catalog-docs.mjs`

Executable bot legality, determinism, and no-leak checks are implemented in
GAT101PLATRI-013.
