use briar_circuit::{
    apply_pass_action, project_pass_view, setup_match, BriarCircuitEffect, BriarCircuitSeat,
    PassAction, PassDirection, SetupOptions,
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
