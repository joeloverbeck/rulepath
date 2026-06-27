use std::collections::BTreeSet;

use engine_core::{
    ActionChoice, ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed,
};
use starbridge_crossing::{
    apply_jump_command, apply_pass_blocked_command, apply_step_command, encode_jump_path,
    encode_step_path, home_spaces, legal_action_tree, legal_jump_landings, legal_step_moves,
    setup_match, validate_jump_command, validate_step_command, SetupOptions, StarCoord, StarPegId,
    StarPoint, StarSpaceId, StarZone, StarbridgeEffect, StarbridgeState, TerminalStatus, Variant,
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

fn space_at(coord: StarCoord) -> StarSpaceId {
    starbridge_crossing::spaces()
        .iter()
        .find(|space| space.coord == coord)
        .map(|space| space.id)
        .expect("fixture coordinate exists")
}

fn set_peg(state: &mut StarbridgeState, peg: StarPegId, space: StarSpaceId) {
    let piece = state
        .pegs
        .iter_mut()
        .find(|candidate| candidate.id == peg)
        .expect("fixture peg exists");
    piece.space = space;
    state.occupancy[usize::from(space.index())] = Some(peg);
}

fn clear_board(state: &mut StarbridgeState) {
    state.occupancy = StarbridgeState::empty_occupancy();
}

fn jump_fixture() -> (
    Vec<SeatId>,
    StarbridgeState,
    StarPegId,
    StarSpaceId,
    StarSpaceId,
    StarSpaceId,
    StarSpaceId,
    StarSpaceId,
) {
    let seats = seats(2);
    let mut state = setup_match(Seed(7), &seats, &SetupOptions::default()).unwrap();
    state.occupancy = StarbridgeState::empty_occupancy();

    let peg = StarPegId::new(0, 0);
    let blocker_one = StarPegId::new(0, 1);
    let blocker_two = StarPegId::new(1, 0);
    let origin = space_at(StarCoord::new(0, 0, 0));
    let over_one = space_at(StarCoord::new(1, -1, 0));
    let landing_one = space_at(StarCoord::new(2, -2, 0));
    let over_two = space_at(StarCoord::new(2, -1, -1));
    let landing_two = space_at(StarCoord::new(2, 0, -2));

    set_peg(&mut state, peg, origin);
    set_peg(&mut state, blocker_one, over_one);
    set_peg(&mut state, blocker_two, over_two);

    (
        seats,
        state,
        peg,
        origin,
        over_one,
        landing_one,
        over_two,
        landing_two,
    )
}

fn origin_return_fixture() -> (
    Vec<SeatId>,
    StarbridgeState,
    StarPegId,
    StarSpaceId,
    StarSpaceId,
) {
    let seats = seats(2);
    let mut state = setup_match(Seed(7), &seats, &SetupOptions::default()).unwrap();
    state.occupancy = StarbridgeState::empty_occupancy();

    let peg = StarPegId::new(0, 0);
    let blocker = StarPegId::new(1, 0);
    let origin = space_at(StarCoord::new(0, 0, 0));
    let over = space_at(StarCoord::new(1, -1, 0));
    let landing = space_at(StarCoord::new(2, -2, 0));

    set_peg(&mut state, peg, origin);
    set_peg(&mut state, blocker, over);

    (seats, state, peg, origin, landing)
}

fn find_choice<'a>(choices: &'a [ActionChoice], segment: &str) -> &'a ActionChoice {
    choices
        .iter()
        .find(|choice| choice.segment == segment)
        .expect("choice segment exists")
}

fn finish_step_fixture(
    seat_count: usize,
    seat_index: u8,
) -> (Vec<SeatId>, StarbridgeState, StarPegId, StarSpaceId) {
    let seats = seats(seat_count);
    let mut state = setup_match(Seed(7), &seats, &SetupOptions::default()).unwrap();
    clear_board(&mut state);
    state.active_seat_index = seat_index;

    let target = state.seats[usize::from(seat_index)].target;
    let target_spaces = home_spaces(target)
        .map(|space| space.id)
        .collect::<Vec<_>>();
    let landing = *target_spaces.first().unwrap();
    let origin = starbridge_crossing::StarDirection::ALL
        .into_iter()
        .filter_map(|direction| starbridge_crossing::neighbor_in_direction(landing, direction))
        .find(|space| !target_spaces.contains(space))
        .expect("target edge has adjacent origin");

    for (ordinal, target_space) in target_spaces.iter().copied().skip(1).enumerate() {
        set_peg(
            &mut state,
            StarPegId::new(seat_index, u8::try_from(ordinal + 1).unwrap()),
            target_space,
        );
    }
    let peg = StarPegId::new(seat_index, 0);
    set_peg(&mut state, peg, origin);
    (seats, state, peg, landing)
}

fn blocked_fixture() -> (Vec<SeatId>, StarbridgeState) {
    let seats = seats(2);
    let mut state = setup_match(Seed(7), &seats, &SetupOptions::default()).unwrap();
    clear_board(&mut state);
    let origin = starbridge_crossing::spaces()
        .iter()
        .find(|space| {
            space
                .neighbors
                .iter()
                .filter(|neighbor| neighbor.is_some())
                .count()
                == 2
        })
        .map(|space| space.id)
        .expect("topology has degree-2 corners");
    set_peg(&mut state, StarPegId::new(0, 0), origin);

    let mut blockers = (0..10)
        .map(|ordinal| StarPegId::new(1, ordinal))
        .collect::<Vec<_>>()
        .into_iter();
    for direction in starbridge_crossing::StarDirection::ALL {
        if let Some(neighbor) = starbridge_crossing::neighbor_in_direction(origin, direction) {
            set_peg(&mut state, blockers.next().unwrap(), neighbor);
            if let Some(beyond) = starbridge_crossing::neighbor_in_direction(neighbor, direction) {
                set_peg(&mut state, blockers.next().unwrap(), beyond);
            }
        }
    }
    (seats, state)
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

#[test]
fn jump_chain_moves_peg_and_keeps_jumped_pegs() {
    let (seats, mut state, peg, origin, over_one, landing_one, _, _) = jump_fixture();
    let freshness_token = state.freshness_token;

    let jumps = legal_jump_landings(&state, peg, origin, &[]);
    assert!(jumps
        .iter()
        .any(|jump| jump.over == over_one && jump.landing == landing_one));

    let effects = apply_jump_command(
        &mut state,
        &command(
            actor(&seats[0]),
            encode_jump_path(peg, &[landing_one]),
            freshness_token,
        ),
    )
    .unwrap();

    assert_eq!(state.occupancy(origin), None);
    assert_eq!(state.occupancy(over_one), Some(StarPegId::new(0, 1)));
    assert_eq!(state.occupancy(landing_one), Some(peg));
    assert_eq!(state.active_seat_index, 1);
    assert_eq!(effects.len(), 1);
    assert!(matches!(
        &effects[0].payload,
        StarbridgeEffect::JumpChain { hops, .. } if hops.len() == 1 && hops[0].over == over_one && hops[0].to == landing_one
    ));
}

#[test]
fn multi_hop_can_change_direction_and_stop_midway() {
    let (seats, state, peg, origin, _, landing_one, over_two, landing_two) = jump_fixture();

    let single = validate_jump_command(
        &state,
        &command(
            actor(&seats[0]),
            encode_jump_path(peg, &[landing_one]),
            state.freshness_token,
        ),
    )
    .expect("stop after first landing is legal");
    assert_eq!(single.chain.hops.len(), 1);

    let chain = validate_jump_command(
        &state,
        &command(
            actor(&seats[0]),
            encode_jump_path(peg, &[landing_one, landing_two]),
            state.freshness_token,
        ),
    )
    .expect("direction-changing second hop is legal");

    assert_eq!(chain.chain.from, origin);
    assert_eq!(chain.chain.hops.len(), 2);
    assert_eq!(chain.chain.hops[1].over, over_two);
    assert_eq!(chain.chain.hops[1].landing, landing_two);
}

#[test]
fn hop_chain_cannot_return_to_origin_space() {
    let (seats, state, peg, origin, landing) = origin_return_fixture();

    let first_landings = legal_jump_landings(&state, peg, origin, &[]);
    assert!(first_landings.iter().any(|jump| jump.landing == landing));
    let return_landings = legal_jump_landings(&state, peg, landing, &[landing]);
    assert!(
        !return_landings.iter().any(|jump| jump.landing == origin),
        "origin must not be offered as a hop-chain landing"
    );

    let diagnostic = validate_jump_command(
        &state,
        &command(
            actor(&seats[0]),
            encode_jump_path(peg, &[landing, origin]),
            state.freshness_token,
        ),
    )
    .expect_err("origin-return jump chain is rejected");
    assert_eq!(diagnostic.code, "invalid_jump");

    let tree = legal_action_tree(&state, &actor(&seats[0]));
    let move_choice = find_choice(&tree.root.choices, "move");
    let peg_choice = find_choice(
        &move_choice.next.as_ref().unwrap().choices,
        &peg.stable_id(),
    );
    let jump_choice = find_choice(&peg_choice.next.as_ref().unwrap().choices, "jump");
    let first_landing_choice = find_choice(
        &jump_choice.next.as_ref().unwrap().choices,
        &landing.to_string(),
    );
    let continuation = first_landing_choice
        .next
        .as_ref()
        .unwrap()
        .choices
        .iter()
        .find(|choice| choice.segment == "continue");
    if let Some(continuation_choice) = continuation {
        assert!(
            continuation_choice
                .next
                .as_ref()
                .unwrap()
                .choices
                .iter()
                .all(|choice| choice.segment != origin.to_string()),
            "action tree must not expose the origin-return continuation"
        );
    }
}

#[test]
fn repeated_landing_and_mixed_step_jump_are_rejected_without_mutation() {
    let (seats, state, peg, _, _, landing_one, _, _) = jump_fixture();

    let repeated = validate_jump_command(
        &state,
        &command(
            actor(&seats[0]),
            encode_jump_path(peg, &[landing_one, landing_one]),
            state.freshness_token,
        ),
    )
    .expect_err("repeat landing is rejected");
    assert_eq!(repeated.code, "repeated_landing");

    let mixed = validate_jump_command(
        &state,
        &command(
            actor(&seats[0]),
            vec![
                "move".to_owned(),
                peg.stable_id(),
                "step".to_owned(),
                landing_one.to_string(),
                "jump".to_owned(),
                landing_one.to_string(),
                "stop".to_owned(),
            ],
            state.freshness_token,
        ),
    )
    .expect_err("mixed move kind is rejected");
    assert_eq!(mixed.code, "mixed_move_kind");
}

#[test]
fn finish_rank_is_assigned_after_all_pegs_reach_target_home() {
    let (seats, mut state, peg, landing) = finish_step_fixture(2, 0);
    let freshness_token = state.freshness_token;
    let effects = apply_step_command(
        &mut state,
        &command(
            actor(&seats[0]),
            encode_step_path(peg, landing),
            freshness_token,
        ),
    )
    .unwrap();

    assert_eq!(state.finish_ranks[0].seat_index, 0);
    assert_eq!(state.finish_ranks[0].rank, 1);
    assert_eq!(state.terminal_status, Some(TerminalStatus::Complete));
    assert!(matches!(
        effects.last().unwrap().payload,
        StarbridgeEffect::Terminal { .. }
    ));
}

#[test]
fn finished_seats_are_skipped_and_last_rank_is_assigned() {
    let (seats, mut state, peg, landing) = finish_step_fixture(3, 1);
    state.finish_ranks.push(starbridge_crossing::FinishRank {
        seat_index: 0,
        rank: 1,
    });
    let freshness_token = state.freshness_token;

    apply_step_command(
        &mut state,
        &command(
            actor(&seats[1]),
            encode_step_path(peg, landing),
            freshness_token,
        ),
    )
    .unwrap();

    assert_eq!(
        state
            .finish_ranks
            .iter()
            .map(|rank| (rank.seat_index, rank.rank))
            .collect::<Vec<_>>(),
        vec![(0, 1), (1, 2), (2, 3)]
    );
    assert_eq!(state.terminal_status, Some(TerminalStatus::Complete));
}

#[test]
fn blocked_pass_is_forced_when_active_seat_has_no_move() {
    let (seats, mut state) = blocked_fixture();
    let tree = legal_action_tree(&state, &actor(&seats[0]));

    assert_eq!(tree.root.choices.len(), 1);
    assert_eq!(tree.root.choices[0].segment, "pass_blocked");

    let freshness_token = state.freshness_token;
    let effects = apply_pass_blocked_command(
        &mut state,
        &command(
            actor(&seats[0]),
            vec!["pass_blocked".to_owned()],
            freshness_token,
        ),
    )
    .unwrap();

    assert_eq!(state.active_seat_index, 1);
    assert_eq!(state.ply_count, 1);
    assert!(matches!(
        effects[0].payload,
        StarbridgeEffect::PassBlocked { seat_index: 0 }
    ));
}

#[test]
fn turn_limit_assigns_deterministic_unfinished_ranks() {
    let seats = seats(3);
    let mut variant = Variant::starbridge_classic();
    variant.max_plies = 1;
    let mut state = setup_match(Seed(7), &seats, &SetupOptions { variant }).unwrap();
    let step = legal_step_moves(&state)[0];
    let freshness_token = state.freshness_token;

    apply_step_command(
        &mut state,
        &command(
            actor(&seats[0]),
            encode_step_path(step.peg, step.to),
            freshness_token,
        ),
    )
    .unwrap();

    assert_eq!(
        state.terminal_status,
        Some(TerminalStatus::TurnLimit { max_plies: 1 })
    );
    assert_eq!(
        state
            .finish_ranks
            .iter()
            .map(|rank| (rank.seat_index, rank.rank))
            .collect::<Vec<_>>(),
        vec![(0, 1), (1, 2), (2, 3)]
    );
}
