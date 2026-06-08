use engine_core::{SeatId, Seed, Viewer};
use token_bazaar::{
    export_public_replay, project_view, setup_match, TokenBazaarLevel1Bot, TokenBazaarSeat,
};

fn state() -> token_bazaar::TokenBazaarState {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    setup_match(Seed(1), &seats, &Default::default()).expect("setup succeeds")
}

#[test]
fn observer_and_seat_views_match_for_public_game() {
    let state = state();
    let observer = project_view(&state, &Viewer { seat_id: None });
    let seat = project_view(
        &state,
        &Viewer {
            seat_id: Some(state.seats[0].clone()),
        },
    );

    assert_eq!(observer, seat);
}

#[test]
fn public_surfaces_do_not_expose_internal_or_candidate_fields() {
    let state = state();
    let view = project_view(&state, &Viewer { seat_id: None });
    let decision = TokenBazaarLevel1Bot::new(Seed(3))
        .select_decision(&state, TokenBazaarSeat::Seat0)
        .expect("bot chooses");
    let export = export_public_replay(
        1,
        &[
            vec!["collect/amber".to_owned()],
            vec!["collect/amber".to_owned()],
        ],
    );
    let combined = format!(
        "{}\n{}\n{}",
        view.stable_summary(),
        decision.rationale,
        export.to_json()
    );

    for forbidden in ["debug", "candidate", "valuation", "internal", "omniscient"] {
        assert!(
            !combined.contains(forbidden),
            "leaked forbidden token {forbidden}"
        );
    }
    assert!(view.hidden_fields.is_empty());
}
