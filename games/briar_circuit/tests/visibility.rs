use briar_circuit::{
    apply_pass_action, project_pass_view, setup_match, BriarCircuitEffect, BriarCircuitSeat, Card,
    CurrentTrick, PassAction, PassDirection, Phase, PlayingTrickState, Rank, SetupOptions, Suit,
    TrickPlay,
};
use engine_core::{SeatId, Seed, Viewer};

fn viewer(seat: Option<BriarCircuitSeat>) -> Viewer {
    Viewer {
        seat_id: seat.map(|seat| SeatId(seat.as_str().to_owned())),
    }
}

#[test]
fn pass_view_shows_only_viewers_own_selection() {
    let mut state = setup_match(
        Seed(1608),
        &briar_circuit::canonical_seat_ids(),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    let seat = BriarCircuitSeat::Seat0;
    let card = state.hand_for_internal(seat)[0];

    apply_pass_action(&mut state, seat, PassAction::Select(card)).expect("select succeeds");

    let owner = project_pass_view(&state, &viewer(Some(seat))).expect("pass view");
    assert_eq!(owner.direction, "left");
    assert_eq!(owner.committed_count, 0);
    assert_eq!(owner.pending_count, 4);
    assert_eq!(owner.own_selection, vec![card]);
    assert!(!owner.own_committed);

    let other =
        project_pass_view(&state, &viewer(Some(BriarCircuitSeat::Seat1))).expect("other pass view");
    assert!(other.own_selection.is_empty());
    assert!(!other.own_committed);

    let observer = project_pass_view(&state, &viewer(None)).expect("observer pass view");
    assert!(observer.own_selection.is_empty());
    assert!(!observer.own_committed);
}

#[test]
fn pass_commitment_public_effect_carries_only_counts_and_direction() {
    let mut state = setup_match(
        Seed(1609),
        &briar_circuit::canonical_seat_ids(),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    let seat = BriarCircuitSeat::Seat0;
    let cards = state.hand_for_internal(seat)[..3].to_vec();

    for card in cards {
        apply_pass_action(&mut state, seat, PassAction::Select(card)).expect("select succeeds");
    }
    let result =
        apply_pass_action(&mut state, seat, PassAction::Confirm).expect("confirm succeeds");

    assert!(result.effects.iter().any(|effect| {
        matches!(
            effect,
            BriarCircuitEffect::PassCommitmentPublic(status)
                if status.direction == PassDirection::Left
                    && status.committed_count == 1
                    && status.pending_count == 3
        )
    }));

    let owner = project_pass_view(&state, &viewer(Some(seat))).expect("pass view");
    assert!(owner.own_committed);
}

#[test]
fn play_diagnostics_do_not_expose_hidden_alternatives() {
    let led = Card::new(Rank::King, Suit::Spades).id();
    let owned_spade = Card::new(Rank::Two, Suit::Spades).id();
    let illegal_club = Card::new(Rank::Ace, Suit::Clubs).id();
    let mut state = setup_match(
        Seed(1611),
        &briar_circuit::canonical_seat_ids(),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    state.private_hands = vec![
        (BriarCircuitSeat::Seat0, vec![]),
        (BriarCircuitSeat::Seat1, vec![owned_spade, illegal_club]),
        (BriarCircuitSeat::Seat2, vec![]),
        (BriarCircuitSeat::Seat3, vec![]),
    ];
    state.phase = Phase::PlayingTrick(PlayingTrickState {
        hearts_broken: true,
        trick_index: 2,
        leader: BriarCircuitSeat::Seat0,
        active_seat: BriarCircuitSeat::Seat1,
        current_trick: CurrentTrick {
            leader: BriarCircuitSeat::Seat0,
            plays: vec![TrickPlay {
                seat: BriarCircuitSeat::Seat0,
                card: led,
            }],
        },
    });

    let err = briar_circuit::validate_play_card(&state, BriarCircuitSeat::Seat1, illegal_club)
        .expect_err("must-follow violation rejects");

    assert_eq!(err.code, "BC_MUST_FOLLOW_SUIT");
    assert!(!err.message.contains(&owned_spade.as_str()));
    assert!(!err.message.contains(BriarCircuitSeat::Seat0.as_str()));
}
