# Starbridge Crossing Implementation Admission

Game ID: `starbridge_crossing`

Public display name: `Starbridge Crossing`

Implemented variant: `starbridge_crossing_classic_star_v1`

Roadmap stage/gate: Public scaling phase / Gate 20 Star Halma topology proof

Public role: perfect-information large-board topology proof / third
forward-v1 game

Prepared by: `Codex`

Date: 2026-06-27

Evidence receipt: `GAME-EVIDENCE.md` in a later ticket

## Purpose

This is the pre-code admission receipt for Starbridge Crossing. It answers
whether implementation work may begin under the current Rulepath foundation
docs, the Gate 20 spec, the topology/path-jump primitive-pressure hard gate,
and the ADR 0008 `forward-v1` mechanical-scaffolding obligation.

Admission does not waive later rule coverage, tests, traces, benchmarks, UI
polish, bot evidence, all-public no-leak proof, forward-v1 machine receipt,
release checklist, or capstone gates. The post-build `ci/scaffolding-audits.json`
receipt and central register reconciliation are assigned to
GAT20STACROSTA-019.

## Authority References

| Authority | Admission use |
|---|---|
| `docs/FOUNDATIONS.md` | Rust remains behavior authority; `engine-core` stays generic; TypeScript does not decide legality; new official games complete the reuse-first scaffolding audit before serious implementation. |
| `docs/ARCHITECTURE.md` | Rust/WASM boundary, action/view/effect/replay model, and deterministic command-log posture. |
| `docs/ENGINE-GAME-DATA-BOUNDARY.md` | Board, space, peg, topology, path, jump, home, target, and finish nouns stay game-local. |
| `docs/OFFICIAL-GAME-CONTRACT.md` | Requirements-first official-game workflow, docs, tests, traces, bots, benchmarks, and UI evidence. |
| `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` | 2/3/4/6 seat declaration, public observer, larger surface, and Rust-owned action fanout. |
| `docs/MECHANIC-ATLAS.md` | Behavioral topology/path-jump defer/reject decision and no-promotion-debt posture. |
| `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | `MSC-8C-001` through `MSC-8C-010` audit targets and forward per-game maintenance cadence. |
| `docs/adr/0008-mechanical-scaffolding-governance.md` | Reuse-first audit, register-new-on-first-use, and queue-or-dispose prior-game refactor obligation. |
| `docs/adr/0009-replay-fixture-hash-taxonomy.md` | Replay/fixture/hash taxonomy and no migration without explicit authority. |
| `specs/gate-20-starbridge-crossing-star-halma.md` | Gate-local scope, variant pin, acceptance evidence, and Appendix A source/rule seed. |

## Source, Scope, And Rule Readiness

| Admission surface | Status | Evidence | Notes/blockers |
|---|---|---|---|
| source/IP notes are ready | ready | [SOURCES.md](SOURCES.md) | Source-use statement, consulted sources, variant reconciliation, neutral-name rationale, excluded variants, and pending human review are recorded. |
| original rules with stable rule IDs are ready | ready | [RULES.md](RULES.md) | Stable `SC-*` IDs and downstream terminal tokens are present. |
| implemented variant and out-of-scope variants are explicit | ready | [RULES.md](RULES.md), [SOURCES.md](SOURCES.md) | Only `starbridge_crossing_classic_star_v1` is admitted. |
| supported seat counts and stable labels are explicit | ready | [RULES.md](RULES.md), [MECHANICS.md](MECHANICS.md) | Exactly 2, 3, 4, and 6 seats; six point labels. |
| rule coverage strategy is identified | pending later artifact | `RULE-COVERAGE.md` lands later | This does not block the pre-code admission record. |

## Novel Mechanics And Pressure

| Surface | Admission result | Evidence | Blocks implementation? |
|---|---|---|---:|
| mechanic inventory complete enough to start | ready | [MECHANICS.md](MECHANICS.md) | no |
| behavioral primitive-pressure decision | defer/reject promotion | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md), [docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md) | no |
| mechanical-scaffolding reuse-first audit complete | ready | [MECHANICS.md](MECHANICS.md#mechanical-scaffolding-reuse-first-audit) | no |
| matching registered/promoted scaffolding will be reused, or accepted exceptions are linked | ready | [MECHANICS.md](MECHANICS.md#mechanical-scaffolding-reuse-first-audit) | no |
| newly anticipated behavior-free scaffolding has a planned register disposition | `no-new-scaffolding` expected | [MECHANICS.md](MECHANICS.md#mechanical-scaffolding-reuse-first-audit); machine receipt deferred to GAT20STACROSTA-019 | no |
| expected prior-game matching sites and follow-on/no-follow-on disposition are identified | no follow-on unit expected | no new behavior-free shape; `MSC-8C-010` keeps topology/path behavior local | no |
| ADR needed for boundary-changing work | no | No kernel, DSL, YAML, trace/hash, visibility, or architecture exception is admitted. | no |

## Boundary Risks

| Boundary | Admission result | Evidence/notes |
|---|---|---|
| `engine-core` remains generic and noun-free | pass | Board, space, peg, hole, coordinate, adjacency, jump, path, home, target, graph, track, and movement nouns stay in `games/starbridge_crossing`. |
| no static-data behavior language is introduced | pass | Data may carry identity, typed topology metadata, presentation metadata, fixtures, traces, and docs only; no formulas, selectors, triggers, scripts, or behavior-looking fields. |
| Rust remains authority for legality, validation, effects, views, and bots | pass | `RULES.md` requires Rust-owned setup, topology interpretation, moves, blocked pass, finish ranks, projection, replay, and bot behavior. |
| visibility risk has a named proof plan | pass | [MECHANICS.md](MECHANICS.md) records all-public visibility with public/seat parity evidence still required. |
| private licensed content is excluded from public paths | pass/constrained | [SOURCES.md](SOURCES.md) records no copied source prose/assets and pending human IP/public-release review. |
| graph/topology helper pressure is resolved | pass | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) records defer/reject promotion and Gate 21 reopen trigger. |

## Forward-v1 Mechanical-Scaffolding Reuse-First Audit

The detailed C-01 through C-10 audit is in
[MECHANICS.md](MECHANICS.md#mechanical-scaffolding-reuse-first-audit). Admission
summary:

| Checkpoint | Admission result |
|---|---|
| C-01 effects | reuse envelope scaffolding only; effect meaning stays local |
| C-02 seat grammar | reuse canonical seat grammar where applicable; point labels and policy stay local |
| C-03 seat count/ring | reuse structural helpers where they fit; discontinuous seat set and finish skipping stay local |
| C-04 action tree | reuse framing only; step/hop/stop legality stays local |
| C-05 stable bytes | no new production authority unless a later evidence ticket names it |
| C-06 test support | dev/test-only support only |
| C-07 no-leak geometry | reuse as all-public proof geometry; no private class invented |
| C-08 evidence profiles | reuse drivers where profile metadata applies |
| C-09 bounded sampling | no production migration authority expected |
| C-10 Non-Promotion bundle | topology/path/jump/finish/bot/UI behavior rejected from scaffolding and kept local |

Admission disposition: `no-new-scaffolding` expected.

## Required Evidence Profile

| Evidence area | Required before coding? | Required before release/closeout? | Owner/link |
|---|---:|---:|---|
| original rules/source notes | yes | yes | [RULES.md](RULES.md), [SOURCES.md](SOURCES.md) |
| topology primitive-pressure hard gate | yes | yes | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) |
| pre-implementation scaffolding audit receipt | yes | yes | [MECHANICS.md](MECHANICS.md) |
| conformance receipt | strategy only | yes | `GAME-EVIDENCE.md` in a later ticket |
| named rule tests and coverage | strategy only | yes | `RULE-COVERAGE.md` in a later ticket |
| replay/hash and serialization proof | no | yes | later replay/serialization tickets and `GAME-EVIDENCE.md` |
| all-public no-leak proof | plan yes | yes | later visibility/no-leak ticket and browser smoke |
| UI evidence | no | yes | renderer/e2e docs and smoke tickets |
| bot evidence | no | yes | bot tests and AI docs |
| benchmark evidence | no | yes | benchmark ticket |
| post-implementation register freshness and prior-game refactor receipt | no | yes | GAT20STACROSTA-019 |
| CI scaffolding-audit record | no | yes | `ci/scaffolding-audits.json` in GAT20STACROSTA-019 |

## Admission Decision

Decision: admitted for crate skeleton and implementation after this ticket is
archived and committed.

Decision rationale:

- GAT20STACROSTA-001 completed the source and rules contract with original
  prose, stable `SC-*` rule IDs, and downstream terminal tokens.
- GAT20STACROSTA-002 completed the topology/path-jump primitive-pressure hard
  gate with defer/reject promotion and no §10A debt.
- The `MSC-8C-001` through `MSC-8C-010` forward-v1 reuse-first audit is complete
  in [MECHANICS.md](MECHANICS.md).
- Topology, path, jump, finish, progress, bot, visibility, and UI behavior are
  classified as behavior and kept local under `MSC-8C-010` plus the mechanic
  atlas.
- No active FOUNDATIONS §12 stop condition is present in the admitted scope.

Explicit constraints:

- Reuse only behavior-free scaffolding whose accepted register entry fits the
  implementation surface.
- Do not broaden `game-stdlib::board_space` for the Starbridge board.
- Do not introduce a graph, topology, path, jump, home/target, finish, progress,
  bot, or UI helper in `engine-core` or `game-stdlib`.
- Keep all production behavior out of static data and TypeScript.
- Do not introduce trace/hash/fixture/export/RNG migration without explicit
  authority and evidence under ADR 0009.
- Human IP/public-release review remains pending before public release.

## Blocking Issues

| Issue | Required fix | Owner | Blocks coding? |
|---|---|---|---:|
| none for implementation admission after this ticket is archived and committed | not applicable | not applicable | no |
| human IP/public-release review pending | complete review before public release | Rulepath maintainers | no for coding; yes before release |
| forward-v1 machine receipt absent | add `starbridge_crossing` row to `ci/scaffolding-audits.json` and pass checker | GAT20STACROSTA-019 | no for coding; yes before gate closeout |

## Sign-off

Prepared by: `Codex`

Reviewed by: pending maintainer review

Date: 2026-06-27
