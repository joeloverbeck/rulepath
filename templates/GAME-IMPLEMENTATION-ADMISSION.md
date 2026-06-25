# <game_id> Implementation Admission

Game ID: `<game_id>`

Public display name: `<display_name>`

Implemented variant: `<variant>`

Roadmap stage/gate: `<stage_or_gate>`

Public role: scaffolding | UI smoke | public showcase | hidden-info proof | original portfolio game | maintenance | other: `<role>`

Prepared by: `<name/agent>`

Date: YYYY-MM-DD

Template realignment mapping: report `B-07 -> GAME-IMPLEMENTATION-ADMISSION.md`

Evidence receipt: [`GAME-EVIDENCE.md`](GAME-EVIDENCE.md)

## Purpose

This is a pre-build admission receipt. It answers whether implementation work
may begin under the current Rulepath foundation docs.

It is not a ticket, implementation plan, release checklist, or post-build proof
report. Post-build proof lives in `GAME-EVIDENCE.md`, domain evidence files,
and the public release checklist.

Admission does not waive later rule coverage, tests, traces, benchmarks, UI
polish, bot evidence, no-leak proof, or release gates.

## Source, Scope, and Rule Readiness

| Admission surface | Status | Evidence link | Notes/blockers |
|---|---|---|---|
| source/IP notes are ready | ready/blocked/constrained | `<GAME-SOURCES.md>` | `<notes>` |
| original rules with stable rule IDs are ready | ready/blocked/constrained | `<GAME-RULES.md>` | `<notes>` |
| implemented variants and out-of-scope variants are explicit | ready/blocked/constrained | `<GAME-RULES.md or GAME-SOURCES.md>` | `<notes>` |
| supported seat counts and stable labels are explicit | ready/blocked/constrained | `<GAME-RULES.md or GAME-MECHANICS.md>` | `<notes>` |
| rule coverage strategy is identified | ready/blocked/constrained | `<GAME-RULE-COVERAGE.md>` | `<notes>` |

## Novel Mechanics and Pressure

| Surface | Status | Evidence link | Blocks implementation? |
|---|---|---|---:|
| mechanic inventory complete enough to start | ready/blocked/constrained | `<GAME-MECHANICS.md>` | yes/no |
| behavioral primitive-pressure decision, if needed | ready/blocked/not applicable: `<rationale>` | `<PRIMITIVE-PRESSURE-LEDGER.md or atlas link>` | yes/no |
| mechanical-scaffolding reuse-first audit complete | ready/blocked | `<GAME-MECHANICS.md reuse-first audit section>` | yes |
| matching registered/promoted scaffolding will be reused, or accepted exceptions are linked | ready/blocked/not applicable: `<rationale>` | `<MSC entry ids / register exception>` | yes |
| newly anticipated behavior-free scaffolding has a planned register disposition | ready/blocked/not applicable: `<rationale>` | `<planned MSC entry or no-new-scaffolding rationale>` | yes |
| expected prior-game matching sites and follow-on/no-follow-on disposition are identified | ready/blocked/not applicable: `<rationale>` | `<GAME-MECHANICS.md / register link>` | yes |
| ADR needed for boundary-changing work | yes/no | `<docs/adr/... or rationale>` | yes/no |

## Boundary Risks

| Boundary | Admission result | Evidence/notes |
|---|---|---|
| `engine-core` remains generic and noun-free | pass/fail/constrained | `<notes>` |
| no static-data behavior language is introduced | pass/fail/constrained | `<notes>` |
| Rust remains the authority for legality, validation, effects, views, and bots | pass/fail/constrained | `<notes>` |
| hidden-information risk has a named proof plan | pass/fail/not applicable: `<rationale>` | `<GAME-EVIDENCE.md planned row or coverage link>` |
| private licensed content is excluded from public paths | pass/fail/not applicable: `<rationale>` | `<GAME-SOURCES.md or release-review link>` |

## Required Evidence Profile

| Evidence area | Required before coding? | Required before release? | Owner/link |
|---|---:|---:|---|
| conformance receipt | yes/no | yes | `<GAME-EVIDENCE.md>` |
| named rule tests and coverage | yes/no | yes | `<GAME-RULE-COVERAGE.md>` |
| replay/hash and serialization proof | yes/no | yes/no | `<GAME-EVIDENCE.md trace/replay rows>` |
| no-leak proof | yes/no/not applicable | yes/no/not applicable | `<GAME-EVIDENCE.md hidden-info matrix>` |
| UI evidence | yes/no/not applicable | yes/no/not applicable | `<GAME-UI.md>` |
| bot evidence | yes/no/not applicable | yes/no/not applicable | `<GAME-AI.md>` |
| benchmark evidence | yes/no/not applicable | yes/no/not applicable | `<GAME-BENCHMARKS.md>` |
| pre-implementation scaffolding audit receipt | yes | yes | `<GAME-EVIDENCE.md mechanic/scaffolding rows>` |
| post-implementation register freshness and prior-game refactor receipt | no | yes | `<GAME-EVIDENCE.md mechanic/scaffolding rows>` |
| CI scaffolding-audit record | no | yes | `<ci/scaffolding-audits.json game row>` |

## Delta Admission

Use this section for later expansions so the original admission does not
accumulate a second implementation report.

| Delta | Scope change | New or changed risks | Required evidence update | Decision |
|---|---|---|---|---|
| `<delta id>` | `<scope>` | `<risks>` | `<GAME-EVIDENCE.md/domain link>` | admitted / blocked / admitted with constraints |

## Admission Decision

Admission is blocked when the reuse-first audit is missing, when a known matching
helper is being reimplemented without an accepted exception, or when anticipated
new scaffolding has no register/closeout plan. Admission does not require a
first-use candidate to be promoted.

Decision: admitted / blocked / admitted with explicit constraints

Decision rationale:

- `<rationale>`

Explicit constraints if admitted conditionally:

- `<constraint>`

## Blocking Issues

| Issue | Required fix | Owner | Blocks coding? |
|---|---|---|---:|
| `<issue>` | `<fix>` | `<owner>` | yes/no |

## Sign-off

Prepared by: `<name/agent>`

Reviewed by: `<name>`

Date: YYYY-MM-DD
