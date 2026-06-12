# Frontier Control Rule Coverage Matrix

Game ID: `frontier_control`

Rules version: `frontier-control-rules-v1`

Data version: `1`

Engine version: `engine-core-0.1.0`

Prepared by: `Codex`

Last updated: 2026-06-11

## Purpose

This matrix maps every stable rule ID in `RULES.md` to the Rust implementation,
tests, fixtures, traces, simulations, and benchmark evidence that cover it.
Rust tests and native tools are the rule authority; browser smoke presents
Rust/WASM output only.

## Rule Coverage Matrix

| Rule ID | Summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `FC-ACT-001` | Prospectors legal action tree. | `src/actions.rs`, `src/rules.rs` | `tests/rules.rs`, `tests/property.rs`, golden traces `early-end-turn`, `stake-and-dismantle`, `muster-and-reinforce-caps` | covered | Legal leaves come from Rust only. |
| `FC-ACT-002` | Prospector march generation and validation. | `src/actions.rs`, `src/rules.rs` | `tests/rules.rs`, `tests/property.rs`, golden traces `clash-crew-into-guards`, `non-adjacent-move-diagnostic` | covered | Includes adjacency and clash checks. |
| `FC-ACT-003` | Prospector stake generation and validation. | `src/actions.rs`, `src/rules.rs` | `tests/rules.rs`, golden traces `stake-and-dismantle`, `stake-on-guarded-site-diagnostic`, `supply-cut-scores-zero` | covered | Stake preconditions remain Rust-owned. |
| `FC-ACT-004` | Prospector muster. | `src/actions.rs`, `src/rules.rs` | `tests/rules.rs`, golden trace `muster-and-reinforce-caps` | covered | Unit cap is enforced in Rust. |
| `FC-ACT-005` | Garrison legal action tree. | `src/actions.rs`, `src/rules.rs` | `tests/rules.rs`, `tests/property.rs`, golden traces `muster-and-reinforce-caps`, `stake-and-dismantle` | covered | Legal leaves come from Rust only. |
| `FC-ACT-006` | Garrison patrol generation and validation. | `src/actions.rs`, `src/rules.rs` | `tests/rules.rs`, golden traces `clash-guard-into-crews`, `non-adjacent-move-diagnostic` | covered | Includes adjacency and clash checks. |
| `FC-ACT-007` | Garrison reinforce. | `src/actions.rs`, `src/rules.rs` | `tests/rules.rs`, golden trace `muster-and-reinforce-caps` | covered | Held-fort and cap conditions are validated. |
| `FC-ACT-008` | Garrison dismantle. | `src/actions.rs`, `src/rules.rs` | `tests/rules.rs`, golden trace `stake-and-dismantle` | covered | Removes stake without unit mutation. |
| `FC-ACT-009` | End turn is legal during action phases. | `src/actions.rs`, `src/rules.rs` | `tests/rules.rs`, golden traces `early-end-turn`, `budget-exhaustion-auto-end` | covered | Prevents stalls. |
| `FC-ACT-010` | Non-active seats receive no gameplay leaves. | `src/actions.rs`, `src/visibility.rs` | `tests/rules.rs`, `tests/visibility.rs`, golden trace `wrong-faction-diagnostic` | covered | Waiting metadata is public. |
| `FC-ACT-011` | Terminal states expose no gameplay leaves. | `src/actions.rs`, `src/rules.rs`, `src/visibility.rs` | `tests/rules.rs`, `tests/property.rs`, golden traces `standard-garrison-win`, `tie-garrison-tiebreak` | covered | Terminal no-action state is enforced. |
| `FC-AMB-001` | Graph helpers stay local. | `src/topology.rs`, `docs/MECHANICS.md` | `tests/rules.rs`, `tests/property.rs`, `scripts/boundary-check.sh` | covered | First graph-map use remains in the game crate. |
| `FC-AMB-002` | TypeScript does not compute supply. | `src/rules.rs`, `src/visibility.rs`, WASM projection | `tests/visibility.rs`, `tests/replay.rs`, golden trace `supply-cut-scores-zero` | covered | Browser receives supplied or cut status from Rust. |
| `FC-AMB-003` | Clashes resolve immediately. | `src/rules.rs` | `tests/rules.rs`, golden traces `clash-crew-into-guards`, `clash-guard-into-crews` | covered | No response window exists. |
| `FC-AMB-004` | Ties are Garrison wins. | `src/rules.rs`, `src/visibility.rs` | `tests/rules.rs`, golden trace `tie-garrison-tiebreak` | covered | Terminal rationale cites the tiebreak. |
| `FC-AMB-005` | Map data cannot encode behavior. | `src/manifest.rs`, `src/variants.rs`, `src/setup.rs` | `tests/serialization.rs`, `fixture-check --game frontier_control`, `data/fixtures/*.json` | covered | Unknown and behavior-looking content fails closed. |
| `FC-BOT-001` | Random legal bot uses legal leaves. | `src/bots.rs` | `tests/bots.rs`, `tests/property.rs`, golden trace `bot-vs-bot-full-game`, `simulate --game frontier_control` | covered | Bot decisions validate through the normal command path. |
| `FC-BOT-002` | Garrison Level 1 policy uses public facts. | `src/bots.rs` | `tests/bots.rs`, `docs/BOT-STRATEGY-EVIDENCE-PACK.md`, `simulate --game frontier_control` | covered | No search, MCTS, ML, RL, or hidden input. |
| `FC-BOT-003` | Prospector Level 1 policy uses public facts. | `src/bots.rs` | `tests/bots.rs`, `docs/BOT-STRATEGY-EVIDENCE-PACK.md`, `simulate --game frontier_control` | covered | No direct state mutation or illegal fallback. |
| `FC-COMP-001` | Two public seats. | `src/state.rs`, `src/setup.rs` | `tests/rules.rs`, `tests/serialization.rs`, fixtures `frontier_control_standard`, `frontier_control_highlands` | covered | Seat IDs are deterministic. |
| `FC-COMP-002` | Public factions. | `src/state.rs`, `src/visibility.rs` | `tests/visibility.rs`, `tests/bots.rs`, fixtures | covered | Faction nouns stay in the game crate. |
| `FC-COMP-003` | Public sites. | `src/topology.rs`, `src/variants.rs`, `src/visibility.rs` | `tests/serialization.rs`, `tests/visibility.rs`, fixtures | covered | Site content is typed data. |
| `FC-COMP-004` | Public trails. | `src/topology.rs`, `src/variants.rs` | `tests/rules.rs`, `tests/serialization.rs`, golden trace `non-adjacent-move-diagnostic` | covered | Trail adjacency is Rust behavior over typed content. |
| `FC-COMP-005` | Guards. | `src/state.rs`, `src/rules.rs`, `src/visibility.rs` | `tests/rules.rs`, golden traces `clash-crew-into-guards`, `clash-guard-into-crews` | covered | Guard state is public. |
| `FC-COMP-006` | Crews. | `src/state.rs`, `src/rules.rs`, `src/visibility.rs` | `tests/rules.rs`, golden traces `clash-crew-into-guards`, `muster-and-reinforce-caps` | covered | Crew state is public. |
| `FC-COMP-007` | Forts. | `src/variants.rs`, `src/rules.rs` | `tests/rules.rs`, golden trace `round-scoring-breakdown`, fixtures | covered | Fort flags are content; scoring is Rust. |
| `FC-COMP-008` | Stakes. | `src/state.rs`, `src/rules.rs`, `src/visibility.rs` | `tests/rules.rs`, golden traces `stake-and-dismantle`, `supply-cut-scores-zero` | covered | Stake state is public. |
| `FC-COMP-009` | Base Camp. | `src/variants.rs`, `src/topology.rs`, `src/rules.rs` | `tests/rules.rs`, fixtures, golden traces `muster-and-reinforce-caps`, `supply-cut-scores-zero` | covered | Base Camp is public typed content. |
| `FC-COMP-010` | Supply path. | `src/topology.rs`, `src/rules.rs`, `src/visibility.rs` | `tests/rules.rs`, `tests/visibility.rs`, golden trace `supply-cut-scores-zero` | covered | Supply is computed by Rust and projected. |
| `FC-COMP-011` | Action budget. | `src/state.rs`, `src/rules.rs`, `src/visibility.rs` | `tests/rules.rs`, golden traces `budget-exhaustion-auto-end`, `early-end-turn` | covered | Budget is public and deterministic. |
| `FC-COMP-012` | Comparable score track. | `src/state.rs`, `src/rules.rs`, `src/visibility.rs` | `tests/rules.rs`, golden traces `round-scoring-breakdown`, `standard-garrison-win`, `standard-prospector-win` | covered | Both factions use one numeric track. |
| `FC-CTRL-001` | Units move along adjacent trails only. | `src/topology.rs`, `src/rules.rs` | `tests/rules.rs`, golden trace `non-adjacent-move-diagnostic` | covered | Non-adjacent moves reject without mutation. |
| `FC-CTRL-002` | Crew entering guards removes one guard and the crew. | `src/rules.rs` | `tests/rules.rs`, golden trace `clash-crew-into-guards` | covered | Effect order is pinned. |
| `FC-CTRL-003` | Guard entering crews removes one crew and survives. | `src/rules.rs` | `tests/rules.rs`, golden trace `clash-guard-into-crews` | covered | Effect order is pinned. |
| `FC-CTRL-004` | Mixed-faction occupancy is transient. | `src/rules.rs`, `src/visibility.rs` | `tests/rules.rs`, `tests/visibility.rs`, clash traces | covered | Projected state is settled and public. |
| `FC-CTRL-005` | Stakes persist until dismantled. | `src/rules.rs`, `src/visibility.rs` | `tests/rules.rs`, golden traces `stake-and-dismantle`, `supply-cut-scores-zero` | covered | Cut stakes score zero but remain. |
| `FC-DEV-001` | Original frontier presentation. | docs and `data/manifest.toml` | `docs/SOURCES.md`, `docs/HOW-TO-PLAY.md`, fixture metadata | covered | Public prose is original Rulepath prose. |
| `FC-DEV-002` | One score track with different scoring formulas. | `src/rules.rs`, `src/visibility.rs` | `tests/rules.rs`, terminal and scoring traces | covered | Asymmetric victory conditions remain out of scope. |
| `FC-DEV-003` | No cards, dice, event decks, or hidden objectives. | `src/setup.rs`, `src/rules.rs`, `src/visibility.rs` | `tests/visibility.rs`, `tests/serialization.rs`, trace not-applicable fields | covered | No game-rule randomness or hidden state exists. |
| `FC-OOS-001` | More-than-two-faction variants. | not implemented | `setup_match` seat-count validation, `tests/rules.rs` | unsupported | Gate 13 supports exactly two factions. |
| `FC-OOS-002` | Hidden units or private objectives. | not implemented | `tests/visibility.rs`, trace `hidden_information_redaction` not-applicable markers | unsupported | Perfect information is the chosen gate scope. |
| `FC-OOS-003` | Game-rule randomness. | not implemented | `tests/serialization.rs`, trace `stochastic_game_rule_events` not-applicable markers | unsupported | Bot RNG is outside game-rule randomness. |
| `FC-OOS-004` | Reaction windows. | not implemented | `tests/rules.rs`, clash traces | unsupported | Clashes resolve immediately. |
| `FC-OOS-005` | Generic graph/control/faction helpers. | not implemented | `docs/MECHANICS.md`, `scripts/boundary-check.sh` | unsupported | First use stays local to `games/frontier_control`. |
| `FC-OOS-006` | Hosted multiplayer or persistence. | not implemented | local-only command log and replay support tests | unsupported | V1 and v2 are local-first. |
| `FC-OOS-007` | MCTS, Monte Carlo, ML, RL, LLM, or hidden sampling bots. | not implemented | `tests/bots.rs`, `docs/BOT-STRATEGY-EVIDENCE-PACK.md` | unsupported | Public bot law forbids these approaches. |
| `FC-RESTRICT-001` | Unknown or non-seat actor rejects. | `src/actions.rs` | `tests/rules.rs`, diagnostic tests | covered | No mutation on rejection. |
| `FC-RESTRICT-002` | Wrong seat rejects. | `src/actions.rs` | `tests/rules.rs`, golden trace `wrong-faction-diagnostic` | covered | Diagnostic uses public active-faction facts. |
| `FC-RESTRICT-003` | Malformed or unavailable action rejects. | `src/actions.rs`, `src/rules.rs` | `tests/rules.rs`, diagnostic traces | covered | No mutation on rejection. |
| `FC-RESTRICT-004` | Stale command rejects. | `src/actions.rs` | `tests/replay.rs`, `tests/rules.rs` | covered | Freshness token protects replay determinism. |
| `FC-RESTRICT-005` | Unknown or non-adjacent move rejects. | `src/topology.rs`, `src/rules.rs` | `tests/rules.rs`, golden trace `non-adjacent-move-diagnostic` | covered | TypeScript does not compute adjacency. |
| `FC-RESTRICT-006` | Missing unit or cap violation rejects. | `src/rules.rs` | `tests/rules.rs`, golden trace `muster-and-reinforce-caps` | covered | Diagnostics are public occupancy facts. |
| `FC-RESTRICT-007` | Stake, muster, reinforce, or dismantle precondition rejects. | `src/rules.rs` | `tests/rules.rs`, golden trace `stake-on-guarded-site-diagnostic` | covered | Action-specific preconditions are Rust-owned. |
| `FC-RESTRICT-008` | Terminal gameplay action rejects. | `src/actions.rs`, `src/rules.rs` | `tests/rules.rs`, terminal traces | covered | Terminal result is final. |
| `FC-RNG-001` | No game-rule randomness. | `src/setup.rs`, `src/rules.rs` | `tests/replay.rs`, `tests/serialization.rs`, trace not-applicable markers | covered | Bot tie-break RNG is not game-rule randomness. |
| `FC-RNG-002` | Deterministic typed map setup. | `src/variants.rs`, `src/setup.rs` | `tests/serialization.rs`, fixtures, `fixture-check --game frontier_control` | covered | Variant content is stable typed data. |
| `FC-RNG-003` | Round scoring is command-driven. | `src/rules.rs` | `tests/rules.rs`, golden traces `budget-exhaustion-auto-end`, `round-scoring-breakdown` | covered | No timer or scoring actor exists. |
| `FC-RNG-004` | Stable serialization order. | `src/replay_support.rs`, `src/visibility.rs`, serde derives | `tests/serialization.rs`, `tests/replay.rs`, golden traces | covered | Ordering covers state, effects, views, and replays. |
| `FC-SCORE-ACTION-BUDGET` | Budget decreases on action and refills on cleanup. | `src/rules.rs`, `src/visibility.rs` | `tests/rules.rs`, golden traces `budget-exhaustion-auto-end`, `early-end-turn` | covered | End turn forfeits remaining budget. |
| `FC-SCORE-COMPARABLE-TRACK` | Both factions score on one comparable track. | `src/rules.rs`, `src/visibility.rs` | `tests/rules.rs`, terminal traces | covered | Higher final score wins. |
| `FC-SCORE-GARRISON-FORT` | Garrison scores held forts. | `src/rules.rs` | `tests/rules.rs`, golden traces `round-scoring-breakdown`, `standard-garrison-win` | covered | Fort with crews scores zero. |
| `FC-SCORE-PROSPECTOR-SUPPLY` | Prospectors score supplied stakes. | `src/topology.rs`, `src/rules.rs` | `tests/rules.rs`, golden traces `supply-cut-scores-zero`, `standard-prospector-win` | covered | Cut stakes score zero and remain. |
| `FC-SCORE-STAKE-VALUE` | Stake values are public content. | `src/variants.rs`, `src/rules.rs` | `tests/serialization.rs`, fixtures, scoring traces | covered | Scoring formula remains Rust behavior. |
| `FC-SETUP-001` | Exactly two seats and faction assignment. | `src/setup.rs`, `src/state.rs` | `tests/rules.rs`, `tests/serialization.rs`, fixtures | covered | Future seat counts are out of scope. |
| `FC-SETUP-002` | Load typed map constants. | `src/manifest.rs`, `src/variants.rs`, `src/setup.rs` | `tests/serialization.rs`, fixtures, `fixture-check --game frontier_control` | covered | Content is typed and behavior-free. |
| `FC-SETUP-003` | Validate map data. | `src/topology.rs`, `src/setup.rs`, `src/variants.rs` | `tests/rules.rs`, `tests/serialization.rs`, fixture validation | covered | Invalid content fails closed. |
| `FC-SETUP-004` | Place starting guards and crews. | `src/setup.rs`, `src/state.rs` | `tests/rules.rs`, `tests/serialization.rs`, fixtures | covered | No random setup. |
| `FC-SETUP-005` | Initialize round, active faction, budget, scores, terminal, freshness. | `src/setup.rs`, `src/state.rs` | `tests/rules.rs`, `tests/serialization.rs`, golden trace `highlands-setup` | covered | Same setup reproduces the match. |
| `FC-TERM-GARRISON-TIEBREAK` | Garrison wins tied final scores. | `src/rules.rs`, `src/visibility.rs` | `tests/rules.rs`, golden trace `tie-garrison-tiebreak` | covered | Terminal rationale carries the tiebreak. |
| `FC-TERM-NO-ACTIONS` | Terminal state has no normal actions. | `src/actions.rs`, `src/rules.rs` | `tests/rules.rs`, terminal traces | covered | No further gameplay command advances the match. |
| `FC-TERM-SCORE-COMPARE` | Final scheduled scoring round decides winner by score. | `src/rules.rs`, `src/visibility.rs` | `tests/rules.rs`, golden traces `standard-garrison-win`, `standard-prospector-win`, `tie-garrison-tiebreak` | covered | Final totals and winner are projected by Rust. |
| `FC-TURN-001` | Prospector action phase. | `src/state.rs`, `src/actions.rs`, `src/rules.rs` | `tests/rules.rs`, golden traces `early-end-turn`, `clash-crew-into-guards` | covered | Prospector seat acts through legal leaves. |
| `FC-TURN-002` | Garrison action phase. | `src/state.rs`, `src/actions.rs`, `src/rules.rs` | `tests/rules.rs`, golden traces `stake-and-dismantle`, `muster-and-reinforce-caps` | covered | Garrison seat acts through legal leaves. |
| `FC-TURN-003` | Waiting state. | `src/actions.rs`, `src/visibility.rs` | `tests/visibility.rs`, golden trace `wrong-faction-diagnostic` | covered | Non-active metadata is viewer-safe. |
| `FC-TURN-004` | Budget exhaustion ends the turn. | `src/rules.rs` | `tests/rules.rs`, golden trace `budget-exhaustion-auto-end` | covered | Final budget spend advances automatically. |
| `FC-TURN-005` | Explicit end turn. | `src/rules.rs` | `tests/rules.rs`, golden trace `early-end-turn` | covered | Remaining budget can be forfeited. |
| `FC-TURN-006` | Round scoring after Garrison turn. | `src/rules.rs` | `tests/rules.rs`, golden trace `round-scoring-breakdown` | covered | Scoring effects are public. |
| `FC-TURN-007` | Non-terminal cleanup. | `src/rules.rs`, `src/state.rs` | `tests/rules.rs`, `tests/property.rs`, golden trace `budget-exhaustion-auto-end` | covered | Budget and active faction reset deterministically. |
| `FC-TURN-008` | Terminal state. | `src/rules.rs`, `src/actions.rs`, `src/visibility.rs` | `tests/rules.rs`, terminal traces | covered | Terminal projection has outcome and no normal actions. |
| `FC-VAR-001` | Standard variant. | `src/variants.rs`, `data/variants.toml` | `tests/serialization.rs`, fixture `frontier_control_standard`, standard traces | covered | Default public variant. |
| `FC-VAR-002` | Highlands variant. | `src/variants.rs`, `data/variants.toml` | `tests/serialization.rs`, fixture `frontier_control_highlands`, golden trace `highlands-setup` | covered | Same Rust rules with different typed content. |
| `FC-VAR-003` | Variant data boundary. | `src/variants.rs`, `src/setup.rs` | `tests/serialization.rs`, fixtures, `fixture-check --game frontier_control` | covered | Variant data cannot encode behavior. |
| `FC-VIS-001` | Public state facts. | `src/visibility.rs`, `src/effects.rs` | `tests/visibility.rs`, `tests/replay.rs`, golden traces | covered | Projection is viewer-safe. |
| `FC-VIS-002` | Public variant constants and labels. | `src/visibility.rs`, `src/variants.rs` | `tests/visibility.rs`, fixtures, `HOW-TO-PLAY.md` | covered | Labels are content, not behavior. |
| `FC-VIS-003` | Bot rationale and rankings use public data only. | `src/bots.rs`, `src/visibility.rs` | `tests/bots.rs`, `docs/BOT-STRATEGY-EVIDENCE-PACK.md` | covered | No hidden state exists. |
| `FC-VIS-004` | Hidden information is not applicable. | `src/visibility.rs`, trace metadata | `tests/visibility.rs`, trace `hidden_information_redaction` markers | covered | Perfect-information game; no hidden units or objectives. |

## Simulation and Native Tool Evidence

| Tool or suite | Coverage role | Current command |
|---|---|---|
| Native simulation | Bot legality, terminal progress, per-faction outcome metrics, action cap safety | `cargo run -p simulate -- --game frontier_control --games 1000` |
| Replay check | Golden trace catalog registration and structural trace contract | `cargo run -p replay-check -- --game frontier_control --all` |
| Fixture check | Static-data boundary, trace schema fields, manifest and variant metadata | `cargo run -p fixture-check -- --game frontier_control` |
| Rule coverage | Exact `RULES.md` to `RULE-COVERAGE.md` mapping | `cargo run -p rule-coverage -- --game frontier_control` |
| Boundary check | Keeps `faction` and `territory` nouns out of `engine-core` | `bash scripts/boundary-check.sh` |

## Benchmark Relevance

| Benchmark | Rule IDs/mechanics relevant | Status |
|---|---|---|
| Legal actions | `FC-ACT-*`, `FC-RESTRICT-*`, `FC-TURN-*` | Covered by `BENCHMARKS.md` thresholds. |
| Apply/action resolution | `FC-CTRL-*`, `FC-SCORE-*`, `FC-TERM-*` | Covered by `BENCHMARKS.md` thresholds. |
| View/replay/serialization | `FC-VIS-*`, `FC-RNG-*` | Covered by `BENCHMARKS.md` thresholds. |
| Bot decisions and simulation | `FC-BOT-*`, `FC-TURN-*` | Covered by `BENCHMARKS.md` thresholds. |
