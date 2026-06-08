# Token Bazaar Implementation Admission

Game ID: `token_bazaar`

Rules version: `token-bazaar-rules-v1`

Last updated: 2026-06-08

## Admission Summary

Token Bazaar is admitted as the Gate 9 public resource/economy proof game. The implementation is game-local Rust with a Rust/WASM browser presentation. It proves public resource accounting, exact payments, supply return, deterministic market refill, terminal tie-breaks, replayable accounting effects, Level 1 public bot policy, and a browser economy board.

## Required Evidence

| Area | Evidence |
|---|---|
| rules and validation | [RULES.md](RULES.md), [RULE-COVERAGE.md](RULE-COVERAGE.md), `cargo test -p token_bazaar` |
| source/IP review | [SOURCES.md](SOURCES.md) |
| mechanic inventory | [MECHANICS.md](MECHANICS.md) |
| static data and fixtures | `games/token_bazaar/data/*`, `cargo run -p fixture-check -- --game token_bazaar` |
| replay/golden traces | `games/token_bazaar/tests/golden_traces/*`, `cargo run -p replay-check -- --game token_bazaar --all` |
| simulation | `cargo run -p simulate -- --game token_bazaar --games 1000` |
| rule coverage | `cargo run -p rule-coverage -- --game token_bazaar` |
| bot policy | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), `cargo test -p token_bazaar --test bots` |
| WASM registration | `cargo test -p wasm-api`, `npm --prefix apps/web run smoke:wasm` |
| web board and no-leak/a11y | [UI.md](UI.md), `TokenBazaarBoard.tsx`, `node apps/web/e2e/token-bazaar.smoke.mjs` |
| benchmarks | [BENCHMARKS.md](BENCHMARKS.md) |

## Admission Constraints

- No `engine-core` resource, market, supply, contract, token, payment, or economy nouns.
- No `game-stdlib` economy primitive is promoted in Gate 9.
- No YAML, DSL, selectors, formulas, or behavior-in-data.
- No TypeScript legality, affordability, refill, terminal, winner, or bot policy.
- No MCTS, ISMCTS, Monte Carlo, ML, RL, or runtime LLM bot.
- No hidden/debug/candidate/internal data in DOM, storage, logs, replay exports, dev panel, or bot rationale.

## Admission Result

Admitted for Gate 9 after the full ticket family lands and the capstone status/index update is complete.
