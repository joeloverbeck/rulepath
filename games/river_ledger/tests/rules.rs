use std::collections::BTreeSet;

use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};
use river_ledger::state::SeatRoles;
use river_ledger::{
    apply_action, canonical_deck, legal_action_tree, setup_match, validate_command, Card, PotShare,
    Rank, RiverLedgerSeat, SeatLedger, SeatStatus, SetupOptions, Street, Suit, TerminalOutcome,
    Variant, STANDARD_BIG_BLIND, STANDARD_CARD_COUNT, STANDARD_SMALL_BLIND,
};

fn seats(count: usize) -> Vec<SeatId> {
    (0..count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect()
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn command(state: &river_ledger::RiverLedgerState, seat: &str, segment: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn apply_segment(state: &mut river_ledger::RiverLedgerState, seat: &str, segment: &str) {
    let action = validate_command(state, &command(state, seat, segment)).expect("valid command");
    apply_action(state, action).expect("apply succeeds");
}

fn legal_segments(state: &river_ledger::RiverLedgerState, seat: &str) -> Vec<String> {
    legal_action_tree(state, &actor(seat))
        .root
        .choices
        .into_iter()
        .map(|choice| choice.segment)
        .collect()
}

fn standard_state(count: usize) -> river_ledger::RiverLedgerState {
    setup_match(Seed(21), &seats(count), &SetupOptions::default()).expect("setup")
}

fn seeded_state(seed: u64, count: usize) -> river_ledger::RiverLedgerState {
    setup_match(Seed(seed), &seats(count), &SetupOptions::default()).expect("setup")
}

fn advance_four_player_hand_to_flop() -> river_ledger::RiverLedgerState {
    let mut state = standard_state(4);
    apply_segment(&mut state, "seat_3", "call");
    apply_segment(&mut state, "seat_0", "call");
    apply_segment(&mut state, "seat_1", "call");
    apply_segment(&mut state, "seat_2", "check");
    state
}

fn check_down_from_flop_to_terminal(state: &mut river_ledger::RiverLedgerState) {
    for seat in ["seat_1", "seat_2", "seat_3", "seat_0"] {
        apply_segment(state, seat, "check");
    }
    for seat in ["seat_1", "seat_2", "seat_3", "seat_0"] {
        apply_segment(state, seat, "check");
    }
    for seat in ["seat_1", "seat_2", "seat_3", "seat_0"] {
        apply_segment(state, seat, "check");
    }
}

fn royal_board() -> Vec<Card> {
    vec![
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Clubs),
        Card::new(Rank::Ten, Suit::Clubs),
    ]
}

fn seat(index: usize) -> RiverLedgerSeat {
    RiverLedgerSeat::from_index(index).unwrap()
}

fn custom_showdown_state(
    private_hands: Vec<[Card; 2]>,
    board: [Card; 5],
) -> river_ledger::RiverLedgerState {
    let mut state = river_ledger::RiverLedgerState::new_after_setup(
        Variant::river_ledger_standard(),
        seats(private_hands.len()),
        SeatRoles {
            button: seat(0),
            small_blind: seat(1),
            big_blind: seat(2),
            active_seat: seat(0),
        },
        private_hands,
        board,
        Vec::new(),
    );
    state.board = board.to_vec();
    state.ledger.seats = (0..state.seats.len())
        .map(|index| SeatLedger {
            seat: seat(index),
            status: SeatStatus::ShowdownEligible,
            street_contribution: 0,
            total_contribution: 3,
        })
        .collect();
    state.ledger.pot_total = 12;
    state
}

#[test]
fn setup_accepts_three_to_six_seats_and_rejects_other_counts() {
    let options = SetupOptions::default();

    for count in 3..=6 {
        setup_match(Seed(100 + count as u64), &seats(count), &options)
            .unwrap_or_else(|err| panic!("{count} seats should be accepted: {err:?}"));
    }

    for count in [0, 1, 2, 7] {
        let err = setup_match(Seed(200 + count as u64), &seats(count), &options)
            .expect_err("seat count should reject");
        assert_eq!(err.code, "invalid_seat_count");
        assert_eq!(err.message, "river_ledger requires between 3 and 6 seats");
    }
}

#[test]
fn setup_is_deterministic_for_same_seed_and_options() {
    let options = SetupOptions::default();
    let seats = seats(6);

    let first = setup_match(Seed(42), &seats, &options).expect("first setup");
    let second = setup_match(Seed(42), &seats, &options).expect("second setup");

    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
}

#[test]
fn setup_varies_shuffle_for_different_seeds() {
    let options = SetupOptions::default();
    let seats = seats(6);

    let first = setup_match(Seed(42), &seats, &options).expect("first setup");
    let second = setup_match(Seed(43), &seats, &options).expect("second setup");

    assert_ne!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
}

#[test]
fn setup_assigns_button_blinds_active_seat_and_initial_ledger() {
    let options = SetupOptions::default();
    let state = setup_match(Seed(7), &seats(4), &options).expect("setup");

    assert_eq!(state.button, RiverLedgerSeat::from_index(0).unwrap());
    assert_eq!(state.small_blind, RiverLedgerSeat::from_index(1).unwrap());
    assert_eq!(state.big_blind, RiverLedgerSeat::from_index(2).unwrap());
    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(3));
    assert_eq!(state.betting.street, Street::Preflop);
    assert_eq!(state.betting.current_to_call, u16::from(STANDARD_BIG_BLIND));
    assert_eq!(
        state.ledger.seats[1].total_contribution,
        u16::from(STANDARD_SMALL_BLIND)
    );
    assert_eq!(
        state.ledger.seats[2].total_contribution,
        u16::from(STANDARD_BIG_BLIND)
    );
    assert_eq!(
        state.ledger.pot_total,
        u16::from(STANDARD_SMALL_BLIND + STANDARD_BIG_BLIND)
    );
}

#[test]
fn setup_deals_unique_hole_cards_reserved_board_and_tail() {
    let options = SetupOptions::default();
    let state = setup_match(Seed(9), &seats(6), &options).expect("setup");

    assert_eq!(state.private_hands_internal().len(), 6);
    assert_eq!(state.community_deck_internal().len(), 5);
    assert_eq!(
        state.deck_tail_internal().len(),
        STANDARD_CARD_COUNT as usize - (6 * 2) - 5
    );

    let mut dealt = BTreeSet::new();
    for hand in state.private_hands_internal() {
        dealt.insert(hand[0]);
        dealt.insert(hand[1]);
    }
    for card in state.community_deck_internal() {
        dealt.insert(*card);
    }
    for card in state.deck_tail_internal() {
        dealt.insert(*card);
    }

    assert_eq!(dealt.len(), canonical_deck().len());
}

#[test]
fn setup_public_summary_exposes_no_hidden_card_identities() {
    let options = SetupOptions::default();
    let state = setup_match(Seed(11), &seats(3), &options).expect("setup");
    let public_summary = state.setup_public_summary();

    assert!(public_summary.contains("hole_counts=seat_0:2 hidden,seat_1:2 hidden,seat_2:2 hidden"));
    assert!(public_summary.contains("reserved_community_count=5"));
    assert!(public_summary.contains("deck_tail_count=41"));

    for card in canonical_deck() {
        assert!(
            !public_summary.contains(&card.id()),
            "public summary leaked {}",
            card.id()
        );
    }
}

#[test]
fn legal_action_generation_uses_active_seat_call_price_and_cap_state() {
    let state = standard_state(4);

    assert_eq!(
        legal_segments(&state, "seat_3"),
        vec!["fold", "call", "raise"]
    );
    assert!(legal_segments(&state, "seat_0").is_empty());

    let tree = legal_action_tree(&state, &actor("seat_3"));
    let call = tree
        .root
        .choices
        .iter()
        .find(|choice| choice.segment == "call")
        .expect("call action");
    assert!(call
        .metadata
        .iter()
        .any(|entry| entry.key == "required_to_call" && entry.value == "2"));
    assert!(call
        .metadata
        .iter()
        .any(|entry| entry.key == "adds_to_pot" && entry.value == "2"));
    assert!(call
        .metadata
        .iter()
        .any(|entry| entry.key == "cap_remaining" && entry.value == "3"));

    let mut capped = advance_four_player_hand_to_flop();
    apply_segment(&mut capped, "seat_1", "bet");
    apply_segment(&mut capped, "seat_2", "raise");
    apply_segment(&mut capped, "seat_3", "raise");
    apply_segment(&mut capped, "seat_0", "raise");
    let capped_tree = legal_action_tree(&capped, &actor("seat_1"));
    assert!(capped_tree.root.choices.iter().all(|choice| choice
        .metadata
        .iter()
        .any(|entry| entry.key == "cap_remaining" && entry.value == "0")));
}

#[test]
fn preflop_calls_and_big_blind_check_advance_to_flop() {
    let mut state = standard_state(4);

    apply_segment(&mut state, "seat_3", "call");
    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(0));
    apply_segment(&mut state, "seat_0", "call");
    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(1));
    apply_segment(&mut state, "seat_1", "call");
    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(2));
    assert_eq!(legal_segments(&state, "seat_2"), vec!["check", "raise"]);

    apply_segment(&mut state, "seat_2", "check");

    assert_eq!(
        state.phase,
        river_ledger::Phase::Betting {
            street: Street::Flop
        }
    );
    assert_eq!(state.betting.street, Street::Flop);
    assert_eq!(state.board.len(), 3);
    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(1));
    assert_eq!(state.betting.current_to_call, 0);
    assert!(state
        .ledger
        .seats
        .iter()
        .all(|entry| entry.street_contribution == 0));
    assert_eq!(state.ledger.pot_total, 8);
}

#[test]
fn flop_fixed_limit_cap_blocks_fourth_raise() {
    let mut state = advance_four_player_hand_to_flop();

    apply_segment(&mut state, "seat_1", "bet");
    assert_eq!(state.betting.current_to_call, 2);
    assert_eq!(state.ledger.pot_total, 10);

    apply_segment(&mut state, "seat_2", "raise");
    apply_segment(&mut state, "seat_3", "raise");
    apply_segment(&mut state, "seat_0", "raise");

    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(1));
    assert_eq!(state.betting.raises_this_street, 3);
    assert_eq!(state.betting.current_to_call, 8);
    assert_eq!(legal_segments(&state, "seat_1"), vec!["fold", "call"]);

    let diagnostic = validate_command(&state, &command(&state, "seat_1", "raise")).unwrap_err();
    assert_eq!(diagnostic.code, "raise_cap_reached");
}

#[test]
fn turn_and_river_use_big_bet_unit() {
    let mut state = advance_four_player_hand_to_flop();

    for seat in ["seat_1", "seat_2", "seat_3", "seat_0"] {
        apply_segment(&mut state, seat, "check");
    }
    assert_eq!(
        state.phase,
        river_ledger::Phase::Betting {
            street: Street::Turn
        }
    );
    assert_eq!(state.betting.street.unit(), 4);
    assert_eq!(state.board.len(), 4);

    apply_segment(&mut state, "seat_1", "bet");
    assert_eq!(state.ledger.seats[1].street_contribution, 4);
    assert_eq!(state.betting.current_to_call, 4);

    for seat in ["seat_2", "seat_3", "seat_0"] {
        apply_segment(&mut state, seat, "call");
    }
    assert_eq!(
        state.phase,
        river_ledger::Phase::Betting {
            street: Street::River
        }
    );
    assert_eq!(state.betting.street.unit(), 4);
    assert_eq!(state.board.len(), 5);

    let before = state.ledger.pot_total;
    apply_segment(&mut state, "seat_1", "bet");
    assert_eq!(state.ledger.pot_total, before + 4);
}

#[test]
fn river_checkdown_resolves_showdown_terminal_and_conserves_pot() {
    let mut state = advance_four_player_hand_to_flop();

    check_down_from_flop_to_terminal(&mut state);

    assert_eq!(state.phase, river_ledger::Phase::Terminal);
    assert_eq!(state.active_seat, None);
    let Some(TerminalOutcome::Showdown {
        winners,
        pot_total,
        allocations,
        explanations,
        ..
    }) = &state.terminal_outcome
    else {
        panic!("showdown terminal expected");
    };

    assert!(!winners.is_empty());
    assert_eq!(*pot_total, state.ledger.pot_total);
    assert_eq!(
        allocations.iter().map(|share| share.amount).sum::<u16>(),
        *pot_total
    );
    assert_eq!(explanations.len(), 4);
    assert!(explanations
        .iter()
        .filter(|explanation| explanation.revealed.is_some())
        .all(|explanation| explanation.status == river_ledger::SeatStatus::ShowdownEligible));
}

#[test]
fn checkdown_can_produce_single_winner_showdown_from_seeded_state() {
    let mut single_winner = None;
    for seed in 0..200 {
        let mut state = seeded_state(seed, 4);
        apply_segment(&mut state, "seat_3", "call");
        apply_segment(&mut state, "seat_0", "call");
        apply_segment(&mut state, "seat_1", "call");
        apply_segment(&mut state, "seat_2", "check");
        check_down_from_flop_to_terminal(&mut state);
        if let Some(TerminalOutcome::Showdown {
            winners,
            allocations,
            ..
        }) = &state.terminal_outcome
        {
            if winners.len() == 1 {
                single_winner = Some((winners[0], allocations.clone(), state.ledger.pot_total));
                break;
            }
        }
    }

    let Some((winner, allocations, pot_total)) = single_winner else {
        panic!("expected a deterministic seed with one showdown winner");
    };
    assert_eq!(
        allocations,
        vec![PotShare {
            seat: winner,
            amount: pot_total,
        }]
    );
}

#[test]
fn tied_showdown_splits_pot_and_reveals_only_showdown_seats() {
    let mut state = standard_state(4);
    state.board = royal_board();
    state.ledger.pot_total = 12;
    for entry in &mut state.ledger.seats {
        entry.status = river_ledger::SeatStatus::ShowdownEligible;
    }
    state.ledger.seats[0].status = river_ledger::SeatStatus::Folded;

    let outcome = river_ledger::resolve_showdown(&state);
    let TerminalOutcome::Showdown {
        winners,
        allocations,
        explanations,
        ..
    } = outcome
    else {
        panic!("showdown expected");
    };

    assert_eq!(
        winners,
        vec![
            RiverLedgerSeat::from_index(1).unwrap(),
            RiverLedgerSeat::from_index(2).unwrap(),
            RiverLedgerSeat::from_index(3).unwrap(),
        ]
    );
    assert_eq!(
        allocations,
        vec![
            PotShare {
                seat: RiverLedgerSeat::from_index(1).unwrap(),
                amount: 4,
            },
            PotShare {
                seat: RiverLedgerSeat::from_index(2).unwrap(),
                amount: 4,
            },
            PotShare {
                seat: RiverLedgerSeat::from_index(3).unwrap(),
                amount: 4,
            },
        ]
    );
    assert!(explanations
        .iter()
        .find(|explanation| explanation.seat == RiverLedgerSeat::from_index(0).unwrap())
        .expect("folded seat explanation")
        .revealed
        .is_none());
    assert!(explanations
        .iter()
        .filter(|explanation| explanation.seat != RiverLedgerSeat::from_index(0).unwrap())
        .all(|explanation| explanation.revealed.is_some()));
}

#[test]
fn showdown_explanation_names_pair_of_queens_beating_pair_of_eights() {
    let state = custom_showdown_state(
        vec![
            [
                Card::new(Rank::Eight, Suit::Diamonds),
                Card::new(Rank::Ten, Suit::Clubs),
            ],
            [
                Card::new(Rank::Queen, Suit::Clubs),
                Card::new(Rank::Ten, Suit::Spades),
            ],
            [
                Card::new(Rank::Ace, Suit::Spades),
                Card::new(Rank::Two, Suit::Clubs),
            ],
            [
                Card::new(Rank::King, Suit::Spades),
                Card::new(Rank::Two, Suit::Spades),
            ],
        ],
        [
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Six, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Hearts),
        ],
    );

    let outcome = river_ledger::resolve_showdown(&state);
    let TerminalOutcome::Showdown {
        winners,
        headline,
        decisive_comparison,
        comparison_basis,
        explanations,
        ..
    } = outcome
    else {
        panic!("showdown expected");
    };

    assert_eq!(winners, vec![seat(1)]);
    assert_eq!(headline, "seat_1 wins with Pair of Queens.");
    assert_eq!(decisive_comparison, "Pair of Queens beats Pair of Eights.");
    assert_eq!(
        comparison_basis,
        "Both hands are one pair, so the pair rank decides first: Queens > Eights."
    );

    let winner = explanations
        .iter()
        .find(|explanation| explanation.seat == seat(1))
        .and_then(|explanation| explanation.revealed.as_ref())
        .expect("winner reveal");
    assert_eq!(winner.category, "one_pair");
    assert_eq!(winner.tie_break_vector, vec![12, 10, 8, 6]);
    assert_eq!(winner.result_label, "Win");
    assert_eq!(winner.hand_name, "Pair of Queens");
    assert_eq!(
        winner.rank_explanation,
        "pair rank Queen; kickers Ten, Eight, Six"
    );
    assert_eq!(
        winner.comparison_note,
        "Pair of Queens beats Pair of Eights."
    );
    assert_eq!(
        winner.best_five_accessibility_label,
        "Best five cards: queen of clubs, queen of hearts, ten of spades, eight of hearts, six of hearts."
    );

    let closest_loser = explanations
        .iter()
        .find(|explanation| explanation.seat == seat(0))
        .and_then(|explanation| explanation.revealed.as_ref())
        .expect("closest loser reveal");
    assert_eq!(closest_loser.result_label, "Showdown loss");
    assert_eq!(closest_loser.hand_name, "Pair of Eights");
    assert_eq!(
        closest_loser.comparison_note,
        "Pair of Eights loses to Pair of Queens."
    );
}

#[test]
fn showdown_explanation_marks_split_and_folded_paths() {
    let mut state = custom_showdown_state(
        vec![
            [
                Card::new(Rank::Two, Suit::Clubs),
                Card::new(Rank::Three, Suit::Clubs),
            ],
            [
                Card::new(Rank::Two, Suit::Diamonds),
                Card::new(Rank::Three, Suit::Diamonds),
            ],
            [
                Card::new(Rank::Two, Suit::Hearts),
                Card::new(Rank::Three, Suit::Hearts),
            ],
        ],
        [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
        ],
    );
    state.ledger.seats[2].status = SeatStatus::Folded;

    let outcome = river_ledger::resolve_showdown(&state);
    let TerminalOutcome::Showdown {
        winners,
        headline,
        decisive_comparison,
        comparison_basis,
        explanations,
        ..
    } = outcome
    else {
        panic!("showdown expected");
    };

    assert_eq!(winners, vec![seat(0), seat(1)]);
    assert_eq!(
        headline,
        "seat_0 and seat_1 split the ledger with Ace-high straight flush."
    );
    assert_eq!(
        decisive_comparison,
        "seat_0 and seat_1 all hold Ace-high straight flush, so the ledger is split."
    );
    assert_eq!(
        comparison_basis,
        "The best revealed hands have equal category and tie-break ranks."
    );

    for index in [0, 1] {
        let reveal = explanations
            .iter()
            .find(|explanation| explanation.seat == seat(index))
            .and_then(|explanation| explanation.revealed.as_ref())
            .expect("split reveal");
        assert_eq!(reveal.result_label, "Split win");
        assert_eq!(
            reveal.comparison_note,
            "Ties for the best hand and shares the ledger."
        );
    }
    assert!(explanations
        .iter()
        .find(|explanation| explanation.seat == seat(2))
        .expect("folded explanation")
        .revealed
        .is_none());
}

#[test]
fn single_pot_remainder_uses_stable_button_order() {
    let seat = |index| RiverLedgerSeat::from_index(index).unwrap();
    let allocation =
        river_ledger::pot::allocate_single_pot(11, &[seat(0), seat(2), seat(3)], seat(2), 4);

    assert_eq!(allocation.remainder, 2);
    assert_eq!(
        allocation.shares,
        vec![
            PotShare {
                seat: seat(2),
                amount: 4,
            },
            PotShare {
                seat: seat(3),
                amount: 4,
            },
            PotShare {
                seat: seat(0),
                amount: 3,
            },
        ]
    );
    assert_eq!(
        allocation
            .shares
            .iter()
            .map(|share| share.amount)
            .sum::<u16>(),
        allocation.pot_total
    );
}

#[test]
fn six_seat_preflop_wraparound_order_is_stable() {
    let mut state = standard_state(6);

    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(3));
    for expected in [3, 4, 5, 0, 1] {
        let seat = format!("seat_{expected}");
        apply_segment(&mut state, &seat, "call");
        assert_eq!(
            state.active_seat,
            RiverLedgerSeat::from_index((expected + 1) % 6)
        );
    }

    apply_segment(&mut state, "seat_2", "check");
    assert_eq!(
        state.phase,
        river_ledger::Phase::Betting {
            street: Street::Flop
        }
    );
    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(1));
    assert_eq!(state.ledger.pot_total, 12);
}

#[test]
fn foldout_terminal_awards_pot_without_showdown_reveal() {
    let mut state = standard_state(3);

    apply_segment(&mut state, "seat_0", "fold");
    apply_segment(&mut state, "seat_1", "fold");

    assert_eq!(state.phase, river_ledger::Phase::Terminal);
    assert_eq!(state.active_seat, None);
    assert_eq!(
        state.terminal_outcome,
        Some(TerminalOutcome::LastLiveHand {
            winner: RiverLedgerSeat::from_index(2).unwrap(),
            pot_total: 3,
        })
    );

    let terminal = format!("{:?}", state.terminal_outcome);
    for card in canonical_deck() {
        assert!(
            !terminal.contains(&card.id()),
            "terminal foldout leaked {}",
            card.id()
        );
    }
}

#[test]
fn diagnostics_are_fail_closed_and_public_only() {
    let state = standard_state(4);

    let wrong_seat = validate_command(&state, &command(&state, "seat_0", "call")).unwrap_err();
    assert_eq!(wrong_seat.code, "wrong_seat");

    let mut stale = command(&state, "seat_3", "call");
    stale.freshness_token = FreshnessToken(99);
    let stale = validate_command(&state, &stale).unwrap_err();
    assert_eq!(stale.code, "stale_action");

    let mut capped = advance_four_player_hand_to_flop();
    apply_segment(&mut capped, "seat_1", "bet");
    apply_segment(&mut capped, "seat_2", "raise");
    apply_segment(&mut capped, "seat_3", "raise");
    apply_segment(&mut capped, "seat_0", "raise");
    let cap = validate_command(&capped, &command(&capped, "seat_1", "raise")).unwrap_err();
    assert_eq!(cap.code, "raise_cap_reached");

    for diagnostic in [wrong_seat, stale, cap] {
        let rendered = format!("{diagnostic:?}");
        for forbidden in ["hidden", "private", "deck", "hole", "rank", "suit"] {
            assert!(!rendered.contains(forbidden), "{rendered}");
        }
        for card in canonical_deck() {
            assert!(
                !rendered.contains(&card.id()),
                "diagnostic leaked {} in {rendered}",
                card.id()
            );
        }
    }
}

#[test]
fn identical_command_stream_is_deterministic() {
    fn run() -> river_ledger::RiverLedgerState {
        let mut state = standard_state(4);
        for (seat, segment) in [
            ("seat_3", "call"),
            ("seat_0", "call"),
            ("seat_1", "call"),
            ("seat_2", "check"),
            ("seat_1", "bet"),
            ("seat_2", "raise"),
            ("seat_3", "call"),
            ("seat_0", "call"),
            ("seat_1", "call"),
        ] {
            apply_segment(&mut state, seat, segment);
        }
        state
    }

    let first = run();
    let second = run();

    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
    assert_eq!(first.ledger, second.ledger);
    assert_eq!(first.betting, second.betting);
}

#[test]
fn legal_action_tree_metadata_remains_public_and_stable() {
    let state = standard_state(4);
    let allowed = [
        "action_family",
        "street",
        "street_unit",
        "actor_seat",
        "required_to_call",
        "adds_to_pot",
        "pot_after",
        "raises_remaining",
        "cap_remaining",
        "accessibility_copy",
    ];

    for choice in legal_action_tree(&state, &actor("seat_3")).root.choices {
        assert!(choice
            .metadata
            .iter()
            .all(|entry| allowed.contains(&entry.key.as_str())));
        let serialized = format!("{choice:?}");
        for forbidden in ["hidden", "private", "deck", "hole", "rank", "suit"] {
            assert!(!serialized.contains(forbidden), "{serialized}");
        }
    }
}
