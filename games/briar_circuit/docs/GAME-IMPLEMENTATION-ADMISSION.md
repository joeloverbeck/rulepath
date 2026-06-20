# Briar Circuit Implementation Admission

Game ID: `briar_circuit`

Public display name: `Briar Circuit`

Implemented variant: `briar_circuit_standard`

Rules version: `briar-circuit-rules-v1`

Roadmap stage/gate: Stage 16 / Gate 16

Last updated: 2026-06-21

## Admission Summary

Briar Circuit is admitted as the Gate 16 fixed-four-seat Hearts-family
trick-taking game. Implementation must stay game-local Rust with Rust/WASM
browser presentation. It proves deterministic full-deck dealing, simultaneous
private passing, lead/follow trick play, penalty scoring, shoot-the-moon
transformation, multi-hand threshold play, public observer projection,
seat-private replay, pairwise no-leak, Level 0 and bounded Level 1 bots, and a
polished web renderer.

Admission covers only `briar_circuit_standard`. It does not admit variable
seats, teams, partnership rules, alternate Hearts variants, generic card/trick
helpers, kernel vocabulary growth, TypeScript legality, or solver/search/ML
bots.

## Required Evidence

| Area | Evidence |
|---|---|
| rules and validation | [RULES.md](RULES.md), `games/briar_circuit/docs/RULE-COVERAGE.md`, `cargo test -p briar_circuit --test rules` |
| source/IP review | [SOURCES.md](SOURCES.md) |
| mechanic inventory | `games/briar_circuit/docs/MECHANICS.md` |
| primitive-pressure ledger | `games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md`, `games/plain_tricks/docs/MECHANICS.md`, `docs/MECHANIC-ATLAS.md` |
| static data and fixtures | `games/briar_circuit/data/*`, `cargo run -p fixture-check -- --game briar_circuit` |
| replay/golden traces | `games/briar_circuit/tests/golden_traces/*`, `cargo run -p replay-check -- --game briar_circuit --all` |
| simulation | `cargo run -p simulate -- --game briar_circuit --seat-count 4 --games 1000 --start-seed 1600 --action-cap 4096` |
| rule coverage | `cargo run -p rule-coverage -- --game briar_circuit` |
| bot policy | `games/briar_circuit/docs/AI.md`, `games/briar_circuit/docs/BOT-STRATEGY-EVIDENCE-PACK.md`, `games/briar_circuit/docs/COMPETENT-PLAYER.md`, `cargo test -p briar_circuit --test bots` |
| WASM registration | `cargo test -p wasm-api`, `npm --prefix apps/web run smoke:wasm` |
| web board and no-leak/a11y | `games/briar_circuit/docs/UI.md`, `BriarCircuitBoard.tsx`, `node apps/web/e2e/briar-circuit.smoke.mjs` |
| benchmarks | `games/briar_circuit/docs/BENCHMARKS.md`, `cargo bench -p briar_circuit` |

## Prerequisite Readiness

| Prerequisite | Path | Complete? | Notes/blockers |
|---|---|---:|---|
| source/IP notes | [SOURCES.md](SOURCES.md) | yes | Sources are consulted for public rules-family facts only; no copied prose/assets. |
| original rules with stable rule IDs | [RULES.md](RULES.md) | yes | `BC-*` IDs cover setup, deal, pass, play, trick, scoring, match, visibility, replay, bots, UI, outcome, variants, and out-of-scope boundaries. |
| rule coverage matrix | [RULE-COVERAGE.md](RULE-COVERAGE.md) | initial | This ticket creates the skeleton; implementation tickets replace planned/open rows with concrete proof. |
| mechanic inventory | `games/briar_circuit/docs/MECHANICS.md` | no | Trailing game doc; final form lands after behavior and UI evidence exist. |
| primitive-pressure ledger | `games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md` | no | Required before gameplay implementation; Gate 16 is the second close trick-taking use. |
| competent-player analysis | `games/briar_circuit/docs/COMPETENT-PLAYER.md` | no | Required for bot evidence posture; L2 is not admitted. |
| ADR, if boundary-changing | not applicable now | yes | No kernel, schema, DSL, or architecture-changing exception is admitted. |

## Source And IP Readiness

| Check | Status | Evidence/notes |
|---|---|
| consulted sources recorded with dates | ready | `SOURCES.md` records project authority, Pagat, Bicycle, OpenSpiel prior-art, and accessibility references. |
| sources used only for verification/context | ready | Sources are consulted-not-copied. |
| Rulepath rules prose is original | ready | `RULES.md` uses original Briar Circuit wording. |
| no copied card/component text | ready | Public name and planned card labels/IDs are Rulepath-authored over common card facts. |
| no copied art/icons/screenshots/scans/fonts/trade dress | ready | No assets land in these docs; later renderer/icon work must keep asset provenance. |
| public naming rationale recorded | ready | `SOURCES.md` records **Briar Circuit** as the neutral public identity. |
| private licensed content excluded from public files | ready | No private licensed content is involved. |
| human/legal review triggers cleared or recorded | constrained | Later card/icon/table assets require review; no current blocker. |

## Rule-ID Readiness

| Check | Status | Evidence/notes |
|---|---|
| every planned rule has a stable rule ID | ready | `RULES.md` creates the initial canonical `BC-*` set. |
| rule IDs use stable prefixes, not section-only references | ready | Prefixes include `BC-SETUP`, `BC-DEAL`, `BC-PASS`, `BC-PLAY`, `BC-TRICK`, `BC-SCORE`, `BC-MATCH`, `BC-VIS`, `BC-REPLAY`, `BC-BOT`, `BC-UI`, `BC-OUTCOME`, `BC-VAR`, and `BC-OOS`. |
| ambiguities have chosen resolutions and IDs | ready | `SOURCES.md` records `BC-AMB-001` through `BC-AMB-006`. |
| out-of-scope variants are explicit | ready | `BC-OOS-001` through `BC-OOS-004`. |
| rule-ID migration policy is understood | ready | Migration note exists in `RULES.md`; later coverage/traces must follow it. |

## Admission Constraints

- No `engine-core` card, deck, hand, suit, rank, pass, trick, heart, queen,
  moon, scoring, or bot-policy nouns.
- No Gate 16 `game-stdlib` trick/card helper. The second-use trick-taking
  decision must be recorded as keep-local/defer before gameplay implementation.
- No YAML, DSL, selectors, formulas, bot policy, hidden-info routing, or
  behavior-looking keys in static data.
- No TypeScript legality, validation, pass completion, follow-suit check,
  hearts-broken check, trick-winner logic, scoring, terminal detection, replay
  authority, hidden-info filtering, or bot policy.
- No unplayed card identity, pass selection, incoming card before exchange,
  pass provenance, deck order, hidden-state inference, or seed-reconstructable
  setup data in unauthorized payloads, DOM, attributes, test IDs, storage, logs,
  dev panels, replay exports, bot rationales, or candidate rankings.
- No MCTS, ISMCTS, Monte Carlo rollouts/dealing/sampling, ML, RL, runtime LLM
  bot, hidden-state sampling, or opponent-card enumeration.
- No Hearts product branding, copied rules prose, copied card imagery,
  conventional trade-dress dependency, casino framing, or affiliation wording.

## Primitive-Pressure Status

| Mechanic shape | Status | Decision/evidence | Blocks implementation? |
|---|---|---|---:|
| follow-suit legality | second-use comparison required | Plain Tricks is first official use; Briar Circuit is the second close use with materially different pass, point, and multi-hand rules. | yes before gameplay implementation |
| led-suit trick comparator | second-use comparison required | Same core shape as Plain Tricks, with four cards and a standard deck. | yes before gameplay implementation |
| trick-winner-leads turn order | second-use comparison required | Repeated from Plain Tricks, but embedded in a 13-trick hand and threshold match. | yes before gameplay implementation |
| deal rotation / trick-round redeal | second-use comparison required | Repeated multi-round setup shape with different dealer/pass-cycle semantics. | yes before gameplay implementation |
| private hands and pairwise no-leak | repeated hidden-info pressure | Existing hidden-info official games prove the general firewall; Briar Circuit extends it to four ordered seat pairs and pass commitments. | no for docs; yes for release proof |

## Mechanic-Atlas Admission Check

`docs/MECHANIC-ATLAS.md` currently records no open promotion debt after Gate
15.1 closeout. Gate 16 must still record the Plain Tricks/Briar Circuit
second-use trick-taking comparison and keep-local/defer decision before
gameplay implementation proceeds.

## Engine-Core Contamination Review

| Boundary check | Result | Notes |
|---|---|---|
| no game noun needs to enter `engine-core` | pass | All card/pass/trick/scoring nouns stay inside `games/briar_circuit`. |
| no rule helper needs to enter `engine-core` | pass | Existing generic action-tree, command, effect, visibility, replay, seat, and RNG contracts are sufficient. |
| no private licensed name/data needs to enter kernel contracts | pass | No private licensed content is involved. |
| any generic contract change has ADR or explicit non-goal | pass | No generic contract change is admitted. |

## Static-Data Behavior Review

| Check | Result | Notes |
|---|---|---|
| static data limited to typed content, parameters, metadata, fixtures, traces, or reports | pass | Manifest, variants, card labels, fixtures, traces, and reports are allowed. |
| no selectors, rule branches, loops, triggers, tactical conditions, or exception logic in data | pass | Pass routing, legality, scoring, visibility, and bots are Rust behavior. |
| no YAML by default | pass | Gate 16 uses TOML/JSON/Rust-shaped evidence patterns only. |
| no DSL at project start | pass | No DSL is admitted. |

## Hidden-Information Risk Review

| Surface | Risk level | Required safeguard/test | Admission status |
|---|---:|---|---|
| public/browser payload | high | Rust observer/seat projection tests, WASM no-leak harness, DOM canary scan. | ready |
| action tree | high | Actor-only card leaves; non-actors/observer receive no private alternatives. | ready |
| preview | high | Owner-only preview facts; no opponent hand, pass provenance, or future deck facts. | ready |
| diagnostics/effect log | high | Viewer-safe diagnostics and public/private effect envelopes. | ready |
| DOM/test IDs/local storage/replay export | high | Browser e2e canary scan and viewer-scoped export tests. | ready |
| bot explanations/candidate rankings | high | Bot input audit and explanation redaction tests. | ready |
| dev inspector | high | Public-summary whitelist and no hidden canaries in dev surfaces. | ready |

## Seat, Projection, And Topology Admission

| Check | Status | Evidence/notes |
|---|---|---|
| min/max seats accepted by setup are explicit | ready | Exactly four seats. |
| unsupported seat counts reject with viewer-safe diagnostics | ready | `BC-SETUP-001` requires stable diagnostics for every non-four count. |
| official seat IDs and stable seat labels are defined | ready | `seat_0` through `seat_3`; player-facing copy avoids internal IDs where possible. |
| per-viewer projection proof plan exists | ready | Observer plus four seat-private projections. |
| pairwise no-leak proof plan exists for every source seat private datum and viewer surface | ready | 12 ordered seat pairs plus observer checks; pass/deck/bot/export categories included. |
| topology/object-count inventory covers all public and private objects | constrained | Full MECHANICS.md lands later; admission records 52 cards, 13-card hands, 13 tricks, 26 points, four seats. |
| mechanic-atlas interlock status is ready for N-seat/topology/evaluator/shared-accounting pressure | ready | Trick-taking second-use comparison blocks gameplay until recorded; no open debt otherwise. |

## Bot Level Required For This Stage

| Level | Required before public release? | Coding may start now? | Evidence required |
|---:|---:|---:|---|
| 0 random legal | yes | yes after legal tree exists | Legal action API, deterministic seed, simulation, explanation redaction. |
| 1 baseline | yes | yes after legal tree/view exists | Own-hand/public-history heuristics, legality, determinism, no-leak explanations. |
| 2 authored policy | no | no | Later L2 requires completed evidence pack and separate admission. |
| 3 shallow deterministic search | no | no | Not admitted for this hidden-information game. |

Public v1/v2 bots must not use MCTS, ISMCTS, Monte Carlo-style bots, ML, RL,
runtime LLM policy, hidden-state sampling, or opponent-hidden-card enumeration.

## UI Exposure Expectations

| Check | Status | Notes |
|---|---|---|
| web-exposed in this stage? | yes | Gate 16 requires a public renderer, observer/seat-private replay, and e2e smoke. |
| React + SVG default accepted | yes | Presentation must be original, neutral, accessible, and not casino/trade-dress-driven. |
| legal-action tree maps to UI controls | ready | Browser controls must render only Rust-supplied pass/play leaves. |
| TypeScript presentation-only boundary understood | ready | TypeScript must not compute legality, trick resolution, scoring, or outcome. |
| effect-driven animation expectations identified | ready | Deal/pass/play/trick/score/moon/terminal feedback comes from semantic effects. |
| accessibility/reduced-motion/responsive expectations identified | ready | Browser smoke must cover keyboard, screen-reader names, reduced motion, and no-leak text/attributes. |

## Benchmark Expectations

| Operation | Needed before implementation? | Needed before release? | Notes |
|---|---:|---:|---|
| setup/deal | no | yes | Benchmark fixed-four setup, shuffle, deal, and serialization. |
| legal action generation | no | yes | Benchmark pass selection and play legality cases. |
| preview | no | yes | Benchmark authorized preview where implemented. |
| validation/apply action | no | yes | Benchmark pass confirm/exchange, play apply, trick resolution, scoring, and terminal paths. |
| public/private view generation | no | yes | Benchmark observer plus four seat projections. |
| effect filtering | no | yes | Include private deal/pass effects and public play/trick/score effects. |
| serialization/deserialization | no | yes | Internal trace and viewer-scoped export paths need coverage. |
| replay throughput/hash | no | yes | Replay-check and golden traces are release evidence. |
| random playout throughput | no | yes | 1,000 seeded four-seat matches are required. |
| bot decision latency | no | yes | L0 and L1 bot simulations must terminate under cap. |
| WASM/browser smoke | no | yes | `smoke:wasm`, `smoke:ui`, and `smoke:e2e` close browser surface. |

## Admission Decision

Decision: admitted with explicit constraints

Decision rationale:

- The Gate 16 spec defines a bounded, original, neutral, fixed-four-seat
  game with no kernel concept changes.
- `docs/MECHANIC-ATLAS.md` currently records no open promotion debt, but the
  trick-taking second-use comparison must land before gameplay implementation.
- The initial `RULES.md`, `SOURCES.md`, `HOW-TO-PLAY.md`, and
  `RULE-COVERAGE.md` establish stable rules and proof expectations before code.

Explicit constraints:

- Public-facing surfaces use **Briar Circuit** and original neutral copy.
- `briar_circuit` remains the internal id for code, roadmap, tool, WASM, and
  web registration.
- Documentation admission alone does not mark Gate 16 complete.

## Blocking Issues

| Issue | Required fix | Owner | Blocks coding? |
|---|---|---|---:|
| trick-taking second-use comparison not yet recorded | Complete GAT16BRICIRTRI-003 before gameplay implementation. | Rulepath | yes |

## Sign-Off

Prepared by: Codex

Reviewed by: pending maintainer review

Date: 2026-06-21
