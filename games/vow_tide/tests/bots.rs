use engine_core::{ActionPath, CommandEnvelope, RulesVersion, Seed};
use vow_tide::{
    actions::{validate_bid_command, validate_play_command},
    bots::{bot_input_for, VowTideL0Bot, VowTideL1Bot},
    cards::{Card, Rank, Suit},
    ids::{canonical_seat_ids, VowTideSeat},
    rules::{apply_bid, apply_play},
    setup::{setup_match, SetupOptions},
};

#[test]
fn l0_selects_deterministic_legal_bid() {
    let state = setup_state();
    let decision = VowTideL0Bot::new(Seed(1))
        .select_decision(&state, VowTideSeat::Seat1)
        .expect("decision");

    validate_bid_command(
        &state,
        &command(&state, VowTideSeat::Seat1, decision.action_path),
    )
    .expect("l0 bid validates");
}

#[test]
fn l1_hook_adjusts_to_nearest_legal_bid() {
    let mut state = setup_state();
    apply_bid_value(&mut state, VowTideSeat::Seat1, 3);
    apply_bid_value(&mut state, VowTideSeat::Seat2, 3);
    apply_bid_value(&mut state, VowTideSeat::Seat3, 3);

    let decision = VowTideL1Bot::new(Seed(2))
        .select_decision(&state, VowTideSeat::Seat0)
        .expect("decision");

    assert_ne!(decision.action_path.segments, vec!["bid", "1"]);
    validate_bid_command(
        &state,
        &command(&state, VowTideSeat::Seat0, decision.action_path),
    )
    .expect("l1 hook-adjusted bid validates");
    assert!(!decision.rationale.contains("stock"));
}

#[test]
fn l1_play_uses_legal_cards_and_viewer_safe_explanation() {
    let mut state = setup_state();
    apply_bid_value(&mut state, VowTideSeat::Seat1, 1);
    apply_bid_value(&mut state, VowTideSeat::Seat2, 0);
    apply_bid_value(&mut state, VowTideSeat::Seat3, 0);
    apply_bid_value(&mut state, VowTideSeat::Seat0, 0);
    let high = Card::new(Rank::Ace, Suit::Clubs).id();
    let low = Card::new(Rank::Two, Suit::Clubs).id();
    *state
        .hand_for_internal_mut(VowTideSeat::Seat1)
        .expect("hand") = vec![low, high];

    let decision = VowTideL1Bot::new(Seed(3))
        .select_decision(&state, VowTideSeat::Seat1)
        .expect("decision");

    assert_eq!(
        decision.action_path.segments,
        vec!["play".to_owned(), low.as_str()]
    );
    let play = validate_play_command(
        &state,
        &command(&state, VowTideSeat::Seat1, decision.action_path.clone()),
    )
    .expect("play validates");
    apply_play(&mut state, play).expect("play applies");
    assert!(!decision.rationale.contains("hidden"));
    assert!(!decision.rationale.contains("stock"));
}

#[test]
fn l1_play_secures_lowest_currently_winning_card_when_contract_needs_trick() {
    let mut state = setup_state();
    apply_bid_value(&mut state, VowTideSeat::Seat1, 0);
    apply_bid_value(&mut state, VowTideSeat::Seat2, 1);
    apply_bid_value(&mut state, VowTideSeat::Seat3, 0);
    apply_bid_value(&mut state, VowTideSeat::Seat0, 0);
    let led_suit = non_trump_suit(state.trump_suit());
    let led = card_with_suit(led_suit, Rank::Jack);
    let low_winner = card_with_suit(led_suit, Rank::Queen);
    let high_winner = card_with_suit(led_suit, Rank::Ace);
    *state
        .hand_for_internal_mut(VowTideSeat::Seat1)
        .expect("hand") = vec![led];
    *state
        .hand_for_internal_mut(VowTideSeat::Seat2)
        .expect("hand") = vec![high_winner, low_winner];
    let first_play = validate_play_command(
        &state,
        &command(
            &state,
            VowTideSeat::Seat1,
            ActionPath {
                segments: vec!["play".to_owned(), led.as_str()],
            },
        ),
    )
    .expect("lead validates");
    apply_play(&mut state, first_play).expect("lead applies");

    let decision = VowTideL1Bot::new(Seed(4))
        .select_decision(&state, VowTideSeat::Seat2)
        .expect("decision");

    assert_eq!(
        decision.action_path.segments,
        vec!["play".to_owned(), low_winner.as_str()]
    );
}

#[test]
fn bot_input_contains_own_hand_only() {
    let mut state = setup_state();
    *state
        .hand_for_internal_mut(VowTideSeat::Seat1)
        .expect("hand") = vec![Card::new(Rank::Two, Suit::Clubs).id()];
    *state
        .hand_for_internal_mut(VowTideSeat::Seat2)
        .expect("hand") = vec![Card::new(Rank::Three, Suit::Diamonds).id()];
    state.hidden_stock = vec![Card::new(Rank::King, Suit::Clubs).id()];

    let input = bot_input_for(&state, VowTideSeat::Seat1);
    let summary = input.stable_summary();

    assert!(summary.contains("two_clubs"));
    assert!(!summary.contains("three_diamonds"));
    assert!(!summary.contains("king_clubs"));
}

fn setup_state() -> vow_tide::state::VowTideState {
    setup_match(
        Seed(20260621),
        &canonical_seat_ids(4),
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn apply_bid_value(state: &mut vow_tide::state::VowTideState, seat: VowTideSeat, value: u8) {
    let path = ActionPath {
        segments: vec!["bid".to_owned(), value.to_string()],
    };
    let bid = validate_bid_command(state, &command(state, seat, path)).expect("bid validates");
    apply_bid(state, bid).expect("bid applies");
}

fn command(
    state: &vow_tide::state::VowTideState,
    seat: VowTideSeat,
    action_path: ActionPath,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: engine_core::Actor {
            seat_id: state.seats[seat.index()].clone(),
        },
        action_path,
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn card_with_suit(suit: Suit, rank: Rank) -> vow_tide::cards::CardId {
    Card::new(rank, suit).id()
}

fn non_trump_suit(trump: Suit) -> Suit {
    Suit::ALL
        .into_iter()
        .find(|suit| *suit != trump)
        .expect("standard deck has non-trump suits")
}
