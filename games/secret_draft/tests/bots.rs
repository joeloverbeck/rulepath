use engine_core::{ActionPath, ActionTree, CommandEnvelope, RulesVersion, SeatId, Seed};
use secret_draft::{
    actions::{commit_segment, legal_action_tree, validate_command},
    apply_action,
    bots::{action_from_decision, actor_for_seat},
    setup_match, DraftItemId, Phase, SecretDraftAction, SecretDraftLevel1Bot, SecretDraftRandomBot,
    SecretDraftSeat, SetupOptions,
};

fn standard_state() -> secret_draft::SecretDraftState {
    setup_match(
        &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn command(
    state: &secret_draft::SecretDraftState,
    seat: SecretDraftSeat,
    action_path: ActionPath,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_seat(state, seat),
        action_path,
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn assert_legal_decision(
    state: &secret_draft::SecretDraftState,
    seat: SecretDraftSeat,
    action_path: &ActionPath,
) {
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

fn apply_path(
    state: &mut secret_draft::SecretDraftState,
    seat: SecretDraftSeat,
    action_path: ActionPath,
) {
    let envelope = command(state, seat, action_path);
    let validated = validate_command(state, &envelope).expect("command validates");
    apply_action(state, validated).expect("action applies");
}

#[test]
fn random_and_level1_decisions_are_legal_and_do_not_mutate_state() {
    let state = standard_state();
    let before = state.clone();
    let random_decision = SecretDraftRandomBot::new(Seed(17))
        .select_decision(&state, SecretDraftSeat::Seat0)
        .expect("random decision");
    let level1_decision = SecretDraftLevel1Bot::new(Seed(29))
        .select_decision(&state, SecretDraftSeat::Seat1)
        .expect("level1 decision");

    assert_legal_decision(&state, SecretDraftSeat::Seat0, &random_decision.action_path);
    assert_legal_decision(&state, SecretDraftSeat::Seat1, &level1_decision.action_path);
    assert_eq!(state, before);
}

#[test]
fn seeded_bots_are_deterministic_on_same_public_state() {
    let state = standard_state();
    let random = SecretDraftRandomBot::new(Seed(42));
    let level1 = SecretDraftLevel1Bot::new(Seed(42));

    assert_eq!(
        random
            .select_decision(&state, SecretDraftSeat::Seat0)
            .expect("first random decision"),
        random
            .select_decision(&state, SecretDraftSeat::Seat0)
            .expect("second random decision")
    );
    assert_eq!(
        level1
            .select_decision(&state, SecretDraftSeat::Seat0)
            .expect("first level1 decision"),
        level1
            .select_decision(&state, SecretDraftSeat::Seat0)
            .expect("second level1 decision")
    );
}

#[test]
fn level1_uses_only_public_information_when_opponent_commitment_differs() {
    let mut ember_hidden = standard_state();
    let mut grove_hidden = standard_state();

    apply_path(
        &mut ember_hidden,
        SecretDraftSeat::Seat0,
        ActionPath {
            segments: vec![commit_segment(DraftItemId::Ember4)],
        },
    );
    apply_path(
        &mut grove_hidden,
        SecretDraftSeat::Seat0,
        ActionPath {
            segments: vec![commit_segment(DraftItemId::Grove4)],
        },
    );

    assert!(ember_hidden.seat_committed(SecretDraftSeat::Seat0));
    assert!(grove_hidden.seat_committed(SecretDraftSeat::Seat0));
    assert_eq!(ember_hidden.visible_pool, grove_hidden.visible_pool);
    assert_eq!(
        ember_hidden.seat_committed(SecretDraftSeat::Seat0),
        grove_hidden.seat_committed(SecretDraftSeat::Seat0)
    );

    let bot = SecretDraftLevel1Bot::new(Seed(3));
    let ember_decision = bot
        .select_decision(&ember_hidden, SecretDraftSeat::Seat1)
        .expect("decision with ember hidden");
    let grove_decision = bot
        .select_decision(&grove_hidden, SecretDraftSeat::Seat1)
        .expect("decision with grove hidden");

    assert_eq!(ember_decision, grove_decision);
    assert!(!ember_decision.rationale.contains("ember_4"));
    assert!(!ember_decision.rationale.contains("grove_4"));
    assert!(!format!("{ember_decision:?}").contains("Ember4"));
    assert!(!format!("{grove_decision:?}").contains("Grove4"));
}

#[test]
fn bot_rationales_do_not_claim_hidden_or_sampled_information() {
    let state = standard_state();
    let decisions = [
        SecretDraftRandomBot::new(Seed(1))
            .select_decision(&state, SecretDraftSeat::Seat0)
            .expect("random decision"),
        SecretDraftLevel1Bot::new(Seed(1))
            .select_decision(&state, SecretDraftSeat::Seat0)
            .expect("level1 decision"),
    ];

    for decision in decisions {
        let rationale = decision.rationale.to_lowercase();
        assert!(!rationale.contains("hidden"));
        assert!(!rationale.contains("opponent chose"));
        assert!(!rationale.contains("sample"));
        assert!(!rationale.contains("mcts"));
        assert!(!rationale.contains("monte carlo"));
        assert!(!rationale.contains("machine learning"));
        assert!(!rationale.contains("llm"));
    }
}

#[test]
fn level1_bots_finish_many_games_with_legal_actions() {
    for game_index in 0..20 {
        let mut state = standard_state();
        let seat_0_bot = SecretDraftLevel1Bot::new(Seed(100 + game_index));
        let seat_1_bot = SecretDraftLevel1Bot::new(Seed(200 + game_index));
        let mut turns = 0;

        while state.phase != Phase::Terminal {
            for seat in SecretDraftSeat::ALL {
                if state.phase == Phase::Terminal || state.seat_committed(seat) {
                    continue;
                }

                let decision = match seat {
                    SecretDraftSeat::Seat0 => seat_0_bot
                        .select_decision(&state, seat)
                        .expect("seat 0 decision"),
                    SecretDraftSeat::Seat1 => seat_1_bot
                        .select_decision(&state, seat)
                        .expect("seat 1 decision"),
                };
                assert_legal_decision(&state, seat, &decision.action_path);
                let action = action_from_decision(&decision, seat).expect("action parses");
                assert_eq!(
                    action,
                    SecretDraftAction {
                        actor: seat,
                        item: action.item
                    }
                );
                apply_path(&mut state, seat, decision.action_path);
                turns += 1;
            }
        }

        assert_eq!(turns, 12);
        assert!(state.terminal_outcome.is_some());
        assert_eq!(state.visible_pool.len(), 0);
    }
}
