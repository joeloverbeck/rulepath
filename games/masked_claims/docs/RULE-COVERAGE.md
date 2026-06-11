# Masked Claims Rule Coverage Matrix

Game ID: `masked_claims`

Rules version: `masked-claims-rules-v1`

Data/manifest version: `1`

Engine version: `engine-core-0.1.0`

Prepared by: `Codex`

Last updated: 2026-06-11

## Purpose

This matrix maps every stable `MC-*` rule ID in `RULES.md` to implementation
areas and primary evidence. Rust tests, golden traces, replay checks,
serialization checks, visibility/no-leak checks, simulations, and benchmarks
are the rule evidence; browser smoke remains presentation evidence only until
the web ticket lands.

## Rule Coverage Matrix

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `MC-ACT-001` | Claim leaves for held masks and grades. | `actions.rs` legal tree and validation. | `actions::claim_tree_contains_held_masks_and_declared_grades`; `tests/rules.rs`; `claim-pending-window.trace.json`; `legal_actions_claim_phase` benchmark. | covered | Internal paths include tile IDs; public summaries redact them. |
| `MC-ACT-002` | Responder has no claim-phase actions. | `actions.rs` actor/phase checks. | `actions::non_actor_trees_are_empty`; `tests/rules.rs`; simulation. | covered | Empty tree is Rust-produced. |
| `MC-ACT-003` | Responder reaction tree has accept/challenge. | `actions.rs` response tree. | `actions::reaction_tree_contains_accept_and_challenge`; `claim-pending-window.trace.json`; `legal_actions_reaction_window` benchmark. | covered | Exactly two response leaves. |
| `MC-ACT-004` | Claimant waits during reaction window. | `actions.rs` waiting metadata. | `wrong-phase-claim-diagnostic.trace.json`; `tests/rules.rs`. | covered | No claimant gameplay action. |
| `MC-ACT-005` | Terminal tree is empty. | `actions.rs` terminal branch. | `terminal-tie-break.trace.json`; `draw-after-tie-breaks.trace.json`; simulation. | covered | No terminal gameplay actions. |
| `MC-ACT-006` | Action metadata is viewer-safe. | `actions.rs` metadata builders. | `tests/visibility.rs`; `public-observer-no-leak.trace.json`. | covered | Metadata excludes hidden tile facts. |
| `MC-ACT-007` | Non-actor viewers receive no private action tree. | `actions.rs`; wasm viewer authorization. | `tests/visibility.rs`; wasm-api masked bridge test. | covered | Seat-private action leaves are actor-scoped. |
| `MC-AMB-001` | Rust owns behavior authority. | `actions.rs`, `rules.rs`, `visibility.rs`, `bots.rs`. | `cargo test -p masked_claims`; `scripts/boundary-check.sh`; ticket 014 wasm test. | covered | TypeScript remains presentation-only. |
| `MC-AMB-002` | No generic engine promotion. | Game-local modules only. | `scripts/boundary-check.sh`; crate/module review. | covered | No `engine-core` mechanic nouns added. |
| `MC-AMB-003` | Static data is typed content only. | `variants.rs`; `data/*` parsers. | `tests/serialization.rs`; `fixture-check --game masked_claims`. | covered | Unknown fields rejected. |
| `MC-AMB-004` | Official-game evidence exists. | Tests, traces, tools, docs. | `cargo test -p masked_claims`; `simulate`; `replay-check`; `fixture-check`; `rule-coverage`. | covered | Browser E2E lands later. |
| `MC-AMB-005` | Hidden info stays hidden on public surfaces. | `visibility.rs`; `replay_support.rs`; wasm serializers. | `tests/visibility.rs`; `accepted-mask-never-revealed.trace.json`; wasm-api masked bridge test. | covered | Challenged masks reveal only after challenge. |
| `MC-BOT-001` | Random legal bot uses legal tree. | `bots.rs` random bot. | `src/bots.rs` tests; `tests/bots.rs`; `bot-claim-and-response.trace.json`. | covered | Deterministic under seed. |
| `MC-BOT-002` | Level 1 claim bot uses allowed inputs. | `bots.rs` Level 1 claim policy. | `tests/bots.rs`; `bot-claim-and-response.trace.json`; `level1_bot_claim_decision` benchmark. | covered | No hidden opponent/reserve access. |
| `MC-BOT-003` | Level 1 response bot uses allowed inputs. | `bots.rs` Level 1 response policy. | `tests/bots.rs`; `certain-lie-challenge.trace.json`; `level1_bot_response_decision` benchmark. | covered | No sampling, MCTS, ML, or hidden-state peeking. |
| `MC-COMP-001` | Two public seats. | `ids.rs`; `setup.rs`; `state.rs`. | `tests/serialization.rs`; `tests/rules.rs`; fixture validation. | covered | Variant is two-seat only. |
| `MC-COMP-002` | Mask tile identity/grade. | `ids.rs`; `setup.rs`. | `src/lib.rs` static data test; `tests/serialization.rs`. | covered | Public only after challenge reveal. |
| `MC-COMP-003` | Five ordered grades. | `ids.rs`; `ui.rs`. | `src/actions.rs` grade tests; `tests/serialization.rs`. | covered | Labels are original. |
| `MC-COMP-004` | Private hands. | `state.rs`; `setup.rs`; `visibility.rs`. | `tests/visibility.rs`; `public-observer-no-leak.trace.json`. | covered | Owners see own hand only. |
| `MC-COMP-005` | Internal reserve. | `state.rs`; `setup.rs`; `visibility.rs`. | `tests/property.rs`; `tests/visibility.rs`. | covered | Reserve never projected. |
| `MC-COMP-006` | Claim pedestal. | `state.rs`; `rules.rs`; `visibility.rs`. | `claim-pending-window.trace.json`; `tests/visibility.rs`. | covered | Declared grade public; tile hidden. |
| `MC-COMP-007` | Reaction window. | `state.rs`; `actions.rs`; `rules.rs`. | `claim-pending-window.trace.json`; `tests/rules.rs`. | covered | Single-depth responder window. |
| `MC-COMP-008` | Veiled gallery. | `state.rs`; `rules.rs`; `visibility.rs`. | `accept-resolution.trace.json`; `accepted-mask-never-revealed.trace.json`. | covered | IDs never reveal. |
| `MC-COMP-009` | Exposed row. | `state.rs`; `rules.rs`; `visibility.rs`. | `challenge-honest-reveal.trace.json`; `challenge-exposed-lie.trace.json`. | covered | Only challenged masks enter. |
| `MC-COMP-010` | Scores and counters. | `state.rs`; `rules.rs`; `visibility.rs`. | `tests/rules.rs`; terminal traces. | covered | Public terminal rationale uses these. |
| `MC-DEV-001` | Developer/debug surfaces must be safe. | `visibility.rs`; wasm serializers. | `tests/visibility.rs`; wasm-api masked bridge test. | covered | Browser dev panel E2E lands in ticket 017/019. |
| `MC-DEV-002` | Diagnostics are safe. | `actions.rs` diagnostics. | diagnostic golden traces; `tests/visibility.rs`. | covered | Messages do not expose hidden alternatives. |
| `MC-DEV-003` | Replay/export surfaces are safe. | `replay_support.rs`; wasm export. | `public-replay-export-import.trace.json`; `tests/replay.rs`; wasm-api masked bridge test. | covered | Public export redacts claim paths. |
| `MC-END-001` | Higher final score wins. | `rules.rs` terminal evaluation. | `tests/rules.rs`; `terminal-tie-break.trace.json`; simulation. | covered | ScoreWin branch covered. |
| `MC-END-002` | Fewer exposed lies tiebreak. | `rules.rs` tiebreak ladder. | `tests/rules.rs`; `terminal-tie-break.trace.json`. | covered | First tiebreak. |
| `MC-END-003` | More successful challenges tiebreak. | `rules.rs` tiebreak ladder. | `tests/rules.rs`; terminal tests. | covered | Second tiebreak. |
| `MC-END-004` | Fewer challenges declared tiebreak. | `rules.rs` tiebreak ladder. | `tests/rules.rs`; terminal tests. | covered | Third tiebreak. |
| `MC-END-005` | Draw after all tiebreakers tie. | `rules.rs` terminal evaluation. | `draw-after-tie-breaks.trace.json`; `tests/rules.rs`. | covered | No priority-seat fallback. |
| `MC-END-006` | Terminal does not reveal hidden residue. | `visibility.rs`; `rules.rs`. | `accepted-mask-never-revealed.trace.json`; `tests/visibility.rs`. | covered | Veiled, hands, and reserve remain redacted. |
| `MC-OOS-001` | No multi-seat variant. | `variants.rs`. | `fixture-check --game masked_claims`; `tests/serialization.rs`. | unsupported | Only the standard two-seat variant ships. |
| `MC-OOS-002` | No nested or cancellation windows. | `rules.rs`; `actions.rs`. | `tests/rules.rs`; `wrong-phase-claim-diagnostic.trace.json`. | unsupported | Gate 11 scopes one clean window. |
| `MC-OOS-003` | No hosted multiplayer/network timers. | Local Rust crate and web bridge only. | Architecture review; no server code paths. | unsupported | Local-first scope. |
| `MC-OOS-004` | No generic reaction engine. | Game-local modules only. | `scripts/boundary-check.sh`; docs. | unsupported | ADR required before promotion. |
| `MC-OOS-005` | No hidden-role/proprietary roster. | `ids.rs`; docs. | `SOURCES.md`; `RULES.md`; static data tests. | unsupported | Original masks/grades only. |
| `MC-OOS-006` | No sampling/MCTS/ML/RL bots. | `bots.rs`. | `tests/bots.rs`; `BOT-STRATEGY-EVIDENCE-PACK.md`. | unsupported | Level 1 heuristic only. |
| `MC-OOS-007` | No TypeScript legality. | Rust/WASM bridge. | wasm-api masked bridge test; `scripts/boundary-check.sh`. | unsupported | Browser only presents Rust output. |
| `MC-RESTRICT-001` | Unknown actor rejected. | `actions.rs` validation. | `src/actions.rs` diagnostics tests. | covered | No mutation. |
| `MC-RESTRICT-002` | Wrong active seat rejected. | `actions.rs` validation. | `wrong-seat-response-diagnostic.trace.json`; `src/actions.rs` tests. | covered | Safe public reason. |
| `MC-RESTRICT-003` | Malformed/unavailable path rejected. | `actions.rs` parser/validation. | `src/actions.rs` malformed tests. | covered | No hidden facts. |
| `MC-RESTRICT-004` | Stale command rejected. | `actions.rs` freshness checks. | `stale-diagnostic.trace.json`; `src/actions.rs` tests. | covered | State hash unchanged. |
| `MC-RESTRICT-005` | Unowned mask claim rejected. | `actions.rs` claim validation. | `unowned-tile-diagnostic.trace.json`; `src/actions.rs` tests. | covered | Echoes only submitted id. |
| `MC-RESTRICT-006` | Invalid grade rejected. | `actions.rs` grade parser. | `src/actions.rs` invalid-grade test. | covered | Grade must be 1 through 5. |
| `MC-RESTRICT-007` | Response outside window rejected. | `actions.rs` phase validation. | `wrong-phase-claim-diagnostic.trace.json`; `src/actions.rs` tests. | covered | No mutation. |
| `MC-RESTRICT-008` | Terminal action rejected. | `actions.rs` terminal validation. | terminal rule tests; simulation terminal cap. | covered | Outcome is final. |
| `MC-RNG-001` | Seeded setup replay determinism. | `setup.rs`; `state.rs`; `replay_support.rs`. | `tests/replay.rs`; `tests/property.rs`; `replay-check --game masked_claims --all`. | covered | Same seed/options reproduce surfaces. |
| `MC-RNG-002` | Public export is redacted. | `replay_support.rs`; wasm export. | `public-replay-export-import.trace.json`; `tests/replay.rs`; wasm-api masked bridge test. | covered | No seed/private paths in public export. |
| `MC-RNG-003` | Serialization order is stable. | `StableSerialize` impls and summaries. | `tests/serialization.rs`; `tests/replay.rs`. | covered | Golden traces pin surface presence. |
| `MC-SCORE-001` | Accepted claim scores declared grade. | `rules.rs`. | `accept-resolution.trace.json`; `tests/rules.rs`; `apply_accept_resolution` benchmark. | covered | Mask becomes veiled. |
| `MC-SCORE-002` | Honest/underclaim challenge scores actual plus bonus. | `rules.rs`. | `challenge-honest-reveal.trace.json`; `underclaim-trap-reveal.trace.json`; `tests/rules.rs`. | covered | Challenged mask reveals. |
| `MC-SCORE-003` | Exposed lie awards responder gap. | `rules.rs`. | `challenge-exposed-lie.trace.json`; `tests/rules.rs`; `apply_challenge_resolve_reveal` benchmark. | covered | Claimant gets zero for exposed lie. |
| `MC-SCORE-004` | Challenge counter increments. | `rules.rs`. | `tests/rules.rs`; challenge traces. | covered | Public tiebreak counter. |
| `MC-SCORE-005` | Exposed lie/successful challenge counters increment. | `rules.rs`. | `challenge-exposed-lie.trace.json`; `tests/rules.rs`. | covered | Public tiebreak counters. |
| `MC-SCORE-006` | Scores accumulate. | `rules.rs`; `state.rs`. | `tests/property.rs`; simulation; terminal traces. | covered | Public score totals. |
| `MC-SETUP-001` | Create exactly two seats. | `setup.rs`; `ids.rs`. | `tests/serialization.rs`; fixture validation. | covered | No other seat count. |
| `MC-SETUP-002` | Stable mask set and seeded shuffle. | `ids.rs`; `setup.rs`. | `tests/property.rs`; `tests/replay.rs`; `state_hash_terminal` benchmark. | covered | Deterministic seeded setup. |
| `MC-SETUP-003` | Deal hands and reserve. | `setup.rs`; `state.rs`. | `tests/property.rs`; `tests/visibility.rs`. | covered | Conservation invariant covers 15 masks. |
| `MC-SETUP-004` | Seat 0 starts; claimants alternate. | `setup.rs`; `rules.rs`. | `tests/rules.rs`; `shortest-normal.trace.json`. | covered | Four claims per seat by terminal. |
| `MC-SETUP-005` | Initialize empty public/internal containers. | `setup.rs`; `state.rs`; `visibility.rs`. | `tests/serialization.rs`; `tests/visibility.rs`. | covered | Initial projection is safe. |
| `MC-TURN-001` | Claim phase sequence. | `actions.rs`; `rules.rs`. | `claim-pending-window.trace.json`; `tests/rules.rs`. | covered | Claim opens window. |
| `MC-TURN-002` | Reaction window sequence. | `actions.rs`; `rules.rs`. | `claim-pending-window.trace.json`; `tests/rules.rs`. | covered | Responder-only accept/challenge. |
| `MC-TURN-003` | Accept resolution. | `rules.rs`. | `accept-resolution.trace.json`; `tests/rules.rs`. | covered | Scores and veils. |
| `MC-TURN-004` | Challenge resolution. | `rules.rs`; `effects.rs`. | challenge traces; `tests/rules.rs`; `challenge_reveal_appears_after_public_claim_effect`. | covered | Reveal ordering checked. |
| `MC-TURN-005` | Non-final cleanup advances claimant. | `rules.rs`. | `tests/rules.rs`; simulation. | covered | Pedestal clears and claimant alternates. |
| `MC-TURN-006` | Final cleanup records terminal. | `rules.rs`. | terminal traces; `tests/rules.rs`. | covered | Terminal rationale emitted. |
| `MC-TURN-007` | Terminal state has no normal play. | `actions.rs`; `rules.rs`. | terminal traces; simulation. | covered | No further gameplay. |
| `MC-VAR-001` | Standard variant only. | `variants.rs`; static data. | `tests/serialization.rs`; fixture validation. | covered | Variant data carries labels/constants only. |
| `MC-VAR-002` | Variant data has no behavior. | `variants.rs`; static data parsers. | `tests/serialization.rs`; `fixture-check --game masked_claims`. | covered | Unknown behavior-like fields rejected. |
| `MC-VIS-001` | Public safe facts are visible. | `visibility.rs`. | `tests/visibility.rs`; `public-observer-no-leak.trace.json`; `project_public_view_pending_reaction` benchmark. | covered | Public phase/scores/counters projected. |
| `MC-VIS-002` | Unplayed hand masks owner-only. | `visibility.rs`. | `tests/visibility.rs`; wasm-api masked bridge test. | covered | Opponent/observer do not receive IDs. |
| `MC-VIS-003` | Pedestal identity hidden before challenge. | `visibility.rs`; `effects.rs`. | `claim-pending-window.trace.json`; `tests/visibility.rs`. | covered | Public sees declared grade only. |
| `MC-VIS-004` | Challenged mask becomes public. | `rules.rs`; `effects.rs`; `visibility.rs`. | challenge traces; `project_public_view_after_reveal` benchmark. | covered | Reveal effect precedes exposed row. |
| `MC-VIS-005` | Accepted masks never reveal. | `rules.rs`; `visibility.rs`; `replay_support.rs`. | `accepted-mask-never-revealed.trace.json`; `tests/visibility.rs`. | covered | Even claimant loses tile identity after veiling. |
| `MC-VIS-006` | Reserve never reveals. | `visibility.rs`. | `tests/visibility.rs`; `tests/property.rs`. | covered | Reserve remains internal only. |
| `MC-VIS-007` | Claim choices actor-scoped. | `actions.rs`; `visibility.rs`. | `tests/visibility.rs`; wasm-api masked bridge test. | covered | Own held masks only. |
| `MC-VIS-008` | Response choices responder-scoped. | `actions.rs`; `visibility.rs`. | `tests/visibility.rs`; `claim-pending-window.trace.json`. | covered | Accept/challenge only. |
| `MC-VIS-009` | Bot rationale/candidates are safe. | `bots.rs`. | `tests/bots.rs`; `certain-lie-challenge.trace.json`; `bot-claim-and-response.trace.json`. | covered | Rationale uses own/public facts only. |

## Test Mapping Summary

| Test suite/file | Type | Rule IDs covered | What it proves | What it does not prove |
|---|---|---|---|---|
| `games/masked_claims/tests/rules.rs` and `src/rules.rs` | rule/unit | setup, turn, scoring, terminal, restrictions | Claim/response resolution, scoring, terminal tiebreaks. | Browser rendering. |
| `games/masked_claims/tests/property.rs` | property | setup, scoring, RNG, conservation | Many seeded legal sequences preserve invariants. | Pixel/UI behavior. |
| `games/masked_claims/tests/visibility.rs` and `src/visibility.rs` | no-leak | visibility and redaction rules | Public/opponent/export surfaces omit hidden facts. | Browser DOM until web ticket. |
| `games/masked_claims/tests/replay.rs` | replay | replay/RNG/export rules | Deterministic replay surfaces and golden trace metadata. | Full external replay adapter hash reconstruction. |
| `games/masked_claims/tests/serialization.rs` | serialization | static data, variant, view/export stability | Unknown fields reject and JSON round-trips. | Performance. |
| `games/masked_claims/tests/bots.rs` and `src/bots.rs` | bot | bot rules | Bots use legal APIs and allowed views in both roles. | Strategic optimality. |

## Simulation/Fuzz Coverage Summary

| Simulation/fuzz run | Seeds/count | Bots/policies | Rule IDs stressed | Metrics recorded | Status/notes |
|---|---:|---|---|---|---|
| `cargo run -p simulate -- --game masked_claims --games 1000` | 1000 | `MaskedClaimsLevel1Bot` | claim, response, scoring, terminal, bot legality | completed games, winners/draws, average length, throughput | covered |

## Benchmark Relevance Map

| Benchmark | Rule IDs/mechanics relevant | Why relevant | Current threshold/status |
|---|---|---|---|
| `legal_actions_claim_phase` | `MC-ACT-001`, `MC-VIS-007` | Claim tree is the largest legal-action surface. | smoke floor in `thresholds.json` |
| `legal_actions_reaction_window` | `MC-ACT-003`, `MC-VIS-008` | Reaction windows are the gate proof. | smoke floor in `thresholds.json` |
| `validate_claim` | `MC-RESTRICT-*`, `MC-ACT-001` | Validates freshness, seat, tile, and grade. | smoke floor in `thresholds.json` |
| `apply_claim_open_window` | `MC-TURN-001`, `MC-COMP-007` | Opens the pending response window. | smoke floor in `thresholds.json` |
| `apply_accept_resolution` | `MC-SCORE-001`, `MC-VIS-005` | Scores and veils accepted masks. | smoke floor in `thresholds.json` |
| `apply_challenge_resolve_reveal` | `MC-SCORE-002`, `MC-SCORE-003`, `MC-VIS-004` | Reveals and resolves challenges. | smoke floor in `thresholds.json` |
| `project_public_view_pending_reaction` | `MC-VIS-001`, `MC-VIS-003` | Projects safe pending claim state. | smoke floor in `thresholds.json` |
| `project_public_view_after_reveal` | `MC-VIS-004` | Projects safe public reveal state. | smoke floor in `thresholds.json` |
| `state_hash_terminal` | `MC-RNG-003`, `MC-END-*` | Pins stable terminal summaries. | smoke floor in `thresholds.json` |
| `public_export_timeline` | `MC-RNG-002`, `MC-DEV-003` | Ensures public export/import path exists. | smoke floor in `thresholds.json` |
| `level1_bot_claim_decision` | `MC-BOT-002` | Exercises claim policy. | smoke floor in `thresholds.json` |
| `level1_bot_response_decision` | `MC-BOT-003` | Exercises response policy. | smoke floor in `thresholds.json` |

## Coverage Review Checklist

- Every rule ID in `RULES.md` has one row in the matrix above.
- Rust tests and traces are primary for rule correctness.
- UI smoke is not claimed as completed until the browser ticket lands.
- Hidden-information surfaces are covered by native tests and bridge tests.
- Benchmarks map to the implemented operations and smoke thresholds.
