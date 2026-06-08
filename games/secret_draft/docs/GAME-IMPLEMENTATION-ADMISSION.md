# Veiled Draft Implementation Admission

Game ID: `secret_draft`

Rules version: `secret-draft-rules-v1`

Last updated: 2026-06-08

## Admission Summary

Veiled Draft is admitted as the Gate 9.1 simultaneous commitment/reveal proof
game after the capstone evidence passes. The implementation is game-local Rust
with a Rust/WASM browser presentation. It proves hidden per-seat commitments,
public pending-seat state, synchronized reveal, deterministic conflict fallback,
visible draft-pool removal, public scoring, viewer-scoped replay export, Level 1
public bot policy, and a browser no-leak board.

## Required Evidence

| Area | Evidence |
|---|---|
| rules and validation | [RULES.md](RULES.md), [RULE-COVERAGE.md](RULE-COVERAGE.md), `cargo test -p secret_draft` |
| source/IP review | [SOURCES.md](SOURCES.md) |
| mechanic inventory | [MECHANICS.md](MECHANICS.md) |
| static data and fixtures | `games/secret_draft/data/*`, `cargo run -p fixture-check -- --game secret_draft` |
| replay/golden traces | `games/secret_draft/tests/golden_traces/*`, `cargo run -p replay-check -- --game secret_draft --all` |
| simulation | `cargo run -p simulate -- --game secret_draft --games 1000` |
| rule coverage | `cargo run -p rule-coverage -- --game secret_draft` |
| bot policy | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), `cargo test -p secret_draft --test bots` |
| WASM registration | `cargo test -p wasm-api`, `npm --prefix apps/web run smoke:wasm` |
| web board and no-leak/a11y | [UI.md](UI.md), `SecretDraftBoard.tsx`, `node apps/web/e2e/secret-draft.smoke.mjs` |
| benchmarks | [BENCHMARKS.md](BENCHMARKS.md) |

## Admission Constraints

- No `engine-core` draft, commit, reveal, pool, item, tile, or scoring nouns.
- No `game-stdlib` commitment/reveal/drafting primitive is promoted in Gate 9.1.
- No YAML, DSL, selectors, formulas, or behavior-in-data.
- No TypeScript legality, commitment, reveal timing, conflict fallback, scoring,
  terminal, winner, replay authority, or bot policy.
- No pre-reveal committed item id in public/seat browser payloads, DOM,
  attributes, test IDs, storage, logs, dev panel, bot rationale, or
  viewer-scoped replay export.
- No MCTS, ISMCTS, Monte Carlo, ML, RL, runtime LLM bot, hidden-state sampling,
  or opponent-commit peeking.

## Admission Result

Admitted for Gate 9.1 after the full ticket family lands and the capstone
status/index/atlas update is complete.
