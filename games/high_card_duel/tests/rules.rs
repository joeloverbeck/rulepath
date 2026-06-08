use std::collections::BTreeSet;

use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};
use high_card_duel::{
    apply_action, canonical_deck, commit_segment, legal_action_tree, next_bounded_index_unbiased,
    setup_match, validate_command, CardId, HighCardDuelEffect, HighCardDuelSeat, Phase,
    SetupOptions, Sigil, TerminalOutcome, STANDARD_DECK_CARD_COUNT, STANDARD_HAND_SIZE,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn actor(index: usize) -> Actor {
    Actor {
        seat_id: seats()[index].clone(),
    }
}

fn command(
    actor_index: usize,
    segment: String,
    freshness_token: FreshnessToken,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(actor_index),
        action_path: ActionPath {
            segments: vec![segment],
        },
        freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn card(rank: u8, sigil: Sigil) -> CardId {
    CardId::new(rank, sigil).expect("test card is valid")
}

fn command_for_card(
    actor_index: usize,
    card: CardId,
    freshness_token: FreshnessToken,
) -> CommandEnvelope {
    command(actor_index, commit_segment(card), freshness_token)
}

fn apply_card(
    state: &mut high_card_duel::HighCardDuelState,
    actor_index: usize,
    card: CardId,
) -> Vec<engine_core::EffectEnvelope<HighCardDuelEffect>> {
    let command = command_for_card(actor_index, card, state.freshness_token);
    let action = validate_command(state, &command).expect("command validates");
    apply_action(state, action)
}

#[test]
fn setup_same_seed_same_initial_deal_internal() {
    let options = SetupOptions::default();
    let left = setup_match(Seed(42), &seats(), &options).expect("setup succeeds");
    let right = setup_match(Seed(42), &seats(), &options).expect("setup succeeds");

    assert_eq!(left, right);
    assert_eq!(left.internal_card_order(), right.internal_card_order());
}

#[test]
fn setup_different_seeds_can_change_initial_deal() {
    let options = SetupOptions::default();
    let left = setup_match(Seed(42), &seats(), &options).expect("setup succeeds");
    let right = setup_match(Seed(43), &seats(), &options).expect("setup succeeds");

    assert_ne!(left.internal_card_order(), right.internal_card_order());
}

#[test]
fn setup_deals_private_hands_and_hides_deck() {
    let state = setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");

    assert_eq!(
        state.hand_for(HighCardDuelSeat::Seat0).len(),
        STANDARD_HAND_SIZE as usize
    );
    assert_eq!(
        state.hand_for(HighCardDuelSeat::Seat1).len(),
        STANDARD_HAND_SIZE as usize
    );
    assert_eq!(
        state.deck.len(),
        STANDARD_DECK_CARD_COUNT as usize - (STANDARD_HAND_SIZE as usize * 2)
    );
    assert!(state.commitment_for(HighCardDuelSeat::Seat0).is_none());
    assert!(state.commitment_for(HighCardDuelSeat::Seat1).is_none());

    let all_cards = state.internal_card_order();
    let unique_cards = all_cards.iter().collect::<BTreeSet<_>>();
    assert_eq!(all_cards.len(), STANDARD_DECK_CARD_COUNT as usize);
    assert_eq!(unique_cards.len(), STANDARD_DECK_CARD_COUNT as usize);

    let canonical = canonical_deck()
        .into_iter()
        .map(|card| card.stable_id())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        unique_cards.into_iter().cloned().collect::<BTreeSet<_>>(),
        canonical
    );
}

#[test]
fn setup_shuffle_uses_unbiased_bounded_index_or_documented_helper() {
    struct FixedRng {
        values: Vec<u64>,
    }

    impl engine_core::DeterministicRng for FixedRng {
        fn next_u64(&mut self) -> u64 {
            self.values.remove(0)
        }
    }

    let range = u128::from(u64::MAX) + 1;
    let accepted_zone_for_three = range - (range % 3);
    let rejected = accepted_zone_for_three as u64;
    let mut rng = FixedRng {
        values: vec![rejected, 4],
    };

    assert_eq!(next_bounded_index_unbiased(&mut rng, 3), Some(1));
}

#[test]
fn observer_has_no_private_commit_actions() {
    let state = setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let unknown_actor = Actor {
        seat_id: SeatId("observer".to_owned()),
    };

    let tree = legal_action_tree(&state, &unknown_actor);

    assert_eq!(tree.freshness_token, state.freshness_token);
    assert!(tree.root.choices.is_empty());
}

#[test]
fn actor_private_tree_names_only_own_cards() {
    let state = setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");

    let seat_0_tree = legal_action_tree(&state, &actor(0));
    let seat_1_tree = legal_action_tree(&state, &actor(1));

    assert_eq!(seat_0_tree.root.choices.len(), STANDARD_HAND_SIZE as usize);
    assert!(seat_1_tree.root.choices.is_empty());
    for choice in &seat_0_tree.root.choices {
        assert!(state
            .hand_for(HighCardDuelSeat::Seat0)
            .iter()
            .any(|card| choice.segment == commit_segment(*card)));
        assert!(!state
            .hand_for(HighCardDuelSeat::Seat1)
            .iter()
            .any(|card| choice.label.contains(&card.stable_id())));
    }
}

#[test]
fn wrong_seat_diagnostic_public_safe() {
    let state = setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let card = state.hand_for(HighCardDuelSeat::Seat1)[0];

    let diagnostic = validate_command(
        &state,
        &command(1, commit_segment(card), state.freshness_token),
    )
    .expect_err("wrong seat rejected");

    assert_eq!(diagnostic.code, "wrong_seat");
    assert!(!diagnostic.message.contains("hcd:r"));
}

#[test]
fn wrong_phase_diagnostic_public_safe() {
    let mut state =
        setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    state.phase = Phase::Revealed;
    let card = state.hand_for(HighCardDuelSeat::Seat0)[0];

    let diagnostic = validate_command(
        &state,
        &command(0, commit_segment(card), state.freshness_token),
    )
    .expect_err("wrong phase rejected");

    assert_eq!(diagnostic.code, "wrong_phase");
    assert!(!diagnostic.message.contains("hcd:r"));
}

#[test]
fn invalid_private_card_diagnostic_redacted_for_unauthorized() {
    let state = setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let opponent_card = state.hand_for(HighCardDuelSeat::Seat1)[0];

    let diagnostic = validate_command(
        &state,
        &command(0, commit_segment(opponent_card), state.freshness_token),
    )
    .expect_err("opponent private card rejected");

    assert_eq!(diagnostic.code, "invalid_private_card");
    assert!(!diagnostic.message.contains(&opponent_card.stable_id()));
    assert!(!diagnostic.message.contains("hcd:r"));
}

#[test]
fn stale_action_diagnostic_no_hidden_leak() {
    let state = setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let card = state.hand_for(HighCardDuelSeat::Seat0)[0];

    let diagnostic = validate_command(
        &state,
        &command(0, commit_segment(card), FreshnessToken(99)),
    )
    .expect_err("stale action rejected");

    assert_eq!(diagnostic.code, "stale_action");
    assert!(!diagnostic.message.contains(&card.stable_id()));
    assert!(!diagnostic.message.contains("hcd:r"));
}

#[test]
fn lead_commit_removes_card_from_own_hand() {
    let mut state =
        setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let lead_card = state.hand_for(HighCardDuelSeat::Seat0)[0];

    let effects = apply_card(&mut state, 0, lead_card);

    assert_eq!(state.phase, Phase::ReplyCommit);
    assert_eq!(
        state.commitment_for(HighCardDuelSeat::Seat0),
        Some(lead_card)
    );
    assert!(!state.hand_for(HighCardDuelSeat::Seat0).contains(&lead_card));
    assert!(effects
        .iter()
        .any(|effect| effect.payload.kind() == "hcd_commit_face_down"));
    assert!(effects
        .iter()
        .any(|effect| effect.payload.kind() == "hcd_own_commit_confirmed"));
}

#[test]
fn reply_commit_cannot_see_lead_identity() {
    let mut state =
        setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let lead_card = state.hand_for(HighCardDuelSeat::Seat0)[0];

    let effects = apply_card(&mut state, 0, lead_card);

    assert_eq!(state.phase, Phase::ReplyCommit);
    assert!(effects
        .iter()
        .filter(|effect| matches!(effect.visibility, engine_core::VisibilityScope::Public))
        .all(|effect| !effect
            .payload
            .public_payload_text()
            .contains(&lead_card.stable_id())));
}

#[test]
fn both_commitments_reveal_together() {
    let mut state =
        setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    state.hands = [
        vec![card(4, Sigil::A), card(6, Sigil::A), card(8, Sigil::A)],
        vec![card(9, Sigil::B), card(3, Sigil::B), card(5, Sigil::B)],
    ];
    state.deck = vec![card(1, Sigil::A), card(2, Sigil::A)];
    let lead_card = state.hands[0][0];
    let reply_card = state.hands[1][0];

    apply_card(&mut state, 0, lead_card);
    let effects = apply_card(&mut state, 1, reply_card);

    assert_eq!(state.revealed_history.len(), 1);
    let reveal = effects
        .iter()
        .find(|effect| effect.payload.kind() == "hcd_cards_revealed")
        .expect("reveal effect emitted");
    assert!(reveal
        .payload
        .public_payload_text()
        .contains(&lead_card.stable_id()));
    assert!(reveal
        .payload
        .public_payload_text()
        .contains(&reply_card.stable_id()));
}

#[test]
fn higher_rank_scores_one_point() {
    let mut state =
        setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    state.hands = [
        vec![card(11, Sigil::A), card(6, Sigil::A), card(8, Sigil::A)],
        vec![card(9, Sigil::B), card(3, Sigil::B), card(5, Sigil::B)],
    ];
    state.deck = vec![card(1, Sigil::A), card(2, Sigil::A)];

    apply_card(&mut state, 0, card(11, Sigil::A));
    apply_card(&mut state, 1, card(9, Sigil::B));

    assert_eq!(state.score.seat_0, 1);
    assert_eq!(state.score.seat_1, 0);
    assert_eq!(
        state.revealed_history[0].winner,
        Some(HighCardDuelSeat::Seat0)
    );
}

#[test]
fn tie_round_scores_no_points() {
    let mut state =
        setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    state.hands = [
        vec![card(7, Sigil::A), card(6, Sigil::A), card(8, Sigil::A)],
        vec![card(7, Sigil::B), card(3, Sigil::B), card(5, Sigil::B)],
    ];
    state.deck = vec![card(1, Sigil::A), card(2, Sigil::A)];

    apply_card(&mut state, 0, card(7, Sigil::A));
    apply_card(&mut state, 1, card(7, Sigil::B));

    assert_eq!(state.score.seat_0, 0);
    assert_eq!(state.score.seat_1, 0);
    assert_eq!(state.revealed_history[0].winner, None);
}

#[test]
fn refill_restores_hand_size_when_deck_available() {
    let mut state =
        setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    state.hands = [
        vec![card(4, Sigil::A), card(6, Sigil::A), card(8, Sigil::A)],
        vec![card(9, Sigil::B), card(3, Sigil::B), card(5, Sigil::B)],
    ];
    state.deck = vec![card(1, Sigil::A), card(2, Sigil::A)];

    apply_card(&mut state, 0, card(4, Sigil::A));
    let effects = apply_card(&mut state, 1, card(9, Sigil::B));

    assert_eq!(state.hand_for(HighCardDuelSeat::Seat0).len(), 3);
    assert_eq!(state.hand_for(HighCardDuelSeat::Seat1).len(), 3);
    assert_eq!(state.deck.len(), 0);
    assert!(effects
        .iter()
        .any(|effect| effect.payload.kind() == "hcd_refill_started"));
    assert!(effects
        .iter()
        .any(|effect| effect.payload.kind() == "hcd_hand_count_changed"));
}

#[test]
fn lead_alternates_by_round() {
    let mut state =
        setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    state.hands = [
        vec![card(4, Sigil::A), card(6, Sigil::A), card(8, Sigil::A)],
        vec![card(9, Sigil::B), card(3, Sigil::B), card(5, Sigil::B)],
    ];
    state.deck = vec![card(1, Sigil::A), card(2, Sigil::A)];

    apply_card(&mut state, 0, card(4, Sigil::A));
    apply_card(&mut state, 1, card(9, Sigil::B));

    assert_eq!(state.round_number, 2);
    assert_eq!(state.lead_seat, HighCardDuelSeat::Seat1);
    assert_eq!(state.phase, Phase::LeadCommit);
}

#[test]
fn terminal_after_six_rounds() {
    let mut state =
        setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    state.round_number = 6;
    state.lead_seat = HighCardDuelSeat::Seat1;
    state.hands = [
        vec![card(4, Sigil::A), card(6, Sigil::A), card(8, Sigil::A)],
        vec![card(9, Sigil::B), card(3, Sigil::B), card(5, Sigil::B)],
    ];
    state.deck = vec![card(1, Sigil::A), card(2, Sigil::A)];

    apply_card(&mut state, 1, card(9, Sigil::B));
    let effects = apply_card(&mut state, 0, card(4, Sigil::A));

    assert_eq!(state.phase, Phase::Terminal);
    assert_eq!(
        state.terminal_outcome,
        Some(TerminalOutcome::Win {
            seat: HighCardDuelSeat::Seat1
        })
    );
    assert!(effects
        .iter()
        .any(|effect| effect.payload.kind() == "hcd_terminal"));
}

#[test]
fn terminal_winner_and_draw_policy() {
    let mut state =
        setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    state.round_number = 6;
    state.hands = [
        vec![card(7, Sigil::A), card(6, Sigil::A), card(8, Sigil::A)],
        vec![card(7, Sigil::B), card(3, Sigil::B), card(5, Sigil::B)],
    ];
    state.deck = vec![card(1, Sigil::A), card(2, Sigil::A)];

    apply_card(&mut state, 0, card(7, Sigil::A));
    apply_card(&mut state, 1, card(7, Sigil::B));

    assert_eq!(state.phase, Phase::Terminal);
    assert_eq!(state.terminal_outcome, Some(TerminalOutcome::Draw));
}
