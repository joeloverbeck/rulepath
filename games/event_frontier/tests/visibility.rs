use engine_core::{
    ActionChoice, ActionNode, ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed,
    Viewer,
};
use game_test_support::no_leak::{
    assert_pairwise_no_leak, ExposureExpectation, LeakProbe,
};
use event_frontier::{
    apply_command, export_public_replay, legal_action_tree, project_view, public_replay_step,
    setup_match, validate_command, EventFrontierEffect, SetupOptions, ACTION_OPERATION,
    ACTION_PASS,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MatrixViewer {
    Observer,
    Seat0,
    Seat1,
}

impl MatrixViewer {
    fn viewer(self) -> Viewer {
        match self {
            Self::Observer => viewer(None),
            Self::Seat0 => viewer(Some("seat_0")),
            Self::Seat1 => viewer(Some("seat_1")),
        }
    }

    fn actor(self) -> Actor {
        match self {
            Self::Observer | Self::Seat0 => actor("seat_0"),
            Self::Seat1 => actor("seat_1"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MatrixSurface {
    View,
    ActionTree,
    Diagnostic,
    Effects,
}

#[test]
fn pairwise_hidden_deeper_deck_matrix_covers_public_surfaces() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    let probes = hidden_deck_probes(&state);

    assert_pairwise_no_leak(
        [MatrixViewer::Observer, MatrixViewer::Seat0, MatrixViewer::Seat1],
        [
            MatrixSurface::View,
            MatrixSurface::ActionTree,
            MatrixSurface::Diagnostic,
            MatrixSurface::Effects,
        ],
        probes,
        |viewer_case, surface| matrix_snapshot(&state, *viewer_case, *surface),
        |_source, _viewer, _surface, _canary_id| ExposureExpectation::MustBeAbsent,
        |snapshot, card| snapshot.contains(card.as_str()),
    )
    .expect("Event Frontier hidden-deck matrix has no failures");
}

fn hidden_deck_probes(
    state: &event_frontier::EventFrontierState,
) -> Vec<LeakProbe<usize, &'static str, event_frontier::CardId>> {
    state
        .deck
        .undrawn
        .iter()
        .filter(|card| Some(**card) != state.deck.current)
        .filter(|card| Some(**card) != state.deck.next_public)
        .enumerate()
        .map(|(index, card)| LeakProbe {
            source_seat: index,
            canary_id: card.as_str(),
            canary: *card,
        })
        .collect()
}

fn matrix_snapshot(
    state: &event_frontier::EventFrontierState,
    viewer_case: MatrixViewer,
    surface: MatrixSurface,
) -> String {
    match surface {
        MatrixSurface::View => format!("{:?}", project_view(state, &viewer_case.viewer())),
        MatrixSurface::ActionTree => {
            format!("{:?}", legal_action_tree(state, &viewer_case.actor()))
        }
        MatrixSurface::Diagnostic => {
            let malformed = CommandEnvelope {
                actor: actor("seat_1"),
                action_path: ActionPath {
                    segments: vec!["operation/cache/site_charterhouse".to_owned()],
                },
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(1),
            };
            format!(
                "{:?}",
                validate_command(state, &malformed).expect_err("diagnostic")
            )
        }
        MatrixSurface::Effects => "no_effects_before_command".to_owned(),
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
    let current = observer.current_card.as_ref().expect("current card");
    let next = observer
        .next_public_card
        .as_ref()
        .expect("next public card");
    assert_eq!(current.id, "ef_high_meadow_fair");
    assert_eq!(current.label, "High Meadow Fair");
    assert!(current.summary.contains("Freeholders gain"));
    assert_eq!(next.id, "ef_reckoning_one");
    assert_eq!(next.label, "First Reckoning");
    assert_eq!(observer.ui.face_down_label, "Face-down event deck");
    assert_eq!(
        observer
            .sites
            .iter()
            .find(|site| site.site == event_frontier::SiteId::GranitePass)
            .expect("Granite Pass site")
            .label,
        "Granite Pass"
    );
}

#[test]
fn operation_action_labels_use_authored_site_names() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    state.deck.current = Some(event_frontier::CardId::LastLight);
    state.card_phase = event_frontier::CardPhase::AwaitingFirstChoice {
        faction: event_frontier::FactionId::Charter,
    };
    let tree = legal_action_tree(&state, &actor("seat_0"));
    let labels = action_labels(&tree.root);

    assert!(labels
        .iter()
        .any(|text| text.contains("Survey Charterhouse")));
    assert!(labels
        .iter()
        .any(|text| text.contains("Apply Survey Charterhouse")));
    assert!(labels.iter().any(|text| text.contains("Granite Pass")));
    assert!(
        !labels
            .iter()
            .any(|text| contains_raw_site_or_card_token(text)),
        "raw token leaked through action labels: {labels:?}"
    );
}

#[test]
fn action_affordance_templates_cover_actual_metadata_tags() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    state.deck.current = Some(event_frontier::CardId::LastLight);
    state.card_phase = event_frontier::CardPhase::AwaitingFirstChoice {
        faction: event_frontier::FactionId::Charter,
    };
    let view = project_view(&state, &viewer(None));
    let tree = legal_action_tree(&state, &actor("seat_0"));
    let operation = tree
        .root
        .choices
        .iter()
        .find(|choice| choice.segment == ACTION_OPERATION)
        .expect("operation root");
    let survey = operation
        .next
        .as_ref()
        .expect("operation kinds")
        .choices
        .iter()
        .find(|choice| choice.segment == "survey")
        .expect("survey kind");
    let survey_leaf = survey
        .next
        .as_ref()
        .expect("survey leaves")
        .choices
        .first()
        .expect("survey leaf");

    assert_eq!(
        metadata_value(survey, "cost_rule"),
        Some("base_one_resource_per_site")
    );
    assert_eq!(
        metadata_value(survey_leaf, "eligibility_consequence"),
        Some("acting_forfeits_next_card")
    );
    let templates = &view.ui.action_affordance_templates;
    assert!(templates.iter().any(|template| {
        template.id == "base_one_resource_per_site"
            && template.text.contains("one matching resource")
    }));
    assert!(templates.iter().any(|template| {
        template.id == "acting_forfeits_next_card"
            && template.text.contains("forfeits your eligibility")
    }));
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

fn contains_raw_site_or_card_token(text: &str) -> bool {
    text.split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
        .any(|token| token.starts_with("site_") || token.starts_with("ef_"))
}

fn action_labels(node: &ActionNode) -> Vec<String> {
    node.choices.iter().flat_map(choice_labels).collect()
}

fn choice_labels(choice: &ActionChoice) -> Vec<String> {
    let mut labels = vec![choice.label.clone(), choice.accessibility_label.clone()];
    if let Some(next) = &choice.next {
        labels.extend(action_labels(next));
    }
    labels
}

fn metadata_value<'a>(choice: &'a ActionChoice, key: &str) -> Option<&'a str> {
    choice
        .metadata
        .iter()
        .find(|entry| entry.key == key)
        .map(|entry| entry.value.as_str())
}
