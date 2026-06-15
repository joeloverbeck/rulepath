# River Ledger Public Release Checklist

Game ID: `river_ledger`

Public display name: `River Ledger`

Implemented variant: `river_ledger_standard`

Rules version: `river-ledger-rules-v1`

Last updated: 2026-06-14

## Release Checklist

| Item | Status | Evidence |
|---|---|---|
| Rules source and coverage docs complete | complete | [RULES.md](RULES.md), [SOURCES.md](SOURCES.md), [RULE-COVERAGE.md](RULE-COVERAGE.md) |
| Player-facing public rules complete | complete | [HOW-TO-PLAY.md](HOW-TO-PLAY.md), `apps/web/public/rules/river_ledger.md`, `node scripts/check-player-rules.mjs` |
| Mechanic inventory complete | complete | [MECHANICS.md](MECHANICS.md) |
| Native tests pass | complete | `cargo test -p river_ledger` |
| Tooling registered | complete | simulate, replay-check, fixture-check, rule-coverage, `ci/games.json` |
| Native simulation evidence complete | complete | `cargo run -p simulate -- --game river_ledger --seat-count 6 --games 1000 --start-seed 1506 --action-cap 48` |
| Replay and fixture evidence complete | complete | `cargo run -p replay-check -- --game river_ledger --all`, `cargo run -p fixture-check -- --game river_ledger` |
| Native benchmark evidence complete | complete | [BENCHMARKS.md](BENCHMARKS.md), `cargo bench -p river_ledger` |
| Bots complete | complete | [AI.md](AI.md), [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), `cargo test -p river_ledger --test bots` |
| WASM registered | complete | `cargo test -p wasm-api`, `npm --prefix apps/web run smoke:wasm` |
| UI integrated | complete | [UI.md](UI.md), `RiverLedgerBoard`, `npm --prefix apps/web run build`, `npm --prefix apps/web run smoke:ui` |
| Browser no-leak/a11y smoke | complete | `node apps/web/e2e/river-ledger.smoke.mjs`, `npm --prefix apps/web run smoke:e2e` |
| Reduced-motion path | complete | shared shell smoke and River Ledger browser smoke |
| Replay export/import safe | complete | golden traces, WASM replay dispatch, browser replay shell |
| Public copy and assets original | complete | [SOURCES.md](SOURCES.md); no copied art, icons, fonts, scans, screenshots, component text, or trade dress |
| Public display is neutral | complete | River Ledger uses abstract contribution units, neutral cards/ledger language, and no real-money framing |
| Boundary checks pass | complete | `bash scripts/boundary-check.sh` |
| Catalog docs reconciled | complete | `README.md`, `apps/web/README.md`, `node scripts/check-catalog-docs.mjs` |
| Primitive-pressure closeout complete | complete | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md), [../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md), GAT15RIVLEDTEX-020 archive |
| Final acceptance/spec closeout complete | pending | GAT15RIVLEDTEX-021 final sweep and spec/index archive |

## IP And Trade-Dress Review

| Check | Status | Evidence |
|---|---|---|
| Public rules prose is original Rulepath prose | pass | [RULES.md](RULES.md), [HOW-TO-PLAY.md](HOW-TO-PLAY.md), [SOURCES.md](SOURCES.md) |
| Sources are recorded with quality and use limits | pass | [SOURCES.md](SOURCES.md) |
| No copied rulebook prose, examples, or hand-ranking table | pass | [SOURCES.md](SOURCES.md), original Rulepath docs |
| No copied component text, icons, art, screenshots, scans, or fonts | pass | Source review; renderer uses project-authored shapes and system fonts |
| Public name avoids proprietary or casino-product framing | pass | `River Ledger` is the product-facing name; `river_ledger` is the internal id |
| Public surface avoids casino trade dress | pass | Neutral board-game renderer, abstract contribution units, no cash/chip/rake language |
| Human/legal review trigger unresolved | not applicable | No copied assets, branding, close-source wording, or private licensed material introduced |

## Hidden-Information No-Leak Surfaces

| Surface | Status | Evidence |
|---|---|---|
| Rust public/seat views | pass | `cargo test -p river_ledger --test visibility`, pairwise 3-6 seat no-leak tests |
| action tree | pass | `cargo test -p river_ledger`, `cargo test -p wasm-api` |
| diagnostics and stale submissions | pass | wrong-seat and cap golden traces, browser smoke diagnostic checks |
| effect logs | pass | Rust effect filtering tests, replay golden traces |
| DOM text and attributes | pass | `node apps/web/e2e/river-ledger.smoke.mjs` |
| test IDs | pass | River Ledger browser smoke checks no private card ids in selectors |
| browser console and storage | pass | `node apps/web/e2e/river-ledger.smoke.mjs` |
| replay export/import | pass | `public-replay-export-import.trace.json`, WASM replay dispatch, browser replay shell |
| bot explanations | pass | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), `cargo test -p river_ledger --test bots` |
| dev inspector | pass | viewer-filtered public summary only; browser no-leak smoke |

## Legal-Only UI

| Check | Status | Evidence |
|---|---|---|
| TypeScript does not decide legality | pass | `RiverLedgerBoard` maps `actionTree.choices`; Rust owns validation |
| UI controls derive from Rust action tree | pass | [UI.md](UI.md), `node apps/web/e2e/river-ledger.smoke.mjs` |
| stale/invalid submissions return safe diagnostics | pass | Rust validation traces and WASM dispatch tests |
| no raw command editing in public mode | pass | Shell action controls and replay import/export only |
| semantic effects drive animation/feedback | pass | shared effect feedback, River Ledger effect payloads, smoke evidence |
| seat counts come from catalog | pass | setup shell consumes `supportedSeatCounts` and `defaultSeats` |

## Replay Export/Import

| Check | Status | Evidence |
|---|---|---|
| public export is viewer-scoped | pass | replay support tests and golden trace coverage |
| import goes through Rust parser/projector | pass | WASM replay dispatch and replay viewer smoke |
| browser shell does not repair replay data | pass | [../../../docs/WASM-CLIENT-BOUNDARY.md](../../../docs/WASM-CLIENT-BOUNDARY.md), replay UI code |
| hidden setup state is excluded from public exports | pass | `RL-REPLAY-EXPORT-001`, `RL-VIS-DECKTAIL-001`, no-leak tests |

## Bot Boundary

| Check | Status | Evidence |
|---|---|---|
| bots choose from legal Rust actions only | pass | [AI.md](AI.md), `cargo test -p river_ledger --test bots` |
| Level 2 uses authored allowed inputs only | pass | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| bot explanations are viewer-safe | pass | bot tests, evidence pack, no-leak checks |
| forbidden AI classes absent | pass | No MCTS, ISMCTS, Monte Carlo, ML, RL, solvers, hidden-state sampling, or runtime LLM policy |

## Release Decision

Decision: release-ready for the trailing-doc scope, with final gate closeout
pending.

River Ledger has the required official-game docs, Rust implementation evidence,
tool registration, WASM registration, public rules copy, browser renderer, e2e
no-leak smoke, catalog docs, bot-boundary proof, primitive-pressure closeout,
replay evidence, and neutral original presentation for public preview. The
final acceptance/spec closeout remains in GAT15RIVLEDTEX-021.

## Release Blockers

| Blocker | Owner | Blocks public preview? |
|---|---|---:|
| Final acceptance sweep and spec/index closeout. | GAT15RIVLEDTEX-021 | no; blocks complete Gate 15 archival |

No known IP, no-leak, catalog, e2e, presentation-copy, smoke, replay
export/import, or bot-boundary blocker remains for this ticket.
