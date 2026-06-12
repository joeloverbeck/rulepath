use engine_core::{SeatId, Seed, StableSerialize};
use event_frontier::{resolve_reckoning, setup_match, CardId, CardPhase, SetupOptions};

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

#[test]
fn reckoning_breakdown_scores_and_terminal_reproduce_for_same_state() {
    let seats = seats();
    let mut first = setup_match(Seed(1), &seats, &SetupOptions::default()).expect("setup");
    let mut second = first.clone();
    for state in [&mut first, &mut second] {
        state.deck.current = Some(CardId::ReckoningOne);
        state.card_phase = CardPhase::Reckoning;
    }

    let first_result = resolve_reckoning(&mut first).expect("first reckoning");
    let second_result = resolve_reckoning(&mut second).expect("second reckoning");

    assert_eq!(first.scores, second.scores);
    assert_eq!(first.terminal_outcome, second.terminal_outcome);
    assert_eq!(
        format!("{:?}", first_result.effects),
        format!("{:?}", second_result.effects)
    );
    assert_eq!(first.stable_hash(), second.stable_hash());
}
