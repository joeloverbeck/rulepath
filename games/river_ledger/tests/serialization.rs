use engine_core::{Actor, SeatId, StableSerialize};
use game_test_support::profiles::{
    DomainEvidenceV1Driver, ProfileArtifact, ProfileMetadata, ProfileValidationErrorKind,
    DOMAIN_EVIDENCE_V1, PROFILE_VERSION_V1,
};
use river_ledger::replay_support::{
    command_for_state, export_public_replay, legal_action_tree_v1_encoding,
    replay_internal_full_trace, trace_from_commands,
};
use river_ledger::{
    apply_action, project_view, setup_match, validate_command, PotShare, RiverLedgerSeat,
    SeatLedger, SeatStatus, SetupOptions, STANDARD_STARTING_STACK,
};

const DOMAIN_EVIDENCE_PROFILE_FIELDS: &[&str] = &[
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
    "domain_schema_version",
    "domain_input",
    "expected_domain",
];

const FOUR_PLAYER_CHECKDOWN: &[(usize, &str)] = &[
    (3, "call"),
    (0, "call"),
    (1, "call"),
    (2, "check"),
    (1, "check"),
    (2, "check"),
    (3, "check"),
    (0, "check"),
    (1, "check"),
    (2, "check"),
    (3, "check"),
    (0, "check"),
    (1, "check"),
    (2, "check"),
    (3, "check"),
    (0, "check"),
];

fn domain_evidence_profile_artifact() -> ProfileArtifact<'static> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id: DOMAIN_EVIDENCE_V1,
            profile_version: PROFILE_VERSION_V1,
            visibility_class: Some("internal-dev"),
            validator_owner: "river_ledger",
            canonical_byte_authority: "none",
            migration_update_note: Some("8CR4NSEAPRITRI-028 virtual domain-evidence profile"),
        },
        fields: DOMAIN_EVIDENCE_PROFILE_FIELDS,
        canonical_byte_claim: false,
    }
}

fn uncalled_return_ledgers() -> Vec<SeatLedger> {
    vec![
        SeatLedger {
            seat: RiverLedgerSeat::from_index(0).unwrap(),
            status: SeatStatus::Live,
            starting_stack: 10,
            remaining_stack: 4,
            street_contribution: 6,
            total_contribution: 6,
        },
        SeatLedger {
            seat: RiverLedgerSeat::from_index(1).unwrap(),
            status: SeatStatus::Live,
            starting_stack: 10,
            remaining_stack: 6,
            street_contribution: 4,
            total_contribution: 4,
        },
        SeatLedger {
            seat: RiverLedgerSeat::from_index(2).unwrap(),
            status: SeatStatus::Folded,
            starting_stack: 10,
            remaining_stack: 6,
            street_contribution: 4,
            total_contribution: 4,
        },
    ]
}

#[test]
fn public_replay_export_json_order_is_stable() {
    let trace = trace_from_commands(21, 4, &[(3, "call"), (0, "call")]);
    let export = export_public_replay(&trace, &engine_core::Viewer { seat_id: None });
    let first = export.to_json();
    let second = export.to_json();

    assert_eq!(first, second);
    assert_eq!(export.stable_bytes(), first.as_bytes());
    assert!(first.starts_with(
        "{\"schema_version\":1,\"export_class\":\"viewer_scoped_public_replay\",\"viewer\":\"observer\",\"game_id\":\"river_ledger\""
    ));
}

#[test]
fn internal_trace_json_order_is_stable() {
    let trace = trace_from_commands(21, 4, &[(3, "call"), (0, "call")]);
    let json = trace.to_json();

    assert_eq!(trace.stable_bytes(), json.as_bytes());
    assert!(json.starts_with(
        "{\"schema_version\":1,\"game_id\":\"river_ledger\",\"rules_version\":\"river-ledger-rules-v2\""
    ));
    assert!(json.contains("\"commands\":[{\"actor\":\"seat_3\",\"path\":[\"call\"]},{\"actor\":\"seat_0\",\"path\":[\"call\"]}]"));
}

#[test]
fn domain_evidence_v1_driver_delegates_side_pot_semantics_to_river() {
    let driver = DomainEvidenceV1Driver::new("river_ledger");
    let profile = domain_evidence_profile_artifact();

    for file_name in [
        "three-way-main-two-side-pots.trace.json",
        "uncalled-return.trace.json",
        "per-pot-remainder-button-order.trace.json",
    ] {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/golden_traces")
            .join(file_name);
        let fixture = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        assert!(!fixture.contains("\"profile_id\""));
        assert!(!fixture.contains("\"canonical_byte_authority\""));
    }

    driver
        .validate_with(&profile, |report| {
            assert_eq!(report.profile_id, DOMAIN_EVIDENCE_V1);
            assert_eq!(report.validator_owner, "river_ledger");

            let mut three_way_state = setup_with_stacks(12, vec![8, 3, 2]);
            for (seat, segment) in [("seat_0", "raise"), ("seat_1", "call")] {
                apply_segment(&mut three_way_state, seat, segment);
            }
            let three_way_view =
                project_view(&three_way_state, &engine_core::Viewer { seat_id: None });
            let summary = three_way_view.stable_summary();
            assert!(summary.contains("main_pot:6"));
            assert!(summary.contains("side_pot_1:2"));

            let uncalled_layers =
                river_ledger::pot::construct_contribution_layers(&uncalled_return_ledgers());
            assert_eq!(uncalled_layers.returns.len(), 1);
            assert_eq!(
                uncalled_layers.returns[0].seat,
                RiverLedgerSeat::from_index(0).unwrap()
            );
            assert_eq!(uncalled_layers.returns[0].amount, 2);

            let remainder = river_ledger::pot::allocate_single_pot(
                11,
                &[
                    RiverLedgerSeat::from_index(0).unwrap(),
                    RiverLedgerSeat::from_index(2).unwrap(),
                    RiverLedgerSeat::from_index(3).unwrap(),
                ],
                RiverLedgerSeat::from_index(2).unwrap(),
                4,
            );
            assert_eq!(remainder.remainder, 2);
            assert_eq!(
                remainder.remainder_order,
                vec![
                    RiverLedgerSeat::from_index(2).unwrap(),
                    RiverLedgerSeat::from_index(3).unwrap(),
                    RiverLedgerSeat::from_index(0).unwrap(),
                ]
            );
            assert_eq!(
                remainder.shares,
                vec![
                    PotShare {
                        seat: RiverLedgerSeat::from_index(0).unwrap(),
                        amount: 3,
                    },
                    PotShare {
                        seat: RiverLedgerSeat::from_index(2).unwrap(),
                        amount: 4,
                    },
                    PotShare {
                        seat: RiverLedgerSeat::from_index(3).unwrap(),
                        amount: 4,
                    },
                ]
            );
        })
        .expect("domain-evidence-v1 driver accepts River side-pot evidence adapter");
}

#[test]
fn domain_evidence_v1_driver_rejects_wrong_metadata() {
    let driver = DomainEvidenceV1Driver::new("river_ledger");
    let valid = domain_evidence_profile_artifact();

    let mut wrong_owner = valid.clone();
    wrong_owner.metadata.validator_owner = "fixture-check";
    assert_eq!(
        driver.validate(&wrong_owner).expect_err("wrong owner").kind,
        ProfileValidationErrorKind::WrongValidatorOwner
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

    let mut byte_claim = valid;
    byte_claim.canonical_byte_claim = true;
    assert_eq!(
        driver
            .validate(&byte_claim)
            .expect_err("canonical byte claim")
            .kind,
        ProfileValidationErrorKind::IllegalCanonicalByteClaim
    );
}

#[test]
fn public_display_labels_do_not_replace_canonical_seat_ids() {
    let state = setup_match(
        engine_core::Seed(21),
        &(0..4)
            .map(|index| engine_core::SeatId(format!("seat_{index}")))
            .collect::<Vec<_>>(),
        &SetupOptions::default(),
    )
    .expect("setup");

    assert_eq!(RiverLedgerSeat::from_index(1).unwrap().as_str(), "seat_1");
    assert!(state.stable_internal_summary().contains("seat_1"));
    assert!(state.stable_internal_summary().contains("betting="));
}

#[test]
fn active_seat_labels_serialize_exact_match_seats() {
    for count in 3..=6 {
        let state = setup_match(
            engine_core::Seed(500 + count as u64),
            &(0..count)
                .map(|index| engine_core::SeatId(format!("seat_{index}")))
                .collect::<Vec<_>>(),
            &SetupOptions::default(),
        )
        .expect("setup");
        let view = project_view(&state, &engine_core::Viewer { seat_id: None });

        assert_eq!(view.active_seat_labels.len(), count);
        assert_eq!(
            view.active_seat_labels
                .iter()
                .map(|label| (label.seat.as_str(), label.label.as_str()))
                .collect::<Vec<_>>(),
            (0..count)
                .map(|index| {
                    let seat = format!("seat_{index}");
                    let label = format!("Seat {}", index + 1);
                    (seat, label)
                })
                .collect::<Vec<_>>()
                .iter()
                .map(|(seat, label)| (seat.as_str(), label.as_str()))
                .collect::<Vec<_>>()
        );

        let summary = view.stable_summary();
        assert!(summary.contains(&format!(
            "active_labels={}",
            (0..count)
                .map(|index| format!("seat_{index}=Seat {}", index + 1))
                .collect::<Vec<_>>()
                .join(",")
        )));
        assert!(!summary.contains(&format!("seat_{count}=Seat {}", count + 1)));
    }
}

#[test]
fn terminal_view_summary_includes_v2_showdown_presentation_deterministically() {
    let trace = trace_from_commands(79, 4, FOUR_PLAYER_CHECKDOWN);
    let result = replay_internal_full_trace(&trace);
    let view = project_view(&result.final_state, &engine_core::Viewer { seat_id: None });
    let first = view.stable_summary();
    let second = view.stable_summary();

    assert_eq!(first, second);
    assert!(first.contains("showdown:"));
    assert!(first.contains("wins with"));
    assert!(first.contains("tiers="));
}

#[test]
fn seed_10018_terminal_serialization_uses_public_winner_label() {
    let trace = trace_from_commands(10018, 4, FOUR_PLAYER_CHECKDOWN);
    let result = replay_internal_full_trace(&trace);
    let view = project_view(&result.final_state, &engine_core::Viewer { seat_id: None });
    let summary = view.stable_summary();

    assert!(summary.contains("showdown:seat_0:"));
    assert!(summary.contains("Seat 1 wins with Two pair, Queens and Fives."));
    assert!(!summary.contains("Seat 0 wins"));
}

#[test]
fn seed_31_terminal_serialization_keeps_canonical_split_order() {
    let trace = trace_from_commands(31, 4, FOUR_PLAYER_CHECKDOWN);
    let result = replay_internal_full_trace(&trace);
    let view = project_view(&result.final_state, &engine_core::Viewer { seat_id: None });
    let summary = view.stable_summary();

    assert!(summary.contains("showdown:seat_1,seat_2,seat_3:"));
    assert!(summary.contains("seat_1=3,seat_2=3,seat_3=2"));
    assert!(summary.contains("Seat 2, Seat 3, and Seat 4 split the ledger"));
}

#[test]
fn all_in_side_pot_state_and_view_hashes_are_deterministic_for_same_canonical_input() {
    fn run() -> river_ledger::RiverLedgerState {
        let mut state = setup_match(
            engine_core::Seed(12),
            &(0..3)
                .map(|index| engine_core::SeatId(format!("seat_{index}")))
                .collect::<Vec<_>>(),
            &SetupOptions {
                starting_stacks: Some(vec![8, 3, 2]),
                ..SetupOptions::default()
            },
        )
        .expect("setup");

        for (seat, segment) in [("seat_0", "raise"), ("seat_1", "call")] {
            let command = river_ledger::replay_support::command_for_state(
                &state,
                seat,
                vec![segment.to_owned()],
            );
            let action = validate_command(&state, &command).expect("valid command");
            apply_action(&mut state, action).expect("action applies");
        }

        state
    }

    let first = run();
    let second = run();
    let first_view = project_view(&first, &engine_core::Viewer { seat_id: None });
    let second_view = project_view(&second, &engine_core::Viewer { seat_id: None });

    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
    assert_eq!(first_view.stable_summary(), second_view.stable_summary());
    assert_eq!(
        engine_core::HashValue::from_stable_bytes(first.stable_internal_summary().as_bytes()),
        engine_core::HashValue::from_stable_bytes(second.stable_internal_summary().as_bytes())
    );
    assert_eq!(first_view.stable_hash(), second_view.stable_hash());

    let summary = first_view.stable_summary();
    assert!(summary.contains(
        "tiers=main_pot:6:0-2:contributorsseat_0,seat_1,seat_2:eligibleseat_0,seat_1,seat_2"
    ));
    assert!(summary.contains("side_pot_1:2:2-3:contributorsseat_0,seat_1:eligibleseat_0,seat_1"));
    assert_eq!(first.ledger.seats[0].starting_stack, 8);
    assert_eq!(first.ledger.seats[0].remaining_stack, 5);
    assert_eq!(first.ledger.seats[0].total_contribution, 3);
    assert!(summary.contains("terminal=showdown"));
}

#[test]
fn richer_all_in_side_pot_action_tree_v1_vectors_are_deterministic() {
    let short_small_blind = include_str!("golden_traces/short-small-blind-all-in.trace.json");
    assert!(short_small_blind.contains("\"trace_id\": \"river-ledger-short-small-blind-all-in\""));
    let short_small_state = setup_with_stacks(1512, vec![8, 1, 24]);
    assert_action_tree_v1_fixture(
        "short-small-blind-all-in",
        &short_small_state,
        8459325685730745957,
    );

    let short_raise = include_str!("golden_traces/short-raise-all-in.trace.json");
    assert!(short_raise.contains("\"trace_id\": \"river-ledger-short-raise-all-in\""));
    let short_raise_state = setup_with_stacks(1517, vec![3, 16, 24]);
    assert_action_tree_v1_fixture(
        "short-raise-all-in",
        &short_raise_state,
        17769399973547313107,
    );

    let cumulative = include_str!("golden_traces/cumulative-reopen.trace.json");
    assert!(cumulative.contains("\"trace_id\": \"river-ledger-cumulative-reopen\""));
    let mut cumulative_state = flop_state_with_remaining([STANDARD_STARTING_STACK, 20, 20, 3]);
    for (seat, segment) in [
        ("seat_1", "bet"),
        ("seat_2", "call"),
        ("seat_3", "raise"),
        ("seat_0", "raise"),
    ] {
        apply_segment(&mut cumulative_state, seat, segment);
    }
    assert_action_tree_v1_fixture("cumulative-reopen", &cumulative_state, 16754997844699440739);

    let all_runout = include_str!("golden_traces/all-all-in-runout.trace.json");
    assert!(all_runout.contains("\"trace_id\": \"river-ledger-all-all-in-runout\""));
    let mut all_runout_state = setup_with_stacks(1528, vec![4, 4, 2]);
    for (seat, segment) in [("seat_0", "raise"), ("seat_1", "call")] {
        apply_segment(&mut all_runout_state, seat, segment);
    }
    assert_action_tree_v1_fixture("all-all-in-runout", &all_runout_state, 1933656550434207641);

    let three_way = include_str!("golden_traces/three-way-main-two-side-pots.trace.json");
    assert!(three_way.contains("\"trace_id\": \"river-ledger-three-way-main-two-side-pots\""));
    let three_way_state = setup_with_stacks(1521, vec![2, 5, 9]);
    assert_action_tree_v1_fixture(
        "three-way-main-two-side-pots",
        &three_way_state,
        11683784456165567154,
    );
}

fn setup_with_stacks(seed: u64, starting_stacks: Vec<u16>) -> river_ledger::RiverLedgerState {
    setup_match(
        engine_core::Seed(seed),
        &(0..starting_stacks.len())
            .map(|index| SeatId(format!("seat_{index}")))
            .collect::<Vec<_>>(),
        &SetupOptions {
            starting_stacks: Some(starting_stacks),
            ..SetupOptions::default()
        },
    )
    .expect("setup")
}

fn flop_state_with_remaining(remaining: [u16; 4]) -> river_ledger::RiverLedgerState {
    let mut state = setup_with_stacks(21, vec![STANDARD_STARTING_STACK; 4]);
    state.phase = river_ledger::Phase::Betting {
        street: river_ledger::Street::Flop,
    };
    state.active_seat = RiverLedgerSeat::from_index(1);
    for (index, seat_ledger) in state.ledger.seats.iter_mut().enumerate() {
        seat_ledger.street_contribution = 0;
        seat_ledger.remaining_stack = remaining[index];
        seat_ledger.starting_stack = seat_ledger.total_contribution + remaining[index];
        seat_ledger.status = if remaining[index] == 0 {
            river_ledger::SeatStatus::AllIn
        } else {
            river_ledger::SeatStatus::Live
        };
    }
    state.betting = river_ledger::BettingRoundState::for_street(
        river_ledger::Street::Flop,
        vec![
            RiverLedgerSeat::from_index(1).expect("seat 1"),
            RiverLedgerSeat::from_index(2).expect("seat 2"),
            RiverLedgerSeat::from_index(3).expect("seat 3"),
            RiverLedgerSeat::from_index(0).expect("seat 0"),
        ],
    );
    state
}

fn apply_segment(state: &mut river_ledger::RiverLedgerState, seat: &str, segment: &str) {
    let command = command_for_state(state, seat, vec![segment.to_owned()]);
    let action = validate_command(state, &command).expect("valid command");
    apply_action(state, action).expect("action applies");
}

fn assert_action_tree_v1_fixture(
    label: &str,
    state: &river_ledger::RiverLedgerState,
    expected_hash: u64,
) {
    let actor = state
        .active_seat
        .map(|seat| SeatId(seat.as_str()))
        .unwrap_or_else(|| SeatId("seat_0".to_owned()));
    let actor = Actor { seat_id: actor };
    let encoding = legal_action_tree_v1_encoding(state, &actor);
    let repeated = legal_action_tree_v1_encoding(state, &actor);

    assert_eq!(encoding, repeated, "{label} v1 encoding is deterministic");
    assert_eq!(
        encoding.stable_hash,
        engine_core::HashValue::from_stable_bytes(&encoding.stable_bytes),
        "{label} v1 hash matches bytes"
    );
    assert_eq!(
        encoding.stable_hash,
        engine_core::HashValue(expected_hash),
        "{label} v1 hash sentinel"
    );
}
