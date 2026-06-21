use std::collections::BTreeSet;

use engine_core::Seed;
use vow_tide::{
    cards::{canonical_deck, Card, CardId, Rank, Suit},
    ids::{canonical_seat_ids, STANDARD_CARD_COUNT},
    setup::{setup_match, SetupOptions},
    variants::{expected_manifest, load_manifest, load_variants, Variant},
};

#[test]
fn canonical_deck_order_is_complete_and_stable() {
    let deck = canonical_deck();

    assert_eq!(deck.len(), STANDARD_CARD_COUNT as usize);
    assert_eq!(
        deck.iter().copied().collect::<BTreeSet<_>>().len(),
        deck.len()
    );
    assert_eq!(
        deck.first().copied(),
        Some(Card::new(Rank::Two, Suit::Clubs).id())
    );
    assert_eq!(
        deck.last().copied(),
        Some(Card::new(Rank::Ace, Suit::Spades).id())
    );

    for (index, card_id) in deck.iter().enumerate() {
        assert_eq!(card_id.index(), index as u8);
        assert_eq!(CardId::parse(&card_id.as_str()), Some(*card_id));
    }
}

#[test]
fn setup_state_summary_is_deterministic_and_ordered() {
    let options = SetupOptions::default();
    let seats = canonical_seat_ids(5);

    let first = setup_match(Seed(42), &seats, &options).expect("first setup succeeds");
    let second = setup_match(Seed(42), &seats, &options).expect("second setup succeeds");

    assert_eq!(first, second);
    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
    assert!(first
        .stable_internal_summary()
        .contains("schedule=[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"));
}

#[test]
fn metadata_stubs_load_inert_standard_content() {
    let manifest = load_manifest().expect("manifest parses");
    let variants = load_variants().expect("variants parse");

    assert_eq!(manifest, expected_manifest());
    assert_eq!(variants.selected, Variant::vow_tide_standard());
}
