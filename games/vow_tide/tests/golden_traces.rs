use engine_core::Seed;
use vow_tide::{
    ids::canonical_seat_ids,
    setup::{setup_match, SetupOptions, HAND_SEED_DERIVATION_V1},
    state::VowTideState,
};

#[test]
fn setup_golden_traces_match_deterministic_state() {
    let cases = [
        (
            "setup-3p-schedule-and-deal",
            3,
            include_str!("golden_traces/setup-3p-schedule-and-deal.trace.json"),
        ),
        (
            "setup-7p-schedule-and-deal",
            7,
            include_str!("golden_traces/setup-7p-schedule-and-deal.trace.json"),
        ),
        (
            "deterministic-turn-up-trump-and-hidden-tail",
            4,
            include_str!("golden_traces/deterministic-turn-up-trump-and-hidden-tail.trace.json"),
        ),
    ];

    for (trace_id, seat_count, expected) in cases {
        let state = setup_match(
            Seed(20260621),
            &canonical_seat_ids(seat_count),
            &SetupOptions::default(),
        )
        .expect("setup succeeds");

        assert_eq!(trace_json(trace_id, &state), expected);
    }
}

fn trace_json(trace_id: &str, state: &VowTideState) -> String {
    let hands = state
        .private_hands
        .iter()
        .map(|(seat, hand)| {
            format!(
                "      {{ \"seat\": \"{}\", \"cards\": [{}] }}",
                seat.as_str(),
                hand.iter()
                    .map(|card| format!("\"{}\"", card.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");
    let stock = state
        .hidden_stock_internal()
        .iter()
        .map(|card| format!("\"{}\"", card.as_str()))
        .collect::<Vec<_>>()
        .join(", ");
    let deal_order = state
        .deal_order
        .iter()
        .map(|seat| format!("\"{}\"", seat.as_str()))
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        concat!(
            "{{\n",
            "  \"trace_id\": \"{}\",\n",
            "  \"seed\": 20260621,\n",
            "  \"seed_derivation\": \"{}\",\n",
            "  \"seat_count\": {},\n",
            "  \"dealer\": \"{}\",\n",
            "  \"deal_order\": [{}],\n",
            "  \"hand_index\": {},\n",
            "  \"hand_size\": {},\n",
            "  \"schedule\": {:?},\n",
            "  \"trump_indicator\": \"{}\",\n",
            "  \"trump_suit\": \"{}\",\n",
            "  \"hidden_stock_count\": {},\n",
            "  \"hidden_stock\": [{}],\n",
            "  \"private_hands\": [\n",
            "{}\n",
            "  ]\n",
            "}}\n"
        ),
        trace_id,
        HAND_SEED_DERIVATION_V1,
        state.seat_count(),
        state.dealer.as_str(),
        deal_order,
        state.hand_index,
        state.current_hand_size().expect("hand size"),
        state.hand_schedule,
        state.trump_indicator.as_str(),
        state.trump_suit().as_str(),
        state.hidden_stock_internal().len(),
        stock,
        hands
    )
}
