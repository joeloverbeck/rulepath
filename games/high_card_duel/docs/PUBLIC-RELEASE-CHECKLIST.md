# High Card Duel Public Release Checklist

Game ID: `high_card_duel`

Last updated: 2026-06-07

## Release Checklist

| Item | Status | Evidence |
|---|---|---|
| Rules source and coverage docs complete | complete | [RULES.md](RULES.md), [SOURCES.md](SOURCES.md), [RULE-COVERAGE.md](RULE-COVERAGE.md) |
| Native tests pass | complete | `cargo test -p high_card_duel` |
| Tooling registered | complete | simulate, replay-check, fixture-check, rule-coverage |
| WASM registered | complete | `cargo test -p wasm-api`, web WASM smoke |
| UI integrated | complete | `HighCardDuelBoard`, `npm --prefix apps/web run smoke:ui` |
| Browser no-leak smoke | complete | `node apps/web/e2e/high-card-duel.smoke.mjs` |
| Reduced-motion path | complete | HCD browser smoke |
| Replay export default safe | complete | public observer projection before hidden command/seed export |
| Benchmark evidence | complete | [BENCHMARKS.md](BENCHMARKS.md) |
| Public copy avoids casino/poker/blackjack trade dress | complete | neutral duel-table theme |

## Release Blockers

No known Gate 8 blocker remains for High Card Duel. Future Blackjack Lite scope is tracked outside this per-game checklist.
