use engine_core::{
    ActionPath, Actor, CommandEnvelope, HashValue, RulesVersion, SeatId, Seed, StableSerialize,
    Viewer,
};
use game_test_support::no_leak::{
    assert_pairwise_no_leak, ExposureExpectation, LeakProbe,
};
use flood_watch::{
    action_tree_hash, action_tree_v1_bytes, action_tree_v1_hash, apply_command,
    export_public_replay, import_public_export, legal_action_tree, public_replay_step, setup_match,
    DistrictId, EventCard, EventKind, FloodWatchState, Phase, ScenarioVariant, SetupOptions,
    ACTION_END_TURN, ACTION_REINFORCE,
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

fn seat_viewer(seat: &str) -> Viewer {
    Viewer {
        seat_id: Some(SeatId(seat.to_owned())),
    }
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

#[test]
fn public_exports_pairwise_omit_hidden_future_deck_cards() {
    let mut state = setup_match(Seed(91), &seats(), &SetupOptions::default()).unwrap();
    let command = end_turn_command(&state);
    let applied = apply_command(&mut state, &command).unwrap();
    let step = public_replay_step(0, &state, &command, &applied.effects, &observer());
    let probes = hidden_future_probes(&state);

    assert_pairwise_no_leak(
        [observer(), seat_viewer("seat_0"), seat_viewer("seat_1")],
        ["public_export_json"],
        probes,
        |viewer, _surface| {
            export_public_replay(state.variant.id.clone(), viewer, vec![step.clone()]).to_json()
        },
        |_source, _viewer, _surface, _canary_id| ExposureExpectation::MustBeAbsent,
        |snapshot, card| snapshot_contains_event(snapshot, card),
    )
    .expect("Flood Watch export no-leak matrix has no failures");

    let canary = ["R3", "FLOOD", "NOLEAK", "CANARY"].join("_");
    let export = export_public_replay(state.variant.id.clone(), &observer(), vec![step]).to_json();
    assert!(!export.contains(&canary), "{export}");
}

#[test]
fn action_tree_v1_parallel_vectors_cover_representative_trees() {
    let vectors = action_tree_v1_vectors();
    let missing = vectors
        .iter()
        .filter(|vector| vector.expected_hash == HashValue(0))
        .map(|vector| {
            format!(
                "{} bytes={} hash={} local_hash={} paths={:?}",
                vector.name,
                vector.bytes.len(),
                vector.hash.0,
                vector.local_hash.0,
                vector.paths
            )
        })
        .collect::<Vec<_>>();
    assert!(missing.is_empty(), "populate v1 vectors:\n{}", missing.join("\n"));

    for vector in vectors {
        assert_eq!(vector.hash, vector.expected_hash, "{} hash", vector.name);
        assert_eq!(
            vector.bytes.len(),
            vector.expected_bytes_len,
            "{} bytes length",
            vector.name
        );
        assert_eq!(
            HashValue::from_stable_bytes(&vector.bytes),
            vector.hash,
            "{} hash derives from bytes",
            vector.name
        );
        assert_eq!(
            vector.local_hash, vector.expected_local_hash,
            "{} local hash",
            vector.name
        );
        assert_eq!(
            vector.paths, vector.expected_paths,
            "{} legal paths",
            vector.name
        );
    }
}

fn hidden_future_probes(state: &FloodWatchState) -> Vec<LeakProbe<usize, String, EventCard>> {
    state
        .event_deck_internal()
        .iter()
        .enumerate()
        .filter(|(_, card)| state.forecast.as_ref() != Some(*card))
        .map(|(index, card)| LeakProbe {
            source_seat: index,
            canary_id: card.stable_id(),
            canary: card.clone(),
        })
        .collect()
}

fn snapshot_contains_event(snapshot: &str, card: &EventCard) -> bool {
    snapshot.contains(&card.stable_id()) || snapshot.contains(&format!("{card:?}"))
}

struct ActionTreeV1Vector {
    name: &'static str,
    bytes: Vec<u8>,
    hash: HashValue,
    local_hash: HashValue,
    paths: Vec<Vec<String>>,
    expected_bytes_len: usize,
    expected_hash: HashValue,
    expected_local_hash: HashValue,
    expected_paths: Vec<Vec<String>>,
}

fn action_tree_v1_vectors() -> Vec<ActionTreeV1Vector> {
    let bail_and_levee = setup_match(Seed(18), &seats(), &SetupOptions::default()).unwrap();

    let mut role_power = setup_match(Seed(18), &seats(), &SetupOptions::default()).unwrap();
    role_power.active_seat = SeatId("seat_1".to_owned());

    let early_end = state_after_commands(18, &[vec![ACTION_END_TURN.to_owned()]]);

    let budget_exhausted = state_after_commands(
        19,
        &[vec![
            ACTION_REINFORCE.to_owned(),
            DistrictId::Riverside.as_str().to_owned(),
        ]],
    );

    let terminal = terminal_state();

    vec![
        vector(
            "bail_and_place_levee",
            &bail_and_levee,
            "seat_0",
            3920,
            HashValue(2247660004428458771),
            HashValue(4425850002041434203),
            vec![
                vec!["bail/district_old_docks".to_owned()],
                vec!["bail/district_terraces".to_owned()],
                vec!["reinforce/district_riverside".to_owned()],
                vec!["reinforce/district_old_docks".to_owned()],
                vec!["reinforce/district_market".to_owned()],
                vec!["reinforce/district_terraces".to_owned()],
                vec!["reinforce/district_gardens".to_owned()],
                vec!["forecast".to_owned()],
                vec!["end_turn".to_owned()],
            ],
        ),
        vector(
            "role_power_levee_warden",
            &role_power,
            "seat_1",
            3920,
            HashValue(4532944654053335564),
            HashValue(8946559128574054524),
            vec![
                vec!["bail/district_old_docks".to_owned()],
                vec!["bail/district_terraces".to_owned()],
                vec!["reinforce/district_riverside".to_owned()],
                vec!["reinforce/district_old_docks".to_owned()],
                vec!["reinforce/district_market".to_owned()],
                vec!["reinforce/district_terraces".to_owned()],
                vec!["reinforce/district_gardens".to_owned()],
                vec!["forecast".to_owned()],
                vec!["end_turn".to_owned()],
            ],
        ),
        vector(
            "early_end_next_turn",
            &early_end,
            "seat_1",
            4375,
            HashValue(6356390137971522057),
            HashValue(13133754107875012264),
            vec![
                vec!["bail/district_old_docks".to_owned()],
                vec!["bail/district_terraces".to_owned()],
                vec!["bail/district_gardens".to_owned()],
                vec!["reinforce/district_riverside".to_owned()],
                vec!["reinforce/district_old_docks".to_owned()],
                vec!["reinforce/district_market".to_owned()],
                vec!["reinforce/district_terraces".to_owned()],
                vec!["reinforce/district_gardens".to_owned()],
                vec!["forecast".to_owned()],
                vec!["end_turn".to_owned()],
            ],
        ),
        vector(
            "budget_exhausted_auto_environment",
            &budget_exhausted,
            "seat_1",
            64,
            HashValue(828296343441045014),
            HashValue(9791162161922510910),
            Vec::new(),
        ),
        vector(
            "terminal_empty_tree",
            &terminal,
            "seat_0",
            64,
            HashValue(828296343441045014),
            HashValue(9791162161922510910),
            Vec::new(),
        ),
    ]
}

fn vector(
    name: &'static str,
    state: &FloodWatchState,
    seat: &str,
    expected_bytes_len: usize,
    expected_hash: HashValue,
    expected_local_hash: HashValue,
    expected_paths: Vec<Vec<String>>,
) -> ActionTreeV1Vector {
    let tree = legal_action_tree(state, &actor(seat));
    let bytes = action_tree_v1_bytes(&tree);
    ActionTreeV1Vector {
        name,
        hash: action_tree_v1_hash(&tree),
        local_hash: action_tree_hash(&tree),
        paths: action_paths(&tree.root.choices),
        bytes,
        expected_bytes_len,
        expected_hash,
        expected_local_hash,
        expected_paths,
    }
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn state_after_commands(seed: u64, commands: &[Vec<String>]) -> FloodWatchState {
    let mut state = setup_match(Seed(seed), &seats(), &SetupOptions::default()).unwrap();
    for segments in commands {
        let command = CommandEnvelope {
            actor: Actor {
                seat_id: state.active_seat.clone(),
            },
            action_path: ActionPath {
                segments: segments.clone(),
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        };
        apply_command(&mut state, &command).expect("test command applies");
    }
    state
}

fn terminal_state() -> FloodWatchState {
    let deck = vec![card(
        EventKind::StormSurge {
            district: DistrictId::OldDocks,
        },
        1,
    )];
    let mut state = FloodWatchState::new_after_setup(ScenarioVariant::standard(), seats(), deck);
    let command = end_turn_command(&state);
    apply_command(&mut state, &command).expect("terminal command applies");
    assert_eq!(state.phase, Phase::Terminal);
    state
}

fn action_paths(choices: &[engine_core::ActionChoice]) -> Vec<Vec<String>> {
    choices
        .iter()
        .map(|choice| vec![choice.segment.clone()])
        .collect()
}
