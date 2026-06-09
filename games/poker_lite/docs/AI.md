# Crest Ledger AI

Game ID: `poker_lite`

Implemented variant: `poker_lite_standard`

Rules version: `poker-lite-rules-v1`

Last updated: 2026-06-09

## Purpose

This is the per-game bot registry for Crest Ledger. It records shipped Rust bot
policies, their information boundary, explanation posture, and evidence. Rules
authority remains [RULES.md](RULES.md).

## Bot Summary

| Bot | Level | Policy/version | Public default? | Information access | Status | Evidence |
|---|---:|---|---:|---|---|---|
| seeded random legal | 0 | `poker-lite-random-legal-v0` / v1 | no | Rust legal action tree only | implemented and tested | `random_and_level2_decisions_are_legal_and_do_not_mutate_state` in [bots.rs](../tests/bots.rs) |
| authored policy | 2 | `poker-lite-crest-ledger-level2-v1` / v1 | yes, constrained to Rust callers | acting-seat bot input: legal tree, public pledge state, own private rank/bucket, public center rank only after reveal | implemented and tested | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |

No Level 1 bot ships for Crest Ledger. No Level 3 search bot ships.

Public v1/v2 bots must not use MCTS, ISMCTS, Monte Carlo, ML, RL, runtime LLM
policy, hidden-state sampling, or TypeScript legality/policy.

## Level 0: Random Legal Bot

`PokerLiteRandomBot` collects legal action paths from `legal_action_tree` and
uses `ai_core::RandomLegalBot` with a deterministic seed. The selected command
must validate through `validate_command` before applying.

Known limitation: it is intentionally not competent. It is a legality and
simulation baseline only.

Evidence:

- `random_and_level2_decisions_are_legal_and_do_not_mutate_state`
- `seeded_bots_are_deterministic_on_same_allowed_state`

## Level 2: Authored Policy Bot

`PokerLiteLevel2Bot` uses policy id `poker-lite-crest-ledger-level2-v1`.

Allowed inputs:

- current Rust legal action tree for the bot seat;
- public phase, active seat, shared pool, contributions, round unit, outstanding
  amount, and lift-cap state;
- own private rank and own strength bucket;
- public center rank only after the center is revealed;
- deterministic policy seed for stable tie-break only.

Forbidden inputs:

- opponent private crest;
- hidden center before reveal;
- deck tail or future random order;
- setup seed as game information;
- full internal replay trace;
- sampled or enumerated opponent holdings.

Decision summary:

1. Stay legal and avoid yielding unless public price makes continuing poor.
2. Protect a made public pair with match/lift responses.
3. Respect the public price and avoid reckless lifts without a made pair.
4. Press pre-reveal with a high private rank.
5. Close uncertain positions with hold/match.
6. Use stable action id plus deterministic seed tie-break.

## Explanation Boundary

Bot decisions emit viewer-safe bot-choice effects:

- public effect: policy id and action family;
- private-to-actor effect: policy id, action family, and own strength bucket.

Public text may mention public price, public center status, own private rank
category, own made-pair bucket after center reveal, and selected action family.
It must not mention hidden center identity, opponent private crest, deck tail,
sampled holdings, debug state, or search/learning claims.

## Verification

- `cargo test -p poker_lite --test bots`
- `cargo test -p poker_lite`
- `node scripts/check-doc-links.mjs`
- Bot trace: [bot-action.trace.json](../tests/golden_traces/bot-action.trace.json)
