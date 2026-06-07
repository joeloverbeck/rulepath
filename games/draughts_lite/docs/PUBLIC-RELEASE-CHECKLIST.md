# Draughts Lite Public Release Checklist

Game ID: `draughts_lite`

Public display name: `Draughts Lite`

Implemented variant: `draughts_lite_standard`

Release target: public web build / compound-action demo candidate

Rules version: `draughts_lite-rules-v1`

Data/manifest version: `1`

Engine version: `engine-core-0.1.0`

Date: 2026-06-07

## Public Shipment Rule

If content ships to an unauthorized browser, it has shipped. Draughts Lite's release review covers source files, browser payloads, replay exports, DOM attributes, storage, console output, UI copy, and screenshots/demo recordings made from the app.

## Official-Game Contract Status

| Requirement | Status | Evidence/notes |
|---|---|---|
| source notes complete | pass | [SOURCES.md](SOURCES.md) |
| rules complete with stable rule IDs | pass | [RULES.md](RULES.md) |
| rule coverage complete | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md) |
| mechanics complete | pass | [MECHANICS.md](MECHANICS.md) |
| implementation admission complete | pass | [GAME-IMPLEMENTATION-ADMISSION.md](GAME-IMPLEMENTATION-ADMISSION.md) |
| competent-player analysis complete | pass | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) |
| bot strategy evidence pack complete | pass | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| AI notes complete | pass | [AI.md](AI.md) |
| UI notes complete | pass | [UI.md](UI.md) |
| benchmark notes complete | pass | [BENCHMARKS.md](BENCHMARKS.md) |
| primitive-pressure ledger complete | pass | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) |

## Rule, Source, And IP Status

| Check | Status | Evidence/notes |
|---|---|---|
| public rules prose is original Rulepath prose | pass | [RULES.md](RULES.md) |
| sources are recorded with dates and quality | pass | [SOURCES.md](SOURCES.md) |
| variant and deviations are clear | pass | standard 8 by 8 scoped English draughts subset |
| no copied rulebook prose | pass | original docs |
| no copied component text/assets/screenshots/scans/trade dress | pass | CSS board and piece marks are project-authored and neutral |
| public name/trademark/trade-dress risk reviewed | pass | neutral `Draughts Lite`; no federation/commercial presentation |
| generated assets reviewed | not applicable | no generated assets |
| fonts are system-only or license-reviewed | pass | app uses system font stack |
| human/legal review triggers resolved | constrained | required before public hosted release if project policy requires legal review |

## Bundle/Public Artifact Inspection

| Surface/artifact | Inspected? | Unsafe content found? | Notes/action |
|---|---:|---:|---|
| production JS/WASM bundle | yes | no | `npm --prefix apps/web run build` |
| static assets | yes | no | Vite bundle and `wasm_api.wasm` only |
| public routes | yes | no | static `/rulepath/` smoke path |
| local storage defaults | yes | no | only reduced-motion preference allowed |
| replay examples/export samples | yes | no | explicit not-applicable private-view marker only |
| console logs/diagnostics | yes | no | no-leak smoke captures console/page errors |
| test IDs/DOM attributes | yes | no | public cell ids/generic hooks only |
| dev inspector disabled/redacted in public build | yes | no | secondary panel with public metadata only |

## Hidden-Information No-Leak Surfaces

| Surface | Status | Evidence/test | Notes |
|---|---|---|---|
| public/browser payload | pass | `smoke:wasm` | perfect-information public view only |
| public view | pass | visibility tests; WASM smoke | hidden fields empty |
| action tree | pass | Rust action tree tests; UI smoke | legal origins/destinations/continuations only |
| diagnostics/disabled reasons | pass | diagnostic traces; WASM smoke | safe codes/messages |
| effect logs | pass | replay traces; E2E smoke | viewer-safe semantic effects |
| command logs | pass | replay export/import smoke | public command paths |
| DOM attributes/test IDs | pass | `draughts-lite.smoke.mjs` | no forbidden leak vocabulary |
| browser console/logs | pass | `draughts-lite.smoke.mjs` | console/page errors scanned |
| local/session storage | pass | `a11y-noleak.smoke.mjs` | local reduced-motion only, session empty |
| replay export/import | pass | WASM smoke and browser replay smoke | no hidden state payload |
| bot explanations | pass | bot tests; Draughts Lite smoke | public rationale only |
| candidate rankings/raw scores | pass | bot tests/no-leak scan | not public |
| dev inspector/public build boundary | pass | shell/a11y smoke | viewer-safe metadata only |

## Replay/Export Safety

| Check | Status | Evidence/notes |
|---|---|---|
| replay reproduces deterministic hashes | pass | `cargo run -p replay-check -- --game draughts_lite --all` |
| exported replay contains ordered multi-segment paths | pass | WASM export trace and browser export smoke |
| hidden-info redaction verified for exports | not applicable | perfect-information game; private view explicitly not applicable |
| replay import validates versions and schema | pass | replay support tests and smoke |
| replay UI is viewer-safe | pass | `ReplayViewer.tsx` uses Rust projection |
| golden traces are not silently updated | pass | trace hashes checked by replay-check |

## UI Polish And Legal-Only UI

| Check | Status | Evidence/notes |
|---|---|---|
| default public screen is play-first | pass | shell and Draughts Lite smokes |
| visual target is polished and neutral | pass | [UI.md](UI.md), `DraughtsLiteBoard.tsx` |
| no proprietary mimicry/trade dress | pass | neutral name/colors/shapes |
| legal moves are obvious | pass | legal origin/destination highlights from Rust action tree |
| compound path is legible | pass | pending path, dev panel, replay UI use full ` > ` paths |
| player feedback is clear after every action | pass | semantic effects/status/cues/live region |
| semantic effects drive highlights | pass | recent path/capture/promotion classes from Rust effects |
| animations settle to Rust public view | pass | board renders latest Rust view |
| React default preserved | pass | no Canvas/PixiJS/WebGL |
| TypeScript does not decide legality | pass | controls derive from Rust action tree metadata |
| stale/invalid submissions return safe diagnostics | pass | WASM smoke and traces |
| no raw command editing in public mode | pass | replay textarea imports full documents only |

## Accessibility, Reduced-Motion, And Responsive Checks

| Check | Status | Evidence/notes |
|---|---|---|
| keyboard path for core actions | pass | `draughts-lite.smoke.mjs` uses arrows, Enter, and Space |
| forced-capture path | pass | smoke creates mandatory capture through public UI |
| visible focus indicators | pass | E2E focus assertion |
| accessible names/labels for controls | pass | 64 cells checked for names |
| screen-reader summaries/live region | pass | turn, selected origin, pending path, mandatory capture, continuation, promotion, effects |
| contrast reviewed | pass | neutral high-contrast CSS; release human review still useful |
| color is not sole information channel | pass | text labels, `K` crown mark, status/cue/effect text |
| reduced-motion behavior implemented | pass | Draughts Lite smoke checks reduced board transitions |
| responsive layout smoke-tested | pass | shell no-leak/a11y narrow viewport; board uses constrained grid |
| accessibility scan where practical | constrained | smoke coverage exists; external audit can be added later |

## Bot Explanation Safety

| Check | Status | Evidence/notes |
|---|---|---|
| public bot uses legal action API | pass | [AI.md](AI.md), bot tests |
| bot does not access forbidden hidden information | pass | perfect-information public board only |
| Level 1 evidence pack complete | pass | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| no public v1/v2 MCTS/ISMCTS/Monte Carlo/ML/RL/search solver | pass | [AI.md](AI.md) |
| public explanations are viewer-safe | pass | bot tests and browser no-leak smoke |
| candidate rankings are not exposed | pass | not exposed publicly |
| bot latency acceptable | pass | [BENCHMARKS.md](BENCHMARKS.md) |
| public default bot suitability recorded | pass | [AI.md](AI.md) |

## Tests, Traces, Simulations, Replay, Serialization, And Benchmarks

| Evidence | Status | Notes |
|---|---|---|
| unit/named rule tests | pass | `cargo test -p draughts_lite` |
| golden traces | pass | 18 trace files including WASM export; replay-check |
| invalid/stale diagnostic traces | pass | stale, non-active, non-playable, occupied, mandatory-capture, continuation, promotion diagnostics |
| multi-jump trace | pass | `multi-jump.trace.json` |
| property/invariant tests | pass | `games/draughts_lite/tests/property.rs` |
| simulation/fuzz runs | pass | `cargo run -p simulate -- --game draughts_lite --games 1000` |
| replay/hash tests | pass | `cargo run -p replay-check -- --game draughts_lite --all` |
| serialization/deserialization tests | pass | replay support tests/benchmarks |
| visibility/no-leak tests | pass | visibility tests and browser no-leak smoke |
| bot legality/determinism/explanation tests | pass | `cargo test -p draughts_lite` |
| UI smoke tests | pass | Draughts Lite and shared E2E smokes |
| native benchmarks | pass with caveat | `cargo bench -p draughts_lite`; thresholds are smoke floors |
| WASM/browser smoke | pass | `smoke:wasm`, `smoke:ui`, E2E smokes |

## Public Release Decision

Decision: release candidate with explicit constraints

Decision rationale:

- Gate 7 implementation and verification evidence is complete.
- Public browser surfaces are play-first, legal-only, accessible, replayable, compound-path-aware, and no-leak checked.
- IP/trade-dress posture is neutral and original.

Release constraints:

- Complete the final status/spec-index ticket before declaring the gate closed.
- Get human/legal review if this leaves local preview and becomes publicly hosted.
- Keep baseline-pending benchmark posture visible until repeated CI evidence supports stronger thresholds.
- Do not add tournament draw/adjudication features without a new accepted spec.

## Blocking Issues

| Issue | Required fix | Owner | Blocks release? |
|---|---|---|---:|
| final status/spec-index closure pending | GAT7DRALITCOM-022 | project | yes for gate closeout |

## Final Checklist

- Official-game contract is satisfied.
- Rule/source/IP evidence is complete.
- Primitive-pressure ledger is complete.
- No private licensed content ships publicly.
- Hidden-information no-leak surfaces are verified.
- Replay/export safety is verified.
- UI is polished, legal-only, accessible, reduced-motion-aware, and responsive.
- Bot explanations are safe.
- Tests, traces, simulations, replay, serialization, CI, and benchmarks are green or explicitly caveated.
