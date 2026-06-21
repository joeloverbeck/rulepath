# Vow Tide Implementation Admission

Game ID: `vow_tide`

Public display name: `Vow Tide`

Implemented variant: `vow_tide_standard`

Rules version: `vow-tide-rules-v1`

Roadmap stage/gate: Stage 17 / Gate 17

Public role: hidden-info proof / public showcase candidate / variable-N exact-bid trick-taking gate

Prepared by: Codex

Date: 2026-06-21

## Admission Summary

Vow Tide is admitted as the Gate 17 variable 3-7-seat exact-bid
trick-taking game. It proves changing hand sizes, public sequential contracts,
the dealer-last hook, trump-aware trick resolution, fixed-schedule scoring,
public observer plus every seat-private view, viewer-scoped replay, L0 and
bounded L1 bots, simulation and benchmark evidence by seat count, and a
browser-playable renderer.

Admission covers only `vow_tide_standard`. It does not admit alternate
schedules, no-trump variants, bid changes, teams, partnerships, generic bidding
frameworks, broad card/deck/trick engines, kernel vocabulary growth, TypeScript
legality, or solver/search/ML bots.

## Required Evidence

| Area | Evidence |
|---|---|
| rules and validation | [RULES.md](RULES.md), `RULE-COVERAGE.md`, `cargo test -p vow_tide --test rules` |
| source/IP review | [SOURCES.md](SOURCES.md), final public-release checklist |
| mechanic inventory | `MECHANICS.md`, `PRIMITIVE-PRESSURE-LEDGER.md`, `docs/MECHANIC-ATLAS.md` |
| promoted helper conformance | `crates/game-stdlib/src/trick_taking.rs`, `cargo test -p game-stdlib`, `cargo test -p plain_tricks`, `cargo test -p briar_circuit` |
| static data and fixtures | `games/vow_tide/data/*`, `cargo run -p fixture-check -- --game vow_tide` |
| replay/golden traces | `games/vow_tide/tests/golden_traces/*`, `cargo run -p replay-check -- --game vow_tide --all` |
| simulation | five 1,000-game runs for seat counts 3, 4, 5, 6, and 7 |
| rule coverage | `cargo run -p rule-coverage -- --game vow_tide` |
| bot policy | `AI.md`, `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `cargo test -p vow_tide --test bots` |
| WASM registration | `cargo test -p wasm-api`, `npm --prefix apps/web run smoke:wasm` |
| web board and no-leak/a11y | `UI.md`, `VowTideBoard.tsx`, `apps/web/e2e/vow-tide.smoke.mjs`, smoke and DOM/storage/log canary checks |
| benchmarks | `BENCHMARKS.md`, `cargo bench -p game-stdlib`, `cargo bench -p plain_tricks`, `cargo bench -p briar_circuit`, `cargo bench -p vow_tide` |

## Prerequisite Readiness

| Prerequisite | Path | Complete? | Notes/blockers |
|---|---|---:|---|
| source/IP notes | [SOURCES.md](SOURCES.md) | initial | Rules-family facts, deviations, neutral naming, and release blockers are recorded; final asset/IP review remains open. |
| original rules with stable rule IDs | [RULES.md](RULES.md) | yes | `VT-*` IDs cover identity, seats, cards, schedule, deal, trump, bidding, play, scoring, visibility, replay, bots, outcome, and boundaries. |
| rule coverage matrix | `RULE-COVERAGE.md` | no | Ticket 016 must create the one-row-per-rule evidence matrix. |
| mechanic inventory | `MECHANICS.md` | no | Ticket 021 completes trailing mechanics documentation. |
| primitive-pressure ledger, if needed | `PRIMITIVE-PRESSURE-LEDGER.md` | no | Tickets 002-004 must resolve the third-use trick-taking hard gate before Vow Tide trick behavior. |
| competent-player analysis | `COMPETENT-PLAYER.md` | no | Ticket 014 completes strategy posture; L2 remains not admitted. |
| ADR, if boundary-changing | not applicable now | yes | No kernel, schema, data-language, visibility-taxonomy, or public API exception is admitted. |

## Source And IP Readiness

| Check | Status | Evidence/notes |
|---|---|
| consulted sources recorded with dates | ready | `SOURCES.md` records Pagat, Trickster, prior-art implementation references, project spec, and IP policy. |
| sources used only for verification/context | ready | Sources are summarized, not copied. |
| Rulepath rules prose is original | ready | `RULES.md` uses original Vow Tide wording. |
| no copied card/component text | ready | Ordinary rank/suit names are common component labels; no protected source text is copied. |
| no copied art/icons/screenshots/scans/fonts/trade dress | constrained | No assets land yet; renderer/icon work must record provenance and pass final review. |
| public naming rationale recorded | ready | `SOURCES.md` records Vow Tide as the neutral public identity. |
| private licensed content excluded from public files | ready | No private licensed content is involved. |
| human/legal review triggers cleared or recorded | constrained | Final public icon/table/card assets and public-release review remain blockers before release. |

## Rule-ID Readiness

| Check | Status | Evidence/notes |
|---|---|
| every implemented rule has a stable rule ID | ready | `RULES.md` creates the canonical Gate 17 `VT-*` set. |
| rule IDs use stable prefixes, not section-only references | ready | Prefixes include `VT-IDENTITY`, `VT-SEATS`, `VT-CARDS`, `VT-SCHEDULE`, `VT-BID`, `VT-HOOK`, `VT-FOLLOW`, `VT-SCORE`, `VT-VIEW`, `VT-REPLAY`, `VT-BOT`, `VT-OUTCOME`, and `VT-BOUNDARY`. |
| ambiguities have chosen resolutions and IDs | ready | `SOURCES.md` records `VT-AMB-001` through `VT-AMB-005`; `RULES.md` records chosen resolutions. |
| out-of-scope variants are explicit | ready | `RULES.md` names excluded schedules, trump methods, bid variants, scoring variants, teams, search bots, and generic frameworks. |
| rule-ID migration policy is understood | ready | Migration notes in `RULES.md` require synchronized coverage/trace/test/doc updates. |

## Rule Coverage Readiness

| Check | Status | Evidence/notes |
|---|---|
| coverage matrix has one row per rule ID | blocked for release | Ticket 016 owns `RULE-COVERAGE.md`. |
| terminal result coverage is complete for every viewer class | blocked for release | Tickets 009-011 and 016 must prove scoring, terminal, no-leak, replay, and outcome coverage. |
| deferred/unsupported/not applicable rows are explicit | constrained | Out-of-scope rules are explicit in `RULES.md`; coverage rows land later. |
| primary Rust test strategy is identified | ready | Spec and tickets require rules, property, replay, serialization, visibility, bots, fixture, coverage, simulation, and WASM tests. |
| golden trace needs are identified | ready | Gate 17 spec §7.6 names the minimum trace inventory. |
| invalid/stale diagnostic trace needs are identified | ready | `RULES.md` lists the diagnostic code floor. |
| replay/hash requirements are identified | ready | `VT-REPLAY-001` and spec require Trace Schema v1 and viewer-scoped exports. |
| serialization requirements are identified | ready | Stable JSON/trace/view/effect/action/bot serialization evidence required by spec. |
| visibility/no-leak requirements are identified | ready | `VT-VIEW-001`, spec viewer matrix, and N-seat pairwise proof are mandatory. |
| UI smoke coverage is scoped as smoke only | ready | Browser e2e samples UI roles; exhaustive pairwise proof remains Rust/WASM authority. |

## Mechanic Inventory Readiness

| Check | Status | Evidence/notes |
|---|---|
| all mechanic atlas categories are inventoried | blocked for release | Ticket 021 owns final `MECHANICS.md`. |
| min/max seats and stable seat labels are recorded | ready | 3-7, default 4, `seat_0` through `seat_6`, `Tide 1` through `Tide 7`. |
| wrong-seat-count diagnostics are viewer-safe and identified | ready | `VT-SEATS-001` and diagnostic floor require stable safe rejection. |
| topology/object-count inventory is complete | constrained | Rules record 52 cards, up to 7 seats, K schedule, public trick and hidden stock; final mechanics/bench docs land later. |
| local mechanics are named and scoped | ready | Bidding/contracts, schedule/deal, scoring, visibility, bots, and outcome remain game-local. |
| reused primitives are justified | constrained | Trick-selection/comparator helper promotion must land in tickets 002-004. |
| repeated-shape comparison is complete | constrained | Gate 17 spec contains the decision; repo/game ledgers must be updated before trick behavior. |
| third-use hard gate is cleared when applicable | blocked before Vow Tide trick behavior | Tickets 002-004 must promote and back-port the helper with no open debt. |
| atlas interlock status is recorded for next-phase scaling pressure | constrained | `docs/MECHANIC-ATLAS.md` must be updated in ticket 002 and final closeout. |
| repo atlas update required? | yes | Trick-taking rows and numeric contract first-use row must update. |

## Primitive-Pressure Status

| Mechanic shape | Status | Decision/evidence | Blocks implementation? |
|---|---|---|---:|
| follow-suit legality | extraction required at Gate 17 | Plain Tricks and Briar Circuit are prior close uses; Gate 17 selects a narrow `game-stdlib` helper and back-port. | yes before Vow Tide trick behavior |
| trick resolution / led-suit comparator | extraction required at Gate 17 | Same helper promotion covers stable winning-index comparison with optional trump. | yes before Vow Tide trick behavior |
| trick-winner-leads turn order | reviewed anti-example | Repeated shape is acknowledged, but mutation/phase policy remains game-local. | no after ledger decision |
| deal rotation / trick-round redeal | reviewed anti-example | Repeated shape is acknowledged, but RNG/deal/schedule/dealer policy remains game-local. | no after ledger decision |
| numeric bid / contract-vs-result / dealer hook | local-only first official use | Vow Tide is first use; record local-only ledger and compare again at Gate 18 or another close use. | no after ledger entry |

## Engine-Core Contamination Review

| Boundary check | Result | Notes |
|---|---|---|
| no game noun needs to enter `engine-core` | pass | Card, deck, hand, suit, rank, trump, trick, bid, contract, dealer, schedule, and score stay in `games/*` or the narrow earned `game-stdlib` helper. |
| no rule helper needs to enter `engine-core` | pass | Existing generic action, command, effect, visibility, replay, seat, and RNG contracts are sufficient. |
| no private licensed name/data needs to enter kernel contracts | pass | No private licensed content is involved. |
| any generic contract change has ADR or explicit non-goal | pass | No generic contract change is admitted. |

## Static-Data Behavior Review

| Check | Result | Notes |
|---|---|---|
| static data limited to typed content, parameters, metadata, fixtures, traces, or reports | pass | Manifest/variants may carry identity, versions, seats, labels, and presentation metadata only. |
| no selectors, rule branches, loops, triggers, tactical conditions, or exception logic in data | pass | Schedule, hook, follow-suit, trump comparison, scoring, visibility, and bot policy remain Rust. |
| no YAML by default | pass | Gate 17 uses TOML/JSON/Rust-shaped evidence patterns only. |
| no DSL at project start | pass | No DSL is admitted. |

## Hidden-Information Risk Review

| Surface | Risk level | Required safeguard/test | Admission status |
|---|---:|---|---|
| public/browser payload | high | Rust observer/seat projections plus WASM/browser payload scans. | ready |
| action tree | high | Active-seat-only legal leaves; non-actors/observer receive no private choice alternatives. | ready |
| preview | high | Preview may use only viewer-safe facts; no hand/stock leak to unauthorized viewers. | ready |
| diagnostics/effect log | high | Viewer-filtered diagnostics/effects with canary tests. | ready |
| DOM/test IDs/local storage/replay export | high | Browser e2e canary scan, hotseat handoff removal, viewer-scoped export tests. | ready |
| bot explanations/candidate rankings | high | Bot input audit, explanation redaction, no hidden-state candidates. | ready |
| dev inspector | high | Explicit whitelist and no private canaries in dev surfaces. | ready |

## Seat, Projection, And Topology Admission

| Check | Status | Evidence/notes |
|---|---|---|
| min/max seats accepted by setup are explicit | ready | Minimum 3, maximum 7, default 4, supported set `{3,4,5,6,7}`. |
| unsupported seat counts reject with viewer-safe diagnostics | ready | `VT-SEATS-001` requires stable rejection for every other count. |
| official seat IDs and stable seat labels are defined | ready | `seat_0` through `seat_6`; fallback labels `Tide 1` through `Tide 7`. |
| per-viewer projection proof plan exists | ready | Observer plus every authorized seat viewer. |
| pairwise no-leak proof plan exists for every source seat private datum and viewer surface | ready | Gate 17 spec requires exhaustive N=3..7 source-to-unauthorized viewer matrix plus stock canaries. |
| topology/object-count inventory covers all public and private objects | constrained | 52 cards, max 7 seats, max 10-card hands, max 19 hands, hidden stock, current trick, bid rail, score/history; final mechanics/bench docs close this. |
| mechanic-atlas interlock status is ready for N-seat/topology/evaluator/shared-accounting pressure | constrained | Trick-taking hard gate is known; final atlas rows land in tickets 002 and 022. |

## Bot Level Required For This Stage

| Level | Required before public release? | Coding may start now? | Evidence required |
|---:|---:|---:|---|
| 0 random legal | yes | yes after legal tree exists | Authorized legal action API, deterministic bot seed, simulations, minimal safe explanations. |
| 1 baseline | yes | yes after legal tree/view exists | Own-hand/public-fact heuristics, legality, determinism, no-leak explanations, latency evidence. |
| 2 authored policy | no | no | Future L2 requires completed strategy evidence pack and separate admission. |
| 3 shallow deterministic search | no | no | Not applicable to this hidden-information game. |

Public v1/v2 bots must not use MCTS, ISMCTS, Monte Carlo rollouts, hidden-world
sampling, determinization from actual state, ML, RL, external solvers, or
runtime LLM move selection.

## UI Exposure Expectations

| Check | Status | Notes |
|---|---|---|
| web-exposed in this stage? | yes | Gate 17 requires public renderer, observer/seat-private views, replay export, outcome explanation, and e2e smoke. |
| React + SVG default accepted | yes | Presentation must be original, neutral, responsive, accessible, and not casino/trade-dress-driven. |
| legal-action tree maps to UI controls | ready | Browser controls must render only Rust-supplied bid/play leaves. |
| TypeScript presentation-only boundary understood | ready | TypeScript must not compute hook filtering, follow-suit legality, trick winner, scoring, active/dealer facts, or standings. |
| effect-driven animation expectations identified | ready | Deal/trump/bid/play/trick/score/terminal feedback comes from semantic effects and the shared scheduler. |
| accessibility/reduced-motion/responsive expectations identified | ready | Browser smoke must cover keyboard, screen-reader names, reduced motion, hotseat handoff, observer, replay, and no-leak text/attributes. |

## Benchmark Expectations

| Operation | Needed before implementation? | Needed before release? | Notes |
|---|---:|---:|---|
| setup/schedule/deal | no | yes | Seat-count-specific operations for N=3..7. |
| legal action generation | no | yes | Bid first/middle/dealer-hook and play unconstrained/forced-follow/void. |
| preview | no | yes | Where implemented, preview remains viewer-safe. |
| validation/apply action | no | yes | Bid, play, trick capture, hand scoring, terminal paths. |
| public/private view generation | no | yes | Observer and every seat projection, especially max public/private surfaces. |
| effect filtering | no | yes | Public and seat-private effect streams. |
| serialization/deserialization | no | yes | State, views, effects, actions, bot outputs, fixtures, traces, exports. |
| replay throughput/hash | no | yes | Internal replay and viewer-scoped export/import. |
| random playout throughput | no | yes | 1,000-game simulations for every supported seat count. |
| bot decision latency | no | yes | L0 and L1 bid/play decisions. |
| WASM/browser smoke | no | yes | `smoke:wasm`, `smoke:ui`, `smoke:e2e`, and effect smoke. |

## Admission Decision

Decision: admitted with explicit constraints

Decision rationale:

- The Gate 17 spec defines a bounded, original, neutral public game that uses
  existing Rulepath architecture and no kernel/schema exception.
- The third-use trick-taking pressure is known and has a selected narrow
  helper/back-port path before Vow Tide trick behavior.
- The initial `RULES.md`, `SOURCES.md`, and this admission receipt establish
  stable rule IDs, source/IP posture, seat/visibility commitments, and
  boundary constraints before gameplay code starts.

Explicit constraints:

- Public-facing surfaces use **Vow Tide** and original neutral copy.
- `vow_tide` remains the internal id for code, tools, WASM, web registration,
  traces, and docs.
- Trick-taking helper promotion and Plain Tricks/Briar Circuit conformance must
  complete before Vow Tide trick-play behavior lands.
- Documentation admission alone does not mark Gate 17 complete.

## Blocking Issues

| Issue | Required fix | Owner | Blocks coding? |
|---|---|---|---:|
| Trick-taking third-use hard gate | Promote/back-port the narrow helper and update atlas/game ledgers in tickets 002-004. | Rulepath maintainers / agents | yes for Vow Tide trick behavior |
| Rule coverage matrix absent | Create `RULE-COVERAGE.md` with one row per `VT-*` ID in ticket 016. | Rulepath maintainers / agents | no for initial setup; yes for release |
| Final asset/IP release review open | Close after icon/renderer/public rules/assets exist in ticket 022. | Rulepath maintainers | no for code; yes for public release |

## Sign-off

Prepared by: Codex

Reviewed by: pending maintainer review

Date: 2026-06-21
