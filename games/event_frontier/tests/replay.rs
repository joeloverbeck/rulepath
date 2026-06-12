use engine_core::{SeatId, Seed, StableSerialize};
use event_frontier::{setup_match, CardId, SetupOptions};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn scenario_options() -> [SetupOptions; 3] {
    [
        SetupOptions::standard(),
        SetupOptions::hard_winter(),
        SetupOptions::land_rush(),
    ]
}

#[test]
fn deterministic_setup_reproduces_deck_order_and_state_hash() {
    let seats = seats();

    for options in scenario_options() {
        let first = setup_match(Seed(99), &seats, &options).expect("first setup");
        let second = setup_match(Seed(99), &seats, &options).expect("second setup");

        assert_eq!(first.deck, second.deck);
        assert_eq!(first.stable_hash(), second.stable_hash());
        assert_eq!(first.stable_summary(), second.stable_summary());
    }
}

#[test]
fn reckoning_is_never_first_in_any_seeded_epoch() {
    let seats = seats();

    for options in scenario_options() {
        for seed in 0..150 {
            let state = setup_match(Seed(seed), &seats, &options).expect("setup");
            let mut deck = Vec::new();
            deck.extend(state.deck.current);
            deck.extend(state.deck.next_public);
            deck.extend(state.deck.undrawn);

            for epoch_start in [0, 7, 14] {
                assert!(!is_reckoning(deck[epoch_start]));
            }
        }
    }
}

fn is_reckoning(card: CardId) -> bool {
    matches!(
        card,
        CardId::ReckoningOne | CardId::ReckoningTwo | CardId::ReckoningThree
    )
}
