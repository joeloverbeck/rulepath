use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed};
use flood_watch::{
    apply_command, legal_action_tree, setup_match, DistrictId, EventCard, EventKind,
    FloodWatchEffect, FloodWatchState, Phase, ScenarioVariant, SetupOptions, ACTION_END_TURN,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn end_turn_command(state: &flood_watch::FloodWatchState) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor("seat_0"),
        action_path: ActionPath {
            segments: vec![ACTION_END_TURN.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn card(kind: EventKind, copy_index: u8) -> EventCard {
    EventCard { kind, copy_index }
}

#[test]
fn active_action_phase_tree_always_contains_end_turn() {
    for seed in 0..25 {
        let state = setup_match(Seed(seed), &seats(), &SetupOptions::default()).unwrap();
        let tree = legal_action_tree(&state, &actor("seat_0"));

        assert!(tree
            .root
            .choices
            .iter()
            .any(|choice| choice.segment == ACTION_END_TURN));
    }
}

#[test]
fn terminal_is_shared_deck_bounded_and_actionless() {
    let mut state = FloodWatchState::new_after_setup(
        ScenarioVariant::standard(),
        seats(),
        vec![card(
            EventKind::StormSurge {
                district: DistrictId::OldDocks,
            },
            1,
        )],
    );
    let starting_deck_len = state.undrawn_deck_len();
    let cmd = end_turn_command(&state);
    let applied = apply_command(&mut state, &cmd).unwrap();

    assert_eq!(state.phase, Phase::Terminal);
    assert!(state.terminal_outcome.is_some());
    assert!(state.drawn.len() <= starting_deck_len);
    assert!(applied.effects.iter().any(|effect| {
        matches!(
            &effect.payload,
            FloodWatchEffect::Terminal { outcome, .. }
                if outcome == "lost:district_old_docks"
        )
    }));
    assert!(legal_action_tree(&state, &actor("seat_0"))
        .root
        .choices
        .is_empty());
}

#[test]
fn environment_runs_once_and_only_draws_resolved_cards() {
    for seed in 0..25 {
        let mut state = setup_match(Seed(seed), &seats(), &SetupOptions::default()).unwrap();
        let starting_deck_len = state.undrawn_deck_len();
        let cmd = end_turn_command(&state);
        let applied = apply_command(&mut state, &cmd).unwrap();
        let drawn_effects = applied
            .effects
            .iter()
            .filter(|effect| matches!(effect.payload, FloodWatchEffect::EventDrawn { .. }))
            .count();

        assert_eq!(
            applied
                .effects
                .iter()
                .filter(|effect| matches!(
                    effect.payload,
                    FloodWatchEffect::EnvironmentPhaseBegan { .. }
                ))
                .count(),
            1
        );
        assert_eq!(state.drawn.len(), drawn_effects);
        assert_eq!(
            state.undrawn_deck_len() + state.drawn.len(),
            starting_deck_len
        );
        assert!(drawn_effects <= state.variant.draws_per_phase as usize);
    }
}
