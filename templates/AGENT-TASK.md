# Agent Task

Task ID: `<task_id>`

Task kind: rules | game implementation | bot work | UI work | testing | benchmark work | refactor | primitive-pressure review | docs | release prep | other: `<kind>`

Prepared by: `<name/agent>`

Date: YYYY-MM-DD

## Context

This task is bounded. It is not a request to redesign Rulepath.

Foundation documents to obey:

- `docs/README.md`
- `docs/FOUNDATIONS.md`
- `docs/ARCHITECTURE.md`
- `docs/ENGINE-GAME-DATA-BOUNDARY.md`
- `docs/OFFICIAL-GAME-CONTRACT.md`
- `docs/MECHANIC-ATLAS.md`
- `docs/AI-BOTS.md`
- `docs/UI-INTERACTION.md`
- `docs/TESTING-REPLAY-BENCHMARKING.md`
- `docs/ROADMAP.md`
- `docs/IP-POLICY.md`
- `docs/AGENT-DISCIPLINE.md`
- `docs/SOURCES.md`

Other required docs, ADRs, source notes, or filled templates:

| Path/source | Must read? | Must update? | Why |
|---|---:|---:|---|
| `<path>` | yes/no | yes/no | `<reason>` |

## Target

Exact target crate, module, game, doc, template, tool, or release artifact:

- `<target>`

Roadmap stage/gate:

- `<stage_or_gate>`

Official-game lifecycle state:

- sources | rules | coverage | mechanics | admitted | implementation | bot | UI | benchmarks | release prep | maintenance

## Mechanics and primitive-pressure status

| Mechanic shape | Current status | Ledger/atlas path | Task impact |
|---|---|---|---|
| `<mechanic_shape>` | local-only / repeated-shape candidate / extraction required / promoted primitive / rejected/deferred with rationale / ADR-required | `<path>` | `<impact>` |

A third official game with the same mechanic shape MUST NOT proceed until the primitive-pressure ledger records reuse, promotion, explicit defer/reject, or ADR-required.

## Goal

When complete, the following observable result MUST be true:

- `<observable_result>`

## Acceptance evidence

| Evidence type | Required? | Exact evidence expected |
|---|---:|---|
| source/rule docs | yes/no | `<paths or rows>` |
| implementation behavior | yes/no | `<observable behavior>` |
| tests | yes/no | `<test names>` |
| golden traces | yes/no | `<trace files>` |
| replay/hash checks | yes/no | `<commands/evidence>` |
| serialization checks | yes/no | `<commands/evidence>` |
| simulations/fuzz | yes/no | `<seeds/counts/commands>` |
| benchmarks | yes/no | `<benchmark names/thresholds>` |
| UI smoke/accessibility | yes/no | `<smoke tests>` |
| bot evidence/explanations | yes/no | `<docs/tests>` |
| IP review | yes/no | `<source/IP evidence>` |

## Non-goals

Do not do these things:

- `<non_goal>`

## Forbidden changes

- Do not add game nouns to `engine-core`.
- Do not move behavior into static data.
- Do not add YAML without ADR.
- Do not create a DSL.
- Do not let TypeScript decide legality.
- Do not let bots access hidden information unavailable to their seat.
- Do not add private licensed content to public files.
- Do not update golden traces without explaining the intentional behavior or format change.
- Do not optimize without benchmark evidence.
- Do not implement public MCTS, ISMCTS, Monte Carlo-style bots, ML, or RL in v1/v2.
- Do not weaken, delete, or rewrite tests merely to get green output.
- Do not use copied rulebook prose, card text, screenshots, scans, fonts, icons, board art, or trade dress.
- `<task_specific_forbidden_change>`

## Implementation boundaries

| Boundary | Rule for this task | Evidence required |
|---|---|---|
| Rust authority | Rust owns setup, legality, validation, transitions, effects, replay, visibility, serialization, and bots. | `<tests/docs>` |
| TypeScript presentation | TypeScript maps Rust legal choices to UI and renders viewer-safe payloads only. | `<UI tests/docs>` |
| static data | Static data may contain typed content, parameters, metadata, fixtures, traces, and reports only. | `<schema/review>` |
| `engine-core` | Generic contracts only; no game nouns or mechanics. | `<boundary review>` |
| `game-stdlib` | Only earned typed helpers after primitive-pressure evidence. | `<ledger/atlas>` |
| private content | Private licensed stress tests are isolated, optional, non-public, and non-architectural. | `<IP review>` |

## Tests required

| Test category | Required? | Specific tests or commands | Notes |
|---|---:|---|---|
| unit tests | yes/no | `<test_name>` | `<notes>` |
| named rule tests | yes/no | `<test_name>` | map to rule IDs |
| golden traces | yes/no | `<trace_file>` | preserve/update policy required |
| property/invariant tests | yes/no | `<test_name>` | `<notes>` |
| simulation/fuzz tests | yes/no | `<command>` | include seeds/counts |
| replay/hash tests | yes/no | `<command>` | deterministic hash evidence |
| serialization tests | yes/no | `<test_name>` | version compatibility where relevant |
| visibility/no-leak tests | yes/no | `<test_name>` | required for hidden information |
| AI legal-action tests | yes/no | `<test_name>` | bots use legal action API |
| explanation tests | yes/no | `<test_name>` | viewer-safe explanations |
| UI smoke tests | yes/no | `<test_name>` | smoke only; not rule proof |
| accessibility tests | yes/no | `<test_name>` | keyboard/focus/labels/axe where practical |

## Traces required

| Trace | Purpose | Rule IDs | Update policy |
|---|---|---|---|
| `<trace_file>` | `<purpose>` | `<rule_ids>` | preserve / update only with rationale / new |

Golden traces MUST NOT be updated as a convenience. Explain every intentional behavior, format, or hash change.

## Simulations required

| Simulation | Seeds/count | Required metrics | Failure-report requirement |
|---|---:|---|---|
| `<simulation>` | `<count>` | games completed, terminal outcomes, illegal action attempts, invariant failures, average length, playout throughput, failing seed command stream | `<requirement>` |

## Benchmarks required

| Benchmark | Target/threshold | Baseline source | Required before merge? |
|---|---:|---|---:|
| `<benchmark>` | `<target>` | `<baseline>` | yes/no |

Native Rust benchmark evidence is primary. WASM/browser smoke benchmark evidence is secondary and required when the task changes public-web hot paths.

## Documentation required

| Document/template | Update required? | Required change |
|---|---:|---|
| `GAME-SOURCES.md` | yes/no | `<change>` |
| `GAME-RULES.md` | yes/no | `<change>` |
| `GAME-RULE-COVERAGE.md` | yes/no | `<change>` |
| `GAME-MECHANICS.md` | yes/no | `<change>` |
| `COMPETENT-PLAYER.md` | yes/no | `<change>` |
| `BOT-STRATEGY-EVIDENCE-PACK.md` | yes/no | `<change>` |
| `GAME-AI.md` | yes/no | `<change>` |
| `GAME-UI.md` | yes/no | `<change>` |
| `GAME-BENCHMARKS.md` | yes/no | `<change>` |
| `PRIMITIVE-PRESSURE-LEDGER.md` | yes/no | `<change>` |
| `PUBLIC-RELEASE-CHECKLIST.md` | yes/no | `<change>` |
| foundation docs/ADR | no by default | ADR required only for architectural decisions |

## Hidden-information review

For hidden-information games or systems, review every surface. Mark `not applicable` only with rationale.

| Surface | Risk | Safeguard/test |
|---|---|---|
| public/browser payload | `<risk>` | `<test>` |
| action tree | `<risk>` | `<test>` |
| preview | `<risk>` | `<test>` |
| diagnostics | `<risk>` | `<test>` |
| effect log | `<risk>` | `<test>` |
| DOM attributes | `<risk>` | `<test>` |
| test IDs | `<risk>` | `<test>` |
| logs | `<risk>` | `<test>` |
| local storage | `<risk>` | `<test>` |
| replay export | `<risk>` | `<test>` |
| bot explanations | `<risk>` | `<test>` |
| candidate rankings | `<risk>` | `<test>` |
| dev inspector | `<risk>` | `<test>` |

## IP review

| Content type | Status | Evidence/action |
|---|---|---|
| rules prose | original Rulepath prose / not applicable / human review needed | `<evidence>` |
| card/component text | none / original / licensed / human review needed | `<evidence>` |
| art/icons/screenshots/scans | none / original / licensed / generated-reviewed / human review needed | `<evidence>` |
| fonts | system only / license-reviewed / human review needed | `<evidence>` |
| names/trademarks/trade dress | neutral / reviewed / human review needed | `<evidence>` |
| private licensed content | not present / isolated private-only / blocked | `<evidence>` |

## Failing-test protocol

When tests fail, the agent MUST:

1. Determine whether the failing tests are still valid.
2. Determine whether the issue is in the system under test or the test suite.
3. Fix the issue.
4. Add or update regression coverage.
5. Report the change and the evidence.

## Output format

Agents MUST output complete files or coherent complete sections, not diffs.

Generated implementation code is allowed only when the task explicitly requests implementation. Documentation/template tasks MUST NOT produce implementation code.

## Final report format

Report:

- files changed;
- tests added/updated;
- traces added/updated and why;
- simulations run;
- benchmarks run;
- docs updated;
- boundary decisions;
- hidden-information review result;
- IP review result;
- unresolved questions;
- commands for human verification.

## Review checklist

- Task kind and target were bounded.
- Foundation docs and relevant filled templates were followed.
- Rust owns behavior.
- TypeScript does not decide legality.
- Static data remains typed content, parameters, metadata, fixtures, traces, or reports only.
- `engine-core` remains noun-free.
- `game-stdlib` extraction is earned or deferred by ledger.
- Replay and hashes remain deterministic.
- Hidden information is safe across browser payloads, DOM, logs, storage, diagnostics, previews, bot explanations, candidate rankings, and replay exports.
- Bots use allowed views and legal action APIs.
- Tests, traces, simulations, and benchmarks cover the work.
- Public files are IP-safe.
- Golden traces were preserved or intentionally updated with rationale.
- Output is bounded and reviewable.
