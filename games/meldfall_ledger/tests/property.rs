use meldfall_ledger::{
    cards::{Card, CardId, Rank, Suit},
    rules::{take_new_meld_from_hand, validate_new_meld},
    state::{MeldId, MeldKind, TurnOrdinal},
};

#[test]
fn generated_same_rank_sets_always_validate_locally() {
    for rank in Rank::ALL {
        let cards = Suit::ALL
            .iter()
            .take(3)
            .copied()
            .map(|suit| Card::new(rank, suit).id())
            .collect::<Vec<_>>();

        assert_eq!(
            validate_new_meld(&cards).expect("generated set validates"),
            MeldKind::Set { rank }
        );
    }
}

#[test]
fn generated_same_suit_runs_always_validate_locally() {
    for suit in Suit::ALL {
        for ranks in Rank::ALL[1..].windows(3) {
            let cards = ranks
                .iter()
                .copied()
                .map(|rank| Card::new(rank, suit).id())
                .collect::<Vec<_>>();

            assert_eq!(
                validate_new_meld(&cards).expect("generated run validates"),
                MeldKind::Run { suit }
            );
        }

        let ace_low = cards(&[Rank::Ace, Rank::Two, Rank::Three], suit);
        let ace_high = cards(&[Rank::Queen, Rank::King, Rank::Ace], suit);
        assert_eq!(
            validate_new_meld(&ace_low).expect("ace-low run validates"),
            MeldKind::Run { suit }
        );
        assert_eq!(
            validate_new_meld(&ace_high).expect("ace-high run validates"),
            MeldKind::Run { suit }
        );
    }
}

#[test]
fn taking_generated_legal_melds_preserves_card_ownership_conservation() {
    for (index, meld_cards) in generated_legal_melds().into_iter().enumerate() {
        let spare = Card::new(Rank::King, Suit::Spades).id();
        if meld_cards.contains(&spare) {
            continue;
        }
        let before_len = meld_cards.len() + 1;
        let mut hand = meld_cards.clone();
        hand.push(spare);

        let group = take_new_meld_from_hand(
            &mut hand,
            &meld_cards,
            MeldId(index as u32),
            0,
            TurnOrdinal(index as u32),
        )
        .expect("generated legal meld is accepted from owned hand");

        assert_eq!(hand, vec![spare]);
        assert_eq!(hand.len() + group.cards.len(), before_len);
        assert_eq!(
            group
                .cards
                .iter()
                .map(|table| table.card)
                .collect::<Vec<_>>(),
            meld_cards
        );
    }
}

fn generated_legal_melds() -> Vec<Vec<CardId>> {
    let mut melds = Vec::new();
    for rank in Rank::ALL {
        melds.push(
            Suit::ALL
                .iter()
                .take(3)
                .copied()
                .map(|suit| Card::new(rank, suit).id())
                .collect(),
        );
    }
    for suit in Suit::ALL {
        for ranks in Rank::ALL[1..].windows(3) {
            melds.push(cards(ranks, suit));
        }
        melds.push(cards(&[Rank::Ace, Rank::Two, Rank::Three], suit));
        melds.push(cards(&[Rank::Queen, Rank::King, Rank::Ace], suit));
    }
    melds
}

fn cards(ranks: &[Rank], suit: Suit) -> Vec<CardId> {
    ranks
        .iter()
        .copied()
        .map(|rank| Card::new(rank, suit).id())
        .collect()
}
