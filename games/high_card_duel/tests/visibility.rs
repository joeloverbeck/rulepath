use engine_core::{SeatId, VisibilityScope};
use high_card_duel::{
    cards_revealed_effect, commit_face_down_effect, deal_private_card_effect,
    hand_count_changed_effect, own_commit_confirmed_effect, private_diagnostic_effect,
    public_diagnostic_effect, refill_started_effect, round_scored_effect, terminal_effect, CardId,
    HighCardDuelEffect, HighCardDuelSeat, Score, Sigil,
};

fn seat_id(index: u8) -> SeatId {
    SeatId(format!("seat-{index}"))
}

fn card(rank: u8, sigil: Sigil) -> CardId {
    CardId::new(rank, sigil).expect("test card is valid")
}

#[test]
fn effect_visibility_scopes_match_spec() {
    let private_owner = seat_id(0);
    let private_card = card(7, Sigil::A);
    let public_card = card(8, Sigil::B);
    let score = Score {
        seat_0: 1,
        seat_1: 0,
    };

    let effects = vec![
        (
            "hcd_deal_private_card",
            deal_private_card_effect(HighCardDuelSeat::Seat0, private_owner.clone(), private_card),
            VisibilityScope::PrivateToSeat(private_owner.clone()),
        ),
        (
            "hcd_hand_count_changed",
            hand_count_changed_effect(3, 3, 18),
            VisibilityScope::Public,
        ),
        (
            "hcd_commit_face_down",
            commit_face_down_effect(HighCardDuelSeat::Seat0, 1),
            VisibilityScope::Public,
        ),
        (
            "hcd_own_commit_confirmed",
            own_commit_confirmed_effect(
                HighCardDuelSeat::Seat0,
                private_owner.clone(),
                private_card,
                1,
            ),
            VisibilityScope::PrivateToSeat(private_owner.clone()),
        ),
        (
            "hcd_cards_revealed",
            cards_revealed_effect(1, private_card, public_card),
            VisibilityScope::Public,
        ),
        (
            "hcd_round_scored",
            round_scored_effect(1, Some(HighCardDuelSeat::Seat1), score),
            VisibilityScope::Public,
        ),
        (
            "hcd_refill_started",
            refill_started_effect(2, HighCardDuelSeat::Seat1),
            VisibilityScope::Public,
        ),
        (
            "hcd_terminal",
            terminal_effect(Some(HighCardDuelSeat::Seat0), score),
            VisibilityScope::Public,
        ),
        (
            "hcd_private_diagnostic",
            private_diagnostic_effect(
                HighCardDuelSeat::Seat0,
                private_owner.clone(),
                "invalid_private_card",
                "that card is not in your hand",
            ),
            VisibilityScope::PrivateToSeat(private_owner),
        ),
        (
            "hcd_public_diagnostic",
            public_diagnostic_effect("wrong_seat", "it is not this seat's turn"),
            VisibilityScope::Public,
        ),
    ];

    for (expected_kind, effect, expected_visibility) in effects {
        assert_eq!(effect.payload.kind(), expected_kind);
        assert_eq!(effect.visibility, expected_visibility);
    }
}

#[test]
fn effect_public_effects_contain_no_private_card_identity() {
    let pre_reveal_public_effects = [
        hand_count_changed_effect(3, 3, 18),
        commit_face_down_effect(HighCardDuelSeat::Seat0, 1),
        round_scored_effect(
            1,
            None,
            Score {
                seat_0: 0,
                seat_1: 0,
            },
        ),
        refill_started_effect(2, HighCardDuelSeat::Seat1),
        terminal_effect(
            None,
            Score {
                seat_0: 3,
                seat_1: 3,
            },
        ),
        public_diagnostic_effect("invalid_private_card", "private card redacted"),
    ];

    for effect in pre_reveal_public_effects {
        assert_eq!(effect.visibility, VisibilityScope::Public);
        assert!(!effect.payload.public_payload_text().contains("hcd:r"));
    }

    let reveal = cards_revealed_effect(1, card(3, Sigil::A), card(9, Sigil::B));
    assert_eq!(reveal.visibility, VisibilityScope::Public);
    assert_eq!(reveal.payload.kind(), "hcd_cards_revealed");
    assert!(reveal.payload.public_payload_text().contains("hcd:r03:a"));
    assert!(reveal.payload.public_payload_text().contains("hcd:r09:b"));
}

#[test]
fn effect_private_card_identity_is_private_to_owner() {
    let owner = seat_id(1);
    let effect = own_commit_confirmed_effect(
        HighCardDuelSeat::Seat1,
        owner.clone(),
        card(12, Sigil::B),
        1,
    );

    assert_eq!(effect.visibility, VisibilityScope::PrivateToSeat(owner));
    let HighCardDuelEffect::OwnCommitConfirmed { card, .. } = effect.payload else {
        panic!("expected own commit confirmation");
    };
    assert_eq!(card.stable_id(), "hcd:r12:b");
}
