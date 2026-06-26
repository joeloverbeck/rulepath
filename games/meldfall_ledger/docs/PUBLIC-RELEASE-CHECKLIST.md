# Meldfall Ledger Public Release Checklist

Game ID: `meldfall_ledger`

Public display name: `Meldfall Ledger`

Implemented variant: `classic_500_single_deck_v1`

Release target: public web build / portfolio demo

Rules version: `meldfall-ledger-rules-v1`

Data/manifest version: `meldfall-ledger-data-v1`

Engine version: `engine-core-0.1.0`

Date: 2026-06-26

## Public Shipment Rule

If content ships to an unauthorized browser, it has shipped. Meldfall Ledger
must not expose private hands, hidden stock order, next stock card, private bot
inputs, private viewer facts, or unauthorized replay data in public files,
browser payloads, DOM, storage, logs, exports, traces, or tests.

## Official-Game Contract Status

| Requirement | Status | Evidence/notes |
|---|---|---|
| sources complete | pass with pending human review | [SOURCES.md](SOURCES.md) records consulted sources and non-copying posture. |
| rules complete with stable IDs | pass | [RULES.md](RULES.md) |
| rule coverage complete | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md); `rule-coverage` passes. |
| mechanics inventory complete | pass | [MECHANICS.md](MECHANICS.md) |
| implementation admission complete | pass | [GAME-IMPLEMENTATION-ADMISSION.md](GAME-IMPLEMENTATION-ADMISSION.md) |
| competent-player analysis | pass with constraint | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md); no L2 admission. |
| bot strategy evidence pack | not applicable for shipped bot | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) records deferred L2. |
| AI registry complete | pass | [AI.md](AI.md) |
| UI doc complete | pass | [UI.md](UI.md) |
| benchmarks complete | pass | [BENCHMARKS.md](BENCHMARKS.md), `cargo bench -p meldfall_ledger`. |
| primitive-pressure ledger complete | pass | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md); repo atlas/register finalization complete. |
| supported player-count smoke | pass | 2, 4, and 6 seat setup/action/export smoke. |
| pairwise no-leak matrix | pass | Rust visibility tests, WASM tests, and Meldfall e2e canary scan. |
| per-seat outcome/scoring explanation | pass | [UI.md](UI.md), `check-outcome-explanations.mjs`. |

## Rule, Source, And IP Status

| Check | Status | Evidence/notes |
|---|---|---|
| public rules prose is original | pass | [RULES.md](RULES.md) |
| sources recorded with dates and quality | pass | [SOURCES.md](SOURCES.md) |
| variant and deviations clear | pass | One deck for 2-6 seats, strict discard-pickup commitment, ace low/high/no-wrap, no final-discard requirement. |
| no copied rulebook prose | pass | source-use statement records consulted-not-copied posture |
| no copied card/component text | pass | standard card facts only; labels are authored locally |
| no copied icons/art/screenshots/fonts/trade dress | pass | original web presentation; no bundled font files |
| public name risk reviewed | pending human | Bounded screening is recorded, but not legal clearance. |
| private licensed content excluded | pass | no private content involved |

## Hidden-Information No-Leak Surfaces

| Surface | Status | Evidence/test | Notes |
|---|---|---|---|
| public/browser payload | pass | `crates/wasm-api` tests, e2e smoke | Observer has no `own_hand` and no stock order. |
| public view | pass | visibility tests | counts, discard, tableau, scores, active seat only |
| seat view | pass | visibility tests | contains owning seat hand only |
| action tree | pass | WASM/action tests | actor-only private card leaves |
| diagnostics | pass | rule/visibility tests | safe stable codes |
| effect logs | pass | effect filter tests | private stock draw detail is viewer-scoped |
| command logs | pass | replay/export checks | no privilege elevation on import |
| DOM/test IDs | pass | `meldfall-ledger.smoke.mjs` canary scan | no hidden tokens |
| console/storage | pass | `meldfall-ledger.smoke.mjs` | no forbidden hidden terms |
| replay export/import | pass | replay tests and e2e | public/seat-private exports only |
| bot explanations | pass | `tests/bots.rs` | no hidden card names or forbidden methods |
| candidate rankings | not applicable | no public candidate ranking | future Level 2 must add tests |

## UI Polish, Legal-Only, Accessibility

| Check | Status | Evidence/notes |
|---|---|---|
| play-first public screen | pass | `MeldfallLedgerBoard.tsx` |
| visual target neutral/original | pass | no copied table/card-app trade dress |
| legal moves derive from Rust action tree | pass | stock/discard/meld/lay-off/finish/discard controls keyed by Rust leaves |
| TypeScript does not decide legality | pass | UI maps action paths only |
| score/tableau/discard facts derive from Rust | pass | board renders projected fields only |
| stale/invalid submissions return Rust diagnostics | pass | WASM/app flow |
| keyboard path for core actions | pass | e2e focus and no-drag action smoke |
| visible/accessibility labels | pass | action/card labels and status regions |
| reduced motion | pass | shared animation smoke |
| responsive layout | pass | e2e 390px layout check |

## Bot Explanation Safety

| Check | Status | Evidence/notes |
|---|---|---|
| public bot uses legal action API | pass | [AI.md](AI.md), bot tests |
| bot does not access forbidden hidden information | pass | opponent-hand and stock-order no-leak tests |
| Level 2 evidence pack complete if Level 2 ships | not applicable | Level 2 not admitted |
| no MCTS/ISMCTS/Monte Carlo/ML/RL | pass | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| public explanations viewer-safe | pass | bot explanation tests |
| candidate rankings dev-only/redacted | not applicable | no candidate ranking ships |
| bot latency acceptable | pass | [BENCHMARKS.md](BENCHMARKS.md) |

## Tests, Traces, Simulations, Replay, Serialization, Benchmarks

| Evidence | Status | Notes |
|---|---|---|
| unit/rule/property tests | pass | `cargo test -p meldfall_ledger` and workspace tests |
| golden traces | pass | `tests/golden_traces`, fixture/replay checks |
| simulation runs | pass | 2/4/6 seat bounded L0 lanes |
| replay/hash tests | pass | `cargo test -p meldfall_ledger --test replay`, replay-check |
| serialization tests | pass | `cargo test -p meldfall_ledger --test serialization` |
| visibility/no-leak tests | pass | Rust, WASM, and browser no-leak evidence |
| bot legality/determinism/explanation tests | pass | `cargo test -p meldfall_ledger --test bots` |
| UI smoke/e2e | pass | `npm --prefix apps/web run smoke:e2e`, Meldfall e2e |
| native benchmarks | pass | `cargo bench -p meldfall_ledger` |
| docs/checkers | pass | doc links, player rules, catalog, CI games, scaffolding governance |

## Public Release Decision

Decision: hold public release until human review

Decision rationale:

- Meldfall Ledger has original rules/source docs, Rust-owned rules,
  hidden-info no-leak evidence, replay/export evidence, web renderer smoke,
  native benchmarks, bot safety docs, and the forward-v1 governance receipt.
- Human IP/public-release review is still required before any external release
  claim.

Release constraints:

- Public copy must use Meldfall Ledger as the product name.
- Rummy-family labels may appear only as source/context labels.
- No public Level 1 or Level 2 competence claim.
- No private licensed content, copied card art, copied table layout, or trade dress.
- No opponent hands, hidden stock material, or hidden bot candidates in browser-facing artifacts.

## Blocking Issues

| Issue | Required fix | Owner | Blocks release? |
|---|---|---|---:|
| human IP/public-release review pending | complete review before external public release | Rulepath maintainers | yes |
| human UI/accessibility release review pending | complete review before external public release | Rulepath maintainers | yes |

## Human Review Notes

| Reviewer | Area | Notes | Date |
|---|---|---|---|
| pending human | IP/UI/accessibility/release | Automated/docs evidence is prepared for Gate 19; human release review has not happened in this session. | 2026-06-26 |
