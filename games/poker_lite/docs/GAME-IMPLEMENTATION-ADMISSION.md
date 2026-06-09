# Crest Ledger Implementation Admission

Game ID: `poker_lite`

Public display name: `Crest Ledger`

Implemented variant: `poker_lite_standard`

Rules version: `poker-lite-rules-v1`

Roadmap stage/gate: Stage 9 / Gate 10 betting-showdown half

Last updated: 2026-06-09

## Admission Summary

Crest Ledger is admitted as the Gate 10 `poker_lite` betting/showdown proof
game. The implementation must be game-local Rust with a Rust/WASM browser
presentation. It proves deterministic private-card setup, staged center reveal,
bounded pledge rounds, exact shared-pool accounting, yield terminal without
private reveal, grouped showdown reveal, pair-before-high-card comparison,
viewer-scoped replay export, Level 0 and Level 2 bot policies, and browser
no-leak behavior.

Admission covers only the `poker_lite` betting/showdown portion of Gate 10. It
does not close the successor trick-taking/follow-suit work.

## Required Evidence

| Area | Evidence |
|---|---|
| rules and validation | [RULES.md](RULES.md), `games/poker_lite/docs/RULE-COVERAGE.md`, `cargo test -p poker_lite --test rules` |
| source/IP review | [SOURCES.md](SOURCES.md) |
| mechanic inventory | `games/poker_lite/docs/MECHANICS.md` |
| primitive-pressure ledger | `games/poker_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md`, `docs/MECHANIC-ATLAS.md` §10B |
| static data and fixtures | `games/poker_lite/data/*`, `cargo run -p fixture-check -- --game poker_lite` |
| replay/golden traces | `games/poker_lite/tests/golden_traces/*`, `cargo run -p replay-check -- --game poker_lite --all` |
| simulation | `cargo run -p simulate -- --game poker_lite --games 1000 --start-seed 0 --action-cap 16` |
| rule coverage | `cargo run -p rule-coverage -- --game poker_lite` |
| bot policy | `games/poker_lite/docs/AI.md`, `games/poker_lite/docs/BOT-STRATEGY-EVIDENCE-PACK.md`, `games/poker_lite/docs/COMPETENT-PLAYER.md`, `cargo test -p poker_lite --test bots` |
| WASM registration | `cargo test -p wasm-api`, `npm --prefix apps/web run smoke:wasm` |
| web board and no-leak/a11y | `games/poker_lite/docs/UI.md`, `PokerLiteBoard.tsx`, `node apps/web/e2e/poker-lite.smoke.mjs` |
| benchmarks | `games/poker_lite/docs/BENCHMARKS.md`, `cargo bench -p poker_lite` |

## Prerequisite Readiness

| Prerequisite | Path | Complete? | Notes/blockers |
|---|---|---:|---|
| source/IP notes | [SOURCES.md](SOURCES.md) | yes | Consulted sources are context only; no copied prose/assets. |
| original rules with stable rule IDs | [RULES.md](RULES.md) | yes | `CL-*` rule IDs cover setup, actions, pledge accounting, reveal, showdown, terminal, visibility, replay, variants, and out-of-scope boundaries. |
| rule coverage matrix | `games/poker_lite/docs/RULE-COVERAGE.md` | no | Later ticket; must map every rule ID. |
| mechanic inventory | `games/poker_lite/docs/MECHANICS.md` | no | Later ticket after implementation surfaces exist. |
| primitive-pressure ledger | `games/poker_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md` | no | Capstone ticket; required because this gate creates second-use pressure for card/private-hand and accounting plus first-use pledge/shared-pool pressure. |
| competent-player analysis | `games/poker_lite/docs/COMPETENT-PLAYER.md` | no | Bot ticket; required before public closeout. |
| ADR, if boundary-changing | not applicable | yes | No new kernel concept or behavior-in-data path is admitted. |

## Source and IP Readiness

| Check | Status | Evidence/notes |
|---|---|---|
| consulted sources recorded with dates | ready | `SOURCES.md` records Gate 10, project authority, Kuhn, Leduc-style, and OpenSpiel context sources. |
| sources used only for verification/context | ready | Sources are consulted-not-copied proof-shape context. |
| Rulepath rules prose is original | ready | `RULES.md` uses original Crest Ledger terminology and rule statements. |
| no copied card/component text | ready | Crest ids and labels are Rulepath-authored. |
| no copied art/icons/screenshots/scans/fonts/trade dress | ready | No assets land in this ticket; later UI must remain neutral. |
| public naming rationale recorded | ready | `SOURCES.md` records **Crest Ledger** as the neutral public name. |
| private licensed content excluded from public files | ready | No private licensed content is involved. |
| human/legal review triggers cleared or recorded | ready | No blocker identified; casino-adjacent presentation remains a release-audit concern. |

## Rule-ID Readiness

| Check | Status | Evidence/notes |
|---|---|---|
| every implemented rule has a stable rule ID | ready | `RULES.md` creates the initial `CL-*` canonical set. |
| rule IDs use stable prefixes, not section-only references | ready | Prefixes include `CL-COMP`, `CL-SETUP`, `CL-TURN`, `CL-ACT`, `CL-RESTRICT`, `CL-PLEDGE`, `CL-REVEAL`, `CL-SCORE`, `CL-END`, `CL-VIS`, `CL-RNG`, `CL-AMB`, `CL-VAR`, and `CL-OOS`. |
| ambiguities have chosen resolutions and IDs | ready | `CL-AMB-001` through `CL-AMB-005`. |
| out-of-scope variants are explicit | ready | `CL-OOS-001` through `CL-OOS-004`. |
| rule-ID migration policy is understood | ready | Migration table exists in `RULES.md`; later coverage/traces must follow it. |

## Admission Constraints

- No `engine-core` crest, card, deck, hand, pledge, marker, shared-pool,
  betting, showdown, pair, rank, fold, match, or yield nouns.
- No `game-stdlib` card/deck/hand/accounting/pledge/showdown primitive is
  promoted in this gate.
- No YAML, DSL, selectors, formulas, bot policy, hidden-info routing, or
  behavior-looking keys in static data.
- No TypeScript legality, validation, center reveal timing, private reveal
  timing, pledge accounting, terminal allocation, showdown comparison, replay
  authority, hidden-info filtering, or bot policy.
- No private crest, hidden center crest, deck tail, hidden-state inference, or
  seed-reconstructable hidden setup data in public/browser payloads, DOM,
  attributes, test IDs, storage, logs, dev panel, public replay export, bot
  rationale, or candidate ranking before the rule-defined reveal point.
- No automatic public reveal of a yielded seat's private crest.
- No MCTS, ISMCTS, Monte Carlo equity simulation, ML, RL, runtime LLM bot,
  hidden-state sampling, opponent-card enumeration, or opponent-card peeking.
- No casino trade dress, real-money framing, public poker-engine claim, copied
  rules prose, copied hand-ranking table, or proprietary naming.

## Primitive-Pressure Status

| Mechanic shape | Status | Decision/evidence | Blocks implementation? |
|---|---|---|---:|
| deterministic shuffle / private hand / staged reveal | repeated-shape candidate, second official use | `high_card_duel` is first official use; `poker_lite` must keep the shape local and record second-use comparison in the capstone. | no |
| public resource/accounting ledger | repeated-shape candidate, second official use | `token_bazaar` is first official use; `poker_lite` must keep the shape local and record second-use comparison in the capstone. | no |
| bounded pledge / shared-pool / showdown allocation | local-only first official use | New game-local pressure; no helper or promotion is authorized. | no |

## Engine-Core Contamination Review

| Boundary check | Result | Notes |
|---|---|---|
| no game noun needs to enter `engine-core` | pass | All crest/pledge/accounting nouns stay inside `games/poker_lite`. |
| no rule helper needs to enter `engine-core` | pass | Existing action-tree, command, effect, visibility, replay, and seeded-RNG contracts are sufficient. |
| no private licensed name/data needs to enter kernel contracts | pass | No private licensed content is involved. |
| any generic contract change has ADR or explicit non-goal | pass | No generic contract change is admitted. |

## Static-Data Behavior Review

| Check | Result | Notes |
|---|---|---|
| static data limited to typed content, parameters, metadata, fixtures, traces, or reports | pass | Crest labels and variant metadata may be data; behavior stays in Rust. |
| no selectors, rule branches, loops, triggers, tactical conditions, or exception logic in data | pass | Later variant parsing must reject behavior-looking keys. |
| no YAML by default | pass | Gate 10 data uses TOML/JSON patterns from existing games. |
| no DSL at project start | pass | No DSL is admitted. |

## Hidden-Information Risk Review

| Surface | Risk level | Required safeguard/test | Admission status |
|---|---|---|---|
| public/browser payload | high | Rust visibility tests for observer/opponent/seat projections and WASM redaction tests. | ready |
| action tree | medium | Action metadata whitelist; actor authorization for action-tree access. | ready |
| preview | medium | No hidden-card previews; no TypeScript legality. | ready |
| diagnostics/effect log | high | Viewer-safe diagnostics and grouped reveal effects. | ready |
| DOM/test IDs/local storage/replay export | high | Browser e2e no-leak sweep and viewer-scoped public export tests. | ready |
| bot explanations/candidate rankings | high | Level 2 allowed-input tests and explanation redaction tests. | ready |
| dev inspector | high | Dev panel public-summary whitelist; e2e no-leak sweep. | ready |

## Bot Level Required for This Stage

| Level | Required before public release? | Coding may start now? | Evidence required |
|---:|---:|---:|---|
| 0 random legal | yes | yes | Legal action API, deterministic seed, simulations. |
| 1 baseline | no | yes | Not required by this gate. |
| 2 authored policy | yes | yes | `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, bot tests, simulations, and no hidden-state sampling proof before closeout. |
| 3 shallow deterministic search | no | no | Out of scope for this gate. |

Public v1/v2 bots must not use MCTS, ISMCTS, Monte Carlo-style bots, ML, RL,
runtime LLM policy, hidden-state sampling, or opponent-hidden-card enumeration.

## UI Exposure Expectations

| Check | Status | Notes |
|---|---|---|
| web-exposed in this stage? | yes | Gate 10 requires a Rust/WASM-backed browser shell renderer. |
| React + SVG/default CSS accepted | yes | Presentation must remain neutral and avoid casino trade dress. |
| legal-action tree maps to UI controls | ready | Browser controls must render only Rust-supplied legal leaves. |
| TypeScript presentation-only boundary understood | ready | TypeScript must not compute legality, accounting, reveal, or showdown. |
| effect-driven animation expectations identified | ready | Center and showdown reveals are grouped effect moments; reduced motion must be respected. |
| accessibility/reduced-motion/responsive expectations identified | ready | Browser smoke must cover keyboard/a11y, reduced motion, and no-leak UI text/attributes. |

## Benchmark Expectations

| Operation | Needed before implementation? | Needed before release? | Notes |
|---|---:|---:|---|
| setup | no | yes | Benchmark setup + shuffle + deal. |
| legal action generation | no | yes | Benchmark action tree generation across phases. |
| preview | no | no | No hidden-card preview is admitted. |
| validation/apply action | no | yes | Benchmark hold, press, lift, match, yield, reveal, and showdown paths. |
| public/private view generation | no | yes | Benchmark observer and seat projection. |
| effect filtering | no | yes | Include grouped reveal and private setup effects. |
| serialization/deserialization | no | yes | Public and internal trace/export paths need coverage. |
| replay throughput/hash | no | yes | Replay-check and golden traces are release evidence. |
| random playout throughput | no | yes | Provisional target is at least 2,000 completed hands/sec unless accepted calibration changes it. |
| bot decision latency | no | yes | Level 0 and Level 2 bot simulations must terminate under cap. |
| WASM/browser smoke | no | yes | `smoke:wasm`, `smoke:ui`, and `smoke:e2e` close the browser surface. |

## Admission Decision

Decision: admitted with explicit constraints

Decision rationale:

- The Gate 10 spec defines a bounded, original, neutral microgame with no kernel
  concept changes.
- `docs/MECHANIC-ATLAS.md` §10A currently records no open promotion debt, so no
  primitive-promotion interlock blocks coding.
- Card/private-hand and resource/accounting pressure stays local for this
  second-use comparison; the capstone must record the comparison without
  promoting a helper.
- The initial `RULES.md` and `SOURCES.md` establish stable rule IDs and
  consulted-not-copied IP posture before implementation.

Explicit constraints:

- Public-facing surfaces use **Crest Ledger** and neutral terminology.
- `poker_lite` remains an internal id only where needed for code, roadmap, and
  tool registration.
- Completing this game does not mark the whole Gate 10 row done; the
  trick-taking half remains for a successor spec.

## Blocking Issues

| Issue | Required fix | Owner | Blocks coding? |
|---|---|---|---:|
| none identified for ticket 001 | Continue ticket family evidence workflow. | Rulepath | no |

## Sign-off

Prepared by: `Codex`

Reviewed by: pending maintainer review

Date: 2026-06-09
