# <game_id> AI Registry

Game ID: `<game_id>`

Implemented variant: `<variant>`

Rules version: `<rules_version>`

Last updated: YYYY-MM-DD

Prepared by: `<name/agent>`

Template realignment mapping: report `B-12 -> GAME-AI.md`

Evidence receipt: [`GAME-EVIDENCE.md`](GAME-EVIDENCE.md)

## Purpose

This document is the shipped bot registry. It records which bot levels exist,
their policy identifiers, public-default status, allowed information access,
deterministic tie-break source, and evidence links.

It does not define strategy, duplicate the Level 2 evidence pack, or restate
the full no-leak, benchmark, simulation, replay, or release proof. Those
surfaces are linked through `GAME-EVIDENCE.md` and the domain evidence files.

Public v1/v2 bots MUST NOT use MCTS, ISMCTS, Monte Carlo-style bots, ML, or
RL.

## Registry

Keep explicit `not implemented` rows for levels that are intentionally absent.

| Bot level | Implemented? | Policy ID/version | Public default? | Supported seats | Allowed viewer/input | Deterministic source | Evidence links | Known limitation |
|---:|---|---|---:|---|---|---|---|---|
| 0 random legal | required / implemented / not implemented: `<reason>` | `<policy_id>` | yes/no | `<min..max seats>` | legal action tree only | seeded random legal choice | `<tests>; GAME-EVIDENCE.md bot row` | random; not competent |
| 1 baseline | implemented / deferred / not planned: `<reason>` | `<policy_id>` | yes/no | `<min..max seats>` | allowed seat view | seeded tie-break / bounded deterministic order | `<tests>; GAME-EVIDENCE.md bot row` | `<limitation>` |
| 2 authored policy | implemented / blocked by evidence pack / deferred / not planned: `<reason>` | `<policy_id>` | yes/no | `<min..max seats>` | allowed seat view | strategy-pack tie-break rule | `<BOT-STRATEGY-EVIDENCE-PACK.md>; <COMPETENT-PLAYER.md>; GAME-EVIDENCE.md bot row` | `<limitation>` |
| 3 shallow deterministic search | not allowed / ADR-needed / implemented: `<reason>` | `<policy_id>` | yes/no | `<min..max seats>` | allowed seat view for perfect-information games only | documented bounded evaluator | `<ADR or benchmark/evidence link>` | `<limitation>` |

## Information Access Receipt

| Check | Status | Evidence link |
|---|---|---|
| bot input is generated from Rust legal action/view authority | pass/fail/blocker | `<test or code link>` |
| public bots receive only acting-seat-allowed information | pass/fail/not applicable: `<rationale>` | `<GAME-EVIDENCE.md hidden-info matrix row>` |
| explanations are viewer-safe | pass/fail/not applicable: `<rationale>` | `<test/report link>` |
| candidate rankings are dev-only or redacted | pass/fail/not applicable: `<rationale>` | `<test/report link>` |
| replay/hash determinism for bot decisions is covered | pass/fail/not applicable: `<rationale>` | `<replay/hash evidence link>` |

## Public Default Decision

Decision: no public bot / Level 0 / Level 1 / Level 2 / constrained default

Decision rationale:

- `<rationale>`

Release blockers or constraints:

- `<constraint or none>`

## Evidence Pointers

| Evidence | Link | Status |
|---|---|---|
| conformance receipt | `<GAME-EVIDENCE.md>` | pass/fail/blocker |
| competent-player analysis, if strategy matters | `<COMPETENT-PLAYER.md or not applicable: rationale>` | complete/partial/not applicable/blocker |
| Level 2 strategy evidence pack, if shipped | `<BOT-STRATEGY-EVIDENCE-PACK.md or not applicable: rationale>` | complete/partial/not applicable/blocker |
| benchmark workload IDs | `<GAME-BENCHMARKS.md or report link>` | pass/fail/not applicable |
| simulation metrics | `<command/report link>` | pass/fail/not applicable |

