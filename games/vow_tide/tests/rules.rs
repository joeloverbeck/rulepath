use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};
use vow_tide::{
    actions::{
        legal_action_tree, legal_bids, legal_cards, validate_bid_command, validate_play_command,
    },
    cards::{Card, CardId, Rank, Suit},
    ids::{
        canonical_seat_ids, hand_schedule_for_seats, max_hand_size_for_seats, VowTideSeat,
        STANDARD_MAX_SEATS, STANDARD_MIN_SEATS,
    },
    rules::{apply_bid, apply_play},
    scoring::{resolve_completed_hand, score_current_hand},
    setup::{deal_order_after, setup_match, SetupOptions},
    state::Phase,
};

#[test]
fn setup_accepts_three_to_seven_ordered_seats() {
    let options = SetupOptions::default();

    for seat_count in STANDARD_MIN_SEATS as usize..=STANDARD_MAX_SEATS as usize {
        let seats = canonical_seat_ids(seat_count);
        let state = setup_match(Seed(11), &seats, &options).expect("setup succeeds");

        assert_eq!(state.seats, seats);
        assert_eq!(state.dealer, VowTideSeat::Seat0);
        assert_eq!(state.hand_index, 0);
        assert_eq!(state.cumulative_scores.len(), seat_count);
        assert_eq!(state.private_hands.len(), seat_count);
        assert_eq!(
            state.current_hand_size(),
            max_hand_size_for_seats(seat_count)
        );
        assert_eq!(
            state.seat_labels,
            (1..=seat_count)
                .map(|index| format!("Tide {index}"))
                .collect::<Vec<_>>()
        );
        assert!(matches!(state.phase, Phase::Bidding(_)));
        assert_eq!(state.active_seat(), Some(VowTideSeat::Seat1));
    }
}

#[test]
fn setup_rejects_unsupported_seat_counts_with_stable_diagnostic() {
    let options = SetupOptions::default();

    for seat_count in [0, 1, 2, 8] {
        let seats = (0..seat_count)
            .map(|index| SeatId(format!("seat_{index}")))
            .collect::<Vec<_>>();
        let diagnostic =
            setup_match(Seed(0), &seats, &options).expect_err("unsupported count is rejected");

        assert_eq!(diagnostic.code, "VT_INVALID_SEAT_COUNT");
        assert_eq!(
            diagnostic.message,
            format!("vow_tide supports 3 to 7 seats; received {seat_count}")
        );
    }
}

#[test]
fn schedule_is_max_down_to_one_up_to_max_for_every_supported_count() {
    let cases = [
        (3, 10, 19),
        (4, 10, 19),
        (5, 10, 19),
        (6, 8, 15),
        (7, 7, 13),
    ];

    for (seat_count, max_hand_size, total_hands) in cases {
        let schedule = hand_schedule_for_seats(seat_count).expect("supported count has schedule");

        assert_eq!(max_hand_size_for_seats(seat_count), Some(max_hand_size));
        assert_eq!(schedule.len(), total_hands);
        assert_eq!(schedule.first(), Some(&max_hand_size));
        assert_eq!(schedule.last(), Some(&max_hand_size));
        assert_eq!(
            schedule.iter().filter(|hand_size| **hand_size == 1).count(),
            1
        );

        for window in schedule[..max_hand_size as usize].windows(2) {
            assert_eq!(window[0], window[1] + 1);
        }
        for window in schedule[max_hand_size as usize - 1..].windows(2) {
            assert_eq!(window[0] + 1, window[1]);
        }
    }
}

#[test]
fn bidding_starts_left_of_dealer_and_advances_clockwise_to_dealer() {
    let mut state = setup_state(4);
    assert_eq!(state.active_seat(), Some(VowTideSeat::Seat1));

    apply_bid_value(&mut state, VowTideSeat::Seat1, 2);
    assert_eq!(state.active_seat(), Some(VowTideSeat::Seat2));
    apply_bid_value(&mut state, VowTideSeat::Seat2, 3);
    assert_eq!(state.active_seat(), Some(VowTideSeat::Seat3));
    apply_bid_value(&mut state, VowTideSeat::Seat3, 4);
    assert_eq!(state.active_seat(), Some(VowTideSeat::Seat0));

    apply_bid_value(&mut state, VowTideSeat::Seat0, 0);
    assert!(matches!(state.phase, Phase::PlayingTrick(_)));
    assert_eq!(state.active_seat(), Some(VowTideSeat::Seat1));
}

#[test]
fn ring_step_and_deal_order_wrap_across_supported_counts() {
    for seat_count in STANDARD_MIN_SEATS as usize..=STANDARD_MAX_SEATS as usize {
        for current_index in 0..seat_count {
            let current = VowTideSeat::from_index(current_index).expect("seat in range");
            let expected = VowTideSeat::from_index((current_index + 1) % seat_count)
                .expect("next seat in range");

            assert_eq!(current.next_clockwise(seat_count), expected);
        }

        for dealer_index in 0..seat_count {
            let dealer = VowTideSeat::from_index(dealer_index).expect("dealer in range");
            let expected = (1..=seat_count)
                .map(|offset| {
                    VowTideSeat::from_index((dealer_index + offset) % seat_count)
                        .expect("deal-order seat in range")
                })
                .collect::<Vec<_>>();

            assert_eq!(deal_order_after(dealer, seat_count), expected);
        }
    }
}

#[test]
fn legal_tree_lists_ascending_bids_and_omits_dealer_hook_value() {
    let mut state = setup_state(4);

    assert_eq!(
        legal_bids(&state, VowTideSeat::Seat1),
        (0..=10).collect::<Vec<_>>()
    );
    let tree = legal_action_tree(&state, &actor(VowTideSeat::Seat1));
    assert_eq!(
        bid_leaf_segments(&tree),
        (0..=10).map(|value| value.to_string()).collect::<Vec<_>>()
    );

    apply_bid_value(&mut state, VowTideSeat::Seat1, 3);
    apply_bid_value(&mut state, VowTideSeat::Seat2, 3);
    apply_bid_value(&mut state, VowTideSeat::Seat3, 3);

    assert_eq!(state.active_seat(), Some(VowTideSeat::Seat0));
    assert_eq!(
        legal_bids(&state, VowTideSeat::Seat0),
        vec![0, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    );
    let tree = legal_action_tree(&state, &actor(VowTideSeat::Seat0));
    assert!(!bid_leaf_segments(&tree).contains(&"1".to_owned()));
}

#[test]
fn dealer_hook_removes_nothing_when_prefix_total_exceeds_hand_size() {
    let mut state = setup_state(4);
    apply_bid_value(&mut state, VowTideSeat::Seat1, 10);
    apply_bid_value(&mut state, VowTideSeat::Seat2, 10);
    apply_bid_value(&mut state, VowTideSeat::Seat3, 10);

    assert_eq!(state.active_seat(), Some(VowTideSeat::Seat0));
    assert_eq!(
        legal_bids(&state, VowTideSeat::Seat0),
        (0..=10).collect::<Vec<_>>()
    );
}

#[test]
fn invalid_bids_return_stable_diagnostics() {
    let mut state = setup_state(4);

    let wrong_seat =
        validate_bid_command(&state, &command_for_state(&state, VowTideSeat::Seat2, 1));
    assert_eq!(
        wrong_seat.expect_err("wrong seat rejected").code,
        "VT_WRONG_SEAT"
    );

    let out_of_range =
        validate_bid_command(&state, &command_for_state(&state, VowTideSeat::Seat1, 11));
    assert_eq!(
        out_of_range.expect_err("out of range rejected").code,
        "VT_BID_OUT_OF_RANGE"
    );

    let stale = validate_bid_command(
        &state,
        &CommandEnvelope {
            freshness_token: FreshnessToken(99),
            ..command_for_state(&state, VowTideSeat::Seat1, 1)
        },
    );
    assert_eq!(stale.expect_err("stale rejected").code, "VT_STALE_COMMAND");

    *state
        .bidding_state_mut()
        .expect("bidding")
        .bid_for_mut(VowTideSeat::Seat1)
        .expect("bid row") = Some(1);
    let duplicate = validate_bid_command(&state, &command_for_state(&state, VowTideSeat::Seat1, 2));
    assert_eq!(
        duplicate.expect_err("duplicate rejected").code,
        "VT_BID_ALREADY_SET"
    );
}

#[test]
fn hook_forbidden_bid_is_rejected_by_validator() {
    let mut state = setup_state(4);
    apply_bid_value(&mut state, VowTideSeat::Seat1, 3);
    apply_bid_value(&mut state, VowTideSeat::Seat2, 3);
    apply_bid_value(&mut state, VowTideSeat::Seat3, 3);

    let diagnostic =
        validate_bid_command(&state, &command_for_state(&state, VowTideSeat::Seat0, 1))
            .expect_err("hook bid rejected");
    assert_eq!(diagnostic.code, "VT_BID_HOOK_FORBIDDEN");
}

#[test]
fn bid_during_playing_phase_is_wrong_phase() {
    let mut state = setup_state(3);
    apply_bid_value(&mut state, VowTideSeat::Seat1, 0);
    apply_bid_value(&mut state, VowTideSeat::Seat2, 0);
    apply_bid_value(&mut state, VowTideSeat::Seat0, 1);

    let diagnostic =
        validate_bid_command(&state, &command_for_state(&state, VowTideSeat::Seat1, 0))
            .expect_err("bid after bidding rejected");
    assert_eq!(diagnostic.code, "VT_WRONG_PHASE");
}

#[test]
fn lead_may_be_any_card_including_trump() {
    let mut state = complete_bidding_to_play();
    let trump_card = card_with_suit(state.trump_suit(), Rank::Ace);
    replace_hand(&mut state, VowTideSeat::Seat1, vec![trump_card]);

    let play = validate_play_command(
        &state,
        &play_command_for_state(&state, VowTideSeat::Seat1, trump_card),
    )
    .expect("trump lead validates");
    apply_play(&mut state, play).expect("trump lead applies");

    let playing = state.playing_state().expect("playing phase");
    assert_eq!(playing.current_trick.plays[0].card, trump_card);
    assert_eq!(state.active_seat(), Some(VowTideSeat::Seat2));
}

#[test]
fn follower_holding_led_suit_must_follow() {
    let mut state = complete_bidding_to_play();
    let led_suit = non_trump_suit(state.trump_suit());
    let led_card = card_with_suit(led_suit, Rank::Two);
    let follow_card = card_with_suit(led_suit, Rank::Three);
    let off_suit = card_with_suit(state.trump_suit(), Rank::Ace);
    replace_hand(&mut state, VowTideSeat::Seat1, vec![led_card]);
    replace_hand(&mut state, VowTideSeat::Seat2, vec![off_suit, follow_card]);

    apply_play_value(&mut state, VowTideSeat::Seat1, led_card);
    assert_eq!(legal_cards(&state, VowTideSeat::Seat2), vec![follow_card]);

    let diagnostic = validate_play_command(
        &state,
        &play_command_for_state(&state, VowTideSeat::Seat2, off_suit),
    )
    .expect_err("off suit rejected when led suit held");
    assert_eq!(diagnostic.code, "VT_MUST_FOLLOW_SUIT");
}

#[test]
fn void_follower_may_play_any_card() {
    let mut state = complete_bidding_to_play();
    let led_suit = non_trump_suit(state.trump_suit());
    let led_card = card_with_suit(led_suit, Rank::Two);
    let trump_card = card_with_suit(state.trump_suit(), Rank::Ace);
    replace_hand(&mut state, VowTideSeat::Seat1, vec![led_card]);
    replace_hand(&mut state, VowTideSeat::Seat2, vec![trump_card]);

    apply_play_value(&mut state, VowTideSeat::Seat1, led_card);
    assert_eq!(legal_cards(&state, VowTideSeat::Seat2), vec![trump_card]);
    apply_play_value(&mut state, VowTideSeat::Seat2, trump_card);
}

#[test]
fn highest_trump_wins_and_winner_leads_next_trick() {
    let mut state = complete_bidding_to_play();
    let trump = state.trump_suit();
    let led_suit = non_trump_suit(trump);
    let seat_1 = card_with_suit(led_suit, Rank::Ace);
    let seat_2 = card_with_suit(led_suit, Rank::King);
    let seat_3 = card_with_suit(trump, Rank::Two);
    let seat_0 = card_with_suit(led_suit, Rank::Queen);
    replace_hand(&mut state, VowTideSeat::Seat1, vec![seat_1]);
    replace_hand(&mut state, VowTideSeat::Seat2, vec![seat_2]);
    replace_hand(&mut state, VowTideSeat::Seat3, vec![seat_3]);
    replace_hand(&mut state, VowTideSeat::Seat0, vec![seat_0]);

    apply_play_value(&mut state, VowTideSeat::Seat1, seat_1);
    apply_play_value(&mut state, VowTideSeat::Seat2, seat_2);
    apply_play_value(&mut state, VowTideSeat::Seat3, seat_3);
    apply_play_value(&mut state, VowTideSeat::Seat0, seat_0);

    assert_eq!(state.captured_tricks.len(), 1);
    assert_eq!(state.captured_tricks[0].winner, VowTideSeat::Seat3);
    assert_eq!(
        state.trick_counts,
        vec![
            (VowTideSeat::Seat0, 0),
            (VowTideSeat::Seat1, 0),
            (VowTideSeat::Seat2, 0),
            (VowTideSeat::Seat3, 1),
        ]
    );
    let playing = state.playing_state().expect("playing phase");
    assert_eq!(playing.leader, VowTideSeat::Seat3);
    assert_eq!(playing.active_seat, VowTideSeat::Seat3);
}

#[test]
fn exact_contract_scoring_includes_zero_and_misses_score_zero() {
    let mut state = setup_state(4);
    set_public_bids(&mut state, &[(0, 0), (1, 2), (2, 3), (3, 4)]);
    set_trick_counts(&mut state, &[(0, 0), (1, 2), (2, 2), (3, 5)]);

    let breakdown = score_current_hand(&mut state).expect("hand scores");

    assert_eq!(breakdown.seats[VowTideSeat::Seat0.index()].addition, 10);
    assert!(breakdown.seats[VowTideSeat::Seat0.index()].successful_zero);
    assert_eq!(breakdown.seats[VowTideSeat::Seat1.index()].addition, 12);
    assert_eq!(breakdown.seats[VowTideSeat::Seat2.index()].addition, 0);
    assert_eq!(breakdown.seats[VowTideSeat::Seat3.index()].addition, 0);
    assert_eq!(
        state.cumulative_scores,
        vec![
            (VowTideSeat::Seat0, 10),
            (VowTideSeat::Seat1, 12),
            (VowTideSeat::Seat2, 0),
            (VowTideSeat::Seat3, 0),
        ]
    );
}

#[test]
fn hand_resolution_records_before_dealer_and_schedule_advance() {
    let mut state = setup_state(4);
    state.hand_schedule = vec![1, 2];
    set_public_bids(&mut state, &[(0, 1), (1, 0), (2, 0), (3, 0)]);
    set_trick_counts(&mut state, &[(0, 1), (1, 0), (2, 0), (3, 0)]);
    let mut effects = Vec::new();

    resolve_completed_hand(&mut state, &mut effects).expect("hand resolves");

    assert_eq!(state.completed_hands.len(), 1);
    assert_eq!(state.completed_hands[0].hand_index, 0);
    assert_eq!(state.hand_index, 1);
    assert_eq!(state.dealer, VowTideSeat::Seat1);
    assert!(matches!(state.phase, Phase::Bidding(_)));
    assert_eq!(state.current_hand_size(), Some(2));
    assert_eq!(
        state
            .public_bids
            .iter()
            .filter(|(_, bid)| bid.is_some())
            .count(),
        0
    );
}

#[test]
fn terminal_co_winners_use_competition_rank() {
    let mut state = setup_state(4);
    state.hand_schedule = vec![1];
    set_public_bids(&mut state, &[(0, 0), (1, 0), (2, 0), (3, 0)]);
    set_trick_counts(&mut state, &[(0, 0), (1, 0), (2, 1), (3, 1)]);
    let mut effects = Vec::new();

    resolve_completed_hand(&mut state, &mut effects).expect("match resolves");

    let outcome = state.terminal_outcome.expect("terminal outcome");
    assert_eq!(
        outcome.winners,
        vec![VowTideSeat::Seat0, VowTideSeat::Seat1]
    );
    assert_eq!(outcome.standings[0].rank, 1);
    assert_eq!(outcome.standings[1].rank, 1);
    assert_eq!(outcome.standings[2].rank, 3);
    assert_eq!(outcome.standings[3].rank, 3);
    assert!(matches!(state.phase, Phase::Terminal(_)));
}

fn setup_state(seat_count: usize) -> vow_tide::state::VowTideState {
    setup_match(
        Seed(19),
        &canonical_seat_ids(seat_count),
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn apply_bid_value(state: &mut vow_tide::state::VowTideState, seat: VowTideSeat, value: u8) {
    let bid =
        validate_bid_command(state, &command_for_state(state, seat, value)).expect("bid validates");
    apply_bid(state, bid).expect("bid applies");
}

fn apply_play_value(state: &mut vow_tide::state::VowTideState, seat: VowTideSeat, card: CardId) {
    let play = validate_play_command(state, &play_command_for_state(state, seat, card))
        .expect("play validates");
    apply_play(state, play).expect("play applies");
}

fn complete_bidding_to_play() -> vow_tide::state::VowTideState {
    let mut state = setup_state(4);
    apply_bid_value(&mut state, VowTideSeat::Seat1, 0);
    apply_bid_value(&mut state, VowTideSeat::Seat2, 0);
    apply_bid_value(&mut state, VowTideSeat::Seat3, 0);
    apply_bid_value(&mut state, VowTideSeat::Seat0, 1);
    state
}

fn replace_hand(state: &mut vow_tide::state::VowTideState, seat: VowTideSeat, cards: Vec<CardId>) {
    *state.hand_for_internal_mut(seat).expect("hand exists") = cards;
}

fn set_public_bids(state: &mut vow_tide::state::VowTideState, values: &[(usize, u8)]) {
    for (seat, bid) in values {
        state.public_bids[*seat].1 = Some(*bid);
    }
}

fn set_trick_counts(state: &mut vow_tide::state::VowTideState, values: &[(usize, u8)]) {
    for (seat, count) in values {
        state.trick_counts[*seat].1 = *count;
    }
}

fn command_for_state(
    state: &vow_tide::state::VowTideState,
    seat: VowTideSeat,
    value: u8,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec!["bid".to_owned(), value.to_string()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn play_command_for_state(
    state: &vow_tide::state::VowTideState,
    seat: VowTideSeat,
    card: CardId,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec!["play".to_owned(), card.as_str()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn actor(seat: VowTideSeat) -> Actor {
    Actor {
        seat_id: SeatId(seat.as_str().to_owned()),
    }
}

fn bid_leaf_segments(tree: &engine_core::ActionTree) -> Vec<String> {
    tree.root.choices[0]
        .next
        .as_ref()
        .expect("bid branch")
        .choices
        .iter()
        .map(|choice| choice.segment.clone())
        .collect()
}

fn card_with_suit(suit: Suit, rank: Rank) -> CardId {
    Card::new(rank, suit).id()
}

fn non_trump_suit(trump: Suit) -> Suit {
    Suit::ALL
        .into_iter()
        .find(|suit| *suit != trump)
        .expect("standard deck has non-trump suits")
}
