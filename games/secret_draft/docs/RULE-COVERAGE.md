# Veiled Draft Rule Coverage Matrix

Game ID: `secret_draft`

Rules version: `secret-draft-rules-v1`

Data/manifest version: `1`

Engine version: `0.1.0`

Prepared by: `Codex`

Last updated: 2026-06-08

## Purpose

This matrix maps every stable rule ID in `RULES.md` to implementation and
evidence. Rust tests, golden traces, replay checks, fixture checks,
serialization checks, visibility checks, bot tests, simulations, and benchmark
smoke floors are primary evidence.

## Rule Coverage Matrix

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `SD-COMP-001` | Two public seats. | `ids.rs`, `setup.rs`, `state.rs`. | `setup_matches_standard_fixture_and_both_seats_have_legal_choices`, `secret_draft_standard.fixture.json`. | covered | Seat IDs are stable. |
| `SD-COMP-002` | Draft items are game-local typed items. | `ids.rs`, `state.rs`. | `constants_match_static_data`, `shortest-normal.trace.json`. | covered-by-trace | Item nouns stay game-local. |
| `SD-COMP-003` | Threads are typed and public. | `ids.rs`, `rules.rs`. | `level1_prefers_completing_a_thread_set`, `terminal-tie-break.trace.json`. | covered-by-trace | Used by set scoring. |
| `SD-COMP-004` | Values are typed and public. | `ids.rs`, `rules.rs`. | `terminal_cap_scores_and_tie_breaks_are_deterministic`, `draw-after-tie-breaks.trace.json`. | covered-by-trace | Values drive score and tie-breaks. |
| `SD-COMP-005` | Visible pool is ordered and public. | `state.rs`, `visibility.rs`. | `deterministic_playouts_preserve_pool_award_visibility_and_terminal_invariants`, `contested-pick-fallback.trace.json`. | covered-by-trace | Pool order drives fallback. |
| `SD-COMP-006` | Commitment slots are internal before reveal. | `state.rs`, `visibility.rs`, `replay_support.rs`. | `pending_views_commitment_fields_and_metadata_do_not_reveal_committed_item`, `first-commit-pending.trace.json`. | covered-by-trace | Browser payloads get booleans only. |
| `SD-COMP-007` | Drafted collections are public. | `state.rs`, `visibility.rs`. | `post_reveal_view_exposes_revealed_and_awarded_items`, `simultaneous-reveal-batch.trace.json`. | covered-by-trace | Awarded items appear after reveal. |
| `SD-COMP-008` | Priority seat is public and alternates. | `state.rs`, `rules.rs`. | `conflict_fallback_removes_contested_and_lowest_remaining_public_item`, `contested-pick-fallback.trace.json`. | covered-by-trace | Starts at seat_0. |
| `SD-COMP-009` | Fallback award is deterministic. | `rules.rs`, `effects.rs`. | `conflict_fallback_removes_contested_and_lowest_remaining_public_item`, `contested-pick-fallback.trace.json`. | covered-by-trace | Lowest stable remaining item. |
| `SD-COMP-010` | Reveal batch is synchronized. | `rules.rs`, `effects.rs`. | `second_commit_emits_reveal_batch_in_fixed_order`, `simultaneous-reveal-batch.trace.json`. | covered-by-trace | Choices appear together. |
| `SD-COMP-011` | Score summary is public. | `rules.rs`, `visibility.rs`. | `scoring_components_match_rules`, `terminal-tie-break.trace.json`. | covered-by-trace | Rust computes scores. |
| `SD-SETUP-001` | Exactly two seats. | `setup.rs`. | `setup_rejects_wrong_seat_count`, `secret_draft_standard.fixture.json`. | covered | Wrong counts reject. |
| `SD-SETUP-002` | Round 1 and seat_0 priority. | `setup.rs`, `state.rs`. | `setup_starts_with_empty_commitments_and_stable_pool`, `shortest-normal.trace.json`. | covered-by-trace | Initial priority is public. |
| `SD-SETUP-003` | Twelve visible items in stable order. | `setup.rs`, `ids.rs`. | `setup_matches_standard_fixture_and_both_seats_have_legal_choices`, `secret_draft_standard.fixture.json`. | covered | Fixture asserts order. |
| `SD-SETUP-004` | Empty drafted/commitment/accounting state. | `setup.rs`, `state.rs`. | `setup_is_deterministic`, `static_data_and_fixture_match_setup_and_reject_unknown_fields`. | covered | Commitment slots are internal. |
| `SD-SETUP-005` | No terminal outcome and initial freshness. | `setup.rs`, `actions.rs`. | `setup_starts_with_empty_commitments_and_stable_pool`, `validation_rejects_stale_already_committed_unavailable_and_wrong_actor`. | covered | Freshness gates stale commands. |
| `SD-TURN-001` | Nonterminal uncommitted seats get legal commits. | `actions.rs`. | `setup_matches_standard_fixture_and_both_seats_have_legal_choices`, `shortest-normal.trace.json`. | covered-by-trace | Both seats can commit. |
| `SD-TURN-002` | One committed seat creates pending state. | `rules.rs`, `visibility.rs`. | `pending_effects_diagnostics_and_public_export_redact_committed_item`, `first-commit-pending.trace.json`. | covered-by-trace | Hidden choice redacted. |
| `SD-TURN-003` | Both commitments trigger reveal resolution. | `rules.rs`. | `second_commit_emits_reveal_batch_in_fixed_order`, `simultaneous-reveal-batch.trace.json`. | covered-by-trace | Slots clear after resolution. |
| `SD-TURN-004` | Rounds 1-5 advance and alternate priority. | `rules.rs`. | `conflict_fallback_removes_contested_and_lowest_remaining_public_item`, `shortest-normal.trace.json`. | covered-by-trace | Round 2 priority flips. |
| `SD-TURN-005` | Round 6 terminalizes. | `rules.rs`. | `terminal_cap_scores_and_tie_breaks_are_deterministic`, `terminal-tie-break.trace.json`. | covered-by-trace | Six reveals end game. |
| `SD-TURN-006` | Terminal exposes no gameplay actions. | `actions.rs`, `rules.rs`. | `terminal_validation_rejects_direct_validated_action`, `terminal-tie-break.trace.json`. | covered-by-trace | Terminal rejects apply. |
| `SD-ACT-001` | Commit choices come from visible pool. | `actions.rs`. | `every_opening_legal_choice_validates_without_panic_or_mutation`, `shortest-normal.trace.json`. | covered-by-trace | Rust owns legal tree. |
| `SD-ACT-002` | Already committed seat has no choices. | `actions.rs`. | `committed_seat_has_no_decision`, `already-committed-diagnostic.trace.json`. | covered-by-trace | Pending metadata is redacted. |
| `SD-ACT-003` | Other seat commitment does not remove choices. | `actions.rs`, `rules.rs`. | `level1_uses_only_public_information_when_opponent_commitment_differs`, `first-commit-pending.trace.json`. | covered-by-trace | Prevents option-removal leak. |
| `SD-ACT-004` | Terminal tree is empty. | `actions.rs`. | `terminal_actor_gets_empty_tree`, `terminal-tie-break.trace.json`. | covered-by-trace | No normal terminal actions. |
| `SD-RESTRICT-001` | Unknown actor rejects. | `actions.rs`. | `validation_diagnostics_cover_stale_already_committed_unavailable_and_wrong_actor`. | covered | Wrong-seat diagnostic is safe. |
| `SD-RESTRICT-002` | Second same-round commit rejects. | `actions.rs`. | `already-committed-diagnostic.trace.json`, `pending_effects_diagnostics_and_public_export_redact_committed_item`. | covered-by-trace | Prior item is not named. |
| `SD-RESTRICT-003` | Unavailable item rejects. | `actions.rs`. | `unavailable-item-diagnostic.trace.json`, `validation_diagnostics_cover_stale_already_committed_unavailable_and_wrong_actor`. | covered-by-trace | Removed public item may be named. |
| `SD-RESTRICT-004` | Stale command rejects. | `actions.rs`. | `stale-diagnostic.trace.json`, `validation_diagnostics_cover_stale_already_committed_unavailable_and_wrong_actor`. | covered-by-trace | State does not mutate. |
| `SD-RESTRICT-005` | Terminal action rejects. | `actions.rs`, `rules.rs`. | `terminal_validation_rejects_direct_validated_action`, `terminal-tie-break.trace.json`. | covered-by-trace | Outcome is final. |
| `SD-REVEAL-001` | First commit emits pending-only effects. | `rules.rs`, `effects.rs`. | `first_commit_emits_pending_only_effects_without_item_id`, `first-commit-pending.trace.json`. | covered-by-trace | No item ID pre-reveal. |
| `SD-REVEAL-002` | Second commit reveals both choices together. | `rules.rs`, `effects.rs`. | `second_commit_emits_reveal_batch_in_fixed_order`, `simultaneous-reveal-batch.trace.json`. | covered-by-trace | Reveal order is stable. |
| `SD-REVEAL-003` | Different choices award both chosen items. | `rules.rs`. | `shortest-normal.trace.json`, `terminal_cap_scores_and_tie_breaks_are_deterministic`. | covered-by-trace | Two chosen items removed. |
| `SD-REVEAL-004` | Contested choice awards priority seat. | `rules.rs`. | `conflict_fallback_removes_contested_and_lowest_remaining_public_item`, `contested-pick-fallback.trace.json`. | covered-by-trace | Conflict win count increments. |
| `SD-REVEAL-005` | Contested fallback awards lowest remaining. | `rules.rs`. | `contested-pick-fallback.trace.json`, `deterministic_playouts_preserve_pool_award_visibility_and_terminal_invariants`. | covered-by-trace | Fallback count increments. |
| `SD-REVEAL-006` | Commitments clear after reveal. | `state.rs`, `rules.rs`. | `second_commit_emits_reveal_batch_in_fixed_order`, `simultaneous-reveal-batch.trace.json`. | covered-by-trace | Next round starts empty. |
| `SD-SCORE-001` | Base score sums drafted values. | `rules.rs`. | `scoring_components_match_rules`, `terminal-tie-break.trace.json`. | covered-by-trace | Values are public. |
| `SD-SCORE-002` | Complete thread sets score bonus. | `rules.rs`. | `level1_prefers_completing_a_thread_set`, `draw-after-tie-breaks.trace.json`. | covered-by-trace | Sets drive tie-break. |
| `SD-SCORE-003` | High-thread bonus scores once per thread. | `rules.rs`. | `scoring_components_match_rules`, `terminal-tie-break.trace.json`. | covered-by-trace | Fourth item does not double count. |
| `SD-SCORE-004` | Conflict-discipline terminal bonus. | `rules.rs`. | `scoring_components_match_rules`, `draw-after-tie-breaks.trace.json`. | covered-by-trace | Terminal-only bonus. |
| `SD-SCORE-005` | Exactly two items leave each reveal. | `rules.rs`. | `deterministic_playouts_preserve_pool_award_visibility_and_terminal_invariants`, `contested-pick-fallback.trace.json`. | covered-by-trace | No duplicate award. |
| `SD-END-001` | Sixth reveal ends game. | `rules.rs`. | `terminal_occurs_after_six_resolved_rounds_with_two_items_removed_each_round`, `terminal-tie-break.trace.json`. | covered-by-trace | Fixed cap. |
| `SD-END-002` | Terminal tie-break ladder. | `rules.rs`. | `terminal_tie_break_ladder_reaches_each_rung_and_draw`, `terminal-tie-break.trace.json`, `draw-after-tie-breaks.trace.json`. | covered-by-trace | Winner and draw covered. |
| `SD-VIS-001` | Public facts are viewer-safe. | `visibility.rs`. | `public_and_committing_seat_views_redact_pre_reveal_commitment`, `public-observer-no-leak.trace.json`. | covered-by-trace | Safe facts remain visible. |
| `SD-VIS-002` | Committed item id redacts before reveal. | `visibility.rs`, `replay_support.rs`. | `seat-private-no-prereveal-choice.trace.json`, `pending_effects_diagnostics_and_public_export_redact_committed_item`. | covered-by-trace | Even committing seat is redacted. |
| `SD-VIS-003` | Pending booleans are public. | `visibility.rs`, `effects.rs`. | `viewer_scoped_pre_reveal_effects_have_pending_booleans_only`, `first-commit-pending.trace.json`. | covered-by-trace | Pending state is required. |
| `SD-VIS-004` | Legal choices use visible pool only. | `actions.rs`. | `level1_uses_only_public_information_when_opponent_commitment_differs`, `first-commit-pending.trace.json`. | covered-by-trace | Hidden state does not alter choices. |
| `SD-VIS-005` | Bot rationale is viewer-safe. | `bots.rs`. | `bot_rationales_do_not_claim_hidden_or_sampled_information`, `bot-action.trace.json`. | covered-by-trace | No hidden-state sampling. |
| `SD-RNG-001` | No random setup or rule resolution. | `setup.rs`, `rules.rs`. | `setup_is_deterministic`, `golden_traces_match_expected_replay_hashes_diagnostics_and_no_leak_surfaces`. | covered-by-trace | Bot RNG is policy-only. |
| `SD-RNG-002` | Public replay export is viewer-scoped. | `replay_support.rs`. | `public-replay-export-import.trace.json`, `pre_reveal_public_export_redacts_item_path_and_seed_material`. | covered-by-trace | Internal traces retain authority. |
| `SD-RNG-003` | Serialization order is stable. | `state.rs`, `replay_support.rs`. | `state_and_view_summaries_are_deterministic`, `golden_traces_match_expected_replay_hashes_diagnostics_and_no_leak_surfaces`. | covered-by-trace | Replay-check catches drift. |
| `SD-AMB-001` | Committing seat does not see own item in browser payload. | `visibility.rs`. | `seat-private-no-prereveal-choice.trace.json`, `pending_views_commitment_fields_and_metadata_do_not_reveal_committed_item`. | covered-by-trace | Rust internal state remains authoritative. |
| `SD-AMB-002` | Hidden commit does not remove public option. | `actions.rs`, `rules.rs`. | `first-commit-pending.trace.json`, `level1_uses_only_public_information_when_opponent_commitment_differs`. | covered-by-trace | Conflict remains possible. |
| `SD-AMB-003` | Fallback is lowest stable remaining item. | `rules.rs`. | `contested-pick-fallback.trace.json`, `conflict_fallback_removes_contested_and_lowest_remaining_public_item`. | covered-by-trace | No randomness. |
| `SD-AMB-004` | Fewer priority conflict wins is late tie-break. | `rules.rs`. | `terminal_tie_break_ladder_reaches_each_rung_and_draw`, `terminal-tie-break.trace.json`. | covered-by-trace | Conflict history is public. |
| `SD-VAR-001` | Standard variant is original and fixed. | `variants.rs`, `docs/SOURCES.md`. | `static_data_and_fixture_match_setup_and_reject_unknown_fields`, `secret_draft_standard.fixture.json`. | covered | Originality documented. |
| `SD-VAR-002` | More than two seats are out of scope. | `setup.rs`. | `setup_rejects_wrong_seat_count`. | unsupported | Future spec required. |
| `SD-VAR-003` | Randomized pool order is out of scope. | `setup.rs`, `variants.rs`. | `setup_is_deterministic`, `SD-RNG-001` evidence. | unsupported | Standard variant is fixed. |
| `SD-VAR-004` | Cryptographic commitments are out of scope. | local Rust/WASM authority only. | `RULES.md`, `public-observer-no-leak.trace.json`. | unsupported | Hosted adversarial multiplayer is not in scope. |
| `SD-VAR-005` | Generic drafting/reveal primitives are out of scope. | game-local modules. | `bash scripts/boundary-check.sh`, `docs/FOUNDATIONS.md`. | unsupported | First focused official use only. |

## Golden Trace Catalog

| Trace file | Purpose | Rule IDs covered | Diagnostic coverage |
|---|---|---|---|
| `shortest-normal.trace.json` | shortest normal reveal | `SD-TURN-001`, `SD-REVEAL-003` | none |
| `first-commit-pending.trace.json` | pending no-leak state | `SD-TURN-002`, `SD-REVEAL-001`, `SD-VIS-003` | none |
| `simultaneous-reveal-batch.trace.json` | synchronized reveal | `SD-REVEAL-002`, `SD-COMP-010` | none |
| `contested-pick-fallback.trace.json` | contested priority/fallback | `SD-REVEAL-004`, `SD-REVEAL-005` | none |
| `terminal-tie-break.trace.json` | terminal winner path | `SD-END-001`, `SD-END-002` | none |
| `draw-after-tie-breaks.trace.json` | terminal draw path | `SD-END-002` | none |
| `already-committed-diagnostic.trace.json` | duplicate commit rejection | `SD-RESTRICT-002` | `already_committed` |
| `unavailable-item-diagnostic.trace.json` | removed item rejection | `SD-RESTRICT-003` | `item_unavailable` |
| `stale-diagnostic.trace.json` | stale command rejection | `SD-RESTRICT-004` | `stale_action` |
| `public-observer-no-leak.trace.json` | observer no-leak export | `SD-VIS-001`, `SD-VIS-002` | none |
| `seat-private-no-prereveal-choice.trace.json` | seat no-leak export | `SD-AMB-001`, `SD-VIS-002` | none |
| `bot-action.trace.json` | bot legal action | `SD-VIS-005`, `SD-ACT-001` | none |
| `public-replay-export-import.trace.json` | export/import round trip | `SD-RNG-002` | none |
| `wasm-exported.trace.json` | future WASM export shape | `SD-RNG-003` | none |

## Tool Coverage

| Surface | Command/evidence | Rule IDs stressed | Status |
|---|---|---|---|
| simulation | `cargo run -p simulate -- --game secret_draft --games 1000` | setup, legality, bot, terminal | covered |
| replay drift gate | `cargo run -p replay-check -- --game secret_draft --all` | trace hashes and replay determinism | covered |
| fixture/schema gate | `cargo run -p fixture-check -- --game secret_draft` | static-data and trace integrity | covered |
| rule coverage gate | `cargo run -p rule-coverage -- --game secret_draft` | all rule IDs | covered |
| native benchmarks | `cargo bench -p secret_draft` | benchmark smoke floors | covered |
