use engine_core::{HashValue, SeatId, StableSerialize, Viewer};
use river_ledger::{
    replay_support::{
        export_public_replay, import_public_export, replay_internal_full_trace,
        replay_internal_full_trace_result, trace_from_commands,
    },
    setup_match, PotShare, RiverLedgerSeat, SetupOptions, TerminalOutcome,
};

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

const GATE_15_1_GOLDEN_TRACE_FILES: &[&str] = &[
    "setup-equal-default-stacks-3p.trace.json",
    "setup-asymmetric-stacks-6p.trace.json",
    "short-small-blind-all-in.trace.json",
    "short-big-blind-all-in.trace.json",
    "call-all-in-below-price.trace.json",
    "exact-call-exhausts-stack.trace.json",
    "short-open-bet-all-in.trace.json",
    "short-raise-all-in.trace.json",
    "cumulative-reopen.trace.json",
    "full-all-in-raise.trace.json",
    "cap-blocks-short-raise.trace.json",
    "three-way-main-two-side-pots.trace.json",
    "folded-contribution-retained.trace.json",
    "uncalled-return.trace.json",
    "sole-eligible-pot.trace.json",
    "different-winners-across-pots.trace.json",
    "tied-winners-in-pot.trace.json",
    "per-pot-remainder-button-order.trace.json",
    "all-all-in-runout.trace.json",
    "public-observer-multipot-no-leak.trace.json",
    "seat-private-multipot-no-leak.trace.json",
    "wasm-exported-side-pot-terminal.trace.json",
];

fn hidden_ids(seed: u64, seat_count: usize) -> Vec<String> {
    let seats = (0..seat_count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect::<Vec<_>>();
    let state =
        setup_match(engine_core::Seed(seed), &seats, &SetupOptions::default()).expect("setup");
    state
        .private_hands_internal()
        .iter()
        .flatten()
        .chain(state.community_deck_internal().iter())
        .chain(state.deck_tail_internal().iter())
        .map(|card| card.id())
        .collect()
}

fn read_golden_trace(file_name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden_traces")
        .join(file_name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read golden trace {}: {error}", path.display()))
}

fn expected_trace_id(file_name: &str) -> String {
    format!(
        "river-ledger-{}",
        file_name
            .strip_suffix(".trace.json")
            .expect("golden trace suffix")
    )
}

#[test]
fn internal_trace_replays_to_same_hashes_and_state() {
    let trace = trace_from_commands(
        21,
        4,
        &[(3, "call"), (0, "call"), (1, "call"), (2, "check")],
    );
    let first = replay_internal_full_trace(&trace);
    let second = replay_internal_full_trace(&trace);

    assert_eq!(first.trace_hash, second.trace_hash);
    assert_eq!(first.state_hash, second.state_hash);
    assert_eq!(first.effect_hash, second.effect_hash);
    assert_eq!(first.view_hash, second.view_hash);
    assert_eq!(first.action_tree_hashes, second.action_tree_hashes);
    assert_eq!(
        first.final_state.stable_internal_summary(),
        second.final_state.stable_internal_summary()
    );
}

#[test]
fn gate_15_1_golden_trace_set_is_present_and_reviewed() {
    assert_eq!(GATE_15_1_GOLDEN_TRACE_FILES.len(), 22);

    let mut combined = String::new();
    for file_name in GATE_15_1_GOLDEN_TRACE_FILES {
        let contents = read_golden_trace(file_name);
        let trace_id = expected_trace_id(file_name);

        assert!(
            contents.contains(&format!("\"trace_id\": \"{trace_id}\"")),
            "{file_name} must use its canonical trace_id"
        );
        assert!(
            contents.contains("\"schema_version\": 1"),
            "{file_name} must stay on the replay-check schema"
        );
        assert!(
            contents.contains("\"rules_version\": \"river-ledger-rules-v1\""),
            "{file_name} must declare the current replay-check rule version"
        );
        assert!(
            contents.contains("\"migration_review\""),
            "{file_name} must carry an individual v2 migration review note"
        );
        assert!(
            contents.contains("\"expected_public_result\"")
                || contents.contains("\"expected_public_setup\"")
                || contents.contains("\"expected_diagnostics\"")
                || contents.contains("\"forbidden_public_facts\"")
                || contents.contains("\"forbidden_cross_seat_facts\"")
                || contents.contains("\"public_export\""),
            "{file_name} must record a reviewable public expectation"
        );

        combined.push_str(&contents);
        combined.push('\n');
    }

    for required_marker in [
        "\"starting_stacks\": [24, 24, 24]",
        "\"starting_stacks\": [4, 8, 12, 16, 20, 24]",
        "\"starting_stacks\": [8, 3, 2]",
        "\"starting_stacks\": [2, 5, 9]",
        "\"raise_cap_reached\"",
        "\"reopen_after_full_unit_pressure\"",
        "\"is_full_raise\"",
        "\"folded_contribution_retained\"",
        "\"returned\"",
        "\"sole_eligible_pot\"",
        "\"pot_winners\"",
        "\"odd_units_in_multiple_pots\"",
        "\"viewer\": \"observer\"",
        "\"viewer\": \"seat_0\"",
        "\"public_export\"",
        "\"terminal\"",
    ] {
        assert!(
            combined.contains(required_marker),
            "Gate 15.1 trace set is missing marker {required_marker}"
        );
    }
}

#[test]
fn v1_internal_trace_is_rejected_with_stable_diagnostic() {
    let mut trace = trace_from_commands(21, 4, &[(3, "call")]);
    trace.rules_version = "river-ledger-rules-v1".to_owned();

    let diagnostic = replay_internal_full_trace_result(&trace).expect_err("v1 rejects");

    assert_eq!(diagnostic.code, "river_ledger_rules_version_mismatch");
    assert_eq!(
        diagnostic.message,
        "River Ledger replay uses river-ledger-rules-v1; expected river-ledger-rules-v2"
    );
}

#[test]
fn public_export_import_round_trips_for_observer_and_seat_viewer() {
    let trace = trace_from_commands(21, 4, &[(3, "call"), (0, "call")]);
    for viewer in [
        Viewer { seat_id: None },
        Viewer {
            seat_id: Some(SeatId("seat_0".to_owned())),
        },
    ] {
        let export = export_public_replay(&trace, &viewer);
        let imported = import_public_export(&export);

        assert_eq!(imported.viewer, export.viewer);
        assert_eq!(imported.steps, export.steps);
        assert_eq!(
            HashValue::from_stable_bytes(export.to_json().as_bytes()),
            export.stable_hash()
        );
    }
}

#[test]
fn characterization_setup_export_and_visibility_artifacts_are_pinned() {
    let setup_fixture = include_str!("../data/fixtures/river_ledger_3p_standard.fixture.json");
    let public_trace = include_str!("golden_traces/public-replay-export-import.trace.json");
    let seat_private_trace = include_str!("golden_traces/seat-private-view.trace.json");
    let trace = trace_from_commands(21, 3, &[]);
    let observer_export = export_public_replay(&trace, &Viewer { seat_id: None });
    let seat_export = export_public_replay(
        &trace,
        &Viewer {
            seat_id: Some(SeatId("seat_0".to_owned())),
        },
    );

    assert_eq!(
        HashValue::from_stable_bytes(setup_fixture.as_bytes()),
        HashValue(2633580370171550625)
    );
    assert_eq!(
        HashValue::from_stable_bytes(public_trace.as_bytes()),
        HashValue(11946834064931283956)
    );
    assert_eq!(
        HashValue::from_stable_bytes(seat_private_trace.as_bytes()),
        HashValue(6382002720248622821)
    );
    assert_eq!(observer_export.stable_hash(), HashValue(2482097568303728278));
    assert_eq!(seat_export.stable_hash(), HashValue(7443748736294317283));
    assert_eq!(observer_export.viewer, "observer");
    assert_eq!(seat_export.viewer, "seat_0");
}

#[test]
fn observer_public_export_omits_hidden_facts_and_seed() {
    let trace = trace_from_commands(21, 4, &[(3, "call"), (0, "call")]);
    let export = export_public_replay(&trace, &Viewer { seat_id: None });
    let json = export.to_json();

    for hidden in hidden_ids(21, 4) {
        assert!(!json.contains(&hidden), "public export leaked {hidden}");
    }
    assert!(!json.contains("seed_evidence"));
    assert!(!json.contains("\"seed\""));
}

#[test]
fn terminal_public_export_keeps_v2_showdown_surface_public_and_deterministic() {
    let trace = trace_from_commands(79, 4, FOUR_PLAYER_CHECKDOWN);
    let first = export_public_replay(&trace, &Viewer { seat_id: None });
    let second = export_public_replay(&trace, &Viewer { seat_id: None });
    let json = first.to_json();

    assert_eq!(first.steps, second.steps);
    assert!(json.contains("wins with"));
    assert!(json.contains("showdown:"));
    assert!(!json.contains("private_hands"));
    assert!(!json.contains("seed_evidence"));
    assert!(!json.contains("\"seed\""));
}

#[test]
fn seed_10018_public_replay_uses_one_based_unique_winner_label() {
    let trace = trace_from_commands(10018, 4, FOUR_PLAYER_CHECKDOWN);
    let result = replay_internal_full_trace(&trace);
    let TerminalOutcome::Showdown {
        winners,
        allocations,
        headline,
        presentation_v2,
        ..
    } = result
        .final_state
        .terminal_outcome
        .as_ref()
        .expect("terminal outcome")
    else {
        panic!("showdown terminal expected");
    };

    assert_eq!(winners, &vec![RiverLedgerSeat::from_index(0).unwrap()]);
    assert_eq!(
        allocations,
        &vec![PotShare {
            seat: RiverLedgerSeat::from_index(0).unwrap(),
            amount: result.final_state.ledger.pot_total,
        }]
    );
    assert_eq!(headline, "Seat 1 wins with Two pair, Queens and Fives.");
    assert_eq!(presentation_v2.standings[0].seat_label, "Seat 1");

    let export = export_public_replay(&trace, &Viewer { seat_id: None });
    let json = export.to_json();
    assert!(json.contains("Seat 1 wins with Two pair, Queens and Fives."));
    assert!(!json.contains("Seat 0 wins"));
    assert!(!json.contains("seat_0 wins"));
}

#[test]
fn seed_31_public_replay_keeps_split_winners_in_canonical_order() {
    let trace = trace_from_commands(31, 4, FOUR_PLAYER_CHECKDOWN);
    let result = replay_internal_full_trace(&trace);
    let TerminalOutcome::Showdown {
        winners,
        allocations,
        headline,
        presentation_v2,
        ..
    } = result
        .final_state
        .terminal_outcome
        .as_ref()
        .expect("terminal outcome")
    else {
        panic!("showdown terminal expected");
    };

    assert_eq!(
        winners,
        &vec![
            RiverLedgerSeat::from_index(1).unwrap(),
            RiverLedgerSeat::from_index(2).unwrap(),
            RiverLedgerSeat::from_index(3).unwrap(),
        ]
    );
    assert_eq!(
        allocations
            .iter()
            .map(|share| share.seat)
            .collect::<Vec<_>>(),
        *winners
    );
    assert!(headline.starts_with("Seat 2, Seat 3, and Seat 4 split the ledger"));
    assert_eq!(
        presentation_v2
            .standings
            .iter()
            .filter(|standing| standing.result_label == "Split win")
            .map(|standing| standing.seat)
            .collect::<Vec<_>>(),
        *winners
    );
}
