# Starbridge Crossing Game Evidence Receipt

Game ID: `starbridge_crossing`

Rules version: `starbridge-crossing-rules-v1`

Data/manifest version: `starbridge-crossing-data-v1`

Trace/profile version set: Trace Schema v1; `replay-command-v1`;
`public-export-v1`; `setup-evidence-v1`; `domain-evidence-v1`

Engine version: current Rulepath workspace

Prepared by: `Codex`

Last updated: 2026-06-27

## Purpose

This receipt is the artifact-link and status index for Starbridge Crossing
official-game conformance. It does not duplicate rules prose, source prose,
strategy prose, or behavior tables.

Rows marked `pass` have command, document, or review evidence in this receipt
and linked artifacts. Human IP/public-release review remains pending before
external public release.

## Completion Profile

| Field | Value |
|---|---|
| Completion profile | `large-board-perfect-information-star-race-release-candidate` |
| Profile rationale | 121-space public board, 2/3/4/6 seats, Rust-owned topology/path legality, jump chains, replay, benchmarks, L0 bot, WASM/web renderer, and all-public no-leak proof. |
| Not applicable summary | Hidden hands, decks, commitments, teams, partnerships, tricks, melds, betting, ADR 0004 hidden-information review, and seat-private exports are not applicable. |
| Deferred checker surface | none; scaffolding governance receipt completed in GAT20STACROSTA-019. |
| Foundation invariants status | Rust owns legality, TypeScript presents Rust/WASM output, no hidden-state leak accepted, no helper promotion debt. |
| Stop-condition review | no active stop condition; human IP/public-release review remains a release blocker, not a Gate 20 implementation blocker. |

## Supported Seats And Variants

| Surface | Status | Artifact link | Notes |
|---|---|---|---|
| Supported seat counts | pass | [RULES.md](RULES.md#setup) | 2, 3, 4, and 6 seats; default 2. |
| Implemented variant | pass | [SOURCES.md](SOURCES.md#variant-choice-and-deviations) | Only `starbridge_crossing_classic_star_v1`. |
| Seat roles/labels | pass | [MECHANICS.md](MECHANICS.md) | No teams; each seat races to its opposite target. |
| N-seat obligations | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md) | 2/3/4/6 setup fixtures and max-seat simulation/e2e coverage. |

## Source And IP Receipt

| Check | Status | Artifact link | Notes |
|---|---|---|---|
| Source notes complete | pass | [SOURCES.md](SOURCES.md) | Consulted sources, variant decisions, and non-copying posture recorded. |
| Original rules prose complete | pass | [RULES.md](RULES.md) | Every Gate 20 `SC-*` rule family represented in original prose. |
| Public name/trade-dress review | pending human review | [SOURCES.md](SOURCES.md#public-naming-rationale) | Human IP/public-release review remains pending. |
| Assets/fonts/license review | pending human review | [PUBLIC-RELEASE-CHECKLIST.md](PUBLIC-RELEASE-CHECKLIST.md) | Original web presentation; final public-release review remains human-owned. |
| Private-source exclusion | pass | [SOURCES.md](SOURCES.md#publicprivate-content-boundary) | No private licensed content involved. |

## Rule-Coverage Summary

| Evidence surface | Status | Artifact link | Notes |
|---|---|---|---|
| Rule coverage matrix | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md) | `cargo run -p rule-coverage -- --game starbridge_crossing`. |
| Unit and named rule tests | pass | `games/starbridge_crossing/tests/` | Rules, property, replay, serialization, visibility, and bot suites. |
| Simulation runs | pass | `simulate` 2/3/4/6 lanes | Bounded L0 playout smoke across official seat counts. |
| Serialization coverage | pass | `games/starbridge_crossing/tests/serialization.rs`; `games/starbridge_crossing/tests/replay.rs` | Stable state/view/replay surfaces. |
| Benchmarks | pass | [BENCHMARKS.md](BENCHMARKS.md); `cargo bench -p starbridge_crossing` | 14 native lanes with smoke floors. |

## Named Trace Profiles

| Profile ID | Profile version | Visibility class | Validator owner | Artifact link | Status | Notes |
|---|---|---|---|---|---|---|
| `replay-command-v1` | `v1` | internal-dev/public | replay-check | `games/starbridge_crossing/tests/golden_traces/`; `cargo run -p replay-check -- --game starbridge_crossing --all` | pass | Setup, step, jump, finish, terminal, invalid, bot, and no-leak traces. |
| `public-export-v1` | `v1` | public | Rust/WASM export/import tests | `public-replay-round-trip.trace.json`; `wasm-exported.trace.json`; web replay smoke | pass | Starbridge exports are all-public. |
| `setup-evidence-v1` | `v1` | internal-dev/public | fixture-check | `games/starbridge_crossing/data/fixtures/`; `cargo run -p fixture-check -- --game starbridge_crossing` | pass | 2p, 3p, 4p, and 6p fixtures. |
| `domain-evidence-v1` | `v1` | internal-dev/public | game-local validator | `games/starbridge_crossing/tests/{rules,property,serialization,replay,visibility,bots}.rs` | pass | Topology, movement, jump chains, finish ranks, visibility, and bots. |
| `scaffolding-forward-v1` | `v1` | governance | scaffolding checker | `ci/scaffolding-audits.json`; [docs/MECHANICAL-SCAFFOLDING-REGISTER.md](../../../docs/MECHANICAL-SCAFFOLDING-REGISTER.md) | pass | Receipt landed in GAT20STACROSTA-019. |

## Fixture Profile

| Fixture | Status | Purpose |
|---|---|---|
| `starbridge_crossing_2p_standard.fixture.json` | pass | 2-seat setup over the full 121-space board. |
| `starbridge_crossing_3p_standard.fixture.json` | pass | 3-seat point assignment and occupancy. |
| `starbridge_crossing_4p_standard.fixture.json` | pass | 4-seat point assignment and occupancy. |
| `starbridge_crossing_6p_standard.fixture.json` | pass | Max-seat setup and board pressure. |

## Viewer Matrix

| Viewer class | Public view evidence | Seat-private view evidence | Action/effect/diagnostic evidence | Replay/export evidence | Status |
|---|---|---|---|---|---|
| public observer | observer corpus and browser smoke | not applicable | public tree/effect/diagnostic checks | public export | pass |
| seat `seat_0` through `seat_5` | same all-public facts | not applicable | actor authorization checks | public export | pass |

## Hidden-Information No-Leak Matrix

| Surface | Status | Evidence/test | Notes |
|---|---|---|---|
| public/browser payload | pass | WASM/e2e no-leak tests | All game facts are public. |
| public view | pass | visibility tests | Full board, occupancy, active seat, finish ranks. |
| action tree | pass | action tests and e2e | Only Rust legal paths. |
| previews/diagnostics | pass | rule/visibility tests | Viewer-safe stable diagnostics. |
| effect logs | pass | effect tests | Step/jump/finish/pass/terminal facts are public. |
| DOM/test IDs | pass | `starbridge-crossing.smoke.mjs` scan | No hidden/debug/candidate terms. |
| console/storage | pass | `starbridge-crossing.smoke.mjs` | No hidden values logged or persisted. |
| replay export/import | pass | replay tests and e2e | All-public export cannot elevate privileges. |
| bot explanations | pass | bot tests | L0 explanations name public legal-choice count only. |
| candidate rankings | not applicable | no public candidate ranking | L1/L2 not admitted. |

## Command Evidence

| Command | Status | Notes |
|---|---|---|
| `cargo test -p starbridge_crossing` | pass | game-local unit/integration coverage. |
| `cargo run -p replay-check -- --game starbridge_crossing --all` | pass | all trace receipts accepted. |
| `cargo run -p fixture-check -- --game starbridge_crossing` | pass | fixture catalog accepted. |
| `cargo run -p rule-coverage -- --game starbridge_crossing` | pass | coverage matrix and benchmark doc present. |
| `cargo bench -p starbridge_crossing` | pass | benchmark smoke floors pass. |
| `npm --prefix apps/web run smoke:e2e` | pass | includes Starbridge browser smoke. |

## Pending Human/Follow-Up Items

| Item | Status | Owner |
|---|---|---|
| Public name/IP review | pending human review | maintainer |
| Scaffolding receipt | completed GAT20STACROSTA-019 | implementation series |
| Final spec closeout | completed GAT20STACROSTA-020 | implementation series |
