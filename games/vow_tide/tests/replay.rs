use engine_core::{Seed, StableSerialize};
use game_test_support::profiles::{
    ProfileArtifact, ProfileMetadata, PublicExportV1Driver, SeatPrivateExportV1Driver,
    PROFILE_VERSION_V1, PUBLIC_EXPORT_V1, SEAT_PRIVATE_EXPORT_V1,
};
use vow_tide::{
    cards::{Card, CardId, Rank, Suit},
    ids::{canonical_seat_ids, VowTideSeat},
    replay_support::{
        export_for_viewer, import_viewer_export, observer, seat_viewer, snapshot, stable_hash,
    },
    setup::{setup_match, SetupOptions},
};

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

#[test]
fn identical_setup_reproduces_snapshot_hashes() {
    let seats = canonical_seat_ids(4);
    let first = setup_match(Seed(77), &seats, &SetupOptions::default()).expect("setup succeeds");
    let second = setup_match(Seed(77), &seats, &SetupOptions::default()).expect("setup succeeds");

    assert_eq!(snapshot(&first, &[]), snapshot(&second, &[]));
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
        .zip(canaries.into_iter())
    {
        *state.hand_for_internal_mut(seat).expect("hand exists") = vec![card];
    }
    state.trump_indicator = Card::new(Rank::Ace, Suit::Spades).id();
    state.hidden_stock = vec![Card::new(Rank::King, Suit::Clubs).id()];
}
