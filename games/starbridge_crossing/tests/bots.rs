use engine_core::Seed;
use starbridge_crossing::{
    legal_action_paths, legal_action_tree, parse_bot_action, setup_match, SetupOptions,
    StarbridgeCrossingL0Bot, L0_POLICY_ID,
};

fn seats(count: usize) -> Vec<engine_core::SeatId> {
    (0..count)
        .map(|index| engine_core::SeatId::from_zero_based_index(index as u32))
        .collect()
}

#[test]
fn l0_selects_deterministic_legal_actions_across_supported_counts() {
    for count in [2, 3, 4, 6] {
        let seats = seats(count);
        let state = setup_match(Seed(20), &seats, &SetupOptions::default()).unwrap();
        let actor = engine_core::Actor {
            seat_id: seats[0].clone(),
        };
        let tree = legal_action_tree(&state, &actor);
        let legal = legal_action_paths(&tree);
        let bot = StarbridgeCrossingL0Bot::new(Seed(99));

        let first = bot.select_decision(&state).unwrap();
        let second = bot.select_decision(&state).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.policy_id, L0_POLICY_ID);
        assert!(legal.contains(&first.action_path));
        assert!(first.explanation.contains("public choices"));
    }
}

#[test]
fn selected_l0_action_parses_through_starbridge_action_parser() {
    let seats = seats(2);
    let state = setup_match(Seed(20), &seats, &SetupOptions::default()).unwrap();
    let decision = StarbridgeCrossingL0Bot::new(Seed(7))
        .select_decision(&state)
        .unwrap();

    parse_bot_action(&decision.action_path).expect("bot action parses");
}

#[test]
fn bot_trace_receipt_records_l0_policy() {
    let trace = include_str!("golden_traces/bot-l0-action.trace.json");

    assert!(trace.contains(L0_POLICY_ID));
    assert!(trace.contains("\"public_no_leak\":true"));
}
