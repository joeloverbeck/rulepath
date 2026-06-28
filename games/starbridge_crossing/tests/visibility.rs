use engine_core::{SeatId, StableSerialize, Viewer};
use starbridge_crossing::{
    home_spaces, project_view, setup_match, FinishRank, SetupOptions, StarSpaceId,
    StarbridgePublicView, StarbridgeState, TerminalStatus,
};

fn seats(count: usize) -> Vec<SeatId> {
    (0..count)
        .map(|index| SeatId::from_zero_based_index(index as u32))
        .collect()
}

fn viewer(seat: Option<&SeatId>) -> Viewer {
    Viewer {
        seat_id: seat.cloned(),
    }
}

fn public_view(count: usize) -> (Vec<SeatId>, StarbridgePublicView) {
    let seats = seats(count);
    let state = setup_match(engine_core::Seed(7), &seats, &SetupOptions::default()).unwrap();
    let view = project_view(&state, &viewer(None));
    (seats, view)
}

fn state(count: usize) -> StarbridgeState {
    setup_match(
        engine_core::Seed(7),
        &seats(count),
        &SetupOptions::default(),
    )
    .unwrap()
}

fn set_progress(state: &mut StarbridgeState, seat_index: u8, progress: u8) {
    let target = state.seats[usize::from(seat_index)].target;
    let target_spaces = home_spaces(target)
        .map(|space| space.id)
        .collect::<Vec<StarSpaceId>>();
    state.occupancy = StarbridgeState::empty_occupancy();

    for peg in &mut state.pegs {
        if peg.owner_seat_index == seat_index && peg.id.ordinal < progress {
            peg.space = target_spaces[usize::from(peg.id.ordinal)];
        }
        state.occupancy[usize::from(peg.space.index())] = Some(peg.id);
    }
}

#[test]
fn public_observer_view_contains_all_board_facts() {
    let (_seats, view) = public_view(2);

    assert_eq!(view.game_id, "starbridge_crossing");
    assert_eq!(view.spaces.len(), 121);
    assert_eq!(view.seats.len(), 2);
    assert_eq!(view.active_seat.as_deref(), Some("seat_0"));
    assert_eq!(view.finish_ranks, Vec::new());
    assert_eq!(view.terminal, None);
    assert_eq!(view.terminal_rationale, None);
    assert_eq!(
        view.spaces
            .iter()
            .filter(|space| space.occupant.is_some())
            .count(),
        20
    );
    assert!(view
        .spaces
        .iter()
        .all(|space| !space.ui.zone_label.is_empty()));
}

#[test]
fn every_supported_seat_view_matches_public_observer_board_facts() {
    for count in [2, 3, 4, 6] {
        let seats = seats(count);
        let state = setup_match(engine_core::Seed(7), &seats, &SetupOptions::default()).unwrap();
        let observer = project_view(&state, &viewer(None));

        for seat in &seats {
            let seat_view = project_view(&state, &viewer(Some(seat)));

            assert_eq!(seat_view, observer);
            assert_eq!(seat_view.stable_summary(), observer.stable_summary());
        }
    }
}

#[test]
fn no_private_visibility_class_exists_for_starbridge() {
    let (_seats, view) = public_view(6);
    let summary = view.stable_summary();

    assert_eq!(view.audit.redaction_class, "none");
    assert!(view.audit.private_fields.is_empty());
    assert!(!summary.contains("private"));
    assert!(!summary.contains("hidden"));
    assert!(!summary.contains("redacted"));
}

#[test]
fn complete_terminal_view_projects_finish_order_rationale() {
    let mut state = state(2);
    let pegs_per_seat = state.variant.pegs_per_seat;
    set_progress(&mut state, 0, pegs_per_seat);
    state.finish_ranks = vec![
        FinishRank {
            seat_index: 0,
            rank: 1,
        },
        FinishRank {
            seat_index: 1,
            rank: 2,
        },
    ];
    state.terminal_status = Some(TerminalStatus::Complete);

    let view = project_view(&state, &viewer(None));
    let rationale = view
        .terminal_rationale
        .expect("complete terminal view carries rationale");

    assert_eq!(rationale.result_kind, "finish_order");
    assert_eq!(rationale.decisive_cause, "finish_order_complete");
    assert_eq!(
        rationale.template_key,
        "starbridge_crossing.finish_order_complete"
    );
    assert_eq!(
        rationale.decisive_rule_ids,
        vec![
            "SC-FINISH-001",
            "SC-FINISH-002",
            "SC-FINISH-003",
            "SC-FINISH-004",
            "SC-END-001",
        ]
    );
    assert_eq!(
        rationale
            .final_standing
            .iter()
            .map(|standing| (
                standing.seat_index,
                standing.finish_rank,
                standing.winner,
                standing.finished,
                standing.progress
            ))
            .collect::<Vec<_>>(),
        vec![
            (0, Some(1), true, true, None),
            (1, Some(2), false, false, None)
        ]
    );
    assert_eq!(
        rationale.final_standing[0].seat,
        SeatId::from_zero_based_index(0)
    );
    assert_eq!(
        rationale.final_standing[1].seat,
        SeatId::from_zero_based_index(1)
    );
}

#[test]
fn turn_limit_terminal_view_projects_progress_vector_rationale() {
    let mut state = state(3);
    set_progress(&mut state, 0, 3);
    set_progress(&mut state, 1, 1);
    state.finish_ranks = vec![
        FinishRank {
            seat_index: 0,
            rank: 1,
        },
        FinishRank {
            seat_index: 1,
            rank: 2,
        },
        FinishRank {
            seat_index: 2,
            rank: 3,
        },
    ];
    state.terminal_status = Some(TerminalStatus::TurnLimit { max_plies: 2000 });

    let view = project_view(&state, &viewer(None));
    let rationale = view
        .terminal_rationale
        .expect("turn-limit terminal view carries rationale");

    assert_eq!(rationale.result_kind, "turn_limit");
    assert_eq!(rationale.decisive_cause, "turn_limit_progress_vector");
    assert_eq!(
        rationale.template_key,
        "starbridge_crossing.turn_limit_progress_vector"
    );
    assert_eq!(
        rationale.decisive_rule_ids,
        vec!["SC-FINISH-005", "SC-FINISH-006", "SC-END-002"]
    );
    assert_eq!(
        rationale
            .final_standing
            .iter()
            .map(|standing| (
                standing.seat_index,
                standing.finish_rank,
                standing.winner,
                standing.finished,
                standing.progress
            ))
            .collect::<Vec<_>>(),
        vec![
            (0, Some(1), true, false, Some(3)),
            (1, Some(2), false, false, Some(1)),
            (2, Some(3), false, false, Some(0)),
        ]
    );
}

#[test]
fn terminal_rationale_is_excluded_from_stable_bytes() {
    let mut state = state(2);
    state.finish_ranks = vec![
        FinishRank {
            seat_index: 0,
            rank: 1,
        },
        FinishRank {
            seat_index: 1,
            rank: 2,
        },
    ];
    state.terminal_status = Some(TerminalStatus::Complete);
    let view = project_view(&state, &viewer(None));

    assert!(view.terminal_rationale.is_some());
    let summary = view.stable_summary();
    assert_eq!(summary, String::from_utf8(view.stable_bytes()).unwrap());
    assert!(!summary.contains("terminal_rationale"));
    assert!(!summary.contains("finish_order_complete"));
    assert!(!summary.contains("SC-FINISH"));
}
