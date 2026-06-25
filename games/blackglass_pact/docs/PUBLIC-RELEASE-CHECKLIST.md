# Blackglass Pact Public Release Checklist

Game ID: `blackglass_pact`

Public display name: `Blackglass Pact`

Implemented variant: `blackglass_pact_standard`

Release target: public web build / portfolio demo

Rules version: `blackglass-pact-rules-v1`

Data/manifest version: `blackglass-pact-data-v1`

Engine version: `engine-core-0.1.0`

Date: 2026-06-25

## Public Shipment Rule

If content ships to an unauthorized browser, it has shipped. Blackglass Pact
must not expose private hands, future deck order, hidden bot inputs, private
viewer facts, or unauthorized replay data in public files, browser payloads,
DOM, storage, logs, exports, traces, or tests.

## Official-Game Contract Status

| Requirement | Status | Evidence/notes |
|---|---|---|
| sources complete | pass with pending human review | [SOURCES.md](SOURCES.md) records consulted sources and non-copying posture. |
| rules complete with stable IDs | pass | [RULES.md](RULES.md) |
| rule coverage complete | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md); `rule-coverage` passes from earlier evidence. |
| mechanics inventory complete | pass | [MECHANICS.md](MECHANICS.md) |
| implementation admission complete | pass | [GAME-IMPLEMENTATION-ADMISSION.md](GAME-IMPLEMENTATION-ADMISSION.md) |
| competent-player analysis | pass with constraint | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md); no L2 admission. |
| bot strategy evidence pack | not applicable for shipped bot | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) records deferred L2. |
| AI registry complete | pass | [AI.md](AI.md) |
| UI doc complete | pass | [UI.md](UI.md), includes outcome section |
| benchmarks complete | pass | [BENCHMARKS.md](BENCHMARKS.md), `cargo bench -p blackglass_pact` evidence recorded earlier. |
| primitive-pressure ledger complete | pending final receipt | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md); repo atlas/register finalization is GAT18BLAPACSPA-018. |
| supported player-count smoke | pass | exactly four seats, setup rejects other counts |
| pairwise no-leak matrix | pass | Rust visibility tests, WASM no-leak tests, Blackglass e2e canary scan |
| per-team outcome explanation | pass | [UI.md](UI.md), `check-outcome-explanations.mjs` |

## Rule, Source, And IP Status

| Check | Status | Evidence/notes |
|---|---|---|
| public rules prose is original | pass | [RULES.md](RULES.md) |
| sources recorded with dates and quality | pass | [SOURCES.md](SOURCES.md) |
| variant and deviations clear | pass | `blackglass_pact_standard`; no house-rule variants |
| no copied rulebook prose | pass | source-use statement records consulted-not-copied posture |
| no copied card/component text | pass | standard card facts only; labels are authored locally |
| no copied icons/art/screenshots/fonts/trade dress | pass | original web presentation; no bundled font files |
| public name risk reviewed | pending human | Bounded screening is recorded, but not legal clearance. |
| private licensed content excluded | pass | no private content involved |

## Hidden-Information No-Leak Surfaces

| Surface | Status | Evidence/test | Notes |
|---|---|---|---|
| public/browser payload | pass | `crates/wasm-api` tests, e2e smoke | observer has no `own_hand` |
| public view | pass | visibility tests | counts/public teams/bids/played cards/scores only |
| seat view | pass | visibility tests | contains owning seat hand only |
| action tree | pass | WASM/action tests | actor-only private card leaves |
| previews | pass | owner-scoped previews only | no hidden alternatives |
| diagnostics | pass | rule/visibility tests | safe stable codes |
| effect logs | pass | effect filter tests | private effects are viewer-scoped |
| command logs | pass | replay/export checks | no privilege elevation on import |
| DOM/test IDs | pass | `blackglass-pact.smoke.mjs` canary scan | no private tokens |
| console/storage | pass | `blackglass-pact.smoke.mjs` | no forbidden hidden terms |
| replay export/import | pass | replay tests and e2e | public/seat-private exports only |
| bot explanations | pass | `tests/bots.rs` | no hidden card names or forbidden methods |
| candidate rankings | not applicable | no public candidate ranking | future Level 2 must add tests |
| dev inspector/public boundary | pass | e2e no-leak scan | full internal state not public |

## Pairwise No-Leak And Outcome Gate

| Check | Status | Evidence/test | Notes |
|---|---|---|---|
| pairwise source-seat private datum by viewer | pass | Rust visibility tests plus browser canary scan | Partner relationship grants no private-hand visibility. |
| blind-phase no-card exposure | pass | blind no-leak tests and e2e | No card datum exists before deal. |
| per-team outcome explanation complete | pass | [UI.md](UI.md), `check-outcome-explanations.mjs` | Final scores and terminal team are public. |
| no-reveal terminal outcome | not applicable | terminal scoring uses completed public hand/score facts | No folded/no-reveal terminal. |

## Replay/Export Safety

| Check | Status | Evidence/notes |
|---|---|---|
| replay reproduces deterministic hashes | pass | `cargo test -p blackglass_pact --test replay`, replay-check evidence |
| exported replay is viewer-scoped | pass | WASM export/import tests |
| hidden-info redaction verified | pass | export tests and e2e scans |
| multi-seat export/import verified | pass | observer plus four seat-private classes |
| import validates versions/schema | pass | replay header/import tests |
| replay UI viewer-safe | pass | Blackglass e2e replay/export/import smoke |
| golden traces not silently updated | pass | fixture/replay checks in series evidence |

## UI Polish, Legal-Only, Accessibility

| Check | Status | Evidence/notes |
|---|---|---|
| play-first public screen | pass | `BlackglassPactBoard.tsx` |
| visual target neutral/original | pass | no casino/trade-dress framing |
| legal moves derive from Rust action tree | pass | blind/bid/card buttons keyed by action leaves |
| TypeScript does not decide legality | pass | UI maps action paths only |
| score/contract/bag facts derive from Rust | pass | board renders projected fields only |
| stale/invalid submissions return Rust diagnostics | pass | WASM/app flow |
| keyboard path for core actions | pass | e2e blind/bid/replay smoke |
| visible/accessibility labels | pass | action/card labels and status regions |
| reduced motion | pass | e2e reduced-motion smoke |
| responsive layout | pass | e2e responsive table check |

## Bot Explanation Safety

| Check | Status | Evidence/notes |
|---|---|---|
| public bot uses legal action API | pass | [AI.md](AI.md), bot tests |
| bot does not access forbidden hidden information | pass | opponent-hidden mutation tests |
| Level 2 evidence pack complete if Level 2 ships | not applicable | Level 2 not admitted |
| no MCTS/ISMCTS/Monte Carlo/ML/RL | pass | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| public explanations viewer-safe | pass | bot explanation tests |
| candidate rankings dev-only/redacted | not applicable | no candidate ranking ships |
| bot latency acceptable | pass | [BENCHMARKS.md](BENCHMARKS.md) |

## Tests, Traces, Simulations, Replay, Serialization, Benchmarks

| Evidence | Status | Notes |
|---|---|---|
| unit/rule/property tests | pass | `cargo test -p blackglass_pact` |
| golden traces | pass | `tests/golden_traces`, fixture/replay checks in series evidence |
| simulation/fuzz runs | pass | simulator lane from earlier tickets |
| replay/hash tests | pass | `cargo test -p blackglass_pact --test replay` |
| serialization tests | pass | `cargo test -p blackglass_pact --test serialization` |
| visibility/no-leak tests | pass | Rust, WASM, and browser no-leak evidence |
| bot legality/determinism/explanation tests | pass | `cargo test -p blackglass_pact --test bots` |
| UI smoke/e2e | pass | `npm --prefix apps/web run smoke:ui`, `smoke:e2e`, Blackglass e2e |
| native benchmarks | pass | `cargo bench -p blackglass_pact`, `bench-report` |
| docs/checkers | pass | doc links, outcome explanations, player rules/catalog checks in series evidence |

## Public Release Decision

Decision: hold public release until human review

Decision rationale:

- Blackglass Pact has original rules/source docs, Rust-owned rules,
  hidden-info no-leak evidence, replay/export evidence, web renderer smoke,
  native benchmarks, and bot safety docs.
- Repo-level primitive-pressure/register receipt remains assigned to
  GAT18BLAPACSPA-018, and final full-gate exit evidence remains assigned to
  GAT18BLAPACSPA-019.
- Human IP/public-release review is still required before any external release
  claim.

Release constraints:

- Public copy must use Blackglass Pact as the product name.
- No public Level 2 competence claim.
- No private licensed content, copied card art, or trade dress.
- No partner/opponent hands, future deck material, or hidden bot candidates in
  browser-facing artifacts.

## Blocking Issues

| Issue | Required fix | Owner | Blocks release? |
|---|---|---|---:|
| human IP/public-release review pending | complete review before external public release | Rulepath maintainers | yes |
| repo-level forward-v1 receipt pending | complete GAT18BLAPACSPA-018 | Rulepath/Codex | yes for Gate 18 closeout |
| final full-gate evidence pending | complete GAT18BLAPACSPA-019 | Rulepath/Codex | yes for Gate 18 closeout |

## Human Review Notes

| Reviewer | Area | Notes | Date |
|---|---|---|---|
| pending human | IP/UI/accessibility/release | Automated/docs evidence is prepared for Gate 18; human release review has not happened in this session. | 2026-06-25 |
