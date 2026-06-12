# event_frontier Implementation Admission

Game ID: `event_frontier`

Public display name: `Event Frontier`

Implemented variant: `event_frontier_standard` plus `event_frontier_hard_winter` and `event_frontier_land_rush`

Roadmap stage/gate: Gate 14

Public role: public showcase / hidden-info proof / original portfolio game

Prepared by: Codex

Date: 2026-06-12

## Purpose

This document records the admission and closeout evidence for Event Frontier as the Gate 14 capstone. The gate proves the public ladder's highest complexity surface: event deck sequencing, eligibility/initiative, periodic Reckoning scoring/reset, asymmetric victory, large Rust action trees, Level 1 bots, native tools, WASM/browser presentation, and hidden deck-order redaction.

## Prerequisite Documents

| Prerequisite | Path | Complete? | Notes/blockers |
|---|---|---:|---|
| source/IP notes | `games/event_frontier/docs/SOURCES.md` | yes | Original names/prose recorded; no copied rulebook text or trade dress. |
| original rules with stable rule IDs | `games/event_frontier/docs/RULES.md` | yes | Event, operation, edict, Reckoning, terminal, visibility, and replay rules carry stable `EF-*` IDs. |
| rule coverage matrix | `games/event_frontier/docs/RULE-COVERAGE.md` | yes | Native `rule-coverage` registration is green. |
| mechanic inventory | `games/event_frontier/docs/MECHANICS.md` | yes | Local mechanics and repeated-shape comparisons are recorded. |
| primitive-pressure ledger | `games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md` | yes | Third-use public-resource decision and budget non-use decision are recorded. |
| competent-player analysis | `games/event_frontier/docs/COMPETENT-PLAYER.md` | yes | Bot and play expectations are documented. |
| ADR | not applicable | yes | No boundary-changing ADR was required. |

## Source and IP Readiness

| Check | Status | Evidence/notes |
|---|---|---|
| consulted sources recorded with dates | ready | `SOURCES.md` records context sources and originality posture. |
| sources used only for verification/context | ready | Implementation uses original rules, names, cards, factions, and sites. |
| Rulepath rules prose is original | ready | `RULES.md` and `HOW-TO-PLAY.md` are original Rulepath prose. |
| no copied card/component text | ready | Event cards are original typed IDs/labels/effects. |
| no copied art/icons/screenshots/scans/fonts/trade dress | ready | Browser board is local React/SVG/CSS; no third-party assets. |
| public naming rationale recorded | ready | `SOURCES.md` records original naming review. |
| private licensed content excluded from public files | ready | No private names, assets, or source material used. |
| human/legal review triggers cleared or recorded | ready | No trigger beyond ordinary maintainer review. |

## Rule-ID Readiness

| Check | Status | Evidence/notes |
|---|---|---|
| every implemented rule has a stable rule ID | ready | `RULES.md` and `RULE-COVERAGE.md`. |
| rule IDs use stable prefixes, not section-only references | ready | `EF-*` prefix used throughout rules, tests, and coverage docs. |
| ambiguities have chosen resolutions and IDs | ready | Both-met Freeholder rule, final fallback, and tiebreaks are explicit. |
| out-of-scope variants are explicit | ready | Three implemented variants only; no expansion variants admitted. |
| rule-ID migration policy is understood | ready | Future migrations require notes, coverage updates, and trace refresh. |

## Rule Coverage Readiness

| Check | Status | Evidence/notes |
|---|---|---|
| coverage matrix has one row per rule ID | ready | `RULE-COVERAGE.md`; `cargo run -p rule-coverage -- --game event_frontier`. |
| deferred/unsupported/not applicable rows are explicit | ready | Non-used preview/private-view surfaces are marked not applicable. |
| primary Rust test strategy is identified | ready | `rules.rs`, `property.rs`, `replay.rs`, `serialization.rs`, `visibility.rs`, `bots.rs`, `golden_traces.rs`. |
| golden trace needs are identified | ready | Eighteen golden traces under `games/event_frontier/tests/golden_traces/`. |
| invalid/stale diagnostic trace needs are identified | ready | Diagnostics covered by rule tests, WASM smoke, and stale action surfaces. |
| replay/hash requirements are identified | ready | `replay-check`, golden traces, and public export/import checks. |
| serialization requirements are identified | ready | `serialization.rs` and fixture/native tool checks. |
| visibility/no-leak requirements are identified | ready | `visibility.rs`, `smoke-load-wasm`, and browser no-leak E2E. |
| UI smoke coverage is scoped as smoke only | ready | `event-frontier.smoke.mjs`, `smoke:ui`, `smoke:effects`, `smoke:e2e`. |

## Mechanic Inventory Readiness

| Check | Status | Evidence/notes |
|---|---|---|
| all mechanic atlas categories are inventoried | ready | `MECHANICS.md` and `PRIMITIVE-PRESSURE-LEDGER.md`. |
| local mechanics are named and scoped | ready | Event deck, edicts, initiative, Reckoning, asymmetric victory, and graph map stay local. |
| reused primitives are justified | ready | No new `game-stdlib` helper; existing shell/WASM/tool patterns reused. |
| repeated-shape comparison is complete | ready | Ledger compares Flood Watch, Frontier Control, Token Bazaar, Crest Ledger, and prior hidden-info games. |
| second-use review is recorded when applicable | ready | Event deck, graph map, faction asymmetry, role/modifier pressure. |
| third-use hard gate is cleared when applicable | ready | Public resource accounting defer/reject; multi-action budget non-use. |
| repo atlas update required? | yes | `docs/MECHANIC-ATLAS.md` updated; §10A remains `_None_`. |

## Primitive-Pressure Status

| Mechanic shape | Status | Decision/evidence | Blocks implementation? |
|---|---|---|---:|
| public resource accounting / shared ledgers | rejected/deferred with rationale | Event Frontier funding/income differs from Token Bazaar and Crest Ledger; no helper promotion. | no |
| multi-action turn budgets | rejected/deferred with rationale / non-use | Event Frontier operations are compound commands, not regenerated action budgets. | no |
| deterministic shuffle / staged public reveal | rejected/deferred with rationale | No per-seat private holdings; hidden surface is undrawn deck order. | no |
| graph-map topology / faction asymmetry | repeated-shape candidate | Second comparable use after Frontier Control; local only. | no |
| event-card initiative / Reckoning / timed edicts / asymmetric instant victory | local-only | First official use; keep game-local. | no |

## Engine-Core Contamination Review

| Boundary check | Result | Notes |
|---|---|---|
| no game noun needs to enter `engine-core` | pass | Event, card, deck, edict, faction, site, Reckoning, and eligibility nouns stay in `games/event_frontier`. |
| no rule helper needs to enter `engine-core` | pass | Validation, scoring, visibility, replay, and bots stay game-local. |
| no private licensed name/data needs to enter kernel contracts | pass | No private licensed content used. |
| any generic contract change has ADR or explicit non-goal | pass | No generic engine contract change. |

## Static-Data Behavior Review

| Check | Result | Notes |
|---|---|---|
| static data limited to typed content, parameters, metadata, fixtures, traces, or reports | pass | TOML data names IDs, labels, variants, and parameters only. |
| no selectors, rule branches, loops, triggers, tactical conditions, or exception logic in data | pass | Event/edict behavior is typed Rust. |
| no YAML by default | pass | No YAML added. |
| no DSL at project start | pass | No DSL added. |

## Hidden-Information Risk Review

| Surface | Risk level | Required safeguard/test | Admission status |
|---|---|---|---|
| public/browser payload | medium | `smoke-load-wasm`, `event-frontier.smoke.mjs` | ready |
| action tree | medium | Rust legal tree only, no hidden deck order | ready |
| preview | none | No separate preview surface | not applicable |
| diagnostics/effect log | medium | Viewer-safe public effects and diagnostics | ready |
| DOM/test IDs/local storage/replay export | high | `event-frontier.smoke.mjs`, `a11y-noleak.smoke.mjs` | ready |
| bot explanations/candidate rankings | medium | Level 1 explanations are viewer-safe; rankings not exported | ready |
| dev inspector | medium | Text/debug surfaces are public projection only | ready |

## Bot Level Required For This Stage

| Level | Required before public release? | Coding may start now? | Evidence required |
|---:|---:|---:|---|
| 0 random legal | yes | yes | Legal action API, deterministic seed, simulations. |
| 1 baseline | yes | yes | `BOT-STRATEGY-EVIDENCE-PACK.md`, bot tests, simulation evidence. |
| 2 authored policy | no | no | Not required for Gate 14 public v1. |
| 3 shallow deterministic search | no | no | Not allowed/needed. |

Public v1/v2 bots do not use MCTS, ISMCTS, Monte Carlo-style bots, ML, or RL.

## UI Exposure Expectations

| Check | Status | Notes |
|---|---|---|
| web-exposed in this stage? | yes | Event Frontier is registered in the catalog and browser shell. |
| React + SVG default accepted | yes | Board uses React/SVG/CSS. |
| legal-action tree maps to UI controls | ready | Buttons submit Rust action-tree leaves only. |
| TypeScript presentation-only boundary understood | ready | TS renders Rust view/tree/effects and does not compute legality/scoring/victory. |
| effect-driven animation expectations identified | ready | Semantic effects drive feedback; reduced motion supported. |
| accessibility/reduced-motion/responsive expectations identified | ready | E2E smokes cover names, redaction, reduced motion, and responsive layout. |

## Benchmark Expectations

| Operation | Needed before implementation? | Needed before release? | Notes |
|---|---:|---:|---|
| setup | yes | yes | Bench identity documented and smoke floor registered. |
| legal action generation | yes | yes | Peak operation branching benchmarked. |
| preview | no | no | No separate preview surface. |
| validation/apply action | yes | yes | Rule tests and benchmark identity. |
| public/private view generation | yes | yes | Public projection benchmarked; private view not applicable. |
| effect filtering | yes | yes | Visibility tests and WASM/browser smokes. |
| serialization/deserialization | yes | yes | Serialization tests and fixture checks. |
| replay throughput/hash | yes | yes | Golden traces and replay-check. |
| random playout throughput | yes | yes | `simulate --game event_frontier --games 1000`. |
| bot decision latency | yes | yes | Bot tests and benchmarks. |
| WASM/browser smoke | yes | yes | `smoke:wasm`, `smoke:ui`, `smoke:effects`, Event Frontier E2E. |

## Admission Decision

Decision: admitted

Decision rationale:

- Gate 14 implementation and verification completed with Event Frontier registered across Rust, native tools, WASM, browser, docs, CI, and catalog surfaces.
- Primitive-pressure decisions are recorded, no `engine-core` noun or `game-stdlib` helper was promoted, and `docs/MECHANIC-ATLAS.md` §10A remains `_None_`.
- Hidden-information surfaces are covered by Rust visibility tests, public replay/export checks, and browser no-leak smokes.

Explicit constraints if admitted conditionally:

- None.

## Blocking Issues

| Issue | Required fix | Owner | Blocks coding? |
|---|---|---|---:|
| None | Not applicable | Rulepath maintainers | no |

## Sign-Off

Prepared by: Codex

Reviewed by: Rulepath maintainers

Date: 2026-06-12
