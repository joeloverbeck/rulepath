# Plain Tricks AI

Game ID: `plain_tricks`

Implemented variant: `plain_tricks_standard`

Rules version: `plain-tricks-rules-v1`

Last updated: 2026-06-09

## Purpose

This is the per-game bot registry for Plain Tricks. It records shipped Rust bot
policies, their information boundary, explanation posture, and evidence. Rules
authority remains [RULES.md](RULES.md).

## Bot Summary

| Bot | Level | Policy/version | Public default? | Information access | Status | Evidence |
|---|---:|---|---:|---|---|---|
| seeded random legal | 0 | `plain-tricks-random-legal-v0` / v1 | no | Rust legal action tree only | implemented and tested | `random_and_level2_decisions_are_legal_and_do_not_mutate_state` in [bots.rs](../tests/bots.rs) |
| authored policy | 2 | `plain-tricks-level2-v1` / v1 | yes, constrained to Rust callers | acting-seat bot input: legal tree, own hand, public current trick, public history, public scores | implemented and tested | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |

No Level 1 bot ships for Plain Tricks. No Level 3 search bot ships.

Public v1/v2 bots must not use MCTS, ISMCTS, Monte Carlo, ML, RL, runtime LLM
policy, hidden-state sampling, opponent-hand access, tail access, seed
reconstruction, or TypeScript legality/policy.

## Level 0: Random Legal Bot

`PlainTricksRandomBot` collects legal action paths from `legal_action_tree` and
uses `ai_core::RandomLegalBot` with a deterministic seed. The selected command
must validate through `validate_command` before applying.

Known limitation: it is intentionally not competent. It is a legality and
simulation baseline only.

Evidence:

- `random_and_level2_decisions_are_legal_and_do_not_mutate_state`
- `seeded_bots_are_deterministic_on_same_allowed_state`

## Level 2: Authored Policy Bot

`PlainTricksLevel2Bot` uses policy id `plain-tricks-level2-v1`.

Allowed inputs:

- current Rust legal action tree for the bot seat;
- own hand from the bot seat-private view;
- public phase, active seat, round/trick index, current leader, and scores;
- public current trick and completed trick history;
- public off-suit evidence as observed play history only;
- deterministic policy seed for stable tie-break only.

Forbidden inputs:

- opponent hand;
- tail cards;
- setup seed or reconstructed shuffle order as game information;
- full internal replay traces with hidden payloads;
- sampled or enumerated opponent holdings;
- explicit hidden void flags or hidden-state candidate rankings.

Decision summary:

1. Stay legal and choose only Rust-supplied legal leaves.
2. When following, win with the cheapest led-suit card that beats the lead when
   control is useful.
3. When unable to win, duck with the lowest legal card.
4. When leading and behind or tied, prefer likely winners.
5. Otherwise lead low from the longest own suit.
6. Use stable suit/rank/card id plus deterministic seed tie-break.

## Explanation Boundary

Bot decisions emit viewer-safe bot-choice effects:

- public effect: policy id and action family only.

Decision rationale may mention own chosen card, public trick posture, public
scores, and public history. It must not mention opponent hand identities, tail
identities, seed/shuffle claims, sampled holdings, debug state, explicit hidden
void flags, or search/learning claims.

## Verification

- `cargo test -p plain_tricks --test bots`
- `cargo test -p plain_tricks`
- `node scripts/check-doc-links.mjs`
- Bot trace: [bot-action.trace.json](../tests/golden_traces/bot-action.trace.json)
