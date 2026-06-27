# Meldfall Ledger Game Evidence Receipt

Game ID: `meldfall_ledger`

Rules version: `meldfall-ledger-rules-v1`

Data/manifest version: `meldfall-ledger-data-v1`

Trace/profile version set: Trace Schema v1; `replay-command-v1`;
`public-export-v1`; `seat-private-export-v1`; `setup-evidence-v1`;
`domain-evidence-v1`

Engine version: current Rulepath workspace

Prepared by: `Codex`

Last updated: 2026-06-27

## Purpose

This receipt is the artifact-link and status index for Meldfall Ledger
official-game conformance. It does not duplicate rules prose, strategy prose,
UI prose, behavior tables, rule data, or hidden information.

Gate 19 closeout completed on 2026-06-26. Rows marked `pass` have command,
document, or review evidence in this receipt and the linked artifacts. Human
IP/public-release review remains pending before external public release.

## Completion Profile

| Field | Value |
|---|---|
| Completion profile | `n-seat-hidden-information-public-meld-release-candidate` |
| Profile rationale | Variable 2-6 seat hidden-hand game with public discard pile, public meld tableau, large action surface, viewer-scoped replay exports, bots, web renderer, and release checklist obligations. |
| Not applicable summary | Teams, partnerships, trick-taking, L2/L3 bots, two-deck larger-table variant, jokers/wilds, opening minimums, Call Rummy, frozen piles, floating, and tabled-meld rearrangement are excluded. |
| Deferred checker surface | Future `GAME-EVIDENCE` checker not present; ticket-level checkers and repo checkers cover concrete surfaces. |
| Foundation invariants status | Rust owns legality, TypeScript presents Rust/WASM output, no hidden-state leak accepted, no helper promotion debt. |
| Stop-condition review | no active stop condition; human IP/public-release review remains a release blocker, not a Gate 19 implementation blocker. |

## Supported Seats And Variants

| Surface | Status | Artifact link | Notes |
|---|---|---|---|
| Supported seat counts | pass | [RULES.md](RULES.md#setup) | 2 through 6 seats; default 4; setup rejection covered by tests/traces. |
| Implemented variant | pass | [SOURCES.md](SOURCES.md#variant-choice-and-deviations) | Only `classic_500_single_deck_v1`. |
| Seat roles/labels | pass | [MECHANICS.md](MECHANICS.md) | No teams or roles; dealer/active seat are public turn-order state only. |
| N-seat obligations | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md) | Public observer plus all six seat-private viewers. |

## Source And IP Receipt

| Check | Status | Artifact link | Notes |
|---|---|---|---|
| Source notes complete | pass | [SOURCES.md](SOURCES.md) | Consulted sources, variant decisions, and non-copying posture recorded. |
| Original rules prose complete | pass | [RULES.md](RULES.md) | Every Gate 19 `ML-*` rule family represented in original prose. |
| Public name/trade-dress review | pending human review | [SOURCES.md](SOURCES.md#public-naming-rationale) | Human IP/public-release review remains pending. |
| Assets/fonts/license review | pending human review | [PUBLIC-RELEASE-CHECKLIST.md](PUBLIC-RELEASE-CHECKLIST.md) | Original web presentation; final public-release review remains human-owned. |
| Private-source exclusion | pass | [SOURCES.md](SOURCES.md#publicprivate-content-boundary) | No private licensed content involved. |

## Rule-Coverage Summary

| Evidence surface | Status | Artifact link | Notes |
|---|---|---|---|
| Rule coverage matrix | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md) | `cargo run -p rule-coverage -- --game meldfall_ledger`. |
| Unit and named rule tests | pass | `games/meldfall_ledger/tests/` | Rules, property, replay, serialization, visibility, and bot suites. |
| Simulation runs | pass | `simulate` 2/4/6 lanes | Bounded L0 playout smoke across min/default/max seat counts. |
| Serialization coverage | pass | `games/meldfall_ledger/tests/serialization.rs`; `games/meldfall_ledger/tests/replay.rs` | Stable IDs and viewer-scoped export/import. |

## Named Trace Profiles

| Profile ID | Profile version | Visibility class | Validator owner | Artifact link | Status | Notes |
|---|---|---|---|---|---|---|
| `replay-command-v1` | `v1` | internal-dev/public | replay-check | `games/meldfall_ledger/tests/golden_traces/`; `cargo run -p replay-check -- --game meldfall_ledger --all` | pass | Setup, draw, pickup, meld, lay-off, scoring, terminal, export, and no-leak traces. |
| `public-export-v1` | `v1` | public | Rust/WASM export/import tests | `games/meldfall_ledger/tests/replay.rs`; web replay smoke | pass | Observer export is viewer scoped. |
| `seat-private-export-v1` | `v1` | seat-private | Rust/WASM export/import tests | `seat-private-export-round-trip-all-viewers.trace.json`; `viewer-export-no-privilege-elevation.trace.json` | pass | All six seat viewers covered. |
| `setup-evidence-v1` | `v1` | internal-dev/public | fixture-check | `games/meldfall_ledger/data/fixtures/`; `cargo run -p fixture-check -- --game meldfall_ledger` | pass | 2p, 4p, 6p, pickup, lay-off, and tie-continuation fixtures. |
| `domain-evidence-v1` | `v1` | internal-dev/public | game-local validator | `games/meldfall_ledger/tests/{rules,property,serialization,replay,visibility,bots}.rs` | pass | Meld, lay-off, pickup, scoring, terminal, visibility, and bot evidence. |

## Fixture Profile

| Fixture | Status | Purpose |
|---|---|---|
| `meldfall_ledger_2p_standard.fixture.json` | pass | 2-player 13-card deal and standard round segment. |
| `meldfall_ledger_4p_standard.fixture.json` | pass | Default setup and ordinary turn flow. |
| `meldfall_ledger_6p_standard.fixture.json` | pass | Max-seat setup and no-leak/export surface. |
| `meldfall_ledger_multi_discard_pickup.fixture.json` | pass | Deep discard pickup and immediate-use commitment. |
| `meldfall_ledger_layoff_any_tableau.fixture.json` | pass | Lay-off onto public tableau with score credit. |
| `meldfall_ledger_500_tie_continues.fixture.json` | pass | Target threshold and tie-continuation behavior. |

## Gate 19.1 Multi-Round Completion Receipt

Completed: 2026-06-26

| Surface | Status | Evidence | Notes |
|---|---|---|---|
| Rust-owned transition | pass | `games/meldfall_ledger/tests/rules.rs`; `round-transition-resets-table-state.trace.json` | Nonterminal settled rounds rotate dealer, clear round-only state, re-deal deterministically, preserve cumulative scores, and continue from the seat left of the new dealer. |
| Host parity | pass | `crates/wasm-api/src/tests.rs`; `tools/simulate/src/main.rs`; `cargo run -p simulate -- --game meldfall_ledger --games 1000 --action-cap 20000` | WASM and native simulator both route through `advance_to_next_round`; the recorded simulator run completed 1000/1000 games with `bounded_nonterminal_at_cap=0`. |
| Replay and no-leak | pass | `cargo run -p replay-check -- --game meldfall_ledger --all`; `games/meldfall_ledger/tests/visibility.rs` | Re-dealt stock and new private hands remain hidden across view, action-tree, effect, and viewer-export surfaces. |
| Web completion | pass | `apps/web/src/components/effectFeedback.ts`; `apps/web/scripts/smoke-effect-feedback.mjs`; `output/playwright/gat191melled-005-meldfall-terminal.png` | Browser Bot vs bot evidence reaches the terminal outcome panel with no `round_settled` dead-end. |
| Coverage closeout | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md); `cargo run -p rule-coverage -- --game meldfall_ledger`; `cargo run -p fixture-check -- --game meldfall_ledger` | `ML-MATCH-003` and `ML-MATCH-006` are covered by traces/tests; existing scoring-illustration fixtures remain intact. |

## Gate 19.2 Settlement Detail Projection Receipt

Completed: 2026-06-27

| Surface | Status | Evidence | Notes |
|---|---|---|---|
| Rust-owned retained projection | pass | `games/meldfall_ledger/src/state.rs`; `games/meldfall_ledger/src/visibility.rs`; `cargo test -p meldfall_ledger` | `last_settlement` snapshots the already-public `ML-VIS-006` settlement totals/counts and persists across the next round until replaced. |
| Replay/hash boundary | pass | `games/meldfall_ledger/src/state.rs`; `cargo run -p replay-check -- --game meldfall_ledger --all` | The retained snapshot is excluded from `MatchState::stable_internal_summary()`, so no trace schema or replay/hash migration was required. |
| WASM/web projection | pass | `crates/wasm-api/src/games/meldfall.rs`; `apps/web/src/wasm/client.ts`; `npm --prefix apps/web run smoke:wasm`; `npm --prefix apps/web run build` | The bridge serializes `last_settlement` as nullable public JSON with stable per-seat rows. |
| Browser settlement panel | pass | `apps/web/src/components/MeldfallLedgerBoard.tsx`; `node apps/web/e2e/meldfall-ledger.smoke.mjs` | The panel renders round-end reason, tabled-positive totals, in-hand penalties, held counts, deltas, cumulative scores, ranks, and winner flags from `view.last_settlement`. |
| Browser no-leak | pass | `node apps/web/e2e/a11y-noleak.smoke.mjs`; `npm --prefix apps/web run smoke:ui`; `npm --prefix apps/web run smoke:effects` | DOM, accessible names, storage, logs, and effect feedback retain the hidden-card boundary while showing authorized totals/counts. |
| Coverage closeout | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md); `cargo run -p rule-coverage -- --game meldfall_ledger`; `cargo run -p fixture-check -- --game meldfall_ledger`; `cargo run -p simulate -- --game meldfall_ledger --games 1000 --action-cap 20000` | `ML-VIS-006` and `ML-SCORE-002` through `ML-SCORE-007` now cite the persistent settlement projection evidence. |

## Viewer Matrix

| Viewer class | Public view evidence | Seat-private view evidence | Action/effect/diagnostic evidence | Replay/export evidence | Status |
|---|---|---|---|---|---|
| public observer | observer corpus and browser smoke | not applicable | public tree/effect/diagnostic checks | public export | pass |
| seat `seat_0` | public facts | own-hand view only | actor/viewer scoped checks | seat-private export | pass |
| seat `seat_1` | public facts | own-hand view only | actor/viewer scoped checks | seat-private export | pass |
| seat `seat_2` | public facts | own-hand view only | actor/viewer scoped checks | seat-private export | pass |
| seat `seat_3` | public facts | own-hand view only | actor/viewer scoped checks | seat-private export | pass |
| seat `seat_4` | public facts | own-hand view only | actor/viewer scoped checks | seat-private export | pass |
| seat `seat_5` | public facts | own-hand view only | actor/viewer scoped checks | seat-private export | pass |

## Hidden-Information No-Leak Matrix

| Surface | Status | Evidence/test | Notes |
|---|---|---|---|
| public/browser payload | pass | WASM/e2e no-leak tests | Observer has no private hand or stock order. |
| public view | pass | visibility tests | Public discard/tableau/scores/counts only. |
| action tree | pass | pairwise tree tests and web smoke | Only active authorized seat receives private leaves. |
| previews/diagnostics | pass | rule/visibility tests | Stable diagnostics reveal no unauthorized alternatives. |
| effect logs | pass | effect filtering tests | Stock draws and private hand facts remain scoped. |
| DOM/test IDs | pass | `meldfall-ledger.smoke.mjs` scan | No hidden cards or stock order in DOM/a11y/test IDs. |
| console/storage | pass | `meldfall-ledger.smoke.mjs` | No hidden values logged or persisted. |
| replay export/import | pass | replay tests and e2e | Public exports cannot elevate to seat-private exports. |
| bot explanations | pass | bot tests | L0 explanations name only legal-choice count. |
| candidate rankings | not applicable | no public candidate ranking | L1/L2 not admitted. |

## Mechanic And Scaffolding Decisions

| Decision surface | Status | Artifact link | Notes |
|---|---|---|---|
| Mechanic inventory | complete | [MECHANICS.md](MECHANICS.md) | Gate 19 local mechanics and audit closeout recorded. |
| Primitive-pressure ledger | complete | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) | `ML-PP-001` through `ML-PP-006`. |
| Mechanical-scaffolding reuse-first audit | complete | [GAME-IMPLEMENTATION-ADMISSION.md](GAME-IMPLEMENTATION-ADMISSION.md) | C-01 through C-10 reviewed. |
| Post-implementation register receipt | pass | [../../../docs/MECHANICAL-SCAFFOLDING-REGISTER.md](../../../docs/MECHANICAL-SCAFFOLDING-REGISTER.md) | Gate 19 no-new-scaffolding receipt recorded. |
| CI scaffolding-audit record | pass | [../../../ci/scaffolding-audits.json](../../../ci/scaffolding-audits.json) | `coverage: "forward-v1"` receipt present. |
| Open behavioral promotion/scaffolding debt | none | [../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md) | §10A remains empty at closeout. |

## Release State And Blockers

| Surface | Status | Artifact link | Notes |
|---|---|---|---|
| Implementation admission | pass | [GAME-IMPLEMENTATION-ADMISSION.md](GAME-IMPLEMENTATION-ADMISSION.md) | Coding proceeded under constraints. |
| UI evidence | pass | [UI.md](UI.md) and e2e smoke | Dedicated renderer, replay export/import, responsive and no-leak smoke complete. |
| Public release checklist | prepared; pending human release review | [PUBLIC-RELEASE-CHECKLIST.md](PUBLIC-RELEASE-CHECKLIST.md) | Checklist is complete for implementation closeout; public release remains human-owned. |
| Known blockers | none for Gate 19 implementation closeout | active ticket series | Machine receipt and capstone gates are closed. |
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
| `COMPETENT-PLAYER.md` | yes | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) | complete |
| `BOT-STRATEGY-EVIDENCE-PACK.md` | profile-dependent | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) | complete; L2 not admitted |
| `AI.md` | yes | [AI.md](AI.md) | complete |
| `UI.md` | web-exposed game | [UI.md](UI.md) | complete |
| `BENCHMARKS.md` | yes | [BENCHMARKS.md](BENCHMARKS.md) | complete |
| `PRIMITIVE-PRESSURE-LEDGER.md` | yes | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) | complete |
| `PUBLIC-RELEASE-CHECKLIST.md` | before public release | [PUBLIC-RELEASE-CHECKLIST.md](PUBLIC-RELEASE-CHECKLIST.md) | prepared; human review pending |

## Final Gate 19 Verification

Completed: 2026-06-26

Representative acceptance commands passed in the Gate 19 ticket series:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace`
- `cargo test --workspace`
- `cargo test -p meldfall_ledger`
- `cargo test -p wasm-api`
- `cargo run -p replay-check -- --game meldfall_ledger --all`
- `cargo run -p simulate -- --game meldfall_ledger --seat-count 2 --games 1000 --start-seed 1900 --action-cap 4096`
- `cargo run -p simulate -- --game meldfall_ledger --seat-count 4 --games 1000 --start-seed 1901 --action-cap 4096`
- `cargo run -p simulate -- --game meldfall_ledger --seat-count 6 --games 1000 --start-seed 1902 --action-cap 8192`
- `cargo bench -p meldfall_ledger`
- `cargo run -p fixture-check -- --game meldfall_ledger`
- `cargo run -p rule-coverage -- --game meldfall_ledger`
- `bash scripts/boundary-check.sh`
- `node scripts/check-doc-links.mjs`
- `node scripts/check-ci-games.mjs`
- `node scripts/check-catalog-docs.mjs`
- `node scripts/check-player-rules.mjs`
- `node scripts/check-scaffolding-governance.mjs`
- `npm --prefix apps/web test`
- `npm --prefix apps/web run smoke:e2e`

The simulator lanes are bounded L0 legality smokes, not competence claims.

## Gate 19.2 Verification

Completed: 2026-06-27

The settlement-detail projection follow-on re-ran these acceptance commands:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo run -p simulate -- --game meldfall_ledger --games 1000 --action-cap 20000`
- `cargo run -p replay-check -- --game meldfall_ledger --all`
- `cargo run -p fixture-check -- --game meldfall_ledger`
- `cargo run -p rule-coverage -- --game meldfall_ledger`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:effects`
- `node scripts/check-catalog-docs.mjs`
- `node apps/web/e2e/meldfall-ledger.smoke.mjs`
- `node apps/web/e2e/a11y-noleak.smoke.mjs`

The simulator run used an explicit `--action-cap 20000` for multi-round bounded
proof. The cap is a verifier guard only; it is not a game rule or competence
claim.

## Receipt Review Checklist

- Evidence receipt contains status, rationale, and artifact links only.
- No hidden state, copied source prose, or rule behavior data appears here.
- Every not-applicable entry has a reason.
- Pre-implementation audit and post-implementation register receipt remain distinct.
- CI audit record is present and green for `forward-v1`.
