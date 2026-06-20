use engine_core::StableSerialize;
use river_ledger::replay_support::{
    export_public_replay, replay_internal_full_trace, trace_from_commands,
};
use river_ledger::{
    apply_action, project_view, setup_match, validate_command, RiverLedgerSeat, SetupOptions,
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
