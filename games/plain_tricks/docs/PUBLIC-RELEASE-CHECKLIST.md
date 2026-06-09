# Plain Tricks Public Release Checklist

Game ID: `plain_tricks`

Public display name: `Plain Tricks`

Implemented variant: `plain_tricks_standard`

Rules version: `plain-tricks-rules-v1`

Last updated: 2026-06-09

## Release Checklist

| Item | Status | Evidence |
|---|---|---|
| Rules source and coverage docs complete | complete | [RULES.md](RULES.md), [SOURCES.md](SOURCES.md), [RULE-COVERAGE.md](RULE-COVERAGE.md) |
| Mechanic inventory complete | complete | [MECHANICS.md](MECHANICS.md) |
| Native tests pass | complete | `cargo test -p plain_tricks` |
| Tooling registered | complete | simulate, replay-check, fixture-check, rule-coverage |
| Native benchmark evidence complete | complete | [BENCHMARKS.md](BENCHMARKS.md), `cargo bench -p plain_tricks` |
| WASM registered | complete | `cargo test -p wasm-api`, `npm --prefix apps/web run smoke:wasm` |
| UI integrated | complete | [UI.md](UI.md), `PlainTricksBoard`, `npm --prefix apps/web run build`, `npm --prefix apps/web run smoke:ui` |
| Browser no-leak/a11y smoke | complete | `node apps/web/e2e/plain-tricks.smoke.mjs`, `npm --prefix apps/web run smoke:e2e` |
| Reduced-motion path | complete | Plain Tricks browser smoke |
| Replay export/import safe | complete | golden traces, WASM export fixture, browser replay import/step |
| Bot evidence complete | complete | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| Public copy and assets original | complete | [SOURCES.md](SOURCES.md); no copied art, icons, fonts, scans, screenshots, component text, or trade dress |
| Public display is neutral | complete | Public UI/docs use Plain Tricks, Gale/River/Ember, hand, trick, led suit, and trick totals |
| Boundary checks pass | complete | `bash scripts/boundary-check.sh` |
| Catalog docs reconciled | complete | `node scripts/check-catalog-docs.mjs` |
| Primitive-pressure closeout tracked | pending capstone | [MECHANICS.md](MECHANICS.md), [../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md), GAT101PLATRI-020 |

## IP And Trade-Dress Review

| Check | Status | Evidence |
|---|---|---|
| Public rules prose is original Rulepath prose | pass | [RULES.md](RULES.md), [SOURCES.md](SOURCES.md) |
| Sources are recorded with quality and use limits | pass | [SOURCES.md](SOURCES.md) |
| No copied rulebook prose, examples, names, or scoring tables | pass | [SOURCES.md](SOURCES.md) |
| No copied component text, icons, art, screenshots, scans, or fonts | pass | Source review; no bundled game art or font files |
| Public name avoids proprietary game framing | pass | `Plain Tricks` is neutral original display copy |
| Public surface avoids copied trade dress | pass | Neutral board-game renderer with text/count/card surfaces |
| Human/legal review trigger unresolved | not applicable | No copied assets, branding, or private licensed material introduced |

## Hidden-Information No-Leak Surfaces

| Surface | Status | Evidence |
|---|---|---|
| Rust public/seat views | pass | `cargo test -p plain_tricks`, `cargo test -p wasm-api` |
| action tree | pass | actor-only WASM authorization tests and `plain-tricks.smoke.mjs` |
| diagnostics and stale submissions | pass | WASM tests and browser stale diagnostic path |
| effect logs | pass | `cargo test -p wasm-api`, replay golden traces |
| DOM text and attributes | pass | `node apps/web/e2e/plain-tricks.smoke.mjs` |
| test IDs | pass | `choice-plain-tricks-trick-${trick}-${index}` smoke assertions |
| browser console and storage | pass | `node apps/web/e2e/plain-tricks.smoke.mjs` |
| replay export/import | pass | `wasm-exported.trace.json`, public export/import branch, browser replay import/step |
| bot explanations | pass | public bot effect exposes policy/action family only |
| dev inspector | pass | viewer-filtered dev-panel classification and browser no-leak smoke |

## Legal-Only UI

| Check | Status | Evidence |
|---|---|---|
| TypeScript does not decide legality | pass | `PlainTricksBoard` maps the Rust `play` node and card leaves |
| UI controls derive from Rust action tree | pass | [UI.md](UI.md), browser smoke |
| stale/invalid submissions return safe diagnostics | pass | `plain-tricks.smoke.mjs`, WASM tests |
| no raw command editing in public mode | pass | Shell action controls and replay import/export only |
| semantic effects drive animation/feedback | pass | `effectFeedback.ts`, `PlainTricksBoard`, smoke evidence |

## Release Decision

Decision: release with explicit constraints.

Plain Tricks is public-preview safe for the Gate 10.1 trick-taking proof after
the documented native, WASM, web, replay, no-leak, benchmark, catalog, boundary,
and documentation checks. Primitive-pressure atlas/status reconciliation remains
owned by GAT101PLATRI-020.

## Release Blockers

No known Plain Tricks public-surface blocker remains in this checklist scope.
GAT101PLATRI-020 owns final capstone reconciliation and archive/status closeout.
