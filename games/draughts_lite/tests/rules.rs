use draughts_lite::{
    apply_action, ids::board_dimensions, is_playable_cell, legal_action_tree, legal_moves,
    setup_match, validate_command, CellOccupancy, DraughtsLiteSeat, DraughtsLiteState, MoveKind,
    Piece, PieceId, PieceKind, SetupOptions, TerminalOutcome, Variant,
};
use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};
use game_stdlib::board_space::Coord;

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn coord(row: u8, col: u8) -> Coord {
    Coord::checked(row, col).unwrap()
}

fn piece_id(owner: DraughtsLiteSeat, ordinal: u8) -> PieceId {
    PieceId::new(owner, ordinal).unwrap()
}

fn man(owner: DraughtsLiteSeat, ordinal: u8, row: u8, col: u8) -> Piece {
    Piece {
        id: piece_id(owner, ordinal),
        owner,
        kind: PieceKind::Man,
        cell: coord(row, col),
    }
}

fn crown(owner: DraughtsLiteSeat, ordinal: u8, row: u8, col: u8) -> Piece {
    Piece {
        id: piece_id(owner, ordinal),
        owner,
        kind: PieceKind::Crown,
        cell: coord(row, col),
    }
}

fn empty_state(active_seat: DraughtsLiteSeat, mut pieces: Vec<Piece>) -> DraughtsLiteState {
    let board = board_dimensions();
    pieces.sort_by_key(|piece| piece.id);
    let mut cells = DraughtsLiteState::empty_cells();
    for piece in &pieces {
        cells[piece.cell.row_col_index(board).unwrap()] = CellOccupancy::Occupied(piece.id);
    }

    DraughtsLiteState {
        variant: Variant::draughts_lite_standard(),
        board,
        cells,
        pieces,
        active_seat,
        seats: [seats()[0].clone(), seats()[1].clone()],
        ply_count: 0,
        command_count: 0,
        terminal_outcome: None,
        freshness_token: FreshnessToken(0),
    }
}

fn command(state: &DraughtsLiteState, segments: &[&str]) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        },
        action_path: ActionPath {
            segments: segments
                .iter()
                .map(|segment| (*segment).to_owned())
                .collect(),
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn legal_paths(state: &DraughtsLiteState) -> Vec<Vec<String>> {
    let actor = Actor {
        seat_id: state.seats[state.active_seat.index()].clone(),
    };
    let tree = legal_action_tree(state, &actor);
    let mut paths = Vec::new();
    collect_paths(&tree.root, Vec::new(), &mut paths);
    paths
}

fn collect_paths(
    node: &engine_core::ActionNode,
    prefix: Vec<String>,
    paths: &mut Vec<Vec<String>>,
) {
    for choice in &node.choices {
        let mut next = prefix.clone();
        next.push(choice.segment.clone());
        if let Some(child) = &choice.next {
            collect_paths(child, next, paths);
        } else {
            paths.push(next);
        }
    }
}

#[test]
fn dl_setup_and_dark_square_play_are_public_and_deterministic() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let repeat = setup_match(Seed(99), &seats(), &SetupOptions::default()).unwrap();

    assert_eq!(state, repeat);
    assert_eq!(state.active_seat, DraughtsLiteSeat::Seat0);
    assert_eq!(state.pieces_for_seat(DraughtsLiteSeat::Seat0).count(), 12);
    assert_eq!(state.pieces_for_seat(DraughtsLiteSeat::Seat1).count(), 12);
    assert!(state
        .pieces
        .iter()
        .all(|piece| is_playable_cell(piece.cell)));
    assert_eq!(
        state.occupancy(coord(1, 1)),
        Some(CellOccupancy::Empty),
        "non-playable cells remain empty"
    );
}

#[test]
fn dl_men_kings_capture_and_mandatory_capture_rules_hold() {
    let quiet = empty_state(
        DraughtsLiteSeat::Seat0,
        vec![man(DraughtsLiteSeat::Seat0, 1, 3, 2)],
    );
    let quiet_paths = legal_paths(&quiet);
    assert_eq!(
        quiet_paths,
        vec![vec!["from/r3c2", "to/r4c1"], vec!["from/r3c2", "to/r4c3"]]
    );

    let capture = empty_state(
        DraughtsLiteSeat::Seat0,
        vec![
            man(DraughtsLiteSeat::Seat0, 1, 3, 2),
            man(DraughtsLiteSeat::Seat1, 1, 4, 3),
        ],
    );
    assert_eq!(legal_paths(&capture), vec![vec!["from/r3c2", "jump/r5c4"]]);
    assert_eq!(
        validate_command(&capture, &command(&capture, &["from/r3c2", "to/r4c1"]))
            .expect_err("quiet move rejected while capture exists")
            .code,
        "quiet_move_while_capture_available"
    );

    let king = empty_state(
        DraughtsLiteSeat::Seat0,
        vec![crown(DraughtsLiteSeat::Seat0, 1, 4, 3)],
    );
    let king_landings = legal_moves(&king)
        .into_iter()
        .flat_map(|legal_move| legal_move.steps.into_iter().map(|step| step.to.id()))
        .collect::<Vec<_>>();
    assert_eq!(king_landings, vec!["r3c2", "r3c4", "r5c4", "r5c2"]);
}

#[test]
fn dl_multi_jump_same_piece_no_double_capture_and_no_maximum_capture_rule() {
    let state = empty_state(
        DraughtsLiteSeat::Seat0,
        vec![
            man(DraughtsLiteSeat::Seat0, 1, 3, 2),
            man(DraughtsLiteSeat::Seat0, 2, 3, 6),
            man(DraughtsLiteSeat::Seat1, 1, 4, 3),
            man(DraughtsLiteSeat::Seat1, 2, 6, 5),
            man(DraughtsLiteSeat::Seat1, 3, 4, 7),
            man(DraughtsLiteSeat::Seat1, 4, 8, 7),
        ],
    );
    let paths = legal_paths(&state);

    assert!(paths.contains(&vec![
        "from/r3c2".to_owned(),
        "jump/r5c4".to_owned(),
        "jump/r7c6".to_owned()
    ]));
    assert!(paths.contains(&vec!["from/r3c6".to_owned(), "jump/r5c8".to_owned()]));
    assert_eq!(
        validate_command(&state, &command(&state, &["from/r3c2", "jump/r5c4"]))
            .expect_err("partial continuation rejected")
            .code,
        "mandatory_continuation_incomplete"
    );
    assert!(
        validate_command(&state, &command(&state, &["from/r3c6", "jump/r5c8"])).is_ok(),
        "shorter complete capture remains legal because there is no maximum-capture rule"
    );

    let legal = legal_moves(&state);
    assert!(legal
        .iter()
        .filter(|legal_move| legal_move.kind == MoveKind::Capture)
        .all(|legal_move| {
            let captured = legal_move.captured_piece_ids();
            captured.len()
                == captured
                    .iter()
                    .collect::<std::collections::BTreeSet<_>>()
                    .len()
        }));
}

#[test]
fn dl_promotion_and_promotion_during_capture_stop_are_enforced() {
    let mut quiet_promotion = empty_state(
        DraughtsLiteSeat::Seat0,
        vec![man(DraughtsLiteSeat::Seat0, 1, 7, 2)],
    );
    let action = validate_command(
        &quiet_promotion,
        &command(&quiet_promotion, &["from/r7c2", "to/r8c1"]),
    )
    .unwrap();
    apply_action(&mut quiet_promotion, action);
    assert_eq!(
        quiet_promotion
            .piece(piece_id(DraughtsLiteSeat::Seat0, 1))
            .unwrap()
            .kind,
        PieceKind::Crown
    );

    let capture_promotion_stop = empty_state(
        DraughtsLiteSeat::Seat0,
        vec![
            man(DraughtsLiteSeat::Seat0, 1, 6, 3),
            man(DraughtsLiteSeat::Seat1, 1, 7, 4),
            man(DraughtsLiteSeat::Seat1, 2, 7, 6),
        ],
    );
    assert!(
        validate_command(
            &capture_promotion_stop,
            &command(
                &capture_promotion_stop,
                &["from/r6c3", "jump/r8c5", "jump/r6c7"]
            )
        )
        .is_err(),
        "a newly promoted man cannot continue capturing as a king in the same move"
    );
}

#[test]
fn dl_terminal_wins_cover_no_pieces_and_no_legal_move() {
    let mut no_pieces = empty_state(
        DraughtsLiteSeat::Seat0,
        vec![
            man(DraughtsLiteSeat::Seat0, 1, 3, 2),
            man(DraughtsLiteSeat::Seat1, 1, 4, 3),
        ],
    );
    let action = validate_command(
        &no_pieces,
        &command(&no_pieces, &["from/r3c2", "jump/r5c4"]),
    )
    .unwrap();
    apply_action(&mut no_pieces, action);
    assert_eq!(
        no_pieces.terminal_outcome,
        Some(TerminalOutcome::Win {
            seat: DraughtsLiteSeat::Seat0
        })
    );

    let blocked = empty_state(
        DraughtsLiteSeat::Seat1,
        vec![
            man(DraughtsLiteSeat::Seat1, 1, 2, 1),
            man(DraughtsLiteSeat::Seat0, 1, 1, 2),
            man(DraughtsLiteSeat::Seat0, 2, 1, 4),
        ],
    );
    assert_eq!(
        draughts_lite::terminal_outcome_for_active_player(&blocked),
        Some(TerminalOutcome::Win {
            seat: DraughtsLiteSeat::Seat0
        })
    );
}
