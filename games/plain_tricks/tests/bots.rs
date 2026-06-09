use engine_core::{ActionPath, ActionTree, CommandEnvelope, RulesVersion, SeatId, Seed, Viewer};
use plain_tricks::{
    action_from_decision, actor_for_seat, apply_action, legal_action_tree, project_view,
    setup_match, validate_command, BotDecision, Phase, PlainTricksAction, PlainTricksLevel2Bot,
    PlainTricksRandomBot, PlainTricksSeat, PlainTricksState, PrivateView, SetupOptions,
    TrickCardId, ACTION_PLAY, LEVEL2_POLICY_ID, STANDARD_MAX_PLAYS,
};

fn standard_state(seed: u64) -> PlainTricksState {
    setup_match(
        Seed(seed),
        &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn command(
    state: &PlainTricksState,
    seat: PlainTricksSeat,
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
    state: &PlainTricksState,
    seat: PlainTricksSeat,
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
        .flat_map(|choice| {
            choice.next.as_ref().into_iter().flat_map(move |node| {
                node.choices.iter().map(move |card| ActionPath {
                    segments: vec![choice.segment.clone(), card.segment.clone()],
                })
            })
        })
        .collect()
}

fn apply_path(state: &mut PlainTricksState, seat: PlainTricksSeat, action_path: ActionPath) {
    let envelope = command(state, seat, action_path);
    let validated = validate_command(state, &envelope).expect("command validates");
    apply_action(state, validated).expect("action applies");
}

#[test]
fn random_and_level2_decisions_are_legal_and_do_not_mutate_state() {
    let state = standard_state(0);
    let before = state.clone();
    let random_decision = PlainTricksRandomBot::new(Seed(17))
        .select_decision(&state, PlainTricksSeat::Seat0)
        .expect("random decision");
    let level2_decision = PlainTricksLevel2Bot::new(Seed(29))
        .select_decision(&state, PlainTricksSeat::Seat0)
        .expect("level2 decision");

    assert_legal_decision(&state, PlainTricksSeat::Seat0, &random_decision.action_path);
    assert_legal_decision(&state, PlainTricksSeat::Seat0, &level2_decision.action_path);
    assert_eq!(state, before);
}

#[test]
fn seeded_bots_are_deterministic_on_same_allowed_state() {
    let state = standard_state(0);
    let random = PlainTricksRandomBot::new(Seed(42));
    let level2 = PlainTricksLevel2Bot::new(Seed(42));

    assert_eq!(
        random
            .select_decision(&state, PlainTricksSeat::Seat0)
            .expect("first random decision"),
        random
            .select_decision(&state, PlainTricksSeat::Seat0)
            .expect("second random decision")
    );
    assert_eq!(
        level2
            .select_decision(&state, PlainTricksSeat::Seat0)
            .expect("first level2 decision"),
        level2
            .select_decision(&state, PlainTricksSeat::Seat0)
            .expect("second level2 decision")
    );
}

#[test]
fn level2_input_whitelist_excludes_forbidden_hidden_material() {
    let state = standard_state(0);
    let input = PlainTricksLevel2Bot::input_for(&state, PlainTricksSeat::Seat0);
    let summary = input.stable_summary();
    let own = own_hand(&state, PlainTricksSeat::Seat0);
    let opponent = own_hand(&state, PlainTricksSeat::Seat1);
    let tail = tail_cards(&state);

    assert!(own.iter().any(|card| summary.contains(card.as_str())));
    for card in opponent.into_iter().chain(tail) {
        assert!(!summary.contains(card.as_str()), "{summary}");
        assert!(!summary.contains(&card.label()), "{summary}");
    }
    assert!(!summary.contains("seed"));
    assert!(!summary.contains("tail"));
    assert!(!summary.contains("opponent"));
    assert!(!summary.to_ascii_lowercase().contains("void"));
}

#[test]
fn level2_policy_uses_authored_priority_and_stable_tie_break() {
    let state = standard_state(0);
    let opening = PlainTricksLevel2Bot::new(Seed(1))
        .select_decision(&state, PlainTricksSeat::Seat0)
        .expect("opening decision");
    assert_eq!(
        action_from_decision(&opening),
        Some(PlainTricksAction {
            card: TrickCardId::River5
        })
    );
    assert!(opening.rationale.contains("likely winning lead"));

    let mut follow = standard_state(0);
    apply_path(
        &mut follow,
        PlainTricksSeat::Seat0,
        ActionPath {
            segments: vec![
                ACTION_PLAY.to_owned(),
                TrickCardId::Gale1.as_str().to_owned(),
            ],
        },
    );
    let response = PlainTricksLevel2Bot::new(Seed(99))
        .select_decision(&follow, PlainTricksSeat::Seat1)
        .expect("follow decision");
    assert_eq!(
        action_from_decision(&response),
        Some(PlainTricksAction {
            card: TrickCardId::Gale2
        })
    );
    assert!(response.rationale.contains("can win the led suit cheaply"));
}

#[test]
fn bot_explanations_and_effects_do_not_leak_hidden_cards_or_sampling_claims() {
    let state = standard_state(0);
    let decision = PlainTricksLevel2Bot::new(Seed(1))
        .select_decision(&state, PlainTricksSeat::Seat0)
        .expect("level2 decision");
    assert_eq!(decision.policy_id, LEVEL2_POLICY_ID);
    assert_no_forbidden_bot_text(&decision, &state, PlainTricksSeat::Seat0);

    let public_effect_text = format!("{:?}", decision.effects);
    assert!(public_effect_text.contains(LEVEL2_POLICY_ID));
    assert!(public_effect_text.contains("BotChoseActionPublic"));
    assert!(public_effect_text.contains("action_family"));
    assert!(!public_effect_text.contains(TrickCardId::River5.as_str()));
}

#[test]
fn level2_bots_finish_many_games_with_legal_actions_under_cap() {
    for seed in 0..32 {
        let mut state = standard_state(seed);
        let mut turns = 0;

        while state.phase != Phase::Terminal {
            assert!(
                turns < STANDARD_MAX_PLAYS as usize,
                "seed {seed} exceeded action cap"
            );
            let seat = state.active_seat.expect("non-terminal has active seat");
            let bot = PlainTricksLevel2Bot::new(Seed(seed + turns as u64));
            let decision = bot.select_decision(&state, seat).expect("bot decision");
            assert_legal_decision(&state, seat, &decision.action_path);
            apply_path(&mut state, seat, decision.action_path);
            turns += 1;
        }

        assert_eq!(turns, STANDARD_MAX_PLAYS as usize);
        assert!(state.terminal_outcome.is_some());
    }
}

#[test]
fn bot_action_golden_trace_names_policy_and_action_without_hidden_material() {
    let trace = include_str!("golden_traces/bot-action.trace.json");
    let state = standard_state(0);

    assert!(trace.contains("\"trace_id\": \"plain-tricks-bot-action\""));
    assert!(trace.contains(LEVEL2_POLICY_ID));
    assert!(trace.contains("\"expected_bot_action\": \"river_5\""));
    for card in own_hand(&state, PlainTricksSeat::Seat1)
        .into_iter()
        .chain(tail_cards(&state))
    {
        assert!(!trace.contains(card.as_str()), "{trace}");
    }
    assert!(trace.contains("No opponent-holding sampling"));
}

fn own_hand(state: &PlainTricksState, seat: PlainTricksSeat) -> Vec<TrickCardId> {
    let view = project_view(
        state,
        &Viewer {
            seat_id: Some(state.seats[seat.index()].clone()),
        },
    );
    let PrivateView::Seat(private) = view.private_view else {
        panic!("seat viewer gets private view");
    };
    private
        .own_hand
        .iter()
        .map(|card| TrickCardId::parse(&card.card_id).expect("known card"))
        .collect()
}

fn tail_cards(state: &PlainTricksState) -> Vec<TrickCardId> {
    let mut hands = Vec::new();
    for seat in PlainTricksSeat::ALL {
        hands.extend(own_hand(state, seat));
    }
    TrickCardId::ALL
        .into_iter()
        .filter(|card| !hands.contains(card))
        .collect()
}

fn assert_no_forbidden_bot_text(
    decision: &BotDecision,
    state: &PlainTricksState,
    bot_seat: PlainTricksSeat,
) {
    let text = format!("{decision:?}").to_lowercase();
    for forbidden in [
        "opponent card",
        "tail",
        "hidden holding",
        "sample",
        "mcts",
        "monte carlo",
        "machine learning",
        "reinforcement",
        "rollout",
        "seed",
        "void",
    ] {
        assert!(!text.contains(forbidden), "{text}");
    }
    for card in own_hand(state, bot_seat.other())
        .into_iter()
        .chain(tail_cards(state))
    {
        assert!(!text.contains(card.as_str()), "{text}");
        assert!(!text.contains(&card.label().to_lowercase()), "{text}");
    }
}
