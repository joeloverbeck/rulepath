use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId};
use secret_draft::{
    actions::{commit_segment, legal_action_tree, validate_command},
    apply_action, load_standard_fixture, setup_match, DraftItemId, Phase, SecretDraftSeat,
    SecretDraftState, SetupOptions, TerminalOutcome, ValidatedAction, STANDARD_ROUND_COUNT,
};

fn setup() -> SecretDraftState {
    setup_match(
        &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn actor(state: &SecretDraftState, seat: SecretDraftSeat) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

fn command(
    state: &SecretDraftState,
    seat: SecretDraftSeat,
    segments: Vec<String>,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(state, seat),
        action_path: ActionPath { segments },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn apply_item(state: &mut SecretDraftState, seat: SecretDraftSeat, item: DraftItemId) {
    let envelope = command(state, seat, vec![commit_segment(item)]);
    let validated = validate_command(state, &envelope).expect("command validates");
    apply_action(state, validated).expect("action applies");
}

#[test]
fn setup_matches_standard_fixture_and_both_seats_have_legal_choices() {
    let state = setup();
    let fixture = load_standard_fixture().expect("fixture parses");

    assert_eq!(fixture.visible_pool.to_vec(), state.visible_pool);
    assert_eq!(fixture.round_number, state.round_number);
    assert_eq!(fixture.priority_seat, state.priority_seat.as_str());
    assert_eq!(fixture.seat_0_commitment, "none");
    assert_eq!(fixture.seat_1_commitment, "none");
    assert_eq!(
        fixture.seat_0_score,
        state.score_for(SecretDraftSeat::Seat0) as u8
    );
    assert_eq!(
        fixture.seat_1_score,
        state.score_for(SecretDraftSeat::Seat1) as u8
    );

    for seat in SecretDraftSeat::ALL {
        let tree = legal_action_tree(&state, &actor(&state, seat));
        assert_eq!(tree.root.choices.len(), fixture.visible_pool.len());
        assert_eq!(tree.root.choices[0].segment, "commit/ember_1");
        assert_eq!(tree.root.choices[11].segment, "commit/grove_4");
    }
}

#[test]
fn validation_diagnostics_cover_stale_already_committed_unavailable_and_wrong_actor() {
    let mut state = setup();
    let first = command(
        &state,
        SecretDraftSeat::Seat0,
        vec![commit_segment(DraftItemId::Ember4)],
    );
    let validated = validate_command(&state, &first).expect("first command validates");
    apply_action(&mut state, validated).expect("first action applies");

    let already_committed = validate_command(
        &state,
        &command(
            &state,
            SecretDraftSeat::Seat0,
            vec![commit_segment(DraftItemId::Tide4)],
        ),
    )
    .expect_err("same seat cannot commit twice");
    assert_eq!(already_committed.code, "already_committed");

    let mut stale = command(
        &state,
        SecretDraftSeat::Seat1,
        vec![commit_segment(DraftItemId::Tide4)],
    );
    stale.freshness_token = first.freshness_token;
    assert_eq!(
        validate_command(&state, &stale)
            .expect_err("stale token rejected")
            .code,
        "stale_action"
    );

    let wrong_actor = CommandEnvelope {
        actor: Actor {
            seat_id: SeatId("not_seated".to_owned()),
        },
        action_path: ActionPath {
            segments: vec![commit_segment(DraftItemId::Tide4)],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    };
    assert_eq!(
        validate_command(&state, &wrong_actor)
            .expect_err("unknown actor rejected")
            .code,
        "wrong_seat"
    );

    apply_item(&mut state, SecretDraftSeat::Seat1, DraftItemId::Tide4);
    assert!(!state.visible_pool.contains(&DraftItemId::Ember4));
    assert_eq!(
        validate_command(
            &state,
            &command(
                &state,
                SecretDraftSeat::Seat0,
                vec![commit_segment(DraftItemId::Ember4)]
            )
        )
        .expect_err("removed item rejected")
        .code,
        "item_unavailable"
    );
}

#[test]
fn conflict_fallback_removes_contested_and_lowest_remaining_public_item() {
    let mut state = setup();

    apply_item(&mut state, SecretDraftSeat::Seat0, DraftItemId::Ember4);
    apply_item(&mut state, SecretDraftSeat::Seat1, DraftItemId::Ember4);

    assert_eq!(
        state.drafted_for(SecretDraftSeat::Seat0),
        &[DraftItemId::Ember4]
    );
    assert_eq!(
        state.drafted_for(SecretDraftSeat::Seat1),
        &[DraftItemId::Ember1]
    );
    assert!(!state.visible_pool.contains(&DraftItemId::Ember4));
    assert!(!state.visible_pool.contains(&DraftItemId::Ember1));
    assert_eq!(state.priority_conflict_wins, [1, 0]);
    assert_eq!(state.fallback_awards, [0, 1]);
    assert_eq!(state.round_number, 2);
    assert_eq!(state.priority_seat, SecretDraftSeat::Seat1);
}

#[test]
fn terminal_cap_scores_and_tie_breaks_are_deterministic() {
    let mut state = setup();
    let pairs = [
        (DraftItemId::Ember1, DraftItemId::Grove4),
        (DraftItemId::Ember2, DraftItemId::Grove3),
        (DraftItemId::Ember3, DraftItemId::Grove2),
        (DraftItemId::Ember4, DraftItemId::Grove1),
        (DraftItemId::Tide1, DraftItemId::Tide4),
        (DraftItemId::Tide2, DraftItemId::Tide3),
    ];

    for (seat_0_item, seat_1_item) in pairs {
        apply_item(&mut state, SecretDraftSeat::Seat0, seat_0_item);
        apply_item(&mut state, SecretDraftSeat::Seat1, seat_1_item);
    }

    assert_eq!(state.phase, Phase::Terminal);
    assert_eq!(state.round_number, STANDARD_ROUND_COUNT);
    assert_eq!(state.visible_pool.len(), 0);
    assert_eq!(state.drafted_for(SecretDraftSeat::Seat0).len(), 6);
    assert_eq!(state.drafted_for(SecretDraftSeat::Seat1).len(), 6);
    assert_eq!(state.scores, [16, 20]);
    assert_eq!(
        state.terminal_outcome,
        Some(TerminalOutcome::Win {
            seat: SecretDraftSeat::Seat1
        })
    );
}

#[test]
fn terminal_validation_rejects_direct_validated_action() {
    let mut state = setup();
    for item in DraftItemId::ALL.chunks(2) {
        apply_item(&mut state, SecretDraftSeat::Seat0, item[0]);
        apply_item(&mut state, SecretDraftSeat::Seat1, item[1]);
    }

    assert_eq!(state.phase, Phase::Terminal);
    assert_eq!(
        apply_action(
            &mut state,
            ValidatedAction {
                actor: SecretDraftSeat::Seat0,
                item: DraftItemId::Ember1
            }
        )
        .expect_err("terminal apply rejected")
        .code,
        "terminal_state"
    );
}
