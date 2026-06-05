use engine_core::{Actor, CommandEnvelope, RulesVersion, SeatId, Seed};
use race_to_n::{
    apply_action, legal_action_tree, setup_match, validate_command, CounterValue, RaceRandomBot,
    RaceSeat, SetupOptions,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn actor(seat: RaceSeat) -> Actor {
    Actor {
        seat_id: seats()[seat.index()].clone(),
    }
}

#[test]
fn random_legal_bot_choices_validate_for_many_seeds_and_states() {
    for seed in 0..64 {
        for counter in 0..21 {
            for bot_seat in [RaceSeat::Seat0, RaceSeat::Seat1] {
                let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
                state.counter = CounterValue(counter);
                state.active_seat = bot_seat;

                let bot = RaceRandomBot::new(Seed(seed));
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

                let command = CommandEnvelope {
                    actor: actor(bot_seat),
                    action_path,
                    freshness_token: state.freshness_token,
                    rules_version: RulesVersion(1),
                };
                validate_command(&state, &command).expect("bot action validates normally");
            }
        }
    }
}

#[test]
fn fixed_seed_and_view_choose_identical_action() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.counter = CounterValue(17);
    state.active_seat = RaceSeat::Seat1;
    let bot = RaceRandomBot::new(Seed(123));

    let left = bot
        .select_action(&state, RaceSeat::Seat1)
        .expect("action selected");
    let right = bot
        .select_action(&state, RaceSeat::Seat1)
        .expect("action selected");

    assert_eq!(left, right);
}

#[test]
fn bot_selection_does_not_mutate_state() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let original = state.clone();

    RaceRandomBot::new(Seed(55))
        .select_action(&state, RaceSeat::Seat0)
        .expect("action selected");

    assert_eq!(state, original);
}

#[test]
fn selected_action_can_drive_apply_after_normal_validation() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let bot = RaceRandomBot::new(Seed(7));
    let action_path = bot
        .select_action(&state, RaceSeat::Seat0)
        .expect("action selected");
    let command = CommandEnvelope {
        actor: actor(RaceSeat::Seat0),
        action_path,
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    };
    let action = validate_command(&state, &command).expect("bot action validates normally");
    let effects = apply_action(&mut state, action);

    assert!(state.counter.0 > 0);
    assert!(!effects.is_empty());
}

#[test]
fn bot_reports_no_action_for_terminal_state() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.counter = CounterValue(21);
    state.winner = Some(RaceSeat::Seat0);

    let diagnostic = RaceRandomBot::new(Seed(1))
        .select_action(&state, RaceSeat::Seat0)
        .expect_err("terminal tree has no actions");

    assert_eq!(diagnostic.code, "no_legal_actions");
}
