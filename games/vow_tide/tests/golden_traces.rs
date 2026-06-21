use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed};
use vow_tide::{
    actions::{legal_bids, validate_bid_command},
    ids::{canonical_seat_ids, VowTideSeat},
    rules::apply_bid,
    setup::{setup_match, SetupOptions, HAND_SEED_DERIVATION_V1},
    state::{Phase, VowTideState},
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

#[test]
fn bidding_golden_traces_match_public_bid_state() {
    let cases = [
        (
            "bidding-left-of-dealer-through-dealer",
            bidding_trace_after(&[
                (VowTideSeat::Seat1, 2),
                (VowTideSeat::Seat2, 3),
                (VowTideSeat::Seat3, 4),
                (VowTideSeat::Seat0, 0),
            ]),
            include_str!("golden_traces/bidding-left-of-dealer-through-dealer.trace.json"),
        ),
        (
            "dealer-hook-forbidden-total",
            bidding_trace_after(&[
                (VowTideSeat::Seat1, 3),
                (VowTideSeat::Seat2, 3),
                (VowTideSeat::Seat3, 3),
            ]),
            include_str!("golden_traces/dealer-hook-forbidden-total.trace.json"),
        ),
        (
            "dealer-hook-out-of-range-no-removal",
            bidding_trace_after(&[
                (VowTideSeat::Seat1, 10),
                (VowTideSeat::Seat2, 10),
                (VowTideSeat::Seat3, 10),
            ]),
            include_str!("golden_traces/dealer-hook-out-of-range-no-removal.trace.json"),
        ),
        (
            "bid-zero-accepted",
            bidding_trace_after(&[(VowTideSeat::Seat1, 0)]),
            include_str!("golden_traces/bid-zero-accepted.trace.json"),
        ),
        (
            "bid-upper-bound-accepted",
            bidding_trace_after(&[(VowTideSeat::Seat1, 10)]),
            include_str!("golden_traces/bid-upper-bound-accepted.trace.json"),
        ),
    ];

    for (trace_id, actual, expected) in cases {
        assert_eq!(actual.trace_id, trace_id);
        assert_eq!(bidding_trace_json(&actual), expected);
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct BiddingTrace {
    trace_id: String,
    phase: String,
    active_seat: Option<VowTideSeat>,
    accepted_bids: Vec<(VowTideSeat, Option<u8>)>,
    legal_bids: Vec<u8>,
    hook_forbidden_bid: Option<u8>,
    public_total: u8,
}

fn bidding_trace_after(steps: &[(VowTideSeat, u8)]) -> BiddingTrace {
    let mut state = setup_match(
        Seed(20260621),
        &canonical_seat_ids(4),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    for (seat, value) in steps {
        let bid =
            validate_bid_command(&state, &command(&state, *seat, *value)).expect("bid validates");
        apply_bid(&mut state, bid).expect("bid applies");
    }

    let phase = match state.phase {
        Phase::Bidding(_) => "bidding",
        Phase::PlayingTrick(_) => "playing_trick",
        Phase::Terminal(_) => "terminal",
    }
    .to_owned();
    let accepted_bids = state.public_bids.clone();
    let active = state.active_seat();
    let legal = active
        .map(|seat| legal_bids(&state, seat))
        .unwrap_or_default();
    let total = accepted_bids.iter().filter_map(|(_, bid)| *bid).sum::<u8>();
    let hook =
        if active == Some(state.dealer) && total <= state.current_hand_size().unwrap_or_default() {
            Some(state.current_hand_size().unwrap_or_default() - total)
        } else {
            None
        };

    BiddingTrace {
        trace_id: trace_id_for_steps(steps).to_owned(),
        phase,
        active_seat: active,
        accepted_bids,
        legal_bids: legal,
        hook_forbidden_bid: hook,
        public_total: total,
    }
}

fn trace_id_for_steps(steps: &[(VowTideSeat, u8)]) -> &'static str {
    match steps {
        [(VowTideSeat::Seat1, 2), (VowTideSeat::Seat2, 3), (VowTideSeat::Seat3, 4), (VowTideSeat::Seat0, 0)] => {
            "bidding-left-of-dealer-through-dealer"
        }
        [(VowTideSeat::Seat1, 3), (VowTideSeat::Seat2, 3), (VowTideSeat::Seat3, 3)] => {
            "dealer-hook-forbidden-total"
        }
        [(VowTideSeat::Seat1, 10), (VowTideSeat::Seat2, 10), (VowTideSeat::Seat3, 10)] => {
            "dealer-hook-out-of-range-no-removal"
        }
        [(VowTideSeat::Seat1, 0)] => "bid-zero-accepted",
        [(VowTideSeat::Seat1, 10)] => "bid-upper-bound-accepted",
        _ => "unknown-bidding-trace",
    }
}

fn bidding_trace_json(trace: &BiddingTrace) -> String {
    let bids = trace
        .accepted_bids
        .iter()
        .map(|(seat, bid)| {
            let value = bid
                .map(|bid| bid.to_string())
                .unwrap_or_else(|| "null".to_owned());
            format!(
                "    {{ \"seat\": \"{}\", \"bid\": {} }}",
                seat.as_str(),
                value
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");
    let legal = trace
        .legal_bids
        .iter()
        .map(|bid| bid.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let active = trace
        .active_seat
        .map(|seat| format!("\"{}\"", seat.as_str()))
        .unwrap_or_else(|| "null".to_owned());
    let hook = trace
        .hook_forbidden_bid
        .map(|bid| bid.to_string())
        .unwrap_or_else(|| "null".to_owned());

    format!(
        concat!(
            "{{\n",
            "  \"trace_id\": \"{}\",\n",
            "  \"seed\": 20260621,\n",
            "  \"seat_count\": 4,\n",
            "  \"hand_size\": 10,\n",
            "  \"phase\": \"{}\",\n",
            "  \"active_seat\": {},\n",
            "  \"public_total\": {},\n",
            "  \"hook_forbidden_bid\": {},\n",
            "  \"legal_bids\": [{}],\n",
            "  \"accepted_bids\": [\n",
            "{}\n",
            "  ]\n",
            "}}\n"
        ),
        trace.trace_id, trace.phase, active, trace.public_total, hook, legal, bids
    )
}

fn command(state: &VowTideState, seat: VowTideSeat, value: u8) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: SeatId(seat.as_str().to_owned()),
        },
        action_path: ActionPath {
            segments: vec!["bid".to_owned(), value.to_string()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}
