use engine_core::{Actor, HashValue, SeatId, Seed, StableSerialize};
use game_test_support::profiles::{
    ProfileArtifact, ProfileMetadata, ProfileValidationErrorKind, PublicExportV1Driver,
    ReplayCommandV1Driver, SeatPrivateExportV1Driver, PROFILE_VERSION_V1, PUBLIC_EXPORT_V1,
    REPLAY_COMMAND_V1, SEAT_PRIVATE_EXPORT_V1, SETUP_EVIDENCE_V1,
};
use vow_tide::{
    actions::{legal_cards, ValidatedBid, ValidatedPlay, ACTION_PLAY},
    cards::{Card, CardId, Rank, Suit},
    ids::{canonical_seat_ids, VowTideSeat, ACTION_BID},
    replay_support::{
        action_tree_v1_encoding, export_for_viewer, import_viewer_export, observer, seat_viewer,
        snapshot, stable_hash,
    },
    rules::{apply_bid, apply_play},
    setup::{setup_match, SetupOptions},
};

const REPLAY_COMMAND_PROFILE_FIELDS: &[&str] = &[
    "profile_id",
    "profile_version",
    "visibility_class",
    "validator_owner",
    "game_id",
    "rules_version",
    "data_version",
    "hash_surface_version",
    "canonical_byte_authority",
    "migration_update_note",
    "not_applicable",
    "commands",
    "checkpoints",
    "expected_hashes",
];

const PUBLIC_EXPORT_PROFILE_FIELDS: &[&str] = &[
    "profile_id",
    "profile_version",
    "visibility_class",
    "validator_owner",
    "game_id",
    "rules_version",
    "data_version",
    "hash_surface_version",
    "canonical_byte_authority",
    "migration_update_note",
    "not_applicable",
    "export_steps",
    "import_round_trip",
    "hidden_absence_tokens",
];

const SEAT_PRIVATE_EXPORT_PROFILE_FIELDS: &[&str] = &[
    "profile_id",
    "profile_version",
    "visibility_class",
    "validator_owner",
    "game_id",
    "rules_version",
    "data_version",
    "hash_surface_version",
    "canonical_byte_authority",
    "migration_update_note",
    "not_applicable",
    "viewer_seat",
    "viewer_seat_version",
    "export_steps",
    "pairwise_no_leak",
];

fn replay_command_profile_artifact() -> ProfileArtifact<'static> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id: REPLAY_COMMAND_V1,
            profile_version: PROFILE_VERSION_V1,
            visibility_class: Some("internal-dev"),
            validator_owner: "replay-check",
            canonical_byte_authority: "vow_tide::golden_traces",
            migration_update_note: Some("8CR4NSEAPRITRI-033 virtual replay-command profile"),
        },
        fields: REPLAY_COMMAND_PROFILE_FIELDS,
        canonical_byte_claim: false,
    }
}

fn public_export_profile_artifact() -> ProfileArtifact<'static> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id: PUBLIC_EXPORT_V1,
            profile_version: PROFILE_VERSION_V1,
            visibility_class: Some("public"),
            validator_owner: "vow_tide::replay_support",
            canonical_byte_authority: "vow_tide::replay_support",
            migration_update_note: Some("UNI8CMECSCA-025 virtual public-export classification"),
        },
        fields: PUBLIC_EXPORT_PROFILE_FIELDS,
        canonical_byte_claim: true,
    }
}

fn seat_private_export_profile_artifact() -> ProfileArtifact<'static> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id: SEAT_PRIVATE_EXPORT_V1,
            profile_version: PROFILE_VERSION_V1,
            visibility_class: Some("seat-private"),
            validator_owner: "vow_tide::replay_support",
            canonical_byte_authority: "vow_tide::replay_support",
            migration_update_note: Some(
                "UNI8CMECSCA-025 virtual seat-private-export classification",
            ),
        },
        fields: SEAT_PRIVATE_EXPORT_PROFILE_FIELDS,
        canonical_byte_claim: true,
    }
}

fn seven_seat_canary_ids() -> Vec<CardId> {
    vec![
        Card::new(Rank::Two, Suit::Clubs).id(),
        Card::new(Rank::Three, Suit::Diamonds).id(),
        Card::new(Rank::Four, Suit::Hearts).id(),
        Card::new(Rank::Five, Suit::Spades).id(),
        Card::new(Rank::Six, Suit::Clubs).id(),
        Card::new(Rank::Seven, Suit::Diamonds).id(),
        Card::new(Rank::Eight, Suit::Hearts).id(),
    ]
}

fn actor(state: &vow_tide::state::VowTideState, seat: VowTideSeat) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

fn wrong_actor() -> Actor {
    Actor {
        seat_id: SeatId("seat_99".to_owned()),
    }
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|candidate| candidate == needle)
}

fn byte_offset(haystack: &[u8], needle: &[u8]) -> usize {
    haystack
        .windows(needle.len())
        .position(|candidate| candidate == needle)
        .expect("needle appears in v1 bytes")
}

fn apply_bid_value(state: &mut vow_tide::state::VowTideState, seat: VowTideSeat, value: u8) {
    let bid = ValidatedBid {
        actor: seat,
        value,
        hand_index: state.hand_index,
        hand_size: state.current_hand_size().expect("hand size"),
    };
    apply_bid(state, bid).expect("bid applies");
}

fn apply_play_card(state: &mut vow_tide::state::VowTideState, seat: VowTideSeat, card: CardId) {
    let play = ValidatedPlay {
        actor: seat,
        card,
        hand_index: state.hand_index,
        trick_index: state.playing_state().expect("playing").trick_index,
    };
    apply_play(state, play).expect("play applies");
}

fn enter_four_seat_play_state() -> vow_tide::state::VowTideState {
    let mut state = setup_match(
        Seed(20260621),
        &canonical_seat_ids(4),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    apply_bid_value(&mut state, VowTideSeat::Seat1, 0);
    apply_bid_value(&mut state, VowTideSeat::Seat2, 0);
    apply_bid_value(&mut state, VowTideSeat::Seat3, 0);
    apply_bid_value(&mut state, VowTideSeat::Seat0, 1);
    *state
        .hand_for_internal_mut(VowTideSeat::Seat1)
        .expect("seat 1 hand") = vec![Card::new(Rank::Two, Suit::Clubs).id()];
    *state
        .hand_for_internal_mut(VowTideSeat::Seat2)
        .expect("seat 2 hand") = vec![
        Card::new(Rank::Three, Suit::Clubs).id(),
        Card::new(Rank::Ace, Suit::Hearts).id(),
    ];
    state
}

#[test]
fn replay_command_v1_driver_validates_vow_bid_play_trace_metadata() {
    let driver = ReplayCommandV1Driver::new("replay-check");
    let profile = replay_command_profile_artifact();
    let trace_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden_traces");

    let delegated = driver
        .validate_with(&profile, |report| {
            assert_eq!(report.profile_id, REPLAY_COMMAND_V1);
            assert_eq!(report.visibility_class, "internal-dev");

            let mut state = setup_match(
                Seed(20260621),
                &canonical_seat_ids(3),
                &SetupOptions::default(),
            )
            .expect("setup succeeds");
            let setup_snapshot = snapshot(&state, &[]);
            apply_bid_value(&mut state, VowTideSeat::Seat1, 1);
            apply_bid_value(&mut state, VowTideSeat::Seat2, 0);
            apply_bid_value(&mut state, VowTideSeat::Seat0, 0);
            let play_card = state.hand_for_internal(VowTideSeat::Seat1)[0];
            apply_play_card(&mut state, VowTideSeat::Seat1, play_card);
            let play_snapshot = snapshot(&state, &[]);
            assert_ne!(setup_snapshot.state_hash, play_snapshot.state_hash);

            format!("{}:vow-tide", report.profile_id)
        })
        .expect("replay-command-v1 driver accepts Vow virtual adapter");

    assert_eq!(delegated, "replay-command-v1:vow-tide");

    for file_name in [
        "l0-bid-and-play.trace.json",
        "l1-contract-relative-bid-and-play.trace.json",
        "public-observer-no-leak-3p.trace.json",
        "seat-private-pairwise-no-leak-7p.trace.json",
    ] {
        let path = trace_dir.join(file_name);
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        assert!(
            !payload.contains("\"profile_id\""),
            "{file_name} must not be rewritten with profile metadata"
        );
        assert!(
            !payload.contains("\"canonical_byte_authority\""),
            "{file_name} must keep legacy trace bytes authoritative"
        );
        driver
            .validate_with(&profile, |_| {
                assert!(
                    payload.contains("\"trace_id\""),
                    "{file_name} remains an existing Vow golden trace"
                );
                assert!(
                    payload.contains("bid")
                        || payload.contains("play")
                        || payload.contains("\"seat_count\": 3")
                        || payload.contains("\"seat_count\": 7"),
                    "{file_name} remains in the selected bid/play 3p/7p evidence set"
                );
            })
            .expect("virtual replay-command profile validates selected Vow trace");
    }
}

#[test]
fn replay_command_v1_driver_rejects_vow_wrong_metadata() {
    let driver = ReplayCommandV1Driver::new("replay-check");
    let valid = replay_command_profile_artifact();

    let mut wrong_profile = valid.clone();
    wrong_profile.metadata.profile_id = SETUP_EVIDENCE_V1;
    assert_eq!(
        driver
            .validate(&wrong_profile)
            .expect_err("wrong profile")
            .kind,
        ProfileValidationErrorKind::WrongProfileId
    );

    let mut wrong_version = valid.clone();
    wrong_version.metadata.profile_version = "v2";
    assert_eq!(
        driver
            .validate(&wrong_version)
            .expect_err("wrong version")
            .kind,
        ProfileValidationErrorKind::WrongProfileVersion
    );

    let mut unknown_field = valid;
    unknown_field.fields = &["profile_id", "commands", "bid_policy"];
    assert_eq!(
        driver
            .validate(&unknown_field)
            .expect_err("unknown field")
            .kind,
        ProfileValidationErrorKind::UnknownField
    );
}

#[test]
fn identical_setup_reproduces_snapshot_hashes() {
    let seats = canonical_seat_ids(4);
    let first = setup_match(Seed(77), &seats, &SetupOptions::default()).expect("setup succeeds");
    let second = setup_match(Seed(77), &seats, &SetupOptions::default()).expect("setup succeeds");

    assert_eq!(snapshot(&first, &[]), snapshot(&second, &[]));
}

#[test]
fn action_tree_v1_parallel_vectors_cover_vow_bid_play_and_empty_trees() {
    let initial =
        setup_match(Seed(77), &canonical_seat_ids(4), &SetupOptions::default()).expect("setup");
    assert_action_tree_v1_case(
        "initial-bid",
        &initial,
        &actor(&initial, VowTideSeat::Seat1),
        10036829592919351680,
        3961,
        &[ACTION_BID.as_bytes(), b"Bid 0", b"Bid 1", b"Bid 10"],
        &[],
    );
    assert_action_tree_v1_case(
        "wrong-actor-empty",
        &initial,
        &actor(&initial, VowTideSeat::Seat2),
        17387353871007407771,
        64,
        &[],
        &[ACTION_BID.as_bytes(), ACTION_PLAY.as_bytes()],
    );
    assert_action_tree_v1_case(
        "unknown-actor-empty",
        &initial,
        &wrong_actor(),
        17387353871007407771,
        64,
        &[],
        &[ACTION_BID.as_bytes(), ACTION_PLAY.as_bytes()],
    );

    let mut hook = initial.clone();
    apply_bid_value(&mut hook, VowTideSeat::Seat1, 3);
    apply_bid_value(&mut hook, VowTideSeat::Seat2, 3);
    apply_bid_value(&mut hook, VowTideSeat::Seat3, 3);
    assert_action_tree_v1_case(
        "dealer-hook-excludes-one",
        &hook,
        &actor(&hook, VowTideSeat::Seat0),
        8783003300598144511,
        4077,
        &[b"hook_forbidden_bid", b"Bid 0", b"Bid 2", b"Bid 10"],
        &[],
    );

    let mut play = enter_four_seat_play_state();
    let lead = Card::new(Rank::Two, Suit::Clubs).id();
    let follow = Card::new(Rank::Three, Suit::Clubs).id();
    let off_suit = Card::new(Rank::Ace, Suit::Hearts).id();
    apply_play_card(&mut play, VowTideSeat::Seat1, lead);
    assert_eq!(legal_cards(&play, VowTideSeat::Seat2), vec![follow]);
    assert_action_tree_v1_case(
        "follow-suit-play",
        &play,
        &actor(&play, VowTideSeat::Seat2),
        17656755402989445629,
        877,
        &[
            ACTION_PLAY.as_bytes(),
            b"led_suit",
            follow.as_str().as_bytes(),
        ],
        &[off_suit.as_str().as_bytes()],
    );
}

#[test]
fn action_tree_v1_parallel_vectors_cover_three_to_seven_seat_bidding() {
    let expected = [
        (3, 10036829592919351680, 3961),
        (4, 10036829592919351680, 3961),
        (5, 10036829592919351680, 3961),
        (6, 7483680400442474840, 3299),
        (7, 6848199286201512187, 2976),
    ];
    for (seat_count, expected_hash, expected_len) in expected {
        let state = setup_match(
            Seed(20260621 + seat_count as u64),
            &canonical_seat_ids(seat_count),
            &SetupOptions::default(),
        )
        .expect("setup succeeds");
        assert_action_tree_v1_case(
            &format!("{seat_count}-seat-opening-bid"),
            &state,
            &actor(&state, VowTideSeat::Seat1),
            expected_hash,
            expected_len,
            &[ACTION_BID.as_bytes(), b"Bid 0"],
            &[],
        );
    }
}

fn assert_action_tree_v1_case(
    label: &str,
    state: &vow_tide::state::VowTideState,
    actor: &Actor,
    expected_hash: u64,
    expected_len: usize,
    required_needles: &[&[u8]],
    forbidden_needles: &[&[u8]],
) {
    let encoding = action_tree_v1_encoding(state, actor);
    let repeated = action_tree_v1_encoding(state, actor);

    assert_eq!(encoding, repeated, "{label} v1 encoding is deterministic");
    assert_eq!(
        encoding.stable_hash,
        HashValue::from_stable_bytes(&encoding.stable_bytes),
        "{label} v1 hash matches bytes"
    );
    assert_eq!(
        encoding.stable_hash,
        HashValue(expected_hash),
        "{label} v1 hash sentinel"
    );
    assert_eq!(
        encoding.stable_bytes.len(),
        expected_len,
        "{label} byte len"
    );
    assert!(
        encoding.stable_bytes.starts_with(b"RPSB"),
        "{label} v1 bytes use stable-bytes header"
    );
    for needle in required_needles {
        assert!(
            contains_bytes(&encoding.stable_bytes, needle),
            "{label} v1 bytes contain {}",
            String::from_utf8_lossy(needle)
        );
    }
    for needle in forbidden_needles {
        assert!(
            !contains_bytes(&encoding.stable_bytes, needle),
            "{label} v1 bytes omit {}",
            String::from_utf8_lossy(needle)
        );
    }
    for pair in required_needles.windows(2) {
        assert!(
            byte_offset(&encoding.stable_bytes, pair[0])
                < byte_offset(&encoding.stable_bytes, pair[1]),
            "{label} preserves required needle order"
        );
    }
}

#[test]
fn public_export_v1_driver_round_trips_observer_fixture() {
    let fixture = include_str!("golden_traces/public-replay-export-import.trace.json");
    assert!(!fixture.contains("\"profile_id\""));
    assert!(fixture.contains("\"viewer\": \"observer\""));
    let mut state = setup_match(
        Seed(20260621),
        &canonical_seat_ids(7),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    install_canaries(&mut state);
    let driver = PublicExportV1Driver::new("vow_tide::replay_support");
    let profile = public_export_profile_artifact();

    driver
        .validate_with(&profile, |_| {
            let export = export_for_viewer(&state, &[], &observer());
            let imported = import_viewer_export(&export).expect("observer import");
            assert_eq!(imported, export);
            let text = export.stable_summary();
            assert_eq!(export.viewer, "observer");
            assert!(!text.contains("king_clubs"));
            for canary in seven_seat_canary_ids() {
                assert!(
                    !text.contains(&canary.as_str()),
                    "observer export leaked {}",
                    canary.as_str()
                );
            }
        })
        .expect("public-export-v1 driver accepts Vow observer export");
}

#[test]
fn seat_private_export_v1_driver_round_trips_all_declared_viewers() {
    let fixture =
        include_str!("golden_traces/seat-private-replay-export-import-all-viewers.trace.json");
    assert!(!fixture.contains("\"profile_id\""));
    let seats = canonical_seat_ids(7);
    for seat_id in &seats {
        assert!(fixture.contains(&format!("\"{}\"", seat_id.0)));
    }

    let mut state =
        setup_match(Seed(20260621), &seats, &SetupOptions::default()).expect("setup succeeds");
    install_canaries(&mut state);
    let driver = SeatPrivateExportV1Driver::new("vow_tide::replay_support");
    let profile = seat_private_export_profile_artifact();
    let canaries = seven_seat_canary_ids();

    driver
        .validate_with(&profile, |_| {
            for (index, seat_id) in seats.iter().enumerate() {
                let export = export_for_viewer(&state, &[], &seat_viewer(&seat_id.0));
                let imported = import_viewer_export(&export).expect("seat import");
                assert_eq!(imported, export);
                assert_eq!(export.viewer, seat_id.0);
                let text = export.stable_summary();
                assert!(text.contains(&canaries[index].as_str()));
                assert!(!text.contains("king_clubs"));
                for (other_index, other_canary) in canaries.iter().enumerate() {
                    if other_index != index {
                        assert!(
                            !text.contains(&other_canary.as_str()),
                            "{} export leaked {}",
                            seat_id.0,
                            other_canary.as_str()
                        );
                    }
                }
            }
        })
        .expect("seat-private-export-v1 driver accepts all Vow seat exports");
}

#[test]
fn viewer_exports_round_trip_and_remain_viewer_scoped() {
    let mut state = setup_match(
        Seed(20260621),
        &canonical_seat_ids(4),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    install_canaries(&mut state);
    let observer_export = export_for_viewer(&state, &[], &observer());
    let seat_0_export = export_for_viewer(&state, &[], &seat_viewer("seat_0"));

    assert_eq!(
        import_viewer_export(&observer_export).expect("observer import"),
        observer_export
    );
    assert_eq!(
        import_viewer_export(&seat_0_export).expect("seat import"),
        seat_0_export
    );
    assert_ne!(observer_export.stable_bytes(), seat_0_export.stable_bytes());

    let observer_text = observer_export.stable_summary();
    assert!(!observer_text.contains("two_clubs"));
    assert!(!observer_text.contains("king_clubs"));

    let seat_0_text = seat_0_export.stable_summary();
    assert!(seat_0_text.contains("two_clubs"));
    assert!(!seat_0_text.contains("three_diamonds"));
    assert!(!seat_0_text.contains("king_clubs"));
}

#[test]
fn characterization_viewer_export_artifacts_are_pinned() {
    let public_trace = include_str!("golden_traces/public-replay-export-import.trace.json");
    let seat_private_trace =
        include_str!("golden_traces/seat-private-replay-export-import-all-viewers.trace.json");
    let state = setup_match(
        Seed(20260621),
        &canonical_seat_ids(4),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    let observer_export = export_for_viewer(&state, &[], &observer());
    let seat_0_export = export_for_viewer(&state, &[], &seat_viewer("seat_0"));

    assert_eq!(stable_hash(public_trace.as_bytes()), 9606057229737834804);
    assert_eq!(
        stable_hash(seat_private_trace.as_bytes()),
        16909558442784598481
    );
    assert_eq!(observer_export.stable_hash().0, 14136592432406028852);
    assert_eq!(seat_0_export.stable_hash().0, 12688236753872554050);
    assert_eq!(observer_export.viewer, "observer");
    assert_eq!(seat_0_export.viewer, "seat_0");
}

#[test]
fn stable_hash_is_byte_order_sensitive_and_repeatable() {
    assert_eq!(stable_hash(b"vow_tide"), stable_hash(b"vow_tide"));
    assert_ne!(stable_hash(b"vow_tide"), stable_hash(b"tide_vow"));
}

fn install_canaries(state: &mut vow_tide::state::VowTideState) {
    let canaries = seven_seat_canary_ids();
    for (seat, card) in VowTideSeat::ALL
        .into_iter()
        .take(state.seat_count())
        .zip(canaries)
    {
        *state.hand_for_internal_mut(seat).expect("hand exists") = vec![card];
    }
    state.trump_indicator = Card::new(Rank::Ace, Suit::Spades).id();
    state.hidden_stock = vec![Card::new(Rank::King, Suit::Clubs).id()];
}
