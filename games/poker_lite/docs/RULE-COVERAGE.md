# Crest Ledger Rule Coverage Matrix

Game ID: `poker_lite`

Rules version: `poker-lite-rules-v1`

Last updated: 2026-06-09

## Purpose

This matrix maps every stable rule ID in [RULES.md](RULES.md) to the current
Rust implementation and evidence. The `rule-coverage` tool validates that every
rule ID has exactly one row.

## Rule Coverage Matrix

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `CL-ACT-001` | Hold/press when no outstanding pledge. | `actions.rs`. | `legal_action_generation_follows_pledge_state`, `hold-hold-center-reveal.trace.json`. | covered-by-trace | Opening and round-2 starts covered. |
| `CL-ACT-002` | Match/yield/lift while facing. | `actions.rs`. | `press_lift_match_accounting_is_exact_and_bounded`, `lift-match-showdown.trace.json`. | covered-by-trace | Lift only when cap remains. |
| `CL-ACT-003` | Used lift cap removes lift. | `actions.rs`. | `legal_tree_never_offers_lift_after_round_lift_cap`, `invalid-lift-cap-diagnostic.trace.json`. | covered-by-trace | Property and diagnostic evidence. |
| `CL-ACT-004` | Terminal exposes no gameplay actions. | `actions.rs`, `rules.rs`. | `seeded_legal_playouts_terminate_within_action_cap`, `yield-terminal-no-showdown.trace.json`. | covered-by-trace | Terminal validation covered in rules tests. |
| `CL-ACT-005` | Action metadata is public safe. | `actions.rs`. | `metadata_is_public_allow_list_only`, `action_effect_and_diagnostic_surfaces_do_not_leak_pre_reveal`. | covered | Metadata allow list checked. |
| `CL-AMB-001` | Public naming remains neutral. | `ids.rs`, `ui.rs`, docs. | `ui_copy_uses_neutral_terms`, `action_segments_are_neutral_and_stable`. | covered | No casino public copy. |
| `CL-AMB-002` | Hidden center is not inferable pre-reveal. | `visibility.rs`, `replay_support.rs`. | `observer_projection_never_leaks_hidden_crests_before_reveal`, `deal-private-no-leak.trace.json`. | covered-by-trace | Public exports redacted. |
| `CL-AMB-003` | Yield does not reveal private crests. | `rules.rs`, `visibility.rs`. | `showdown_view_reveals_both_private_crests_and_yield_does_not`, `yield-terminal-no-showdown.trace.json`. | covered-by-trace | Yield terminal is no-showdown. |
| `CL-AMB-004` | Bot policy is authored and no hidden sampling. | `bots.rs`. | `bot_explanations_and_effects_do_not_leak_hidden_cards_or_sampling_claims`, `bot-action.trace.json`. | covered-by-trace | No MCTS/ML/RL. |
| `CL-AMB-005` | Public export cannot reconstruct folded cards. | `replay_support.rs`. | `yield_terminal_public_export_cannot_reconstruct_folded_private_cards`, `public-replay-export-import.trace.json`. | covered-by-trace | Seed material omitted. |
| `CL-COMP-001` | Exactly two seats. | `ids.rs`, `setup.rs`. | `setup_rejects_wrong_seat_count`, `poker_lite_standard.fixture.json`. | covered | Stable seat ids. |
| `CL-COMP-002` | Crest component is typed local data. | `ids.rs`. | `canonical_card_ids_match_rules_order`, `deal-private-no-leak.trace.json`. | covered-by-trace | Nouns stay game-local. |
| `CL-COMP-003` | Rank values are stable. | `ids.rs`, `rules.rs`. | `crest_rank_labels_and_values_are_stable`, `pair-beats-high-card.trace.json`. | covered-by-trace | Used by showdown. |
| `CL-COMP-004` | Copy identity is stable. | `ids.rs`. | `canonical_card_ids_match_rules_order`, `tie-split.trace.json`. | covered-by-trace | Copy does not break ties. |
| `CL-COMP-005` | Private crest visibility. | `state.rs`, `visibility.rs`. | `seat_view_gets_only_own_private_strength_bucket`, `seat-private-view.trace.json`. | covered-by-trace | Owner-only before showdown. |
| `CL-COMP-006` | Center crest hidden then public. | `state.rs`, `visibility.rs`. | `center_reveal_does_not_reveal_private_or_tail`, `hold-hold-center-reveal.trace.json`. | covered-by-trace | Center reveal grouped. |
| `CL-COMP-007` | Deck tail is internal only. | `state.rs`, `visibility.rs`. | `observer_before_reveal_sees_counts_and_no_hidden_crests`, `no-leak-public-observer.trace.json`. | covered-by-trace | No viewer-facing tail. |
| `CL-COMP-008` | Markers are public accounting units. | `rules.rs`, `visibility.rs`. | `press_lift_match_accounting_is_exact_and_bounded`, `seeded_legal_playouts_preserve_accounting_invariants`. | covered | No money semantics. |
| `CL-COMP-009` | Shared pool is exact. | `rules.rs`. | `shared_pool == sum(contributions)` property, `press-match-showdown-reveal.trace.json`. | covered-by-trace | Property enforces invariant. |
| `CL-COMP-010` | Two pledge rounds. | `rules.rs`, `state.rs`. | `hold_hold_closes_round_one_and_reveals_center_only`, `press-match-showdown-reveal.trace.json`. | covered-by-trace | Round 2 resolves terminal. |
| `CL-COMP-011` | Outstanding pledge is public. | `actions.rs`, `visibility.rs`. | `legal_action_generation_follows_pledge_state`, `lift-match-showdown.trace.json`. | covered-by-trace | Action tree metadata covers price. |
| `CL-COMP-012` | One lift per round. | `actions.rs`, `rules.rs`. | `invalid-lift-cap-diagnostic.trace.json`, `legal_tree_never_offers_lift_after_round_lift_cap`. | covered-by-trace | Cap resets by round. |
| `CL-COMP-013` | Terminal allocation is public. | `rules.rs`, `visibility.rs`. | `showdown_terminal_allocates_win_or_split_from_rust_comparator`, `tie-split.trace.json`. | covered-by-trace | Yield and showdown covered. |
| `CL-END-001` | Yield terminal. | `rules.rs`. | `yield_terminal_awards_pool_without_showdown_reveal`, `yield-terminal-no-showdown.trace.json`. | covered-by-trace | No private reveal. |
| `CL-END-002` | Showdown win terminal. | `rules.rs`. | `showdown_terminal_allocates_win_or_split_from_rust_comparator`, `high-card-showdown.trace.json`. | covered-by-trace | Winner chosen by comparator. |
| `CL-END-003` | Showdown split terminal. | `rules.rs`. | `comparator_covers_pair_high_card_and_split`, `tie-split.trace.json`. | covered-by-trace | Even pool split. |
| `CL-OOS-001` | Trick-taking is out of scope. | docs. | `RULES.md`, `GAME-IMPLEMENTATION-ADMISSION.md`. | not-applicable | Crest Ledger is betting/showdown only. |
| `CL-OOS-002` | Casino/money semantics are out of scope. | `ui.rs`, docs. | `ui_copy_uses_neutral_terms`, `SOURCES.md`. | not-applicable | Neutral terms only. |
| `CL-OOS-003` | General poker engine is out of scope. | docs. | `RULES.md`, `SOURCES.md`. | not-applicable | Only Crest Ledger standard variant. |
| `CL-OOS-004` | Search/ML bots are out of scope. | `bots.rs`, docs. | `BOT-STRATEGY-EVIDENCE-PACK.md`, `bot-action.trace.json`. | not-applicable | Public v1/v2 forbids search/learning classes. |
| `CL-PLEDGE-001` | Hold adds no markers. | `rules.rs`. | `hold_hold_closes_round_one_and_reveals_center_only`, `hold-hold-center-reveal.trace.json`. | covered-by-trace | Shared pool unchanged. |
| `CL-PLEDGE-002` | Press adds round unit. | `rules.rs`. | `press_lift_match_accounting_is_exact_and_bounded`, `press-match-showdown-reveal.trace.json`. | covered-by-trace | Creates outstanding pledge. |
| `CL-PLEDGE-003` | Lift matches plus unit. | `rules.rs`. | `press_lift_match_accounting_is_exact_and_bounded`, `lift-match-showdown.trace.json`. | covered-by-trace | Consumes cap. |
| `CL-PLEDGE-004` | Match adds outstanding amount. | `rules.rs`. | `press_lift_match_accounting_is_exact_and_bounded`, `press-match-showdown-reveal.trace.json`. | covered-by-trace | Closes round. |
| `CL-PLEDGE-005` | Yield ends immediately. | `rules.rs`. | `yield_terminal_awards_pool_without_showdown_reveal`, `yield-terminal-no-showdown.trace.json`. | covered-by-trace | Non-yielder wins pool. |
| `CL-RESTRICT-001` | Unknown/non-seat actor rejects. | `actions.rs`. | `validation_reports_fail_closed_diagnostics`. | covered | Wrong actor diagnostic. |
| `CL-RESTRICT-002` | Wrong seat rejects. | `actions.rs`. | `invalid-wrong-seat-diagnostic.trace.json`, `validation_reports_fail_closed_diagnostics`. | covered-by-trace | No mutation. |
| `CL-RESTRICT-003` | Malformed/unavailable path rejects. | `actions.rs`. | `invalid-private-card-redacted.trace.json`, `validation_reports_fail_closed_diagnostics`. | covered-by-trace | Diagnostic no-leak. |
| `CL-RESTRICT-004` | Stale command rejects. | `actions.rs`. | `invalid-stale-diagnostic.trace.json`, `validation_reports_fail_closed_diagnostics`. | covered-by-trace | Freshness enforced. |
| `CL-RESTRICT-005` | Second lift rejects. | `actions.rs`. | `invalid-lift-cap-diagnostic.trace.json`. | covered-by-trace | Public cap reason. |
| `CL-RESTRICT-006` | Terminal action rejects. | `actions.rs`, `rules.rs`. | `validation_reports_fail_closed_diagnostics`, `seeded_legal_playouts_terminate_within_action_cap`. | covered | Terminal is final. |
| `CL-REVEAL-001` | Center reveal after round 1 close. | `rules.rs`, `effects.rs`. | `center_reveal_does_not_reveal_private_or_tail`, `hold-hold-center-reveal.trace.json`. | covered-by-trace | Grouped public reveal. |
| `CL-REVEAL-002` | Showdown reveal after round 2 close. | `rules.rs`, `effects.rs`. | `showdown_transition_emits_one_grouped_showdown_reveal`, `press-match-showdown-reveal.trace.json`. | covered-by-trace | Both private crests together. |
| `CL-RNG-001` | Seeded setup replay determinism. | `setup.rs`, `replay_support.rs`. | `seeded_legal_playouts_replay_deterministically`, `deal-private-no-leak.trace.json`. | covered-by-trace | Same seed/commands reproduce. |
| `CL-RNG-002` | Public export redaction. | `replay_support.rs`. | `public_export_import_round_trips_for_observer_and_seat_viewer`, `public-replay-export-import.trace.json`. | covered-by-trace | No reconstructing secrets. |
| `CL-RNG-003` | Stable serialization order. | `replay_support.rs`. | `internal_trace_json_round_trips_stably_and_rejects_unknown_fields`, `golden_traces_match_expected_replay_hashes_diagnostics_and_no_leak_surfaces`. | covered-by-trace | Hash drift catches changes. |
| `CL-SCORE-001` | Opening contributions. | `setup.rs`. | `setup_deals_private_center_tail_and_opening_accounting`, `poker_lite_standard.fixture.json`. | covered | Pool starts at 2. |
| `CL-SCORE-002` | Pledge actions update pool exactly. | `rules.rs`. | `press_lift_match_accounting_is_exact_and_bounded`. | covered | Invalid actions do not mutate. |
| `CL-SCORE-003` | Contribution cap is bounded. | `rules.rs`, `actions.rs`. | `seeded_legal_playouts_terminate_within_action_cap`, `seeded_legal_playouts_preserve_accounting_invariants`. | covered | Max contribution 7. |
| `CL-SCORE-004` | Showdown strength comparator. | `rules.rs`. | `comparator_covers_pair_high_card_and_split`, `pair-beats-high-card.trace.json`. | covered-by-trace | Pair then high card. |
| `CL-SCORE-005` | Equal showdown splits pool. | `rules.rs`. | `tie-split.trace.json`, `showdown_terminal_allocates_win_or_split_from_rust_comparator`. | covered-by-trace | Split exact. |
| `CL-SCORE-006` | Yield awards current pool. | `rules.rs`. | `yield-terminal-no-showdown.trace.json`, `yield_terminal_awards_pool_without_showdown_reveal`. | covered-by-trace | No showdown reveal. |
| `CL-SETUP-001` | Exactly two seats. | `setup.rs`. | `setup_rejects_wrong_seat_count`. | covered | Wrong counts reject. |
| `CL-SETUP-002` | Stable seeded shuffle. | `setup.rs`. | `setup_is_deterministic_for_same_seed_and_options`. | covered | Deterministic RNG. |
| `CL-SETUP-003` | Deal private/center/tail. | `setup.rs`, `state.rs`. | `setup_deals_private_center_tail_and_opening_accounting`, `deal-private-no-leak.trace.json`. | covered-by-trace | Mixed visibility. |
| `CL-SETUP-004` | Initial phase and active seat. | `setup.rs`, `state.rs`. | `setup_deals_private_center_tail_and_opening_accounting`. | covered | Round 1 starts at seat_0. |
| `CL-SETUP-005` | Opening shared pool. | `setup.rs`. | `poker_lite_standard.fixture.json`, `static_data_and_fixture_match_setup_and_reject_unknown_fields`. | covered | Contributions 1 each. |
| `CL-SETUP-006` | Initial round state. | `state.rs`. | `initial_state_summary_keeps_hidden_fields_internal`. | covered | No outstanding pledge. |
| `CL-SETUP-007` | Setup effects scoped safely. | `effects.rs`. | `private_deal_effects_are_scoped_to_owner`, `deal-private-no-leak.trace.json`. | covered-by-trace | Public setup has counts only. |
| `CL-TURN-001` | Round 1 start choices. | `actions.rs`. | `legal_action_generation_follows_pledge_state`. | covered | Seat_0 hold/press. |
| `CL-TURN-002` | One hold passes turn. | `rules.rs`. | `hold_hold_closes_round_one_and_reveals_center_only`. | covered | Second hold closes. |
| `CL-TURN-003` | Facing outstanding pledge. | `actions.rs`, `rules.rs`. | `legal_action_generation_follows_pledge_state`, `lift-match-showdown.trace.json`. | covered-by-trace | Match/yield/lift states. |
| `CL-TURN-004` | Round 1 close reveals center. | `rules.rs`. | `hold-hold-center-reveal.trace.json`. | covered-by-trace | Seat_1 leads round 2. |
| `CL-TURN-005` | Round 2 starts with fresh cap. | `rules.rs`. | `press-match-showdown-reveal.trace.json`, `lift-match-showdown.trace.json`. | covered-by-trace | Unit 2, cap reset. |
| `CL-TURN-006` | Round 2 close resolves showdown. | `rules.rs`. | `press-match-showdown-reveal.trace.json`, `tie-split.trace.json`. | covered-by-trace | Terminal outcome. |
| `CL-TURN-007` | Terminal has no active seat. | `rules.rs`, `visibility.rs`. | `seeded_legal_playouts_terminate_within_action_cap`, `yield-terminal-no-showdown.trace.json`. | covered-by-trace | Active seat none. |
| `CL-VAR-001` | Standard variant is selected. | `variants.rs`, data. | `constants_match_static_data`, `static_data_and_fixture_match_setup_and_reject_unknown_fields`. | covered | `poker_lite_standard`. |
| `CL-VAR-002` | Static data is non-behavioral. | `variants.rs`, data. | `behavior_looking_keys_are_rejected`, `static_data_and_fixture_match_setup_and_reject_unknown_fields`. | covered | No formulas/selectors. |
| `CL-VAR-003` | Public display uses Crest Ledger. | `ui.rs`, data, docs. | `ui_copy_uses_neutral_terms`, `manifest parses` tests. | covered | Internal id remains code-only. |
| `CL-VIS-001` | Public pledge facts visible. | `visibility.rs`. | `observer_before_reveal_sees_counts_and_no_hidden_crests`. | covered | Public view summary stable. |
| `CL-VIS-002` | Private crests redacted before showdown. | `visibility.rs`, `replay_support.rs`. | `observer_projection_never_leaks_hidden_crests_before_reveal`, `seat-private-view.trace.json`. | covered-by-trace | Owner sees own only. |
| `CL-VIS-003` | Center hidden before reveal. | `visibility.rs`. | `observer_before_reveal_sees_counts_and_no_hidden_crests`, `no-leak-public-observer.trace.json`. | covered-by-trace | Hidden status only. |
| `CL-VIS-004` | Deck tail never viewer-facing. | `visibility.rs`. | `center_reveal_does_not_reveal_private_or_tail`, `deal-private-no-leak.trace.json`. | covered-by-trace | Native internal only. |
| `CL-VIS-005` | Legal choices depend on public pledge state. | `actions.rs`. | `legal_tree_never_offers_lift_after_round_lift_cap`, `metadata_is_public_allow_list_only`. | covered | Hidden cards not in action tree. |
| `CL-VIS-006` | Showdown reveal visible to all. | `visibility.rs`, `effects.rs`. | `showdown_view_reveals_both_private_crests_and_yield_does_not`, `press-match-showdown-reveal.trace.json`. | covered-by-trace | Reveal grouped. |
| `CL-VIS-007` | Yield terminal public payload safe. | `visibility.rs`, `rules.rs`. | `yield-terminal-no-showdown.trace.json`, `yield_terminal_public_export_cannot_reconstruct_folded_private_cards`. | covered-by-trace | Private crests stay hidden. |
| `CL-VIS-008` | Bot rationale is viewer-safe. | `bots.rs`, `effects.rs`. | `bot_explanations_and_effects_do_not_leak_hidden_cards_or_sampling_claims`, `bot-action.trace.json`. | covered-by-trace | Public effect is policy/action only. |
