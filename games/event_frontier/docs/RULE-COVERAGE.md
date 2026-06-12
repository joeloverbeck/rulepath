# Event Frontier Rule Coverage Matrix

Game ID: `event_frontier`

Rules version: `event-frontier-rules-v1`

Data/manifest version: `1`

Engine version: `engine-core-0.1.0`

Prepared by: `Codex`

Last updated: 2026-06-12

## Purpose

This matrix maps every stable rule ID in `RULES.md` to Rust-owned
implementation and verification evidence. UI smoke proves presentation only;
Rust tests, golden traces, fixture checks, replay checks, simulation, and
benchmarks are the rule evidence.

## Rule Coverage Matrix

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `EF-ACT-001` | First eligible action menu. | `actions.rs`, `rules.rs` | `tests/rules.rs`, golden traces, `rule-coverage`, `simulate` | covered | Rust generates first-choice action trees. |
| `EF-ACT-002` | Second menu after first event. | `actions.rs`, `rules.rs` | `tests/rules.rs`, golden traces, `replay-check` | covered | Same-card second event is excluded. |
| `EF-ACT-003` | Second menu after first operation. | `actions.rs`, `rules.rs` | `limited-op-after-full-op.trace.json`, `tests/rules.rs` | covered | Limited operation path is trace-pinned. |
| `EF-ACT-004` | Second menu after first pass. | `actions.rs`, `rules.rs` | `pass-keeps-eligibility.trace.json`, `double-pass-discards-card.trace.json` | covered | Pass flow stays Rust-owned. |
| `EF-ACT-005` | Charter operation tree. | `actions.rs` | `op-full-multi-site.trace.json`, `tests/rules.rs`, benchmarks | covered | Survey, fortify, and writ are typed operation variants. |
| `EF-ACT-006` | Freeholder operation tree. | `actions.rs` | `tests/rules.rs`, `property.rs`, `simulate` | covered | Trek, cache, and rally are typed operation variants. |
| `EF-ACT-007` | Non-active viewer has no gameplay choices. | `actions.rs`, `visibility.rs` | `tests/visibility.rs`, golden traces | covered | Waiting metadata is public only. |
| `EF-ACT-008` | Automated and terminal phases expose no normal action. | `actions.rs`, `rules.rs` | `tests/rules.rs`, terminal golden traces | covered | Reckoning and terminal states reject gameplay actions. |
| `EF-AMB-001` | Card data is not behavior. | `cards.rs`, static parsers | `serialization.rs`, `fixture-check` | covered | Behavior-looking fields are rejected. |
| `EF-AMB-002` | Reckoning has no command actor. | `rules.rs`, replay support | `replay.rs`, `reckoning-scoring-breakdown.trace.json` | covered | Replay derives automation from state. |
| `EF-AMB-003` | Current and next card are public. | `visibility.rs` | `visibility.rs` tests, WASM smoke | covered | Deeper order remains hidden. |
| `EF-AMB-004` | Eligibility is not a reaction window. | `actions.rs`, `rules.rs` | `tests/rules.rs`, atlas ledger | covered | No pending-response state is introduced. |
| `EF-AMB-005` | Operations are compound commands, not budgets. | `actions.rs`, `rules.rs` | `tests/rules.rs`, atlas ledger | covered | One validated command resolves the operation. |
| `EF-AMB-006` | Terminal does not reveal deck tail. | `visibility.rs`, `replay_support.rs` | `visibility.rs`, `replay-export-import-no-deck-leak.trace.json` | covered | Public exports stay redacted. |
| `EF-BOT-001` | Random bot uses legal leaves. | `bots.rs` | `tests/bots.rs`, `simulate` | covered | Bot validates through normal command path. |
| `EF-BOT-002` | Charter Level 1 uses public inputs only. | `bots.rs`, `visibility.rs` | `tests/bots.rs`, `BOT-STRATEGY-EVIDENCE-PACK.md` | covered | No hidden deck order enters rationale. |
| `EF-BOT-003` | Freeholder Level 1 uses public inputs only. | `bots.rs`, `visibility.rs` | `tests/bots.rs`, `BOT-STRATEGY-EVIDENCE-PACK.md` | covered | No sampling or search bot is used. |
| `EF-COMP-001` | Two public seats. | `state.rs`, `setup.rs` | `tests/rules.rs`, fixtures | covered | Seat IDs are stable. |
| `EF-COMP-002` | Public factions. | `ids.rs`, `state.rs` | `tests/rules.rs`, docs | covered | Faction nouns stay game-local. |
| `EF-COMP-003` | Six graph sites. | `ids.rs`, `setup.rs` | fixtures, `tests/rules.rs` | covered | No rectangular board helper is introduced. |
| `EF-COMP-004` | Public trails. | `setup.rs`, variants | fixtures, `fixture-check` | covered | Graph validation is setup-local. |
| `EF-COMP-005` | Charter agents. | `state.rs`, `rules.rs` | `tests/rules.rs`, golden traces | covered | Cap and presence scoring are tested. |
| `EF-COMP-006` | Charter depots. | `state.rs`, `rules.rs` | `tests/rules.rs`, golden traces | covered | Depot build and presence effects are tested. |
| `EF-COMP-007` | Freeholder settlers. | `state.rs`, `rules.rs` | `tests/rules.rs`, golden traces | covered | Movement and presence are tested. |
| `EF-COMP-008` | Freeholder caches. | `state.rs`, `rules.rs` | `standard-freeholder-cache-win.trace.json`, `tests/rules.rs` | covered | Cache threshold victory is trace-pinned. |
| `EF-COMP-009` | Funds pool. | `state.rs`, `rules.rs` | `tests/rules.rs`, `property.rs` | covered | Resource cap invariants are tested. |
| `EF-COMP-010` | Provisions pool. | `state.rs`, `rules.rs` | `tests/rules.rs`, `property.rs` | covered | Resource cap invariants are tested. |
| `EF-COMP-011` | Hidden event deck order. | `cards.rs`, `setup.rs`, `visibility.rs` | `visibility.rs`, `replay.rs`, no-leak trace | covered | Only current and next cards are public. |
| `EF-COMP-012` | Typed event cards. | `cards.rs`, `rules.rs` | `serialization.rs`, `tests/rules.rs` | covered | Closed card ID enum drives behavior. |
| `EF-COMP-013` | Public edicts. | `cards.rs`, `rules.rs` | edict golden traces, `tests/rules.rs` | covered | Edicts are typed modifiers. |
| `EF-COMP-014` | Reckoning card. | `cards.rs`, `rules.rs` | reckoning golden traces, `replay.rs` | covered | Reckoning placement and pipeline are tested. |
| `EF-COMP-015` | Eligibility state. | `state.rs`, `actions.rs`, `rules.rs` | eligibility traces, `property.rs` | covered | Eligibility is public and deterministic. |
| `EF-COMP-016` | Score track. | `state.rs`, `rules.rs` | fallback trace, `tests/rules.rs` | covered | Scores decide only final fallback. |
| `EF-DEV-001` | Original public product identity. | docs, static data | `SOURCES.md`, doc-link check | covered | No private source content is used. |
| `EF-DEV-002` | Card text is not rules. | `cards.rs`, data parsers | `serialization.rs`, `fixture-check` | covered | Data carries identity and parameters only. |
| `EF-DEV-003` | No combat, hidden objectives, or extra factions. | docs, state model | `RULES.md`, `tests/rules.rs` | covered | Scope is enforced by types and setup. |
| `EF-DEV-004` | Remaining deck is never revealed. | `visibility.rs`, `replay_support.rs` | no-leak tests and trace | covered | Terminal projection remains redacted. |
| `EF-EDICT-001` | Edict activation. | `rules.rs`, `cards.rs` | `edict-activation-and-expiry.trace.json` | covered | Activation emits public effect. |
| `EF-EDICT-002` | Toll Roads cost modifier. | `actions.rs`, `rules.rs` | `tests/rules.rs` | covered | Affordability uses Rust-computed cost. |
| `EF-EDICT-003` | Survey Ban contested-site block. | `actions.rs`, `rules.rs` | `edict-blocks-action-diagnostic.trace.json` | covered | Diagnostic is viewer-safe. |
| `EF-EDICT-004` | Requisition depot cost modifier. | `actions.rs`, `rules.rs` | `tests/rules.rs` | covered | Applies to Charter depot operations only. |
| `EF-EDICT-005` | Long Season operation bound modifier. | `actions.rs` | `tests/rules.rs`, benchmark branch case | covered | Limited operations remain unchanged. |
| `EF-EDICT-006` | Stable edict order. | `rules.rs` | `tests/rules.rs`, `property.rs` | covered | Modifier order is deterministic. |
| `EF-EDICT-007` | Edict expiry at Reckoning. | `rules.rs` | `edict-activation-and-expiry.trace.json` | covered | Expiry effects precede next reveal. |
| `EF-END-001` | Charter instant victory. | `rules.rs` | `standard-charter-instant-win.trace.json` | covered | Checked before scoring. |
| `EF-END-002` | Freeholder cache victory. | `rules.rs` | `standard-freeholder-cache-win.trace.json` | covered | Threshold comes from variant data. |
| `EF-END-003` | Both-met Freeholder rule. | `rules.rs` | `tests/rules.rs` | covered | Tie rule is deterministic. |
| `EF-END-004` | Third Reckoning fallback. | `rules.rs` | `final-reckoning-fallback.trace.json` | covered | Freeholders win tied fallback totals. |
| `EF-END-005` | Terminal no-action state. | `rules.rs`, `actions.rs` | terminal traces, `tests/rules.rs` | covered | Terminal does not reveal hidden order. |
| `EF-EVENT-001` | Ordinary event effects. | `cards.rs`, `rules.rs` | `event-choice-resolves-card.trace.json`, `tests/rules.rs` | covered | Effects are typed Rust matches. |
| `EF-EVENT-002` | Event determinism. | `rules.rs`, replay support | `replay.rs`, golden traces | covered | No post-setup random sampling. |
| `EF-OOS-001` | More seats and team variants excluded. | docs, setup validation | `RULES.md`, `setup.rs` | covered | Two seats are the gate scope. |
| `EF-OOS-002` | Private hands/objectives excluded. | docs, state model | `visibility.rs`, no-leak tests | covered | Only deck tail is hidden. |
| `EF-OOS-003` | Reaction windows excluded. | docs, state model | atlas ledger, `tests/rules.rs` | covered | No response queue exists. |
| `EF-OOS-004` | Multi-action budgets excluded. | docs, actions model | atlas ledger, `tests/rules.rs` | covered | Compound operation is one command. |
| `EF-OOS-005` | Mid-game randomness excluded. | `setup.rs`, `rules.rs` | `replay.rs`, `property.rs` | covered | Setup shuffle is the only randomness. |
| `EF-OOS-006` | Generic helpers excluded. | docs, boundary check | `boundary-check.sh`, atlas ledger | covered | Nouns stay out of engine-core. |
| `EF-OOS-007` | Hosted services excluded. | docs, workspace shape | `RULES.md`, web smoke | covered | Local-first shell only. |
| `EF-OOS-008` | Search/ML bots excluded. | `bots.rs`, docs | `tests/bots.rs`, `AI.md` | covered | Scripted and random legal bots only. |
| `EF-OP-001` | Operation cost. | `actions.rs`, `rules.rs` | `tests/rules.rs`, `property.rs` | covered | Costs are paid before effects. |
| `EF-OP-002` | Full operation site count. | `actions.rs` | `op-full-multi-site.trace.json` | covered | Ops value and edicts bound choices. |
| `EF-OP-003` | Limited operation site count. | `actions.rs` | `limited-op-after-full-op.trace.json` | covered | Exactly one site is required. |
| `EF-OP-004` | Survey operation. | `actions.rs`, `rules.rs` | `tests/rules.rs`, golden traces | covered | Adjacency and cap checks are Rust-owned. |
| `EF-OP-005` | Fortify operation. | `actions.rs`, `rules.rs` | `tests/rules.rs` | covered | Depot cap and agent requirement are tested. |
| `EF-OP-006` | Writ operation. | `actions.rs`, `rules.rs` | `tests/rules.rs` | covered | Cache removal and fund gain are effect-visible. |
| `EF-OP-007` | Trek operation. | `actions.rs`, `rules.rs` | `tests/rules.rs`, `property.rs` | covered | Trail and cap validation are tested. |
| `EF-OP-008` | Cache operation. | `actions.rs`, `rules.rs` | `tests/rules.rs`, cache victory trace | covered | Depot absence and cap are checked. |
| `EF-OP-009` | Rally operation. | `actions.rs`, `rules.rs` | `tests/rules.rs` | covered | Landing/cache eligibility is checked. |
| `EF-RESTRICT-001` | Unknown actor rejected. | `actions.rs` | `tests/rules.rs`, diagnostics traces | covered | Rejection does not mutate state. |
| `EF-RESTRICT-002` | Wrong or ineligible faction rejected. | `actions.rs` | `ineligible-faction-diagnostic.trace.json` | covered | Diagnostic cites public eligibility only. |
| `EF-RESTRICT-003` | Malformed or stale path rejected. | `actions.rs` | `tests/rules.rs`, golden traces | covered | State hashes remain stable. |
| `EF-RESTRICT-004` | Menu constraint rejected. | `actions.rs` | `tests/rules.rs` | covered | First choice determines second menu. |
| `EF-RESTRICT-005` | Invalid operation selection rejected. | `actions.rs` | `tests/rules.rs` | covered | Bounds and site preconditions are Rust-owned. |
| `EF-RESTRICT-006` | Unaffordable operation rejected. | `actions.rs` | `tests/rules.rs` | covered | Diagnostic is public-resource only. |
| `EF-RESTRICT-007` | Edict restrictions enforced. | `actions.rs`, `rules.rs` | `edict-blocks-action-diagnostic.trace.json` | covered | Edict effects stay typed. |
| `EF-RESTRICT-008` | Automated/terminal actions rejected. | `actions.rs`, `rules.rs` | `tests/rules.rs` | covered | No synthetic actor is accepted. |
| `EF-RNG-001` | Setup shuffle deterministic. | `setup.rs`, `cards.rs` | `replay.rs`, setup traces | covered | Reckoning-never-first is tested. |
| `EF-RNG-002` | Card/reckoning replay deterministic. | `rules.rs`, replay support | `replay.rs`, golden traces | covered | Automation needs no extra actor. |
| `EF-RNG-003` | Public replay redacts seed/order. | `replay_support.rs` | `replay.rs`, no-leak trace | covered | Public export cannot reconstruct future cards. |
| `EF-RNG-004` | Stable serialization order. | `state.rs`, replay support | `serialization.rs`, golden traces | covered | Hash drift is caught by traces. |
| `EF-SCOPE-001` | Original two-faction public game. | crate docs, state model | docs, `fixture-check` | covered | Scope is recorded in rules and sources. |
| `EF-SCOPE-002` | Gate 14 mechanics proof. | crate modules | full Event Frontier test suite | covered | Event, eligibility, edict, scoring, replay, and bots are implemented. |
| `EF-SCOPE-003` | Explicit exclusions. | docs, data parsers | atlas ledger, boundary check | covered | Excluded systems have no implementation surface. |
| `EF-SCORE-001` | Public resource pools and cap. | `state.rs`, `rules.rs` | `tests/rules.rs`, `property.rs` | covered | Funds and provisions are capped. |
| `EF-SCORE-002` | Pass income and eligibility. | `rules.rs` | `pass-keeps-eligibility.trace.json` | covered | Double pass discard is separately traced. |
| `EF-SCORE-003` | Operation cost before consequences. | `rules.rs` | `tests/rules.rs`, effect traces | covered | Resource effects precede site consequences. |
| `EF-SCORE-004` | Reckoning site scoring. | `rules.rs` | `reckoning-scoring-breakdown.trace.json` | covered | Ties award no score. |
| `EF-SCORE-005` | Reckoning income. | `rules.rs` | `tests/rules.rs`, reckoning trace | covered | Income is skipped when terminal fires. |
| `EF-SCORE-006` | Final fallback score comparison. | `rules.rs` | `final-reckoning-fallback.trace.json` | covered | Freeholders win tied totals. |
| `EF-SETUP-001` | Two seats and faction assignment. | `setup.rs`, `state.rs` | setup traces, fixtures | covered | Variant faction order is typed. |
| `EF-SETUP-002` | Scenario constants loaded. | `variants.rs`, `setup.rs` | fixtures, `fixture-check` | covered | Behavior-looking fields reject. |
| `EF-SETUP-003` | Site/trail validation. | `setup.rs` | `tests/rules.rs`, fixtures | covered | Invalid content fails setup. |
| `EF-SETUP-004` | Epoch deck build. | `setup.rs`, `cards.rs` | `reckoning-never-first-in-epoch.trace.json`, `replay.rs` | covered | Authored data stores no shuffled order. |
| `EF-SETUP-005` | Initial public projection. | `setup.rs`, `visibility.rs` | setup traces, WASM smoke | covered | Current and next card are initialized. |
| `EF-TURN-001` | First eligible faction chosen. | `actions.rs`, `rules.rs` | `tests/rules.rs`, golden traces | covered | Printed faction is used when eligible. |
| `EF-TURN-002` | No eligible faction discards card. | `rules.rs` | `no-eligible-faction-discard.trace.json` | covered | Discard and reveal are effect-visible. |
| `EF-TURN-003` | First event then second menu. | `rules.rs`, `actions.rs` | `event-choice-resolves-card.trace.json` | covered | Event effects complete before second choice. |
| `EF-TURN-004` | First operation then constrained second menu. | `rules.rs`, `actions.rs` | `limited-op-after-full-op.trace.json` | covered | Limited second operation is trace-pinned. |
| `EF-TURN-005` | First pass preserves eligibility. | `rules.rs` | `pass-keeps-eligibility.trace.json` | covered | Resource gain is capped. |
| `EF-TURN-006` | Second faction choice resolves. | `rules.rs`, `actions.rs` | `tests/rules.rs`, golden traces | covered | Menu constraints are validated. |
| `EF-TURN-007` | Card cleanup updates eligibility. | `rules.rs` | `tests/rules.rs`, golden traces | covered | Used card moves to discard. |
| `EF-TURN-008` | Reckoning pipeline. | `rules.rs` | reckoning traces, `replay.rs` | covered | Victory check precedes scoring and reset. |
| `EF-TURN-009` | Terminal state. | `rules.rs`, `actions.rs` | terminal traces, `tests/rules.rs` | covered | No further gameplay actions are legal. |
| `EF-VAR-001` | Standard variant. | `variants.rs`, fixtures | standard fixture, simulation | covered | Standard scenario is default. |
| `EF-VAR-002` | Hard winter variant. | `variants.rs`, fixtures | `hard-winter-setup.trace.json`, `fixture-check` | covered | Same rules, pressure variant data. |
| `EF-VAR-003` | Land rush variant. | `variants.rs`, fixtures | `land-rush-setup.trace.json`, `fixture-check` | covered | Same rules, faster cache race. |
| `EF-VAR-004` | Scenario data has no behavior. | `variants.rs`, parsers | `serialization.rs`, `fixture-check` | covered | Unknown and behavior keys reject. |
| `EF-VIS-001` | Public facts visible to all. | `visibility.rs` | `tests/visibility.rs`, WASM smoke | covered | Seat and observer projections are equivalent. |
| `EF-VIS-002` | Undrawn deck order hidden. | `visibility.rs`, replay support | `tests/visibility.rs`, no-leak trace | covered | Hidden order is native-test only. |
| `EF-VIS-003` | Current and next card public. | `visibility.rs`, effects | `tests/visibility.rs`, WASM smoke | covered | Reveal timing is public. |
| `EF-VIS-004` | Discard history public. | `visibility.rs`, effects | golden traces, `tests/visibility.rs` | covered | Resolved cards remain visible. |
| `EF-VIS-005` | Bot rationale is viewer-safe. | `bots.rs`, `visibility.rs` | `tests/bots.rs`, evidence pack | covered | Rationale cites public facts only. |

## Golden Trace Catalog

| Trace file | Purpose | Rule IDs covered | Hidden-info coverage | Update policy |
|---|---|---|---|---|
| `games/event_frontier/tests/golden_traces/*.trace.json` | setup, terminal, event, operation, edict, Reckoning, diagnostics, bot, replay export | `EF-*` coverage rows above | `undrawn_deck_order` marker and `public_no_leak` assertion | update only with intentional behavior or format migration note |

## Simulation/Fuzz Coverage Summary

| Simulation/fuzz run | Seeds/count | Bots/policies | Rule IDs stressed | Metrics recorded | Status/notes |
|---|---:|---|---|---|---|
| `cargo run -p simulate -- --game event_frontier --games 1000` | 1,000 | Charter and Freeholder Level 1 | turn, action, operation, event, Reckoning, terminal, bot rules | per-faction wins, victory types, average cards, Reckoning scores, fallback/pass rate, throughput | covered |

## Benchmark Relevance Map

| Benchmark | Rule IDs/mechanics relevant | Why relevant | Current threshold/status |
|---|---|---|---|
| `setup_standard` | `EF-SETUP-*`, `EF-RNG-001` | deterministic setup and epoch shuffle | smoke floor in `benches/thresholds.json` |
| `legal_tree_first_choice`, `legal_tree_peak_op_branching` | `EF-ACT-*`, `EF-OP-*`, `EF-EDICT-*` | large legal tree generation | smoke floor in `benches/thresholds.json` |
| `apply_event`, `apply_op_multi_site`, `reckoning_pipeline` | `EF-EVENT-*`, `EF-OP-*`, `EF-TURN-*`, `EF-SCORE-*`, `EF-END-*` | transition throughput | smoke floor in `benches/thresholds.json` |
| `bot_l1_choice_charter`, `bot_l1_choice_freeholders`, `full_random_playout` | `EF-BOT-*`, full game flow | demo-coherent bot and playout proof | smoke floor in `benches/thresholds.json` |

## Rule-ID Migration Notes

| Old rule ID | New rule ID(s) | Reason | Coverage rows updated? | Traces/tests updated? | Date |
|---|---|---|---:|---:|---|
| none | not applicable | Initial Event Frontier rule ID set. | yes | yes | 2026-06-12 |
