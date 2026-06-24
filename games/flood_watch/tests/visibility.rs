use engine_core::{
    ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, StableSerialize, Viewer,
};
use flood_watch::{
    action_tree_hash, apply_command, contains_hidden_event_identity, diagnostic_hash, effect_hash,
    filter_effects_for_viewer, legal_action_tree, project_view, public_effect_text, setup_match,
    view_hash, EventCard, EventKind, FloodWatchEffect, FloodWatchState, ScenarioVariant,
    SetupOptions, ACTION_END_TURN,
};
use game_test_support::no_leak::{assert_pairwise_no_leak, ExposureExpectation, LeakProbe};

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

fn end_turn_command(state: &flood_watch::FloodWatchState) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor("seat_0"),
        action_path: ActionPath {
            segments: vec![ACTION_END_TURN.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn card(kind: EventKind, copy_index: u8) -> EventCard {
    EventCard { kind, copy_index }
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
    StaleDiagnostic,
    Effects,
}

#[test]
fn pairwise_hidden_future_deck_matrix_covers_public_surfaces() {
    let state = setup_match(Seed(11), &seats(), &SetupOptions::default()).unwrap();
    let probes = hidden_future_probes(&state);

    assert_pairwise_no_leak(
        [
            MatrixViewer::Observer,
            MatrixViewer::Seat0,
            MatrixViewer::Seat1,
        ],
        [
            MatrixSurface::View,
            MatrixSurface::ActionTree,
            MatrixSurface::StaleDiagnostic,
            MatrixSurface::Effects,
        ],
        probes,
        |viewer, surface| matrix_snapshot(&state, *viewer, *surface),
        |_source, _viewer, _surface, _canary_id| ExposureExpectation::MustBeAbsent,
        |snapshot, card| snapshot_contains_event_card(snapshot, card),
    )
    .expect("Flood Watch hidden-future matrix has no failures");
}

fn hidden_future_probes(state: &FloodWatchState) -> Vec<LeakProbe<usize, String, EventCard>> {
    state
        .event_deck_internal()
        .iter()
        .enumerate()
        .skip(1)
        .map(|(index, card)| LeakProbe {
            source_seat: index,
            canary_id: card.stable_id(),
            canary: card.clone(),
        })
        .collect()
}

fn matrix_snapshot(
    state: &FloodWatchState,
    viewer_case: MatrixViewer,
    surface: MatrixSurface,
) -> String {
    match surface {
        MatrixSurface::View => format!("{:?}", project_view(state, &viewer_case.viewer())),
        MatrixSurface::ActionTree => {
            format!("{:?}", legal_action_tree(state, &viewer_case.actor()))
        }
        MatrixSurface::StaleDiagnostic => {
            let mut stale = end_turn_command(state);
            stale.freshness_token = engine_core::FreshnessToken(99);
            format!(
                "{:?}",
                apply_command(&mut state.clone(), &stale).expect_err("stale command rejects")
            )
        }
        MatrixSurface::Effects => {
            format!(
                "{:?}",
                filter_effects_for_viewer(&[], &viewer_case.viewer())
            )
        }
    }
}

fn snapshot_contains_event_card(snapshot: &str, card: &EventCard) -> bool {
    snapshot.contains(&card.stable_id())
        || (!matches!(card.kind, EventKind::Reprieve) && snapshot.contains(&card.kind.id()))
        || snapshot.contains(&format!("{:?}", card.kind))
}

#[test]
fn all_viewers_receive_identical_public_projection() {
    let state = setup_match(Seed(11), &seats(), &SetupOptions::default()).unwrap();

    let observer = project_view(&state, &viewer(None));
    let seat_0 = project_view(&state, &viewer(Some("seat_0")));
    let seat_1 = project_view(&state, &viewer(Some("seat_1")));

    assert_eq!(observer, seat_0);
    assert_eq!(seat_0, seat_1);
    assert_eq!(observer.undrawn_count, state.undrawn_deck_len() as u8);
    assert_eq!(
        observer.remaining_composition.reprieves,
        state.remaining_composition().reprieves
    );
    assert_eq!(observer.ui.event_deck_label, "Storm deck");
}

#[test]
fn public_projection_action_tree_and_diagnostics_do_not_leak_undrawn_order() {
    let state = setup_match(Seed(11), &seats(), &SetupOptions::default()).unwrap();
    let view = project_view(&state, &viewer(None));
    let tree = legal_action_tree(&state, &actor("seat_0"));
    let mut stale = end_turn_command(&state);
    stale.freshness_token = engine_core::FreshnessToken(99);
    let diagnostic = apply_command(&mut state.clone(), &stale).unwrap_err();

    assert!(!contains_hidden_event_identity(&state, &view));
    assert!(!contains_hidden_event_identity(&state, &tree));
    assert!(!contains_hidden_event_identity(&state, &diagnostic));
    assert_ne!(view_hash(&view), diagnostic_hash(&diagnostic));
    assert_ne!(action_tree_hash(&tree), diagnostic_hash(&diagnostic));
}

#[test]
fn card_identity_first_appears_in_forecast_or_draw_effects() {
    let mut state = FloodWatchState::new_after_setup(
        ScenarioVariant::standard(),
        seats(),
        vec![
            card(EventKind::Reprieve, 1),
            card(
                EventKind::Downpour {
                    district: flood_watch::DistrictId::Market,
                },
                1,
            ),
            card(
                EventKind::StormSurge {
                    district: flood_watch::DistrictId::Gardens,
                },
                1,
            ),
        ],
    );
    let before = project_view(&state, &viewer(None));
    assert!(!format!("{before:?}").contains("downpour/district_market"));

    let cmd = end_turn_command(&state);
    let applied = apply_command(&mut state, &cmd).unwrap();
    let texts = applied
        .effects
        .iter()
        .map(|effect| public_effect_text(&effect.payload))
        .collect::<Vec<_>>();

    assert!(texts.iter().any(|text| text.contains("Reprieve")));
    assert!(texts.iter().any(|text| text.contains("Downpour at Market")));
    assert!(!texts
        .iter()
        .any(|text| text.contains("Storm Surge at Gardens")));
    assert!(!applied.effects.iter().any(|effect| {
        matches!(
            effect.payload,
            FloodWatchEffect::EventDrawn {
                card: EventKind::StormSurge {
                    district: flood_watch::DistrictId::Gardens
                },
                ..
            }
        )
    }));
}

#[test]
fn public_effect_filtering_is_identical_for_all_viewers() {
    let mut state = setup_match(Seed(91), &seats(), &SetupOptions::default()).unwrap();
    let cmd = end_turn_command(&state);
    let applied = apply_command(&mut state, &cmd).unwrap();

    let observer = filter_effects_for_viewer(&applied.effects, &viewer(None));
    let seat_0 = filter_effects_for_viewer(&applied.effects, &viewer(Some("seat_0")));
    let seat_1 = filter_effects_for_viewer(&applied.effects, &viewer(Some("seat_1")));

    assert_eq!(observer, applied.effects);
    assert_eq!(observer, seat_0);
    assert_eq!(seat_0, seat_1);
}

#[test]
fn action_effect_and_view_hashes_are_deterministic() {
    let mut first = setup_match(Seed(91), &seats(), &SetupOptions::default()).unwrap();
    let mut second = setup_match(Seed(91), &seats(), &SetupOptions::default()).unwrap();
    let first_tree = legal_action_tree(&first, &actor("seat_0"));
    let second_tree = legal_action_tree(&second, &actor("seat_0"));

    assert_eq!(
        action_tree_hash(&first_tree),
        action_tree_hash(&second_tree)
    );
    assert_eq!(
        project_view(&first, &viewer(None)).stable_hash(),
        project_view(&second, &viewer(None)).stable_hash()
    );

    let first_command = end_turn_command(&first);
    let second_command = end_turn_command(&second);
    let first_effects = apply_command(&mut first, &first_command).unwrap().effects;
    let second_effects = apply_command(&mut second, &second_command).unwrap().effects;

    assert_eq!(
        first_effects.iter().map(effect_hash).collect::<Vec<_>>(),
        second_effects.iter().map(effect_hash).collect::<Vec<_>>()
    );
    assert_eq!(
        view_hash(&project_view(&first, &viewer(None))),
        view_hash(&project_view(&second, &viewer(None)))
    );
}
