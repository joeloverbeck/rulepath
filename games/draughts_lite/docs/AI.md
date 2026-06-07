# Draughts Lite AI Notes

Game ID: `draughts_lite`

Implemented variant: `draughts_lite_standard`

Rules version: `draughts_lite-rules-v1`

Last updated: 2026-06-07

## Purpose

This is the per-game bot registry for Draughts Lite. It links to the strategy evidence instead of duplicating it.

Public v1/v2 Draughts Lite bots do not use MCTS, ISMCTS, Monte Carlo tree search or playout search, minimax/alpha-beta search, ML, RL, hidden-information inference, opening books, endgame databases, solved-game claims, or runtime LLM move selection.

## Bot Summary

| Bot | Level | Policy/version | Public default? | Information access | Status | Evidence |
|---|---:|---|---:|---|---|---|
| random legal | 0 | `draughts_lite_random_legal_v1` | no | Rust legal action tree and public state only | implemented/tested | `DraughtsLiteRandomBot`; bot tests; simulation |
| authored policy | 1 | `draughts_lite_level1_v1` | yes for public bot turns | public board, legal paths, public terminal/promotion/capture features only | implemented/tested | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), bot tests |
| search/solver policy | 2+ | none | no | not applicable | not implemented | excluded for public v1/v2 |

## Level 0: Random Legal Bot

| Item | Decision/evidence |
|---|---|
| legal action API used | Rust legal action tree leaves and command validation |
| deterministic seed behavior | same seed, state, rules, and legal tree choose the same complete legal path |
| action selection method | deterministic seeded selection among complete leaf paths, including multi-segment captures |
| simulation tests | `cargo run -p simulate -- --game draughts_lite --games 1000` |
| legality tests | `cargo test -p draughts_lite` bot/property coverage |
| replay/hash tests | `bot-action.trace.json`; `cargo run -p replay-check -- --game draughts_lite --all` |
| known limitations | random; not competent |
| public explanation text | random/legal fallback prose only when surfaced by bot effect |
| benchmark evidence | [BENCHMARKS.md](BENCHMARKS.md) |

## Level 1: Authored Policy Bot

Required evidence pack: [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md)

Competent-player analysis: [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md)

| Item | Summary |
|---|---|
| policy name/version | `draughts_lite_level1_v1` |
| candidate extraction | complete Rust legal paths only |
| priority shape | terminal capture, promotion, capture quality, complete continuation, avoiding obvious king hang when priorities tie, deterministic tie-break |
| mandatory capture handling | follows Rust legal tree; does not claim longest capture is required |
| multi-jump handling | never emits a partial continuation path |
| hidden/private state | none; game is perfect-information and policy uses public state only |
| explanation contract | short public rationale; no candidate rankings, score arrays, debug/hash/search terms |
| output effect | `BotChoseAction { level: 1, policy_id: "draughts_lite_level1_v1", action_path, rationale }` |
| latency posture | bounded local feature policy; no external calls |

## Explicit Exclusions

| Exclusion | Reason |
|---|---|
| MCTS/ISMCTS/Monte Carlo | forbidden for public v1/v2 and unnecessary for Gate 7 |
| minimax/alpha-beta/search solver | out of scope; would need separate benchmarks and policy docs |
| ML/RL/runtime LLM | forbidden public bot surface |
| opening books/endgame databases | out of scope and unnecessary for a scoped compound-action proof |
| hidden-information inference | not applicable; there is no hidden state |
| maximum-capture doctrine | not a rule of this variant; bot may prefer longer captures but must not imply requirement |

## Verification

| Command/test | Coverage | Status |
|---|---|---|
| `cargo test -p draughts_lite` | bot legality, determinism, public rationale, rule integration | passing |
| `cargo run -p simulate -- --game draughts_lite --games 1000` | random playout legality and bounded nonterminal taxonomy | passing |
| `cargo run -p replay-check -- --game draughts_lite --all` | bot trace and replay determinism | passing |
| `node apps/web/e2e/draughts-lite.smoke.mjs` | public bot/effect surfaces and no-leak DOM scan | passing |
| `cargo bench -p draughts_lite -- legal_actions` | legal-tree throughput | available |

## Public Default Suitability

| Check | Status | Notes |
|---|---|---|
| bot is legal and deterministic | yes | choices flow through Rust legal tree and validation |
| bot submits complete compound paths | yes | tests reject partial continuation behavior |
| bot does not look broken | yes | recognizes terminal, promotion, capture, and simple exposure patterns |
| bot is fair under information rules | yes | public board only |
| explanations are safe and useful | yes | no rankings/scores/internal state |
| latency fits public UX | yes | local bounded feature policy |
| public default decision | yes | Level 1 is suitable for Gate 7 public bot turns |
