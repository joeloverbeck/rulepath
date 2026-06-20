use std::collections::BTreeSet;

use briar_circuit::{
    apply_pass_action, apply_play_action, canonical_deck, canonical_seat_ids, legal_play_cards,
    setup_match, trick_winner, BriarCircuitSeat, Card, CardId, CurrentTrick, PassAction, Phase,
    PlayAction, PlayingTrickState, Rank, SetupOptions, Suit, TrickPlay, STANDARD_CARD_COUNT,
    STANDARD_HAND_SIZE,
};
use engine_core::Seed;

#[test]
fn canonical_deck_contains_fifty_two_unique_cards_in_stable_order() {
    let deck = canonical_deck();

    assert_eq!(deck.len(), STANDARD_CARD_COUNT as usize);
    assert_eq!(deck.iter().copied().collect::<BTreeSet<_>>().len(), 52);
    assert_eq!(deck[0], Card::new(Rank::Two, Suit::Clubs).id());
    assert_eq!(deck[12], Card::new(Rank::Ace, Suit::Clubs).id());
    assert_eq!(deck[13], Card::new(Rank::Two, Suit::Diamonds).id());
    assert_eq!(deck[26], Card::new(Rank::Two, Suit::Hearts).id());
    assert_eq!(deck[39], Card::new(Rank::Two, Suit::Spades).id());
    assert_eq!(deck[51], Card::new(Rank::Ace, Suit::Spades).id());
}

#[test]
fn card_ids_round_trip_to_cards_and_strings() {
    for card_id in canonical_deck() {
        let card = card_id.card();

        assert_eq!(card.id(), card_id);
        assert_eq!(CardId::parse(&card.id_str()), Some(card_id));
    }
}

#[test]
fn point_cards_match_briar_circuit_penalty_values() {
    assert_eq!(Card::new(Rank::Two, Suit::Hearts).point_value(), 1);
    assert_eq!(Card::new(Rank::Ace, Suit::Hearts).point_value(), 1);
    assert_eq!(Card::new(Rank::Queen, Suit::Spades).point_value(), 13);
    assert_eq!(Card::new(Rank::Queen, Suit::Clubs).point_value(), 0);
}

#[test]
fn setup_deals_full_deck_into_four_private_hands() {
    let state = setup_match(Seed(1605), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");
    let mut all_dealt = Vec::new();

    for seat in briar_circuit::BriarCircuitSeat::ALL {
        let hand = state.hand_for_internal(seat);
        assert_eq!(hand.len(), STANDARD_HAND_SIZE as usize);
        all_dealt.extend_from_slice(hand);
    }

    all_dealt.sort();
    assert_eq!(all_dealt, canonical_deck());
    assert_eq!(all_dealt.len(), STANDARD_CARD_COUNT as usize);
}

#[test]
fn pass_exchange_conserves_full_deck_partition() {
    let mut state = setup_match(Seed(1607), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");
    let selected: Vec<_> = BriarCircuitSeat::ALL
        .into_iter()
        .map(|seat| (seat, state.hand_for_internal(seat)[..3].to_vec()))
        .collect();

    for (seat, cards) in selected {
        for card in cards {
            apply_pass_action(&mut state, seat, PassAction::Select(card))
                .expect("select pass card");
        }
        apply_pass_action(&mut state, seat, PassAction::Confirm).expect("confirm pass");
    }

    let mut all_after = Vec::new();
    for seat in BriarCircuitSeat::ALL {
        let hand = state.hand_for_internal(seat);
        assert_eq!(hand.len(), STANDARD_HAND_SIZE as usize);
        all_after.extend_from_slice(hand);
    }

    all_after.sort();
    assert_eq!(all_after, canonical_deck());
    assert_eq!(
        all_after.iter().copied().collect::<BTreeSet<_>>().len(),
        STANDARD_CARD_COUNT as usize
    );
}

fn card(rank: Rank, suit: Suit) -> CardId {
    Card::new(rank, suit).id()
}

fn trick_state(
    hands: Vec<(BriarCircuitSeat, Vec<CardId>)>,
    play: PlayingTrickState,
) -> briar_circuit::BriarCircuitState {
    let mut state = setup_match(Seed(1610), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");
    state.private_hands = hands;
    state.phase = Phase::PlayingTrick(play);
    state
}

#[test]
fn legal_set_is_non_empty_for_acting_non_terminal_play_state() {
    let state = trick_state(
        vec![
            (
                BriarCircuitSeat::Seat0,
                vec![
                    card(Rank::Two, Suit::Hearts),
                    card(Rank::Three, Suit::Clubs),
                ],
            ),
            (BriarCircuitSeat::Seat1, vec![]),
            (BriarCircuitSeat::Seat2, vec![]),
            (BriarCircuitSeat::Seat3, vec![]),
        ],
        PlayingTrickState {
            hearts_broken: false,
            trick_index: 4,
            leader: BriarCircuitSeat::Seat0,
            active_seat: BriarCircuitSeat::Seat0,
            current_trick: CurrentTrick::new(BriarCircuitSeat::Seat0),
        },
    );

    assert!(!legal_play_cards(&state, BriarCircuitSeat::Seat0)
        .expect("legal cards")
        .is_empty());
}

#[test]
fn follow_suit_legal_set_is_closed_over_led_suit_when_available() {
    let led = card(Rank::King, Suit::Diamonds);
    let state = trick_state(
        vec![
            (BriarCircuitSeat::Seat0, vec![]),
            (
                BriarCircuitSeat::Seat1,
                vec![
                    card(Rank::Two, Suit::Diamonds),
                    card(Rank::Ace, Suit::Clubs),
                    card(Rank::Queen, Suit::Spades),
                ],
            ),
            (BriarCircuitSeat::Seat2, vec![]),
            (BriarCircuitSeat::Seat3, vec![]),
        ],
        PlayingTrickState {
            hearts_broken: true,
            trick_index: 3,
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

    let legal = legal_play_cards(&state, BriarCircuitSeat::Seat1).expect("legal cards");
    assert!(!legal.is_empty());
    assert!(legal.iter().all(|card| card.card().suit == Suit::Diamonds));
}

#[test]
fn off_suit_cards_never_win_trick() {
    let plays = vec![
        TrickPlay {
            seat: BriarCircuitSeat::Seat0,
            card: card(Rank::Three, Suit::Clubs),
        },
        TrickPlay {
            seat: BriarCircuitSeat::Seat1,
            card: card(Rank::Ace, Suit::Spades),
        },
        TrickPlay {
            seat: BriarCircuitSeat::Seat2,
            card: card(Rank::King, Suit::Clubs),
        },
        TrickPlay {
            seat: BriarCircuitSeat::Seat3,
            card: card(Rank::Ace, Suit::Hearts),
        },
    ];

    assert_eq!(
        trick_winner(&plays).expect("winner").seat,
        BriarCircuitSeat::Seat2
    );
}

#[test]
fn captured_cards_partition_played_cards_after_completed_trick() {
    let cards = [
        card(Rank::Nine, Suit::Clubs),
        card(Rank::Four, Suit::Clubs),
        card(Rank::Ace, Suit::Diamonds),
        card(Rank::Queen, Suit::Clubs),
    ];
    let mut state = trick_state(
        vec![
            (BriarCircuitSeat::Seat0, vec![cards[0]]),
            (BriarCircuitSeat::Seat1, vec![cards[1]]),
            (BriarCircuitSeat::Seat2, vec![cards[2]]),
            (BriarCircuitSeat::Seat3, vec![cards[3]]),
        ],
        PlayingTrickState {
            hearts_broken: true,
            trick_index: 6,
            leader: BriarCircuitSeat::Seat0,
            active_seat: BriarCircuitSeat::Seat0,
            current_trick: CurrentTrick::new(BriarCircuitSeat::Seat0),
        },
    );

    for (seat, played) in BriarCircuitSeat::ALL.into_iter().zip(cards) {
        apply_play_action(&mut state, seat, PlayAction::Play(played)).expect("play succeeds");
    }

    let captured: Vec<_> = state.captured_tricks[0]
        .plays
        .iter()
        .map(|play| play.card)
        .collect();
    assert_eq!(captured, cards);
    for seat in BriarCircuitSeat::ALL {
        assert!(state.hand_for_internal(seat).is_empty());
    }
}
