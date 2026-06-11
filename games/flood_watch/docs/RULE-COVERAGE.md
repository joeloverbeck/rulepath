# flood_watch Rule Coverage Matrix

Game ID: `flood_watch`

Rules version: `flood-watch-rules-v1`

This matrix maps every stable rule ID in [RULES.md](RULES.md) to code, tests,
traces, docs, or tool gates. `rule-coverage --game flood_watch` enforces exactly
one row per `FW-*` rule ID.

## Rule Coverage Matrix

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
| --- | --- | --- | --- | --- | --- |
| `FW-ACT-001` | Flood Watch obligation. | game-local Rust/docs/traces | `games/flood_watch/src/actions.rs`; `tests/rules.rs`; `tests/golden_traces/budget-exhaustion-auto-environment.trace.json` | covered | Active-seat legal leaves are generated from Rust state. |
| `FW-ACT-002` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `role-power-pumpwright.trace.json`; `bail-dry-district-diagnostic.trace.json` | covered-by-trace | Bail legality and role amount are traced. |
| `FW-ACT-003` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `role-power-levee-warden.trace.json`; `levee-absorption.trace.json` | covered-by-trace | Reinforce legality and cap behavior are traced. |
| `FW-ACT-004` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `forecast-public-reveal.trace.json`; `visibility.rs` | covered-by-trace | Forecast exposes only the top card. |
| `FW-ACT-005` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `early-end-turn.trace.json`; `tests/rules.rs` | covered-by-trace | End turn prevents stalls. |
| `FW-ACT-006` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `wrong-seat-diagnostic.trace.json`; `tests/rules.rs` | covered-by-trace | Non-active trees/diagnostics stay public. |
| `FW-ACT-007` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `standard-win.trace.json`; `loss-by-inundation.trace.json` | covered-by-trace | Terminal action tree is empty. |
| `FW-AMB-001` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `replay_support.rs`; `budget-exhaustion-auto-environment.trace.json` | covered-by-trace | Environment is an effect batch, not a command actor. |
| `FW-AMB-002` | Flood Watch obligation. | game-local Rust/docs/traces | `visibility.rs`; `public-observer-no-leak.trace.json`; `public-replay-export-import.trace.json` | covered-by-trace | Terminal exports remain redacted. |
| `FW-AMB-003` | Flood Watch obligation. | game-local Rust/docs/traces | `forecast-public-reveal.trace.json`; `visibility.rs`; `effects.rs` | covered-by-trace | Forecast is public to all viewers. |
| `FW-AMB-004` | Flood Watch obligation. | game-local Rust/docs/traces | `RULES.md`; `actions.rs`; `tests/serialization.rs` | covered | Districts are flat IDs only. |
| `FW-AMB-005` | Flood Watch obligation. | game-local Rust/docs/traces | `variants.rs`; `tests/serialization.rs`; `fixture-check --game flood_watch` | covered | Static data rejects behavior-looking fields. |
| `FW-BOT-001` | Flood Watch obligation. | game-local Rust/docs/traces | `bots.rs`; `tests/bots.rs` | covered | Random legal bot uses legal tree only. |
| `FW-BOT-002` | Flood Watch obligation. | game-local Rust/docs/traces | `bots.rs`; `tests/bots.rs`; `BOT-STRATEGY-EVIDENCE-PACK.md` | covered | Level 1 priority policy is documented and tested. |
| `FW-BOT-003` | Flood Watch obligation. | game-local Rust/docs/traces | `tests/bots.rs`; `COMPETENT-PLAYER.md` | covered | Both seats/roles are covered. |
| `FW-COMP-001` | Flood Watch obligation. | game-local Rust/docs/traces | `state.rs`; `setup.rs`; `tests/serialization.rs` | covered | Two cooperative seats are stable. |
| `FW-COMP-002` | Flood Watch obligation. | game-local Rust/docs/traces | `ids.rs`; `setup.rs`; `role-power-pumpwright.trace.json`; `role-power-levee-warden.trace.json` | covered-by-trace | Roles are public game-local modifiers. |
| `FW-COMP-003` | Flood Watch obligation. | game-local Rust/docs/traces | `ids.rs`; `state.rs`; `RULES.md` | covered | District IDs are closed local nouns. |
| `FW-COMP-004` | Flood Watch obligation. | game-local Rust/docs/traces | `state.rs`; `rules.rs`; `tests/property.rs` | covered | Flood level bounds are tested. |
| `FW-COMP-005` | Flood Watch obligation. | game-local Rust/docs/traces | `state.rs`; `rules.rs`; `levee-absorption.trace.json` | covered-by-trace | Levee prevention is public and capped. |
| `FW-COMP-006` | Flood Watch obligation. | game-local Rust/docs/traces | `setup.rs`; `state.rs`; `tests/visibility.rs` | covered | Event deck order is internal only. |
| `FW-COMP-007` | Flood Watch obligation. | game-local Rust/docs/traces | `ids.rs`; `rules.rs`; `forecast-public-reveal.trace.json` | covered-by-trace | Closed event kinds are Rust behavior. |
| `FW-COMP-008` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `visibility.rs`; `forecast-public-reveal.trace.json` | covered-by-trace | Forecast marker projection is public. |
| `FW-COMP-009` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `rules.rs`; `budget-exhaustion-auto-environment.trace.json` | covered-by-trace | Budget is public and regenerates. |
| `FW-COMP-010` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `effects.rs`; `early-end-turn.trace.json` | covered-by-trace | Environment effects are public automation. |
| `FW-COMP-011` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `effects.rs`; `standard-win.trace.json`; `loss-by-inundation.trace.json` | covered-by-trace | Outcome is shared. |
| `FW-DEV-001` | Flood Watch obligation. | game-local Rust/docs/traces | `SOURCES.md`; `RULES.md`; `GAME-IMPLEMENTATION-ADMISSION.md` | covered | Source/IP posture documented. |
| `FW-DEV-002` | Flood Watch obligation. | game-local Rust/docs/traces | `SOURCES.md`; `RULES.md`; `PRIMITIVE-PRESSURE-LEDGER.md` | covered | Excluded commercial mechanics are documented. |
| `FW-DEV-003` | Flood Watch obligation. | game-local Rust/docs/traces | `visibility.rs`; `tests/visibility.rs`; `public-replay-export-import.trace.json` | covered-by-trace | Terminal no-leak posture is tested. |
| `FW-END-001` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `loss-by-inundation.trace.json`; `tests/rules.rs` | covered-by-trace | Inundation loss is traced. |
| `FW-END-002` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `standard-win.trace.json`; `tests/rules.rs` | covered-by-trace | Deck-exhaustion win is traced. |
| `FW-END-003` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `visibility.rs`; `standard-win.trace.json`; `loss-by-inundation.trace.json` | covered-by-trace | Terminal state has no further normal actions. |
| `FW-ENV-001` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `budget-exhaustion-auto-environment.trace.json`; `early-end-turn.trace.json` | covered-by-trace | Environment follows turn-ending or final-budget actions. |
| `FW-ENV-002` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `storm-surge-double-rise.trace.json`; `forecast-public-reveal.trace.json` | covered-by-trace | Draw order is deterministic and public on draw. |
| `FW-ENV-003` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `levee-absorption.trace.json` | covered-by-trace | Downpour/levee ordering is traced. |
| `FW-ENV-004` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `storm-surge-double-rise.trace.json` | covered-by-trace | Storm surge multi-rise is traced. |
| `FW-ENV-005` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `reprieve-no-op.trace.json` | covered-by-trace | Reprieve consumes a card without district change. |
| `FW-ENV-006` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `loss-by-inundation.trace.json` | covered-by-trace | Inundation stops remaining draws. |
| `FW-ENV-007` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `standard-win.trace.json` | covered-by-trace | Deck exhaustion win is traced. |
| `FW-OOS-001` | Flood Watch obligation. | game-local Rust/docs/traces | `RULES.md`; `setup.rs`; `tests/serialization.rs` | not-applicable | Out of scope by gate definition; two seats are enforced. |
| `FW-OOS-002` | Flood Watch obligation. | game-local Rust/docs/traces | `RULES.md`; `ids.rs`; `actions.rs` | not-applicable | Graph pressure is assigned to a later gate. |
| `FW-OOS-003` | Flood Watch obligation. | game-local Rust/docs/traces | `RULES.md`; `rules.rs`; `PRIMITIVE-PRESSURE-LEDGER.md` | not-applicable | No reaction windows in Flood Watch. |
| `FW-OOS-004` | Flood Watch obligation. | game-local Rust/docs/traces | `RULES.md`; `visibility.rs`; `tests/visibility.rs` | not-applicable | No per-seat hidden information exists. |
| `FW-OOS-005` | Flood Watch obligation. | game-local Rust/docs/traces | `RULES.md`; web/local-first scope docs | not-applicable | Hosted multiplayer is outside v1/v2. |
| `FW-OOS-006` | Flood Watch obligation. | game-local Rust/docs/traces | `BOT-STRATEGY-EVIDENCE-PACK.md`; `bots.rs`; `tests/bots.rs` | not-applicable | Public bot law forbids MCTS/ML/RL. |
| `FW-RESTRICT-001` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `tests/rules.rs`; `wrong-seat-diagnostic.trace.json` | covered-by-trace | Unknown actors reject without mutation. |
| `FW-RESTRICT-002` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `wrong-seat-diagnostic.trace.json` | covered-by-trace | Wrong seat diagnostic is viewer-safe. |
| `FW-RESTRICT-003` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `out-of-budget-diagnostic.trace.json` | covered-by-trace | Malformed/unavailable paths reject. |
| `FW-RESTRICT-004` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `tests/replay.rs`; `tests/rules.rs` | covered | Stale token rejection is tested. |
| `FW-RESTRICT-005` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `bail-dry-district-diagnostic.trace.json` | covered-by-trace | Dry bail rejects. |
| `FW-RESTRICT-006` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `tests/rules.rs` | covered | Full/unknown reinforce target rejects. |
| `FW-RESTRICT-007` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `tests/rules.rs` | covered | Forecast unavailable states reject. |
| `FW-RESTRICT-008` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `standard-win.trace.json`; `loss-by-inundation.trace.json` | covered-by-trace | Terminal action rejection is covered. |
| `FW-RNG-001` | Flood Watch obligation. | game-local Rust/docs/traces | `setup.rs`; `tests/serialization.rs`; `tests/replay.rs` | covered | Seeded shuffle is deterministic. |
| `FW-RNG-002` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `replay_support.rs`; `tests/replay.rs` | covered | Event draws replay from command stream. |
| `FW-RNG-003` | Flood Watch obligation. | game-local Rust/docs/traces | `replay_support.rs`; `public-replay-export-import.trace.json`; `tests/visibility.rs` | covered-by-trace | Public export is redacted. |
| `FW-RNG-004` | Flood Watch obligation. | game-local Rust/docs/traces | `tests/serialization.rs`; `tests/replay.rs`; golden traces | covered | Stable serialization/hash surfaces are guarded. |
| `FW-SCORE-001` | Flood Watch obligation. | game-local Rust/docs/traces | `state.rs`; `rules.rs`; `tests/property.rs` | covered | Flood levels are public bounded counters. |
| `FW-SCORE-002` | Flood Watch obligation. | game-local Rust/docs/traces | `state.rs`; `rules.rs`; `levee-absorption.trace.json` | covered-by-trace | Levee stacks are capped and public. |
| `FW-SCORE-003` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `budget-exhaustion-auto-environment.trace.json` | covered-by-trace | Budget spend/refill is traced. |
| `FW-SCORE-004` | Flood Watch obligation. | game-local Rust/docs/traces | `visibility.rs`; `tests/visibility.rs`; `COMPETENT-PLAYER.md` | covered | Remaining composition is public counts, not order. |
| `FW-SCORE-005` | Flood Watch obligation. | game-local Rust/docs/traces | `state.rs`; `effects.rs`; `standard-win.trace.json`; `loss-by-inundation.trace.json` | covered-by-trace | No individual score exists. |
| `FW-SETUP-001` | Flood Watch obligation. | game-local Rust/docs/traces | `setup.rs`; `tests/serialization.rs`; `fixture-check --game flood_watch` | covered | Seat count is validated. |
| `FW-SETUP-002` | Flood Watch obligation. | game-local Rust/docs/traces | `variants.rs`; `tests/serialization.rs`; `fixture-check --game flood_watch` | covered | Scenario constants parse strictly. |
| `FW-SETUP-003` | Flood Watch obligation. | game-local Rust/docs/traces | `setup.rs`; `role-power-pumpwright.trace.json`; `role-power-levee-warden.trace.json` | covered-by-trace | Role assignment is public and deterministic. |
| `FW-SETUP-004` | Flood Watch obligation. | game-local Rust/docs/traces | `setup.rs`; `tests/serialization.rs`; `scenario-deluge-setup.trace.json` | covered-by-trace | Deck is built from closed event-kind counts. |
| `FW-SETUP-005` | Flood Watch obligation. | game-local Rust/docs/traces | `state.rs`; `tests/serialization.rs`; `standard-win.trace.json` | covered-by-trace | Initial state is deterministic. |
| `FW-TURN-001` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `rules.rs`; `tests/rules.rs` | covered | Action phase spends legal actions. |
| `FW-TURN-002` | Flood Watch obligation. | game-local Rust/docs/traces | `actions.rs`; `wrong-seat-diagnostic.trace.json` | covered-by-trace | Teammate waiting state is safe. |
| `FW-TURN-003` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `budget-exhaustion-auto-environment.trace.json` | covered-by-trace | Final budget triggers environment. |
| `FW-TURN-004` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `early-end-turn.trace.json` | covered-by-trace | Explicit end turn triggers environment. |
| `FW-TURN-005` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `effects.rs`; `storm-surge-double-rise.trace.json` | covered-by-trace | Environment batch emits public effects. |
| `FW-TURN-006` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `mid-phase-early-stop.trace.json`; `tests/rules.rs` | covered-by-trace | Cleanup advances active seat and budget. |
| `FW-TURN-007` | Flood Watch obligation. | game-local Rust/docs/traces | `rules.rs`; `standard-win.trace.json`; `loss-by-inundation.trace.json` | covered-by-trace | Terminal state is final. |
| `FW-VAR-001` | Flood Watch obligation. | game-local Rust/docs/traces | `variants.rs`; `flood_watch_standard.fixture.json`; `tests/serialization.rs` | covered | Standard scenario is the default. |
| `FW-VAR-002` | Flood Watch obligation. | game-local Rust/docs/traces | `variants.rs`; `flood_watch_deluge.fixture.json`; `scenario-deluge-setup.trace.json` | covered-by-trace | Deluge scenario parses and sets pressure. |
| `FW-VAR-003` | Flood Watch obligation. | game-local Rust/docs/traces | `variants.rs`; `tests/serialization.rs`; `fixture-check --game flood_watch` | covered | Scenario data cannot encode behavior. |
| `FW-VIS-001` | Flood Watch obligation. | game-local Rust/docs/traces | `visibility.rs`; `tests/visibility.rs`; `public-observer-no-leak.trace.json` | covered-by-trace | Public facts project to all viewers. |
| `FW-VIS-002` | Flood Watch obligation. | game-local Rust/docs/traces | `visibility.rs`; `tests/visibility.rs`; `public-replay-export-import.trace.json` | covered-by-trace | Undrawn deck order is never projected. |
| `FW-VIS-003` | Flood Watch obligation. | game-local Rust/docs/traces | `visibility.rs`; `forecast-public-reveal.trace.json` | covered-by-trace | Forecast is public after reveal. |
| `FW-VIS-004` | Flood Watch obligation. | game-local Rust/docs/traces | `visibility.rs`; `storm-surge-double-rise.trace.json`; `reprieve-no-op.trace.json` | covered-by-trace | Drawn cards remain public history. |
| `FW-VIS-005` | Flood Watch obligation. | game-local Rust/docs/traces | `bots.rs`; `tests/bots.rs`; `BOT-STRATEGY-EVIDENCE-PACK.md` | covered | Bot rationale uses public facts only. |

## Tool Gates

| Gate | Command | Status |
| --- | --- | --- |
| quick simulation | `cargo run -p simulate -- --game flood_watch --games 1000` | covered |
| replay drift | `cargo run -p replay-check -- --game flood_watch --all` | covered |
| fixture/schema | `cargo run -p fixture-check -- --game flood_watch` | covered |
| rule coverage | `cargo run -p rule-coverage -- --game flood_watch` | covered |
| boundary | `bash scripts/boundary-check.sh` | covered |
| native benchmarks | `cargo bench -p flood_watch`; `cargo run -p bench-report -- --game flood_watch --input <report>` | covered |
