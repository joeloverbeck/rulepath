# High Card Duel Rule Coverage Matrix

Game ID: `high_card_duel`

Rules version: `high-card-duel-rules-v1`

Data/manifest version: `1`

Engine version: `0.1.0`

Prepared by: `Codex`

Last updated: 2026-06-07

## Purpose

This matrix maps every stable rule ID in `RULES.md` to implementation and evidence. Rust tests, golden traces, replay checks, fixture checks, simulations, serialization checks, and no-leak tests are primary evidence; browser smoke proves integration only.

## Rule Coverage Matrix

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `HCD-ACT-001` | Lead seat alone may commit during `lead_commit`. | `actions.rs`, `rules.rs`. | `lead_commit_removes_card_from_own_hand`; `observer_has_no_private_commit_actions`; `shortest-normal.trace.json`. | covered-by-trace | Rust legal tree owns the actor boundary. |
| `HCD-ACT-002` | Reply seat alone may commit during `reply_commit`. | `actions.rs`, `rules.rs`. | `reply_commit_cannot_see_lead_identity`; `shortest-normal.trace.json`; `terminal.trace.json`. | covered-by-trace | Reply choices come from the reply seat's hand only. |
| `HCD-ACT-003` | Commit action must target one actor-owned private card. | `actions.rs`, `rules.rs`. | `actor_private_tree_names_only_own_cards`; `invalid_private_card_diagnostic_redacted_for_unauthorized`; `invalid-private-card-redacted.trace.json`. | covered-by-trace | Opponent/deck identities are not legal targets. |
| `HCD-ACT-004` | A seat cannot commit twice in the same round. | `rules.rs`. | `both_commitments_reveal_together`; validation tests. | covered | Existing commitment conflicts reject without mutation. |
| `HCD-ACT-005` | Observer viewers have no private commit actions. | `actions.rs`, `visibility.rs`. | `observer_has_no_private_commit_actions`; `hidden-info-public-observer.trace.json`. | covered-by-trace | Public controls must consume Rust output only. |
| `HCD-ACT-006` | Terminal state has no gameplay actions. | `actions.rs`, `rules.rs`. | `terminal_after_six_rounds`; `terminal.trace.json`; replay-check. | covered-by-trace | Terminal action tree hashes empty. |
| `HCD-ACT-007` | No new no-op terminal convention is introduced. | `actions.rs`, `rules.rs`. | `terminal.trace.json`; action-tree hash coverage. | covered-by-trace | Existing tools use an empty tree for this game. |
| `HCD-ACT-008` | Authorized action labels may show own card; unauthorized surfaces must not. | `actions.rs`, `visibility.rs`, `replay_support.rs`. | `actor_private_tree_names_only_own_cards`; `public_projection_never_grows_hidden_fields_across_seeds`; `public-replay-export-import.trace.json`. | covered-by-trace | Public exports redact raw action paths. |
| `HCD-DIAG-001` | Wrong-seat actions return public-safe diagnostics. | `rules.rs`, `effects.rs`. | `wrong_seat_diagnostic_public_safe`; `invalid-wrong-seat-diagnostic.trace.json`. | covered-by-trace | Active seat is public. |
| `HCD-DIAG-002` | Wrong-phase actions return public-safe diagnostics. | `rules.rs`, `effects.rs`. | `wrong_phase_diagnostic_public_safe`; validation tests. | covered | Phase is public. |
| `HCD-DIAG-003` | Invalid private-card diagnostics are redacted. | `rules.rs`, `effects.rs`. | `invalid_private_card_diagnostic_redacted_for_unauthorized`; `invalid-private-card-redacted.trace.json`. | covered-by-trace | Public message does not echo hidden IDs. |
| `HCD-DIAG-004` | Stale actions do not leak hidden state. | `rules.rs`, replay support. | `stale_action_diagnostic_no_hidden_leak`; `stale-diagnostic.trace.json`. | covered-by-trace | Freshness rejects before private validation. |
| `HCD-DIAG-005` | Commitment conflicts reveal only reason class. | `rules.rs`, effects. | validation tests; visibility no-leak suite. | covered | Face-down occupancy is public; card identity is not. |
| `HCD-DIAG-006` | Browser-visible diagnostics must remain redacted. | `rules.rs`, `effects.rs`, `replay_support.rs`. | no-leak serialization/replay tests; `public-replay-export-import.trace.json`. | covered-by-trace | Browser-specific smoke lands later. |
| `HCD-ROUND-001` | Each round has one lead and one reply. | `state.rs`, `rules.rs`, `visibility.rs`. | `lead_alternates_by_round`; `public_projection_never_grows_hidden_fields_across_seeds`; traces. | covered-by-trace | Lead/reply are public. |
| `HCD-ROUND-002` | Lead commits one own private card face-down. | `rules.rs`, `effects.rs`. | `lead_commit_removes_card_from_own_hand`; `shortest-normal.trace.json`. | covered-by-trace | Public effect is face-down only. |
| `HCD-ROUND-003` | Reply commits one own private card without seeing lead identity. | `rules.rs`, `visibility.rs`. | `reply_actor_view_lacks_lead_card_before_reveal`; `shortest-normal.trace.json`. | covered-by-trace | Reply view redacts lead card. |
| `HCD-ROUND-004` | Both commitments reveal simultaneously. | `rules.rs`, `effects.rs`. | `both_commitments_reveal_together`; `tie-round.trace.json`; `shortest-normal.trace.json`. | covered-by-trace | Reveal effect exposes both cards together. |
| `HCD-ROUND-005` | Higher rank wins one point. | `rules.rs`. | `higher_rank_scores_one_point`; `shortest-normal.trace.json`. | covered-by-trace | Sigil is identity only. |
| `HCD-ROUND-006` | Equal ranks score no point. | `rules.rs`. | `tie_round_scores_no_points`; `tie-round.trace.json`. | covered-by-trace | Tie history is public after reveal. |
| `HCD-ROUND-007` | Revealed cards move to public history. | `rules.rs`, `visibility.rs`. | `committed_cards_reveal_exactly_once`; replay tests. | covered | Only revealed cards enter public history. |
| `HCD-ROUND-008` | Hands refill to three while deck remains. | `rules.rs`, `effects.rs`. | `refill_restores_hand_size_when_deck_available`; property tests. | covered | Deal effects are private. |
| `HCD-ROUND-009` | Refill starts with next lead and alternates. | `rules.rs`. | `lead_alternates_by_round`; property tests; traces. | covered-by-trace | Draw identities remain private. |
| `HCD-ROUND-010` | Lead alternates by odd/even round. | `rules.rs`. | `lead_alternates_by_round`; golden traces. | covered-by-trace | Odd is seat_0, even is seat_1. |
| `HCD-ROUND-011` | Nonterminal cleanup advances to next lead commit. | `rules.rs`. | `card_conservation_holds_across_commit_reveal_refill_transitions`; traces. | covered-by-trace | Freshness advances. |
| `HCD-ROUND-012` | Round six cleanup reaches terminal. | `rules.rs`. | `terminal_after_six_rounds`; `terminal.trace.json`. | covered-by-trace | No gameplay actions remain. |
| `HCD-ROUND-013` | Terminal winner is higher score; equal score is draw. | `rules.rs`, `visibility.rs`. | `terminal_winner_and_draw_policy`; `terminal.trace.json`; draw full-trace replay seeds. | covered-by-trace | Hidden deck tail remains hidden. |
| `HCD-SETUP-001` | Build canonical 24-card deck. | `ids.rs`, `setup.rs`. | `canonical_card_ids_are_stable_and_bounded`; setup tests. | covered | Rank then sigil order. |
| `HCD-SETUP-002` | Shuffle with deterministic Rust-owned seeded RNG. | `setup.rs`. | `setup_same_seed_same_initial_deal_internal`; `setup_different_seeds_can_change_initial_deal`; golden traces. | covered-by-trace | Uses `hcd-shuffle-v1`. |
| `HCD-SETUP-003` | Deal three private cards per seat alternating seat_0 then seat_1. | `setup.rs`, `state.rs`. | `setup_deals_private_hands_and_hides_deck`; fixture metadata. | covered | Private identities are scoped to owners. |
| `HCD-SETUP-004` | Initialize round, score, phase, and lead seat. | `setup.rs`, `state.rs`. | setup tests; `hidden-info-public-observer.trace.json`. | covered-by-trace | Public setup fields are deterministic. |
| `HCD-SETUP-005` | Store deck internally and expose only deck count publicly. | `state.rs`, `visibility.rs`. | `observer_view_has_no_private_hand_or_deck_or_facedown_identity`; `terminal_public_view_still_hides_unused_deck_tail`; public export tests. | covered | Terminal public exports do not reveal deck tail. |

## Golden Trace Catalog

| Trace file | Purpose | Rule IDs covered | Diagnostic coverage |
|---|---|---|---|
| `shortest-normal.trace.json` | normal one-round replay | `HCD-ACT-001`, `HCD-ACT-002`, `HCD-ROUND-002`, `HCD-ROUND-003`, `HCD-ROUND-005` | none |
| `tie-round.trace.json` | tied rank scoring | `HCD-ROUND-004`, `HCD-ROUND-006` | none |
| `invalid-wrong-seat-diagnostic.trace.json` | wrong actor rejection | `HCD-DIAG-001` | `wrong_seat` |
| `invalid-private-card-redacted.trace.json` | invalid private card rejection | `HCD-ACT-003`, `HCD-DIAG-003` | `invalid_private_card` |
| `stale-diagnostic.trace.json` | stale token rejection | `HCD-DIAG-004` | `stale_action` |
| `bot-action.trace.json` | Level 0 bot command | `HCD-ACT-003`, `HCD-ACT-008` | none |
| `hidden-info-public-observer.trace.json` | public observer setup projection | `HCD-ACT-005`, `HCD-SETUP-005` | none |
| `seat-private-view.trace.json` | authorized seat-private view hash | `HCD-ACT-008`, `HCD-ROUND-002` | none |
| `public-replay-export-import.trace.json` | public export/import no-leak | `HCD-DIAG-006`, `HCD-ACT-008` | none |
| `terminal.trace.json` | six-round terminal replay | `HCD-ACT-006`, `HCD-ROUND-012`, `HCD-ROUND-013` | none |

## Tool Coverage

| Surface | Command/evidence | Rule IDs stressed | Status |
|---|---|---|---|
| simulation | `cargo run -p simulate -- --game high_card_duel --games 100 --start-seed 1` | setup, legality, bot, terminal | covered |
| replay drift gate | `cargo run -p replay-check -- --game high_card_duel --all` | trace hashes and replay determinism | covered |
| fixture/schema gate | `cargo run -p fixture-check -- --game high_card_duel` | static-data and trace integrity | covered |
| rule coverage gate | `cargo run -p rule-coverage -- --game high_card_duel` | all rule IDs | covered |
| native benchmarks | `cargo bench -p high_card_duel` | performance surfaces | covered |
