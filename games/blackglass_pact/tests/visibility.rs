use blackglass_pact::{
    apply_blind_nil_choice, canonical_deck, canonical_seat_ids, legal_action_tree, observer_view,
    opening_blind_nil_effect, seat_view, setup_match, setup_match_with_scores, BlackglassSeat,
    BlindNilChoice, Card, CardId, Rank, SetupOptions, Suit,
};
use engine_core::{Actor, SeatId, Seed};

#[test]
fn pre_deal_blind_nil_surfaces_expose_no_card_identity() {
    let mut state = setup_match_with_scores(
        Seed(1807),
        &canonical_seat_ids(),
        &SetupOptions::default(),
        [0, 100],
    )
    .expect("blind nil setup succeeds");

    assert!(state.private_hands.is_empty());

    let opening_effect =
        opening_blind_nil_effect(&state).expect("blind nil window effect is public");
    assert_no_card_identity(&format!("{opening_effect:?}"));

    let actor = actor_for(BlackglassSeat::South);
    let tree = legal_action_tree(&state, &actor);
    assert_eq!(
        leaf_paths(&tree),
        vec![vec!["blind_nil", "declare"], vec!["blind_nil", "decline",]]
    );
    assert_no_card_identity(&format!("{tree:?}"));

    let effects =
        apply_blind_nil_choice(&mut state, BlackglassSeat::South, BlindNilChoice::Declared)
            .expect("first blind decision succeeds");
    assert!(state.private_hands.is_empty());
    assert_no_card_identity(&format!("{effects:?}"));
}

#[test]
fn observer_and_all_ordered_seat_pairs_do_not_leak_private_hands() {
    let state = private_hand_state();

    let public_surface = format!("{:?}", observer_view(&state));
    for seat in BlackglassSeat::ALL {
        assert_private_cards_absent(&public_surface, state.hand_for_internal(seat));
    }

    for source in BlackglassSeat::ALL {
        for target in BlackglassSeat::ALL {
            if source == target {
                continue;
            }
            let target_surface = format!("{:?}", seat_view(&state, target));
            assert_private_cards_absent(&target_surface, state.hand_for_internal(source));
        }
    }
}

#[test]
fn partner_view_does_not_grant_partner_hand_visibility() {
    let state = private_hand_state();
    let north_view = format!("{:?}", seat_view(&state, BlackglassSeat::North));

    assert_private_cards_absent(&north_view, state.hand_for_internal(BlackglassSeat::South));
    assert_private_cards_present(&north_view, state.hand_for_internal(BlackglassSeat::North));
}

fn actor_for(seat: BlackglassSeat) -> Actor {
    Actor {
        seat_id: SeatId::from_zero_based_index(seat.index() as u32),
    }
}

fn assert_no_card_identity(surface: &str) {
    let lowered = surface.to_ascii_lowercase();
    for card_id in canonical_deck() {
        assert!(
            !lowered.contains(&card_id.as_str()),
            "surface leaked card id {card_id:?}: {surface}"
        );
    }
    for forbidden in [
        "clubs", "diamonds", "hearts", "spades", "deck", "rng", "two", "three", "four", "five",
        "six", "seven", "eight", "nine", "ten", "jack", "queen", "king", "ace",
    ] {
        assert!(
            !lowered.contains(forbidden),
            "surface leaked future card/deck vocabulary {forbidden}: {surface}"
        );
    }
}

fn leaf_paths(tree: &engine_core::ActionTree) -> Vec<Vec<&str>> {
    let mut paths = Vec::new();
    for choice in &tree.root.choices {
        if let Some(next) = &choice.next {
            for leaf in &next.choices {
                paths.push(vec![choice.segment.as_str(), leaf.segment.as_str()]);
            }
        } else {
            paths.push(vec![choice.segment.as_str()]);
        }
    }
    paths
}

fn private_hand_state() -> blackglass_pact::BlackglassPactState {
    let mut state = setup_match(Seed(1820), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");
    state.private_hands = vec![
        (
            BlackglassSeat::North,
            vec![card(Rank::Two, Suit::Clubs), card(Rank::Three, Suit::Clubs)],
        ),
        (
            BlackglassSeat::East,
            vec![
                card(Rank::Four, Suit::Diamonds),
                card(Rank::Five, Suit::Diamonds),
            ],
        ),
        (
            BlackglassSeat::South,
            vec![
                card(Rank::Six, Suit::Hearts),
                card(Rank::Seven, Suit::Hearts),
            ],
        ),
        (
            BlackglassSeat::West,
            vec![
                card(Rank::Eight, Suit::Spades),
                card(Rank::Nine, Suit::Spades),
            ],
        ),
    ];
    state
}

fn assert_private_cards_absent(surface: &str, cards: &[CardId]) {
    for card in cards {
        assert!(
            !surface.contains(&format!("{card:?}")),
            "surface leaked private card debug {card:?}: {surface}"
        );
        assert!(
            !surface.contains(&card.as_str()),
            "surface leaked private card id {}: {surface}",
            card.as_str()
        );
    }
}

fn assert_private_cards_present(surface: &str, cards: &[CardId]) {
    for card in cards {
        assert!(
            surface.contains(&format!("{card:?}")),
            "own view should include own private card {card:?}: {surface}"
        );
    }
}

fn card(rank: Rank, suit: Suit) -> CardId {
    Card::new(rank, suit).id()
}
