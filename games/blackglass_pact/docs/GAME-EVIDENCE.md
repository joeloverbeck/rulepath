# Blackglass Pact Game Evidence Receipt

Game ID: `blackglass_pact`

Rules version: `blackglass-pact-rules-v1`

Data/manifest version: `blackglass-pact-data-v1`

Trace/profile version set: Trace Schema v1; `replay-command-v1`;
`public-export-v1`; `seat-private-export-v1`; `setup-evidence-v1`;
`domain-evidence-v1`

Engine version: current Rulepath workspace

Prepared by: `Codex`

Last updated: 2026-06-25

## Purpose

This receipt is the status and artifact-link index for Blackglass Pact
official-game conformance. It does not duplicate rules prose, strategy prose,
UI prose, behavior tables, rule data, or hidden information.

Rows marked `planned` are required before Gate 18 closeout. This ticket only
initializes the receipt and records the completed pre-implementation
admission/audit surfaces.

## Completion Profile

| Field | Value |
|---|---|
| Completion profile | `n-seat-hidden-information-release-candidate` |
| Profile rationale | Fixed-four hidden-hand partnership trick-taking game with public observer, four seat-private viewers, viewer-scoped replay exports, bots, web renderer, and release checklist obligations. |
| Not applicable summary | Team-private viewer is not applicable because the locked rules have no fact shared only with a partner. L2/L3 bots are not admitted. |
| Deferred checker surface | Future `GAME-EVIDENCE` checker not present; ticket-level checkers cover concrete surfaces. |
| Foundation invariants status | pre-code pass; final pass required at GAT18BLAPACSPA-019 |
| Stop-condition review | no stop condition in admitted pre-code scope |

## Supported Seats And Variants

| Surface | Status | Artifact link | Notes |
|---|---|---|---|
| Supported seat counts | planned | [RULES.md](RULES.md#identity-seats-teams-and-setup) | Exactly four seats; setup tests land in GAT18BLAPACSPA-003. |
| Implemented variants | planned | [SOURCES.md](SOURCES.md#variant-choice-and-deviations) | Only `blackglass_pact_standard`. |
| Seat roles/labels | planned | [RULES.md](RULES.md#identity-seats-teams-and-setup), [MECHANICS.md](MECHANICS.md) | North/East/South/West and fixed teams. |
| N-seat obligations | planned | [RULE-COVERAGE.md](RULE-COVERAGE.md) | Fixed-four proof, observer plus four seat-private viewers. |

## Source And IP Receipt

| Check | Status | Artifact link | Notes |
|---|---|---|---|
| Source notes complete | pass | [SOURCES.md](SOURCES.md) | Consulted sources and variant decisions recorded. |
| Original rules prose complete | pass | [RULES.md](RULES.md) | Every Gate 18 `BP-*` ID represented in original prose. |
| Public name/trade-dress review | pending | [SOURCES.md](SOURCES.md#public-naming-rationale) | Human IP/public-release review remains pending. |
| Assets/fonts/license review | planned | `PUBLIC-RELEASE-CHECKLIST.md` | No assets or fonts introduced yet. |
| Private-source exclusion | pass | [SOURCES.md](SOURCES.md#publicprivate-content-boundary) | No private licensed content involved. |

## Rule-Coverage Summary

| Evidence surface | Status | Artifact link | Notes |
|---|---|---|---|
| Rule coverage matrix | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md) | `cargo run -p rule-coverage -- --game blackglass_pact`. |
| Unit and named rule tests | pass | `games/blackglass_pact/tests/` | Rules, property, replay, serialization, visibility, and bot suites. |
| Property/invariant tests | pass | `games/blackglass_pact/tests/property.rs` | Card conservation, deal determinism, scoring, terminal, helper conformance. |
| Simulation/fuzz runs | smoke | `cargo run -p simulate -- --game blackglass_pact --seat-count 4 --games 1000 --start-seed 180400 --action-cap 4096` | Fixed-four bot-smoke summary; full terminal command replay remains future work. |
| Serialization coverage | pass | `games/blackglass_pact/tests/serialization.rs`; `games/blackglass_pact/tests/replay.rs` | Stable IDs and viewer-scoped export/import. |

## Named Trace Profiles

| Profile ID | Profile version | Visibility class | Validator owner | Artifact link | Status | Notes |
|---|---|---|---|---|---|---|
| `replay-command-v1` | `v1` | internal-dev/public | replay-check | `games/blackglass_pact/tests/golden_traces/`; `cargo run -p replay-check -- --game blackglass_pact --all` | pass | Bounded metadata/setup/no-leak inventory validator; full terminal command replay remains future work. |
| `public-export-v1` | `v1` | public | Rust export/import tests | `games/blackglass_pact/tests/replay.rs`; `public-export-v1-round-trip.trace.json` | pass | Observer export is viewer scoped. |
| `seat-private-export-v1` | `v1` | seat-private | Rust export/import tests | `games/blackglass_pact/tests/replay.rs`; `seat-private-export-v1-round-trip.trace.json`; `seat-private-pairwise-no-leak-all-four.trace.json` | pass | All four seat viewers covered. |
| `setup-evidence-v1` | `v1` | internal-dev/public | fixture-check | `games/blackglass_pact/data/fixtures/blackglass_pact_standard.fixture.json`; `cargo run -p fixture-check -- --game blackglass_pact` | pass | Static typed fixture keys only. |
| `domain-evidence-v1` | `v1` | internal-dev/public | game-local validator | `games/blackglass_pact/data/fixtures/*`; `games/blackglass_pact/tests/{rules,property,serialization,replay,visibility,bots}.rs` | pass | Scoring, nil/bags, target tie, visibility, and bot evidence. |

## Viewer Matrix

| Viewer class | Public view evidence | Seat-private view evidence | Action/effect/diagnostic evidence | Replay/export evidence | Status |
|---|---|---|---|---|---|
| public observer | planned observer corpus | not applicable: observer has no private hand | planned public tree/effect/diagnostic checks | planned public export | planned |
| seat `seat_0` | planned public facts | planned North own-hand view | planned actor/viewer scoped checks | planned seat-private export | planned |
| seat `seat_1` | planned public facts | planned East own-hand view | planned actor/viewer scoped checks | planned seat-private export | planned |
| seat `seat_2` | planned public facts | planned South own-hand view | planned actor/viewer scoped checks | planned seat-private export | planned |
| seat `seat_3` | planned public facts | planned West own-hand view | planned actor/viewer scoped checks | planned seat-private export | planned |
| team-private viewer | not applicable: no team-private fact exists | not applicable | not applicable | not applicable | not applicable |

## Hidden-Information No-Leak Matrix

| Surface | Status | Evidence/test | Notes |
|---|---|---|---|
| public/browser payload | planned | WASM/e2e no-leak tests | No unplayed hand, future deal, private action tree, private bot candidate. |
| public view | planned | observer corpus | Public bids/plays/scores only. |
| action tree | planned | pairwise tree tests | Only active authorized seat receives private leaves. |
| previews | planned | preview tests | Rust-owned and viewer-safe. |
| diagnostics/disabled reasons | planned | rejection tests | Stable diagnostics reveal no unauthorized alternatives. |
| effect logs | planned | effect filtering tests | Public/private scopes only. |
| command logs | planned | replay tests | Command stream carries accepted public/private-safe command facts. |
| DOM attributes | planned | e2e no-leak scan | Browser closeout. |
| test IDs | planned | e2e no-leak scan | No hidden values in test IDs. |
| browser console/logs | planned | e2e no-leak scan | No hidden values logged. |
| local storage/session storage | planned | e2e no-leak scan | Viewer handoff/export safety. |
| replay export/import | planned | export/import tests | Public and seat-private exports remain scoped. |
| bot explanations | planned | bot no-leak tests | L1 explanations viewer-safe. |
| candidate rankings | planned | bot no-leak tests | Unauthorized candidates absent. |
| dev inspector/public build boundary | planned | web smoke/boundary review | Public build safe. |

## Replay And Hash Compatibility

| Surface | Version/status | Artifact link | Notes |
|---|---|---|---|
| Replay import/export compatibility | planned | replay/export tests | No Trace Schema migration authorized. |
| Hash surface version | current workspace hash surfaces | replay support tests | No broad hash migration admitted. |
| Canonical byte authority | Rust replay/fixture validators or none per profile | planned artifacts | ADR 0009 governs any migration. |
| Migration/update note | none admitted | not applicable | No blanket golden regeneration. |

## Mechanic And Scaffolding Decisions

| Decision surface | Status | Artifact link | Notes |
|---|---|---|---|
| Mechanic inventory | complete pre-code | [MECHANICS.md](MECHANICS.md) | Final evidence updates land later. |
| Primitive-pressure ledger | complete pre-code | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) | Trick reuse, numeric keep-local, partnership local-only. |
| Pre-implementation mechanical-scaffolding reuse-first audit | complete | [MECHANICS.md](MECHANICS.md#mechanical-scaffolding-reuse-first-audit) | C-01 through C-10 reviewed. |
| Existing registered/promoted scaffolding adoption | planned | [MECHANICS.md](MECHANICS.md#mechanical-scaffolding-reuse-first-audit) | Reuse expected; implementation evidence pending. |
| Post-implementation new-scaffolding/register-freshness receipt | planned | `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | GAT18BLAPACSPA-018. |
| Prior-game duplication/refactor disposition | planned no-unit unless evidence changes | [MECHANICS.md](MECHANICS.md#mechanical-scaffolding-reuse-first-audit) | GAT18BLAPACSPA-018 confirms. |
| CI scaffolding-audit record | planned | `ci/scaffolding-audits.json` | GAT18BLAPACSPA-018 adds `coverage: "forward-v1"`. |
| Open behavioral promotion/scaffolding debt | none pre-code | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) | §10A must remain empty at closeout. |

## Release State And Blockers

| Surface | Status | Artifact link | Notes |
|---|---|---|---|
| Implementation admission | pass | [GAME-IMPLEMENTATION-ADMISSION.md](GAME-IMPLEMENTATION-ADMISSION.md) | Coding may begin under constraints. |
| UI evidence | planned | `UI.md` and e2e smoke | Later tickets. |
| Public release checklist | planned | `PUBLIC-RELEASE-CHECKLIST.md` | Ticket 017. |
| Known blockers | none for coding | active ticket series | Machine forward-v1 receipt and full gates still required for closeout. |
| Human/legal review | pending | [SOURCES.md](SOURCES.md) | Required before public release. |

## Artifact Links

| Artifact | Required? | Link | Status |
|---|---:|---|---|
| `SOURCES.md` | yes | [SOURCES.md](SOURCES.md) | complete |
| `RULES.md` | yes | [RULES.md](RULES.md) | complete |
| `RULE-COVERAGE.md` | yes | [RULE-COVERAGE.md](RULE-COVERAGE.md) | initialized |
| `MECHANICS.md` | yes | [MECHANICS.md](MECHANICS.md) | complete pre-code |
| `GAME-IMPLEMENTATION-ADMISSION.md` | yes | [GAME-IMPLEMENTATION-ADMISSION.md](GAME-IMPLEMENTATION-ADMISSION.md) | complete pre-code |
| `HOW-TO-PLAY.md` | yes | `HOW-TO-PLAY.md` | planned |
| `COMPETENT-PLAYER.md` | yes for L1 posture | `COMPETENT-PLAYER.md` | planned |
| `BOT-STRATEGY-EVIDENCE-PACK.md` | profile-dependent | `BOT-STRATEGY-EVIDENCE-PACK.md` | planned; L2 not admitted |
| `AI.md` | yes | `AI.md` | planned |
| `UI.md` | web-exposed game | `UI.md` | planned |
| `BENCHMARKS.md` | yes | `BENCHMARKS.md` | planned |
| `PRIMITIVE-PRESSURE-LEDGER.md` | yes | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) | complete pre-code |
| `PUBLIC-RELEASE-CHECKLIST.md` | before public release | `PUBLIC-RELEASE-CHECKLIST.md` | planned |

## Receipt Review Checklist

- Pre-code evidence receipt contains status, rationale, and artifact links only.
- No hidden state, copied source prose, or rule behavior data appears here.
- Every not-applicable entry has a reason.
- Pre-implementation audit and post-implementation register receipt remain distinct.
- CI audit record is intentionally pending until GAT18BLAPACSPA-018.
