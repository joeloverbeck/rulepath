use engine_core::{ActionPath, CommandEnvelope, RulesVersion, Seed};
use high_card_duel::{
    active_commit_seat, apply_action, commit_segment, setup_match, validate_command,
    HighCardDuelRandomBot, HighCardDuelSeat, Phase, SetupOptions, RANDOM_POLICY_ID,
};

fn seats() -> Vec<engine_core::SeatId> {
    vec![
        engine_core::SeatId("seat-0".to_owned()),
        engine_core::SeatId("seat-1".to_owned()),
    ]
}

#[test]
fn level0_chooses_only_legal_actions() {
    let state = setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let bot = HighCardDuelRandomBot::new(Seed(13));
    let action = bot
        .select_action(&state, HighCardDuelSeat::Seat0)
        .expect("bot chooses action");

    let legal_segments = state
        .hand_for(HighCardDuelSeat::Seat0)
        .iter()
        .map(|card| commit_segment(*card))
        .collect::<Vec<_>>();

    assert_eq!(action.segments.len(), 1);
    assert!(legal_segments.contains(&action.segments[0]));
}

#[test]
fn level0_uses_actor_private_action_tree_only() {
    let state = setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let input = HighCardDuelRandomBot::input_for(&state, HighCardDuelSeat::Seat0);
    let bot = HighCardDuelRandomBot::new(Seed(5));
    let decision = bot
        .select_decision(&state, HighCardDuelSeat::Seat0)
        .expect("bot chooses decision");

    assert_eq!(decision.policy_id, RANDOM_POLICY_ID);
    assert_eq!(decision.level, 0);
    assert!(input
        .legal_action_tree
        .root
        .choices
        .iter()
        .any(|choice| choice.segment == decision.action_path.segments[0]));
}

#[test]
fn bot_cannot_access_opponent_hand_deck_or_hidden_commitment_via_input_type() {
    let mut state =
        setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let lead_card = state.hand_for(HighCardDuelSeat::Seat0)[0];
    apply_bot_path(
        &mut state,
        HighCardDuelSeat::Seat0,
        ActionPath {
            segments: vec![commit_segment(lead_card)],
        },
    );

    let input = HighCardDuelRandomBot::input_for(&state, HighCardDuelSeat::Seat1);
    let summary = input.stable_summary();

    assert!(!summary.contains(&lead_card.stable_id()));
    for card in state.hand_for(HighCardDuelSeat::Seat0) {
        assert!(!summary.contains(&card.stable_id()));
    }
    for card in &state.deck {
        assert!(!summary.contains(&card.stable_id()));
    }
    for card in state.hand_for(HighCardDuelSeat::Seat1) {
        assert!(summary.contains(&card.stable_id()));
    }
}

#[test]
fn same_seed_policy_version_deterministic() {
    let state = setup_match(Seed(11), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let left = HighCardDuelRandomBot::new(Seed(77))
        .select_decision(&state, HighCardDuelSeat::Seat0)
        .expect("left decision");
    let right = HighCardDuelRandomBot::new(Seed(77))
        .select_decision(&state, HighCardDuelSeat::Seat0)
        .expect("right decision");

    assert_eq!(left, right);
    assert_eq!(left.policy_version, 1);
}

#[test]
fn many_seed_terminal_simulation() {
    for seed in 0..25 {
        let mut state =
            setup_match(Seed(seed), &seats(), &SetupOptions::default()).expect("setup succeeds");
        let mut turns = 0;

        while state.phase != Phase::Terminal {
            let bot_seat = active_commit_seat(&state).expect("non-terminal active seat");
            let bot = HighCardDuelRandomBot::new(Seed(seed + turns));
            let action_path = bot
                .select_action(&state, bot_seat)
                .expect("bot chooses legal action");
            apply_bot_path(&mut state, bot_seat, action_path);
            turns += 1;
            assert!(turns <= 12);
        }

        assert!(state.terminal_outcome.is_some());
    }
}

fn apply_bot_path(
    state: &mut high_card_duel::HighCardDuelState,
    seat: HighCardDuelSeat,
    action_path: ActionPath,
) {
    let command = CommandEnvelope {
        actor: high_card_duel::actor_for_seat(state, seat),
        action_path,
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    };
    let action = validate_command(state, &command).expect("bot action validates");
    apply_action(state, action);
}
