# High Card Duel Implementation Admission

Game ID: `high_card_duel`

Rules version: `high-card-duel-rules-v1`

Last updated: 2026-06-07

## Admission Summary

High Card Duel is admitted as the Gate 8 chance and hidden-information proof game. The implementation is game-local Rust with a viewer-filtered WASM/web presentation.

## Required Evidence

| Area | Evidence |
|---|---|
| rules and validation | [RULES.md](RULES.md), [RULE-COVERAGE.md](RULE-COVERAGE.md), `cargo test -p high_card_duel` |
| deterministic chance | setup/property/replay tests and golden traces |
| hidden information | visibility tests, WASM no-leak tests, browser no-leak smoke |
| replay/export | internal native traces plus public observer export taxonomy |
| tooling | simulate, replay-check, fixture-check, rule-coverage registration |
| web | typed WASM client, `HighCardDuelBoard`, browser no-leak smoke |
| benchmarks | [BENCHMARKS.md](BENCHMARKS.md) |

## Admission Constraints

- No `engine-core` card/deck/hand nouns.
- No `game-stdlib` card promotion.
- No YAML, DSL, or procedural static rule data.
- No MCTS, ISMCTS, Monte Carlo, ML, RL, or runtime LLM bot.
- Public replay export defaults to no-leak observer projection.
