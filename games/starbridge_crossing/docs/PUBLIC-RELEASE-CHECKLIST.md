# Starbridge Crossing Public Release Checklist

Game ID: `starbridge_crossing`

Public display name: `Starbridge Crossing`

Implemented variant: `starbridge_crossing_classic_star_v1`

Release target: public web build / portfolio demo

Rules version: `starbridge-crossing-rules-v1`

Data/manifest version: `starbridge-crossing-data-v1`

Engine version: `engine-core-0.1.0`

Date: 2026-06-27

## Public Shipment Rule

If content ships to an unauthorized browser, it has shipped. Starbridge
Crossing has no hidden game facts, but public files, browser payloads, DOM,
storage, logs, exports, traces, tests, and bot explanations still must not
expose debug-only state, private framework fields, candidate rankings, or
TypeScript legality decisions.

## Official-Game Contract Status

| Requirement | Status | Evidence/notes |
|---|---|---|
| sources complete | pass with pending human review | [SOURCES.md](SOURCES.md) records consulted sources and non-copying posture. |
| rules complete with stable IDs | pass | [RULES.md](RULES.md) |
| rule coverage complete | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md); `rule-coverage` passes. |
| mechanics inventory complete | pass | [MECHANICS.md](MECHANICS.md) |
| implementation admission complete | pass | [GAME-IMPLEMENTATION-ADMISSION.md](GAME-IMPLEMENTATION-ADMISSION.md) |
| competent-player analysis | pass with constraint | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md); no higher bot admission. |
| bot strategy evidence pack | not applicable for shipped bot | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) records deferred L1/L2. |
| AI registry complete | pass | [AI.md](AI.md) |
| UI doc complete | pass | [UI.md](UI.md) |
| benchmarks complete | pass | [BENCHMARKS.md](BENCHMARKS.md), `cargo bench -p starbridge_crossing`. |
| primitive-pressure ledger complete | pass | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md); topology/jump helpers stay game-local. |
| supported player-count smoke | pass | 2, 3, 4, and 6 seat fixtures and simulation lanes. |
| pairwise no-leak matrix | pass | Rust visibility tests, WASM tests, and Starbridge e2e canary scan. |
| per-seat outcome explanation | pass | [UI.md](UI.md), `check-outcome-explanations.mjs`. |
| scaffolding governance receipt | pass | GAT20STACROSTA-019 added `ci/scaffolding-audits.json` and the register receipt. |

## Rule, Source, And IP Status

| Check | Status | Evidence/notes |
|---|---|---|
| public rules prose is original | pass | [RULES.md](RULES.md) |
| sources recorded with dates and quality | pass | [SOURCES.md](SOURCES.md) |
| variant and deviations clear | pass | Classic star-board race; excluded variants are listed. |
| no copied rulebook prose | pass | source-use statement records consulted-not-copied posture |
| no copied icons/art/screenshots/fonts/trade dress | pass | original web presentation and catalog icon |
| public name risk reviewed | pending human | Bounded screening is recorded, but not legal clearance. |
| private licensed content excluded | pass | no private content involved |

## No-Leak Surfaces

| Surface | Status | Evidence/test | Notes |
|---|---|---|---|
| public/browser payload | pass | WASM and e2e smoke | All game facts are public. |
| public view | pass | visibility tests | Full board, occupancy, ranks, terminal reason. |
| seat view | not applicable | seat-viewer parity trace | Seat viewers receive the same public facts. |
| action tree | pass | action tests and e2e | Legal paths from Rust only. |
| diagnostics | pass | rule tests | Stable viewer-safe diagnostic codes. |
| effect logs | pass | effect tests and smoke | Step/jump/finish/pass/terminal facts are public. |
| DOM/test IDs | pass | `starbridge-crossing.smoke.mjs` canary scan | No hidden/debug tokens. |
| console/storage | pass | `starbridge-crossing.smoke.mjs` | No forbidden hidden/private/candidate terms. |
| replay export/import | pass | replay tests and e2e | All-public export. |
| bot explanations | pass | bot tests and [AI.md](AI.md) | L0 names legal-choice count only. |
| candidate rankings | not applicable | no public candidate ranking | Future strategy bot must add tests. |

## UI Polish, Legal-Only, Accessibility

| Check | Status | Evidence/notes |
|---|---|---|
| play-first public screen | pass | `StarbridgeCrossingBoard.tsx` |
| visual target neutral/original | pass | abstract star-board presentation |
| legal moves derive from Rust action tree | pass | peg/step/jump/stop/continue controls keyed by Rust leaves |
| TypeScript does not decide legality | pass | UI maps action paths and metadata only |
| score/rank facts derive from Rust | pass | finish ranks and terminal reason are projected fields |
| keyboard path for core actions | pass | e2e focus/Enter path smoke |
| visible/accessibility labels | pass | board-space and control labels |
| reduced motion | pass | e2e reduced-motion check |
| responsive layout | pass | e2e 390px layout check |

## Bot Safety

| Check | Status | Evidence/notes |
|---|---|---|
| public bot uses legal action API | pass | [AI.md](AI.md), bot tests |
| bot does not access forbidden hidden information | pass | no hidden facts; dev fields excluded |
| L1/L2 evidence pack complete if higher bot ships | not applicable | higher bots not admitted |
| no MCTS/ISMCTS/Monte Carlo/ML/RL | pass | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| bot latency acceptable | pass | [BENCHMARKS.md](BENCHMARKS.md) |

## Release Blockers

| Blocker | Owner | Status |
|---|---|---|
| human public-name/IP review | maintainer | pending human review |
| scaffolding governance receipt | GAT20STACROSTA-019 | complete |
| final spec closeout | GAT20STACROSTA-020 | complete |
