use draughts_lite::{
    apply_action, legal_action_tree, setup_match, validate_command, DraughtsLiteEffect,
    DraughtsLiteLevel1Bot, DraughtsLiteRandomBot, DraughtsLiteSeat, SetupOptions, TerminalOutcome,
};
use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn command(state: &draughts_lite::DraughtsLiteState, path: ActionPath) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        },
        action_path: path,
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn legal_paths(state: &draughts_lite::DraughtsLiteState) -> Vec<ActionPath> {
    let tree = legal_action_tree(
        state,
        &Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        },
    );
    let mut paths = Vec::new();
    collect_paths(&tree.root, Vec::new(), &mut paths);
    paths
}

fn collect_paths(node: &engine_core::ActionNode, prefix: Vec<String>, paths: &mut Vec<ActionPath>) {
    for choice in &node.choices {
        let mut next = prefix.clone();
        next.push(choice.segment.clone());
        if let Some(child) = &choice.next {
            collect_paths(child, next, paths);
        } else {
            paths.push(ActionPath { segments: next });
        }
    }
}

fn apply_path(state: &mut draughts_lite::DraughtsLiteState, segments: &[&str]) {
    let path = ActionPath {
        segments: segments
            .iter()
            .map(|segment| (*segment).to_owned())
            .collect(),
    };
    let action = validate_command(state, &command(state, path)).unwrap();
    apply_action(state, action);
}

#[test]
fn level0_and_level1_choices_are_complete_legal_paths_across_seeds() {
    for seed in 0..32 {
        let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
        apply_path(&mut state, &["from/r3c2", "to/r4c1"]);
        apply_path(&mut state, &["from/r6c1", "to/r5c2"]);
        let bot_seat = state.active_seat;

        for action_path in [
            DraughtsLiteRandomBot::new(Seed(seed))
                .select_action(&state, bot_seat)
                .unwrap(),
            DraughtsLiteLevel1Bot::new(Seed(seed))
                .select_action(&state, bot_seat)
                .unwrap(),
        ] {
            assert!(legal_paths(&state).contains(&action_path));
            validate_command(&state, &command(&state, action_path)).expect("bot action validates");
        }
    }
}

#[test]
fn level0_and_level1_emit_complete_multi_segment_paths() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.cells = draughts_lite::DraughtsLiteState::empty_cells();
    state.pieces = vec![
        draughts_lite::Piece {
            id: draughts_lite::PieceId::new(DraughtsLiteSeat::Seat0, 1).unwrap(),
            owner: DraughtsLiteSeat::Seat0,
            kind: draughts_lite::PieceKind::Man,
            cell: game_stdlib::board_space::Coord::checked(3, 2).unwrap(),
        },
        draughts_lite::Piece {
            id: draughts_lite::PieceId::new(DraughtsLiteSeat::Seat1, 1).unwrap(),
            owner: DraughtsLiteSeat::Seat1,
            kind: draughts_lite::PieceKind::Man,
            cell: game_stdlib::board_space::Coord::checked(4, 3).unwrap(),
        },
        draughts_lite::Piece {
            id: draughts_lite::PieceId::new(DraughtsLiteSeat::Seat1, 2).unwrap(),
            owner: DraughtsLiteSeat::Seat1,
            kind: draughts_lite::PieceKind::Man,
            cell: game_stdlib::board_space::Coord::checked(6, 5).unwrap(),
        },
        draughts_lite::Piece {
            id: draughts_lite::PieceId::new(DraughtsLiteSeat::Seat1, 3).unwrap(),
            owner: DraughtsLiteSeat::Seat1,
            kind: draughts_lite::PieceKind::Man,
            cell: game_stdlib::board_space::Coord::checked(8, 7).unwrap(),
        },
    ];
    for piece in state.pieces.clone() {
        state.set_occupancy(piece.cell, draughts_lite::CellOccupancy::Occupied(piece.id));
    }
    state.active_seat = DraughtsLiteSeat::Seat0;

    let expected = vec![
        "from/r3c2".to_owned(),
        "jump/r5c4".to_owned(),
        "jump/r7c6".to_owned(),
    ];
    assert_eq!(
        DraughtsLiteRandomBot::new(Seed(1))
            .select_action(&state, DraughtsLiteSeat::Seat0)
            .unwrap()
            .segments,
        expected
    );
    assert_eq!(
        DraughtsLiteLevel1Bot::new(Seed(1))
            .select_action(&state, DraughtsLiteSeat::Seat0)
            .unwrap()
            .segments,
        expected
    );
}

#[test]
fn level1_determinism_and_public_explanation_hold() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let bot = DraughtsLiteLevel1Bot::new(Seed(7));

    let left = bot
        .select_decision(&state, DraughtsLiteSeat::Seat0)
        .unwrap();
    let right = bot
        .select_decision(&state, DraughtsLiteSeat::Seat0)
        .unwrap();

    assert_eq!(left, right);
    assert!(!left.rationale.is_empty());
    assert!(!left.rationale.contains("score"));
    assert!(!left.rationale.contains("candidate"));
    assert!(!left.rationale.contains("debug"));
    assert!(!left.rationale.contains("hash"));
    assert!(!left.rationale.contains("search"));
    assert!(matches!(
        left.effects[0].payload,
        DraughtsLiteEffect::BotChoseAction { level: 1, .. }
    ));
}

#[test]
fn terminal_state_yields_no_bot_action() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.terminal_outcome = Some(TerminalOutcome::Win {
        seat: DraughtsLiteSeat::Seat0,
    });

    assert_eq!(
        DraughtsLiteRandomBot::new(Seed(1))
            .select_action(&state, DraughtsLiteSeat::Seat0)
            .expect_err("terminal random bot has no action")
            .code,
        "no_legal_actions"
    );
    assert_eq!(
        DraughtsLiteLevel1Bot::new(Seed(1))
            .select_action(&state, DraughtsLiteSeat::Seat0)
            .expect_err("terminal level1 bot has no action")
            .code,
        "no_legal_actions"
    );
}
