# Token Bazaar Rule Coverage Matrix

Game ID: `token_bazaar`

Rules version: `token-bazaar-rules-v1`

Data/manifest version: `1`

Engine version: `0.1.0`

Prepared by: `Codex`

Last updated: 2026-06-08

## Purpose

This matrix maps every stable rule ID in `RULES.md` to implementation and evidence.
Rust tests, golden traces, replay checks, fixture checks, simulations,
serialization checks, visibility checks, bot tests, and benchmark smoke floors are
primary evidence. Browser smoke proves integration only.

## Rule Coverage Matrix

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `TB-COMP-001` | Two public seats. | `ids.rs`, `setup.rs`, `state.rs`. | `seats_and_slots_are_bounded`, `token_bazaar_standard.fixture.json`. | covered | Seat ids are stable and public. |
| `TB-COMP-002` | Three game-local resources. | `ids.rs`, `state.rs`, `actions.rs`. | `stable_id_strings_round_trip_in_canonical_order`, `standard_fixture_metadata_is_present`. | covered | Resource nouns stay in the game crate. |
| `TB-COMP-003` | Public supply. | `state.rs`, `rules.rs`, `visibility.rs`. | `public_view_exposes_required_board_fields`, `supply-exhaustion-diagnostic.trace.json`. | covered-by-trace | Supply is public and replay-visible. |
| `TB-COMP-004` | Public inventories. | `state.rs`, `rules.rs`, `visibility.rs`. | `observer_and_seat_views_match_for_public_game`, `deterministic_playout_conserves_resources_and_never_panics`. | covered | Inventories are included in public view hashes. |
| `TB-COMP-005` | Contracts have id, cost, and points. | `ids.rs`, `state.rs`, `setup.rs`. | `contract_specs_are_in_standard_queue_order`, `contract-fulfill-refill.trace.json`. | covered-by-trace | Contract specs are typed Rust constants. |
| `TB-COMP-006` | Three visible market slots. | `ids.rs`, `setup.rs`, `visibility.rs`. | `public_view_exposes_required_board_fields`, `empty-slot-diagnostic.trace.json`. | covered-by-trace | Empty slots remain visible. |
| `TB-COMP-007` | Fulfilled-contract lists are public. | `state.rs`, `rules.rs`, `visibility.rs`. | `apply_fulfill_scores_refills_and_advances`, `terminal-turn-cap.trace.json`. | covered-by-trace | Used by terminal tie-breaks. |
| `TB-SETUP-001` | Create exactly two seats. | `setup.rs`. | `setup_rejects_wrong_seat_count`, `token_bazaar_standard.fixture.json`. | covered | Setup fails closed for wrong seat counts. |
| `TB-SETUP-002` | Seat 0 starts. | `setup.rs`, `state.rs`. | `setup_is_deterministic_standard_public_state`, `shortest-normal.trace.json`. | covered-by-trace | Active seat is public. |
| `TB-SETUP-003` | Supply starts at 14 each. | `setup.rs`, `variants.rs`. | `constants_match_static_data`, `setup_is_deterministic_standard_public_state`. | covered | Manifest and variant constants agree. |
| `TB-SETUP-004` | Initial inventories, scores, and fulfilled lists. | `setup.rs`, `state.rs`. | `setup_is_deterministic_standard_public_state`, `state_snapshot_round_trips_with_stable_bytes`. | covered | Setup state serializes stably. |
| `TB-SETUP-005` | Standard ten-contract queue. | `state.rs`, `setup.rs`. | `contract_specs_are_in_standard_queue_order`, `token_bazaar_standard.fixture.json`. | covered | Queue order is deterministic. |
| `TB-SETUP-006` | Initial visible market and remaining queue. | `setup.rs`, `visibility.rs`. | `public_view_exposes_required_board_fields`, `contract-fulfill-refill.trace.json`. | covered-by-trace | Refill preserves slot identity. |
| `TB-TURN-001` | One nonterminal active turn. | `actions.rs`, `rules.rs`. | `validate_accepts_each_legal_family`, `shortest-normal.trace.json`. | covered-by-trace | Legal tree is actor-scoped. |
| `TB-TURN-002` | Applied actions emit effects and advance active seat. | `rules.rs`, `effects.rs`. | `apply_collect_updates_accounting_and_turn`, `exchange.trace.json`. | covered-by-trace | Turn advance is effect-visible. |
| `TB-TURN-003` | Eight turns per seat cap. | `rules.rs`, `state.rs`. | `turn_cap_terminal_uses_tie_breaks`, `terminal-turn-cap.trace.json`. | covered-by-trace | Terminal occurs after both seats reach cap. |
| `TB-TURN-004` | Terminal has no active gameplay. | `rules.rs`, `actions.rs`, `visibility.rs`. | `terminal_exposes_no_normal_actions`, `terminal-turn-cap.trace.json`. | covered-by-trace | Terminal tree is empty. |
| `TB-ACT-001` | Legal collect actions come from Rust. | `actions.rs`, `rules.rs`. | `legal_actions_cover_collect_exchange_fulfill_in_stable_order`, `shortest-normal.trace.json`. | covered-by-trace | Supply constraints are validated in Rust. |
| `TB-ACT-002` | Legal exchange actions come from Rust. | `actions.rs`, `rules.rs`. | `validate_accepts_each_legal_family`, `exchange.trace.json`. | covered-by-trace | Pay and take metadata comes from Rust. |
| `TB-ACT-003` | Legal fulfill actions come from Rust. | `actions.rs`, `rules.rs`. | `apply_fulfill_scores_refills_and_advances`, `contract-fulfill-refill.trace.json`. | covered-by-trace | Affordability is Rust-owned. |
| `TB-ACT-004` | Forced pass only when no other action is legal. | `actions.rs`, `rules.rs`. | `forced_pass_appears_only_when_stuck`, `validate_accepts_each_legal_family`. | covered | Voluntary pass rejects. |
| `TB-ACT-005` | Terminal exposes no normal actions. | `actions.rs`, `rules.rs`. | `terminal_exposes_no_normal_actions`, `terminal-turn-cap.trace.json`. | covered-by-trace | Empty gameplay tree is expected. |
| `TB-COLLECT-001` | Amber bundle requires amber supply. | `actions.rs`, `rules.rs`. | `validate_accepts_each_legal_family`, `supply-exhaustion-diagnostic.trace.json`. | covered-by-trace | Exhausted supply rejects without mutation. |
| `TB-COLLECT-002` | Jade bundle requires jade supply. | `actions.rs`, `rules.rs`. | `validate_accepts_each_legal_family`, `deterministic_playout_conserves_resources_and_never_panics`. | covered | Conservation covers supply movement. |
| `TB-COLLECT-003` | Iron bundle requires iron supply. | `actions.rs`, `rules.rs`. | `validate_accepts_each_legal_family`, `terminal-turn-cap.trace.json`. | covered-by-trace | Iron collection appears in terminal trace. |
| `TB-COLLECT-004` | Amber-jade bundle requires both resources. | `actions.rs`, `rules.rs`. | `legal_actions_cover_collect_exchange_fulfill_in_stable_order`, `action_ids_are_stable_and_duplicate_free`. | covered | Bundle metadata is stable. |
| `TB-COLLECT-005` | Jade-iron bundle requires both resources. | `actions.rs`, `rules.rs`. | `legal_actions_cover_collect_exchange_fulfill_in_stable_order`, `action_ids_are_stable_and_duplicate_free`. | covered | Bundle metadata is stable. |
| `TB-COLLECT-006` | Iron-amber bundle requires both resources. | `actions.rs`, `rules.rs`. | `legal_actions_cover_collect_exchange_fulfill_in_stable_order`, `action_ids_are_stable_and_duplicate_free`. | covered | Bundle metadata is stable. |
| `TB-EXCHANGE-001` | Exchange resources must differ. | `actions.rs`, `rules.rs`. | `apply_exchange_returns_paid_supply_and_takes_requested_resource`, `exchange.trace.json`. | covered-by-trace | Same-resource exchange is invalid. |
| `TB-EXCHANGE-002` | Active seat must pay two resources. | `rules.rs`, `effects.rs`. | `apply_exchange_returns_paid_supply_and_takes_requested_resource`, `insufficient-resources-diagnostic.trace.json`. | covered-by-trace | Insufficient payment rejects without mutation. |
| `TB-EXCHANGE-003` | Public supply must provide taken resource. | `rules.rs`, `effects.rs`. | `apply_exchange_returns_paid_supply_and_takes_requested_resource`, `deterministic_playout_conserves_resources_and_never_panics`. | covered | Supply return and take are effect-visible. |
| `TB-FULFILL-001` | Fulfill targets an occupied slot. | `rules.rs`, `state.rs`. | `empty_slot_rejects_fulfill`, `empty-slot-diagnostic.trace.json`. | covered-by-trace | Empty slot rejects without mutation. |
| `TB-FULFILL-002` | Fulfill requires full cost. | `rules.rs`, `effects.rs`. | `fulfill_requires_affordable_contract`, `insufficient-resources-diagnostic.trace.json`. | covered-by-trace | Partial payment is illegal. |
| `TB-FULFILL-003` | Fulfill awards printed points. | `rules.rs`, `effects.rs`. | `apply_fulfill_scores_refills_and_advances`, `contract-fulfill-refill.trace.json`. | covered-by-trace | Score change is public. |
| `TB-FULFILL-004` | Fulfilled id appends to public list. | `rules.rs`, `state.rs`, `visibility.rs`. | `apply_fulfill_scores_refills_and_advances`, `fulfilled_contract_is_removed_from_visible_and_queued_market`. | covered | Fulfilled list affects tie-breaks. |
| `TB-FULFILL-005` | Vacated slot refills from queue. | `rules.rs`, `effects.rs`. | `apply_fulfill_scores_refills_and_advances`, `contract-fulfill-refill.trace.json`. | covered-by-trace | Empty slot remains when queue is exhausted. |
| `TB-FULFILL-006` | Terminal checks after fulfill and refill. | `rules.rs`. | `empty_slot_when_queue_exhausted_and_terminal_when_market_empty`, `market-exhaustion.trace.json`. | covered-by-trace | Market exhaustion can end the game. |
| `TB-RESTRICT-001` | Wrong seat rejects without mutation. | `rules.rs`. | `wrong_actor_rejects`, `non-active-seat-diagnostic.trace.json`. | covered-by-trace | Active seat is public. |
| `TB-RESTRICT-002` | Illegal targets reject without mutation. | `rules.rs`. | `invalid_commands_reject_without_mutation`, diagnostic golden traces. | covered-by-trace | Public target diagnostics are safe. |
| `TB-RESTRICT-003` | Stale command rejects without mutation. | `rules.rs`, `replay_support.rs`. | `invalid_and_stale_commands_reject_without_mutation_during_replay`, `stale-diagnostic.trace.json`. | covered-by-trace | Freshness is validated before action-specific checks. |
| `TB-RESTRICT-004` | Pass is illegal unless forced. | `rules.rs`. | `forced_pass_appears_only_when_stuck`, `invalid_commands_reject_without_mutation`. | covered | Prevents voluntary skipping. |
| `TB-SCORE-001` | Scores start at zero and increase by fulfill. | `setup.rs`, `rules.rs`. | `setup_is_deterministic_standard_public_state`, `apply_fulfill_scores_refills_and_advances`. | covered | Scores never decrease. |
| `TB-SCORE-002` | Collect moves resources from supply to inventory. | `rules.rs`, `effects.rs`. | `apply_collect_updates_accounting_and_turn`, `shortest-normal.trace.json`. | covered-by-trace | Deltas are effect-visible. |
| `TB-SCORE-003` | Exchange returns two and takes one. | `rules.rs`, `effects.rs`. | `apply_exchange_returns_paid_supply_and_takes_requested_resource`, `exchange.trace.json`. | covered-by-trace | Both supply movements are public. |
| `TB-SCORE-004` | Fulfill returns cost and increases score. | `rules.rs`, `effects.rs`. | `apply_fulfill_scores_refills_and_advances`, `contract-fulfill-refill.trace.json`. | covered-by-trace | Cost and score effect summaries are stable. |
| `TB-SCORE-005` | Inventory total is terminal tie-break three. | `rules.rs`. | `terminal_tie_break_order_is_score_fulfilled_inventory_draw`, `turn_cap_terminal_uses_tie_breaks`. | covered | Resource mix has no value order. |
| `TB-END-001` | Both seats at turn cap ends game. | `rules.rs`. | `turn_cap_terminal_uses_tie_breaks`, `terminal-turn-cap.trace.json`. | covered-by-trace | Normal terminal path. |
| `TB-END-002` | Last contract and empty market ends game. | `rules.rs`. | `empty_slot_when_queue_exhausted_and_terminal_when_market_empty`, `market-exhaustion.trace.json`. | covered-by-trace | Near-state trace pins the edge case. |
| `TB-END-003` | Terminal tie-break order. | `rules.rs`. | `terminal_tie_break_order_is_score_fulfilled_inventory_draw`, `terminal-turn-cap.trace.json`. | covered-by-trace | Draw is possible after all comparisons tie. |
| `TB-VIS-001` | Public game facts visible to all viewers. | `visibility.rs`. | `observer_and_seat_views_match_for_public_game`, `public_view_exposes_required_board_fields`. | covered | Observer and seat views are identical. |
| `TB-VIS-002` | Legal actions and metadata come from Rust. | `actions.rs`, `visibility.rs`. | `ui_metadata_has_labels_without_debug_or_candidate_data`, `action_ids_are_stable_and_duplicate_free`. | covered | TypeScript may render only. |
| `TB-VIS-003` | Hidden choices do not exist in this game. | `visibility.rs`, `bots.rs`, `replay_support.rs`. | `public_surfaces_do_not_expose_internal_or_candidate_fields`, `wasm-exported.trace.json`. | covered-by-trace | Gate 9.1 owns commitment hiding. |
| `TB-RNG-001` | No random setup or resolution. | `setup.rs`, `replay_support.rs`. | `setup_is_deterministic_standard_public_state`, `golden_traces_replay_hashes_diagnostics_exports_and_no_leak_surfaces`. | covered-by-trace | Seeds are metadata for tool consistency only. |
| `TB-RNG-002` | Contract queue and refill are deterministic. | `setup.rs`, `rules.rs`. | `contract_specs_are_in_standard_queue_order`, `contract-fulfill-refill.trace.json`. | covered-by-trace | Browser state cannot affect refill. |
| `TB-RNG-003` | Serialization order remains stable. | `state.rs`, `effects.rs`, `replay_support.rs`. | `state_snapshot_round_trips_with_stable_bytes`, `effect_serialization_is_stable`, replay golden traces. | covered-by-trace | Hash drift is caught by replay-check. |
| `TB-AMB-001` | Queue projection is Rust-owned. | `visibility.rs`. | `public_view_exposes_required_board_fields`, `observer_and_seat_views_match_for_public_game`. | covered | UI consumes queue count from projection. |
| `TB-AMB-002` | Pass is forced-only. | `rules.rs`. | `forced_pass_appears_only_when_stuck`, `invalid_commands_reject_without_mutation`. | covered | Matches the chosen ambiguity resolution. |
| `TB-AMB-003` | Inventory tie-break ignores resource type values. | `rules.rs`. | `terminal_tie_break_order_is_score_fulfilled_inventory_draw`. | covered | Total inventory only. |
| `TB-VAR-001` | Original two-seat public deterministic economy variant. | `docs/SOURCES.md`, `setup.rs`, `variants.rs`. | `static_data_parses_and_rejects_unknown_fields`, `token_bazaar_standard.fixture.json`. | covered | Originality and scope are documented. |
| `TB-VAR-002` | Secret draft commitment/reveal is deferred. | successor gate only. | `RULES.md` explicit out-of-scope row, Gate 9 spec sequencing. | intentionally-deferred | Deferred to Gate 9.1. |
| `TB-VAR-003` | Auctions, betting, trading, random variants, and generic economy primitives are out of scope. | not implemented. | `boundary-check.sh`, `docs/FOUNDATIONS.md`, `RULES.md`. | unsupported | Future variants require accepted spec or ADR. |

## Golden Trace Catalog

| Trace file | Purpose | Rule IDs covered | Diagnostic coverage |
|---|---|---|---|
| `shortest-normal.trace.json` | shortest public collect sequence | `TB-TURN-001`, `TB-SCORE-002`, `TB-SETUP-002` | none |
| `terminal-turn-cap.trace.json` | turn-cap terminal draw path | `TB-TURN-003`, `TB-END-001`, `TB-END-003` | none |
| `contract-fulfill-refill.trace.json` | fulfill and refill | `TB-ACT-003`, `TB-FULFILL-003`, `TB-FULFILL-005` | none |
| `market-exhaustion.trace.json` | near-state market exhaustion | `TB-END-002`, `TB-FULFILL-006` | none |
| `exchange.trace.json` | exchange payment and supply movement | `TB-ACT-002`, `TB-EXCHANGE-002`, `TB-SCORE-003` | none |
| `supply-exhaustion-diagnostic.trace.json` | exhausted supply rejection | `TB-COLLECT-001`, `TB-RESTRICT-002` | `exhausted_supply` |
| `insufficient-resources-diagnostic.trace.json` | unaffordable fulfill rejection | `TB-FULFILL-002`, `TB-RESTRICT-002` | `insufficient_cost` |
| `empty-slot-diagnostic.trace.json` | empty slot rejection | `TB-FULFILL-001`, `TB-RESTRICT-002` | `empty_slot` |
| `stale-diagnostic.trace.json` | stale command rejection | `TB-RESTRICT-003` | `stale_action` |
| `non-active-seat-diagnostic.trace.json` | wrong actor rejection | `TB-RESTRICT-001` | `not_active_seat` |
| `bot-action.trace.json` | Level 1 bot command | `TB-VIS-003`, `TB-ACT-003` | none |
| `wasm-exported.trace.json` | public export/import shape | `TB-RNG-003`, `TB-VIS-003` | none |

## Tool Coverage

| Surface | Command/evidence | Rule IDs stressed | Status |
|---|---|---|---|
| simulation | `cargo run -p simulate -- --game token_bazaar --games 1000 --start-seed 1` | setup, legality, bot, terminal | covered |
| replay drift gate | `cargo run -p replay-check -- --game token_bazaar --all` | trace hashes and replay determinism | covered |
| fixture/schema gate | `cargo run -p fixture-check -- --game token_bazaar` | static-data and trace integrity | covered |
| rule coverage gate | `cargo run -p rule-coverage -- --game token_bazaar` | all rule IDs | covered |
| native benchmarks | `cargo bench -p token_bazaar` | benchmark smoke floors | covered |
