# Starbridge Crossing Rule Coverage

Game ID: `starbridge_crossing`

Rules version: `starbridge-crossing-rules-v1`

Data version: `starbridge-crossing-data-v1`

Last updated: 2026-06-28

## Rule Coverage Matrix

This matrix maps stable `SC-*` rules to current Rust, fixture, trace, WASM, and
browser evidence. Human public-release review remains outside rule coverage.

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `SC-ID-001` | Game id, variant, rules version, and data version are stable. | `ids.rs`, `variants.rs`, `topology.rs`, manifest data, tool registrations | manifest/variant tests, `fixture-check`, `replay-check`, `rule-coverage` | `covered` | WASM/catalog wiring lands later. |
| `SC-ID-002` | Public copy uses Starbridge Crossing; source-family labels are maintenance context only. | [RULES.md](RULES.md), [SOURCES.md](SOURCES.md), [AI.md](AI.md) | source docs, AI docs, doc-link check | `covered` | Human public-release review remains pending. |
| `SC-SETUP-001` | Supported seat counts are exactly 2, 3, 4, and 6. | `setup.rs`, `ids.rs`, `variants.rs` | setup tests, setup traces, fixture profiles | `covered-by-trace` | Unsupported counts reject with stable diagnostics. |
| `SC-SETUP-002` | Stable point labels are clockwise from north. | `ids.rs`, `topology.rs`, `setup.rs` | id/topology tests, setup traces | `covered-by-trace` | Labels are game-local nouns only. |
| `SC-SETUP-003` | Active homes and targets are deterministic by seat count. | `ids.rs`, `setup.rs` | setup home/target tests, setup traces | `covered-by-trace` | Four-player setup uses two opposite pairs. |
| `SC-SETUP-004` | The board has 121 stable star-topology spaces. | `topology.rs`, `data/manifest.toml` | topology property tests, manifest receipt tests | `covered` | Rust-generated topology is the deterministic content authority. |
| `SC-SETUP-005` | Each active seat starts with 10 public pegs in its home. | `setup.rs`, `state.rs` | setup tests, setup traces | `covered-by-trace` | Fifteen-piece variants are out of scope. |
| `SC-SETUP-006` | Setup ordering is deterministic from seed and version inputs. | `setup.rs`, `variants.rs`, `replay_support.rs` | serialization and replay tests, setup traces | `covered-by-trace` | Wall-clock and browser randomness are not inputs. |
| `SC-VIS-001` | Public observer sees all board facts and no hidden state. | `visibility.rs`, `ui.rs`, `effects.rs` | visibility tests, public-observer trace | `covered-by-trace` | Starbridge has no hidden-information class. |
| `SC-VIS-002` | Seat viewers receive the same board facts as public observer. | `visibility.rs`, `ui.rs` | seat-viewer parity tests and trace | `covered-by-trace` | Seat-local labels do not add private facts. |
| `SC-VIS-003` | There are no private hands, decks, commitments, roles, or hidden score facts. | `visibility.rs`, `state.rs`, `replay_support.rs` | no-private-visibility tests, no-leak traces | `covered-by-trace` | Hidden-info redaction is not applicable. |
| `SC-VIS-004` | Public replay exports may include all game facts and remain deterministic. | `replay_support.rs`, `visibility.rs` | replay round-trip tests and trace | `covered-by-trace` | Export/import stays versioned. |
| `SC-TURN-001` | One active seat acts at a time and play advances clockwise. | `rules.rs`, `state.rs` | turn-flow rules tests, action/effect traces | `covered-by-trace` | Finished seats are skipped. |
| `SC-TURN-002` | A turn is one step, one hop chain, or forced blocked pass. | `actions.rs`, `rules.rs` | step, jump, mixed-step-jump, blocked-pass tests and traces | `covered-by-trace` | Step and hop cannot mix. |
| `SC-TURN-003` | Finished seats keep pegs on board and are skipped. | `rules.rs`, `state.rs`, `effects.rs` | finish-order tests and traces | `covered-by-trace` | Occupancy remains public. |
| `SC-MOVE-001` | A step moves one owned peg to an adjacent empty space. | `actions.rs`, `rules.rs`, `topology.rs` | step tests and single-step trace | `covered-by-trace` | Adjacency is Rust-owned. |
| `SC-MOVE-002` | Illegal steps reject without mutation. | `rules.rs`, `actions.rs` | invalid-step tests and diagnostics | `covered` | Diagnostics are viewer-safe. |
| `SC-MOVE-003` | A hop crosses one adjacent occupied space into the empty space beyond. | `actions.rs`, `rules.rs`, `topology.rs` | one-hop and jump-chain tests/traces | `covered-by-trace` | Jumped pegs are not captured. |
| `SC-MOVE-004` | Illegal hops reject without mutation. | `rules.rs`, `actions.rs` | invalid-hop and repeated-landing tests/traces | `covered-by-trace` | Landing and midpoint checks are Rust-owned. |
| `SC-MOVE-005` | Hop chains may continue and change direction after each landing. | `actions.rs`, `rules.rs` | multi-hop direction-change tests and trace | `covered-by-trace` | Continuations are deterministic. |
| `SC-MOVE-006` | A player may stop after any legal hop landing. | `actions.rs`, `rules.rs` | stop-midway tests and trace | `covered-by-trace` | Stop leaves are in the action tree. |
| `SC-MOVE-007` | A hop chain may not revisit a landing in the same turn. | `actions.rs`, `rules.rs` | repeat-landing rejection tests and trace | `covered-by-trace` | This bounds the action tree. |
| `SC-MOVE-008` | A move may not combine a step with a hop. | `actions.rs`, `rules.rs` | mixed-step-jump rejection tests and trace | `covered-by-trace` | Accepted paths are step-only or hop-only. |
| `SC-MOVE-009` | A blocked active seat gets exactly one forced pass. | `actions.rs`, `rules.rs`, `effects.rs` | blocked-pass tests and trace | `covered-by-trace` | No optional strategic pass exists. |
| `SC-MOVE-010` | A hop chain may not land on the moving peg's own origin space. | `actions.rs`, `rules.rs` | `tests/rules.rs::hop_chain_cannot_return_to_origin_space`, `tests/property.rs::committed_non_pass_turns_change_board_occupancy` | `covered` | Prevents net-zero no-op turns; trace/hash migration lands in `GAT201STACROHOP-003`. |
| `SC-SCORE-001` | Finish-rank accounting, not point scoring, determines standings. | `rules.rs`, `state.rs`, `effects.rs`, `visibility.rs` | finish assignment, terminal standings, and outcome rationale tests/traces | `covered-by-trace` | Alias surface for `SC-FINISH-001` through `SC-FINISH-004`. |
| `SC-SCORE-002` | Turn-limit fallback accounts for unfinished seats by progress vector and clockwise order. | `rules.rs`, `state.rs` | turn-limit tests and trace | `covered-by-trace` | Alias surface for `SC-FINISH-005` and `SC-FINISH-006`. |
| `SC-END-001` | Normal terminal condition is all-but-one seats finished. | `rules.rs`, `state.rs`, `effects.rs` | terminal standings tests and trace | `covered-by-trace` | Equivalent to `SC-FINISH-003`. |
| `SC-END-002` | Turn-limit terminal condition records deterministic unfinished standings. | `rules.rs`, `state.rs` | turn-limit tests and trace | `covered-by-trace` | Equivalent to `SC-FINISH-006`. |
| `SC-FINISH-001` | A seat receives next rank when all pegs reach target home. | `rules.rs`, `state.rs`, `effects.rs` | finish assignment tests and trace | `covered-by-trace` | Checked after accepted moves. |
| `SC-FINISH-002` | Finish ranks are assigned in completion order. | `rules.rs`, `state.rs` | finish-order tests and traces | `covered-by-trace` | Lower rank is better. |
| `SC-FINISH-003` | Match ends when all but one active seat are ranked. | `rules.rs`, `state.rs` | terminal standings tests and traces | `covered-by-trace` | Last unfinished seat receives final rank. |
| `SC-FINISH-004` | Terminal standings are stable and Rust-authored. | `rules.rs`, `visibility.rs`, `ui.rs`, `crates/wasm-api`, `StarbridgeCrossingBoard.tsx` | terminal standings traces, `tests/visibility.rs` terminal rationale regressions, wasm rationale serialization test, Starbridge terminal e2e panel smoke | `covered-by-trace` | TypeScript renders Rust-projected `terminal_rationale` only. |
| `SC-FINISH-005` | Variants include deterministic max plies. | `variants.rs`, `rules.rs` | variant tests and turn-limit trace | `covered-by-trace` | Default max plies is 2000. |
| `SC-FINISH-006` | Turn limit records deterministic unfinished standings. | `rules.rs`, `state.rs`, `visibility.rs`, `crates/wasm-api`, `StarbridgeCrossingBoard.tsx` | turn-limit tests and trace, progress-vector rationale regressions, wasm serialization test, Starbridge terminal e2e panel smoke | `covered-by-trace` | Progress vector is public and Rust-owned. |
| `SC-REPLAY-001` | Accepted command streams reproduce state, effects, views, and hashes. | `replay_support.rs`, `rules.rs`, `visibility.rs` | replay tests, public replay trace, `replay-check` | `covered-by-trace` | Trace receipts are versioned. |
| `SC-REPLAY-002` | Trace Schema v1 records setup, moves, diagnostics, terminal, and visibility notes. | golden traces, `replay_support.rs` | trace inventory tests, `fixture-check`, `replay-check` | `covered-by-trace` | No trace migration is authorized. |
| `SC-BOT-001` | L0 bots select deterministically from Rust legal actions. | `bots.rs`, `tools/simulate` | bot tests, L0 trace, simulator smoke | `covered-by-trace` | L0 submits through normal validation. |
| `SC-BOT-002` | Higher bots may use only public facts and admitted evidence. | [AI.md](AI.md), `bots.rs` | AI docs and not-admitted policy constant | `covered` | Higher bots are not admitted in this gate. |
| `SC-BOT-003` | MCTS, ISMCTS, Monte Carlo, ML, RL, and runtime LLM move selection are forbidden. | [AI.md](AI.md), `bots.rs` | AI docs, code review, bot tests | `covered` | Applies even for perfect-information play. |
| `SC-UI-001` | Browser controls present Rust legal actions and previews only. | `crates/wasm-api`, `apps/web/src/components/StarbridgeCrossingBoard.tsx` | wasm API surface tests, Starbridge web smoke legal-action/preview flow, `boundary-check.sh` | `covered` | Rust/WASM provide legal actions; TypeScript presents them. |
| `SC-UI-002` | Public UI supports board, peg selection, hop chains, replay, and no-drag paths. | `apps/web/src/components/StarbridgeCrossingBoard.tsx`, Starbridge e2e smoke | board/jump/replay/responsive smoke plus terminal outcome panel smoke | `covered` | Includes `OutcomeExplanationPanel` and `aria-live` terminal surface. |
| `SC-UI-003` | DOM, a11y names, test IDs, storage, logs, and effects contain public facts only. | `visibility.rs`, `crates/wasm-api`, Starbridge e2e no-leak scan | visibility no-private regressions, wasm rationale serialization, Starbridge no-leak/browser smoke | `covered` | Starbridge remains all-public; terminal rationale adds no hidden facts. |

## Golden Trace Inventory

| Trace | Required focus | Rule IDs |
|---|---|---|
| `setup-2p-standard.trace.json` | 2-seat setup and public board facts | `SC-SETUP-001`, `SC-SETUP-003`, `SC-SETUP-005`, `SC-VIS-001` |
| `setup-3p-standard.trace.json` | 3-seat setup | `SC-SETUP-001`, `SC-SETUP-003`, `SC-SETUP-005` |
| `setup-4p-standard.trace.json` | 4-seat setup | `SC-SETUP-001`, `SC-SETUP-003`, `SC-SETUP-005` |
| `setup-6p-standard.trace.json` | max-seat setup | `SC-SETUP-001`, `SC-SETUP-003`, `SC-SETUP-005` |
| `single-step-move.trace.json` | legal step | `SC-MOVE-001`, `SC-TURN-002` |
| `one-hop-move.trace.json` | legal hop | `SC-MOVE-003` |
| `jump-chain.trace.json` | hop continuation and stop leaf | `SC-MOVE-005`, `SC-MOVE-006` |
| `multi-hop-change-direction.trace.json` | direction change in hop chain | `SC-MOVE-005` |
| `repeat-landing-rejected.trace.json` | repeated landing rejection | `SC-MOVE-007` |
| `invalid-mixed-step-jump.trace.json` | mixed step/hop rejection | `SC-MOVE-008` |
| `blocked-forced-pass.trace.json` | forced pass | `SC-MOVE-009` |
| `reach-home-first-finish.trace.json` | first finish rank | `SC-FINISH-001`, `SC-FINISH-002` |
| `finish-order-continues.trace.json` | finished-seat skip | `SC-TURN-003`, `SC-FINISH-002` |
| `terminal-full-standings.trace.json` | terminal all-but-one standings | `SC-END-001`, `SC-SCORE-001`, `SC-FINISH-003`, `SC-FINISH-004` |
| `turn-limit-cutoff.trace.json` | deterministic turn-limit standings | `SC-END-002`, `SC-SCORE-002`, `SC-FINISH-005`, `SC-FINISH-006` |
| `public-observer-all-public.trace.json` | all-public observer view | `SC-VIS-001`, `SC-VIS-003` |
| `seat-viewer-parity.trace.json` | seat/public parity | `SC-VIS-002`, `SC-VIS-003` |
| `public-replay-round-trip.trace.json` | replay and export round trip | `SC-REPLAY-001`, `SC-VIS-004` |
| `bot-l0-action.trace.json` | L0 random-legal receipt | `SC-BOT-001` |

## Coverage Status

This matrix is complete for current Rust, tooling, WASM, and browser outcome
evidence. Human public-release review remains outside rule coverage.
