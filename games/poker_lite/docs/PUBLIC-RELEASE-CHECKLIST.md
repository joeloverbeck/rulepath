# Crest Ledger Public Release Checklist

Game ID: `poker_lite`

Public display name: `Crest Ledger`

Implemented variant: `poker_lite_standard`

Rules version: `poker-lite-rules-v1`

Last updated: 2026-06-09

## Release Checklist

| Item | Status | Evidence |
|---|---|---|
| Rules source and coverage docs complete | complete | [RULES.md](RULES.md), [SOURCES.md](SOURCES.md), [RULE-COVERAGE.md](RULE-COVERAGE.md) |
| Mechanic inventory complete | complete | [MECHANICS.md](MECHANICS.md) |
| Native tests pass | complete | `cargo test -p poker_lite` |
| Tooling registered | complete | simulate, replay-check, fixture-check, rule-coverage |
| Native benchmark evidence complete | complete | [BENCHMARKS.md](BENCHMARKS.md), `cargo bench -p poker_lite` |
| WASM registered | complete | `cargo test -p wasm-api`, `npm --prefix apps/web run smoke:wasm` |
| UI integrated | complete | [UI.md](UI.md), `PokerLiteBoard`, `npm --prefix apps/web run build`, `npm --prefix apps/web run smoke:ui` |
| Browser no-leak/a11y smoke | complete | `node apps/web/e2e/poker-lite.smoke.mjs`, `npm --prefix apps/web run smoke:e2e` |
| Reduced-motion path | complete | Crest Ledger browser smoke |
| Replay export/import safe | complete | golden traces, WASM export fixture, browser replay import/step |
| Bot evidence complete | complete | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), `cargo test -p poker_lite --test bots` |
| Public copy and assets original | complete | [SOURCES.md](SOURCES.md); no copied art, icons, fonts, scans, screenshots, component text, or trade dress |
| Public display is neutral | complete | Public UI/docs use Crest Ledger, crest, marker, pledge, shared pool, hold, press, lift, match, and yield |
| Boundary checks pass | complete | `bash scripts/boundary-check.sh` |
| Catalog docs reconciled | complete | `node scripts/check-catalog-docs.mjs` |
| Primitive-pressure closeout complete | complete | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md), [../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md), GAT10POKLITBET-018 archive |

## IP And Trade-Dress Review

| Check | Status | Evidence |
|---|---|---|
| Public rules prose is original Rulepath prose | pass | [RULES.md](RULES.md), [SOURCES.md](SOURCES.md) |
| Sources are recorded with quality and use limits | pass | [SOURCES.md](SOURCES.md) |
| No copied rulebook prose, examples, or hand-ranking table | pass | [SOURCES.md](SOURCES.md) |
| No copied component text, icons, art, screenshots, scans, or fonts | pass | Source review; no bundled game art or font files |
| Public name avoids proprietary or casino-product framing | pass | `Crest Ledger` display name; internal `poker_lite` remains an id |
| Public surface avoids casino trade dress | pass | Neutral board-game renderer, abstract crests/markers, no real-money framing |
| Human/legal review trigger unresolved | not applicable | No copied assets, branding, or private licensed material introduced |

## Hidden-Information No-Leak Surfaces

| Surface | Status | Evidence |
|---|---|---|
| Rust public/seat views | pass | `cargo test -p poker_lite --test visibility` |
| action tree | pass | `cargo test -p poker_lite --test rules`, `cargo test -p wasm-api` |
| diagnostics and stale submissions | pass | invalid golden traces, `poker-lite.smoke.mjs` stale diagnostic path |
| effect logs | pass | `cargo test -p poker_lite`, replay golden traces |
| DOM text and attributes | pass | `node apps/web/e2e/poker-lite.smoke.mjs` |
| test IDs | pass | `choice-poker-lite-round-${round}-${index}` smoke assertions |
| browser console and storage | pass | `node apps/web/e2e/poker-lite.smoke.mjs` |
| replay export/import | pass | `wasm-exported.trace.json`, `public-replay-export-import.trace.json`, browser replay import/step |
| bot explanations | pass | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), `cargo test -p poker_lite --test bots` |
| dev inspector | pass | viewer-filtered dev-panel classification and browser no-leak smoke |

## Legal-Only UI

| Check | Status | Evidence |
|---|---|---|
| TypeScript does not decide legality | pass | `PokerLiteBoard` maps `actionTree.choices`; Rust owns validation |
| UI controls derive from Rust action tree | pass | [UI.md](UI.md), browser smoke |
| stale/invalid submissions return safe diagnostics | pass | `poker-lite.smoke.mjs`, invalid golden traces |
| no raw command editing in public mode | pass | Shell action controls and replay import/export only |
| semantic effects drive animation/feedback | pass | `effectFeedback.ts`, `PokerLiteBoard`, smoke evidence |

## Release Decision

Decision: release with explicit constraints.

Crest Ledger is public-preview safe for the Gate 10 betting/showdown proof after
the documented native, WASM, web, replay, no-leak, benchmark, catalog, boundary,
primitive-pressure, atlas, progress, spec-index, and spec-archive checks.

## Release Blockers

No known Crest Ledger public-surface blocker remains in the mechanics/UI/public
release checklist scope. The `poker_lite` betting/showdown gate closeout
bookkeeping is complete; `plain_tricks` remains the broader Gate 10 successor.
