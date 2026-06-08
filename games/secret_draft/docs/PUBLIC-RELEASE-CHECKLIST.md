# Veiled Draft Public Release Checklist

Game ID: `secret_draft`

Last updated: 2026-06-08

## Release Checklist

| Item | Status | Evidence |
|---|---|---|
| Rules source and coverage docs complete | complete | [RULES.md](RULES.md), [SOURCES.md](SOURCES.md), [RULE-COVERAGE.md](RULE-COVERAGE.md) |
| Mechanic inventory complete | complete | [MECHANICS.md](MECHANICS.md) |
| Native tests pass | complete | `cargo test -p secret_draft` |
| Tooling registered | complete | simulate, replay-check, fixture-check, rule-coverage |
| WASM registered | complete | `cargo test -p wasm-api`, `npm --prefix apps/web run smoke:wasm` |
| UI integrated | complete | [UI.md](UI.md), `SecretDraftBoard`, `npm --prefix apps/web run smoke:ui` |
| Browser no-leak/a11y smoke | complete | `node apps/web/e2e/secret-draft.smoke.mjs` |
| Reduced-motion path | complete | Secret Draft browser smoke |
| Replay export/import safe | complete | golden traces, WASM export fixture, browser replay import/step |
| Bot evidence complete | complete | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), `cargo test -p secret_draft --test bots` |
| Benchmark evidence complete | complete | [BENCHMARKS.md](BENCHMARKS.md) |
| Public copy and assets original | complete | [SOURCES.md](SOURCES.md); no copied art, icons, fonts, scans, screenshots, component text, or trade dress |
| Boundary checks pass | complete | `bash scripts/boundary-check.sh` |

## Release Blockers

No known Gate 9.1 Veiled Draft public-release blocker remains after the
capstone evidence/status/index/atlas update.
