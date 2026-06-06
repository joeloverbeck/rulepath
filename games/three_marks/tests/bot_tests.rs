use engine_core::{Actor, CommandEnvelope, RulesVersion, SeatId, Seed};
use three_marks::{
    apply_action, legal_action_tree, setup_match, validate_command, CellId, CellOccupancy,
    SetupOptions, TerminalOutcome, ThreeMarksEffect, ThreeMarksLevel1Bot, ThreeMarksRandomBot,
    ThreeMarksSeat,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn actor(seat: ThreeMarksSeat) -> Actor {
    Actor {
        seat_id: seats()[seat.index()].clone(),
    }
}

fn command(
    state: &three_marks::ThreeMarksState,
    seat: ThreeMarksSeat,
    segment: String,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: engine_core::ActionPath {
            segments: vec![segment],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn place(state: &mut three_marks::ThreeMarksState, seat: ThreeMarksSeat, segment: &str) {
    let action = validate_command(state, &command(state, seat, segment.to_owned())).unwrap();
    apply_action(state, action);
}

fn assert_choice_validates(
    state: &three_marks::ThreeMarksState,
    seat: ThreeMarksSeat,
    segment: &str,
) {
    validate_command(state, &command(state, seat, segment.to_owned()))
        .expect("bot action validates normally");
}

#[test]
fn level0_choices_validate_for_many_seeds_and_states() {
    for seed in 0..64 {
        for occupied_count in 0..5 {
            let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
            for (seat, segment) in [
                (ThreeMarksSeat::Seat0, "place/r1c1"),
                (ThreeMarksSeat::Seat1, "place/r1c2"),
                (ThreeMarksSeat::Seat0, "place/r1c3"),
                (ThreeMarksSeat::Seat1, "place/r2c1"),
            ]
            .into_iter()
            .take(occupied_count)
            {
                place(&mut state, seat, segment);
            }
            if state.terminal_outcome.is_some() {
                continue;
            }
            let bot_seat = state.active_seat;
            let bot = ThreeMarksRandomBot::new(Seed(seed));
            let action_path = bot
                .select_action(&state, bot_seat)
                .expect("legal action selected");
            let tree = legal_action_tree(&state, &actor(bot_seat));
            let legal_paths: Vec<_> = tree
                .root
                .choices
                .iter()
                .map(|choice| choice.path())
                .collect();
            assert!(legal_paths.contains(&action_path));
            assert_choice_validates(&state, bot_seat, &action_path.segments[0]);
        }
    }
}

#[test]
fn level0_fixed_seed_is_deterministic_and_terminal_reports_no_action() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    place(&mut state, ThreeMarksSeat::Seat0, "place/r2c2");
    let bot = ThreeMarksRandomBot::new(Seed(123));

    let left = bot
        .select_action(&state, ThreeMarksSeat::Seat1)
        .expect("action selected");
    let right = bot
        .select_action(&state, ThreeMarksSeat::Seat1)
        .expect("action selected");
    assert_eq!(left, right);

    state.terminal_outcome = Some(TerminalOutcome::Draw);
    let diagnostic = bot
        .select_action(&state, ThreeMarksSeat::Seat1)
        .expect_err("terminal tree has no actions");
    assert_eq!(diagnostic.code, "no_legal_actions");
}

#[test]
fn level1_takes_immediate_win_before_blocking() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.cells[CellId::R1C1.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat0);
    state.cells[CellId::R1C2.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat0);
    state.cells[CellId::R2C1.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat1);
    state.cells[CellId::R2C2.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat1);
    state.active_seat = ThreeMarksSeat::Seat0;
    state.ply_count = 4;

    let decision = ThreeMarksLevel1Bot::new()
        .select_decision(&state, ThreeMarksSeat::Seat0)
        .unwrap();

    assert_eq!(decision.action_path.segments, vec!["place/r1c3"]);
    assert_eq!(decision.explanation, "completed a line");
    assert_choice_validates(&state, ThreeMarksSeat::Seat0, "place/r1c3");
}

#[test]
fn level1_blocks_immediate_opponent_win_when_no_own_win_exists() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.cells[CellId::R1C1.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat1);
    state.cells[CellId::R1C2.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat1);
    state.cells[CellId::R2C2.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat0);
    state.active_seat = ThreeMarksSeat::Seat0;
    state.ply_count = 3;

    let decision = ThreeMarksLevel1Bot::new()
        .select_decision(&state, ThreeMarksSeat::Seat0)
        .unwrap();

    assert_eq!(decision.action_path.segments, vec!["place/r1c3"]);
    assert_eq!(decision.explanation, "blocked a line");
    assert_choice_validates(&state, ThreeMarksSeat::Seat0, "place/r1c3");
}

#[test]
fn level1_handles_fork_subset_center_opposite_corner_corner_and_side_priorities() {
    let mut fork_state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    fork_state.cells[CellId::R1C1.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat0);
    fork_state.cells[CellId::R3C3.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat0);
    fork_state.cells[CellId::R2C2.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat1);
    fork_state.active_seat = ThreeMarksSeat::Seat0;
    fork_state.ply_count = 3;
    let fork_decision = ThreeMarksLevel1Bot::new()
        .select_decision(&fork_state, ThreeMarksSeat::Seat0)
        .unwrap();
    assert_eq!(fork_decision.action_path.segments, vec!["place/r1c3"]);
    assert_eq!(fork_decision.explanation, "created two line threats");

    let mut block_fork_state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    block_fork_state.cells[CellId::R1C1.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat1);
    block_fork_state.cells[CellId::R3C3.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat1);
    block_fork_state.cells[CellId::R2C2.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat0);
    block_fork_state.active_seat = ThreeMarksSeat::Seat0;
    block_fork_state.ply_count = 3;
    let block_fork = ThreeMarksLevel1Bot::new()
        .select_decision(&block_fork_state, ThreeMarksSeat::Seat0)
        .unwrap();
    assert_eq!(block_fork.action_path.segments, vec!["place/r1c3"]);
    assert_eq!(block_fork.explanation, "blocked a fork threat");

    let center_state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let center = ThreeMarksLevel1Bot::new()
        .select_decision(&center_state, ThreeMarksSeat::Seat0)
        .unwrap();
    assert_eq!(center.action_path.segments, vec!["place/r2c2"]);
    assert_eq!(center.explanation, "took center");

    let mut opposite_state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    opposite_state.cells[CellId::R1C1.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat1);
    opposite_state.cells[CellId::R2C2.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat0);
    opposite_state.active_seat = ThreeMarksSeat::Seat0;
    opposite_state.ply_count = 2;
    let opposite = ThreeMarksLevel1Bot::new()
        .select_decision(&opposite_state, ThreeMarksSeat::Seat0)
        .unwrap();
    assert_eq!(opposite.action_path.segments, vec!["place/r3c3"]);
    assert_eq!(opposite.explanation, "took the opposite corner");

    let mut corner_state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    corner_state.cells[CellId::R2C2.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat1);
    corner_state.active_seat = ThreeMarksSeat::Seat0;
    corner_state.ply_count = 1;
    let corner = ThreeMarksLevel1Bot::new()
        .select_decision(&corner_state, ThreeMarksSeat::Seat0)
        .unwrap();
    assert_eq!(corner.action_path.segments, vec!["place/r1c1"]);
    assert_eq!(corner.explanation, "chose first stable corner");

    let mut side_state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    side_state.cells[CellId::R1C1.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat0);
    side_state.cells[CellId::R1C3.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat1);
    side_state.cells[CellId::R2C2.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat0);
    side_state.cells[CellId::R3C1.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat1);
    side_state.cells[CellId::R3C3.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat0);
    side_state.active_seat = ThreeMarksSeat::Seat0;
    side_state.ply_count = 5;
    let side = ThreeMarksLevel1Bot::new()
        .select_decision(&side_state, ThreeMarksSeat::Seat0)
        .unwrap();
    assert_eq!(side.action_path.segments, vec!["place/r1c2"]);
    assert_eq!(side.explanation, "chose first stable side");
}

#[test]
fn level1_is_deterministic_explains_safely_and_emits_bot_choice_effect() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let bot = ThreeMarksLevel1Bot::new();
    let left = bot.select_decision(&state, ThreeMarksSeat::Seat0).unwrap();
    let right = bot.select_decision(&state, ThreeMarksSeat::Seat0).unwrap();

    assert_eq!(left, right);
    assert!(!left.explanation.is_empty());
    assert!(!left.explanation.contains("debug"));
    assert!(!left.explanation.contains("private"));
    assert!(matches!(
        left.effects[0].payload,
        ThreeMarksEffect::BotChoseAction {
            level: 1,
            cell: CellId::R2C2,
            ..
        }
    ));
    assert_choice_validates(&state, ThreeMarksSeat::Seat0, &left.action_path.segments[0]);
}
