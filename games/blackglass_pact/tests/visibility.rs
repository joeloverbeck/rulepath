use blackglass_pact::{
    apply_blind_nil_choice, canonical_deck, canonical_seat_ids, legal_action_tree,
    opening_blind_nil_effect, setup_match_with_scores, BlackglassSeat, BlindNilChoice,
    SetupOptions,
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
