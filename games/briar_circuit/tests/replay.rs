use briar_circuit::{
    apply_pass_action, canonical_seat_ids, export_viewer_timeline, import_viewer_timeline,
    replay_bot_match, replay_hash_snapshot,
    replay_support::{action_tree_v1_encoding, replay_action_tree_v1_snapshot},
    score_completed_hand,
    setup::{deal_hand, next_dealer},
    setup_match, BriarCircuitSeat, CapturedTrick, Card, CardId, CurrentTrick, PassAction,
    PassDirection, Phase, PlayingTrickState, Rank, SetupOptions, Suit, TrickPlay,
    ViewerExportClass,
};
use engine_core::{FreshnessToken, HashValue, SeatId, Seed, SeededRng, Viewer};
use game_test_support::profiles::{
    DomainEvidenceV1Driver, ProfileArtifact, ProfileMetadata, ProfileValidationErrorKind,
    PublicExportV1Driver, ReplayCommandV1Driver, DOMAIN_EVIDENCE_V1, PROFILE_VERSION_V1,
    PUBLIC_EXPORT_V1, REPLAY_COMMAND_V1, SETUP_EVIDENCE_V1,
};

const REQUIRED_TRACES: &[&str] = &[
    "setup-four-seat-deterministic-deal.trace.json",
    "invalid-seat-count-below.trace.json",
    "invalid-seat-count-above.trace.json",
    "deal-private-no-leak.trace.json",
    "pass-left-atomic-exchange.trace.json",
    "pass-right-atomic-exchange.trace.json",
    "pass-across-atomic-exchange.trace.json",
    "hold-hand-no-pass.trace.json",
    "pass-choice-in-flight-no-leak.trace.json",
    "invalid-pass-not-three.trace.json",
    "invalid-pass-unowned-or-duplicate.trace.json",
    "two-clubs-forced-opening.trace.json",
    "follow-suit-forced.trace.json",
    "void-free-discard.trace.json",
    "first-trick-points-suppressed.trace.json",
    "first-trick-all-points-exception.trace.json",
    "hearts-not-broken-lead-diagnostic.trace.json",
    "only-hearts-lead-exception.trace.json",
    "heart-discard-breaks-hearts.trace.json",
    "queen-spades-does-not-break-hearts.trace.json",
    "off-suit-never-wins.trace.json",
    "trick-winner-leads-next.trace.json",
    "normal-hand-scoring.trace.json",
    "shoot-the-moon-fixed-addition.trace.json",
    "dealer-and-pass-cycle-rotation.trace.json",
    "threshold-unique-low-winner.trace.json",
    "threshold-low-tie-continues.trace.json",
    "invalid-wrong-seat-diagnostic.trace.json",
    "invalid-stale-diagnostic.trace.json",
    "l0-bot-action.trace.json",
    "l1-bot-pass-and-play.trace.json",
    "public-observer-no-leak.trace.json",
    "seat-private-pairwise-no-leak.trace.json",
    "public-replay-export-import.trace.json",
    "seat-private-replay-export-import.trace.json",
    "bot-vs-bot-full-match.trace.json",
    "wasm-exported-moon-terminal.trace.json",
];

const DOMAIN_EVIDENCE_PROFILE_FIELDS: &[&str] = &[
    "profile_id",
    "profile_version",
    "visibility_class",
    "validator_owner",
    "game_id",
    "rules_version",
    "data_version",
    "canonical_byte_authority",
    "migration_update_note",
    "domain_schema_version",
    "domain_input",
    "expected_domain",
];

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

fn domain_evidence_profile_artifact<'a>(
    visibility_class: &'a str,
    canonical_byte_claim: bool,
) -> ProfileArtifact<'a> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id: DOMAIN_EVIDENCE_V1,
            profile_version: PROFILE_VERSION_V1,
            visibility_class: Some(visibility_class),
            validator_owner: "briar_circuit",
            canonical_byte_authority: "none",
            migration_update_note: Some(
                "virtual domain-evidence-v1 metadata around legacy Briar Circuit fixture",
            ),
        },
        fields: DOMAIN_EVIDENCE_PROFILE_FIELDS,
        canonical_byte_claim,
    }
}

fn replay_command_profile_artifact() -> ProfileArtifact<'static> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id: REPLAY_COMMAND_V1,
            profile_version: PROFILE_VERSION_V1,
            visibility_class: Some("internal-dev"),
            validator_owner: "replay-check",
            canonical_byte_authority: "none",
            migration_update_note: Some("8CR4NSEAPRITRI-029 virtual replay-command profile"),
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
            validator_owner: "wasm-export",
            canonical_byte_authority: "none",
            migration_update_note: Some("8CR4NSEAPRITRI-031 virtual public-export profile"),
        },
        fields: PUBLIC_EXPORT_PROFILE_FIELDS,
        canonical_byte_claim: false,
    }
}

fn card(rank: Rank, suit: Suit) -> CardId {
    Card::new(rank, suit).id()
}

fn viewer(seat: Option<BriarCircuitSeat>) -> Viewer {
    Viewer {
        seat_id: seat.map(|seat| SeatId(seat.as_str().to_owned())),
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

fn commit_standard_pass(state: &mut briar_circuit::BriarCircuitState, seat: BriarCircuitSeat) {
    let cards =
        state.hand_for_internal(seat)[..briar_circuit::STANDARD_PASS_SIZE as usize].to_vec();
    for card in cards {
        apply_pass_action(state, seat, PassAction::Select(card)).expect("select pass card");
    }
    apply_pass_action(state, seat, PassAction::Confirm).expect("confirm pass");
}

fn captured_trick(winner: BriarCircuitSeat, cards: Vec<CardId>) -> CapturedTrick {
    CapturedTrick {
        hand_index: 0,
        trick_index: 0,
        winner,
        plays: cards
            .into_iter()
            .enumerate()
            .map(|(index, card)| TrickPlay {
                seat: BriarCircuitSeat::from_index(index % BriarCircuitSeat::ALL.len())
                    .expect("seat index"),
                card,
            })
            .collect(),
    }
}

#[test]
fn action_tree_v1_parallel_surface_is_deterministic_and_viewer_scoped() {
    let mut state =
        setup_match(Seed(1600), &canonical_seat_ids(), &SetupOptions::default()).expect("setup");
    assert_action_tree_v1_case(
        "observer-pass",
        &state,
        None,
        17387353871007407771,
        64,
        &[],
        &[],
    );
    let seat0_hand = state.hand_for_internal(BriarCircuitSeat::Seat0).to_vec();
    assert_action_tree_v1_case(
        "seat0-pass-select",
        &state,
        Some(BriarCircuitSeat::Seat0),
        5009386235525238101,
        4825,
        &[
            b"pass".as_slice(),
            b"select".as_slice(),
            seat0_hand[0].as_str().as_bytes(),
            seat0_hand[1].as_str().as_bytes(),
        ],
        &[b"unselect".as_slice()],
    );

    for card in seat0_hand[..briar_circuit::STANDARD_PASS_SIZE as usize]
        .iter()
        .copied()
    {
        apply_pass_action(
            &mut state,
            BriarCircuitSeat::Seat0,
            PassAction::Select(card),
        )
        .expect("select pass card");
    }
    assert_action_tree_v1_case(
        "seat0-pass-confirm",
        &state,
        Some(BriarCircuitSeat::Seat0),
        4031711539671785757,
        291,
        &[b"pass".as_slice(), b"confirm".as_slice()],
        &[b"unselect".as_slice()],
    );

    let mut play_state =
        setup_match(Seed(1600), &canonical_seat_ids(), &SetupOptions::default()).expect("setup");
    for seat in BriarCircuitSeat::ALL {
        commit_standard_pass(&mut play_state, seat);
    }
    let active = play_state
        .playing_state()
        .expect("playing phase")
        .active_seat;
    assert_action_tree_v1_case(
        "active-play",
        &play_state,
        Some(active),
        3978888525668030180,
        301,
        &[b"play".as_slice()],
        &[b"pass".as_slice(), b"unselect".as_slice()],
    );
}

#[test]
fn action_tree_v1_snapshot_covers_observer_and_all_seat_viewers_without_cross_seat_leak() {
    let state =
        setup_match(Seed(1600), &canonical_seat_ids(), &SetupOptions::default()).expect("setup");
    let snapshot = replay_action_tree_v1_snapshot(&state);

    assert_eq!(
        snapshot.public_action_tree,
        action_tree_v1_encoding(&state, &viewer(None))
    );
    assert_eq!(
        snapshot.private_action_trees.len(),
        BriarCircuitSeat::ALL.len()
    );
    for seat in BriarCircuitSeat::ALL {
        assert!(snapshot
            .private_action_trees
            .iter()
            .any(|(candidate, _)| *candidate == seat));
    }

    let seat0_hidden_card = state.hand_for_internal(BriarCircuitSeat::Seat0)[0].as_str();
    assert!(!contains_bytes(
        &snapshot.public_action_tree.stable_bytes,
        seat0_hidden_card.as_bytes()
    ));
    for (seat, encoding) in &snapshot.private_action_trees {
        if *seat != BriarCircuitSeat::Seat0 {
            assert!(
                !contains_bytes(&encoding.stable_bytes, seat0_hidden_card.as_bytes()),
                "{seat:?} v1 bytes must not include Seat0 private card {seat0_hidden_card}"
            );
        }
    }
}

fn assert_action_tree_v1_case(
    label: &str,
    state: &briar_circuit::BriarCircuitState,
    seat: Option<BriarCircuitSeat>,
    expected_hash: u64,
    expected_len: usize,
    required_needles: &[&[u8]],
    forbidden_needles: &[&[u8]],
) {
    let encoding = action_tree_v1_encoding(state, &viewer(seat));
    let repeated = action_tree_v1_encoding(state, &viewer(seat));

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
fn identical_seed_reproduces_identical_initial_deal() {
    let seats = canonical_seat_ids();
    let first = setup_match(Seed(1600), &seats, &SetupOptions::default()).expect("first setup");
    let second = setup_match(Seed(1600), &seats, &SetupOptions::default()).expect("second setup");

    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
    for seat in BriarCircuitSeat::ALL {
        assert_eq!(
            first.hand_for_internal(seat),
            second.hand_for_internal(seat)
        );
    }
}

#[test]
fn different_seed_changes_the_deal_but_not_public_rotation_facts() {
    let seats = canonical_seat_ids();
    let first = setup_match(Seed(1600), &seats, &SetupOptions::default()).expect("first setup");
    let second = setup_match(Seed(1601), &seats, &SetupOptions::default()).expect("second setup");

    assert_ne!(
        first.hand_for_internal(BriarCircuitSeat::Seat0),
        second.hand_for_internal(BriarCircuitSeat::Seat0)
    );
    assert_eq!(first.dealer, second.dealer);
    assert_eq!(first.pass_direction(), second.pass_direction());
}

#[test]
fn sequential_hand_deals_are_replayable_from_seed_and_hand_index() {
    let mut first_rng = SeededRng::from_seed(Seed(1600));
    let first_hand = deal_hand(&mut first_rng, BriarCircuitSeat::Seat0, 0).expect("hand 0");
    let second_hand =
        deal_hand(&mut first_rng, next_dealer(BriarCircuitSeat::Seat0), 1).expect("hand 1");
    let hold_hand =
        deal_hand(&mut first_rng, BriarCircuitSeat::Seat3, 3).expect("hold hand fixture");

    let mut replay_rng = SeededRng::from_seed(Seed(1600));
    let replay_first = deal_hand(&mut replay_rng, BriarCircuitSeat::Seat0, 0).expect("hand 0");
    let replay_second =
        deal_hand(&mut replay_rng, next_dealer(BriarCircuitSeat::Seat0), 1).expect("hand 1");
    let replay_hold = deal_hand(&mut replay_rng, BriarCircuitSeat::Seat3, 3).expect("hand 3");

    assert_eq!(first_hand, replay_first);
    assert_eq!(second_hand, replay_second);
    assert_eq!(hold_hand, replay_hold);
    assert_eq!(first_hand.pass_direction, PassDirection::Left);
    assert_eq!(second_hand.pass_direction, PassDirection::Right);
    assert_eq!(hold_hand.pass_direction, PassDirection::Hold);
    assert_ne!(first_hand.hands, second_hand.hands);
}

#[test]
fn replay_hash_snapshot_reproduces_for_identical_seed() {
    let seats = canonical_seat_ids();
    let first = setup_match(Seed(1614), &seats, &SetupOptions::default()).expect("first setup");
    let second = setup_match(Seed(1614), &seats, &SetupOptions::default()).expect("second setup");

    assert_eq!(replay_hash_snapshot(&first), replay_hash_snapshot(&second));
}

#[test]
fn viewer_exports_round_trip_without_seed_or_deck_order() {
    let seats = canonical_seat_ids();
    let state = setup_match(Seed(1615), &seats, &SetupOptions::default()).expect("setup");
    let public_export = export_viewer_timeline(&state, ViewerExportClass::Public);
    let seat_export = export_viewer_timeline(
        &state,
        ViewerExportClass::SeatPrivate(BriarCircuitSeat::Seat0),
    );

    assert_eq!(
        import_viewer_timeline(&public_export),
        Ok(public_export.clone())
    );
    assert_eq!(
        import_viewer_timeline(&seat_export),
        Ok(seat_export.clone())
    );

    let public_payload = format!("{public_export:?}");
    assert!(!public_payload.contains("Seed("));
    assert!(!public_payload.contains("deck"));
    assert!(!public_payload.contains("private_hands"));

    let private_payload = format!("{seat_export:?}");
    assert!(private_payload.contains("seat_0"));
    assert!(!private_payload.contains("Seed("));
    assert!(!private_payload.contains("deck"));
}

#[test]
fn golden_trace_minimum_inventory_exists() {
    let trace_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden_traces");

    for trace in REQUIRED_TRACES {
        let path = trace_dir.join(trace);
        assert!(path.exists(), "missing golden trace {}", path.display());
        let payload = std::fs::read_to_string(&path).expect("trace readable");
        assert!(
            payload.contains("\"schema_version\":1") || payload.contains("\"schema_version\": 1")
        );
        assert!(
            payload.contains("\"game_id\":\"briar_circuit\"")
                || payload.contains("\"game\": \"briar_circuit\"")
        );
        assert!(
            payload.contains("\"rules_version\":\"briar-circuit-rules-v1\"")
                || payload.contains("\"rules_version\": \"briar-circuit-rules-v1\"")
        );
        assert!(payload.contains("migration_notes"));
    }
}

#[test]
fn characterization_domain_fixture_artifacts_are_pinned() {
    let moon_fixture = include_str!("../data/fixtures/briar_circuit_moon.fixture.json");
    let first_trick_exception_fixture =
        include_str!("../data/fixtures/briar_circuit_first_trick_exception.fixture.json");

    assert_eq!(
        HashValue::from_stable_bytes(moon_fixture.as_bytes()),
        HashValue(12129920730792203110)
    );
    assert_eq!(
        HashValue::from_stable_bytes(first_trick_exception_fixture.as_bytes()),
        HashValue(16932830783837267987)
    );
    assert!(moon_fixture.contains("\"trace_id\": \"briar_circuit_moon\""));
    assert!(first_trick_exception_fixture
        .contains("\"trace_id\": \"briar_circuit_first_trick_exception\""));
    assert!(!moon_fixture.contains("selector"));
    assert!(!first_trick_exception_fixture.contains("selector"));
}

#[test]
fn replay_command_v1_driver_validates_briar_pass_and_play_trace_metadata() {
    let driver = ReplayCommandV1Driver::new("replay-check");
    let profile = replay_command_profile_artifact();
    let trace_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden_traces");

    let delegated = driver
        .validate_with(&profile, |report| {
            assert_eq!(report.profile_id, REPLAY_COMMAND_V1);
            assert_eq!(report.visibility_class, "internal-dev");

            let mut state =
                setup_match(Seed(1600), &canonical_seat_ids(), &SetupOptions::default())
                    .expect("setup");
            let setup_snapshot = replay_hash_snapshot(&state);
            for seat in BriarCircuitSeat::ALL {
                commit_standard_pass(&mut state, seat);
            }
            let play_snapshot = replay_hash_snapshot(&state);
            assert_ne!(setup_snapshot.state_hash, play_snapshot.state_hash);
            format!("{}:briar-circuit", report.profile_id)
        })
        .expect("replay-command-v1 driver accepts Briar virtual adapter");

    assert_eq!(delegated, "replay-command-v1:briar-circuit");

    for file_name in [
        "pass-left-atomic-exchange.trace.json",
        "pass-choice-in-flight-no-leak.trace.json",
        "follow-suit-forced.trace.json",
        "trick-winner-leads-next.trace.json",
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
                    payload.contains("\"schema_version\":1")
                        || payload.contains("\"schema_version\": 1")
                );
                assert!(
                    payload.contains("\"game_id\":\"briar_circuit\"")
                        || payload.contains("\"game_id\": \"briar_circuit\"")
                );
            })
            .expect("virtual replay-command profile validates selected Briar trace");
    }
}

#[test]
fn replay_command_v1_driver_rejects_briar_wrong_metadata() {
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
    unknown_field.fields = &["profile_id", "pass_policy"];
    assert_eq!(
        driver
            .validate(&unknown_field)
            .expect_err("unknown field")
            .kind,
        ProfileValidationErrorKind::UnknownField
    );
}

#[test]
fn public_export_v1_driver_validates_briar_public_timeline_round_trip_and_no_leak() {
    let driver = PublicExportV1Driver::new("wasm-export");
    let profile = public_export_profile_artifact();

    driver
        .validate_with(&profile, |report| {
            assert_eq!(report.profile_id, PUBLIC_EXPORT_V1);
            assert_eq!(report.visibility_class, "public");

            let mut state =
                setup_match(Seed(1600), &canonical_seat_ids(), &SetupOptions::default())
                    .expect("setup");
            let hidden_cards = BriarCircuitSeat::ALL
                .into_iter()
                .flat_map(|seat| state.hand_for_internal(seat).to_vec())
                .collect::<Vec<_>>();
            let selected = state.hand_for_internal(BriarCircuitSeat::Seat0)[0];
            apply_pass_action(
                &mut state,
                BriarCircuitSeat::Seat0,
                PassAction::Select(selected),
            )
            .expect("select pass card");

            let export = export_viewer_timeline(&state, ViewerExportClass::Public);
            let imported = import_viewer_timeline(&export).expect("public export imports");
            assert_eq!(imported, export);
            assert_eq!(export.class, ViewerExportClass::Public);
            assert_eq!(export.viewer_label, "public");

            let payload = format!("{export:?}");
            for card in hidden_cards {
                assert!(
                    !payload.contains(&card.as_str()),
                    "public export leaked {card:?}"
                );
            }
            assert!(!payload.contains(&selected.as_str()));
        })
        .expect("public-export-v1 driver accepts Briar public timeline adapter");
}

#[test]
fn public_export_v1_driver_rejects_briar_wrong_metadata() {
    let driver = PublicExportV1Driver::new("wasm-export");
    let valid = public_export_profile_artifact();

    let mut wrong_owner = valid.clone();
    wrong_owner.metadata.validator_owner = "replay-check";
    assert_eq!(
        driver.validate(&wrong_owner).expect_err("wrong owner").kind,
        ProfileValidationErrorKind::WrongValidatorOwner
    );

    let mut wrong_visibility = valid.clone();
    wrong_visibility.metadata.visibility_class = Some("seat-private");
    assert_eq!(
        driver
            .validate(&wrong_visibility)
            .expect_err("wrong visibility")
            .kind,
        ProfileValidationErrorKind::InvalidVisibility
    );

    let mut unknown_field = valid;
    unknown_field.fields = &["profile_id", "viewer_seat"];
    assert_eq!(
        driver
            .validate(&unknown_field)
            .expect_err("unknown field")
            .kind,
        ProfileValidationErrorKind::UnknownField
    );
}

#[test]
fn moon_fixture_drives_domain_evidence_v1_without_data_scoring() {
    let moon_fixture = include_str!("../data/fixtures/briar_circuit_moon.fixture.json");
    assert!(!moon_fixture.contains("profile_id"));
    assert!(moon_fixture.contains("\"fixture_kind\": \"scoring\""));
    assert!(moon_fixture.contains("\"game_id\": \"briar_circuit\""));
    assert!(moon_fixture.contains("\"rules_version\": \"briar-circuit-rules-v1\""));
    assert!(moon_fixture.contains("\"data_version\": 1"));
    assert!(moon_fixture.contains("migration_notes"));

    DomainEvidenceV1Driver::new("briar_circuit")
        .validate_with(
            &domain_evidence_profile_artifact("internal-dev", false),
            |report| {
                assert_eq!(report.profile_id, DOMAIN_EVIDENCE_V1);
                assert_eq!(report.profile_version, PROFILE_VERSION_V1);
                assert_eq!(report.visibility_class, "internal-dev");
                assert_eq!(report.validator_owner, "briar_circuit");

                let mut point_cards = Vec::new();
                for rank in Rank::ALL {
                    point_cards.push(card(rank, Suit::Hearts));
                }
                point_cards.push(card(Rank::Queen, Suit::Spades));

                let scoring = score_completed_hand(
                    &[captured_trick(BriarCircuitSeat::Seat0, point_cards)],
                    [0, 0, 0, 0],
                );

                assert_eq!(scoring.raw_points, [26, 0, 0, 0]);
                assert_eq!(scoring.moon_shooter, Some(BriarCircuitSeat::Seat0));
                assert_eq!(scoring.hand_additions, [0, 26, 26, 26]);
            },
        )
        .expect("domain-evidence-v1 moon fixture metadata validates");
}

#[test]
fn first_trick_exception_fixture_is_shape_only_domain_evidence() {
    let exception_fixture =
        include_str!("../data/fixtures/briar_circuit_first_trick_exception.fixture.json");
    assert!(!exception_fixture.contains("profile_id"));
    assert!(exception_fixture.contains("\"fixture_kind\": \"play_legality\""));
    assert!(exception_fixture.contains("\"game_id\": \"briar_circuit\""));
    assert!(exception_fixture.contains("\"rules_version\": \"briar-circuit-rules-v1\""));
    assert!(exception_fixture.contains("\"data_version\": 1"));
    assert!(exception_fixture.contains("migration_notes"));

    DomainEvidenceV1Driver::new("briar_circuit")
        .validate_with(
            &domain_evidence_profile_artifact("internal-dev", false),
            |report| {
                assert_eq!(report.profile_id, DOMAIN_EVIDENCE_V1);
                assert_eq!(report.visibility_class, "internal-dev");

                let led = card(Rank::Two, Suit::Clubs);
                let point_cards = vec![
                    card(Rank::Two, Suit::Hearts),
                    card(Rank::Three, Suit::Hearts),
                    card(Rank::Queen, Suit::Spades),
                ];
                let state = briar_circuit::BriarCircuitState {
                    variant: briar_circuit::Variant::briar_circuit_standard(),
                    seats: canonical_seat_ids(),
                    dealer: BriarCircuitSeat::Seat3,
                    hand_index: 0,
                    cumulative_scores: [0, 0, 0, 0],
                    phase: Phase::PlayingTrick(PlayingTrickState {
                        hearts_broken: false,
                        trick_index: 0,
                        leader: BriarCircuitSeat::Seat0,
                        active_seat: BriarCircuitSeat::Seat1,
                        current_trick: CurrentTrick {
                            leader: BriarCircuitSeat::Seat0,
                            plays: vec![TrickPlay {
                                seat: BriarCircuitSeat::Seat0,
                                card: led,
                            }],
                        },
                    }),
                    private_hands: vec![
                        (BriarCircuitSeat::Seat0, vec![]),
                        (BriarCircuitSeat::Seat1, point_cards.clone()),
                        (BriarCircuitSeat::Seat2, vec![]),
                        (BriarCircuitSeat::Seat3, vec![]),
                    ],
                    captured_tricks: Vec::new(),
                    freshness_token: FreshnessToken(0),
                    seed: Seed(1601),
                    last_hand_summary: None,
                };

                let legal = briar_circuit::legal_play_cards(&state, BriarCircuitSeat::Seat1)
                    .expect("legal");
                assert_eq!(legal, point_cards);
            },
        )
        .expect("domain-evidence-v1 first-trick boundary metadata validates");
}

#[test]
fn bot_match_replay_is_deterministic_and_reaches_a_multi_hand_terminal() {
    // A fixed seed fully determines the L1 bot match, so two replays must be identical
    // and must reach a unique-low-score terminal across several hands.
    for seed in [1600_u64, 1601, 1700, 9001] {
        let first = replay_bot_match(seed, 8192).expect("bot match replays");
        let second = replay_bot_match(seed, 8192).expect("bot match replays");
        assert_eq!(first, second, "seed {seed} replay is deterministic");
        assert!(first.terminal, "seed {seed} reaches a terminal outcome");
        assert!(
            first.winner.is_some(),
            "seed {seed} has a unique-low winner"
        );
        assert!(
            first.hands_played >= 2,
            "seed {seed} plays multiple hands (got {})",
            first.hands_played
        );
        // The winner holds the unique lowest cumulative score.
        let winner = first.winner.expect("winner");
        let low = *first.cumulative_scores.iter().min().expect("scores");
        assert_eq!(first.cumulative_scores[winner.index()], low);
        assert_eq!(
            first
                .cumulative_scores
                .iter()
                .filter(|score| **score == low)
                .count(),
            1,
            "seed {seed} winner low score is unique"
        );
    }
}

#[test]
fn bot_match_replay_diverges_across_seeds() {
    // Different seeds produce different deals, so their replay hashes should differ.
    let a = replay_bot_match(1600, 8192).expect("replay");
    let b = replay_bot_match(2600, 8192).expect("replay");
    assert_ne!(
        a.snapshot.state_hash, b.snapshot.state_hash,
        "distinct seeds yield distinct terminal state hashes"
    );
}

#[test]
fn bot_match_replay_records_pass_cycle_dealer_rotation_and_tie_continuation() {
    // Pass direction and dealer rotation are deterministic from the hand index, so the
    // first four hands of any multi-hand match follow the published cycle.
    let canonical = replay_bot_match(1600, 8192).expect("replay");
    assert_eq!(
        canonical.pass_directions[..4],
        ["left", "right", "across", "hold"]
    );
    assert_eq!(
        canonical.dealers[..4],
        ["seat_0", "seat_1", "seat_2", "seat_3"]
    );
    assert_eq!(canonical.tie_continuation_hands, 0);

    // Seed 1625 crosses the threshold with a tied low score, so the match continues.
    let tie = replay_bot_match(1625, 8192).expect("replay");
    assert!(
        tie.tie_continuation_hands >= 1,
        "seed 1625 continues past a tied-low threshold hand"
    );
    assert!(tie.terminal && tie.winner.is_some());
}
