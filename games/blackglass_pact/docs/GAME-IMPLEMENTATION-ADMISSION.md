# Blackglass Pact Implementation Admission

Game ID: `blackglass_pact`

Public display name: `Blackglass Pact`

Implemented variant: `blackglass_pact_standard`

Roadmap stage/gate: Stage 18 / Gate 18 partnership trick-taking proof

Public role: hidden-info proof / original portfolio game / first forward-v1 game

Prepared by: `Codex`

Date: 2026-06-25

Evidence receipt: [GAME-EVIDENCE.md](GAME-EVIDENCE.md)

## Purpose

This is the pre-build admission receipt for Blackglass Pact. It answers whether
implementation work may begin under the current Rulepath foundation docs and
the ADR 0008 `forward-v1` mechanical-scaffolding obligation.

Admission does not waive later rule coverage, tests, traces, benchmarks, UI
polish, bot evidence, no-leak proof, forward-v1 machine receipt, release
checklist, or capstone gates.

## Source, Scope, And Rule Readiness

| Admission surface | Status | Evidence link | Notes/blockers |
|---|---|---|---|
| source/IP notes are ready | ready | [SOURCES.md](SOURCES.md) | Source-use statement, consulted sources, variant reconciliation, and pending human review are recorded. |
| original rules with stable rule IDs are ready | ready | [RULES.md](RULES.md) | Every Gate 18 appendix `BP-*` ID is represented. |
| implemented variants and out-of-scope variants are explicit | ready | [RULES.md](RULES.md), [SOURCES.md](SOURCES.md) | Only `blackglass_pact_standard` is admitted. |
| supported seat counts and stable labels are explicit | ready | [RULES.md](RULES.md), [MECHANICS.md](MECHANICS.md) | Exactly four seats; North/East/South/West; fixed public teams. |
| rule coverage strategy is identified | ready | [RULE-COVERAGE.md](RULE-COVERAGE.md) | Initial rows are open until implementation tickets land evidence. |

## Novel Mechanics And Pressure

| Surface | Status | Evidence link | Blocks implementation? |
|---|---|---|---:|
| mechanic inventory complete enough to start | ready | [MECHANICS.md](MECHANICS.md) | no |
| behavioral primitive-pressure decision | ready | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) | no |
| mechanical-scaffolding reuse-first audit complete | ready | [MECHANICS.md](MECHANICS.md#mechanical-scaffolding-reuse-first-audit) | no |
| matching registered/promoted scaffolding will be reused, or accepted exceptions are linked | ready | [MECHANICS.md](MECHANICS.md#mechanical-scaffolding-reuse-first-audit) | no |
| newly anticipated behavior-free scaffolding has a planned register disposition | ready | [MECHANICS.md](MECHANICS.md#mechanical-scaffolding-reuse-first-audit) | no |
| expected prior-game matching sites and follow-on/no-follow-on disposition are identified | ready | [MECHANICS.md](MECHANICS.md#mechanical-scaffolding-reuse-first-audit) | no |
| ADR needed for boundary-changing work | no | No kernel, DSL, YAML, trace/hash, visibility, or architecture exception is admitted. | no |

## Boundary Risks

| Boundary | Admission result | Evidence/notes |
|---|---|---|
| `engine-core` remains generic and noun-free | pass | Card, hand, suit, rank, trick, trump, bid, contract, nil, blind nil, bag, team, and partnership stay in `games/blackglass_pact`. |
| no static-data behavior language is introduced | pass | Data may carry identity/presentation/fixtures/traces only; no formulas, selectors, triggers, or scripts. |
| Rust remains authority for legality, validation, effects, views, and bots | pass | `RULES.md` and `MECHANICS.md` require Rust-owned blind/bid/play/scoring/projection/bot behavior. |
| hidden-information risk has a named proof plan | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md) and [GAME-EVIDENCE.md](GAME-EVIDENCE.md) name blind no-leak, partner no-leak, pairwise matrix, exports, and e2e no-leak. |
| private licensed content is excluded from public paths | pass/constrained | [SOURCES.md](SOURCES.md) records no copied source/prose/assets and pending human IP/public-release review. |

## Required Evidence Profile

| Evidence area | Required before coding? | Required before release? | Owner/link |
|---|---:|---:|---|
| conformance receipt | yes | yes | [GAME-EVIDENCE.md](GAME-EVIDENCE.md) |
| named rule tests and coverage | yes (strategy) | yes | [RULE-COVERAGE.md](RULE-COVERAGE.md) |
| replay/hash and serialization proof | no | yes | [GAME-EVIDENCE.md](GAME-EVIDENCE.md#replay-and-hash-compatibility) |
| no-leak proof | yes (plan) | yes | [GAME-EVIDENCE.md](GAME-EVIDENCE.md#hidden-information-no-leak-matrix) |
| UI evidence | no | yes | `UI.md` in GAT18BLAPACSPA-017 |
| bot evidence | no | yes | `AI.md`, `COMPETENT-PLAYER.md`, and bot tests in later tickets |
| benchmark evidence | no | yes | `BENCHMARKS.md` and `cargo bench -p blackglass_pact` in later tickets |
| pre-implementation scaffolding audit receipt | yes | yes | [MECHANICS.md](MECHANICS.md#mechanical-scaffolding-reuse-first-audit) |
| post-implementation register freshness and prior-game refactor receipt | no | yes | GAT18BLAPACSPA-018 |
| CI scaffolding-audit record | no | yes | `ci/scaffolding-audits.json` in GAT18BLAPACSPA-018 |

## Admission Decision

Decision: admitted with explicit constraints

Decision rationale:

- GAT18BLAPACSPA-001 completed the rules/source/coverage contract with the
  exact Gate 18 `BP-*` rule-ID set.
- The C-01 through C-10 forward-v1 reuse-first audit is complete in
  [MECHANICS.md](MECHANICS.md).
- The primitive-pressure ledger records the required pre-code decisions:
  trick reuse, numeric keep-local, and partnership first-use local-only.
- No active FOUNDATIONS §12 stop condition is present in the admitted scope.

Explicit constraints:

- Reuse `game-stdlib::trick_taking` helpers unchanged; do not broaden them for
  teams, broken spades, bidding, scoring, or visibility.
- Keep team/partnership semantics game-local; do not fold them into generic
  seat identity.
- Keep numeric bids, nils, blind nil, contracts, bags, scoring, and outcomes
  in typed Rust.
- Keep all production behavior out of static data and TypeScript.
- Do not introduce trace/hash/fixture/export/RNG migration without explicit
  authority and evidence.
- Human IP/public-release review remains pending before public release.

## Blocking Issues

| Issue | Required fix | Owner | Blocks coding? |
|---|---|---|---:|
| none for implementation admission | not applicable | not applicable | no |
| human IP/public-release review pending | complete review before public release | Rulepath maintainers | no for coding; yes before release |
| forward-v1 machine receipt absent | add `blackglass_pact` row to `ci/scaffolding-audits.json` and pass checker | GAT18BLAPACSPA-018 | no for coding; yes before gate closeout |

## Sign-off

Prepared by: `Codex`

Reviewed by: pending maintainer review

Date: 2026-06-25
