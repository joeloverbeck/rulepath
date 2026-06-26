use engine_core::{FreshnessToken, Seed};
use meldfall_ledger::{
    bots::{
        legal_action_paths, legal_action_tree_for_seat, parse_bot_action, MeldfallL0Bot,
        L0_POLICY_ID, L1_POLICY_STATUS,
    },
    cards::{Card, CardId, Rank, Suit},
    rules::{
        discard_card, draw_from_discard, draw_from_stock, finish_turn_after_table_plays,
        table_new_meld,
    },
    setup::{default_seats, setup_match, SetupOptions},
    state::{MatchState, TurnOrdinal, TurnPhase},
};

#[test]
fn l0_selects_deterministic_legal_actions_from_authorized_tree() {
    let mut state = bot_state();
    state.round.phase = TurnPhase::Discard;
    state.round.active_seat_index = 0;
    let bot = MeldfallL0Bot::new(Seed(42));

    let first = bot
        .select_decision(&state, 0)
        .expect("discard action selected");
    let second = bot
        .select_decision(&state, 0)
        .expect("discard action selected");
    let legal = legal_action_paths(&legal_action_tree_for_seat(&state, 0, FreshnessToken(0)));

    assert_eq!(first, second);
    assert_eq!(first.policy_id, L0_POLICY_ID);
    assert!(legal.contains(&first.action_path));
    assert!(first.explanation.contains("viewer-authorized"));
}

#[test]
fn l0_input_contains_own_hand_only_and_no_stock_order() {
    let state = bot_state();
    let input = MeldfallL0Bot::input_for(&state, 1);
    let surface = format!("{input:?}");

    for own in &state.round.seats[1].hand {
        assert!(surface.contains(&own.as_str()));
    }
    for hidden_stock in &state.round.stock {
        assert!(!surface.contains(&hidden_stock.as_str()));
    }
    for (seat_index, seat) in state.round.seats.iter().enumerate() {
        if seat_index == 1 {
            continue;
        }
        for hidden in &seat.hand {
            assert!(!surface.contains(&hidden.as_str()));
        }
    }
}

#[test]
fn selected_l0_actions_apply_through_rules_api_for_current_phases() {
    let mut state = bot_state();
    let bot = MeldfallL0Bot::new(Seed(7));

    state.round.phase = TurnPhase::Draw;
    state.round.active_seat_index = 0;
    let draw = bot.select_action(&state, 0).expect("draw selected");
    match parse_bot_action(&draw).expect("parse draw") {
        meldfall_ledger::actions::MeldfallAction::DrawFromStock => {
            draw_from_stock(&mut state.round, 0).expect("draw applies");
        }
        meldfall_ledger::actions::MeldfallAction::DrawFromDiscard { discard_index } => {
            draw_from_discard(&mut state.round, 0, discard_index).expect("discard draw applies");
            let card = state
                .round
                .pending_pickup
                .as_ref()
                .expect("discard draw creates commitment")
                .selected_card;
            let supporting = Suit::ALL
                .into_iter()
                .filter(|suit| *suit != card.card().suit)
                .take(2)
                .map(|suit| Card::new(card.card().rank, suit).id())
                .collect::<Vec<_>>();
            let meld = vec![card, supporting[0], supporting[1]];
            state.round.seats[0]
                .hand
                .extend(meld.iter().copied().skip(1));
            meldfall_ledger::rules::table_new_meld(&mut state.round, 0, &meld, TurnOrdinal(0))
                .expect("commitment meld applies");
        }
        other => panic!("expected draw action, got {other:?}"),
    }

    let finish = bot.select_action(&state, 0).expect("finish selected");
    assert_eq!(
        parse_bot_action(&finish).expect("parse finish"),
        meldfall_ledger::actions::MeldfallAction::FinishTurn
    );
    finish_turn_after_table_plays(&mut state.round, 0).expect("finish applies");

    let discard = bot.select_action(&state, 0).expect("discard selected");
    let meldfall_ledger::actions::MeldfallAction::Discard { card } =
        parse_bot_action(&discard).expect("parse discard")
    else {
        panic!("expected discard action");
    };
    discard_card(&mut state.round, 0, card).expect("discard applies");
}

#[test]
fn bot_trace_inventory_records_l1_not_admitted() {
    let l0 = include_str!("golden_traces/l0-random-legal-full-match.trace.json");
    let l1 = include_str!("golden_traces/l1-rule-informed-smoke.trace.json");

    assert!(l0.contains("\"policy_id\": \"meldfall-ledger-l0-random-legal-v1\""));
    assert!(l0.contains("\"public_no_leak\": true"));
    assert!(l1.contains(L1_POLICY_STATUS));
}

#[test]
fn draw_phase_omits_unusable_discard_pickup_to_prevent_deadlock() {
    // Active seat cannot meld or lay off the only discard card, but the stock is
    // non-empty. Offering the unusable pickup would create an unsatisfiable
    // immediate-use commitment (ML-TURN-004) with no legal way to continue the
    // turn, deadlocking the round. The draw tree must therefore omit it.
    let mut state = bot_state();
    state.round.active_seat_index = 0;
    state.round.phase = TurnPhase::Draw;
    state.round.pending_pickup = None;
    state.round.stock = vec![card(Rank::Ace, Suit::Spades)];
    state.round.discard = vec![card(Rank::Nine, Suit::Clubs)];
    state.round.seats[0].hand = vec![card(Rank::Two, Suit::Clubs), card(Rank::Five, Suit::Hearts)];

    let segments = draw_segments(&state);
    assert!(
        segments.iter().any(|segment| segment == "draw-stock"),
        "stock draw stays available: {segments:?}"
    );
    assert!(
        !segments
            .iter()
            .any(|segment| segment.starts_with("draw-discard")),
        "an unusable discard pickup must not be offered: {segments:?}"
    );
}

#[test]
fn draw_phase_offers_discard_pickup_usable_in_a_new_meld() {
    let mut state = bot_state();
    state.round.active_seat_index = 0;
    state.round.phase = TurnPhase::Draw;
    state.round.pending_pickup = None;
    state.round.stock = vec![card(Rank::Ace, Suit::Spades)];
    state.round.discard = vec![card(Rank::Nine, Suit::Clubs)];
    state.round.seats[0].hand = vec![
        card(Rank::Nine, Suit::Diamonds),
        card(Rank::Nine, Suit::Hearts),
    ];

    let segments = draw_segments(&state);
    assert!(
        segments.iter().any(|segment| segment == "draw-discard-0"),
        "a discard pickup that completes a new set must be offered: {segments:?}"
    );
}

#[test]
fn draw_phase_offers_discard_pickup_usable_as_a_layoff() {
    let mut state = bot_state();
    state.round.active_seat_index = 0;
    state.round.phase = TurnPhase::Draw;
    state.round.pending_pickup = None;
    state.round.stock = vec![card(Rank::Ace, Suit::Spades)];
    state.round.discard = vec![card(Rank::Five, Suit::Clubs)];
    state.round.seats[0].hand = vec![card(Rank::King, Suit::Spades)];
    // A public run 6-7-8 of clubs that the picked-up 5C extends at the low end.
    let run = [
        card(Rank::Six, Suit::Clubs),
        card(Rank::Seven, Suit::Clubs),
        card(Rank::Eight, Suit::Clubs),
    ];
    state.round.seats[1].hand = run.to_vec();
    table_new_meld(&mut state.round, 1, &run, TurnOrdinal(0)).expect("run tables");

    let segments = draw_segments(&state);
    assert!(
        segments.iter().any(|segment| segment == "draw-discard-0"),
        "a discard pickup usable via lay-off must be offered: {segments:?}"
    );
}

fn draw_segments(state: &MatchState) -> Vec<String> {
    legal_action_paths(&legal_action_tree_for_seat(
        state,
        state.round.active_seat_index,
        FreshnessToken(0),
    ))
    .iter()
    .map(|path| path.segments.join("/"))
    .collect()
}

fn bot_state() -> MatchState {
    let seats = default_seats(4).expect("seat count supported");
    let setup = setup_match(Seed(1915), &seats, &SetupOptions::default()).expect("setup succeeds");
    let mut state = MatchState::from_initial_setup(setup);
    state.round.stock = vec![
        card(Rank::Ace, Suit::Spades),
        card(Rank::King, Suit::Spades),
    ];
    state.round.discard = vec![card(Rank::Nine, Suit::Clubs)];
    state.round.seats[0].hand = vec![card(Rank::Two, Suit::Clubs)];
    state.round.seats[1].hand = vec![card(Rank::Three, Suit::Diamonds)];
    state.round.seats[2].hand = vec![card(Rank::Four, Suit::Hearts)];
    state.round.seats[3].hand = vec![card(Rank::Five, Suit::Spades)];
    state
}

fn card(rank: Rank, suit: Suit) -> CardId {
    Card::new(rank, suit).id()
}
