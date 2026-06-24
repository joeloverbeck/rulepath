use engine_core::{
    ActionPath, Actor, CommandEnvelope, HashValue, RulesVersion, SeatId, StableSerialize, Viewer,
};
use frontier_control::{
    action_tree_hash, action_tree_v1_bytes, action_tree_v1_hash, apply_command,
    export_public_replay, import_public_export, legal_action_tree, public_replay_step, setup_match,
    FactionId, FrontierControlState, Phase, SetupOptions, SiteId, ACTION_END_TURN, ACTION_MARCH,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn end_turn_command(state: &frontier_control::FrontierControlState, seat: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: SeatId(seat.to_owned()),
        },
        action_path: ActionPath {
            segments: vec![ACTION_END_TURN.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn command(
    state: &FrontierControlState,
    faction: FactionId,
    segments: Vec<&str>,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for(state, faction),
        action_path: ActionPath {
            segments: segments.into_iter().map(str::to_owned).collect(),
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn setup_and_replay_hashes_are_deterministic() {
    let first = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let second = setup_match(&seats(), &SetupOptions::default()).unwrap();
    assert_eq!(first.stable_hash(), second.stable_hash());
}

#[test]
fn command_stream_reproduces_effects_state_and_public_export() {
    let mut first = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let mut second = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let first_command = end_turn_command(&first, "seat_1");
    let second_command = end_turn_command(&second, "seat_1");
    let first_applied = apply_command(&mut first, &first_command).unwrap();
    let second_applied = apply_command(&mut second, &second_command).unwrap();

    assert_eq!(first_applied.effects, second_applied.effects);
    assert_eq!(first.stable_hash(), second.stable_hash());

    let step = public_replay_step(
        0,
        &first,
        &first_command,
        &first_applied.effects,
        &Viewer { seat_id: None },
    );
    let export = export_public_replay(first.variant.id.clone(), vec![step]);
    assert_eq!(import_public_export(&export).raw_json, export.to_json());
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
    assert!(
        missing.is_empty(),
        "populate v1 vectors:\n{}",
        missing.join("\n")
    );

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
    let opening = setup_match(&seats(), &SetupOptions::default()).unwrap();

    let mut clash_branch = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let first_march = command(
        &clash_branch,
        FactionId::Prospectors,
        vec![
            ACTION_MARCH,
            SiteId::BaseCamp.as_str(),
            SiteId::Ford.as_str(),
        ],
    );
    apply_command(&mut clash_branch, &first_march).expect("first march applies");

    let mut stake_available = setup_match(&seats(), &SetupOptions::default()).unwrap();
    stake_available
        .site_mut(SiteId::Ford)
        .expect("ford exists")
        .crews = 1;

    let mut dismantle_available = setup_match(&seats(), &SetupOptions::default()).unwrap();
    dismantle_available.active_faction = FactionId::Garrison;
    dismantle_available
        .site_mut(SiteId::Ford)
        .expect("ford exists")
        .stake = true;
    dismantle_available
        .site_mut(SiteId::Ford)
        .expect("ford exists")
        .guards = 1;

    let mut early_end = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let end_turn = command(&early_end, FactionId::Prospectors, vec![ACTION_END_TURN]);
    apply_command(&mut early_end, &end_turn).expect("end turn applies");

    let mut terminal = setup_match(&seats(), &SetupOptions::default()).unwrap();
    terminal.phase = Phase::Terminal;

    vec![
        vector(
            "opening_moves",
            &opening,
            FactionId::Prospectors,
            1291,
            HashValue(14934942909345403747),
            HashValue(16277890795749786444),
            vec![
                vec!["march/site_base_camp/site_ford".to_owned()],
                vec!["march/site_base_camp/site_timberline".to_owned()],
                vec!["end_turn".to_owned()],
            ],
        ),
        vector(
            "move_clash_branch",
            &clash_branch,
            FactionId::Prospectors,
            3310,
            HashValue(4769522588459725601),
            HashValue(8239912348712405228),
            vec![
                vec!["march/site_base_camp/site_ford".to_owned()],
                vec!["march/site_base_camp/site_timberline".to_owned()],
                vec!["march/site_ford/site_gatehouse".to_owned()],
                vec!["march/site_ford/site_base_camp".to_owned()],
                vec!["march/site_ford/site_quarry".to_owned()],
                vec!["stake/site_ford".to_owned()],
                vec!["muster".to_owned()],
                vec!["end_turn".to_owned()],
            ],
        ),
        vector(
            "stake_available",
            &stake_available,
            FactionId::Prospectors,
            2601,
            HashValue(12908324649299837008),
            HashValue(11013731039854121046),
            vec![
                vec!["march/site_base_camp/site_ford".to_owned()],
                vec!["march/site_base_camp/site_timberline".to_owned()],
                vec!["march/site_ford/site_gatehouse".to_owned()],
                vec!["march/site_ford/site_quarry".to_owned()],
                vec!["stake/site_ford".to_owned()],
                vec!["end_turn".to_owned()],
            ],
        ),
        vector(
            "dismantle_available",
            &dismantle_available,
            FactionId::Garrison,
            5890,
            HashValue(4031145394212002295),
            HashValue(26708586450493490),
            vec![
                vec!["patrol/site_gatehouse/site_signal_hill".to_owned()],
                vec!["patrol/site_gatehouse/site_ford".to_owned()],
                vec!["patrol/site_gatehouse/site_quarry".to_owned()],
                vec!["patrol/site_signal_hill/site_gatehouse".to_owned()],
                vec!["patrol/site_signal_hill/site_quarry".to_owned()],
                vec!["patrol/site_signal_hill/site_goldfield".to_owned()],
                vec!["patrol/site_ford/site_gatehouse".to_owned()],
                vec!["patrol/site_ford/site_base_camp".to_owned()],
                vec!["patrol/site_ford/site_quarry".to_owned()],
                vec!["reinforce/site_gatehouse".to_owned()],
                vec!["reinforce/site_signal_hill".to_owned()],
                vec!["dismantle/site_ford".to_owned()],
                vec!["end_turn".to_owned()],
            ],
        ),
        vector(
            "early_end_next_turn",
            &early_end,
            FactionId::Garrison,
            4092,
            HashValue(480402586032591446),
            HashValue(16861215057075239797),
            vec![
                vec!["patrol/site_gatehouse/site_signal_hill".to_owned()],
                vec!["patrol/site_gatehouse/site_ford".to_owned()],
                vec!["patrol/site_gatehouse/site_quarry".to_owned()],
                vec!["patrol/site_signal_hill/site_gatehouse".to_owned()],
                vec!["patrol/site_signal_hill/site_quarry".to_owned()],
                vec!["patrol/site_signal_hill/site_goldfield".to_owned()],
                vec!["reinforce/site_gatehouse".to_owned()],
                vec!["reinforce/site_signal_hill".to_owned()],
                vec!["end_turn".to_owned()],
            ],
        ),
        vector(
            "terminal_empty_tree",
            &terminal,
            FactionId::Prospectors,
            64,
            HashValue(17387353871007407771),
            HashValue(10022657772393329959),
            Vec::new(),
        ),
    ]
}

fn vector(
    name: &'static str,
    state: &FrontierControlState,
    faction: FactionId,
    expected_bytes_len: usize,
    expected_hash: HashValue,
    expected_local_hash: HashValue,
    expected_paths: Vec<Vec<String>>,
) -> ActionTreeV1Vector {
    let tree = legal_action_tree(state, &actor_for(state, faction));
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

fn actor_for(state: &FrontierControlState, faction: FactionId) -> Actor {
    Actor {
        seat_id: state
            .seats
            .iter()
            .find(|seat| state.faction_for_seat(seat) == Some(faction))
            .expect("seat exists")
            .clone(),
    }
}

fn action_paths(choices: &[engine_core::ActionChoice]) -> Vec<Vec<String>> {
    choices
        .iter()
        .map(|choice| vec![choice.segment.clone()])
        .collect()
}
