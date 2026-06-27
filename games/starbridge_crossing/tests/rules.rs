use std::collections::BTreeSet;

use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};
use starbridge_crossing::{
    apply_step_command, encode_step_path, home_spaces, legal_action_tree, legal_step_moves,
    setup_match, validate_step_command, SetupOptions, StarPoint, StarSpaceId, StarZone,
    STANDARD_PEGS_PER_SEAT,
};

fn seats(count: usize) -> Vec<SeatId> {
    (0..count)
        .map(|index| SeatId::from_zero_based_index(index as u32))
        .collect()
}

fn actor(seat: &SeatId) -> Actor {
    Actor {
        seat_id: seat.clone(),
    }
}

fn command(
    actor: Actor,
    segments: Vec<String>,
    freshness_token: FreshnessToken,
) -> CommandEnvelope {
    CommandEnvelope {
        actor,
        action_path: ActionPath { segments },
        freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn setup_supports_exact_official_seat_counts() {
    for count in [2, 3, 4, 6] {
        let state =
            setup_match(Seed(7), &seats(count), &SetupOptions::default()).expect("setup succeeds");

        assert_eq!(state.seats.len(), count);
        assert_eq!(state.active_seat_index, 0);
        assert_eq!(
            state.pegs.len(),
            count * usize::from(STANDARD_PEGS_PER_SEAT)
        );
        assert_eq!(state.finish_ranks, Vec::new());
        assert_eq!(state.terminal_status, None);
        assert_eq!(state.ply_count, 0);
        assert_eq!(state.command_count, 0);
    }
}

#[test]
fn setup_rejects_unsupported_seat_counts_with_stable_diagnostics() {
    for count in [1, 5, 7] {
        let diagnostic =
            setup_match(Seed(7), &seats(count), &SetupOptions::default()).expect_err("rejects");

        assert_eq!(diagnostic.code, "invalid_seat_count");
        assert_eq!(
            diagnostic.message,
            format!("starbridge_crossing supports exactly 2, 3, 4, or 6 seats; got {count}")
        );
    }
}

#[test]
fn setup_assigns_homes_and_targets_by_supported_seat_count() {
    let expected = [
        (2, vec![StarPoint::North, StarPoint::South]),
        (
            3,
            vec![StarPoint::North, StarPoint::SouthEast, StarPoint::SouthWest],
        ),
        (
            4,
            vec![
                StarPoint::North,
                StarPoint::NorthEast,
                StarPoint::South,
                StarPoint::SouthWest,
            ],
        ),
        (6, StarPoint::ALL.to_vec()),
    ];

    for (count, homes) in expected {
        let state = setup_match(Seed(7), &seats(count), &SetupOptions::default()).unwrap();
        let actual_homes = state.seats.iter().map(|seat| seat.home).collect::<Vec<_>>();
        let actual_targets = state
            .seats
            .iter()
            .map(|seat| seat.target)
            .collect::<Vec<_>>();

        assert_eq!(actual_homes, homes);
        assert_eq!(
            actual_targets,
            homes
                .iter()
                .copied()
                .map(StarPoint::opposite)
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn setup_places_ten_public_pegs_in_each_home() {
    let state = setup_match(Seed(7), &seats(6), &SetupOptions::default()).unwrap();
    let occupied_spaces = state
        .pegs
        .iter()
        .map(|peg| peg.space)
        .collect::<BTreeSet<_>>();

    assert_eq!(occupied_spaces.len(), state.pegs.len());
    for seat in &state.seats {
        let home = home_spaces(seat.home)
            .map(|space| space.id)
            .collect::<BTreeSet<_>>();
        let seat_pegs = state.pegs_for_seat(seat.seat_index).collect::<Vec<_>>();

        assert_eq!(home.len(), usize::from(STANDARD_PEGS_PER_SEAT));
        assert_eq!(seat_pegs.len(), usize::from(STANDARD_PEGS_PER_SEAT));
        for peg in seat_pegs {
            assert!(home.contains(&peg.space));
            assert_eq!(state.occupancy(peg.space), Some(peg.id));
        }
    }
}

#[test]
fn setup_leaves_non_home_spaces_empty() {
    let state = setup_match(Seed(7), &seats(2), &SetupOptions::default()).unwrap();

    for space in starbridge_crossing::spaces() {
        if !matches!(
            space.zone,
            StarZone::Home(StarPoint::North) | StarZone::Home(StarPoint::South)
        ) {
            assert_eq!(state.occupancy(space.id), None);
        }
    }
}

#[test]
fn step_action_tree_lists_active_seat_step_paths_in_deterministic_order() {
    let seats = seats(2);
    let state = setup_match(Seed(7), &seats, &SetupOptions::default()).unwrap();
    let tree = legal_action_tree(&state, &actor(&seats[0]));

    assert!(!tree.has_dead_branches());
    assert_eq!(tree.freshness_token, state.freshness_token);
    assert_eq!(tree.root.choices.len(), 1);
    assert_eq!(tree.root.choices[0].segment, "move");

    let moves = legal_step_moves(&state);
    assert!(!moves.is_empty());
    assert_eq!(moves[0].peg.seat_index, 0);
}

#[test]
fn accepted_step_moves_one_peg_and_emits_step_effect() {
    let seats = seats(2);
    let mut state = setup_match(Seed(7), &seats, &SetupOptions::default()).unwrap();
    let before = state.clone();
    let step = legal_step_moves(&state)[0];
    let freshness_token = state.freshness_token;

    let effects = apply_step_command(
        &mut state,
        &command(
            actor(&seats[0]),
            encode_step_path(step.peg, step.to),
            freshness_token,
        ),
    )
    .unwrap();

    assert_eq!(before.occupancy(step.from), Some(step.peg));
    assert_eq!(before.occupancy(step.to), None);
    assert_eq!(state.occupancy(step.from), None);
    assert_eq!(state.occupancy(step.to), Some(step.peg));
    assert_eq!(state.active_seat_index, 1);
    assert_eq!(state.freshness_token, before.freshness_token.next());
    assert_eq!(effects.len(), 1);
}

#[test]
fn invalid_step_diagnostics_do_not_mutate_state() {
    let seats = seats(2);
    let state = setup_match(Seed(7), &seats, &SetupOptions::default()).unwrap();
    let active_peg = state.pegs_for_seat(0).next().unwrap();
    let step = legal_step_moves(&state)[0];
    let occupied_destination = state
        .pegs_for_seat(0)
        .find(|peg| peg.id != active_peg.id)
        .unwrap()
        .space;
    let non_adjacent_empty = starbridge_crossing::spaces()
        .iter()
        .map(|space| space.id)
        .find(|space| {
            state.occupancy(*space).is_none()
                && !starbridge_crossing::StarDirection::ALL
                    .into_iter()
                    .any(|direction| {
                        starbridge_crossing::neighbor_in_direction(active_peg.space, direction)
                            == Some(*space)
                    })
        })
        .unwrap();

    for (path, code) in [
        (
            encode_step_path(active_peg.id, occupied_destination),
            "occupied_destination",
        ),
        (
            encode_step_path(active_peg.id, non_adjacent_empty),
            "non_adjacent_destination",
        ),
        (
            vec![
                "move".to_owned(),
                active_peg.id.stable_id(),
                "step".to_owned(),
                "s999".to_owned(),
            ],
            "off_board_destination",
        ),
        (encode_step_path(step.peg, step.to), "wrong_seat"),
    ] {
        let mut candidate = state.clone();
        let actor = if code == "wrong_seat" {
            actor(&seats[1])
        } else {
            actor(&seats[0])
        };
        let diagnostic =
            apply_step_command(&mut candidate, &command(actor, path, state.freshness_token))
                .expect_err("invalid step is rejected");

        assert_eq!(diagnostic.code, code);
        assert_eq!(candidate, state);
    }

    let mut stale = state.clone();
    let diagnostic = apply_step_command(
        &mut stale,
        &command(
            actor(&seats[0]),
            encode_step_path(step.peg, step.to),
            FreshnessToken(99),
        ),
    )
    .expect_err("stale command is rejected");
    assert_eq!(diagnostic.code, "stale_action");
    assert_eq!(stale, state);
}

#[test]
fn validate_step_rejects_wrong_owner_peg_without_mutation() {
    let seats = seats(2);
    let state = setup_match(Seed(7), &seats, &SetupOptions::default()).unwrap();
    let wrong_peg = state.pegs_for_seat(1).next().unwrap();
    let destination = StarSpaceId::new(60).unwrap();

    let diagnostic = validate_step_command(
        &state,
        &command(
            actor(&seats[0]),
            encode_step_path(wrong_peg.id, destination),
            state.freshness_token,
        ),
    )
    .expect_err("wrong owner peg is rejected");

    assert_eq!(diagnostic.code, "wrong_peg_seat");
}
