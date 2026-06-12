use engine_core::{
    ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, StableSerialize, Viewer,
};
use flood_watch::{
    apply_command, export_public_replay, import_public_export, public_replay_step, setup_match,
    DistrictId, EventCard, EventKind, FloodWatchState, ScenarioVariant, SetupOptions,
    ACTION_END_TURN,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn end_turn_command(state: &flood_watch::FloodWatchState) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: SeatId("seat_0".to_owned()),
        },
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

fn observer() -> Viewer {
    Viewer { seat_id: None }
}

#[test]
fn setup_state_hash_is_deterministic_for_same_seed_and_scenario() {
    let options = SetupOptions::default();

    let first = setup_match(Seed(55), &seats(), &options).unwrap();
    let second = setup_match(Seed(55), &seats(), &options).unwrap();

    assert_eq!(first.event_deck_internal(), second.event_deck_internal());
    assert_eq!(first.stable_hash(), second.stable_hash());
}

#[test]
fn terminal_outcome_replays_deterministically() {
    let deck = vec![card(
        EventKind::StormSurge {
            district: DistrictId::OldDocks,
        },
        1,
    )];
    let mut first =
        FloodWatchState::new_after_setup(ScenarioVariant::standard(), seats(), deck.clone());
    let mut second = FloodWatchState::new_after_setup(ScenarioVariant::standard(), seats(), deck);
    let first_command = end_turn_command(&first);
    let second_command = end_turn_command(&second);

    let first_applied = apply_command(&mut first, &first_command).unwrap();
    let second_applied = apply_command(&mut second, &second_command).unwrap();

    assert_eq!(first.terminal_outcome, second.terminal_outcome);
    assert_eq!(first_applied.effects, second_applied.effects);
    assert_eq!(first.stable_hash(), second.stable_hash());
}

#[test]
fn setup_state_hash_changes_with_seed_or_scenario() {
    let standard = setup_match(Seed(55), &seats(), &SetupOptions::default()).unwrap();
    let other_seed = setup_match(Seed(56), &seats(), &SetupOptions::default()).unwrap();
    let deluge = setup_match(
        Seed(55),
        &seats(),
        &SetupOptions {
            variant: ScenarioVariant::deluge(),
        },
    )
    .unwrap();

    assert_ne!(
        standard.event_deck_internal(),
        other_seed.event_deck_internal()
    );
    assert_ne!(standard.stable_hash(), other_seed.stable_hash());
    assert_ne!(standard.stable_hash(), deluge.stable_hash());
}

#[test]
fn environment_effects_and_hashes_replay_deterministically() {
    let options = SetupOptions::default();
    let mut first = setup_match(Seed(91), &seats(), &options).unwrap();
    let mut second = setup_match(Seed(91), &seats(), &options).unwrap();
    let first_command = end_turn_command(&first);
    let second_command = end_turn_command(&second);

    let first_applied = apply_command(&mut first, &first_command).unwrap();
    let second_applied = apply_command(&mut second, &second_command).unwrap();

    assert_eq!(first_applied.effects, second_applied.effects);
    assert_eq!(first.drawn, second.drawn);
    assert_eq!(first.event_deck_internal(), second.event_deck_internal());
    assert_eq!(first.stable_hash(), second.stable_hash());
}

#[test]
fn public_export_import_redacts_undrawn_deck_after_terminal() {
    let deck = vec![
        card(EventKind::Reprieve, 1),
        card(
            EventKind::StormSurge {
                district: DistrictId::Gardens,
            },
            1,
        ),
    ];
    let mut state = FloodWatchState::new_after_setup(ScenarioVariant::standard(), seats(), deck);
    let command = end_turn_command(&state);
    let applied = apply_command(&mut state, &command).unwrap();
    let step = public_replay_step(0, &state, &command, &applied.effects, &observer());
    let export = export_public_replay(state.variant.id.clone(), &observer(), vec![step]);
    let imported = import_public_export(&export);
    let rendered = imported.raw_json;

    assert!(state.terminal_outcome.is_some());
    assert!(rendered.contains("Event 1 drawn: Reprieve"));
    assert!(rendered.contains("Event 2 drawn: Storm Surge at Gardens"));
    assert!(!rendered.contains("storm_surge/district_gardens#1"));
    assert!(!rendered.contains("full_deck_order"));
    assert!(!rendered.contains("deck_order"));
}
