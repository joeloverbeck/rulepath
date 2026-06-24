use briar_circuit::{
    apply_pass_action, apply_play_action, legal_play_cards, score_completed_hand,
    setup::{deal_order_after, next_dealer},
    setup_match, validate_pass_command, validate_play_card, BriarCircuitSeat, BriarCircuitState,
    CapturedTrick, Card, CurrentTrick, MoonStatus, OutcomeStatus, PassAction, PassDirection, Phase,
    PlayAction, PlayingTrickState, Rank, SetupOptions, Suit, TrickPlay,
};
use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};

fn seats(count: usize) -> Vec<SeatId> {
    (0..count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect()
}

fn fresh_state() -> BriarCircuitState {
    setup_match(Seed(1606), &seats(4), &SetupOptions::default()).expect("setup succeeds")
}

fn card(rank: Rank, suit: Suit) -> briar_circuit::CardId {
    Card::new(rank, suit).id()
}

fn trick_state(
    hands: Vec<(BriarCircuitSeat, Vec<briar_circuit::CardId>)>,
    play: PlayingTrickState,
) -> BriarCircuitState {
    let mut state = fresh_state();
    state.private_hands = hands;
    state.phase = Phase::PlayingTrick(play);
    state
}

fn captured_trick(winner: BriarCircuitSeat, cards: Vec<briar_circuit::CardId>) -> CapturedTrick {
    CapturedTrick {
        hand_index: 0,
        trick_index: 0,
        winner,
        plays: cards
            .into_iter()
            .enumerate()
            .map(|(index, card)| TrickPlay {
                seat: BriarCircuitSeat::from_index(index % BriarCircuitSeat::ALL.len())
                    .expect("seat index"),
                card,
            })
            .collect(),
    }
}

fn pass_command(
    state: &BriarCircuitState,
    seat: BriarCircuitSeat,
    segments: &[&str],
) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: SeatId(seat.as_str().to_owned()),
        },
        action_path: ActionPath {
            segments: segments
                .iter()
                .map(|segment| (*segment).to_owned())
                .collect(),
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn setup_accepts_exactly_four_seats() {
    let state = setup_match(Seed(16), &seats(4), &SetupOptions::default()).expect("setup succeeds");

    assert_eq!(state.seats[0], SeatId("seat_0".to_owned()));
    assert_eq!(state.dealer, BriarCircuitSeat::Seat0);
    assert_eq!(state.hand_index, 0);
    assert_eq!(state.cumulative_scores, [0, 0, 0, 0]);
    assert!(matches!(
        state.phase,
        Phase::Passing(ref pass) if pass.direction == PassDirection::Left
    ));
}

#[test]
fn deal_starts_left_of_dealer_and_dealer_rotates_clockwise() {
    assert_eq!(
        deal_order_after(BriarCircuitSeat::Seat0),
        [
            BriarCircuitSeat::Seat1,
            BriarCircuitSeat::Seat2,
            BriarCircuitSeat::Seat3,
            BriarCircuitSeat::Seat0,
        ]
    );
    assert_eq!(
        next_dealer(BriarCircuitSeat::Seat0),
        BriarCircuitSeat::Seat1
    );
    assert_eq!(
        next_dealer(BriarCircuitSeat::Seat3),
        BriarCircuitSeat::Seat0
    );
}

#[test]
fn setup_rejects_every_non_four_count_with_stable_diagnostic() {
    for count in [0, 1, 2, 3, 5, 6, 7] {
        let err = setup_match(Seed(16), &seats(count), &SetupOptions::default())
            .expect_err("unsupported seat count rejects");

        assert_eq!(err.code, "BC_UNSUPPORTED_SEAT_COUNT");
        assert_eq!(
            err.message,
            format!("briar_circuit requires exactly four seats; received {count}")
        );
    }
}

#[test]
fn pass_direction_cycle_and_targets_are_stable() {
    assert_eq!(PassDirection::for_hand_index(0), PassDirection::Left);
    assert_eq!(PassDirection::for_hand_index(1), PassDirection::Right);
    assert_eq!(PassDirection::for_hand_index(2), PassDirection::Across);
    assert_eq!(PassDirection::for_hand_index(3), PassDirection::Hold);
    assert_eq!(PassDirection::for_hand_index(4), PassDirection::Left);
    assert_eq!(
        PassDirection::Left.target_for(BriarCircuitSeat::Seat3),
        BriarCircuitSeat::Seat0
    );
    assert_eq!(
        PassDirection::Right.target_for(BriarCircuitSeat::Seat0),
        BriarCircuitSeat::Seat3
    );
    assert_eq!(
        PassDirection::Across.target_for(BriarCircuitSeat::Seat1),
        BriarCircuitSeat::Seat3
    );
}

#[test]
fn pass_command_validates_actor_path_freshness_and_rules_version() {
    let state = fresh_state();
    let seat = BriarCircuitSeat::Seat0;
    let card = state.hand_for_internal(seat)[0];
    let card_segment = card.as_str();
    let envelope = pass_command(&state, seat, &["pass", "select", &card_segment]);

    let (validated_seat, action) =
        validate_pass_command(&state, &envelope).expect("command validates");

    assert_eq!(validated_seat, seat);
    assert_eq!(action, PassAction::Select(card));

    let mut stale = envelope.clone();
    stale.freshness_token = FreshnessToken(state.freshness_token.0 + 1);
    assert_eq!(
        validate_pass_command(&state, &stale)
            .expect_err("stale command rejects")
            .code,
        "BC_STALE_COMMAND"
    );

    let mut wrong_rules = envelope;
    wrong_rules.rules_version = RulesVersion(2);
    assert_eq!(
        validate_pass_command(&state, &wrong_rules)
            .expect_err("wrong rules reject")
            .code,
        "BC_WRONG_RULES_VERSION"
    );
}

#[test]
fn pass_select_unselect_and_confirm_require_three_distinct_owned_cards() {
    let mut state = fresh_state();
    let seat = BriarCircuitSeat::Seat0;
    let first = state.hand_for_internal(seat)[0];
    let second = state.hand_for_internal(seat)[1];
    let third = state.hand_for_internal(seat)[2];
    let unowned = state.hand_for_internal(BriarCircuitSeat::Seat1)[0];

    apply_pass_action(&mut state, seat, PassAction::Select(first)).expect("first select");

    assert_eq!(
        apply_pass_action(&mut state, seat, PassAction::Select(first))
            .expect_err("duplicate select rejects")
            .code,
        "BC_PASS_DUPLICATE_CARD"
    );
    assert_eq!(
        apply_pass_action(&mut state, seat, PassAction::Select(unowned))
            .expect_err("unowned select rejects")
            .code,
        "BC_CARD_NOT_OWNED"
    );
    assert_eq!(
        apply_pass_action(&mut state, seat, PassAction::Confirm)
            .expect_err("short confirm rejects")
            .code,
        "BC_PASS_REQUIRES_THREE"
    );

    apply_pass_action(&mut state, seat, PassAction::Unselect(first)).expect("unselect");
    assert_eq!(
        apply_pass_action(&mut state, seat, PassAction::Unselect(first))
            .expect_err("missing unselect rejects")
            .code,
        "BC_CARD_NOT_SELECTED"
    );

    for card in [first, second, third] {
        apply_pass_action(&mut state, seat, PassAction::Select(card)).expect("select card");
    }
    apply_pass_action(&mut state, seat, PassAction::Confirm).expect("confirm succeeds");

    assert_eq!(
        apply_pass_action(&mut state, seat, PassAction::Unselect(first))
            .expect_err("committed seat cannot mutate")
            .code,
        "BC_PASS_ALREADY_COMMITTED"
    );
}

#[test]
fn fourth_confirm_exchanges_cards_atomically_and_enters_playing_phase() {
    let mut state = fresh_state();
    let initial_hands: Vec<_> = BriarCircuitSeat::ALL
        .into_iter()
        .map(|seat| (seat, state.hand_for_internal(seat).to_vec()))
        .collect();
    let selected: Vec<_> = initial_hands
        .iter()
        .map(|(seat, hand)| (*seat, hand[..3].to_vec()))
        .collect();

    for (seat, cards) in &selected {
        for card in cards {
            apply_pass_action(&mut state, *seat, PassAction::Select(*card))
                .expect("select pass card");
        }
        let result = apply_pass_action(&mut state, *seat, PassAction::Confirm)
            .expect("confirm pass selection");
        assert_eq!(
            result.exchange_completed,
            *seat == BriarCircuitSeat::Seat3,
            "only fourth confirm completes exchange"
        );
    }

    assert!(matches!(state.phase, Phase::PlayingTrick(_)));
    for seat in BriarCircuitSeat::ALL {
        assert_eq!(state.hand_for_internal(seat).len(), 13);
    }

    let direction = PassDirection::Left;
    for (source, cards) in &selected {
        let target = direction.target_for(*source);
        for card in cards {
            assert!(!state.hand_for_internal(*source).contains(card));
            assert!(state.hand_for_internal(target).contains(card));
        }
    }
}

#[test]
fn hold_hand_skips_selection_and_exchange() {
    use briar_circuit::{setup::deal_hand, Variant};
    use engine_core::SeededRng;

    let mut rng = SeededRng::from_seed(Seed(1616));
    let deal = deal_hand(&mut rng, BriarCircuitSeat::Seat3, 3).expect("deal succeeds");
    let state = BriarCircuitState::new_after_deal(
        Variant::briar_circuit_standard(),
        [
            SeatId("seat_0".to_owned()),
            SeatId("seat_1".to_owned()),
            SeatId("seat_2".to_owned()),
            SeatId("seat_3".to_owned()),
        ],
        BriarCircuitSeat::Seat3,
        3,
        deal.hands,
        deal.pass_direction,
    );

    assert_eq!(deal.pass_direction, PassDirection::Hold);
    assert!(matches!(state.phase, Phase::PlayingTrick(_)));
    assert_eq!(
        apply_pass_action(
            &mut state.clone(),
            BriarCircuitSeat::Seat0,
            PassAction::Confirm
        )
        .expect_err("hold hand has no pass phase")
        .code,
        "BC_WRONG_PHASE"
    );
}

#[test]
fn first_play_of_hand_must_be_two_of_clubs() {
    let two_clubs = card(Rank::Two, Suit::Clubs);
    let three_clubs = card(Rank::Three, Suit::Clubs);
    let mut state = trick_state(
        vec![
            (BriarCircuitSeat::Seat0, vec![two_clubs, three_clubs]),
            (BriarCircuitSeat::Seat1, vec![]),
            (BriarCircuitSeat::Seat2, vec![]),
            (BriarCircuitSeat::Seat3, vec![]),
        ],
        PlayingTrickState {
            hearts_broken: false,
            trick_index: 0,
            leader: BriarCircuitSeat::Seat0,
            active_seat: BriarCircuitSeat::Seat0,
            current_trick: CurrentTrick::new(BriarCircuitSeat::Seat0),
        },
    );

    assert_eq!(
        legal_play_cards(&state, BriarCircuitSeat::Seat0).expect("legal cards"),
        vec![two_clubs]
    );
    assert_eq!(
        validate_play_card(&state, BriarCircuitSeat::Seat0, three_clubs)
            .expect_err("non-two-clubs opening rejects")
            .code,
        "BC_TWO_CLUBS_MUST_OPEN"
    );

    apply_play_action(
        &mut state,
        BriarCircuitSeat::Seat0,
        PlayAction::Play(two_clubs),
    )
    .expect("two clubs opens");
}

#[test]
fn followers_must_follow_led_suit_when_able() {
    let led = card(Rank::Ten, Suit::Spades);
    let spade = card(Rank::Three, Suit::Spades);
    let club = card(Rank::Ace, Suit::Clubs);
    let state = trick_state(
        vec![
            (BriarCircuitSeat::Seat0, vec![]),
            (BriarCircuitSeat::Seat1, vec![club, spade]),
            (BriarCircuitSeat::Seat2, vec![]),
            (BriarCircuitSeat::Seat3, vec![]),
        ],
        PlayingTrickState {
            hearts_broken: false,
            trick_index: 1,
            leader: BriarCircuitSeat::Seat0,
            active_seat: BriarCircuitSeat::Seat1,
            current_trick: CurrentTrick {
                leader: BriarCircuitSeat::Seat0,
                plays: vec![TrickPlay {
                    seat: BriarCircuitSeat::Seat0,
                    card: led,
                }],
            },
        },
    );

    assert_eq!(
        legal_play_cards(&state, BriarCircuitSeat::Seat1).expect("legal cards"),
        vec![spade]
    );
    assert_eq!(
        validate_play_card(&state, BriarCircuitSeat::Seat1, club)
            .expect_err("off-suit rejects")
            .code,
        "BC_MUST_FOLLOW_SUIT"
    );
}

#[test]
fn first_trick_forbids_points_only_while_non_point_discard_exists() {
    let led = card(Rank::Two, Suit::Clubs);
    let heart = card(Rank::Ace, Suit::Hearts);
    let queen_spades = card(Rank::Queen, Suit::Spades);
    let diamond = card(Rank::Four, Suit::Diamonds);
    let state = trick_state(
        vec![
            (BriarCircuitSeat::Seat0, vec![]),
            (BriarCircuitSeat::Seat1, vec![heart, queen_spades, diamond]),
            (BriarCircuitSeat::Seat2, vec![]),
            (BriarCircuitSeat::Seat3, vec![]),
        ],
        PlayingTrickState {
            hearts_broken: false,
            trick_index: 0,
            leader: BriarCircuitSeat::Seat0,
            active_seat: BriarCircuitSeat::Seat1,
            current_trick: CurrentTrick {
                leader: BriarCircuitSeat::Seat0,
                plays: vec![TrickPlay {
                    seat: BriarCircuitSeat::Seat0,
                    card: led,
                }],
            },
        },
    );

    assert_eq!(
        legal_play_cards(&state, BriarCircuitSeat::Seat1).expect("legal cards"),
        vec![diamond]
    );
    assert_eq!(
        validate_play_card(&state, BriarCircuitSeat::Seat1, heart)
            .expect_err("first-trick heart rejects")
            .code,
        "BC_FIRST_TRICK_POINT_FORBIDDEN"
    );

    let exception_state = trick_state(
        vec![
            (BriarCircuitSeat::Seat0, vec![]),
            (BriarCircuitSeat::Seat1, vec![heart, queen_spades]),
            (BriarCircuitSeat::Seat2, vec![]),
            (BriarCircuitSeat::Seat3, vec![]),
        ],
        state.playing_state().expect("playing").clone(),
    );
    assert_eq!(
        legal_play_cards(&exception_state, BriarCircuitSeat::Seat1).expect("legal cards"),
        vec![heart, queen_spades]
    );
}

#[test]
fn unbroken_hearts_cannot_be_led_until_only_hearts_remain() {
    let heart = card(Rank::Five, Suit::Hearts);
    let club = card(Rank::Five, Suit::Clubs);
    let mut state = trick_state(
        vec![
            (BriarCircuitSeat::Seat0, vec![heart, club]),
            (BriarCircuitSeat::Seat1, vec![]),
            (BriarCircuitSeat::Seat2, vec![]),
            (BriarCircuitSeat::Seat3, vec![]),
        ],
        PlayingTrickState {
            hearts_broken: false,
            trick_index: 1,
            leader: BriarCircuitSeat::Seat0,
            active_seat: BriarCircuitSeat::Seat0,
            current_trick: CurrentTrick::new(BriarCircuitSeat::Seat0),
        },
    );

    assert_eq!(
        legal_play_cards(&state, BriarCircuitSeat::Seat0).expect("legal cards"),
        vec![club]
    );
    assert_eq!(
        validate_play_card(&state, BriarCircuitSeat::Seat0, heart)
            .expect_err("heart lead rejects")
            .code,
        "BC_HEARTS_NOT_BROKEN"
    );

    state.private_hands = vec![
        (BriarCircuitSeat::Seat0, vec![heart]),
        (BriarCircuitSeat::Seat1, vec![]),
        (BriarCircuitSeat::Seat2, vec![]),
        (BriarCircuitSeat::Seat3, vec![]),
    ];
    apply_play_action(&mut state, BriarCircuitSeat::Seat0, PlayAction::Play(heart))
        .expect("only-heart lead succeeds");
    assert!(state.playing_state().expect("playing").hearts_broken);
}

#[test]
fn hearts_break_on_hearts_but_not_queen_spades() {
    let queen_spades = card(Rank::Queen, Suit::Spades);
    let mut state = trick_state(
        vec![
            (BriarCircuitSeat::Seat0, vec![queen_spades]),
            (BriarCircuitSeat::Seat1, vec![]),
            (BriarCircuitSeat::Seat2, vec![]),
            (BriarCircuitSeat::Seat3, vec![]),
        ],
        PlayingTrickState {
            hearts_broken: false,
            trick_index: 1,
            leader: BriarCircuitSeat::Seat0,
            active_seat: BriarCircuitSeat::Seat0,
            current_trick: CurrentTrick::new(BriarCircuitSeat::Seat0),
        },
    );

    apply_play_action(
        &mut state,
        BriarCircuitSeat::Seat0,
        PlayAction::Play(queen_spades),
    )
    .expect("queen spades lead succeeds");
    assert!(!state.playing_state().expect("playing").hearts_broken);
}

#[test]
fn highest_led_suit_wins_and_winner_leads_next() {
    let lead = card(Rank::Ten, Suit::Clubs);
    let low_club = card(Rank::Two, Suit::Clubs);
    let off_suit_ace = card(Rank::Ace, Suit::Spades);
    let high_club = card(Rank::Ace, Suit::Clubs);
    let mut state = trick_state(
        vec![
            (BriarCircuitSeat::Seat0, vec![lead]),
            (BriarCircuitSeat::Seat1, vec![low_club]),
            (BriarCircuitSeat::Seat2, vec![off_suit_ace]),
            (BriarCircuitSeat::Seat3, vec![high_club]),
        ],
        PlayingTrickState {
            hearts_broken: true,
            trick_index: 1,
            leader: BriarCircuitSeat::Seat0,
            active_seat: BriarCircuitSeat::Seat0,
            current_trick: CurrentTrick::new(BriarCircuitSeat::Seat0),
        },
    );

    for (seat, played) in [
        (BriarCircuitSeat::Seat0, lead),
        (BriarCircuitSeat::Seat1, low_club),
        (BriarCircuitSeat::Seat2, off_suit_ace),
        (BriarCircuitSeat::Seat3, high_club),
    ] {
        apply_play_action(&mut state, seat, PlayAction::Play(played)).expect("play succeeds");
    }

    assert_eq!(state.captured_tricks.len(), 1);
    assert_eq!(state.captured_tricks[0].winner, BriarCircuitSeat::Seat3);
    assert_eq!(state.captured_tricks[0].plays.len(), 4);
    let play = state.playing_state().expect("next trick");
    assert_eq!(play.trick_index, 2);
    assert_eq!(play.leader, BriarCircuitSeat::Seat3);
    assert_eq!(play.active_seat, BriarCircuitSeat::Seat3);
}

#[test]
fn scoring_counts_hearts_and_queen_spades_without_moon() {
    let scoring = score_completed_hand(
        &[
            captured_trick(
                BriarCircuitSeat::Seat0,
                vec![
                    card(Rank::Two, Suit::Hearts),
                    card(Rank::Queen, Suit::Spades),
                ],
            ),
            captured_trick(
                BriarCircuitSeat::Seat2,
                vec![card(Rank::Ace, Suit::Hearts), card(Rank::King, Suit::Clubs)],
            ),
        ],
        [10, 20, 30, 40],
    );

    assert_eq!(scoring.raw_points, [14, 0, 1, 0]);
    assert_eq!(scoring.hand_additions, [14, 0, 1, 0]);
    assert_eq!(scoring.cumulative_after, [24, 20, 31, 40]);
    assert_eq!(scoring.moon_shooter, None);
    let seat0 = &scoring.outcome.seats[BriarCircuitSeat::Seat0.index()];
    assert_eq!(seat0.raw_hearts_count, 1);
    assert!(seat0.captured_queen_spades);
    assert_eq!(seat0.moon_status, MoonStatus::None);
}

#[test]
fn shoot_the_moon_adds_zero_to_shooter_and_twenty_six_to_opponents() {
    let mut moon_cards = Vec::new();
    for rank in Rank::ALL {
        moon_cards.push(card(rank, Suit::Hearts));
    }
    moon_cards.push(card(Rank::Queen, Suit::Spades));

    let scoring = score_completed_hand(
        &[captured_trick(BriarCircuitSeat::Seat2, moon_cards)],
        [70, 71, 72, 73],
    );

    assert_eq!(scoring.raw_points, [0, 0, 26, 0]);
    assert_eq!(scoring.moon_shooter, Some(BriarCircuitSeat::Seat2));
    assert_eq!(scoring.hand_additions, [26, 26, 0, 26]);
    assert_eq!(scoring.cumulative_after, [96, 97, 72, 99]);
    assert_eq!(
        scoring.outcome.seats[BriarCircuitSeat::Seat2.index()].moon_status,
        MoonStatus::Shooter
    );
    assert_eq!(
        scoring.outcome.seats[BriarCircuitSeat::Seat0.index()].moon_status,
        MoonStatus::OpponentAdjusted
    );
}

#[test]
fn threshold_unique_low_score_produces_terminal_winner() {
    let scoring = score_completed_hand(
        &[captured_trick(
            BriarCircuitSeat::Seat3,
            vec![card(Rank::Queen, Suit::Spades)],
        )],
        [88, 99, 101, 90],
    );

    assert!(scoring.outcome.threshold_reached);
    assert_eq!(
        scoring.outcome.status,
        OutcomeStatus::Terminal {
            winner: BriarCircuitSeat::Seat0,
            losers: vec![
                BriarCircuitSeat::Seat1,
                BriarCircuitSeat::Seat2,
                BriarCircuitSeat::Seat3,
            ],
        }
    );
}

#[test]
fn threshold_tied_low_score_continues_without_seat_order_break() {
    let scoring = score_completed_hand(
        &[captured_trick(
            BriarCircuitSeat::Seat3,
            vec![card(Rank::Queen, Suit::Spades)],
        )],
        [80, 80, 101, 90],
    );

    assert!(scoring.outcome.threshold_reached);
    assert_eq!(
        scoring.outcome.status,
        OutcomeStatus::TiedLowContinuation {
            tied_low_score: 80,
            tied_seats: vec![BriarCircuitSeat::Seat0, BriarCircuitSeat::Seat1],
        }
    );
}

/// Drives the active seat through one full hand using only Rust-legal actions.
/// Returns when the hand index advances, a terminal outcome is reached, or a
/// generous step cap is hit (which indicates a soft-lock).
fn drive_until_hand_advances(state: &mut BriarCircuitState) {
    let start_hand = state.hand_index;
    for _ in 0..400 {
        enum Step {
            Select(BriarCircuitSeat, briar_circuit::CardId),
            Confirm(BriarCircuitSeat),
            Play(BriarCircuitSeat, briar_circuit::CardId),
            Stop(&'static str),
            Done,
        }
        let step = match &state.phase {
            Phase::Passing(pass) => {
                let seat = BriarCircuitSeat::ALL
                    .into_iter()
                    .find(|seat| !pass.is_committed(*seat))
                    .expect("an uncommitted seat exists during passing");
                let selection = pass.selection_for(seat);
                if selection.len() < 3 {
                    let card = *state
                        .hand_for_internal(seat)
                        .iter()
                        .find(|card| !selection.contains(card))
                        .expect("a selectable owned card exists");
                    Step::Select(seat, card)
                } else {
                    Step::Confirm(seat)
                }
            }
            Phase::PlayingTrick(play) => {
                let seat = play.active_seat;
                let legal = legal_play_cards(state, seat).expect("legal play cards");
                let card = *legal.first().expect("at least one legal play exists");
                Step::Play(seat, card)
            }
            Phase::ScoringHand(_) => {
                Step::Stop("soft-locked in ScoringHand; next hand never dealt")
            }
            Phase::Terminal(_) => Step::Done,
        };
        match step {
            Step::Select(seat, card) => {
                apply_pass_action(state, seat, PassAction::Select(card)).expect("select legal");
            }
            Step::Confirm(seat) => {
                apply_pass_action(state, seat, PassAction::Confirm).expect("confirm legal");
            }
            Step::Play(seat, card) => {
                apply_play_action(state, seat, PlayAction::Play(card)).expect("play legal");
            }
            Step::Stop(reason) => panic!("{reason}"),
            Step::Done => return,
        }
        if state.hand_index != start_hand {
            return;
        }
    }
    panic!("hand did not advance within the step cap");
}

#[test]
fn completed_non_terminal_hand_deals_and_advances_to_next_hand() {
    let mut state =
        setup_match(Seed(1606), &seats(4), &SetupOptions::default()).expect("setup succeeds");
    assert_eq!(state.hand_index, 0);
    assert_eq!(state.dealer, BriarCircuitSeat::Seat0);

    drive_until_hand_advances(&mut state);

    // A single hand can score at most 26 points, well under the 100 threshold, so
    // the match must continue into a freshly dealt second hand rather than freeze.
    assert_eq!(state.hand_index, 1, "match advanced to the second hand");
    assert_eq!(
        state.dealer,
        next_dealer(BriarCircuitSeat::Seat0),
        "dealer rotates clockwise after a completed hand"
    );
    assert!(
        matches!(state.phase, Phase::Passing(_)),
        "hand index 1 uses the right-pass selection phase"
    );
    assert!(
        state.captured_tricks.is_empty(),
        "captured tricks reset for the new hand so scoring does not double-count"
    );
    for seat in BriarCircuitSeat::ALL {
        assert_eq!(
            state.hand_for_internal(seat).len(),
            13,
            "every seat is re-dealt a full hand"
        );
    }
}

#[test]
fn second_hand_deal_is_deterministic_for_a_fixed_seed() {
    let mut first =
        setup_match(Seed(4242), &seats(4), &SetupOptions::default()).expect("setup succeeds");
    let mut second =
        setup_match(Seed(4242), &seats(4), &SetupOptions::default()).expect("setup succeeds");
    drive_until_hand_advances(&mut first);
    drive_until_hand_advances(&mut second);
    assert_eq!(first.hand_index, 1);
    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary(),
        "the same seed reproduces the same second-hand deal"
    );
}

#[test]
fn completed_hand_retains_public_scoring_summary_for_between_hands_display() {
    let mut state =
        setup_match(Seed(1606), &seats(4), &SetupOptions::default()).expect("setup succeeds");
    assert!(
        state.last_hand_summary.is_none(),
        "no hand summary exists before any hand completes"
    );

    drive_until_hand_advances(&mut state);

    let summary = state
        .last_hand_summary
        .as_ref()
        .expect("a completed hand retains its public scoring summary");
    assert_eq!(
        summary.cumulative_after, state.cumulative_scores,
        "summary cumulative matches the live cumulative scores"
    );
    let total_additions: u16 = summary.hand_additions.iter().map(|v| u16::from(*v)).sum();
    assert!(
        total_additions == 26 || total_additions == 78,
        "hand additions conserve the 26 raw points (or 78 across a moon), got {total_additions}"
    );
}
