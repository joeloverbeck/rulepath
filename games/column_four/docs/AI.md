# Column Four AI Notes

Game ID: `column_four`

Implemented variant: `column_four_standard`

Rules version: `column_four-rules-v1`

Last updated: 2026-06-06

## Purpose

This is the per-game bot registry for Column Four. It links to the strategy evidence instead of duplicating it.

Public v1/v2 Column Four bots do not use MCTS, ISMCTS, Monte Carlo tree search, ML, RL, hidden-information inference, or search-heavy policies.

## Bot Summary

| Bot | Level | Policy/version | Public default? | Information access | Status | Evidence |
|---|---:|---|---:|---|---|---|
| random legal | 0 | `column_four-random-legal-v1` | no | Rust legal action tree and public state only | implemented/tested | `ColumnFourRandomBot`; bot tests; random simulation |
| authored tactical policy | 2 | `column_four-level2-v1` | yes for showcase bot turns | public board, legal targets, active seat only | implemented/tested | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), bot tests |
| shallow deterministic search | 3 | none | no | not applicable | not implemented | excluded for Gate 5 |

## Level 0: Random Legal Bot

| Item | Decision/evidence |
|---|---|
| legal action API used | `legal_action_tree` / Rust action path validation |
| deterministic seed behavior | same seed, state, rules, and legal tree choose the same legal action |
| action selection method | deterministic seeded selection among legal `drop/c*` paths |
| simulation tests | `cargo run -p simulate -- --game column_four --games 1000` |
| legality tests | `cargo test -p column_four` bot/property coverage |
| replay/hash tests | `bot-action.trace.json`; `cargo run -p replay-check -- --game column_four --all` |
| known limitations | random; not competent |
| public explanation text | random/legal fallback prose only when surfaced by bot effect |
| benchmark evidence | [BENCHMARKS.md](BENCHMARKS.md), `level0_bot_decision` |

## Level 2: Authored Policy Bot

Required evidence pack: [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md)

Competent-player analysis: [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md)

| Item | Summary |
|---|---|
| policy name/version | `column_four-level2-v1` |
| evidence pack status | complete for Gate 5 |
| candidate extraction | legal columns from Rust action tree and public landing facts |
| priority vector | immediate win, immediate block, avoid immediate reply, threat extension, center preference, deterministic fallback |
| bounded scoring tie-breakers | public tactical/positional priorities only |
| deterministic seeded tie-break | used only after policy priorities tie |
| explanation contract | public prose such as winning now, blocking a visible threat, or preferring central pressure |
| known weaknesses | bounded one-ply tactical policy; does not solve deep traps |
| public default suitability | suitable for Gate 5 showcase because it plays legally, blocks obvious threats, and explains safely |

## Exact Information Access

| Information | Human acting seat sees? | Bot sees? | Public observer sees? | Tests/notes |
|---|---:|---:|---:|---|
| legal action tree | yes | yes | yes, through controls/action surfaces | action/tree tests and WASM smoke |
| public board/state | yes | yes | yes | visibility tests and browser smoke |
| own private hand/role/zone | not applicable | not applicable | not applicable | Column Four is perfect-information. |
| opponent private hand/role/zone | not applicable | not applicable | not applicable | No hidden zones. |
| unrevealed deck/order/future random outcome | not applicable | no | no | Rules use no randomness. |
| private logs/full internal state | no for public bot | no for public bot | no | no-leak browser smoke |
| candidate ranking/raw scores | no | internal bounded policy only | no | bot explanation tests and no-leak smoke |

## Explanation Examples

| Bot | Situation | Example explanation | Hidden-info safe? | Test |
|---|---|---|---:|---|
| random legal | no authored policy is requested | random legal choice from Rust legal columns | yes | bot legality tests |
| Level 2 | own immediate win exists | chose the visible winning column | yes | strategy examples and bot tests |
| Level 2 | opponent immediate threat exists | chose the column that blocks a visible immediate threat | yes | strategy examples and bot tests |
| Level 2 | no urgent tactic exists | prefers central pressure | yes | center-preference test |

Explanations must not list candidate rankings, score arrays, search depth, hidden facts, or internal state.

## Known Weaknesses

| Bot | Weakness | Why acceptable | Mitigation/future trigger |
|---|---|---|---|
| Level 0 random legal | Can ignore wins and blocks. | Required baseline and simulation bot only. | Use Level 2 for public showcase mode. |
| Level 2 authored policy | Bounded one-ply tactical policy can miss deeper traps. | Gate 5 forbids MCTS/search-heavy public bots and prioritizes explainable play. | Future authored-policy expansion with docs/benchmarks if public need appears. |

## Tests And Benchmarks

| Evidence | Purpose | Status | Notes |
|---|---|---|---|
| `cargo test -p column_four` | bot legality, determinism, public rationale, rule integration | passing | includes random and Level 2 policy tests |
| `cargo run -p simulate -- --game column_four --games 1000` | random playout legality and terminal outcomes | passing | records winner/draw distribution and average length |
| `cargo run -p replay-check -- --game column_four --all` | bot trace and replay determinism | passing | includes `bot-action.trace.json` |
| `npm --prefix apps/web run smoke:e2e` | public bot rationale renders without leaks | passing | Column Four smoke covers bot explanation |
| `cargo bench -p column_four -- level0_bot_decision` | random bot decision throughput | available | thresholds in [BENCHMARKS.md](BENCHMARKS.md) |
| `cargo bench -p column_four -- level2_bot_decision` | authored bot decision throughput | available | thresholds in [BENCHMARKS.md](BENCHMARKS.md) |

## Public Default Suitability

| Check | Status | Notes |
|---|---|---|
| bot is legal and deterministic | yes | choices flow through Rust legal action tree and validation |
| bot does not look broken | yes | Level 2 handles immediate wins/blocks and central preference |
| bot is fair under information rules | yes | perfect-information public board only |
| explanations are safe and useful | yes | no rankings/scores/internal state |
| latency fits public UX | yes | native bot benchmarks recorded |
| known weaknesses acceptable | yes | one-ply limits are documented |
| public default decision | yes | Level 2 is suitable for Gate 5 showcase bot turns |
