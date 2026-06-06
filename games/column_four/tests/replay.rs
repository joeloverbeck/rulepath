use column_four::replay_support::{replay_commands, ColumnFourReplayJson};
use column_four::{ColumnFourSeat, ColumnId, RowId, TerminalOutcome, WinningLine};
use engine_core::StableSerialize;

fn cell(row: RowId, column: ColumnId) -> column_four::CellId {
    column_four::CellId::new(row, column)
}

#[test]
fn replay_hashes_are_identical_for_same_input_stream() {
    let commands = vec![
        "drop/c1".to_owned(),
        "drop/c2".to_owned(),
        "drop/c1".to_owned(),
        "drop/c2".to_owned(),
        "drop/c1".to_owned(),
        "drop/c2".to_owned(),
        "drop/c1".to_owned(),
    ];

    let left = replay_commands(99, &commands);
    let right = replay_commands(99, &commands);

    assert_eq!(left, right);
    assert_eq!(
        left.outcome,
        Some(TerminalOutcome::Win {
            seat: ColumnFourSeat::Seat0,
            line: WinningLine {
                cells: [
                    cell(RowId::R1, ColumnId::C1),
                    cell(RowId::R2, ColumnId::C1),
                    cell(RowId::R3, ColumnId::C1),
                    cell(RowId::R4, ColumnId::C1),
                ]
            }
        })
    );
    assert_eq!(
        left.projections.last().unwrap().public_view_hash,
        left.view_hash
    );
    assert!(left
        .projections
        .last()
        .unwrap()
        .effects
        .iter()
        .any(|effect| effect.starts_with("WinDetected:seat_0")));
}

#[test]
fn replay_json_stable_serialization_rejects_unknown_fields() {
    let replay = ColumnFourReplayJson {
        schema_version: 1,
        game_id: "column_four".to_owned(),
        rules_version: "column_four-rules-v1".to_owned(),
        variant: "column_four_standard".to_owned(),
        seed: 3,
        initial_snapshot: "snapshot".to_owned(),
        command_segments: vec!["drop/c4".to_owned(), "drop/c3".to_owned()],
    };
    let json = replay.to_json();

    assert_eq!(json.as_bytes(), replay.stable_bytes());
    assert_eq!(ColumnFourReplayJson::from_json(&json).unwrap(), replay);
    assert!(ColumnFourReplayJson::from_json("{\"debug\":\"nope\"}").is_err());
}
