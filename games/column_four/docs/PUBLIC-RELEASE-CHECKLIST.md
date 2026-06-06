# Column Four Public Release Checklist

Game ID: `column_four`

Public display name: `Column Four`

Implemented variant: `column_four_standard`

Release target: public web build / portfolio demo candidate

Rules version: `column_four-rules-v1`

Data/manifest version: `1`

Engine version: `engine-core-0.1.0`

Date: 2026-06-06

## Public Shipment Rule

If content ships to an unauthorized browser, it has shipped. Column Four's public release review therefore covers source files, browser payloads, replay exports, DOM attributes, storage, console output, UI copy, and screenshots/demo recordings made from the app.

## Official-Game Contract Status

| Requirement | Status | Evidence/notes |
|---|---|---|
| `GAME-SOURCES.md` complete | pass | [SOURCES.md](SOURCES.md) |
| `GAME-RULES.md` complete with stable rule IDs | pass | [RULES.md](RULES.md) |
| `GAME-RULE-COVERAGE.md` complete | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md) |
| `GAME-MECHANICS.md` complete | pass | [MECHANICS.md](MECHANICS.md) |
| `GAME-IMPLEMENTATION-ADMISSION.md` complete | pass | [GAME-IMPLEMENTATION-ADMISSION.md](GAME-IMPLEMENTATION-ADMISSION.md) |
| `COMPETENT-PLAYER.md` complete | pass | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) |
| `BOT-STRATEGY-EVIDENCE-PACK.md` complete | pass | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| `GAME-AI.md` complete | pass | [AI.md](AI.md) |
| `GAME-UI.md` complete | pass | [UI.md](UI.md) |
| `GAME-BENCHMARKS.md` complete | pass | [BENCHMARKS.md](BENCHMARKS.md) |
| primitive-pressure ledger complete if needed | not applicable | atlas/status update queued in GAT5COLFOUPUB-018 |

## Rule, Source, And IP Status

| Check | Status | Evidence/notes |
|---|---|---|
| public rules prose is original Rulepath prose | pass | [RULES.md](RULES.md) |
| sources are recorded with dates and quality | pass | [SOURCES.md](SOURCES.md) |
| variant and deviations are clear | pass | standard 7 by 6, two seats, four-in-row |
| no copied rulebook prose | pass | original docs |
| no copied card/component text | not applicable | no cards or text components |
| no copied icons, board art, screenshots, scans, or trade dress | pass | CSS/SVG board is project-authored and neutral |
| public name/trademark/trade-dress risk reviewed | pass | neutral `Column Four`; no proprietary name or palette |
| generated assets reviewed | not applicable | no generated assets |
| fonts are system-only or license-reviewed | pass | app uses system font stack |
| human/legal review triggers resolved | constrained | required before public hosted release if project policy requires legal review |

## Bundle/Public Artifact Inspection

| Surface/artifact | Inspected? | Unsafe content found? | Notes/action |
|---|---:|---:|---|
| production JS/WASM bundle | yes | no | `npm --prefix apps/web run build`; no external game assets |
| static assets | yes | no | `public/wasm_api.wasm` and Vite bundle only |
| source maps if shipped | not shipped | no | default production build output checked by smoke |
| public routes | yes | no | static `/rulepath/` smoke path |
| local storage defaults | yes | no | only reduced-motion preference allowed |
| replay examples/export samples | yes | no | no hidden payloads; private-view hash explicitly not applicable |
| console logs/diagnostics | yes | no | no-leak smoke captures console/page errors |
| test IDs/DOM attributes | yes | no | public column ids/generic hooks only |
| dev inspector disabled/redacted in public build | yes | no | secondary panel with public metadata only |

## Hidden-Information No-Leak Surfaces

| Surface | Status | Evidence/test | Notes |
|---|---|---|---|
| public/browser payload | pass | `smoke:wasm` | perfect-information public view only |
| public view | pass | visibility tests; WASM smoke | hidden fields empty |
| action tree | pass | Rust action tree tests; UI smoke | legal columns only |
| previews | pass | `column-four.smoke.mjs` | Rust landing preview only |
| diagnostics/disabled reasons | pass | stale/full-column traces; WASM smoke | safe codes/messages |
| effect logs | pass | replay traces; E2E smoke | viewer-safe semantic effects |
| command logs | pass | replay export/import smoke | public command paths |
| DOM attributes | pass | `column-four.smoke.mjs` | no forbidden leak vocabulary |
| test IDs | pass | `column-four.smoke.mjs` | public `c1` through `c7` controls |
| browser console/logs | pass | `column-four.smoke.mjs` | console/page errors scanned |
| local storage/session storage | pass | `a11y-noleak.smoke.mjs`; Column Four smoke | local reduced-motion only, session empty |
| replay export/import | pass | `column-four.smoke.mjs`; replay-check | no hidden state payload |
| bot explanations | pass | bot tests; Column Four smoke | public rationale only |
| candidate rankings | pass | bot tests/no-leak scan | not public |
| dev inspector/public build boundary | pass | shell/a11y smoke | viewer-safe metadata only |

## Replay/Export Safety

| Check | Status | Evidence/notes |
|---|---|---|
| replay reproduces deterministic hashes | pass | `cargo run -p replay-check -- --game column_four --all` |
| exported replay contains only intended public data | pass | WASM smoke and browser replay smoke |
| hidden-info redaction verified for exports | not applicable | perfect-information game; private view explicitly not applicable |
| replay import validates versions and schema | pass | replay support tests and smoke |
| replay UI is viewer-safe | pass | `ReplayViewer.tsx` uses Rust projection |
| golden traces are not silently updated | pass | trace hashes checked by replay-check |

## UI Polish And Legal-Only UI

| Check | Status | Evidence/notes |
|---|---|---|
| default public screen is play-first | pass | shell smoke and Column Four smoke |
| visual target is polished and neutral | pass | [UI.md](UI.md), `ColumnFourBoard.tsx` |
| no proprietary mimicry/trade dress | pass | neutral name/colors/shapes |
| legal moves are obvious | pass | seven column controls from Rust legal targets |
| player feedback is clear after every action | pass | semantic effects/status/bot rationale |
| semantic effects drive animations | pass | landed-piece class from Rust `piece_landed` effect log |
| animations settle to Rust public view | pass | board renders latest Rust view |
| React + SVG default preserved | pass | no Canvas/PixiJS/WebGL |
| TypeScript does not decide legality | pass | controls derive from Rust legal targets |
| stale/invalid submissions return safe diagnostics | pass | WASM smoke and traces |
| no raw command editing in public mode | pass | replay textarea imports full documents only |

## Accessibility, Reduced-Motion, And Responsive Checks

| Check | Status | Evidence/notes |
|---|---|---|
| keyboard path for core actions | pass | `column-four.smoke.mjs` focuses a column and presses Enter |
| visible focus indicators | pass | E2E focus assertion |
| accessible names/labels for controls | pass | seven controls checked for names |
| screen-reader summaries where practical | pass | live board summary/status text |
| contrast reviewed | pass | neutral high-contrast CSS; release human review still useful |
| color is not sole information channel | pass | seat shapes, labels, terminal text, winning class |
| reduced-motion behavior implemented | pass | Column Four smoke checks animation disabled |
| responsive layout smoke-tested | pass | shell no-leak/a11y narrow viewport; board uses constrained SVG |
| accessibility scan where practical | constrained | smoke coverage exists; external audit can be added later |

## Bot Explanation Safety

| Check | Status | Evidence/notes |
|---|---|---|
| public bot uses legal action API | pass | [AI.md](AI.md), bot tests |
| bot does not access forbidden hidden information | pass | perfect-information public board only |
| Level 2 evidence pack complete | pass | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| no public v1/v2 MCTS/ISMCTS/Monte Carlo/ML/RL | pass | [AI.md](AI.md) |
| public explanations are viewer-safe | pass | bot tests and browser no-leak smoke |
| candidate rankings are dev-only/redacted | pass | not exposed publicly |
| bot latency acceptable | pass | [BENCHMARKS.md](BENCHMARKS.md) |
| public default bot suitability recorded | pass | [AI.md](AI.md) |

## Tests, Traces, Simulations, Replay, Serialization, And Benchmarks

| Evidence | Status | Notes |
|---|---|---|
| unit/named rule tests | pass | `cargo test -p column_four` |
| golden traces | pass | 11 trace files; replay-check |
| invalid/stale diagnostic traces | pass | full-column, invalid-column, stale diagnostics |
| property/invariant tests | pass | `games/column_four/tests/property.rs` |
| simulation/fuzz runs | pass | `cargo run -p simulate -- --game column_four --games 1000` |
| replay/hash tests | pass | `cargo run -p replay-check -- --game column_four --all` |
| serialization/deserialization tests | pass | replay support tests/benchmarks |
| visibility/no-leak tests | pass | visibility tests and browser no-leak smoke |
| bot legality/determinism/explanation tests | pass | `cargo test -p column_four` |
| UI smoke tests | pass | `npm --prefix apps/web run smoke:e2e` |
| native benchmarks | pass with caveat | `cargo bench -p column_four`; random playout target miss documented |
| WASM/browser smoke | pass | `smoke:wasm`, `smoke:e2e` |

## Public Release Decision

Decision: release with explicit constraints

Decision rationale:

- Gate 5 implementation and verification evidence is complete.
- Public browser surfaces are play-first, legal-only, accessible, replayable, and no-leak checked.
- IP/trade-dress posture is neutral and original.

Release constraints:

- Complete GAT5COLFOUPUB-018 repo-level status and atlas updates before declaring the gate closed.
- Get human/legal review if this leaves local preview and becomes publicly hosted.
- Keep the random-playout benchmark target miss visible until an accepted follow-up changes it.

## Blocking Issues

| Issue | Required fix | Owner | Blocks release? |
|---|---|---|---:|
| repo-level status/atlas still pending | GAT5COLFOUPUB-018 | project | yes for gate-close label, no for local smoke |

## Final Checklist

- Official-game contract is satisfied.
- Rule/source/IP evidence is complete.
- No private licensed content ships publicly.
- Hidden-information no-leak surfaces are verified.
- Replay/export safety is verified.
- UI is polished, legal-only, accessible, reduced-motion-aware, and responsive.
- Bot explanations are safe.
- Tests, traces, simulations, replay, serialization, and benchmarks are green or explicitly caveated.
