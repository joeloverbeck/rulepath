use engine_core::{ActionPath, ActionTree, CommandEnvelope, RulesVersion, SeatId, Seed};
use river_ledger::{
    action_from_decision, actor_for_bot_seat, apply_action, legal_action_tree, setup_match,
    validate_command, BotDecision, Phase, RiverLedgerAction, RiverLedgerLevel1Bot,
    RiverLedgerLevel2Bot, RiverLedgerRandomBot, RiverLedgerSeat, SetupOptions, LEVEL2_POLICY_ID,
};

const ACTION_CAP: usize = 96;

fn seats(count: usize) -> Vec<SeatId> {
    (0..count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect()
}

fn standard_state(seed: u64, count: usize) -> river_ledger::RiverLedgerState {
    setup_match(Seed(seed), &seats(count), &SetupOptions::default()).expect("setup")
}

fn command(
    state: &river_ledger::RiverLedgerState,
    seat: RiverLedgerSeat,
    action_path: ActionPath,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_bot_seat(state, seat),
        action_path,
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
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

fn assert_legal_decision(
    state: &river_ledger::RiverLedgerState,
    seat: RiverLedgerSeat,
    action_path: &ActionPath,
) {
    let tree = legal_action_tree(state, &actor_for_bot_seat(state, seat));
    assert!(legal_paths(&tree).contains(action_path));

    let envelope = command(state, seat, action_path.clone());
    validate_command(state, &envelope).expect("bot decision validates through command API");
}

fn apply_path(
    state: &mut river_ledger::RiverLedgerState,
    seat: RiverLedgerSeat,
    action_path: ActionPath,
) {
    let envelope = command(state, seat, action_path);
    let validated = validate_command(state, &envelope).expect("command validates");
    apply_action(state, validated).expect("action applies");
}

#[test]
fn random_l1_and_l2_decisions_are_legal_and_do_not_mutate_state() {
    let state = standard_state(13, 4);
    let before = state.clone();
    let seat = state.active_seat.expect("setup active");

    let random = RiverLedgerRandomBot::new(Seed(1))
        .select_decision(&state, seat)
        .expect("random decision");
    let level1 = RiverLedgerLevel1Bot::new(Seed(2))
        .select_decision(&state, seat)
        .expect("level1 decision");
    let level2 = RiverLedgerLevel2Bot::new(Seed(3))
        .select_decision(&state, seat)
        .expect("level2 decision");

    assert_legal_decision(&state, seat, &random.action_path);
    assert_legal_decision(&state, seat, &level1.action_path);
    assert_legal_decision(&state, seat, &level2.action_path);
    assert_eq!(state, before);
}

#[test]
fn seeded_bots_are_deterministic_on_same_allowed_state() {
    for count in 3..=6 {
        let state = standard_state(17, count);
        let seat = state.active_seat.expect("setup active");
        let random = RiverLedgerRandomBot::new(Seed(42));
        let level2 = RiverLedgerLevel2Bot::new(Seed(42));

        assert_eq!(
            random.select_decision(&state, seat).expect("first random"),
            random.select_decision(&state, seat).expect("second random")
        );
        assert_eq!(
            level2.select_decision(&state, seat).expect("first level2"),
            level2.select_decision(&state, seat).expect("second level2")
        );
    }
}

#[test]
fn level2_input_whitelist_excludes_forbidden_hidden_material() {
    let state = standard_state(19, 6);
    let seat = state.active_seat.expect("setup active");
    let input = RiverLedgerLevel2Bot::input_for(&state, seat);
    let summary = input.stable_summary().to_lowercase();

    assert!(summary.contains("own_bucket="));
    assert!(summary.contains("live_opponents="));
    assert!(!summary.contains("seed"));
    assert!(!summary.contains("opponent hole"));
    for forbidden in hidden_card_ids(&state) {
        assert!(!summary.contains(&forbidden), "{summary}");
    }
}

#[test]
fn level2_policy_uses_authored_priority_and_stable_tie_break() {
    let state = standard_state(0, 4);
    let seat = state.active_seat.expect("setup active");
    let decision = RiverLedgerLevel2Bot::new(Seed(1))
        .select_decision(&state, seat)
        .expect("level2 decision");

    assert!(matches!(
        action_from_decision(&decision),
        Some(RiverLedgerAction::Fold | RiverLedgerAction::Call | RiverLedgerAction::Raise)
    ));
    assert!(decision.rationale.contains("own authorized"));
    assert!(decision.rationale.contains("public price"));
}

#[test]
fn bot_explanations_do_not_leak_hidden_cards_or_sampling_claims() {
    let state = standard_state(23, 4);
    let seat = state.active_seat.expect("setup active");
    let decision = RiverLedgerLevel2Bot::new(Seed(1))
        .select_decision(&state, seat)
        .expect("level2 decision");

    assert_eq!(decision.policy_id, LEVEL2_POLICY_ID);
    assert_no_forbidden_bot_text(&decision, &state);
}

#[test]
fn level2_bots_finish_seeded_games_with_legal_actions_under_cap() {
    for count in 3..=6 {
        for seed in 0..8 {
            let mut state = standard_state(seed, count);
            let mut turns = 0;

            while state.phase != Phase::Terminal {
                assert!(
                    turns < ACTION_CAP,
                    "{count}-seat seed {seed} exceeded action cap"
                );
                let seat = state.active_seat.expect("non-terminal has active seat");
                let bot = RiverLedgerLevel2Bot::new(Seed(seed + turns as u64));
                let decision = bot.select_decision(&state, seat).expect("bot decision");
                assert_legal_decision(&state, seat, &decision.action_path);
                assert_no_forbidden_bot_text(&decision, &state);
                apply_path(&mut state, seat, decision.action_path);
                turns += 1;
            }

            assert!(turns <= ACTION_CAP);
            assert!(state.terminal_outcome.is_some());
        }
    }
}

#[test]
fn bot_golden_trace_names_policy_without_hidden_material() {
    let trace = include_str!("golden_traces/bot-vs-bot-full-game-6p.trace.json");

    assert!(trace.contains("\"trace_id\": \"river-ledger-bot-vs-bot-full-game-6p\""));
    assert!(trace.contains(LEVEL2_POLICY_ID));
    assert!(trace.contains("\"forbidden_policy_classes\""));
}

fn hidden_card_ids(state: &river_ledger::RiverLedgerState) -> Vec<String> {
    state
        .private_hands_internal()
        .iter()
        .flatten()
        .chain(state.community_deck_internal().iter())
        .chain(state.deck_tail_internal().iter())
        .map(|card| card.id())
        .collect()
}

fn assert_no_forbidden_bot_text(decision: &BotDecision, state: &river_ledger::RiverLedgerState) {
    let text = format!("{decision:?}").to_lowercase();
    for forbidden in [
        "opponent card",
        "opponent hole",
        "deck tail",
        "future community",
        "sample",
        "mcts",
        "monte carlo",
        "machine learning",
        "reinforcement",
        "rollout",
        "solver",
    ] {
        assert!(!text.contains(forbidden), "{text}");
    }
    for card in hidden_card_ids(state) {
        assert!(!text.contains(&card), "{text}");
    }
}
