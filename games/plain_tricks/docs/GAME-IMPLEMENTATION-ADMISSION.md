# Plain Tricks Implementation Admission

Game ID: `plain_tricks`

Public display name: `Plain Tricks`

Implemented variant: `plain_tricks_standard`

Rules version: `plain-tricks-rules-v1`

Roadmap stage/gate: Stage 10 / Gate 10.1 trick-taking half

Last updated: 2026-06-09

## Admission Summary

Plain Tricks is admitted as the Gate 10.1 `plain_tricks` trick/follow-suit
proof game. The implementation must be game-local Rust with a Rust/WASM browser
presentation. It proves deterministic private-hand setup, must-follow-suit
legality, void free-discard, trick resolution, trick-winner-led turn order,
round scoring, deal rotation, viewer-scoped replay export, Level 0 and Level 2
bot policies, and browser no-leak behavior.

Admission covers only the `plain_tricks` trick-taking portion of Gate 10. The
`poker_lite` betting/showdown half is already complete; Gate 10 as a whole is
not done until this spec's full evidence and closeout pass.

## Required Evidence

| Area | Evidence |
|---|---|
| rules and validation | [RULES.md](RULES.md), `games/plain_tricks/docs/RULE-COVERAGE.md`, `cargo test -p plain_tricks --test rules` |
| source/IP review | [SOURCES.md](SOURCES.md) |
| mechanic inventory | `games/plain_tricks/docs/MECHANICS.md` |
| primitive-pressure ledger | `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md`, `docs/MECHANIC-ATLAS.md` §10B |
| static data and fixtures | `games/plain_tricks/data/*`, `cargo run -p fixture-check -- --game plain_tricks` |
| replay/golden traces | `games/plain_tricks/tests/golden_traces/*`, `cargo run -p replay-check -- --game plain_tricks --all` |
| simulation | `cargo run -p simulate -- --game plain_tricks --games 1000 --start-seed 0 --action-cap 32` |
| rule coverage | `cargo run -p rule-coverage -- --game plain_tricks` |
| bot policy | `games/plain_tricks/docs/AI.md`, `games/plain_tricks/docs/BOT-STRATEGY-EVIDENCE-PACK.md`, `games/plain_tricks/docs/COMPETENT-PLAYER.md`, `cargo test -p plain_tricks --test bots` |
| WASM registration | `cargo test -p wasm-api`, `npm --prefix apps/web run smoke:wasm` |
| web board and no-leak/a11y | `games/plain_tricks/docs/UI.md`, `PlainTricksBoard.tsx`, `node apps/web/e2e/plain-tricks.smoke.mjs` |
| benchmarks | `games/plain_tricks/docs/BENCHMARKS.md`, `cargo bench -p plain_tricks` |

## Prerequisite Readiness

| Prerequisite | Path | Complete? | Notes/blockers |
|---|---|---:|---|
| source/IP notes | [SOURCES.md](SOURCES.md) | yes | Consulted sources are context only; no copied prose/assets. |
| original rules with stable rule IDs | [RULES.md](RULES.md) | yes | `PT-*` rule IDs cover setup, actions, restrictions, trick resolution, scoring, terminal, visibility, replay, bots, variants, and out-of-scope boundaries. |
| rule coverage matrix | `games/plain_tricks/docs/RULE-COVERAGE.md` | no | Later ticket; must map every rule ID. |
| mechanic inventory | `games/plain_tricks/docs/MECHANICS.md` | no | Later ticket after implementation surfaces exist. |
| primitive-pressure ledger | `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md` | no | Next ticket; required before rules/setup implementation proceeds beyond crate skeleton work. |
| competent-player analysis | `games/plain_tricks/docs/COMPETENT-PLAYER.md` | no | Bot ticket; required before Level 2 bot coding. |
| ADR, if boundary-changing | not applicable now | yes | No new kernel concept or behavior-in-data path is admitted by this ticket. |

## Source and IP Readiness

| Check | Status | Evidence/notes |
|---|---|
| consulted sources recorded with dates | ready | `SOURCES.md` records Gate 10.1, project authority, and Pagat trick-taking context sources. |
| sources used only for verification/context | ready | Sources are consulted-not-copied proof-shape context. |
| Rulepath rules prose is original | ready | `RULES.md` uses original Plain Tricks terminology and rule statements. |
| no copied card/component text | ready | Card ids and labels are Rulepath-authored. |
| no copied art/icons/screenshots/scans/fonts/trade dress | ready | No assets land in this ticket; later UI must remain neutral. |
| public naming rationale recorded | ready | `SOURCES.md` records **Plain Tricks** as the neutral public name. |
| private licensed content excluded from public files | ready | No private licensed content is involved. |
| human/legal review triggers cleared or recorded | ready | No blocker identified; card-table presentation remains a release-audit concern. |

## Rule-ID Readiness

| Check | Status | Evidence/notes |
|---|---|
| every implemented rule has a stable rule ID | ready | `RULES.md` creates the initial `PT-*` canonical set. |
| rule IDs use stable prefixes, not section-only references | ready | Prefixes include `PT-COMP`, `PT-SETUP`, `PT-TURN`, `PT-ACT`, `PT-RESTRICT`, `PT-TRICK`, `PT-SCORE`, `PT-END`, `PT-VIS`, `PT-RNG`, `PT-BOT`, `PT-VAR`, `PT-OOS`, and `PT-AMB`. |
| ambiguities have chosen resolutions and IDs | ready | `PT-AMB-001` through `PT-AMB-005`. |
| out-of-scope variants are explicit | ready | `PT-OOS-001` through `PT-OOS-004`. |
| rule-ID migration policy is understood | ready | Migration note exists in `RULES.md`; later coverage/traces must follow it. |

## Admission Constraints

- No `engine-core` card, deck, hand, suit, rank, trick, lead, follow, void,
  deal, scoring, or bot-policy nouns.
- No `game-stdlib` card/deck/hand/shuffle/trick helper is promoted unless
  GAT101PLATRI-002 records that decision and its required back-port/debt
  evidence.
- No progression past crate-skeleton work before GAT101PLATRI-002 records the
  third-use primitive-pressure ledger decision.
- No YAML, DSL, selectors, formulas, bot policy, hidden-info routing, or
  behavior-looking keys in static data.
- No TypeScript legality, validation, follow-suit check, trick-winner logic,
  score calculation, deal rotation, terminal detection, replay authority,
  hidden-info filtering, or bot policy.
- No unplayed card identity, opponent hand, tail, hidden-state inference, or
  seed-reconstructable hidden setup data in public/browser payloads, DOM,
  attributes, test IDs, storage, logs, dev panel, public replay export, bot
  rationale, or candidate ranking before the rule-defined reveal point.
- No tail reveal, including at terminal.
- No MCTS, ISMCTS, Monte Carlo rollouts/dealing/sampling, ML, RL, runtime LLM
  bot, hidden-state sampling, opponent-card enumeration, or opponent-card
  peeking.
- No Whist/Hearts branding, copied rules prose, copied card imagery,
  conventional trade-dress dependency, or commercial product naming.

## Primitive-Pressure Status

| Mechanic shape | Status | Decision/evidence | Blocks implementation? |
|---|---|---|---:|
| deterministic shuffle / private hand / staged reveal | third-use hard gate | `high_card_duel` is first official use; `poker_lite` is second; `plain_tricks` repeats the shape with six-card hands and staged play-by-play reveal. GAT101PLATRI-002 must record the ledger decision in this game doc set and `docs/MECHANIC-ATLAS.md` before rules/setup implementation proceeds beyond crate skeleton work. | yes after skeleton |
| trick-count scoring vs public resource accounting | stance to confirm | This admission treats one-point-per-trick scoring as scoring/outcome, not the `token_bazaar`/`poker_lite` public resource-accounting shape. GAT101PLATRI-002 must confirm or refute this explicitly. | no for docs; yes if ledger refutes without decision |
| lead/follow/trick resolution | local-only first official use | New game-local pressure; no helper or promotion is authorized by this ticket. | no |

## Mechanic-Atlas Admission Check

`docs/MECHANIC-ATLAS.md` §10A is empty at admission time, so no open
promotion-debt interlock blocks this documentation ticket or the next ledger
ticket. The §10B deterministic shuffle / private hand / staged reveal row names
`plain_tricks` as the likely third use and requires a ledger decision before
extraction.

## Engine-Core Contamination Review

| Boundary check | Result | Notes |
|---|---|---|
| no game noun needs to enter `engine-core` | pass | All card/trick nouns stay inside `games/plain_tricks`. |
| no rule helper needs to enter `engine-core` | pass | Existing action-tree, command, effect, visibility, replay, and seeded-RNG contracts are sufficient. |
| no private licensed name/data needs to enter kernel contracts | pass | No private licensed content is involved. |
| any generic contract change has ADR or explicit non-goal | pass | No generic contract change is admitted. |

## Static-Data Behavior Review

| Check | Result | Notes |
|---|---|---|
| static data limited to typed content, parameters, metadata, fixtures, traces, or reports | pass | Card labels and variant metadata may be data; behavior stays in Rust. |
| no selectors, rule branches, loops, triggers, tactical conditions, or exception logic in data | pass | Later variant parsing must reject behavior-looking keys. |
| no YAML by default | pass | Gate 10.1 data uses TOML/JSON patterns from existing games. |
| no DSL at project start | pass | No DSL is admitted. |

## Hidden-Information Risk Review

| Surface | Risk level | Required safeguard/test | Admission status |
|---|---:|---|---|
| public/browser payload | high | Rust visibility tests for observer/opponent/seat projections and WASM redaction tests. | ready |
| action tree | high | Actor-only authorization; non-actor empty trees; metadata whitelist. | ready |
| preview | medium | No hidden-card previews; no TypeScript legality. | ready |
| diagnostics/effect log | high | Viewer-safe diagnostics and public/private effect envelopes. | ready |
| DOM/test IDs/local storage/replay export | high | Browser e2e no-leak sweep and viewer-scoped public export tests. | ready |
| bot explanations/candidate rankings | high | Level 2 allowed-input tests and explanation redaction tests. | ready |
| dev inspector | high | Dev panel public-summary whitelist; e2e no-leak sweep. | ready |

## Bot Level Required for This Stage

| Level | Required before public release? | Coding may start now? | Evidence required |
|---:|---:|---:|---|
| 0 random legal | yes | yes after legal tree exists | Legal action API, deterministic seed, simulations. |
| 1 baseline | no | yes | Not required by this gate. |
| 2 authored policy | yes | only after `COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md` | Bot tests, simulations, no hidden-state sampling proof, and viewer-safe explanations. |
| 3 shallow deterministic search | no | no | Out of scope for this gate. |

Public v1/v2 bots must not use MCTS, ISMCTS, Monte Carlo-style bots, ML, RL,
runtime LLM policy, hidden-state sampling, or opponent-hidden-card enumeration.

## UI Exposure Expectations

| Check | Status | Notes |
|---|---|---|
| web-exposed in this stage? | yes | Gate 10.1 requires a Rust/WASM-backed browser shell renderer. |
| React + SVG/default CSS accepted | yes | Presentation must remain neutral and avoid copied card-game trade dress. |
| legal-action tree maps to UI controls | ready | Browser controls must render only Rust-supplied legal leaves. |
| TypeScript presentation-only boundary understood | ready | TypeScript must not compute legality, trick resolution, scoring, or rotation. |
| effect-driven animation expectations identified | ready | Deal, play, trick, score, rotation, and terminal feedback come from semantic effects. |
| accessibility/reduced-motion/responsive expectations identified | ready | Browser smoke must cover keyboard/a11y, reduced motion, and no-leak UI text/attributes. |

## Benchmark Expectations

| Operation | Needed before implementation? | Needed before release? | Notes |
|---|---:|---:|---|
| setup | no | yes | Benchmark setup + shuffle + deal. |
| legal action generation | no | yes | Benchmark leading, forced-follow, and void cases. |
| preview | no | no | No hidden-card preview is admitted. |
| validation/apply action | no | yes | Benchmark play validation, trick resolution, round close, and rotation paths. |
| public/private view generation | no | yes | Benchmark observer and seat projection. |
| effect filtering | no | yes | Include private deal effects and public play/trick/score effects. |
| serialization/deserialization | no | yes | Public and internal trace/export paths need coverage. |
| replay throughput/hash | no | yes | Replay-check and golden traces are release evidence. |
| random playout throughput | no | yes | Provisional target is at least 2,000 completed matches/sec unless accepted calibration changes it. |
| bot decision latency | no | yes | Level 0 and Level 2 bot simulations must terminate under cap. |
| WASM/browser smoke | no | yes | `smoke:wasm`, `smoke:ui`, and `smoke:e2e` close the browser surface. |

## Admission Decision

Decision: admitted with explicit constraints

Decision rationale:

- The Gate 10.1 spec defines a bounded, original, neutral microgame with no
  kernel concept changes.
- `docs/MECHANIC-ATLAS.md` §10A currently records no open promotion debt, so no
  primitive-promotion interlock blocks the admission/source ticket.
- The deterministic shuffle / private hand / staged reveal shape is a third-use
  hard gate and is deliberately assigned to GAT101PLATRI-002 before gameplay
  implementation.
- The initial `RULES.md` and `SOURCES.md` establish stable rule IDs and
  consulted-not-copied IP posture before implementation.

Explicit constraints:

- Public-facing surfaces use **Plain Tricks** and neutral terminology.
- `plain_tricks` remains the internal id for code, roadmap, and tool
  registration.
- Completing this game is what closes the remaining Gate 10 trick-taking half;
  documentation admission alone does not mark Gate 10 or Gate 10.1 complete.

## Blocking Issues

| Issue | Required fix | Owner | Blocks coding? |
|---|---|---|---:|
| third-use primitive-pressure decision not yet recorded | Complete GAT101PLATRI-002 before rules/setup implementation proceeds beyond crate skeleton work. | Rulepath | yes after skeleton |

## Sign-off

Prepared by: `Codex`

Reviewed by: pending maintainer review

Date: 2026-06-09
