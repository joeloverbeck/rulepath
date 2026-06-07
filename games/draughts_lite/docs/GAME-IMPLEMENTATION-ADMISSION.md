# Draughts Lite Implementation Admission

Game ID: `draughts_lite`

Public display name: `Draughts Lite`

Implemented variant: `draughts_lite_standard`

Roadmap stage/gate: Gate 7 compound-action official game

Public role: official game / public release candidate after checklist

Date: 2026-06-07

## Admission Summary

Draughts Lite is admitted as the Gate 7 compound-action official game after implementation tickets landed source/rule docs, primitive-pressure review, Rust rules, visibility, bot evidence, replay traces, benchmarks, WASM bridge, tools, web UI, E2E/no-leak smoke, and CI wiring.

This document is a receipt for the [../../../docs/OFFICIAL-GAME-CONTRACT.md](../../../docs/OFFICIAL-GAME-CONTRACT.md). It does not by itself close the gate index; the capstone ticket owns final status and spec archival.

## Prerequisite Documents

| Prerequisite | Path | Complete? | Notes/blockers |
|---|---|---:|---|
| source/IP notes | [SOURCES.md](SOURCES.md) | yes | neutral naming and scoped English draughts references recorded |
| original rules with stable rule IDs | [RULES.md](RULES.md) | yes | `DL-*` IDs cover setup, actions, moves, restrictions, terminal, visibility, UI, bots |
| rule coverage matrix | [RULE-COVERAGE.md](RULE-COVERAGE.md) | yes | all rule IDs covered or explicitly documented |
| mechanic inventory | [MECHANICS.md](MECHANICS.md) | yes | compound action tree and board-space pressure recorded |
| primitive-pressure ledger | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) | yes | behavior-free board-space reuse recorded |
| competent-player analysis | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | yes | strategy input for Level 1 bot |
| bot strategy evidence pack | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) | yes | public explanation/no-ranking contract recorded |
| UI notes | [UI.md](UI.md) | yes | compound path UI, keyboard, reduced motion, no-legality boundary |
| AI notes | [AI.md](AI.md) | yes | Level 0 and Level 1 registry |
| benchmark notes | [BENCHMARKS.md](BENCHMARKS.md) | yes | baseline-pending benchmark posture |
| ADR, if boundary-changing | not applicable | yes | no engine-core noun, DSL, YAML, or behavior helper change |

## Contract Evidence

| Area | Evidence | Status | Notes |
|---|---|---|---|
| Rust setup/rules | `setup.rs`, `actions.rs`, `rules.rs` | complete | typed board, legal moves, mandatory capture, continuation, promotion, terminal |
| semantic effects | `effects.rs` | complete | move, step, capture, promotion, forced capture/continuation, invalid, terminal, bot |
| visibility | `visibility.rs`, visibility tests | complete | perfect information; hidden fields empty |
| bots | `bots.rs`, [AI.md](AI.md) | complete | Level 0 and Level 1 only |
| replay/traces | `replay_support.rs`, `tests/golden_traces/*.trace.json` | complete | traces cover quiet, capture, multi-jump, diagnostics, terminal, bot, WASM |
| WASM/API | `crates/wasm-api/src/lib.rs`, `smoke:wasm` | complete | registered game, recursive action tree, effects, bot, replay |
| web UI | `DraughtsLiteBoard.tsx`, `draughts-lite.smoke.mjs`, [UI.md](UI.md) | complete | grid, compound path, keyboard, replay, a11y/no-leak |
| benchmarks | `benches/draughts_lite.rs`, [BENCHMARKS.md](BENCHMARKS.md) | complete with baseline-pending caveat | thresholds are smoke floors |
| CLI tools | `simulate`, `replay-check`, `fixture-check`, `rule-coverage`, `bench-report` | complete | registered and locally verified |
| CI | `.github/workflows/gate-1-game-smoke.yml`, `.github/workflows/gate-2-benchmarks.yml` | complete | Draughts Lite wired into smoke and benchmark lanes |

## Source And IP Readiness

| Check | Status | Evidence/notes |
|---|---|---|
| consulted sources recorded with dates | ready | [SOURCES.md](SOURCES.md) |
| rules prose is original | ready | [RULES.md](RULES.md) |
| no copied component text/assets/screenshots/fonts/trade dress | ready | no external assets; neutral CSS board |
| public naming rationale recorded | ready | `Draughts Lite` is neutral and scoped |
| private licensed content excluded | ready | no private licensed content in rules, traces, UI, or replay |
| human/legal review triggers | constrained | required before public hosted release if project policy requires legal review |

## Mechanic And Boundary Readiness

| Check | Status | Evidence/notes |
|---|---|---|
| official mechanic inventory complete | ready | [MECHANICS.md](MECHANICS.md) |
| primitive-pressure decision complete | ready | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) |
| no game noun in `engine-core` | pass | `bash scripts/boundary-check.sh` |
| no YAML/DSL/procedural static behavior | pass | static data remains typed metadata/fixtures/traces/thresholds |
| TypeScript presentation-only boundary | pass | [UI.md](UI.md), `DraughtsLiteBoard.tsx`, browser smoke |
| replay/hash determinism | pass | `cargo run -p replay-check -- --game draughts_lite --all` |

## Hidden-Information Risk Review

| Surface | Risk level | Required safeguard/test | Admission status |
|---|---|---|---|
| public/browser payload | low | WASM smoke and no-leak E2E | ready |
| action tree | low | Rust legal choices only; UI derives from them | ready |
| diagnostics/effect log | low | viewer-safe diagnostics/effects; traces | ready |
| DOM/test IDs/local storage/replay export | low | Draughts Lite and shared no-leak smokes | ready |
| bot explanations/candidate rankings | low | public rationale, no rankings/scores | ready |
| dev inspector | low | secondary panel; viewer-safe metadata only | ready |

## Bot Level Required For This Stage

| Level | Required before public release? | Status | Evidence |
|---:|---:|---|---|
| 0 random legal | yes | complete | implemented and tested |
| 1 authored policy | yes | complete | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), [AI.md](AI.md) |
| 2+ search/solver | no | excluded | public v1/v2 exclusions apply |

## Admission Decision

Decision: admitted with explicit constraints

Decision rationale:

- The official-game evidence set is implemented and locally verified.
- Rust remains behavior authority; TypeScript renders public Rust/WASM output.
- Compound action paths, forced continuation, replay hashes, and browser path display are covered by tests and smokes.

Explicit constraints:

- Final gate/index closure remains owned by GAT7DRALITCOM-022.
- Any future draw/adjudication/tournament support requires a new accepted spec.
- Any public hosted release still needs human/IP review if project policy requires it.

## Blocking Issues

| Issue | Required fix | Owner | Blocks coding? |
|---|---|---|---:|
| none for Gate 7 implementation | not applicable | not applicable | no |
