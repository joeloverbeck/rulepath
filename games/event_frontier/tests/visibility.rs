use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, Viewer};
use event_frontier::{
    apply_command, export_public_replay, legal_action_tree, project_view, public_replay_step,
    setup_match, validate_command, EventFrontierEffect, SetupOptions, ACTION_PASS,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn viewer(seat: Option<&str>) -> Viewer {
    Viewer {
        seat_id: seat.map(|seat| SeatId(seat.to_owned())),
    }
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn pass_command(seat: &str, state: &event_frontier::EventFrontierState) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec![ACTION_PASS.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn seat_and_observer_views_are_output_equivalent() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");

    let observer = project_view(&state, &viewer(None));
    let seat_0 = project_view(&state, &viewer(Some("seat_0")));
    let seat_1 = project_view(&state, &viewer(Some("seat_1")));

    assert_eq!(observer, seat_0);
    assert_eq!(observer, seat_1);
    assert_eq!(
        observer.current_card.as_deref(),
        Some("ef_high_meadow_fair")
    );
    assert_eq!(
        observer.next_public_card.as_deref(),
        Some("ef_reckoning_one")
    );
}

#[test]
fn public_surfaces_do_not_contain_hidden_undrawn_order() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    let hidden = state.deck.undrawn[0].as_str();

    let view = project_view(&state, &viewer(None));
    assert!(!format!("{view:?}").contains(hidden));
    assert!(!view.stable_summary().contains(hidden));

    let tree = legal_action_tree(&state, &actor("seat_1"));
    assert!(!format!("{tree:?}").contains(hidden));

    let malformed = CommandEnvelope {
        actor: actor("seat_1"),
        action_path: ActionPath {
            segments: vec!["operation/cache/site_charterhouse".to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    };
    let diagnostic = validate_command(&state, &malformed).expect_err("diagnostic");
    assert!(!format!("{diagnostic:?}").contains(hidden));

    let command = pass_command("seat_1", &state);
    let applied = apply_command(&mut state, &command).expect("pass");
    assert!(!format!("{:?}", applied.effects).contains(hidden));

    let step = public_replay_step(0, &state, &command, &applied.effects, &viewer(None));
    let export = export_public_replay(state.variant.id.clone(), &viewer(None), vec![step]);
    assert!(!export.stable_summary().contains(hidden));
    assert!(!export.to_json().contains(hidden));
}

#[test]
fn effect_filtering_keeps_public_payloads_only() {
    let effects = vec![engine_core::EffectEnvelope {
        visibility: engine_core::VisibilityScope::Public,
        payload: EventFrontierEffect::CardDiscarded {
            card: event_frontier::CardId::HighMeadowFair,
            reason: "test".to_owned(),
        },
    }];
    let filtered = event_frontier::filter_effects_for_viewer(&effects, &viewer(Some("seat_0")));
    assert_eq!(filtered, effects);
}
