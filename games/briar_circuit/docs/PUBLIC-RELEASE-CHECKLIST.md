# Briar Circuit Public Release Checklist

Game ID: `briar_circuit`

Public display name: `Briar Circuit`

Implemented variant: `briar_circuit_standard`

Release target: public web build / portfolio demo

Rules version: `briar-circuit-rules-v1`

Data/manifest version: `1`

Engine version: `engine-core-0.1.0`

Date: 2026-06-21

## Public Shipment Rule

If content ships to an unauthorized browser, it has shipped. Briar Circuit must
not expose private hands, pass selections, pass provenance, deck order, seed
reconstruction, hidden bot inputs, or private replay facts in public files,
browser payloads, DOM, storage, logs, exports, traces, or tests.

## Official-Game Contract Status

| Requirement | Status | Evidence/notes |
|---|---|---|
| sources complete | pass | [SOURCES.md](SOURCES.md) |
| rules complete with stable IDs | pass | [RULES.md](RULES.md) |
| rule coverage complete | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md); `rule-coverage` passes |
| mechanics inventory complete | pass | [MECHANICS.md](MECHANICS.md) |
| implementation admission complete | pass | [GAME-IMPLEMENTATION-ADMISSION.md](GAME-IMPLEMENTATION-ADMISSION.md) |
| competent-player analysis | pass with constraint | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md); does not admit L2 |
| bot strategy evidence pack | not applicable for shipped bot | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) records `L2 not admitted` |
| AI registry complete | pass | [AI.md](AI.md) |
| UI doc complete | pass | [UI.md](UI.md), includes outcome section |
| benchmarks complete | pass | [BENCHMARKS.md](BENCHMARKS.md), `cargo bench -p briar_circuit` |
| primitive-pressure ledger complete | pass local / central pending capstone | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md); central atlas update belongs to closeout |
| supported player-count smoke | pass | exactly four seats, setup rejects other counts |
| pairwise no-leak matrix | pass | Rust visibility tests, WASM no-leak tests, Briar e2e canary scan |
| per-seat outcome explanation | pass | [UI.md](UI.md), `check-outcome-explanations.mjs` |

## Rule, Source, And IP Status

| Check | Status | Evidence/notes |
|---|---|---|
| public rules prose is original | pass | [RULES.md](RULES.md) |
| sources recorded with dates and quality | pass | [SOURCES.md](SOURCES.md) |
| variant and deviations clear | pass | `briar_circuit_standard`; no house-rule variants |
| no copied rulebook prose | pass | source-use statement records consulted-not-copied posture |
| no copied card/component text | pass | standard card facts only; labels are authored locally |
| no copied icons/art/screenshots/fonts/trade dress | pass | original icon/UI work; no bundled font files |
| public name risk reviewed | pass | Briar Circuit is neutral/original |
| private licensed content excluded | pass | no private content involved |

## Hidden-Information No-Leak Surfaces

| Surface | Status | Evidence/test | Notes |
|---|---|---|---|
| public/browser payload | pass | `crates/wasm-api` tests, e2e smoke | observer has no `own_hand` |
| public view | pass | `tests/visibility.rs` | counts/public cards only |
| action tree | pass | WASM/action tests | actor-only private card leaves |
| previews | pass | owner-scoped previews only | no hidden alternatives |
| diagnostics | pass | rule/visibility tests | safe stable codes |
| effect logs | pass | `filter_effects_for_viewer` tests | private pass effects seat-scoped |
| command logs | pass | viewer-scoped replay export | redacted summaries |
| DOM/test IDs | pass | `briar-circuit.smoke.mjs` canary scan | no private tokens |
| console/storage | pass | `briar-circuit.smoke.mjs` | no forbidden hidden terms |
| replay export/import | pass | replay tests and e2e | public/seat-private exports only |
| bot explanations | pass | `tests/bots.rs` | no hidden card names or forbidden methods |
| candidate rankings | not applicable | no public candidate ranking | future Level 2 must add tests |
| dev inspector/public boundary | pass | e2e no-leak scan | full internal state not public |

## Pairwise No-Leak And Outcome Gate

| Check | Status | Evidence/test | Notes |
|---|---|---|---|
| pairwise source-seat private datum by viewer | pass | Rust visibility tests plus browser canary scan | Four-seat hidden hands and pass selection/provenance. |
| per-seat outcome explanation complete | pass | [UI.md](UI.md), `check-outcome-explanations.mjs` | Final scores and terminal winner are public. |
| team outcome explanation | not applicable | no teams | Gate 18 owns partnership pressure later. |
| no-reveal terminal outcome | not applicable | terminal scoring uses completed public hand/score facts | No folded/no-reveal terminal. |

## Replay/Export Safety

| Check | Status | Evidence/notes |
|---|---|---|
| replay reproduces deterministic hashes | pass | `cargo test -p briar_circuit --test replay`, replay-check series evidence |
| exported replay is viewer-scoped | pass | `export_viewer_timeline`, WASM public export |
| hidden-info redaction verified | pass | export tests and e2e scans |
| multi-seat export/import verified | pass | observer plus seat-private classes |
| import validates versions/schema | pass | replay header/import tests |
| replay UI viewer-safe | pass | Briar e2e replay/export/import smoke |
| golden traces not silently updated | pass | fixture/replay checks in series evidence |

## UI Polish, Legal-Only, Accessibility

| Check | Status | Evidence/notes |
|---|---|---|
| play-first public screen | pass | `BriarCircuitBoard.tsx` |
| visual target neutral/original | pass | no casino/trade-dress framing |
| legal moves derive from Rust action tree | pass | card buttons keyed by action leaves |
| TypeScript does not decide legality | pass | UI maps action paths only |
| pass compound flow uses Rust leaves | pass | confirm appears from action tree |
| stale/invalid submissions return Rust diagnostics | pass | WASM/app flow |
| keyboard path for core actions | pass | e2e pass-selection smoke |
| visible/accessibility labels | pass | card labels and status regions |
| reduced motion | pass | e2e reduced-motion smoke |
| responsive layout | pass | e2e responsive table check |

## Bot Explanation Safety

| Check | Status | Evidence/notes |
|---|---|---|
| public bot uses legal action API | pass | [AI.md](AI.md), bot tests |
| bot does not access forbidden hidden information | pass | opponent-hidden mutation test |
| Level 2 evidence pack complete if Level 2 ships | not applicable | Level 2 not admitted |
| no MCTS/ISMCTS/Monte Carlo/ML/RL | pass | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| public explanations viewer-safe | pass | bot explanation tests |
| candidate rankings dev-only/redacted | not applicable | no candidate ranking ships |
| bot latency acceptable | pass | [BENCHMARKS.md](BENCHMARKS.md) |

## Tests, Traces, Simulations, Replay, Serialization, Benchmarks

| Evidence | Status | Notes |
|---|---|---|
| unit/rule/property tests | pass | `cargo test -p briar_circuit` |
| golden traces | pass | `tests/golden_traces`, fixture/replay checks in series evidence |
| simulation/fuzz runs | pass | simulation lane run in earlier tickets / capstone rerun |
| replay/hash tests | pass | `cargo test -p briar_circuit --test replay` |
| serialization tests | pass | `cargo test -p briar_circuit --test serialization` |
| visibility/no-leak tests | pass | Rust, WASM, and browser no-leak evidence |
| bot legality/determinism/explanation tests | pass | `cargo test -p briar_circuit --test bots` |
| UI smoke/e2e | pass | `npm --prefix apps/web run smoke:ui`, `smoke:e2e`, Briar e2e |
| native benchmarks | pass | `cargo bench -p briar_circuit`, `bench-report` |
| docs/checkers | pass | doc links, outcome explanations, player rules/catalog checks in series evidence |

## Public Release Decision

Decision: release with explicit constraints after capstone closeout

Decision rationale:

- Briar Circuit has original rules/source docs, Rust-owned rules, hidden-info
  no-leak evidence, viewer-scoped replay/export, web renderer smoke, native
  benchmarks, and bot safety docs.
- Level 2 is not part of the release claim.
- The series capstone still owns central atlas/spec/index closeout and final
  full-gate evidence aggregation.

Release constraints:

- Public copy must use Briar Circuit as the product name.
- No public Level 2 competence claim.
- No private licensed content, copied card art, or trade dress.
- No hidden hands/pass provenance/deck material in browser-facing artifacts.

## Blocking Issues

| Issue | Required fix | Owner | Blocks release? |
|---|---|---|---:|
| central atlas/spec closeout pending | complete GAT16BRICIRTRI-018 | Rulepath | yes for gate finalization |

## Human Review Notes

| Reviewer | Area | Notes | Date |
|---|---|---|---|
| pending human | IP/UI/accessibility/release | Automated/docs evidence is complete for this ticket; human release review may still inspect final public build. | 2026-06-21 |
