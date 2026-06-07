# Directional Flip AI Notes

Game ID: `directional_flip`

Implemented variant: `directional_flip_standard`

Rules version: `directional_flip-rules-v1`

Last updated: 2026-06-07

## Purpose

This is the per-game bot registry for Directional Flip. It links to the strategy evidence instead of duplicating it.

Public v1/v2 Directional Flip bots do not use MCTS, ISMCTS, Monte Carlo tree search or playouts, minimax/alpha-beta search, ML, RL, hidden-information inference, opening books, exact stable-disc solvers, or runtime LLM move selection.

## Bot Summary

| Bot | Level | Policy/version | Public default? | Information access | Status | Evidence |
|---|---:|---|---:|---|---|---|
| random legal | 0 | `directional_flip_random_legal_v1` | no | Rust legal action tree and public state only | implemented/tested | `DirectionalFlipRandomBot`; bot tests; random simulation |
| authored Level 2-lite policy | 2 | `directional_flip_level2_lite_v1` | yes for public bot turns | public board, Rust legal targets, public previews/counts only | implemented/tested | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), bot tests |
| search/solver policy | 3+ | none | no | not applicable | not implemented | excluded for public v1/v2 |

## Level 0: Random Legal Bot

| Item | Decision/evidence |
|---|---|
| legal action API used | Rust legal action tree / command validation |
| deterministic seed behavior | same seed, state, rules, and legal tree choose the same legal action |
| action selection method | deterministic seeded selection among legal `place/*` or mandatory `pass/forced` paths |
| simulation tests | `cargo run -p simulate -- --game directional_flip --games 1000` |
| legality tests | `cargo test -p directional_flip` bot/property coverage |
| replay/hash tests | `bot-action.trace.json`; `cargo run -p replay-check -- --game directional_flip --all` |
| known limitations | random; not competent |
| public explanation text | random/legal fallback prose only when surfaced by bot effect |
| benchmark evidence | [BENCHMARKS.md](BENCHMARKS.md), `level0_bot_decision` |

## Level 2-lite: Authored Policy Bot

Required evidence pack: [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md)

Competent-player analysis: [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md)

| Item | Summary |
|---|---|
| policy name/version | `directional_flip_level2_lite_v1` |
| evidence pack status | complete for Gate 6 |
| candidate extraction | legal placements or forced pass from Rust action tree |
| priority vector | forced pass, favorable terminal, legal corner, avoid open-corner danger, reduce opponent mobility, preserve own mobility, stable edge/corner extension, lower frontier exposure, late count pressure, deterministic tie-break |
| bounded scoring tie-breakers | public mobility/frontier/count features only |
| deterministic seeded tie-break | used only after higher priorities tie |
| explanation contract | public prose tied to visible board facts |
| known weaknesses | no deep search, no exact stability solver, no opening book, beatable by stronger players |
| public default suitability | suitable for Gate 6 because it plays legally, values corners/mobility, handles forced pass, and explains safely |

## Exact Information Access

| Information | Human acting seat sees? | Bot sees? | Public observer sees? | Tests/notes |
|---|---:|---:|---:|---|
| legal action tree | yes | yes | yes through controls/action surfaces | action/tree tests and WASM smoke |
| public board/state | yes | yes | yes | visibility tests and browser smoke |
| public previews/counts | yes | yes | yes | preview/apply parity tests |
| own private hand/role/zone | not applicable | not applicable | not applicable | Perfect-information board only. |
| opponent private hand/role/zone | not applicable | not applicable | not applicable | No hidden zones. |
| unrevealed deck/order/future random outcome | not applicable | no | no | Rules use no randomness. |
| private logs/full internal state | no for public bot | no for public bot | no | no-leak browser smoke |
| candidate ranking/raw scores | no | internal bounded policy only | no | explanation tests and no-leak smoke |

## Explanation Examples

| Bot | Situation | Example explanation | Hidden-info safe? | Test |
|---|---|---|---:|---|
| Level 0 | no authored policy requested | random legal choice from Rust legal targets | yes | bot legality tests |
| Level 2-lite | only legal action is forced pass | no placement is legal, so pass is mandatory | yes | forced-pass bot test |
| Level 2-lite | legal corner exists | takes a corner anchor | yes | corner-priority test |
| Level 2-lite | unsafe adjacent-to-open-corner move exists | avoids giving up corner access | yes | open-corner danger test |
| Level 2-lite | mobility choice dominates | reduces the opponent's visible choices or keeps future options open | yes | mobility tests |
| Level 2-lite | late board tie-break | improves the visible late count | yes | strategy evidence and bot tests |

Explanations must not list candidate rankings, score arrays, search depth, hidden facts, replay hashes, freshness internals, or full state dumps.

## Known Weaknesses

| Bot | Weakness | Why acceptable | Mitigation/future trigger |
|---|---|---|---|
| Level 0 random legal | Can ignore corners, mobility, and terminal chances. | Required baseline and simulation bot only. | Use Level 2-lite for public bot turns. |
| Level 2-lite | Bounded one-ply public-feature policy misses multi-turn sacrifices and exact stability. | Gate 6 forbids solver/search-heavy public bots and prioritizes explainable play. | Future accepted spec/ADR for stronger bot class. |
| Level 2-lite | No opening book or endgame solver. | Avoids copied strategy assets and solver complexity. | Add original evidence and tests before any future enhancement. |

## Tests And Benchmarks

| Evidence | Purpose | Status | Notes |
|---|---|---|---|
| `cargo test -p directional_flip` | bot legality, determinism, public rationale, forced pass, rule integration | passing | includes Level 0 and Level 2-lite tests |
| `cargo run -p simulate -- --game directional_flip --games 1000` | random playout legality and terminal outcomes | passing | records winner/draw distribution and average length |
| `cargo run -p replay-check -- --game directional_flip --all` | bot trace and replay determinism | passing | includes `bot-action.trace.json` |
| `node apps/web/e2e/directional-flip.smoke.mjs` | public bot rationale renders without leaks | passing | Directional Flip smoke covers bot explanation and forced pass |
| `cargo bench -p directional_flip -- level0_bot_decision` | random bot decision throughput | available | thresholds in [BENCHMARKS.md](BENCHMARKS.md) |
| `cargo bench -p directional_flip -- level2_bot_decision` | authored bot decision throughput | available | thresholds in [BENCHMARKS.md](BENCHMARKS.md) |

## Public Default Suitability

| Check | Status | Notes |
|---|---|---|
| bot is legal and deterministic | yes | choices flow through Rust legal action tree and validation |
| bot handles mandatory pass | yes | Level 2-lite has explicit forced-pass priority |
| bot does not look broken | yes | values corners, avoids obvious corner concessions, uses mobility |
| bot is fair under information rules | yes | perfect-information public board only |
| explanations are safe and useful | yes | no rankings/scores/internal state |
| latency fits public UX | yes | native bot benchmarks recorded |
| public default decision | yes | Level 2-lite is suitable for Gate 6 public bot turns |
