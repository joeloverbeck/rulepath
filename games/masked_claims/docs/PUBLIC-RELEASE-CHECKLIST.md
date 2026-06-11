# masked_claims Public Release Checklist

Game ID: `masked_claims`

Public display name: `Masked Claims`

Implemented variant: `masked_claims_standard`

Release target: local preview / public web build

URL/build artifact if applicable: `apps/web/dist` from `npm --prefix apps/web run build`

Rules version: `masked-claims-rules-v1`

Data/manifest version: `1`

Engine version: `rulepath-wasm-api/0.1.0`

Prepared by: Codex

Date: 2026-06-11

## Public shipment rule

If content ships to an unauthorized browser, it has shipped. Masked Claims is release-eligible only because public browser payloads, replay exports, logs, diagnostics, DOM attributes, test IDs, local storage, and bot explanations are covered by redaction tests and smoke evidence.

## Official-game contract status

| Requirement | Status | Evidence/notes |
|---|---|---|
| `SOURCES.md` complete | pass | `games/masked_claims/docs/SOURCES.md` |
| `RULES.md` complete with stable rule IDs | pass | `games/masked_claims/docs/RULES.md` |
| `RULE-COVERAGE.md` complete | pass | `cargo run -p rule-coverage -- --game masked_claims` |
| `MECHANICS.md` complete | pass | `games/masked_claims/docs/MECHANICS.md` |
| `GAME-IMPLEMENTATION-ADMISSION.md` complete | pass | This capstone document set. |
| `COMPETENT-PLAYER.md` complete | pass | `games/masked_claims/docs/COMPETENT-PLAYER.md` |
| `BOT-STRATEGY-EVIDENCE-PACK.md` complete | pass | Level 1 evidence pack present. |
| `AI.md` complete | pass | Bot levels and redaction boundaries recorded. |
| `UI.md` complete | pass | Browser and no-leak surfaces recorded. |
| `BENCHMARKS.md` complete | pass | Native benchmark identity and thresholds recorded. |
| primitive-pressure ledger complete | pass | Fourth-use shuffle/private-hand review and first-use reaction window recorded. |

## Rule, source, and IP status

| Check | Status | Evidence/notes |
|---|---|---|
| public rules prose is original Rulepath prose | pass | `RULES.md` and `HOW-TO-PLAY.md`. |
| sources are recorded with dates and quality | pass | `SOURCES.md`. |
| variant and deviations are clear | pass | Only `masked_claims_standard` ships. |
| no copied rulebook prose | pass | Consulted sources used only as context. |
| no copied card/component text | pass | Original mask/grade labels; no roles. |
| no copied icons, board art, screenshots, scans, or trade dress | pass | No external assets ship. |
| public name/trademark/trade-dress risk reviewed | pass | Neutral original name recorded. |
| generated assets reviewed | pass | Player rules generated from `HOW-TO-PLAY.md`; `check-player-rules` passes. |
| fonts are system-only or license-reviewed | pass | Existing web shell fonts only. |
| human/legal review triggers resolved | not applicable | No trigger remains open. |

## Original prose, assets, and font status

| Content group | Status | Public artifact path | Reviewer/notes |
|---|---|---|---|
| rules/help prose | original | `games/masked_claims/docs/HOW-TO-PLAY.md`, `apps/web/public/rules/masked_claims.md` | Generated copy is in sync. |
| UI copy | original | `apps/web/src/components/MaskedClaimsBoard.tsx` | Original neutral labels. |
| component/card text | original | `games/masked_claims/src/ids.rs`, docs | Mask labels are original. |
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
| replay examples/export samples | yes | no | Public export redacts hidden claims. |
| console logs/diagnostics | yes | no | Browser smoke checks console text. |
| test IDs/DOM attributes | yes | no | Browser smoke forbids unrevealed `mask_g*` IDs. |
| dev inspector disabled/redacted in public build | yes | no | Dev panel reports viewer-filtered hidden-info surfaces. |

## Hidden-information no-leak surfaces

| Surface | Status | Evidence/test | Notes |
|---|---|---|---|
| public/browser payload | pass | `cargo test --workspace`, `npm --prefix apps/web run smoke:e2e` | Observer/seat views are filtered. |
| public view | pass | native visibility tests, WASM bridge tests | Reserve and unrevealed masks stay hidden. |
| action tree | pass | action tests, browser smoke | Only actor gets legal private claim paths. |
| previews | not applicable | n/a | No separate preview payload. |
| diagnostics/disabled reasons | pass | native diagnostic tests | Diagnostics echo only submitted/public facts. |
| effect logs | pass | native effect filtering and browser smoke | Reveal effects only expose challenged masks. |
| command logs | pass | replay/export tests | Claim command summaries redact tile IDs. |
| DOM attributes | pass | `apps/web/e2e/masked-claims.smoke.mjs` | `data-testid`s use turn/position, not tile IDs. |
| test IDs | pass | `apps/web/e2e/masked-claims.smoke.mjs` | No unrevealed `mask_g*` anchors. |
| browser console/logs | pass | `apps/web/e2e/masked-claims.smoke.mjs` | Console checked for internal terms. |
| local storage/session storage | pass | `apps/web/e2e/masked-claims.smoke.mjs` | Only motion preference persists. |
| replay export/import | pass | WASM tests, browser smoke | Viewer-scoped public observation export/import. |
| bot explanations | pass | bot tests and WASM/browser smoke | Public explanations are safe. |
| candidate rankings | not applicable | n/a | No public candidate ranking ships. |
| dev inspector/public build boundary | pass | browser smoke | Dev panel redacts hidden action paths. |

## Replay/export safety

| Check | Status | Evidence/notes |
|---|---|---|
| replay reproduces deterministic hashes | pass | `cargo test --workspace`; `replay-check --all`. |
| exported replay contains only intended public data | pass | Public export redaction tests. |
| hidden-info redaction verified for exports | pass | `masked_claims_bridge_filters_unrevealed_masks_and_exports_redacted_claims`; browser smoke. |
| replay import validates versions and schema | pass | WASM import tests. |
| replay UI is viewer-safe | pass | Browser smoke imports public export and checks replay viewer text. |
| golden traces are not silently updated | pass | Trace files checked by `replay-check --all`. |

## UI polish and legal-only UI

| Check | Status | Evidence/notes |
|---|---|---|
| default public screen is play-first | pass | Game picker starts playable shell. |
| visual target is polished and neutral | pass | Masked Claims board uses existing quiet game surface patterns. |
| no proprietary mimicry/trade dress | pass | No external art/theme. |
| legal moves are obvious | pass | Claim and response controls come from Rust action tree. |
| player feedback is clear after every action | pass | Semantic effect feedback covers claim, response, reveal, resolution, and terminal. |
| semantic effects drive animations | pass | Effect log and board feedback use Rust effects. |
| animations settle to Rust public view | pass | Browser smoke exercises claim, accept, challenge, and bot response. |
| TypeScript does not decide legality | pass | TS submits Rust action paths only. |
| stale/invalid submissions return safe diagnostics | pass | WASM and smoke tests cover stale diagnostics. |

## Accessibility, reduced-motion, and responsive checks

| Check | Status | Evidence/notes |
|---|---|---|
| keyboard path for core actions | pass | `masked-claims.smoke.mjs` focuses and activates claim control. |
| visible focus indicators | pass | Existing button/focus styling and smoke check. |
| accessible names/labels for controls | pass | Browser smoke checks unnamed buttons. |
| screen-reader summaries where practical | pass | Board includes live/sr-only summary. |
| contrast reviewed | pass | Uses existing web shell palette. |
| color is not sole information channel | pass | Labels and text identify seats/grades/counters. |
| reduced-motion behavior implemented | pass | Browser smoke selects reduce mode. |
| responsive layout smoke-tested | pass | Browser smoke checks mobile table layout. |

## Bot explanation safety

| Check | Status | Evidence/notes |
|---|---|---|
| public bot uses legal action API | pass | Bot tests and simulation. |
| bot does not access forbidden hidden information | pass | Bot tests cover viewer-safe role decisions. |
| Level 2 evidence pack complete if Level 2 ships | not applicable | Level 2 does not ship. |
| no public v1/v2 MCTS/ISMCTS/Monte Carlo/ML/RL | pass | Only random legal and Level 1 baseline bots ship. |
| public explanations are viewer-safe | pass | Native/WASM effect tests. |
| candidate rankings are dev-only and redacted | not applicable | No public candidate rankings. |
| bot latency acceptable | pass | Benchmarks and simulations pass. |

## Tests, traces, simulations, replay, serialization, and benchmarks

| Evidence | Status | Notes |
|---|---|---|
| unit and integration tests | pass | `cargo test --workspace`. |
| named rule tests | pass | `games/masked_claims/tests/rules.rs`. |
| golden traces | pass | `cargo run -p replay-check -- --game masked_claims --all`. |
| invalid/stale diagnostics | pass | Native tests and golden traces. |
| property/invariant tests | pass | `games/masked_claims/tests/property.rs`. |
| simulation/fuzz runs | pass | `cargo run -p simulate -- --game masked_claims --games 1000`. |
| replay/hash tests | pass | native replay tests and `replay-check`. |
| serialization/deserialization tests | pass | `games/masked_claims/tests/serialization.rs`. |
| visibility/no-leak tests | pass | native visibility tests, WASM bridge tests, browser smoke. |
| bot legality/determinism/explanation tests | pass | `games/masked_claims/tests/bots.rs`. |
| UI smoke tests | pass | `smoke:wasm`, `smoke:ui`, `smoke:e2e`. |
| accessibility/reduced-motion/responsive smoke | pass | `apps/web/e2e/masked-claims.smoke.mjs`. |
| native benchmarks | pass | Bench docs and benchmark ticket evidence recorded. |
| WASM/browser smoke benchmarks | pass | web smoke layers pass. |

## Public release decision

Decision: release

Decision rationale:

- Masked Claims satisfies the official-game contract and Gate 11 exit criteria with current native, WASM, browser, documentation, catalog, player-rules, and outcome-explanation evidence.

Release constraints:

- No generic reaction-window helper is admitted.
- Accepted masks, unplayed masks, and reserve identities remain redacted forever.

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
