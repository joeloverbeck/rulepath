use briar_circuit::{
    apply_pass_action, apply_play_action, canonical_seat_ids, legal_bot_actions, setup_match,
    BriarCircuitBotAction, BriarCircuitL0Bot, BriarCircuitL1Bot, BriarCircuitSeat, Card,
    CurrentTrick, PassAction, Phase, PlayAction, PlayingTrickState, Rank, SetupOptions, Suit,
};
use engine_core::Seed;

fn card(rank: Rank, suit: Suit) -> briar_circuit::CardId {
    Card::new(rank, suit).id()
}

#[test]
fn l0_bot_selects_only_legal_pass_actions_and_is_seed_deterministic() {
    let state = setup_match(Seed(1617), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");
    let seat = BriarCircuitSeat::Seat0;
    let mut first = BriarCircuitL0Bot::new(Seed(7));
    let mut second = BriarCircuitL0Bot::new(Seed(7));

    let first_decision = first.select_decision(&state, seat).expect("decision");
    let second_decision = second.select_decision(&state, seat).expect("decision");
    let legal = legal_bot_actions(&state, seat).expect("legal actions");

    assert_eq!(first_decision, second_decision);
    assert!(legal.contains(&first_decision.action));
    assert!(first_decision
        .explanation
        .starts_with("Random legal choice from "));
}

#[test]
fn l1_bot_uses_same_legal_action_api_for_pass_and_play() {
    let mut state = setup_match(Seed(1618), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");
    let seat = BriarCircuitSeat::Seat0;
    let bot = BriarCircuitL1Bot::new(Seed(8));

    for _ in 0..3 {
        let decision = bot.select_decision(&state, seat).expect("pass decision");
        assert!(legal_bot_actions(&state, seat)
            .expect("legal")
            .contains(&decision.action));
        let BriarCircuitBotAction::Pass(action) = decision.action else {
            panic!("pass phase bot must choose pass action");
        };
        apply_pass_action(&mut state, seat, action).expect("bot pass applies");
    }
    let decision = bot.select_decision(&state, seat).expect("confirm decision");
    assert_eq!(
        decision.action,
        BriarCircuitBotAction::Pass(PassAction::Confirm)
    );

    let play_card = card(Rank::Five, Suit::Clubs);
    state.private_hands = BriarCircuitSeat::ALL
        .into_iter()
        .map(|candidate| {
            if candidate == seat {
                (candidate, vec![play_card])
            } else {
                (candidate, Vec::new())
            }
        })
        .collect();
    state.phase = Phase::PlayingTrick(PlayingTrickState {
        hearts_broken: true,
        trick_index: 1,
        leader: seat,
        active_seat: seat,
        current_trick: CurrentTrick::new(seat),
    });

    let play = bot.select_decision(&state, seat).expect("play decision");
    assert_eq!(
        play.action,
        BriarCircuitBotAction::Play(PlayAction::Play(play_card))
    );
    apply_play_action(&mut state, seat, PlayAction::Play(play_card)).expect("bot play applies");
}

#[test]
fn l1_bot_decision_is_invariant_to_opponent_hidden_hand_changes() {
    let mut state = setup_match(Seed(1619), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");
    let seat = BriarCircuitSeat::Seat0;
    let own_card = card(Rank::Two, Suit::Clubs);
    state.private_hands = vec![
        (seat, vec![own_card]),
        (BriarCircuitSeat::Seat1, vec![card(Rank::Ace, Suit::Spades)]),
        (BriarCircuitSeat::Seat2, vec![card(Rank::Ace, Suit::Hearts)]),
        (
            BriarCircuitSeat::Seat3,
            vec![card(Rank::Ace, Suit::Diamonds)],
        ),
    ];
    state.phase = Phase::PlayingTrick(PlayingTrickState {
        hearts_broken: true,
        trick_index: 1,
        leader: seat,
        active_seat: seat,
        current_trick: CurrentTrick::new(seat),
    });

    let mut changed = state.clone();
    changed.private_hands[1].1 = vec![card(Rank::Three, Suit::Spades)];
    changed.private_hands[2].1 = vec![card(Rank::Three, Suit::Hearts)];
    changed.private_hands[3].1 = vec![card(Rank::Three, Suit::Diamonds)];

    let bot = BriarCircuitL1Bot::new(Seed(9));
    assert_eq!(
        bot.select_decision(&state, seat).expect("decision").action,
        bot.select_decision(&changed, seat)
            .expect("decision")
            .action
    );
}

#[test]
fn bot_explanations_do_not_name_hidden_cards_or_forbidden_methods() {
    let state = setup_match(Seed(1620), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");
    let decision = BriarCircuitL1Bot::new(Seed(10))
        .select_decision(&state, BriarCircuitSeat::Seat0)
        .expect("decision");

    for forbidden in [
        "ace_",
        "king_",
        "queen_",
        "jack_",
        "MCTS",
        "Monte Carlo",
        "ML",
        "RL",
        "opponent",
    ] {
        assert!(!decision.explanation.contains(forbidden));
    }
}
