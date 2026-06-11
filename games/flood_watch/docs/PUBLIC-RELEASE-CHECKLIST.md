# flood_watch Public Release Checklist

Game ID: `flood_watch`

Public display name: `Flood Watch`

Implemented variants: `flood_watch_standard`, `flood_watch_deluge`

Release target: local preview / public web build

URL/build artifact if applicable: `apps/web/dist` from `npm --prefix apps/web run build`

Rules version: `flood-watch-rules-v1`

Data/manifest version: `1`

Engine version: `rulepath-wasm-api/0.1.0`

Prepared by: Codex

Date: 2026-06-11

## Public shipment rule

If content ships to an unauthorized browser, it has shipped. Flood Watch is release-eligible only because public browser payloads, replay exports, logs, diagnostics, DOM attributes, test IDs, local storage, and bot explanations are covered by redaction tests and smoke evidence.

## Official-game contract status

| Requirement | Status | Evidence/notes |
|---|---|---|
| `SOURCES.md` complete | pass | `games/flood_watch/docs/SOURCES.md` |
| `RULES.md` complete with stable rule IDs | pass | `games/flood_watch/docs/RULES.md` |
| `RULE-COVERAGE.md` complete | pass | `cargo run -p rule-coverage -- --game flood_watch` |
| `MECHANICS.md` complete | pass | `games/flood_watch/docs/MECHANICS.md` |
| `GAME-IMPLEMENTATION-ADMISSION.md` complete | pass | This capstone document set. |
| `COMPETENT-PLAYER.md` complete | pass | `games/flood_watch/docs/COMPETENT-PLAYER.md` |
| `BOT-STRATEGY-EVIDENCE-PACK.md` complete | pass | Level 1 evidence pack present. |
| `AI.md` complete | pass | Bot levels and redaction boundaries recorded. |
| `UI.md` complete | pass | Browser and no-leak surfaces recorded. |
| `BENCHMARKS.md` complete | pass | Native benchmark identity and thresholds recorded. |
| primitive-pressure ledger complete | pass | Reaction-window/shuffle reviews and first-use Gate 12 rows recorded. |

## Rule, source, and IP status

| Check | Status | Evidence/notes |
|---|---|---|
| public rules prose is original Rulepath prose | pass | `RULES.md` and `HOW-TO-PLAY.md`. |
| sources are recorded with dates and quality | pass | `SOURCES.md`. |
| variants and deviations are clear | pass | Standard and Deluge scenario variants are documented. |
| no copied rulebook prose | pass | Consulted sources used only as context. |
| no copied card/component text | pass | Original district, role, and event labels. |
| no copied icons, board art, screenshots, scans, or trade dress | pass | No external assets ship. |
| public name/trademark/trade-dress risk reviewed | pass | Neutral original name recorded. |
| generated assets reviewed | pass | Player rules generated from `HOW-TO-PLAY.md`; `check-player-rules` passes. |
| fonts are system-only or license-reviewed | pass | Existing web shell fonts only. |
| human/legal review triggers resolved | not applicable | No trigger remains open. |

## Original prose, assets, and font status

| Content group | Status | Public artifact path | Reviewer/notes |
|---|---|---|---|
| rules/help prose | original | `games/flood_watch/docs/HOW-TO-PLAY.md`, `apps/web/public/rules/flood_watch.md` | Generated copy is in sync. |
| UI copy | original | `apps/web/src/components/FloodWatchBoard.tsx` | Original neutral labels. |
| component/card text | original | `games/flood_watch/src/ids.rs`, docs | District, role, and event labels are original. |
| icons/SVG/assets | none | n/a | No external art. |
| screenshots/scans | none | n/a | None shipped. |
| fonts | system only | app shell CSS | No new font asset. |

## Bundle/public artifact inspection

| Surface/artifact | Inspected? | Unsafe content found? | Notes/action |
|---|---:|---:|---|
| production JS/WASM bundle | yes | no | `npm --prefix apps/web run smoke:e2e` passed. |
| static assets | yes | no | Generated rules asset checked. |
| source maps if shipped | not shipped | no | Vite production output does not add source maps by default. |
| public routes | yes | no | Static `/rulepath/` mount smoke passed. |
| local storage defaults | yes | no | Browser smoke allows only `rulepath.reducedMotion`. |
| replay examples/export samples | yes | no | Public export redacts hidden event-deck order and raw command stream. |
| console logs/diagnostics | yes | no | Browser smoke checks console text. |
| test IDs/DOM attributes | yes | no | Browser smoke checks forbidden deck/internal terms. |
| dev inspector disabled/redacted in public build | yes | no | Dev panel receives viewer-safe state only. |

## Hidden-information no-leak surfaces

| Surface | Status | Evidence/test | Notes |
|---|---|---|---|
| public/browser payload | pass | `cargo test --workspace`, `npm --prefix apps/web run smoke:e2e` | The undrawn event-deck order is absent. |
| public view | pass | native visibility tests, WASM bridge tests | View exposes drawn/forecast cards and remaining composition counts only. |
| action tree | pass | action tests, browser smoke | Tree labels expose public districts/actions only. |
| previews | not applicable | n/a | No separate preview payload. |
| diagnostics/disabled reasons | pass | native diagnostic tests | Diagnostics echo only submitted/public facts. |
| effect logs | pass | native effect filtering and browser smoke | Effects expose only public draws, forecast, rises, absorbs, and terminal summaries. |
| command logs | pass | replay/export tests | Public export uses redacted command summaries. |
| DOM attributes | pass | `apps/web/e2e/flood-watch.smoke.mjs` | District IDs in test IDs are public; deck order terms are absent. |
| test IDs | pass | `apps/web/e2e/flood-watch.smoke.mjs` | No event-card order or hidden deck identifiers. |
| browser console/logs | pass | `apps/web/e2e/flood-watch.smoke.mjs` | Console checked for internal terms. |
| local storage/session storage | pass | `apps/web/e2e/flood-watch.smoke.mjs` | Only motion preference persists. |
| replay export/import | pass | WASM tests, browser smoke | Viewer-scoped public observation export/import. |
| bot explanations | pass | bot tests and WASM/browser smoke | Public explanations and policy id are safe. |
| candidate rankings | not applicable | n/a | No public candidate ranking ships. |
| dev inspector/public build boundary | pass | browser smoke | Dev panel never receives full internal deck order. |

## Replay/export safety

| Check | Status | Evidence/notes |
|---|---|---|
| replay reproduces deterministic hashes | pass | `cargo test --workspace`; `replay-check --all`. |
| exported replay contains only intended public data | pass | Public export redaction tests. |
| hidden-info redaction verified for exports | pass | WASM no-leak tests and `flood-watch.smoke.mjs`. |
| replay import validates versions and schema | pass | WASM import tests. |
| replay UI is viewer-safe | pass | Browser smoke imports public export and checks replay viewer text. |
| golden traces are not silently updated | pass | Trace files checked by `replay-check --all`. |

## UI polish and legal-only UI

| Check | Status | Evidence/notes |
|---|---|---|
| default public screen is play-first | pass | Game picker starts playable shell. |
| visual target is polished and neutral | pass | Flood Watch board uses existing quiet game surface patterns. |
| no proprietary mimicry/trade dress | pass | No external art/theme. |
| legal moves are obvious | pass | District and turn controls come from Rust action tree. |
| player feedback is clear after every action | pass | Semantic effect feedback covers action, forecast, storm draw, levee absorb, flood rise, inundation, and terminal. |
| semantic effects drive animations | pass | Effect log and board feedback use Rust effects. |
| animations settle to Rust public view | pass | Browser smoke exercises forecast, action, environment, bot, and terminal flows. |
| TypeScript does not decide legality | pass | TS submits Rust action paths only. |
| stale/invalid submissions return safe diagnostics | pass | WASM and smoke tests cover stale diagnostics. |

## Accessibility, reduced-motion, and responsive checks

| Check | Status | Evidence/notes |
|---|---|---|
| keyboard path for core actions | pass | Buttons expose accessible names; smoke checks unnamed controls. |
| visible focus indicators | pass | Existing button/focus styling and smoke check. |
| accessible names/labels for controls | pass | Browser smoke checks unnamed buttons. |
| screen-reader summaries where practical | pass | Board includes live/sr-only summary. |
| contrast reviewed | pass | Uses existing web shell palette. |
| color is not sole information channel | pass | Labels and text identify districts, flood, levees, budget, roles, forecast, and outcomes. |
| reduced-motion behavior implemented | pass | Browser smoke selects reduce mode. |
| responsive layout smoke-tested | pass | Browser smoke checks mobile table layout. |

## Bot explanation safety

| Check | Status | Evidence/notes |
|---|---|---|
| public bot uses legal action API | pass | Bot tests and simulation. |
| bot does not access forbidden hidden information | pass | Bot tests cover public remaining-composition and forecast inputs only. |
| Level 2 evidence pack complete if Level 2 ships | not applicable | Level 2 does not ship. |
| no public v1/v2 MCTS/ISMCTS/Monte Carlo/ML/RL | pass | Only random legal and Level 1 baseline bots ship. |
| public explanations are viewer-safe | pass | Native/WASM effect tests and browser smoke. |
| candidate rankings are dev-only and redacted | not applicable | No public candidate rankings. |
| bot latency acceptable | pass | Benchmarks and simulations pass. |

## Tests, traces, simulations, replay, serialization, and benchmarks

| Evidence | Status | Notes |
|---|---|---|
| unit and integration tests | pass | `cargo test --workspace`. |
| named rule tests | pass | `games/flood_watch/tests/rules.rs`. |
| golden traces | pass | `cargo run -p replay-check -- --game flood_watch --all`. |
| invalid/stale diagnostics | pass | Native tests and golden traces. |
| property/invariant tests | pass | `games/flood_watch/tests/property.rs`. |
| simulation/fuzz runs | pass | `cargo run -p simulate -- --game flood_watch --games 1000`. |
| replay/hash tests | pass | native replay tests and `replay-check`. |
| serialization/deserialization tests | pass | `games/flood_watch/tests/serialization.rs`. |
| visibility/no-leak tests | pass | native visibility tests, WASM bridge tests, browser smoke. |
| bot legality/determinism/explanation tests | pass | `games/flood_watch/tests/bots.rs`. |
| UI smoke tests | pass | `smoke:wasm`, `smoke:ui`, `smoke:effects`, `smoke:e2e`. |
| accessibility/reduced-motion/responsive smoke | pass | `apps/web/e2e/flood-watch.smoke.mjs`. |
| native benchmarks | pass | Bench docs and benchmark ticket evidence recorded. |
| WASM/browser smoke benchmarks | pass | web smoke layers pass. |

## Public release decision

Decision: release

Decision rationale:

- Flood Watch satisfies the official-game contract and Gate 12 exit criteria with current native, WASM, browser, documentation, catalog, player-rules, and outcome-explanation evidence.

Release constraints:

- No generic cooperative/event/budget/role/scenario helper is admitted.
- The undrawn event-deck order remains redacted from public surfaces forever.

## Blocking issues

| Issue | Required fix | Owner | Blocks release? |
|---|---|---|---:|
| None | Not applicable | Rulepath maintainers | no |

## Human review notes

| Reviewer | Area | Notes | Date |
|---|---|---|---|
| Rulepath maintainers | release | Capstone evidence recorded by Codex; no blocking issue remains. | 2026-06-11 |

## Final checklist

- Official-game contract is satisfied.
- Rule/source/IP evidence is complete.
- No private licensed content ships publicly.
- Production bundle/artifact inspection is clean.
- Hidden-information no-leak surfaces are verified.
- Replay/export safety is verified.
- UI is legal-only, accessible, reduced-motion-aware, and responsive.
- Bot explanations are safe.
- Dev inspector/public build boundary is safe.
- Tests, traces, simulations, replay, serialization, and benchmarks are green.
- Blocking issues are resolved.
