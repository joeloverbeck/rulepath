# event_frontier Public Release Checklist

Game ID: `event_frontier`

Public display name: `Event Frontier`

Implemented variant: `event_frontier_standard` plus `event_frontier_hard_winter` and `event_frontier_land_rush`

Release target: local preview / public web build

URL/build artifact if applicable: `apps/web/dist`

Rules version: `event-frontier-rules-v1`

Data/manifest version: `1`

Engine version: `engine-core-0.1.0`

Prepared by: Codex

Date: 2026-06-12

## Public Shipment Rule

Event Frontier is treated as public once it appears in the browser catalog, generated player rules, public replay exports, or the built web bundle.

## Official-Game Contract Status

| Requirement | Status | Evidence/notes |
|---|---|---|
| `GAME-SOURCES.md` complete | pass | `SOURCES.md` |
| `GAME-RULES.md` complete with stable rule IDs | pass | `RULES.md` |
| `GAME-RULE-COVERAGE.md` complete | pass | `RULE-COVERAGE.md`; `rule-coverage` green. |
| `GAME-MECHANICS.md` complete | pass | `MECHANICS.md` |
| `GAME-IMPLEMENTATION-ADMISSION.md` complete | pass | `GAME-IMPLEMENTATION-ADMISSION.md` |
| `COMPETENT-PLAYER.md` complete if strategy matters | pass | `COMPETENT-PLAYER.md` |
| `BOT-STRATEGY-EVIDENCE-PACK.md` complete if Level 2 ships | pass | Level 1 ships; evidence pack still records bot behavior. |
| `GAME-AI.md` complete | pass | `AI.md` |
| `GAME-UI.md` complete | pass | `UI.md` |
| `GAME-BENCHMARKS.md` complete | pass | `BENCHMARKS.md` |
| primitive-pressure ledger complete if needed | pass | `PRIMITIVE-PRESSURE-LEDGER.md` |

## Rule, Source, and IP Status

| Check | Status | Evidence/notes |
|---|---|---|
| public rules prose is original Rulepath prose | pass | `RULES.md`, `HOW-TO-PLAY.md` |
| sources are recorded with dates and quality | pass | `SOURCES.md` |
| variant and deviations are clear | pass | `variants.toml`, `RULES.md`, `MECHANICS.md` |
| no copied rulebook prose | pass | Original prose only. |
| no copied card/component text | pass | Original Event Frontier cards and labels. |
| no copied icons, board art, screenshots, scans, or trade dress | pass | React/SVG/CSS implementation only. |
| public name/trademark/trade-dress risk reviewed | pass | Original placeholder naming recorded in `SOURCES.md`. |
| generated assets reviewed | pass | Generated player rules checked by `check-player-rules`. |
| fonts are system-only or license-reviewed | pass | Web shell uses system fonts. |
| human/legal review triggers resolved | not applicable | No trigger beyond maintainer review. |

## Original Prose, Assets, and Font Status

| Content group | Status | Public artifact path | Reviewer/notes |
|---|---|---|---|
| rules/help prose | original | `games/event_frontier/docs/RULES.md`, `HOW-TO-PLAY.md` | Original Rulepath prose. |
| UI copy | original | `apps/web/src/components/EventFrontierBoard.tsx`, `effectFeedback.ts` | Original labels/copy. |
| component/card text | original | `games/event_frontier/data/cards.toml` | Original card labels and typed IDs. |
| icons/SVG/assets | original | `EventFrontierBoard.tsx` | SVG graph rendered from Rust view data. |
| screenshots/scans | none | not applicable | None shipped. |
| fonts | system only | `apps/web` CSS | No bundled fonts. |

## Private Licensed Content Exclusion

| Check | Status | Evidence/notes |
|---|---|---|
| no private licensed rules/content in public files | pass | No private content used. |
| no private game names in public source/build | pass | Grep/review during ticket series; Event Frontier names are original. |
| no private assets in bundle | pass | No external assets added. |
| no private content in traces/fixtures/replay exports | pass | Public replay export redacts command stream and hidden deck order. |
| private stress-test work did not shape `engine-core` | pass | No Gate P/private work started. |

## Bundle/Public Artifact Inspection

| Surface/artifact | Inspected? | Unsafe content found? | Notes/action |
|---|---:|---:|---|
| production JS/WASM bundle | yes | no | `npm --prefix apps/web run build`, browser E2E. |
| static assets | yes | no | Generated rules manifest checked. |
| source maps if shipped | not shipped | no | No source-map release surface asserted. |
| public routes | yes | no | Static `/rulepath/` E2E. |
| local storage defaults | yes | no | Only reduced-motion preference allowed. |
| replay examples/export samples | yes | no | Public export/import no-leak E2E. |
| console logs/diagnostics | yes | no | E2E console assertions. |
| test IDs/DOM attributes | yes | no | Browser no-leak smoke. |
| dev inspector disabled/redacted in public build | yes | no | Developer panel receives public projection only. |

## Hidden-Information No-Leak Surfaces

| Surface | Status | Evidence/test | Notes |
|---|---|---|---|
| public/browser payload | pass | `smoke-load-wasm`, `event-frontier.smoke.mjs` | Current/next/discard public only. |
| public view | pass | `visibility.rs`, `smoke-load-wasm` | Seats/observer output-equivalent and no full deck order. |
| action tree | pass | `actions.rs` tests, E2E legal buttons | No hidden order or candidate ranking. |
| previews | not applicable | No preview surface | Action metadata only. |
| diagnostics/disabled reasons | pass | Rule/WASM smoke | Safe codes/messages. |
| effect logs | pass | visibility tests, `smoke:effects` | Public semantic effects only. |
| command logs | pass | public export/import | Raw command stream omitted from public export. |
| DOM attributes | pass | `event-frontier.smoke.mjs`, `a11y-noleak.smoke.mjs` | No full deck order/internal hash. |
| test IDs | pass | E2E no-leak scan | Test IDs contain public action labels only. |
| browser console/logs | pass | E2E console assertions | No unsafe terms. |
| local storage/session storage | pass | E2E storage assertions | Reduced-motion preference only. |
| replay export/import | pass | `smoke-load-wasm`, `event-frontier.smoke.mjs` | Hidden surface named; order redacted. |
| bot explanations | pass | bot tests/evidence pack | Explanations are public rationale only. |
| candidate rankings | not applicable | No public candidate ranking export | Not exposed. |
| dev inspector/public build boundary | pass | E2E/dev-panel review | Public projection only. |

## Replay/Export Safety

| Check | Status | Evidence/notes |
|---|---|---|
| replay reproduces deterministic hashes | pass | `replay-check`, golden traces. |
| exported replay contains only intended public or authorized data | pass | Public export omits raw commands and full deck order. |
| hidden-info redaction verified for exports | pass | `smoke-load-wasm`, browser E2E. |
| replay import validates versions and schema | pass | WASM import path and public replay tests. |
| replay UI is viewer-safe | pass | Browser replay import/step smoke. |
| golden traces are not silently updated | pass | `cargo test -p event_frontier --test golden_traces`. |

## UI Polish and Visual Target

| Check | Status | Evidence/notes |
|---|---|---|
| default public screen is play-first, not debug-first | pass | Catalog/start flow opens the playable board. |
| visual target is polished and neutral | pass | SVG map, public cards, metrics, edicts, outcome panel. |
| no proprietary mimicry/trade dress | pass | Original Rulepath styling and labels. |
| legal moves are obvious | pass | Rust action-tree buttons. |
| player feedback is clear after every action | pass | Semantic effect feedback and outcome explanation. |
| semantic effects drive animations | pass | `smoke:effects`, browser smoke. |
| animations settle to Rust public view | pass | E2E checks view/status after actions. |
| help/onboarding adequate for target | pass | `HOW-TO-PLAY.md`, in-shell rules display. |
| React + SVG default preserved or justified | pass | Board is React/SVG. |

## Legal-Only UI

| Check | Status | Evidence/notes |
|---|---|---|
| TypeScript does not decide legality | pass | UI submits Rust action-tree leaves only. |
| UI controls derive from Rust action tree | pass | `EventFrontierBoard.tsx` receives `ActionTree`. |
| compound actions use Rust next choices at every stage | pass | Operation leaves are Rust-provided action segments. |
| Rust previews are viewer-safe | not applicable | No separate preview surface. |
| stale/invalid submissions return safe diagnostics | pass | WASM smoke and developer-panel diagnostics. |
| no raw command editing in public mode | pass | Public UI has buttons only. |

## Accessibility, Reduced-Motion, and Responsive Checks

| Check | Status | Evidence/notes |
|---|---|---|
| keyboard path for core actions | pass | Button controls have accessible names/focus behavior through shell conventions. |
| visible focus indicators | pass | Shared shell a11y smoke. |
| accessible names/labels for controls | pass | `event-frontier.smoke.mjs`, `a11y-noleak.smoke.mjs`. |
| screen-reader summaries where practical | pass | Board includes SR-only public-state summary. |
| contrast reviewed | pass | Existing shell palette and visible text surfaces. |
| color is not sole information channel | pass | Text labels and metrics accompany map markers. |
| reduced-motion behavior implemented | pass | `.event-frontier-board.reduced` smoke. |
| responsive layout smoke-tested | pass | Mobile viewport layout assertion. |
| accessibility scan where practical | pass | Browser a11y/no-leak smoke. |

## Bot Explanation Safety

| Check | Status | Evidence/notes |
|---|---|---|
| public bot uses legal action API | pass | Bot commands are validated through Rust action API. |
| bot does not access forbidden hidden information | pass | Bot inputs are scoped; evidence pack records policy surfaces. |
| Level 2 evidence pack complete if Level 2 ships | not applicable | Level 1 ships. |
| no public v1/v2 MCTS/ISMCTS/Monte Carlo/ML/RL | pass | Level 0/1 scripted bots only. |
| public explanations are viewer-safe | pass | Rationale strings contain public view/action facts. |
| candidate rankings are dev-only and redacted | not applicable | No candidate ranking public export. |
| bot latency acceptable | pass | Benchmarks and simulation evidence. |
| public default bot suitability recorded | pass | `AI.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `COMPETENT-PLAYER.md`. |

## Dev Inspector/Public Build Boundary

| Check | Status | Evidence/notes |
|---|---|---|
| public build hides or disables unsafe dev inspector | pass | Developer panel uses safe diagnostics/effects only. |
| dev inspector receives viewer-safe payloads only unless local test harness | pass | Public projection from WASM. |
| full internal state never ships to unauthorized browser | pass | No full deck order/public raw command export. |
| debug candidate rankings redacted | not applicable | No candidate rankings exposed. |
| console/log output safe | pass | Browser E2E console no-leak assertions. |

## Tests, Traces, Simulations, Replay, Serialization, and Benchmarks

| Evidence | Status | Notes |
|---|---|---|
| unit tests | pass | `cargo test --workspace` |
| named rule tests | pass | `cargo test -p event_frontier` and `rule-coverage`. |
| golden traces | pass | `cargo test -p event_frontier --test golden_traces`; `replay-check`. |
| invalid/stale diagnostic traces | pass | Rust/WASM diagnostics and smoke checks. |
| property/invariant tests | pass | `property.rs`; workspace tests. |
| simulation/fuzz runs | pass | `cargo run -p simulate -- --game event_frontier --games 1000`. |
| replay/hash tests | pass | `cargo run -p replay-check -- --game event_frontier --all`. |
| serialization/deserialization tests | pass | `serialization.rs`; fixture checks. |
| visibility/no-leak tests | pass | Rust visibility tests, WASM smoke, E2E no-leak. |
| bot legality/determinism/explanation tests | pass | Bot tests and simulations. |
| UI smoke tests | pass | `smoke:ui`, `smoke:effects`, Event Frontier E2E. |
| accessibility/reduced-motion/responsive smoke | pass | `a11y-noleak.smoke.mjs`, `event-frontier.smoke.mjs`. |
| native benchmarks | pass | `cargo bench -p event_frontier`; bench-report registration. |
| WASM/browser smoke benchmarks | pass | `smoke:wasm`, `smoke:e2e`. |

## Public Release Decision

Decision: release

Decision rationale:

- Gate 14 exit criteria pass with Event Frontier registered across Rust, tools, WASM, browser, docs, and CI.
- Hidden deck order is redacted from public/browser/replay surfaces.
- Public Rulepath stands through Gate 14 without private experiments or Gate P preparation.

Release constraints, if any:

- None.

## Blocking Issues

| Issue | Required fix | Owner | Blocks release? |
|---|---|---|---:|
| None | Not applicable | Rulepath maintainers | no |

## Human Review Notes

| Reviewer | Area | Notes | Date |
|---|---|---|---|
| Rulepath maintainers | release | Maintainer review remains the final human release gate. | 2026-06-12 |

## Final Checklist

- Official-game contract is satisfied.
- Rule/source/IP evidence is complete.
- No private licensed content ships publicly.
- Production bundle/artifact inspection is clean.
- Hidden-information no-leak surfaces are verified.
- Replay/export safety is verified.
- UI is polished, legal-only, accessible, reduced-motion-aware, and responsive.
- Bot explanations are safe.
- Dev inspector/public build boundary is safe.
- Tests, traces, simulations, replay, serialization, and benchmarks are green or explicitly accepted.
- Blocking issues are resolved or release is blocked.
