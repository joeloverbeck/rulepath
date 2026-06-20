# River Ledger Game Implementation Admission

Game ID: `river_ledger`

Public display name: `River Ledger`

Implemented variant: `river_ledger_standard`

Rules version: `river-ledger-rules-v1`

Roadmap stage/gate: Stage 15 / Gate 15

Created: 2026-06-14

Last updated: 2026-06-14

## Admission Summary

River Ledger is admitted as the Gate 15 Texas Hold'Em-family official game. It
supports exactly 3-6 seats, fixed-limit contribution rounds, deterministic
shuffle/deal, public community-card streets, one opening bet plus three raises
per street, single-pot allocation, foldout terminal results, showdown
evaluation, split pots with deterministic remainders, viewer-scoped replay, and
Level 0, Level 1, and Level 2 Rust bots.

Admission is for the base Rulepath implementation only. All-in, side pots,
no-limit or pot-limit play, tournament systems, real-money framing, networked
multiplayer, lookup-table evaluators, and advanced public bots remain out of
scope.

## Gate 15.1 Admission Delta

Gate 15.1 supersedes the base admission's all-in and side-pot exclusion for one
bounded v2 delta: finite public contribution stacks, fixed-limit all-in
qualification on existing `Call`, `Bet`, and `Raise` actions, full-unit
reopening, ordered contribution-layer side pots, uncalled returns, per-pot
allocation, and Rust-authored public explanations.

The delta remains a pure extension over the shipped single-hand River Ledger
surface. It does not rebuild the dealer/button model, fixed-limit streets,
raise cap, evaluator, base split/remainder order, N-seat viewer model, bots,
WASM bridge, or web renderer except where those surfaces must consume the new
stack and pot facts.

The stack model is configurable per seat through typed setup metadata, with an
equal default of 24 contribution units for each seat. Stack values are ordered
by canonical seat order, validated by Rust, positive, bounded, non-monetary, and
hand-local. They are public accounting facts, not currency, bankroll,
tournament, or casino product state.

River Ledger uses a documented full-unit reopening rule for this game: an
incomplete all-in increase smaller than one street unit does not by itself
reopen raising for a seat that already acted; cumulative incomplete increases
reopen only after the seat faces at least one full street unit since its last
completed action. Contemporary poker authorities differ on some fixed-limit
reopening thresholds, so this is recorded as a deliberate River Ledger rule,
not a universal poker claim.

The planned rules/data cutover is `river-ledger-rules-v2` with manifest
`rules_version = 2` and `data_version = 2`. GAT151RIVLED-011 owns the actual
version bump and hash/serialization migration. Existing v1 replay imports will
be rejected with a stable deterministic version-mismatch diagnostic unless a
later accepted ticket supplies a bounded explicit converter with historical
semantics. Silent interpretation of v1 replays under v2 stack rules is
forbidden.

## Gate 15.1 Rule-Family Plan

The v2 rule set extends the existing `RL-<DOMAIN>-<CONCEPT>-NNN` scheme with the
following stable families:

| Rule family | Planned coverage |
|---|---|
| `RL-STACK-SETUP-*` | Ordered public starting-stack setup, equal 24-unit default, validation, and setup diagnostics. |
| `RL-STACK-CONSERVE-*` | Checked conservation across starting stacks, remaining stacks, contributions, returns, awards, and final stacks. |
| `RL-ALLIN-CAP-*` | Stack-capped blinds and actions; no contribution above remaining stack. |
| `RL-ALLIN-CALL-*` | Short and exact all-in call behavior while preserving `Call` as the action family. |
| `RL-ALLIN-BET-*` | Short/full all-in bet and raise qualification on fixed-limit `Bet` and `Raise` actions. |
| `RL-ALLIN-ACT-*` | Folded/all-in actor exclusion, response obligation, no-action all-in states, and deterministic rotation. |
| `RL-ALLIN-REOPEN-*` | Full-unit reopening, cumulative incomplete-increase accounting, and raise-cap interaction. |
| `RL-ALLIN-RUNOUT-*` | Automatic deterministic runout when no betting decision remains and at least two non-folded seats remain. |
| `RL-POT-LAYER-*` | Ascending contribution caps, coalesced ordered pot tiers, and deterministic pot identities. |
| `RL-POT-ELIG-*` | Folded contributions count in amounts while folded seats remain ineligible. |
| `RL-POT-RETURN-*` | Singleton unmatched top layers return as uncalled contribution, not as pot wins. |
| `RL-POT-ALLOC-*` | Independent per-pot evaluation, split, award, and final-stack aggregation. |
| `RL-POT-REMAINDER-*` | Independent odd-unit remainder assignment per pot using the existing button order. |
| `RL-OUTCOME-POT-*` | Rust-authored public explanations for every pot award and return. |
| `RL-VIS-POT-*` | Viewer-safe stacks, eligibility, pot tiers, returns, awards, and hidden-card redaction. |
| `RL-BOT-ALLIN-*` | Bot legality, determinism, authorized views, and stack-aware explanations. |
| `RL-REPLAY-STACK-*` | v2 stack/reopen/pot state hashing, serialization, replay, export, and v1 import handling. |

Supersession map for existing Gate 15 all-in and single-pot rows:

| Legacy rule ID | v2 treatment |
|---|---|
| `RL-POT-SINGLE-001` | Superseded by `RL-POT-LAYER-*`, `RL-POT-ALLOC-*`, and `RL-POT-REMAINDER-*`; the one-pot case remains the single-layer case. |
| `RL-POT-SINGLE-002` | Superseded by `RL-POT-LAYER-*`; terminal accounting may now contain ordered main and side pots. |
| `RL-POT-ALLIN-001` | Superseded by `RL-ALLIN-CAP-*`, `RL-ALLIN-CALL-*`, `RL-ALLIN-BET-*`, `RL-ALLIN-ACT-*`, and `RL-ALLIN-RUNOUT-*`. |
| `RL-VAR-ALLIN-001` | Superseded by `RL-STACK-SETUP-*`, `RL-ALLIN-*`, and `RL-POT-*`; all-in is part of the standard v2 River Ledger variant, not a separate no-limit variant. |
| `RL-OOS-ALLIN-001` | Superseded by the Gate 15.1 v2 families; no-limit, pot-limit, tournament, bankroll, real-money, and multi-hand carryover remain out of scope. |

GAT151RIVLED-019 owns the final `RULES.md` and `RULE-COVERAGE.md`
reconciliation after behavior, traces, no-leak evidence, benchmarks, and
versioned replay migration land.

## Required Evidence

| Area | Evidence |
|---|---|
| rules and validation | [RULES.md](RULES.md), [RULE-COVERAGE.md](RULE-COVERAGE.md), `cargo test -p river_ledger` |
| source/IP review | [SOURCES.md](SOURCES.md) |
| mechanic inventory | [MECHANICS.md](MECHANICS.md) |
| primitive-pressure ledger | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md), `docs/MECHANIC-ATLAS.md` Gate 15 row |
| static data and fixtures | `games/river_ledger/data/*`, `cargo run -p fixture-check -- --game river_ledger` |
| replay/golden traces | `games/river_ledger/tests/golden_traces/*`, `cargo run -p replay-check -- --game river_ledger --all` |
| simulation | `cargo run -p simulate -- --game river_ledger --seat-count 6 --games 1000 --start-seed 1506 --action-cap 48` |
| rule coverage | `cargo run -p rule-coverage -- --game river_ledger` |
| bot policy | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md), `cargo test -p river_ledger --test bots` |
| benchmarks | [BENCHMARKS.md](BENCHMARKS.md), `cargo bench -p river_ledger` |
| WASM registration | `cargo test -p wasm-api`, `npm --prefix apps/web run smoke:wasm` |
| web board and no-leak/a11y | [UI.md](UI.md), `RiverLedgerBoard.tsx`, `node apps/web/e2e/river-ledger.smoke.mjs`, `npm --prefix apps/web run smoke:e2e` |
| catalog and public rules | `ci/games.json`, `README.md`, `apps/web/README.md`, `apps/web/public/rules/river_ledger.md`, `node scripts/check-catalog-docs.mjs`, `node scripts/check-player-rules.mjs` |

## Prerequisite Readiness

| Prerequisite | Path | Complete? | Notes |
|---|---|---:|---|
| Gate 15 spec and ticket sequence | `archive/specs/gate-15-river-ledger-texas-holdem-base.md`, `archive/tickets/GAT15RIVLEDTEX-*` | yes | Gate 15 is complete and archived. |
| source/IP notes | [SOURCES.md](SOURCES.md) | yes | Sources verify common rules facts only; no copied prose or assets. |
| original rules with stable rule IDs | [RULES.md](RULES.md) | yes | `RL-*` families cover setup, deal, betting, street, evaluator, showdown, pot, visibility, bots, UI, replay, variants, and out-of-scope boundaries. |
| rule coverage matrix | [RULE-COVERAGE.md](RULE-COVERAGE.md) | yes | Rows name implementation, traces, tests, and later web evidence. |
| mechanic inventory | [MECHANICS.md](MECHANICS.md) | yes | Records game-local card/deck/evaluator/accounting/showdown shapes. |
| primitive-pressure ledger | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) | yes | GAT15RIVLEDTEX-020 completed the final atlas-pressure closeout. |
| competent-player analysis | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | yes | Feeds the Level 2 authored policy. |
| bot evidence pack | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) | yes | Documents allowed inputs and forbidden hidden facts. |
| UI notes | [UI.md](UI.md) | yes | Documents the implemented N-seat browser renderer and outcome surface. |
| public-release checklist | [PUBLIC-RELEASE-CHECKLIST.md](PUBLIC-RELEASE-CHECKLIST.md) | yes | Release evidence and remaining capstone dependency are explicit. |
| ADR, if boundary-changing | not applicable | yes | No accepted ADR is required; existing engine, WASM, replay, trace, and app seams are reused. |

## Source and IP Readiness

| Check | Status | Evidence/notes |
|---|---|---|
| consulted sources recorded with dates | ready | [SOURCES.md](SOURCES.md) records Texas Hold'Em and hand-ranking references. |
| sources used only for verification/context | ready | Rulepath prose and presentation are original. |
| no copied rulebook prose or hand-ranking table | ready | [RULES.md](RULES.md), [HOW-TO-PLAY.md](HOW-TO-PLAY.md), and web rules copy use original wording. |
| no copied art/icons/screenshots/scans/fonts/trade dress | ready | The renderer uses project-authored CSS/HTML/SVG shapes and system fonts. |
| public naming rationale recorded | ready | [SOURCES.md](SOURCES.md) records River Ledger as the product-facing name. |
| private licensed content excluded from public files | ready | No private content is used in docs, fixtures, traces, WASM, or browser assets. |
| human/legal review triggers cleared or recorded | ready | No external asset or close-source wording trigger remains; casino-adjacent trade dress is avoided. |

## Rule-ID Readiness

| Check | Status | Evidence/notes |
|---|---|---|
| every implemented rule has a stable rule ID | ready | [RULES.md](RULES.md) defines the canonical `RL-*` set. |
| rule IDs use stable prefixes | ready | Prefixes include `RL-SETUP`, `RL-DEAL`, `RL-BET`, `RL-STREET`, `RL-EVAL`, `RL-SHOW`, `RL-POT`, `RL-VIS`, `RL-BOT`, `RL-UI`, `RL-REPLAY`, `RL-VAR`, and `RL-OOS`. |
| chosen ambiguities are documented | ready | Ambiguity rows cover seat range, burn behavior, evaluator tie rules, remainder order, and public replay redaction. |
| out-of-scope variants are explicit | ready | All-in, side pots, no-limit/pot-limit, tournament systems, real-money framing, and advanced bots are rejected. |
| rule coverage is reconciled to implementation | ready | [RULE-COVERAGE.md](RULE-COVERAGE.md) maps rows to Rust modules, traces, tests, docs, and browser evidence. |

## Admission Constraints

- No card, deck, hand, street, blind, button, pot, evaluator, fold, bet, call,
  raise, showdown, or River Ledger nouns enter `engine-core`.
- No `game-stdlib` promotion is authorized by this admission.
- No YAML, DSL, selectors, formulas, tactical conditions, or behavior-looking
  static data.
- No TypeScript legality, validation, contribution accounting, street
  advancement, evaluator, winner choice, split allocation, replay authority,
  hidden-info filtering, or bot policy.
- No unauthorized private cards, future board cards, deck-tail data, burn data,
  seed-reconstructable hidden setup data, candidate rankings, or private bot
  reasoning in public/browser payloads, DOM, attributes, test IDs, storage,
  logs, dev panel, public replay export, bot rationale, diagnostics, effects,
  or traces.
- No MCTS, ISMCTS, Monte Carlo equity simulation, ML, RL, runtime LLM policy,
  hidden-state sampling, external solvers, or opponent-card peeking.
- No casino trade dress, real-money framing, tournament product mimicry, copied
  rules prose, copied hand-ranking tables, or proprietary naming.

## Primitive-Pressure Status

| Mechanic shape | Status | Decision/evidence | Blocks admission? |
|---|---|---|---:|
| deterministic deck/deal/private hands | repeated shape | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) compares existing card games and keeps River Ledger game-local. | no |
| seven-card evaluator/showdown explanation | game-local pressure | Evaluator remains straightforward and audit-oriented inside `games/river_ledger`. | no |
| public contribution ledger/single pot | repeated accounting shape | Ledger and pot allocation stay game-local until the atlas admits a shared primitive. | no |
| N-seat hidden-information visibility | public scaling pressure | Pairwise tests and browser no-leak smoke prove the game-specific projection. | no |

GAT15RIVLEDTEX-020 completed the final mechanic-atlas pressure review and
confirmed no promotion debt remains.

## Engine-Core Contamination Review

| Boundary check | Result | Notes |
|---|---|---|
| no game noun needs to enter `engine-core` | pass | River Ledger nouns remain under `games/river_ledger`. |
| no rule helper needs to enter `engine-core` | pass | Existing action tree, command, effect, visibility, replay, and seeded-RNG contracts are sufficient. |
| no static behavior system is added | pass | Variant data is typed metadata only. |
| any generic contract change has ADR or explicit non-goal | pass | No generic contract migration is admitted. |

## Static-Data Behavior Review

| Check | Result | Notes |
|---|---|---|
| static data limited to typed content, parameters, metadata, fixtures, traces, or reports | pass | Variant and manifest files carry metadata and setup parameters only. |
| no selectors, rule branches, loops, triggers, tactical conditions, or exception logic in data | pass | Behavior-looking keys are rejected by variant-loader tests. |
| no YAML by default | pass | Game data uses TOML and JSON fixtures/traces. |
| no DSL at project start | pass | Rust owns behavior. |

## Hidden-Information Risk Review

| Surface | Risk level | Safeguard/test | Admission status |
|---|---|---|---|
| public/browser payload | high | Rust visibility tests, WASM redaction, River Ledger browser smoke. | ready |
| action tree | high | Rust legal-action tree only; actor/viewer authorization in WASM. | ready |
| diagnostics/effect log | high | Safe diagnostics, filtered semantic effects, no-leak smoke. | ready |
| DOM/test IDs/storage/logs | high | `node apps/web/e2e/river-ledger.smoke.mjs`. | ready |
| replay export/import | high | Public replay traces, WASM replay dispatch, browser e2e coverage. | ready |
| bot explanations | high | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), bot tests, no hidden-state sampling. | ready |
| dev inspector | high | Viewer-filtered public summary only; browser no-leak smoke. | ready |

## Bot Level Required for This Stage

| Level | Required before public release? | Status | Evidence required |
|---:|---:|---|---|
| 0 random legal | yes | implemented | [AI.md](AI.md), `cargo test -p river_ledger --test bots`, simulator. |
| 1 conservative | yes | implemented | [AI.md](AI.md), bot tests, allowed-input evidence. |
| 2 authored policy | yes | implemented | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), bot tests. |
| 3 shallow deterministic search | no | not implemented | Out of scope. |

Public bots must not use MCTS, ISMCTS, Monte Carlo-style bots, ML, RL,
runtime LLM policy, hidden-state sampling, or opponent-hidden-card enumeration.

## UI Exposure Readiness

| Check | Status | Notes |
|---|---|---|
| web-exposed in this stage? | ready | River Ledger is in the WASM catalog and web renderer mapping. |
| Rust/WASM payloads drive presentation | ready | `RiverLedgerBoard` consumes structured WASM view/action/effect data. |
| legal-action tree maps to UI controls | ready | Browser controls render Rust-supplied choices only. |
| N-seat viewer matrix documented | ready | [UI.md](UI.md) records public observer and every authorized seat viewer. |
| browser no-leak smoke exists | ready | `node apps/web/e2e/river-ledger.smoke.mjs`. |
| accessibility/reduced-motion/responsive expectations identified | ready | [UI.md](UI.md), e2e smoke, shared shell checks. |
| public rules surface exists | ready | `apps/web/public/rules/river_ledger.md`, copied from [HOW-TO-PLAY.md](HOW-TO-PLAY.md). |

## Benchmark Expectations

| Operation | Needed before release? | Evidence |
|---|---:|---|
| setup/deal | yes | [BENCHMARKS.md](BENCHMARKS.md), `cargo bench -p river_ledger` |
| legal action generation | yes | [BENCHMARKS.md](BENCHMARKS.md), benchmark output |
| validation/apply action | yes | [BENCHMARKS.md](BENCHMARKS.md), benchmark output |
| public/seat view generation | yes | [BENCHMARKS.md](BENCHMARKS.md), benchmark output |
| replay export/import | yes | [BENCHMARKS.md](BENCHMARKS.md), replay-check |
| evaluator batch | yes | [BENCHMARKS.md](BENCHMARKS.md), benchmark output |
| bot decision/full playout | yes | [BENCHMARKS.md](BENCHMARKS.md), simulator and benchmark output |
| WASM/browser smoke | yes | `npm --prefix apps/web run smoke:wasm`, `npm --prefix apps/web run smoke:e2e` |

## Admission Decision

Decision: admitted with explicit constraints and final capstone dependency.

Decision rationale:

- River Ledger satisfies the official-game evidence set across Rust rules,
  traces, simulation, replay, fixture, coverage, bot, benchmark, WASM, web,
  no-leak, public rules, and catalog surfaces.
- The game consumes existing Rulepath seams. It does not require an `engine-core`
  noun, trace-schema migration, behavior-in-data path, TypeScript legality, or
  new AI class.
- Public presentation uses original River Ledger terminology, abstract
  contribution units, and neutral board-game visual language.
- GAT15RIVLEDTEX-021 remains responsible for the final acceptance sweep and
  spec/index closeout before the Gate 15 family is completely archived.

## Release Blockers

| Issue | Required fix | Owner | Blocks current admission? |
|---|---|---|---:|
| Final acceptance sweep and spec/index archive are pending. | Complete GAT15RIVLEDTEX-021 after 020. | Rulepath | no; blocks final gate closeout |

No additional implementation, WASM, UI, public-copy, no-leak, or bot blocker is
known for the trailing-doc scope.
