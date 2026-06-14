use engine_core::{EffectEnvelope, SeatId, Viewer, VisibilityScope};

use crate::{
    cards::Card,
    ids::RiverLedgerSeat,
    state::{Street, TerminalOutcome},
    RiverLedgerState,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RiverLedgerEffect {
    PrivateCardsDealt {
        owner: RiverLedgerSeat,
        cards: [Card; 2],
    },
    DealStarted {
        private_count_per_seat: u8,
        reserved_community_count: u8,
        deck_tail_count: u8,
    },
    ContributionChanged {
        seat: RiverLedgerSeat,
        amount_added: u16,
        pot_total: u16,
    },
    StreetAdvanced {
        street: Street,
        public_board: Vec<Card>,
    },
    ShowdownResolved {
        outcome: TerminalOutcome,
    },
}

pub fn public_effect(payload: RiverLedgerEffect) -> EffectEnvelope<RiverLedgerEffect> {
    EffectEnvelope {
        visibility: VisibilityScope::Public,
        payload,
    }
}

pub fn private_effect(
    owner_seat_id: SeatId,
    payload: RiverLedgerEffect,
) -> EffectEnvelope<RiverLedgerEffect> {
    EffectEnvelope {
        visibility: VisibilityScope::PrivateToSeat(owner_seat_id),
        payload,
    }
}

pub fn setup_effects(state: &RiverLedgerState) -> Vec<EffectEnvelope<RiverLedgerEffect>> {
    let mut effects = state
        .private_hands_internal()
        .iter()
        .enumerate()
        .map(|(index, hand)| {
            let seat = RiverLedgerSeat::from_index(index).expect("setup has bounded seats");
            private_effect(
                state.seats[index].clone(),
                RiverLedgerEffect::PrivateCardsDealt {
                    owner: seat,
                    cards: *hand,
                },
            )
        })
        .collect::<Vec<_>>();

    effects.push(public_effect(RiverLedgerEffect::DealStarted {
        private_count_per_seat: 2,
        reserved_community_count: state.community_deck_internal().len() as u8,
        deck_tail_count: state.deck_tail_internal().len() as u8,
    }));
    effects
}

pub fn filter_effects_for_viewer(
    effects: &[EffectEnvelope<RiverLedgerEffect>],
    viewer: &Viewer,
) -> Vec<EffectEnvelope<RiverLedgerEffect>> {
    effects
        .iter()
        .filter(|effect| match &effect.visibility {
            VisibilityScope::Public => true,
            VisibilityScope::PrivateToSeat(seat_id) => viewer.seat_id.as_ref() == Some(seat_id),
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use engine_core::{SeatId, Seed, Viewer};

    use super::*;
    use crate::{canonical_deck, setup_match, SetupOptions};

    fn seats(count: usize) -> Vec<SeatId> {
        (0..count)
            .map(|index| SeatId(format!("seat_{index}")))
            .collect()
    }

    #[test]
    fn setup_effect_filtering_keeps_private_cards_scoped_to_owner() {
        let state = setup_match(Seed(8), &seats(4), &SetupOptions::default()).expect("setup");
        let effects = setup_effects(&state);
        let observer = filter_effects_for_viewer(&effects, &Viewer { seat_id: None });
        let seat_0 = filter_effects_for_viewer(
            &effects,
            &Viewer {
                seat_id: Some(SeatId("seat_0".to_owned())),
            },
        );

        assert!(observer
            .iter()
            .all(|effect| matches!(effect.visibility, VisibilityScope::Public)));
        assert!(seat_0.iter().any(|effect| matches!(
            effect.payload,
            RiverLedgerEffect::PrivateCardsDealt {
                owner,
                ..
            } if owner == RiverLedgerSeat::from_index(0).unwrap()
        )));

        let observer_text = format!("{observer:?}");
        for card in state.private_hands_internal().iter().flatten() {
            assert!(!observer_text.contains(&card.id()));
        }
        for card in canonical_deck() {
            if !state.board.contains(&card) {
                assert!(!observer_text.contains(&card.id()));
            }
        }
    }
}
