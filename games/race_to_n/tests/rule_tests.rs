use engine_core::{
    ActionPath, Actor, CommandEnvelope, FreshnessToken, Game, RulesVersion, SeatId, Seed,
};
use race_to_n::{
    apply_action, legal_action_tree, setup_match, validate_command, CounterValue, RaceEffect,
    RaceSeat, RaceToN, SetupOptions, TerminalAdvance, ValidatedAction,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn actor(index: usize) -> Actor {
    Actor {
        seat_id: seats()[index].clone(),
    }
}

fn command(index: usize, segment: &str, freshness_token: FreshnessToken) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(index),
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn r_action_001_legal_actions_are_flat_and_target_bounded() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();

    let tree = legal_action_tree(&state, &actor(0));
    let segments: Vec<_> = tree
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.as_str())
        .collect();
    assert_eq!(segments, vec!["add-1", "add-2", "add-3"]);

    state.counter = CounterValue(19);
    let tree = legal_action_tree(&state, &actor(0));
    let segments: Vec<_> = tree
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.as_str())
        .collect();
    assert_eq!(segments, vec!["add-1", "add-2"]);

    state.counter = CounterValue(20);
    let tree = legal_action_tree(&state, &actor(0));
    let segments: Vec<_> = tree
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.as_str())
        .collect();
    assert_eq!(segments, vec!["add-1"]);
}

#[test]
fn r_restrict_001_validation_is_fail_closed_for_invalid_stale_and_wrong_actor() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();

    let stale = validate_command(&state, &command(0, "add-1", FreshnessToken(99)))
        .expect_err("stale token rejected");
    assert_eq!(stale.code, "stale_action");

    let wrong_actor = validate_command(&state, &command(1, "add-1", state.freshness_token))
        .expect_err("wrong actor rejected");
    assert_eq!(wrong_actor.code, "wrong_actor");

    let invalid = validate_command(&state, &command(0, "add-4", state.freshness_token))
        .expect_err("invalid action rejected");
    assert_eq!(invalid.code, "invalid_action");
}

#[test]
fn r_turn_001_r_turn_002_valid_action_advances_turn_and_token() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let action = validate_command(&state, &command(0, "add-2", state.freshness_token)).unwrap();
    let effects = apply_action(&mut state, action);

    assert_eq!(state.counter, CounterValue(2));
    assert_eq!(state.active_seat, RaceSeat::Seat1);
    assert_eq!(state.freshness_token, FreshnessToken(1));
    assert!(matches!(
        effects[2].payload,
        RaceEffect::TurnChanged {
            next_actor: RaceSeat::Seat1
        }
    ));
}

#[test]
fn r_end_001_r_score_001_reaching_target_ends_with_mover_as_winner() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.counter = CounterValue(20);
    let action = validate_command(&state, &command(0, "add-1", state.freshness_token)).unwrap();
    let effects = apply_action(&mut state, action);

    assert_eq!(state.counter, CounterValue(21));
    assert_eq!(state.winner, Some(RaceSeat::Seat0));
    assert_eq!(
        state.terminal_advance,
        Some(TerminalAdvance {
            counter_before: CounterValue(20),
            addition: 1,
            counter_after: CounterValue(21),
        })
    );
    assert!(matches!(
        effects[2].payload,
        RaceEffect::GameEnded {
            winner: RaceSeat::Seat0
        }
    ));

    let terminal = validate_command(&state, &command(0, "add-1", state.freshness_token))
        .expect_err("terminal match rejects further commands");
    assert_eq!(terminal.code, "match_finished");
}

#[test]
fn game_impl_uses_rules_surface() {
    let game = RaceToN;
    let mut state = game
        .setup(Seed(1), &seats(), &SetupOptions::default())
        .expect("setup succeeds");
    let tree = game.legal_action_tree(&state, &actor(0));
    let path = tree.root.choices[0].path();
    let action = game
        .validate(
            &state,
            &CommandEnvelope {
                actor: actor(0),
                action_path: path,
                freshness_token: tree.freshness_token,
                rules_version: RulesVersion(1),
            },
        )
        .expect("action validates");

    let mut rng = engine_core::SeededRng::from_seed(Seed(0));
    let effects = game.apply(&mut state, action, &mut rng);

    assert_eq!(state.counter, CounterValue(1));
    assert!(!effects.is_empty());
}

#[test]
fn rejected_command_does_not_mutate_state() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let original = state.clone();

    let diagnostic = validate_command(&state, &command(0, "add-99", state.freshness_token))
        .expect_err("invalid command rejected");

    assert_eq!(diagnostic.code, "invalid_action");
    assert_eq!(state, original);
}

#[test]
fn direct_validated_action_can_drive_terminal_effect_order() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.counter = CounterValue(18);

    let effects = apply_action(
        &mut state,
        ValidatedAction {
            actor: RaceSeat::Seat0,
            amount: 3,
        },
    );

    assert!(matches!(
        effects[0].payload,
        RaceEffect::ActionStarted { .. }
    ));
    assert!(matches!(
        effects[1].payload,
        RaceEffect::CounterAdvanced { .. }
    ));
    assert!(matches!(effects[2].payload, RaceEffect::GameEnded { .. }));
    assert!(matches!(
        effects[3].payload,
        RaceEffect::ActionCompleted { .. }
    ));
}
