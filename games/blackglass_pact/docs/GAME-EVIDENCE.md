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

Gate 18 closeout completed on 2026-06-25. Rows marked `pass` have command,
document, or review evidence in this receipt and the linked artifacts. Human
IP/public-release review remains pending before external public release.

## Completion Profile

| Field | Value |
|---|---|
| Completion profile | `n-seat-hidden-information-release-candidate` |
| Profile rationale | Fixed-four hidden-hand partnership trick-taking game with public observer, four seat-private viewers, viewer-scoped replay exports, bots, web renderer, and release checklist obligations. |
| Not applicable summary | Team-private viewer is not applicable because the locked rules have no fact shared only with a partner. L2/L3 bots are not admitted. |
| Deferred checker surface | Future `GAME-EVIDENCE` checker not present; ticket-level checkers and repo checkers cover concrete surfaces. |
| Foundation invariants status | final pass at GAT18BLAPACSPA-019; Rust owns legality, TypeScript presents Rust/WASM output, no hidden-state leak accepted. |
| Stop-condition review | no active stop condition; human IP/public-release review remains a release blocker, not a Gate 18 implementation blocker. |

## Supported Seats And Variants

| Surface | Status | Artifact link | Notes |
|---|---|---|---|
| Supported seat counts | pass | [RULES.md](RULES.md#identity-seats-teams-and-setup) | Exactly four seats; setup rejection and seat metadata are covered by game and WASM tests. |
| Implemented variants | pass | [SOURCES.md](SOURCES.md#variant-choice-and-deviations) | Only `blackglass_pact_standard`. |
| Seat roles/labels | pass | [RULES.md](RULES.md#identity-seats-teams-and-setup), [MECHANICS.md](MECHANICS.md) | North/East/South/West and fixed teams. |
| N-seat obligations | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md) | Fixed-four proof, observer plus four seat-private viewers. |

## Source And IP Receipt

| Check | Status | Artifact link | Notes |
|---|---|---|---|
| Source notes complete | pass | [SOURCES.md](SOURCES.md) | Consulted sources and variant decisions recorded. |
| Original rules prose complete | pass | [RULES.md](RULES.md) | Every Gate 18 `BP-*` ID represented in original prose. |
| Public name/trade-dress review | pending | [SOURCES.md](SOURCES.md#public-naming-rationale) | Human IP/public-release review remains pending. |
| Assets/fonts/license review | pending human review | [PUBLIC-RELEASE-CHECKLIST.md](PUBLIC-RELEASE-CHECKLIST.md) | No external assets or fonts introduced; final public-release review remains human-owned. |
| Private-source exclusion | pass | [SOURCES.md](SOURCES.md#publicprivate-content-boundary) | No private licensed content involved. |

## Rule-Coverage Summary

| Evidence surface | Status | Artifact link | Notes |
|---|---|---|---|
| Rule coverage matrix | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md) | `cargo run -p rule-coverage -- --game blackglass_pact`. |
| Unit and named rule tests | pass | `games/blackglass_pact/tests/` | Rules, property, replay, serialization, visibility, and bot suites. |
| Property/invariant tests | pass | `games/blackglass_pact/tests/property.rs` | Card conservation, deal determinism, scoring, terminal, helper conformance. |
| Simulation/fuzz runs | pass | `cargo run -p simulate -- --game blackglass_pact --seat-count 4 --games 1000 --start-seed 180400 --action-cap 4096` | 1000 games, 100% completion, 0 action-cap failures. |
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
| public observer | observer corpus and browser smoke | not applicable: observer has no private hand | public tree/effect/diagnostic checks | public export | pass |
| seat `seat_0` | public facts | North own-hand view | actor/viewer scoped checks | seat-private export | pass |
| seat `seat_1` | public facts | East own-hand view | actor/viewer scoped checks | seat-private export | pass |
| seat `seat_2` | public facts | South own-hand view | actor/viewer scoped checks | seat-private export | pass |
| seat `seat_3` | public facts | West own-hand view | actor/viewer scoped checks | seat-private export | pass |
| team-private viewer | not applicable: no team-private fact exists | not applicable | not applicable | not applicable | not applicable |

## Hidden-Information No-Leak Matrix

| Surface | Status | Evidence/test | Notes |
|---|---|---|---|
| public/browser payload | pass | WASM/e2e no-leak tests | No unplayed hand, future deal, private action tree, or private bot candidate. |
| public view | pass | observer corpus | Public bids/plays/scores only. |
| action tree | pass | pairwise tree tests | Only active authorized seat receives private leaves. |
| previews | pass | preview tests | Rust-owned and viewer-safe. |
| diagnostics/disabled reasons | pass | rejection tests | Stable diagnostics reveal no unauthorized alternatives. |
| effect logs | pass | effect filtering tests | Public/private scopes only. |
| command logs | pass | replay tests | Command stream carries accepted public/private-safe command facts. |
| DOM attributes | pass | e2e no-leak scan | Browser closeout. |
| test IDs | pass | e2e no-leak scan | No hidden values in test IDs. |
| browser console/logs | pass | e2e no-leak scan | No hidden values logged. |
| local storage/session storage | pass | e2e no-leak scan | Viewer handoff/export safety. |
| replay export/import | pass | export/import tests | Public and seat-private exports remain scoped. |
| bot explanations | pass | bot no-leak tests | L1 explanations viewer-safe. |
| candidate rankings | not applicable | bot no-leak tests | No public ranked private candidate payload is emitted. |
| dev inspector/public build boundary | pass | web smoke/boundary review | Public build safe. |

## Replay And Hash Compatibility

| Surface | Version/status | Artifact link | Notes |
|---|---|---|---|
| Replay import/export compatibility | pass | replay/export tests | No Trace Schema migration authorized. |
| Hash surface version | current workspace hash surfaces | replay support tests | No broad hash migration admitted. |
| Canonical byte authority | Rust replay/fixture validators or none per profile | fixture/replay artifacts | ADR 0009 governs any migration. |
| Migration/update note | none admitted | not applicable | No blanket golden regeneration. |

## Mechanic And Scaffolding Decisions

| Decision surface | Status | Artifact link | Notes |
|---|---|---|---|
| Mechanic inventory | complete | [MECHANICS.md](MECHANICS.md) | Gate 18 reuse-first audit and closeout dispositions recorded. |
| Primitive-pressure ledger | complete | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) | Trick reuse, numeric keep-local, partnership local-only. |
| Pre-implementation mechanical-scaffolding reuse-first audit | complete | [MECHANICS.md](MECHANICS.md#mechanical-scaffolding-reuse-first-audit) | C-01 through C-10 reviewed. |
| Existing registered/promoted scaffolding adoption | pass | [MECHANICS.md](MECHANICS.md#mechanical-scaffolding-reuse-first-audit) | Reused where lawful; game-local behavior stayed local. |
| Post-implementation new-scaffolding/register-freshness receipt | pass | `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | GAT18BLAPACSPA-018 closed the receipt. |
| Prior-game duplication/refactor disposition | pass | [MECHANICS.md](MECHANICS.md#mechanical-scaffolding-reuse-first-audit) | No prior-game follow-on unit required. |
| CI scaffolding-audit record | pass | `ci/scaffolding-audits.json` | `coverage: "forward-v1"` receipt present. |
| Open behavioral promotion/scaffolding debt | none | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) | §10A remains empty at closeout. |

## Release State And Blockers

| Surface | Status | Artifact link | Notes |
|---|---|---|---|
| Implementation admission | pass | [GAME-IMPLEMENTATION-ADMISSION.md](GAME-IMPLEMENTATION-ADMISSION.md) | Coding may begin under constraints. |
| UI evidence | pass | [UI.md](UI.md) and e2e smoke | Dedicated renderer, live/replay route coverage, and e2e smoke complete. |
| Public release checklist | prepared; pending human release review | [PUBLIC-RELEASE-CHECKLIST.md](PUBLIC-RELEASE-CHECKLIST.md) | Checklist is complete for implementation closeout; public release remains human-owned. |
| Known blockers | none for Gate 18 implementation closeout | active ticket series | Machine forward-v1 receipt and full gates are closed. |
| Human/legal review | pending | [SOURCES.md](SOURCES.md) | Required before public release. |

## Artifact Links

| Artifact | Required? | Link | Status |
|---|---:|---|---|
| `SOURCES.md` | yes | [SOURCES.md](SOURCES.md) | complete |
| `RULES.md` | yes | [RULES.md](RULES.md) | complete |
| `RULE-COVERAGE.md` | yes | [RULE-COVERAGE.md](RULE-COVERAGE.md) | complete |
| `MECHANICS.md` | yes | [MECHANICS.md](MECHANICS.md) | complete |
| `GAME-IMPLEMENTATION-ADMISSION.md` | yes | [GAME-IMPLEMENTATION-ADMISSION.md](GAME-IMPLEMENTATION-ADMISSION.md) | complete |
| `HOW-TO-PLAY.md` | yes | [HOW-TO-PLAY.md](HOW-TO-PLAY.md) | complete |
| `COMPETENT-PLAYER.md` | yes for L1 posture | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | complete |
| `BOT-STRATEGY-EVIDENCE-PACK.md` | profile-dependent | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) | complete; L2 not admitted |
| `AI.md` | yes | [AI.md](AI.md) | complete |
| `UI.md` | web-exposed game | [UI.md](UI.md) | complete |
| `BENCHMARKS.md` | yes | [BENCHMARKS.md](BENCHMARKS.md) | complete |
| `PRIMITIVE-PRESSURE-LEDGER.md` | yes | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) | complete |
| `PUBLIC-RELEASE-CHECKLIST.md` | before public release | [PUBLIC-RELEASE-CHECKLIST.md](PUBLIC-RELEASE-CHECKLIST.md) | prepared; human review pending |

## Final Gate 18 Verification

Completed: 2026-06-25

Representative acceptance commands passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace`
- `cargo test --workspace`
- `cargo test -p blackglass_pact`
- `cargo test -p wasm-api`
- `cargo run -p replay-check -- --game blackglass_pact --all`
- `cargo run -p simulate -- --game blackglass_pact --seat-count 4 --games 1000 --start-seed 180400 --action-cap 4096`
- `cargo bench -p blackglass_pact`
- `cargo run -p fixture-check -- --game blackglass_pact`
- `cargo run -p rule-coverage -- --game blackglass_pact`
- `node scripts/check-catalog-docs.mjs`
- `node scripts/check-scaffolding-governance.mjs`
- `node --test scripts/check-scaffolding-governance.test.mjs`
- `node scripts/check-doc-links.mjs`
- `node scripts/check-outcome-explanations.mjs`
- `node scripts/check-ci-games.mjs`
- `node scripts/check-player-rules.mjs`
- `node scripts/check-presentation-copy.mjs`
- `bash scripts/boundary-check.sh`
- `npm --prefix apps/web run smoke:wasm`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:effects`
- `npm --prefix apps/web run smoke:e2e`
- `npm --prefix apps/web run smoke:preview`
- `npm --prefix apps/web run smoke:animation`

The capstone run also repaired two acceptance-suite maintenance issues exposed
by the full proof: a clippy-only large-enum/format cleanup in Blackglass Pact
and the catalog-sweep animation smoke's stale generic-only count. Neither
changed rule behavior.

## Receipt Review Checklist

- Evidence receipt contains status, rationale, and artifact links only.
- No hidden state, copied source prose, or rule behavior data appears here.
- Every not-applicable entry has a reason.
- Pre-implementation audit and post-implementation register receipt remain distinct.
- CI audit record is present and green for `forward-v1`.
