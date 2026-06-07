# Directional Flip Implementation Admission

Game ID: `directional_flip`

Public display name: `Directional Flip`

Implemented variant: `directional_flip_standard`

Roadmap stage/gate: Gate 6 directional-flip official game

Public role: official game / public release candidate after checklist

Date: 2026-06-07

## Admission Summary

Directional Flip is admitted as the Gate 6 official game after implementation tickets landed source/rule docs, primitive-pressure review, Rust rules, visibility, bot evidence, replay traces, benchmarks, WASM bridge, tools, web UI, E2E/no-leak smoke, and CI wiring.

This document is a receipt for the official-game contract. It does not expose the game publicly by itself; public picker exposure remains gated by the final release checklist/status ticket.

## Prerequisite Documents

| Prerequisite | Path | Complete? | Notes/blockers |
|---|---|---:|---|
| source/IP notes | [SOURCES.md](SOURCES.md) | yes | neutral naming and no copied assets/prose recorded |
| original rules with stable rule IDs | [RULES.md](RULES.md) | yes | `DF-*` IDs cover setup, actions, flips, pass, terminal, visibility, UI, bots |
| rule coverage matrix | [RULE-COVERAGE.md](RULE-COVERAGE.md) | yes | all rule IDs covered or explicitly documented |
| mechanic inventory | [MECHANICS.md](MECHANICS.md) | yes | third-use pressure linked to ledger |
| primitive-pressure ledger | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) | yes | coordinate/ray helper defer-reject decision |
| competent-player analysis | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | yes | strategy input for Level 2-lite bot |
| Level 2 evidence pack | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) | yes | public explanation/no-ranking contract recorded |
| UI notes | [UI.md](UI.md) | yes | legal-only renderer, previews, keyboard, reduced motion |
| AI notes | [AI.md](AI.md) | yes | Level 0 and Level 2-lite registry |
| benchmark notes | [BENCHMARKS.md](BENCHMARKS.md) | yes | baseline-pending benchmark posture |
| ADR, if boundary-changing | not applicable | yes | no engine-core, DSL, YAML, or shared primitive change |

## Contract Evidence

| Area | Evidence | Status | Notes |
|---|---|---|---|
| Rust setup/rules | `setup.rs`, `actions.rs`, `rules.rs` | complete | typed game-local board, legal placements, pass, flips, terminal rules |
| semantic effects | `effects.rs` | complete | placement, grouped flips, pass, turn, terminal, bot rationale |
| visibility | `visibility.rs`, visibility tests | complete | perfect information; hidden fields empty |
| bots | `bots.rs`, [AI.md](AI.md) | complete | Level 0 and Level 2-lite only |
| replay/traces | `replay_support.rs`, `tests/golden_traces/*.trace.json` | complete | traces cover openings, diagnostics, pass, terminal, bot, WASM |
| WASM/API | `crates/wasm-api/src/lib.rs`, `smoke:wasm` | complete | registered game, public view, action tree, effects, bot, replay |
| web UI | `DirectionalFlipBoard.tsx`, `directional-flip.smoke.mjs`, [UI.md](UI.md) | complete | grid, previews, forced pass, replay, a11y/no-leak |
| benchmarks | `benches/directional_flip.rs`, [BENCHMARKS.md](BENCHMARKS.md) | complete with baseline-pending caveat | thresholds are smoke floors |
| CLI tools | `simulate`, `replay-check`, `fixture-check`, `rule-coverage`, `bench-report`, `seed-reducer`, `trace-viewer` | complete | registered and locally verified |
| CI | `.github/workflows/gate-1-game-smoke.yml`, `.github/workflows/gate-2-benchmarks.yml` | complete | Directional Flip wired into smoke and benchmark lanes |

## Source And IP Readiness

| Check | Status | Evidence/notes |
|---|---|---|
| consulted sources recorded with dates | ready | [SOURCES.md](SOURCES.md) |
| sources used only for verification/context | ready | original Rulepath prose in [RULES.md](RULES.md) |
| no copied component text/assets/screenshots/fonts/trade dress | ready | no external assets; neutral CSS/SVG board |
| public naming rationale recorded | ready | `Directional Flip` is project-owned neutral naming |
| private licensed content excluded | ready | no private licensed content in rules, traces, UI, or replay |
| human/legal review triggers | constrained | required before public hosted release if project policy requires legal review |

## Mechanic And Boundary Readiness

| Check | Status | Evidence/notes |
|---|---|---|
| all mechanic atlas categories inventoried | ready | [MECHANICS.md](MECHANICS.md) |
| primitive-pressure third-use decision complete | ready | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) |
| no game noun in `engine-core` | pass | `bash scripts/boundary-check.sh` |
| no YAML/DSL/procedural static behavior | pass | static data remains typed metadata/fixtures/traces/thresholds |
| TypeScript presentation-only boundary | pass | [UI.md](UI.md), `DirectionalFlipBoard.tsx`, browser smoke |

## Hidden-Information Risk Review

| Surface | Risk level | Required safeguard/test | Admission status |
|---|---|---|---|
| public/browser payload | low | WASM smoke checks hidden fields and private-view status | ready |
| action tree | low | Rust legal targets only; browser controls derive from them | ready |
| preview | low | Rust target/flip preview only | ready |
| diagnostics/effect log | low | viewer-safe diagnostics/effects; replay traces | ready |
| DOM/test IDs/local storage/replay export | low | Directional Flip and shared no-leak smokes | ready |
| bot explanations/candidate rankings | low | public rationale, no rankings/scores | ready |
| dev inspector | low | secondary panel; viewer-safe metadata only | ready |

## Bot Level Required For This Stage

| Level | Required before public release? | Status | Evidence |
|---:|---:|---|---|
| 0 random legal | yes | complete | implemented and tested |
| 1 baseline | no | not planned | Level 2-lite authored policy is the public bot |
| 2 authored policy | yes | complete | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), [AI.md](AI.md) |
| 3+ search/solver | no | excluded | public v1/v2 exclusions apply |

## Benchmark Expectations

| Operation | Needed before release? | Evidence |
|---|---:|---|
| legal action generation | yes | [BENCHMARKS.md](BENCHMARKS.md), thresholds |
| flip scanning/apply action | yes | [BENCHMARKS.md](BENCHMARKS.md) |
| public view generation | yes | [BENCHMARKS.md](BENCHMARKS.md) |
| replay/serialization | yes | [BENCHMARKS.md](BENCHMARKS.md), replay-check |
| random playout throughput | yes | [BENCHMARKS.md](BENCHMARKS.md) |
| bot decision latency | yes | Level 0 and Level 2-lite benchmark thresholds |
| WASM/browser smoke | yes | `smoke:wasm`, E2E smokes |

## Admission Decision

Decision: admitted with explicit constraints

Decision rationale:

- The official-game evidence set is implemented and locally verified.
- Rust remains behavior authority; TypeScript renders public Rust/WASM output.
- Third-use rectangular/ray mechanic pressure is recorded and explicitly deferred, not silently ignored.

Explicit constraints:

- Public picker/status exposure remains gated by the release/status ticket.
- Any future shared coordinate/ray helper must reopen the primitive-pressure ledger and preserve traces/hashes.
- Any public hosted release still needs human/IP review if project policy requires it.

## Blocking Issues

| Issue | Required fix | Owner | Blocks coding? |
|---|---|---|---:|
| none for Gate 6 implementation | not applicable | not applicable | no |
