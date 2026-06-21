# Vow Tide Public Release Checklist

Game ID: `vow_tide`

Public display name: `Vow Tide`

Implemented variant: `vow_tide_standard`

Release target: public web build / portfolio demo

Rules version: `vow-tide-rules-v1`

Data/manifest version: `vow-tide-data-v1`

Engine version: `engine-core-0.1.0`

Date: 2026-06-21

## Public Shipment Rule

If content ships to an unauthorized browser, it has shipped. Vow Tide must not
expose private hands, undealt-stock identities/order, future shuffled cards,
unsubmitted bids, hidden bot inputs, seed-derived future state, or private replay
facts in public files, browser payloads, DOM, storage, logs, exports, traces, or
tests.

## Official-Game Contract Status

| Requirement | Status | Evidence/notes |
|---|---|---|
| sources complete | pass | [SOURCES.md](SOURCES.md) |
| rules complete with stable IDs | pass | [RULES.md](RULES.md) |
| player-facing rules complete | pass | [HOW-TO-PLAY.md](HOW-TO-PLAY.md), `apps/web/public/rules/vow_tide.md` |
| rule coverage complete | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md); `rule-coverage` passes |
| mechanics inventory complete | pass | [MECHANICS.md](MECHANICS.md) |
| implementation admission receipt complete | pass | [GAME-IMPLEMENTATION-ADMISSION.md](GAME-IMPLEMENTATION-ADMISSION.md) |
| competent-player analysis complete | pass with constraint | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md); does not admit L2 |
| bot strategy evidence pack complete | not applicable for shipped bot | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) records `L2 not admitted` |
| AI registry complete | pass | [AI.md](AI.md) |
| UI doc complete | pass | [UI.md](UI.md) |
| benchmarks complete | pass | [BENCHMARKS.md](BENCHMARKS.md), `cargo bench -p vow_tide` |
| primitive-pressure ledger complete | pass | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md); central atlas no-debt confirmation recorded at Gate 17 closeout |
| supported player-count smoke | pass | 3, 4, 5, 6, and 7 seats accepted; other counts rejected by Rust setup |
| pairwise no-leak matrix | pass | Rust visibility tests, WASM tests, replay export tests, and browser smoke |
| per-seat outcome explanation | pass | [UI.md](UI.md), `check-outcome-explanations.mjs` |

## Rule, Source, And IP Status

| Check | Status | Evidence/notes |
|---|---|---|
| public rules prose is original | pass | [RULES.md](RULES.md), [HOW-TO-PLAY.md](HOW-TO-PLAY.md) |
| sources recorded with dates and quality | pass | [SOURCES.md](SOURCES.md) |
| variant and deviations clear | pass | `vow_tide_standard`; exact-or-zero scoring, immutable bids, fixed schedule, and fixed terminal are documented choices |
| no copied rulebook prose | pass | source-use statement records consulted-not-copied posture |
| no copied card/component text | pass | standard card facts only; labels and explanations are authored locally |
| no copied icons/art/screenshots/fonts/trade dress | pass | original catalog icon/UI styling; no bundled font files |
| public name risk reviewed | pass | `Vow Tide` is the product name; `Oh Hell` appears only as rules-family/source context |
| private licensed content excluded | pass | no private content involved |

## Hidden-Information No-Leak Surfaces

| Surface | Status | Evidence/test | Notes |
|---|---|---|---|
| public/browser payload | pass | `crates/wasm-api` tests, Vow e2e smoke | observer has no `own_hand` or stock identity |
| public view | pass | Rust visibility tests | public bids, trump indicator, trick plays, scores, and standings only |
| action tree | pass | native/WASM action tests | actor-only private card leaves; bid leaves from Rust |
| diagnostics | pass | rule/visibility tests | stable public error codes only |
| effect logs | pass | visibility/effect filtering tests | private deal/card effects seat-scoped |
| command logs | pass | viewer-scoped replay export tests | unauthorized private facts redacted |
| DOM/test IDs | pass | `vow-tide.smoke.mjs` canary scan | no private card tokens in public selectors |
| console/storage | pass | `vow-tide.smoke.mjs` | no forbidden hidden terms |
| replay export/import | pass | replay tests and browser smoke | observer plus every seat projection supported |
| bot explanations | pass | bot tests and AI docs | no opponent cards, stock identities, or forbidden methods |
| candidate rankings | not applicable | no public candidate ranking ships | future Level 2 must add evidence |
| dev inspector/public boundary | pass | browser smoke and WASM boundary tests | full internal state not public |

## Tests, Traces, Simulations, Replay, Serialization, Benchmarks

| Evidence | Status | Notes |
|---|---|---|
| workspace tests | pass | `cargo test --workspace` in Gate 17 closeout evidence |
| game unit/rule/property tests | pass | `cargo test -p vow_tide` covered by workspace suite |
| fixture checks | pass | `cargo run -p fixture-check -- --game vow_tide` |
| replay/hash checks | pass | `cargo run -p replay-check -- --game vow_tide --all` |
| rule coverage | pass | `cargo run -p rule-coverage -- --game vow_tide` |
| simulations | pass | `simulate` matrix for 3 through 7 seats |
| helper conformance baselines | pass | Plain Tricks and Briar Circuit replay checks pass after `game-stdlib::trick_taking` conformance |
| native benchmarks | pass | `cargo bench -p vow_tide`; complete seeded matches stay above the public floor for every supported seat count |
| WASM/web build | pass | `npm --prefix apps/web run build` |
| browser e2e | pass | `npm --prefix apps/web run smoke:e2e`, including Vow Tide smoke |
| docs/checkers | pass | `node scripts/check-catalog-docs.mjs`, `node scripts/check-doc-links.mjs`, `node scripts/check-outcome-explanations.mjs` |

## Public Release Decision

Decision: release-ready for Gate 17 public preview.

Vow Tide has the required official-game docs, Rust implementation evidence,
tool registration, WASM registration, public rules copy, browser renderer, e2e
no-leak smoke, catalog docs, bot-boundary proof, primitive-pressure closeout,
same-gate helper conformance for prior trick-taking games, replay evidence,
benchmark evidence, and neutral original presentation for public preview.

Release constraints:

- Public copy must use Vow Tide as the product name.
- `Oh Hell` may appear only as a rules-family/source label.
- No public Level 2 competence claim.
- No private licensed content, copied card art, copied prose, screenshots, font
  files, or trade dress.
- No private hands, undealt-stock material, future choices, or seed-derived
  future card identities in browser-facing artifacts.

## Blocking Issues

| Issue | Required fix | Owner | Blocks release? |
|---|---|---|---:|
| none | no blocking release issue remains after GAT17VOWTIDOHHEL-022 closeout | Rulepath | no |

## Human Review Notes

| Reviewer | Area | Notes | Date |
|---|---|---|---|
| pending human | IP/UI/accessibility/release | Automated/docs evidence is complete for Gate 17; a human maintainer may still inspect the final public build before external publication. | 2026-06-21 |
