use std::collections::BTreeSet;

use engine_core::{
    ActionPath, Actor, CommandEnvelope, DeterministicRng, RulesVersion, SeatId, Seed, SeededRng,
    Viewer,
};
use secret_draft::{
    actions::{legal_action_tree, parse_commit_segment, validate_command},
    apply_action, project_view, setup_match, DraftItemId, Phase, SecretDraftSeat, SecretDraftState,
    SetupOptions, STANDARD_ROUND_COUNT,
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
    action_path: ActionPath,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(state, seat),
        action_path,
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn deterministic_playouts_preserve_pool_award_visibility_and_terminal_invariants() {
    for seed in 0..96 {
        let mut state = setup();
        let mut rng = SeededRng::from_seed(Seed(seed));
        let mut action_count = 0;

        while state.phase != Phase::Terminal {
            for seat in SecretDraftSeat::ALL {
                if state.phase == Phase::Terminal || state.seat_committed(seat) {
                    continue;
                }

                assert_invariants(&state);
                let tree = legal_action_tree(&state, &actor(&state, seat));
                let index = rng
                    .next_index(tree.root.choices.len())
                    .expect("non-terminal uncommitted seat has legal choices");
                let path = ActionPath {
                    segments: vec![tree.root.choices[index].segment.clone()],
                };
                let envelope = command(&state, seat, path);
                let action = validate_command(&state, &envelope).expect("legal path validates");
                apply_action(&mut state, action).expect("legal path applies");
                action_count += 1;
                assert_invariants(&state);
            }
        }

        assert_eq!(action_count, usize::from(STANDARD_ROUND_COUNT) * 2);
        assert_eq!(state.phase, Phase::Terminal);
        assert_eq!(
            state.revealed_history.len(),
            usize::from(STANDARD_ROUND_COUNT)
        );
    }
}

#[test]
fn every_opening_legal_choice_validates_without_panic_or_mutation() {
    let state = setup();

    for seat in SecretDraftSeat::ALL {
        let tree = legal_action_tree(&state, &actor(&state, seat));
        for choice in &tree.root.choices {
            let before = state.clone();
            let item = parse_commit_segment(&choice.segment).expect("commit choice parses");
            assert!(DraftItemId::ALL.contains(&item));
            validate_command(
                &state,
                &command(
                    &state,
                    seat,
                    ActionPath {
                        segments: vec![choice.segment.clone()],
                    },
                ),
            )
            .expect("opening choice validates");
            assert_eq!(state, before);
        }
    }
}

fn assert_invariants(state: &SecretDraftState) {
    let drafted_count = state.drafted_for(SecretDraftSeat::Seat0).len()
        + state.drafted_for(SecretDraftSeat::Seat1).len();
    assert_eq!(
        state.visible_pool.len() + drafted_count,
        DraftItemId::ALL.len()
    );
    assert_unique_awards(state);
    assert!(state.revealed_history.len() <= usize::from(STANDARD_ROUND_COUNT));
    assert_eq!(
        state.phase == Phase::Terminal,
        state.terminal_outcome.is_some()
    );

    let observer = project_view(state, &Viewer { seat_id: None });
    assert_eq!(observer.visible_pool.len(), state.visible_pool.len());
    assert_eq!(observer.scores, state.scores);
    assert_eq!(
        observer.commitments.seat_0.committed,
        state.seat_committed(SecretDraftSeat::Seat0)
    );
    assert_eq!(
        observer.commitments.seat_1.committed,
        state.seat_committed(SecretDraftSeat::Seat1)
    );
    assert_eq!(state.stable_summary(), state.clone().stable_summary());

    if state.seat_committed(SecretDraftSeat::Seat0) != state.seat_committed(SecretDraftSeat::Seat1)
    {
        assert!(!format!("{:?}", observer.commitments).contains("commit/"));
        assert!(!format!("{:?}", observer.private_view).contains("commit/"));
    }
}

fn assert_unique_awards(state: &SecretDraftState) {
    let mut seen = BTreeSet::new();
    for item in state
        .drafted_for(SecretDraftSeat::Seat0)
        .iter()
        .chain(state.drafted_for(SecretDraftSeat::Seat1))
    {
        assert!(seen.insert(item.as_str()));
    }

    for round in &state.revealed_history {
        assert_ne!(round.seat_0_award, round.seat_1_award);
    }
}
