# Column Four Implementation Admission

Game ID: `column_four`

Public display name: `Column Four`

Implemented variant: `column_four_standard`

Roadmap stage/gate: Gate 5 public showcase

Public role: public showcase

Date: 2026-06-06

## Admission Summary

Column Four is admitted as the Gate 5 public showcase game after the implementation tickets landed the rule docs, Rust rules, visibility, bot evidence, replay traces, benchmarks, WASM bridge, web UI, E2E smoke, and CI wiring.

This document is a receipt for the official-game contract. It does not waive future atlas/status updates or public release review.

## Prerequisite Documents

| Prerequisite | Path | Complete? | Notes/blockers |
|---|---|---:|---|
| source/IP notes | [SOURCES.md](SOURCES.md) | yes | neutral naming and no copied assets/prose recorded |
| original rules with stable rule IDs | [RULES.md](RULES.md) | yes | `CF-*` IDs cover setup, actions, gravity, terminal, visibility, bot, variants |
| rule coverage matrix | [RULE-COVERAGE.md](RULE-COVERAGE.md) | yes | all `CF-*` IDs covered, traced, or explicitly not applicable |
| mechanic inventory | [MECHANICS.md](MECHANICS.md) | yes | second-use pressure recorded; no extraction |
| primitive-pressure ledger, if needed | not applicable | yes | atlas update is queued for GAT5COLFOUPUB-018; no separate ledger file |
| competent-player analysis | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | yes | strategy matters for Level 2 bot |
| Level 2 evidence pack | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) | yes | public explanation/no-ranking contract recorded |
| ADR, if boundary-changing | not applicable | yes | no engine-core, DSL, YAML, or shared primitive change |

## Contract Evidence

| Area | Evidence | Status | Notes |
|---|---|---|---|
| Rust setup/rules | `games/column_four/src/setup.rs`, `actions.rs`, `rules.rs` | complete | typed game-local grid, gravity, terminal rules |
| semantic effects | `games/column_four/src/effects.rs` | complete | drop, landing, turn, terminal, bot rationale |
| visibility | `games/column_four/src/visibility.rs`, visibility tests | complete | perfect information; hidden fields empty |
| bots | `games/column_four/src/bots.rs`, [AI.md](AI.md) | complete | Level 0 and Level 2 only |
| replay/traces | `games/column_four/src/replay_support.rs`, `tests/golden_traces/*.trace.json` | complete | 11 traces including bot, draw, diagnostics, wins |
| WASM/API | `crates/wasm-api/src/lib.rs`, `npm --prefix apps/web run smoke:wasm` | complete | registered game, public view, action tree, effects, replay |
| web UI | `ColumnFourBoard.tsx`, `column-four.smoke.mjs`, [UI.md](UI.md) | complete | seven controls, preview, replay, a11y/no-leak |
| benchmarks | `games/column_four/benches/column_four.rs`, [BENCHMARKS.md](BENCHMARKS.md) | complete with visible caveat | random playout target miss documented; thresholds not hidden |
| CLI tools | `simulate`, `replay-check`, `fixture-check`, `rule-coverage` | complete | registered and locally verified |
| CI | `.github/workflows/gate-1-game-smoke.yml`, `.github/workflows/gate-2-benchmarks.yml` | complete | Column Four wired into smoke and benchmark lanes |

## Source And IP Readiness

| Check | Status | Evidence/notes |
|---|---|---|
| consulted sources recorded with dates | ready | [SOURCES.md](SOURCES.md) |
| sources used only for verification/context | ready | original Rulepath prose in [RULES.md](RULES.md) |
| no copied component text/assets/screenshots/fonts/trade dress | ready | no external assets; neutral SVG/CSS board |
| public naming rationale recorded | ready | `Column Four` is project-owned neutral naming |
| private licensed content excluded | ready | no private licensed content in rules, traces, UI, or replay |
| human/legal review triggers | constrained | public legal review remains an external release question, not a code blocker |

## Mechanic And Boundary Readiness

| Check | Status | Evidence/notes |
|---|---|---|
| all mechanic atlas categories inventoried | ready | [MECHANICS.md](MECHANICS.md) |
| repeated-shape comparison complete | ready | fixed grid and line detection compared to `three_marks` |
| third-use hard gate | not applicable | this is second-use pressure, not third use |
| no game noun in `engine-core` | pass | `bash scripts/boundary-check.sh` |
| no YAML/DSL/procedural static behavior | pass | static files are docs, traces, fixtures, thresholds |
| TypeScript presentation-only boundary | pass | [UI.md](UI.md), `ColumnFourBoard.tsx`, browser smoke |

## Hidden-Information Risk Review

| Surface | Risk level | Required safeguard/test | Admission status |
|---|---|---|---|
| public/browser payload | low | WASM smoke checks hidden fields and private-view status | ready |
| action tree | low | Rust legal targets only; browser controls derive from them | ready |
| preview | low | Rust landing preview only | ready |
| diagnostics/effect log | low | viewer-safe diagnostics/effects; replay traces | ready |
| DOM/test IDs/local storage/replay export | low | `column-four.smoke.mjs` no-leak scan | ready |
| bot explanations/candidate rankings | low | public rationale, no rankings/scores | ready |
| dev inspector | low | secondary panel; viewer-safe metadata only | ready |

## Bot Level Required For This Stage

| Level | Required before public release? | Coding may start now? | Evidence required |
|---:|---:|---:|---|
| 0 random legal | yes | complete | implemented and tested |
| 1 baseline | no | not planned | Level 2 authored policy supersedes it for Gate 5 |
| 2 authored policy | yes | complete | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| 3 shallow deterministic search | no | no | excluded for public v1/v2 |

## Benchmark Expectations

| Operation | Needed before release? | Evidence |
|---|---:|---|
| legal action generation | yes | [BENCHMARKS.md](BENCHMARKS.md), thresholds |
| validation/apply action | yes | [BENCHMARKS.md](BENCHMARKS.md) |
| public view generation | yes | [BENCHMARKS.md](BENCHMARKS.md) |
| replay/serialization | yes | [BENCHMARKS.md](BENCHMARKS.md), replay-check |
| random playout throughput | yes | [BENCHMARKS.md](BENCHMARKS.md) records provisional target miss |
| bot decision latency | yes | Level 0 and Level 2 benchmark thresholds |
| WASM/browser smoke | yes | `smoke:wasm`, `smoke:e2e` |

## Admission Decision

Decision: admitted with explicit constraints

Decision rationale:

- The official-game evidence set is implemented and locally verified.
- Rust remains behavior authority; TypeScript renders public Rust/WASM output.
- Repeated board/line mechanic pressure is recorded, but extraction is deferred.

Explicit constraints:

- GAT5COLFOUPUB-018 must update repo-level status and mechanic atlas surfaces.
- Any future third fixed-grid/line-detection official game must clear primitive-pressure review before implementation.
- Public hosted release still needs a human release/IP review.

## Blocking Issues

| Issue | Required fix | Owner | Blocks coding? |
|---|---|---|---:|
| none for Gate 5 implementation | not applicable | not applicable | no |
