use engine_core::StableSerialize;
use river_ledger::replay_support::{export_public_replay, trace_from_commands};

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
        "{\"schema_version\":1,\"game_id\":\"river_ledger\",\"rules_version\":\"river-ledger-rules-v1\""
    ));
    assert!(json.contains("\"commands\":[{\"actor\":\"seat_3\",\"path\":[\"call\"]},{\"actor\":\"seat_0\",\"path\":[\"call\"]}]"));
}
