use engine_core::{ActionPath, ActionTree, CommandEnvelope, RulesVersion, SeatId, Seed};
use poker_lite::{
    action_from_decision, actor_for_seat, apply_action, legal_action_tree, setup_match,
    validate_command, BotDecision, Phase, PokerLiteAction, PokerLiteLevel2Bot, PokerLiteRandomBot,
    PokerLiteSeat, PokerLiteState, SetupOptions, LEVEL2_POLICY_ID,
};

const ACTION_CAP: usize = 16;

fn standard_state(seed: u64) -> PokerLiteState {
    setup_match(
        Seed(seed),
        &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn command(
    state: &PokerLiteState,
    seat: PokerLiteSeat,
    action_path: ActionPath,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_seat(state, seat),
        action_path,
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn assert_legal_decision(state: &PokerLiteState, seat: PokerLiteSeat, action_path: &ActionPath) {
    let tree = legal_action_tree(state, &actor_for_seat(state, seat));
    assert!(legal_paths(&tree).contains(action_path));

    let envelope = command(state, seat, action_path.clone());
    validate_command(state, &envelope).expect("bot decision validates through command API");
}

fn legal_paths(tree: &ActionTree) -> Vec<ActionPath> {
    tree.root
        .choices
        .iter()
        .map(|choice| ActionPath {
            segments: vec![choice.segment.clone()],
        })
        .collect()
}

fn apply_path(state: &mut PokerLiteState, seat: PokerLiteSeat, action_path: ActionPath) {
    let envelope = command(state, seat, action_path);
    let validated = validate_command(state, &envelope).expect("command validates");
    apply_action(state, validated).expect("action applies");
}

#[test]
fn random_and_level2_decisions_are_legal_and_do_not_mutate_state() {
    let state = standard_state(11);
    let before = state.clone();
    let random_decision = PokerLiteRandomBot::new(Seed(17))
        .select_decision(&state, PokerLiteSeat::Seat0)
        .expect("random decision");
    let level2_decision = PokerLiteLevel2Bot::new(Seed(29))
        .select_decision(&state, PokerLiteSeat::Seat0)
        .expect("level2 decision");

    assert_legal_decision(&state, PokerLiteSeat::Seat0, &random_decision.action_path);
    assert_legal_decision(&state, PokerLiteSeat::Seat0, &level2_decision.action_path);
    assert_eq!(state, before);
}

#[test]
fn seeded_bots_are_deterministic_on_same_allowed_state() {
    let state = standard_state(0);
    let random = PokerLiteRandomBot::new(Seed(42));
    let level2 = PokerLiteLevel2Bot::new(Seed(42));

    assert_eq!(
        random
            .select_decision(&state, PokerLiteSeat::Seat0)
            .expect("first random decision"),
        random
            .select_decision(&state, PokerLiteSeat::Seat0)
            .expect("second random decision")
    );
    assert_eq!(
        level2
            .select_decision(&state, PokerLiteSeat::Seat0)
            .expect("first level2 decision"),
        level2
            .select_decision(&state, PokerLiteSeat::Seat0)
            .expect("second level2 decision")
    );
}

#[test]
fn level2_input_whitelist_excludes_forbidden_hidden_material() {
    let state = standard_state(0);
    let input = PokerLiteLevel2Bot::input_for(&state, PokerLiteSeat::Seat0);
    let summary = input.stable_summary();

    assert!(summary.contains("own_rank=high"));
    assert!(summary.contains("center_rank=hidden"));
    for card in state.private_cards_internal() {
        assert!(!summary.contains(card.as_str()));
        assert!(!summary.contains(&card.label()));
    }
    assert!(!summary.contains(state.center_card_internal().as_str()));
    for card in state.deck_tail_internal() {
        assert!(!summary.contains(card.as_str()));
    }
    assert!(!summary.contains("seed"));
    assert!(!summary.contains("opponent"));
}

#[test]
fn level2_policy_uses_authored_priority_and_stable_tie_break() {
    let state = standard_state(0);
    let opening = PokerLiteLevel2Bot::new(Seed(1))
        .select_decision(&state, PokerLiteSeat::Seat0)
        .expect("opening decision");
    assert_eq!(action_from_decision(&opening), Some(PokerLiteAction::Press));

    let mut facing_with_pair = standard_state(0);
    apply_path(
        &mut facing_with_pair,
        PokerLiteSeat::Seat0,
        ActionPath {
            segments: vec!["hold".to_owned()],
        },
    );
    apply_path(
        &mut facing_with_pair,
        PokerLiteSeat::Seat1,
        ActionPath {
            segments: vec!["hold".to_owned()],
        },
    );
    apply_path(
        &mut facing_with_pair,
        PokerLiteSeat::Seat1,
        ActionPath {
            segments: vec!["press".to_owned()],
        },
    );

    let pair_response = PokerLiteLevel2Bot::new(Seed(99))
        .select_decision(&facing_with_pair, PokerLiteSeat::Seat0)
        .expect("pair response");
    assert_eq!(
        action_from_decision(&pair_response),
        Some(PokerLiteAction::Match)
    );
    assert!(pair_response.rationale.contains("made public pair"));
}

#[test]
fn bot_explanations_and_effects_do_not_leak_hidden_cards_or_sampling_claims() {
    let state = standard_state(0);
    let decision = PokerLiteLevel2Bot::new(Seed(1))
        .select_decision(&state, PokerLiteSeat::Seat0)
        .expect("level2 decision");
    assert_eq!(decision.policy_id, LEVEL2_POLICY_ID);
    assert_no_forbidden_bot_text(&decision, &state);

    let public_effect_text = format!("{:?}", decision.effects[0]);
    assert!(public_effect_text.contains(LEVEL2_POLICY_ID));
    assert!(public_effect_text.contains("BotChoseActionPublic"));
    assert!(!public_effect_text.contains("private"));
    assert!(!public_effect_text.contains("high_dawn"));

    let private_effect_text = format!("{:?}", decision.effects[1]);
    assert!(private_effect_text.contains("high_private"));
    assert!(!private_effect_text.contains("high_dawn"));
    assert!(!private_effect_text.contains("middle_dawn"));
}

#[test]
fn level2_bots_finish_many_games_with_legal_actions_under_cap() {
    for seed in 0..32 {
        let mut state = standard_state(seed);
        let mut turns = 0;

        while state.phase != Phase::Terminal {
            assert!(turns < ACTION_CAP, "seed {seed} exceeded action cap");
            let seat = state.active_seat.expect("non-terminal has active seat");
            let bot = PokerLiteLevel2Bot::new(Seed(seed + turns as u64));
            let decision = bot.select_decision(&state, seat).expect("bot decision");
            assert_legal_decision(&state, seat, &decision.action_path);
            apply_path(&mut state, seat, decision.action_path);
            turns += 1;
        }

        assert!(turns <= ACTION_CAP);
        assert!(state.terminal_outcome.is_some());
    }
}

#[test]
fn bot_action_golden_trace_names_policy_and_action_without_hidden_material() {
    let trace = include_str!("golden_traces/bot-action.trace.json");

    assert!(trace.contains("\"trace_id\": \"poker-lite-bot-action\""));
    assert!(trace.contains(LEVEL2_POLICY_ID));
    assert!(trace.contains("\"expected_bot_action\": \"press\""));
    assert!(!trace.contains("high_dawn"));
    assert!(!trace.contains("middle_dawn"));
}

fn assert_no_forbidden_bot_text(decision: &BotDecision, state: &PokerLiteState) {
    let text = format!("{decision:?}").to_lowercase();
    for forbidden in [
        "opponent card",
        "deck tail",
        "hidden center",
        "sample",
        "mcts",
        "monte carlo",
        "machine learning",
        "reinforcement",
        "rollout",
    ] {
        assert!(!text.contains(forbidden), "{text}");
    }
    for card in state.private_cards_internal() {
        assert!(!text.contains(card.as_str()), "{text}");
        assert!(!text.contains(&card.label().to_lowercase()), "{text}");
    }
    assert!(
        !text.contains(state.center_card_internal().as_str()),
        "{text}"
    );
    for card in state.deck_tail_internal() {
        assert!(!text.contains(card.as_str()), "{text}");
    }
}
