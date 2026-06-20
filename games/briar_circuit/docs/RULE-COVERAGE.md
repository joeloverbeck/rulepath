# Briar Circuit Rule Coverage

Game ID: `briar_circuit`

Rules version: `briar-circuit-rules-v1`

Last updated: 2026-06-21

## Rule Coverage Matrix

This is the initial coverage skeleton. Rows are `open` until the implementation
tickets land the named modules, tests, traces, fixtures, tools, WASM bridge, and
browser smoke evidence.

| Rule ID | Rule summary | Planned implementation | Planned evidence | Status | Notes |
|---|---|---|---|---|---|
| `BC-SETUP-001` | Exactly four seats are accepted; all other counts receive a stable diagnostic. | `games/briar_circuit/src/setup.rs` | unit/rule tests, fixture check, WASM setup diagnostic test | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-SETUP-002` | Standard deck has 52 unique cards and deterministic canonical IDs. | `games/briar_circuit/src/cards.rs` | unit/property/serialization tests | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-DEAL-001` | Seeded shuffle/deal produces 13 unique private cards per seat and no remainder. | `games/briar_circuit/src/setup.rs`; `games/briar_circuit/src/state.rs` | property tests, setup trace, replay check | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-DEAL-002` | Dealer rotates clockwise after each hand; deal starts left of dealer. | `games/briar_circuit/src/setup.rs`; `games/briar_circuit/src/scoring.rs` | rule tests, multi-hand trace | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-PASS-001` | Pass cycle is left, right, across, hold, repeating by hand index. | `games/briar_circuit/src/rules.rs`; `games/briar_circuit/src/state.rs` | unit/rule tests, pass-cycle trace | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-PASS-002` | Each seat selects exactly three distinct owned cards on a pass hand. | `games/briar_circuit/src/actions.rs` | rule/diagnostic/property tests | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-PASS-003` | No incoming cards are delivered until all four commitments; exchange is atomic. | `games/briar_circuit/src/actions.rs`; `games/briar_circuit/src/effects.rs`; `games/briar_circuit/src/visibility.rs` | rule tests, pass-in-flight trace, visibility tests | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-PASS-004` | Hold hand skips selection/exchange and proceeds to 2 clubs opening. | `games/briar_circuit/src/rules.rs` | rule test, hold-hand trace | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-PLAY-001` | Holder of 2 clubs leads it to trick one. | `games/briar_circuit/src/actions.rs` | rule test, opening trace | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-PLAY-002` | A seat must follow led suit when able. | `games/briar_circuit/src/actions.rs` | rule/property/diagnostic tests, trace | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-PLAY-003` | A void seat may discard any otherwise legal card. | `games/briar_circuit/src/actions.rs` | rule tests, void-discard trace | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-PLAY-004` | Hearts and queen of spades are forbidden on trick one while a non-point discard exists; no-alternative exception allows all. | `games/briar_circuit/src/actions.rs` | rule/property tests, restriction trace, exception trace | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-PLAY-005` | Hearts cannot be led unbroken while a non-heart remains. | `games/briar_circuit/src/actions.rs` | rule/diagnostic tests, trace | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-PLAY-006` | If the leader holds only hearts, a heart lead is legal and breaks hearts. | `games/briar_circuit/src/actions.rs`; `games/briar_circuit/src/state.rs` | rule test, all-hearts lead trace | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-PLAY-007` | A played heart breaks hearts; queen of spades alone does not. | `games/briar_circuit/src/rules.rs`; `games/briar_circuit/src/effects.rs` | rule tests, broken-heart traces | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-TRICK-001` | Highest card of led suit wins; off-suit never wins. | `games/briar_circuit/src/rules.rs` | unit/property tests, trick trace | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-TRICK-002` | Trick winner captures all four cards and leads next unless hand closes. | `games/briar_circuit/src/rules.rs`; `games/briar_circuit/src/state.rs` | rule/property tests, winner-leads trace | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-SCORE-001` | Each captured heart is 1; captured queen of spades is 13; others 0. | `games/briar_circuit/src/scoring.rs` | unit/property tests | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-SCORE-002` | Raw point total for a complete hand is 26. | `games/briar_circuit/src/scoring.rs` | property tests, hand-scoring trace | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-SCORE-003` | Capturing all 26 triggers fixed moon: shooter +0, each opponent +26. | `games/briar_circuit/src/scoring.rs` | unit/rule tests, moon trace | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-MATCH-001` | Hand additions accumulate monotonically by seat. | `games/briar_circuit/src/scoring.rs`; `games/briar_circuit/src/state.rs` | property tests, replay check | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-MATCH-002` | Threshold check occurs after a completed hand when any score is at least 100. | `games/briar_circuit/src/scoring.rs` | rule test, threshold trace | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-MATCH-003` | Unique lowest score wins; tied low continues complete hands without seat-order tie-break. | `games/briar_circuit/src/scoring.rs` | rule tests, simulation, threshold-tie trace | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-VIS-001` | Unplayed cards are visible only to their owner. | `games/briar_circuit/src/visibility.rs` | pairwise visibility tests, no-leak trace, e2e canary scan | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-VIS-002` | Pass selection and pass provenance remain owner-only; card identity may separately become public only when played. | `games/briar_circuit/src/visibility.rs`; `games/briar_circuit/src/replay_support.rs` | pairwise visibility tests, export scan, e2e canary scan | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-VIS-003` | Deck order and seed-reconstructable material never enter viewer-scoped export. | `games/briar_circuit/src/replay_support.rs`; `games/briar_circuit/src/visibility.rs` | export scan, replay test | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-VIS-004` | Unauthorized viewers receive no private action tree, preview, diagnostics, effects, bot candidates, or explanation facts. | `games/briar_circuit/src/actions.rs`; `games/briar_circuit/src/effects.rs`; `games/briar_circuit/src/bots.rs`; `games/briar_circuit/src/visibility.rs` | pairwise harness, bot no-leak tests, e2e canary scan | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-REPLAY-001` | Internal replay reproduces deterministic state/effect/action/view hashes. | `games/briar_circuit/src/replay_support.rs` | replay tests, golden traces, `replay-check --all` | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-REPLAY-002` | Public and seat-private exports reproduce only authorized observation timelines. | `games/briar_circuit/src/replay_support.rs`; WASM replay adapter | replay/export tests, visibility tests | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-BOT-001` | L0 samples uniformly from the Rust legal leaf set using declared bot RNG. | `games/briar_circuit/src/bots.rs` | bot unit tests, trace, simulation | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-BOT-002` | L1 uses only own projected hand, public state/history, legal actions, and deterministic tie-breaks. | `games/briar_circuit/src/bots.rs` | input audit, bot tests, no-leak trace | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-UI-001` | Browser controls expose legal actions only and do not derive legality. | `apps/web/src/components/BriarCircuitBoard.tsx`; WASM legal tree | e2e smoke, boundary review | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-OUTCOME-001` | Rust supplies per-seat hand and cumulative breakdown, moon adjustment, threshold/tie reason, and terminal winner. | `games/briar_circuit/src/scoring.rs`; `crates/wasm-api` adapter | rule tests, terminal traces, outcome check | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-VAR-001` | `briar_circuit_standard` is the only shipped variant. | `games/briar_circuit/data/variants.toml`; setup defaults | serialization tests, fixture check | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-VAR-002` | Public copy uses Briar Circuit as the original neutral name. | docs, manifest, web catalog | catalog docs check, player-rules check, manual IP review | `covered` | Implemented evidence landed through Gate 16 tickets. |
| `BC-OOS-001` | Variable seats, partnerships, teams, bidding, trump, bonus variants, moon choice, shoot the sun, hosted multiplayer, and takebacks are out of scope. | no implementation path | docs review, setup rejection tests | `not-applicable` | Explicitly out of scope. |
| `BC-OOS-002` | No generic card, suit, hand, pass, or trick helper is added. | local `games/briar_circuit` modules only | boundary check, mechanic atlas review | `not-applicable` | Gate 16 keeps local/defer. |
| `BC-OOS-003` | Static data cannot define behavior. | typed data loaders and fixture validation | fixture checks, serialization tests, boundary review | `not-applicable` | No formulas/selectors/triggers/scripts. |
| `BC-OOS-004` | Solver and learning bots are forbidden. | `games/briar_circuit/src/bots.rs` | bot tests, source scan | `not-applicable` | No MCTS/ISMCTS/Monte Carlo/ML/RL. |

## Planned Golden Trace Inventory

| Trace | Required focus | Rule IDs |
|---|---|---|
| standard opening | deal, pass, 2 clubs opening, first trick | `BC-DEAL-001`, `BC-PASS-001`, `BC-PLAY-001` |
| first-trick restriction | no-points-on-first-trick and no-alternative exception | `BC-PLAY-004` |
| hearts broken | heart play breaks hearts; queen of spades does not | `BC-PLAY-005` through `BC-PLAY-007` |
| moon | all 26 raw points captured by one seat and transformed | `BC-SCORE-002`, `BC-SCORE-003`, `BC-OUTCOME-001` |
| threshold tie | low-score tie continues beyond threshold | `BC-MATCH-002`, `BC-MATCH-003` |
| no-leak export | observer and seat-private exports remain authorized | `BC-VIS-001` through `BC-REPLAY-002` |

## Initial Coverage Status

This skeleton intentionally records implementation rows as `open` because no
`games/briar_circuit` crate exists yet. The gate cannot be marked official or
Done until later tickets replace open rows with concrete files, tests, traces,
tool output, browser smoke, benchmark evidence, and release-doc receipts.
