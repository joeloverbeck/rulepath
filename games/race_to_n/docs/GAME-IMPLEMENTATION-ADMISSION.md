# race_to_n Implementation Admission

Game ID: `race_to_n`

Public display name: `Race to 21`

Implemented variant: `single-counter normal-play race to 21; add 1, 2, or 3`

Roadmap stage/gate: `Gate 1`

Public role: `foundation-smoke`

Prepared by: `Codex`

Date: 2026-06-05

## Purpose

This is a gate receipt before serious coding starts.

It is not a ticket. It is not an implementation plan. It decides whether
implementation work may start under the corrected Rulepath foundation docs.

Admission does not waive later rule coverage, tests, traces, benchmarks, UI,
bot evidence, or release gates.

## Prerequisite documents

| Prerequisite | Path | Complete? | Notes/blockers |
|---|---|---:|---|
| source/IP notes | `games/race_to_n/docs/SOURCES.md` | yes | Sources, variant choice, naming, and IP boundary recorded. |
| original rules with stable rule IDs | `games/race_to_n/docs/RULES.md` | yes | Stable rule IDs are authored for the selected variant. |
| rule coverage matrix | `games/race_to_n/docs/RULE-COVERAGE.md` | yes | Rows are `not started` because implementation begins in later tickets. |
| mechanic inventory | `games/race_to_n/docs/MECHANICS.md` | yes | Atlas categories are filled and first-use local-only status is recorded. |
| primitive-pressure ledger, if needed | not applicable | not applicable | First use only; no repeated-shape pressure. |
| competent-player analysis, if strategy matters | not applicable | not applicable | Gate 1 requires Level 0 random legal bot only. |
| ADR, if boundary-changing | not applicable | not applicable | This ticket changes no kernel boundary. |

## Source and IP readiness

| Check | Status | Evidence/notes |
|---|---|---|
| consulted sources recorded with dates | ready | `SOURCES.md#consulted-sources`. |
| sources used only for verification/context | ready | No source prose, assets, or examples are copied. |
| Rulepath rules prose is original | ready | `RULES.md` is project-authored prose for the selected variant. |
| no copied card/component text | not applicable | The game has no cards and no protected component text. |
| no copied art/icons/screenshots/scans/fonts/trade dress | ready | No assets or fonts introduced. |
| public naming rationale recorded | ready | `SOURCES.md#public-naming-rationale`. |
| private licensed content excluded from public files | not applicable | No private licensed content is involved. |
| human/legal review triggers cleared or recorded | ready | No triggers apply in `SOURCES.md#humanlegal-review-triggers`. |

## Rule-ID readiness

| Check | Status | Evidence/notes |
|---|---|---|
| every implemented rule has a stable rule ID | ready | `RULES.md` defines stable IDs for scope, setup, action, validation, outcome, visibility, replay, and variants. |
| rule IDs use stable prefixes, not section-only references | ready | IDs use `R-*` prefixes. |
| ambiguities have chosen resolutions and IDs | ready | `R-AMB-001` and `R-AMB-002`. |
| out-of-scope variants are explicit | ready | `R-VAR-004`. |
| rule-ID migration policy is understood | ready | `RULES.md#rule-id-migration-notes`. |

Rule IDs must remain stable after implementation unless intentionally migrated
with notes and coverage updates.

## Rule coverage readiness

| Check | Status | Evidence/notes |
|---|---|---|
| coverage matrix has one row per rule ID | ready | `RULE-COVERAGE.md#rule-coverage-matrix`. |
| deferred/unsupported/not applicable rows are explicit | ready | Rows are `not started`; not-applicable visibility rationale is explicit. |
| primary Rust test strategy is identified | ready | Named rule, property, replay, serialization, bot, and simulation test surfaces are listed. |
| golden trace needs are identified | ready | Shortest normal, terminal, bot-action, invalid/stale, and public-view traces are listed. |
| invalid/stale diagnostic trace needs are identified | ready | `R-RESTRICT-001`, `R-AMB-002`. |
| replay/hash requirements are identified | ready | `R-RNG-002`. |
| serialization requirements are identified | ready | Coverage rows and test mapping identify snapshot/public-view/replay JSON needs. |
| visibility/no-leak requirements are identified | ready | Perfect-information no-leak is recorded as not applicable with rationale. |
| UI smoke coverage is scoped as smoke only | ready | UI smoke rows explicitly do not prove Rust rule correctness. |

## Mechanic inventory readiness

| Check | Status | Evidence/notes |
|---|---|---|
| all mechanic atlas categories are inventoried | ready | `MECHANICS.md#mechanic-inventory-categories`. |
| local mechanics are named and scoped | ready | Target counter race and target-bounded flat additions. |
| reused primitives are justified | not applicable | No primitive reuse. |
| repeated-shape comparison is complete | ready | First use only. |
| second-use review is recorded when applicable | not applicable | No second use. |
| third-use hard gate is cleared when applicable | not applicable | No third use. |
| repo atlas update required? | no | Existing atlas row already records `race_to_n` as `local-only`; final confirmation is owned by GAT1RACTON-014. |

## Primitive-pressure status

| Mechanic shape | Status | Decision/evidence | Blocks implementation? |
|---|---|---|---:|
| tiny numeric turn race | `local-only` | First official use; keep local per mechanic atlas. | no |

## Engine-core contamination review

| Boundary check | Result | Notes |
|---|---|---|
| no game noun needs to enter `engine-core` | pass | Counter, seat, and target vocabulary stay in game docs and later game crate. |
| no rule helper needs to enter `engine-core` | pass | Later generic contracts may be needed, but this game-specific rule shape stays local. |
| no private licensed name/data needs to enter kernel contracts | pass | No private licensed content. |
| any generic contract change has ADR or explicit non-goal | pass | This ticket changes no generic contract; later WB2 must follow kernel-change protocol. |

## Static-data behavior review

| Check | Result | Notes |
|---|---|---|
| static data limited to typed content, parameters, metadata, fixtures, traces, or reports | pass | No static data is added in this ticket. |
| no selectors, rule branches, loops, triggers, tactical conditions, or exception logic in data | pass | No behavior data is added. |
| no YAML by default | pass | No YAML is added. |
| no DSL at project start | pass | No DSL is added. |

## Hidden-information risk review

| Surface | Risk level | Required safeguard/test | Admission status |
|---|---|---|---|
| public/browser payload | none | Public-view tests; no hidden state exists. | ready |
| action tree | none | Action-tree tests; legal actions are public. | ready |
| preview | none | Preview tests or explicit non-use rationale; resulting counter is public. | ready |
| diagnostics/effect log | low | Viewer-safe diagnostics; invalid/stale trace. | ready |
| DOM/test IDs/local storage/replay export | none | UI smoke and replay/serialization checks. | ready |
| bot explanations/candidate rankings | none | Bot tests if explanations exist; bot sees public state only. | ready |
| dev inspector | none | Web boundary review when web-exposed. | ready |

## Bot level required for this stage

| Level | Required before public release? | Coding may start now? | Evidence required |
|---:|---:|---:|---|
| 0 random legal | yes | yes | Legal action API, deterministic seed, simulations. |
| 1 baseline | no | no | Out of Gate 1 scope. |
| 2 authored policy | no | no | Out of Gate 1 scope. |
| 3 shallow deterministic search | no | no | Out of Gate 1 scope. |

Public v1/v2 bots must not use MCTS, ISMCTS, Monte Carlo-style bots, ML, or RL.

## UI exposure expectations

| Check | Status | Notes |
|---|---|---|
| web-exposed in this stage? | yes | Bare Gate 1 WASM/web harness only. |
| React + SVG default accepted | constrained | UI is out of this ticket; later work must stay minimal and original. |
| legal-action tree maps to UI controls | ready | Direct buttons for Rust-supplied additions. |
| TypeScript presentation-only boundary understood | ready | TS must not decide legality. |
| effect-driven animation expectations identified | constrained | Effects needed for logs/UI smoke; polish is out of Gate 1. |
| accessibility/reduced-motion/responsive expectations identified | constrained | Later UI smoke owns this within bare harness scope. |

## Benchmark expectations

| Operation | Needed before implementation? | Needed before release? | Notes |
|---|---:|---:|---|
| setup | no | yes | Covered by later benchmarks if needed and simulation setup cost. |
| legal action generation | no | yes | Gate 1 benchmark floor. |
| preview | no | no | Preview may be minimal or omitted with rationale. |
| validation/apply action | no | yes | Gate 1 benchmark floor. |
| public/private view generation | no | yes | Public view only; no private view. |
| effect filtering | no | yes | Gate 1 benchmark floor. |
| serialization/deserialization | no | yes | Gate 1 benchmark floor. |
| replay throughput/hash | no | yes | Gate 1 benchmark floor. |
| random playout throughput | no | yes | Stage 1 throughput budget. |
| bot decision latency | no | yes | Level 0 random legal bot benchmark. |
| WASM/browser smoke | no | yes | UI smoke, not native benchmark. |

## Admission decision

Decision: admitted with explicit constraints

Decision rationale:

- Requirements-first documents exist and pin one small original variant.
- IP/source notes record neutral naming and no copied prose/assets.
- The mechanic is first-use and stays local.
- Implementation can begin in later tickets without inventing rules.

Explicit constraints if admitted conditionally:

- Do not add Rust code, `Cargo.toml`, static data, or UI under GAT1RACTON-001.
- Do not implement a take-away pile, multi-pile, misere, randomized, or
  configurable variant.
- Do not put counter/seat/target vocabulary into `engine-core`.
- Do not let TypeScript decide legality in later UI work.

## Blocking issues

| Issue | Required fix | Owner | Blocks coding? |
|---|---|---|---:|
| none for Gate 1 implementation start | not applicable | Rulepath | no |

## Sign-off

Prepared by: `Codex`

Reviewed by: `joeloverbeck`

Date: 2026-06-05
