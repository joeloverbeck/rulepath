# <game_id> Implementation Admission

Game ID: `<game_id>`

Public display name: `<display_name>`

Implemented variant: `<variant>`

Roadmap stage/gate: `<stage_or_gate>`

Public role: scaffolding | UI smoke | public showcase | hidden-info proof | original portfolio game | maintenance | other: `<role>`

Prepared by: `<name/agent>`

Date: YYYY-MM-DD

## Purpose

This is a gate receipt before serious coding starts.

It is not a ticket. It is not an implementation plan. It decides whether implementation work may start under the corrected Rulepath foundation docs.

Admission does not waive later rule coverage, tests, traces, benchmarks, UI polish, bot evidence, or release gates.

## Prerequisite documents

| Prerequisite | Path | Complete? | Notes/blockers |
|---|---|---:|---|
| source/IP notes | `<path/to/GAME-SOURCES.md>` | yes/no | `<notes>` |
| original rules with stable rule IDs | `<path/to/GAME-RULES.md>` | yes/no | `<notes>` |
| rule coverage matrix | `<path/to/GAME-RULE-COVERAGE.md>` | yes/no | `<notes>` |
| mechanic inventory | `<path/to/GAME-MECHANICS.md>` | yes/no | `<notes>` |
| primitive-pressure ledger, if needed | `<path/to/PRIMITIVE-PRESSURE-LEDGER.md>` | yes/no/not applicable | `<notes>` |
| competent-player analysis, if strategy matters | `<path/to/COMPETENT-PLAYER.md>` | yes/no/not applicable | `<notes>` |
| ADR, if boundary-changing | `<path/to/docs/adr/...>` | yes/no/not applicable | `<notes>` |

## Source and IP readiness

| Check | Status | Evidence/notes |
|---|---|---|
| consulted sources recorded with dates | ready / blocked / constrained | `<notes>` |
| sources used only for verification/context | ready / blocked / constrained | `<notes>` |
| Rulepath rules prose is original | ready / blocked / constrained | `<notes>` |
| no copied card/component text | ready / blocked / constrained / not applicable | `<notes>` |
| no copied art/icons/screenshots/scans/fonts/trade dress | ready / blocked / constrained | `<notes>` |
| public naming rationale recorded | ready / blocked / constrained | `<notes>` |
| private licensed content excluded from public files | ready / blocked / constrained / not applicable | `<notes>` |
| human/legal review triggers cleared or recorded | ready / blocked / constrained | `<notes>` |

## Rule-ID readiness

| Check | Status | Evidence/notes |
|---|---|---|
| every implemented rule has a stable rule ID | ready / blocked / constrained | `<notes>` |
| rule IDs use stable prefixes, not section-only references | ready / blocked / constrained | `<notes>` |
| ambiguities have chosen resolutions and IDs | ready / blocked / constrained / not applicable | `<notes>` |
| out-of-scope variants are explicit | ready / blocked / constrained / not applicable | `<notes>` |
| rule-ID migration policy is understood | ready / blocked / constrained | `<notes>` |

Rule IDs MUST remain stable after implementation unless intentionally migrated with notes and coverage updates.

## Rule coverage readiness

| Check | Status | Evidence/notes |
|---|---|---|
| coverage matrix has one row per rule ID | ready / blocked / constrained | `<notes>` |
| deferred/unsupported/not applicable rows are explicit | ready / blocked / constrained | `<notes>` |
| primary Rust test strategy is identified | ready / blocked / constrained | `<notes>` |
| golden trace needs are identified | ready / blocked / constrained | `<notes>` |
| invalid/stale diagnostic trace needs are identified | ready / blocked / constrained | `<notes>` |
| replay/hash requirements are identified | ready / blocked / constrained | `<notes>` |
| serialization requirements are identified | ready / blocked / constrained | `<notes>` |
| visibility/no-leak requirements are identified | ready / blocked / constrained / not applicable | `<notes>` |
| UI smoke coverage is scoped as smoke only | ready / blocked / constrained / not applicable | `<notes>` |

## Mechanic inventory readiness

| Check | Status | Evidence/notes |
|---|---|---|
| all mechanic atlas categories are inventoried | ready / blocked / constrained | `<notes>` |
| local mechanics are named and scoped | ready / blocked / constrained | `<notes>` |
| reused primitives are justified | ready / blocked / constrained / not applicable | `<notes>` |
| repeated-shape comparison is complete | ready / blocked / constrained / not applicable | `<notes>` |
| second-use review is recorded when applicable | ready / blocked / constrained / not applicable | `<notes>` |
| third-use hard gate is cleared when applicable | ready / blocked / constrained / not applicable | `<notes>` |
| repo atlas update required? | yes/no | `<notes>` |

A third official game with the same mechanic shape is blocked until the primitive-pressure ledger records reuse, promotion, explicit defer/reject, or ADR-required.

## Primitive-pressure status

| Mechanic shape | Status | Decision/evidence | Blocks implementation? |
|---|---|---|---:|
| `<shape>` | local-only / repeated-shape candidate / extraction required / promoted primitive / rejected/deferred with rationale / ADR-required | `<evidence>` | yes/no |

## Engine-core contamination review

| Boundary check | Result | Notes |
|---|---|---|
| no game noun needs to enter `engine-core` | pass / fail / constrained | `<notes>` |
| no rule helper needs to enter `engine-core` | pass / fail / constrained | `<notes>` |
| no private licensed name/data needs to enter kernel contracts | pass / fail / constrained | `<notes>` |
| any generic contract change has ADR or explicit non-goal | pass / fail / constrained | `<notes>` |

## Static-data behavior review

| Check | Result | Notes |
|---|---|---|
| static data limited to typed content, parameters, metadata, fixtures, traces, or reports | pass / fail / constrained | `<notes>` |
| no selectors, rule branches, loops, triggers, tactical conditions, or exception logic in data | pass / fail / constrained | `<notes>` |
| no YAML by default | pass / fail / constrained | `<notes>` |
| no DSL at project start | pass / fail / constrained | `<notes>` |

## Hidden-information risk review

| Surface | Risk level | Required safeguard/test | Admission status |
|---|---|---|---|
| public/browser payload | none/low/medium/high | `<test>` | ready/blocked/not applicable |
| action tree | none/low/medium/high | `<test>` | ready/blocked/not applicable |
| preview | none/low/medium/high | `<test>` | ready/blocked/not applicable |
| diagnostics/effect log | none/low/medium/high | `<test>` | ready/blocked/not applicable |
| DOM/test IDs/local storage/replay export | none/low/medium/high | `<test>` | ready/blocked/not applicable |
| bot explanations/candidate rankings | none/low/medium/high | `<test>` | ready/blocked/not applicable |
| dev inspector | none/low/medium/high | `<test>` | ready/blocked/not applicable |

## Bot level required for this stage

| Level | Required before public release? | Coding may start now? | Evidence required |
|---:|---:|---:|---|
| 0 random legal | yes/no | yes/no | legal action API, deterministic seed, simulations |
| 1 baseline | yes/no | yes/no | obvious tactics, legality/determinism/explanation tests |
| 2 authored policy | yes/no | yes/no | completed `COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md` before coding |
| 3 shallow deterministic search | yes/no | yes/no | allowed only for small perfect-information games with benchmark evidence |

Public v1/v2 bots MUST NOT use MCTS, ISMCTS, Monte Carlo-style bots, ML, or RL.

## UI exposure expectations

| Check | Status | Notes |
|---|---|---|
| web-exposed in this stage? | yes/no | `<notes>` |
| React + SVG default accepted | yes/no/constrained | `<notes>` |
| legal-action tree maps to UI controls | ready/blocked/constrained | `<notes>` |
| TypeScript presentation-only boundary understood | ready/blocked/constrained | `<notes>` |
| effect-driven animation expectations identified | ready/blocked/constrained/not applicable | `<notes>` |
| accessibility/reduced-motion/responsive expectations identified | ready/blocked/constrained/not applicable | `<notes>` |

## Benchmark expectations

| Operation | Needed before implementation? | Needed before release? | Notes |
|---|---:|---:|---|
| setup | yes/no | yes/no | `<notes>` |
| legal action generation | yes/no | yes/no | `<notes>` |
| preview | yes/no | yes/no | `<notes>` |
| validation/apply action | yes/no | yes/no | `<notes>` |
| public/private view generation | yes/no | yes/no | `<notes>` |
| effect filtering | yes/no | yes/no | `<notes>` |
| serialization/deserialization | yes/no | yes/no | `<notes>` |
| replay throughput/hash | yes/no | yes/no | `<notes>` |
| random playout throughput | yes/no | yes/no | `<notes>` |
| bot decision latency | yes/no | yes/no | `<notes>` |
| WASM/browser smoke | yes/no | yes/no | `<notes>` |

## Admission decision

Decision: admitted / blocked / admitted with explicit constraints

Decision rationale:

- `<rationale>`

Explicit constraints if admitted conditionally:

- `<constraint>`

## Blocking issues

| Issue | Required fix | Owner | Blocks coding? |
|---|---|---|---:|
| `<issue>` | `<fix>` | `<owner>` | yes/no |

## Sign-off

Prepared by: `<name/agent>`

Reviewed by: `<name>`

Date: YYYY-MM-DD
