use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};
use flood_watch::{
    apply_command, legal_action_metadata, legal_action_tree, load_deluge_fixture,
    load_standard_fixture, setup_match, DistrictId, EventCard, EventKind, FloodWatchEffect,
    FloodWatchRole, FloodWatchState, Phase, ScenarioVariant, SetupOptions, SharedOutcome,
    ACTION_END_TURN, ACTION_FORECAST, ACTION_REINFORCE, GAME_ID, RULES_VERSION_LABEL,
    STANDARD_ACTION_BUDGET, STANDARD_DECK_SIZE, STANDARD_DRAWS_PER_PHASE, STANDARD_LEVEE_CAP,
    STANDARD_MAX_FLOOD_LEVEL,
};
use game_test_support::profiles::{
    DomainEvidenceV1Driver, ProfileArtifact, ProfileMetadata, ProfileValidationErrorKind,
    DOMAIN_EVIDENCE_V1, PROFILE_VERSION_V1, SETUP_EVIDENCE_V1,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn command(
    state: &flood_watch::FloodWatchState,
    seat: &str,
    segments: Vec<&str>,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: segments.into_iter().map(str::to_owned).collect(),
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn card(kind: EventKind, copy_index: u8) -> EventCard {
    EventCard { kind, copy_index }
}

fn state_with_deck(deck: Vec<EventCard>) -> FloodWatchState {
    FloodWatchState::new_after_setup(ScenarioVariant::standard(), seats(), deck)
}

#[derive(Debug, Eq, PartialEq)]
struct DomainEvidenceSummary {
    fixture_count: usize,
    start_budget: u8,
    forecast_budget_after: u8,
    levee_absorbed: u8,
    inundated_district: Option<DistrictId>,
    win_outcome: Option<SharedOutcome>,
}

#[test]
fn standard_setup_initializes_bounded_state() {
    let state = setup_match(Seed(100), &seats(), &SetupOptions::default()).unwrap();

    assert_eq!(state.variant, ScenarioVariant::standard());
    assert_eq!(state.seats, seats());
    assert_eq!(
        state.roles,
        [FloodWatchRole::Pumpwright, FloodWatchRole::LeveeWarden]
    );
    assert_eq!(state.turn_number, 1);
    assert_eq!(state.active_seat, SeatId("seat_0".to_owned()));
    assert_eq!(
        state.phase,
        Phase::Action {
            budget_remaining: STANDARD_ACTION_BUDGET
        }
    );
    assert_eq!(state.freshness_token, FreshnessToken(0));
    assert_eq!(state.districts.len(), DistrictId::ALL.len());
    assert_eq!(
        state
            .districts
            .iter()
            .map(|district| district.flood_level)
            .collect::<Vec<_>>(),
        vec![0, 1, 0, 1, 0]
    );
    assert!(state
        .districts
        .iter()
        .all(|district| district.flood_level <= STANDARD_MAX_FLOOD_LEVEL));
    assert!(state
        .districts
        .iter()
        .all(|district| district.levees <= STANDARD_LEVEE_CAP));
    assert_eq!(state.undrawn_deck_len(), STANDARD_DECK_SIZE as usize);
    assert_eq!(
        state.remaining_composition().total_cards(),
        STANDARD_DECK_SIZE
    );
    assert!(state.drawn.is_empty());
    assert!(state.forecast.is_none());
    assert!(state.terminal_outcome.is_none());
}

#[test]
fn deluge_setup_initializes_scenario_constants() {
    let options = SetupOptions {
        variant: ScenarioVariant::deluge(),
    };
    let state = setup_match(Seed(100), &seats(), &options).unwrap();

    assert_eq!(state.variant, ScenarioVariant::deluge());
    assert_eq!(
        state.phase,
        Phase::Action {
            budget_remaining: STANDARD_ACTION_BUDGET
        }
    );
    assert_eq!(state.variant.draws_per_phase, STANDARD_DRAWS_PER_PHASE);
    assert_eq!(state.variant.levee_cap, STANDARD_LEVEE_CAP);
    assert_eq!(
        state
            .districts
            .iter()
            .map(|district| district.flood_level)
            .collect::<Vec<_>>(),
        vec![1, 1, 1, 2, 1]
    );
    assert_eq!(
        state.undrawn_deck_len(),
        state.variant.event_composition.total_cards() as usize
    );
    assert_eq!(state.variant.event_composition.total_cards(), 27);
}

#[test]
fn domain_evidence_v1_profile_driver_wraps_flood_watch_domain_validator() {
    let driver = DomainEvidenceV1Driver::new("flood_watch");
    let artifact = domain_evidence_profile_artifact(
        DOMAIN_EVIDENCE_V1,
        PROFILE_VERSION_V1,
        Some("internal-dev"),
        "flood_watch",
        &["domain_schema_version", "domain_input", "expected_domain"],
    );

    let report = driver
        .validate(&artifact)
        .expect("domain metadata validates");
    assert_eq!(report.profile_id, DOMAIN_EVIDENCE_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "internal-dev");
    assert_eq!(report.validator_owner, "flood_watch");
    assert_eq!(artifact.metadata.canonical_byte_authority, "none");
    assert!(!artifact.canonical_byte_claim);

    let summary = driver
        .validate_with(&artifact, |_| validate_flood_watch_domain_evidence())
        .expect("profile delegates to Flood Watch domain validator");
    assert_eq!(summary.fixture_count, 2);
    assert_eq!(summary.start_budget, STANDARD_ACTION_BUDGET);
    assert_eq!(summary.forecast_budget_after, 2);
    assert_eq!(summary.levee_absorbed, 1);
    assert_eq!(summary.inundated_district, Some(DistrictId::Terraces));
    assert_eq!(summary.win_outcome, Some(SharedOutcome::Won));

    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            SETUP_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("internal-dev"),
            "flood_watch",
            &["expected_domain"],
        ),
        ProfileValidationErrorKind::WrongProfileId,
    );
    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            DOMAIN_EVIDENCE_V1,
            "v2",
            Some("internal-dev"),
            "flood_watch",
            &["expected_domain"],
        ),
        ProfileValidationErrorKind::WrongProfileVersion,
    );
    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            DOMAIN_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("unsupported"),
            "flood_watch",
            &["expected_domain"],
        ),
        ProfileValidationErrorKind::InvalidVisibility,
    );
    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            DOMAIN_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("internal-dev"),
            "fixture-check",
            &["expected_domain"],
        ),
        ProfileValidationErrorKind::WrongValidatorOwner,
    );
    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            DOMAIN_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("internal-dev"),
            "flood_watch",
            &["expected_domain", "setup_options"],
        ),
        ProfileValidationErrorKind::UnknownField,
    );
}

#[test]
fn setup_rejects_wrong_seat_count() {
    let err = setup_match(
        Seed(1),
        &[SeatId("seat_0".to_owned())],
        &SetupOptions::default(),
    )
    .unwrap_err();

    assert_eq!(err.code, "invalid_seat_count");
}

#[test]
fn active_tree_contains_legal_budgeted_choices_and_teammate_waits() {
    let state = setup_match(Seed(7), &seats(), &SetupOptions::default()).unwrap();
    let tree = legal_action_tree(&state, &actor("seat_0"));
    let segments = tree
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.as_str())
        .collect::<Vec<_>>();

    assert!(segments.contains(&"bail/district_old_docks"));
    assert!(segments.contains(&"bail/district_terraces"));
    assert!(!segments.contains(&"bail/district_riverside"));
    assert!(segments.contains(&"reinforce/district_riverside"));
    assert!(segments.contains(&ACTION_FORECAST));
    assert!(segments.contains(&ACTION_END_TURN));

    let rendered = format!("{:?}", tree);
    assert!(rendered.contains("remaining_budget"));
    assert!(rendered.contains("role_amount"));
    assert!(!rendered.contains("storm_surge/"));
    assert!(!rendered.contains("downpour/"));

    assert!(legal_action_tree(&state, &actor("seat_1"))
        .root
        .choices
        .is_empty());
    assert!(legal_action_metadata(&state, &actor("seat_1"))
        .iter()
        .any(|entry| entry.key == "action_status" && entry.value == "waiting"));
}

#[test]
fn budget_decrements_and_role_powers_apply_in_rust() {
    let mut state = setup_match(Seed(9), &seats(), &SetupOptions::default()).unwrap();

    let cmd = command(&state, "seat_0", vec!["bail", "district_old_docks"]);
    let applied = apply_command(&mut state, &cmd).unwrap();
    assert!(!applied.environment_pending);
    assert_eq!(state.district(DistrictId::OldDocks).unwrap().flood_level, 0);
    assert_eq!(
        state.phase,
        Phase::Action {
            budget_remaining: 2
        }
    );

    state.active_seat = SeatId("seat_1".to_owned());
    state.phase = Phase::Action {
        budget_remaining: 3,
    };
    let cmd = command(
        &state,
        "seat_1",
        vec![ACTION_REINFORCE, "district_riverside"],
    );
    let applied = apply_command(&mut state, &cmd).unwrap();
    assert!(!applied.environment_pending);
    assert_eq!(state.district(DistrictId::Riverside).unwrap().levees, 2);
    assert_eq!(
        state.phase,
        Phase::Action {
            budget_remaining: 2
        }
    );
}

#[test]
fn forecast_reveals_top_card_without_drawing_and_then_becomes_unavailable() {
    let mut state = setup_match(Seed(11), &seats(), &SetupOptions::default()).unwrap();
    let top = state.top_undrawn_card().cloned().unwrap();

    let cmd = command(&state, "seat_0", vec![ACTION_FORECAST]);
    let applied = apply_command(&mut state, &cmd).unwrap();
    assert!(!applied.environment_pending);
    assert_eq!(state.forecast, Some(top));
    assert_eq!(state.undrawn_deck_len(), STANDARD_DECK_SIZE as usize);
    assert_eq!(
        state.phase,
        Phase::Action {
            budget_remaining: 2
        }
    );

    let cmd = command(&state, "seat_0", vec![ACTION_FORECAST]);
    let err = apply_command(&mut state, &cmd).unwrap_err();
    assert_eq!(err.code, "forecast_unavailable");
    assert!(!err.message.contains("storm_surge/"));
    assert!(!err.message.contains("downpour/"));
}

#[test]
fn final_budget_action_and_end_turn_resolve_environment_atomically() {
    let mut state = state_with_deck(vec![
        card(EventKind::Reprieve, 1),
        card(EventKind::Reprieve, 2),
        card(EventKind::Reprieve, 3),
    ]);
    state.phase = Phase::Action {
        budget_remaining: 1,
    };

    let cmd = command(&state, "seat_0", vec!["reinforce", "district_market"]);
    let applied = apply_command(&mut state, &cmd).unwrap();
    assert!(!applied.environment_pending);
    assert!(matches!(
        applied.effects.first().map(|effect| &effect.payload),
        Some(FloodWatchEffect::LeveePlaced {
            district: DistrictId::Market,
            amount: 1
        })
    ));
    assert_eq!(
        applied.effects.get(1).map(|effect| &effect.payload),
        Some(&FloodWatchEffect::EnvironmentPhaseBegan { turn: 1, draws: 2 })
    );
    assert_eq!(
        state.phase,
        Phase::Action {
            budget_remaining: STANDARD_ACTION_BUDGET
        }
    );
    assert_eq!(state.active_seat, SeatId("seat_1".to_owned()));

    let mut state = state_with_deck(vec![
        card(EventKind::Reprieve, 1),
        card(EventKind::Reprieve, 2),
        card(EventKind::Reprieve, 3),
    ]);
    let cmd = command(&state, "seat_0", vec![ACTION_END_TURN]);
    let applied = apply_command(&mut state, &cmd).unwrap();
    assert!(!applied.environment_pending);
    assert_eq!(
        applied.effects.first().map(|effect| &effect.payload),
        Some(&FloodWatchEffect::EnvironmentPhaseBegan { turn: 1, draws: 2 })
    );
    assert_eq!(
        state.phase,
        Phase::Action {
            budget_remaining: STANDARD_ACTION_BUDGET
        }
    );
    assert_eq!(state.active_seat, SeatId("seat_1".to_owned()));
}

#[test]
fn diagnostics_are_fail_closed_and_deck_safe() {
    let mut state = setup_match(Seed(5), &seats(), &SetupOptions::default()).unwrap();

    let mut stale = command(&state, "seat_0", vec![ACTION_END_TURN]);
    stale.freshness_token = FreshnessToken(99);
    assert_eq!(
        apply_command(&mut state.clone(), &stale).unwrap_err().code,
        "stale_action"
    );
    assert_eq!(
        apply_command(
            &mut state.clone(),
            &command(&state, "seat_1", vec![ACTION_END_TURN])
        )
        .unwrap_err()
        .code,
        "wrong_seat"
    );
    assert_eq!(
        apply_command(
            &mut state.clone(),
            &command(&state, "seat_x", vec![ACTION_END_TURN])
        )
        .unwrap_err()
        .code,
        "wrong_actor"
    );
    assert_eq!(
        apply_command(
            &mut state.clone(),
            &command(&state, "seat_0", vec!["bail", "district_riverside"]),
        )
        .unwrap_err()
        .code,
        "dry_bail"
    );

    state
        .district_mut(DistrictId::Riverside)
        .expect("district")
        .levees = STANDARD_LEVEE_CAP;
    let cap = apply_command(
        &mut state.clone(),
        &command(&state, "seat_0", vec!["reinforce", "district_riverside"]),
    )
    .unwrap_err();
    assert_eq!(cap.code, "levee_cap");
    assert!(!format!("{:?}", cap).contains("storm_surge/"));
    assert!(!format!("{:?}", cap).contains("downpour/"));

    let mut empty_budget = state.clone();
    empty_budget.phase = Phase::Action {
        budget_remaining: 0,
    };
    assert_eq!(
        apply_command(
            &mut empty_budget,
            &command(&state, "seat_0", vec![ACTION_END_TURN]),
        )
        .unwrap_err()
        .code,
        "out_of_budget"
    );
}

#[test]
fn environment_absorbs_levees_before_flood_rises() {
    let mut state = state_with_deck(vec![
        card(
            EventKind::StormSurge {
                district: DistrictId::Riverside,
            },
            1,
        ),
        card(EventKind::Reprieve, 1),
        card(EventKind::Reprieve, 2),
    ]);
    state.district_mut(DistrictId::Riverside).unwrap().levees = 1;

    let cmd = command(&state, "seat_0", vec![ACTION_END_TURN]);
    let applied = apply_command(&mut state, &cmd).unwrap();
    let payloads = applied
        .effects
        .iter()
        .map(|effect| &effect.payload)
        .collect::<Vec<_>>();

    assert_eq!(
        payloads[..5],
        [
            &FloodWatchEffect::EnvironmentPhaseBegan { turn: 1, draws: 2 },
            &FloodWatchEffect::EventDrawn {
                index: 1,
                card: EventKind::StormSurge {
                    district: DistrictId::Riverside
                }
            },
            &FloodWatchEffect::LeveeAbsorbed {
                district: DistrictId::Riverside,
                amount: 1,
                remaining_levees: 0
            },
            &FloodWatchEffect::FloodLevelRose {
                district: DistrictId::Riverside,
                amount: 1,
                new_level: 1
            },
            &FloodWatchEffect::EventDrawn {
                index: 2,
                card: EventKind::Reprieve
            },
        ]
    );
    let riverside = state.district(DistrictId::Riverside).unwrap();
    assert_eq!(riverside.levees, 0);
    assert_eq!(riverside.flood_level, 1);
}

#[test]
fn storm_surge_can_raise_two_levels_without_levees() {
    let mut state = state_with_deck(vec![
        card(
            EventKind::StormSurge {
                district: DistrictId::Market,
            },
            1,
        ),
        card(EventKind::Reprieve, 1),
        card(EventKind::Reprieve, 2),
    ]);

    let cmd = command(&state, "seat_0", vec![ACTION_END_TURN]);
    let applied = apply_command(&mut state, &cmd).unwrap();

    assert!(applied.effects.iter().any(|effect| {
        effect.payload
            == FloodWatchEffect::FloodLevelRose {
                district: DistrictId::Market,
                amount: 2,
                new_level: 2,
            }
    }));
    assert_eq!(state.district(DistrictId::Market).unwrap().flood_level, 2);
}

#[test]
fn reprieve_draws_without_changing_districts() {
    let mut state = state_with_deck(vec![
        card(EventKind::Reprieve, 1),
        card(EventKind::Reprieve, 2),
        card(
            EventKind::Downpour {
                district: DistrictId::Gardens,
            },
            1,
        ),
    ]);
    let before = state.districts.clone();

    let cmd = command(&state, "seat_0", vec![ACTION_END_TURN]);
    let applied = apply_command(&mut state, &cmd).unwrap();

    assert_eq!(state.districts, before);
    assert_eq!(state.drawn.len(), 2);
    assert!(applied
        .effects
        .iter()
        .all(|effect| !matches!(effect.payload, FloodWatchEffect::LeveeAbsorbed { .. })));
    assert!(applied
        .effects
        .iter()
        .all(|effect| !matches!(effect.payload, FloodWatchEffect::FloodLevelRose { .. })));
}

#[test]
fn inundation_stops_environment_before_remaining_draws() {
    let mut state = state_with_deck(vec![
        card(
            EventKind::StormSurge {
                district: DistrictId::Terraces,
            },
            1,
        ),
        card(
            EventKind::Downpour {
                district: DistrictId::Market,
            },
            1,
        ),
        card(EventKind::Reprieve, 1),
    ]);

    let cmd = command(&state, "seat_0", vec![ACTION_END_TURN]);
    let applied = apply_command(&mut state, &cmd).unwrap();

    assert_eq!(state.drawn.len(), 1);
    assert_eq!(state.undrawn_deck_len(), 2);
    assert!(applied.effects.iter().any(|effect| {
        effect.payload
            == FloodWatchEffect::DistrictInundated {
                district: DistrictId::Terraces,
            }
    }));
    assert_eq!(
        applied
            .effects
            .iter()
            .filter(|effect| matches!(effect.payload, FloodWatchEffect::EventDrawn { .. }))
            .count(),
        1
    );
    assert_eq!(state.phase, Phase::Terminal);
    assert_eq!(
        state.terminal_outcome,
        Some(SharedOutcome::Lost {
            district: DistrictId::Terraces
        })
    );
}

#[test]
fn deck_exhaustion_effect_emits_without_undrawn_order_leak() {
    let mut state = state_with_deck(vec![
        card(EventKind::Reprieve, 1),
        card(
            EventKind::Downpour {
                district: DistrictId::Market,
            },
            1,
        ),
        card(
            EventKind::StormSurge {
                district: DistrictId::Gardens,
            },
            1,
        ),
    ]);

    let cmd = command(&state, "seat_0", vec![ACTION_END_TURN]);
    let applied = apply_command(&mut state, &cmd).unwrap();
    let rendered = format!("{:?}", applied.effects);

    assert!(rendered.contains("downpour/district_market") || rendered.contains("Market"));
    assert!(!rendered.contains("storm_surge/district_gardens"));
    assert!(!applied
        .effects
        .iter()
        .any(|effect| matches!(effect.payload, FloodWatchEffect::DeckExhausted)));

    let mut exhausted = state_with_deck(vec![card(EventKind::Reprieve, 1)]);
    let cmd = command(&exhausted, "seat_0", vec![ACTION_END_TURN]);
    let applied = apply_command(&mut exhausted, &cmd).unwrap();
    assert!(applied
        .effects
        .iter()
        .any(|effect| matches!(effect.payload, FloodWatchEffect::DeckExhausted)));
    assert!(applied.effects.iter().any(|effect| {
        matches!(
            &effect.payload,
            FloodWatchEffect::Terminal { outcome, summary }
                if outcome == "won" && summary.rule_id == "FW-END-002"
        )
    }));
    assert_eq!(exhausted.phase, Phase::Terminal);
    assert_eq!(exhausted.terminal_outcome, Some(SharedOutcome::Won));
}

#[test]
fn terminal_loss_effect_is_shared_and_public_safe() {
    let mut state = state_with_deck(vec![
        card(
            EventKind::StormSurge {
                district: DistrictId::OldDocks,
            },
            1,
        ),
        card(
            EventKind::Downpour {
                district: DistrictId::Gardens,
            },
            1,
        ),
    ]);

    let cmd = command(&state, "seat_0", vec![ACTION_END_TURN]);
    let applied = apply_command(&mut state, &cmd).unwrap();
    let terminal = applied
        .effects
        .iter()
        .find_map(|effect| match &effect.payload {
            FloodWatchEffect::Terminal { outcome, summary } => Some((outcome, summary)),
            _ => None,
        })
        .expect("terminal effect");

    assert_eq!(state.phase, Phase::Terminal);
    assert_eq!(
        state.terminal_outcome,
        Some(SharedOutcome::Lost {
            district: DistrictId::OldDocks
        })
    );
    assert_eq!(terminal.0, "lost:district_old_docks");
    assert_eq!(terminal.1.rule_id, "FW-END-001");
    assert_eq!(terminal.1.drawn_card_count, 1);
    assert_eq!(terminal.1.surviving_levels.len(), DistrictId::ALL.len());
    assert!(!format!("{:?}", terminal).contains("downpour/district_gardens"));

    assert!(legal_action_tree(&state, &actor("seat_0"))
        .root
        .choices
        .is_empty());
    let post_terminal = command(&state, "seat_0", vec![ACTION_END_TURN]);
    let err = apply_command(&mut state, &post_terminal).unwrap_err();
    assert_eq!(err.code, "terminal_state");
}

#[test]
fn terminal_win_effect_has_no_per_seat_winner() {
    let mut state = state_with_deck(vec![card(EventKind::Reprieve, 1)]);

    let cmd = command(&state, "seat_0", vec![ACTION_END_TURN]);
    let applied = apply_command(&mut state, &cmd).unwrap();
    let rendered = format!("{:?}", applied.effects);

    assert_eq!(state.phase, Phase::Terminal);
    assert_eq!(state.terminal_outcome, Some(SharedOutcome::Won));
    assert!(rendered.contains("FW-END-002"));
    assert!(rendered.contains("won"));
    assert!(!rendered.contains("winner"));
    assert!(!rendered.contains("seat_0"));
    assert!(!rendered.contains("seat_1"));
}

fn domain_evidence_profile_artifact<'a>(
    profile_id: &'a str,
    profile_version: &'a str,
    visibility_class: Option<&'a str>,
    validator_owner: &'a str,
    fields: &'a [&'a str],
) -> ProfileArtifact<'a> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id,
            profile_version,
            visibility_class,
            validator_owner,
            canonical_byte_authority: "none",
            migration_update_note: Some("profile migration reviewed"),
        },
        fields,
        canonical_byte_claim: false,
    }
}

fn assert_domain_profile_rejects(
    artifact: ProfileArtifact<'_>,
    expected: ProfileValidationErrorKind,
) {
    let driver = DomainEvidenceV1Driver::new("flood_watch");
    assert_eq!(
        driver
            .validate(&artifact)
            .expect_err("invalid domain-evidence-v1 metadata rejects")
            .kind,
        expected
    );
}

fn validate_flood_watch_domain_evidence() -> DomainEvidenceSummary {
    let fixtures = [
        load_standard_fixture().expect("standard fixture parses"),
        load_deluge_fixture().expect("deluge fixture parses"),
    ];

    for fixture in &fixtures {
        let variant = ScenarioVariant::resolve(&fixture.variant).expect("fixture variant resolves");
        let state = setup_match(
            Seed(11),
            &seats(),
            &SetupOptions {
                variant: variant.clone(),
            },
        )
        .expect("fixture setup succeeds");

        assert_eq!(fixture.game_id, GAME_ID);
        assert_eq!(fixture.rules_version, RULES_VERSION_LABEL);
        assert_eq!(fixture.action_budget, variant.action_budget);
        assert_eq!(fixture.draws_per_phase, variant.draws_per_phase);
        assert_eq!(fixture.levee_cap, variant.levee_cap);
        assert_eq!(fixture.max_flood_level, variant.max_flood_level);
        assert_eq!(fixture.starting_levels, variant.starting_levels);
        assert_eq!(fixture.event_composition, variant.event_composition);
        assert_eq!(fixture.terminal_outcome, "none");
        assert_eq!(state.roles, variant.role_order);
        assert_eq!(
            state.phase,
            Phase::Action {
                budget_remaining: variant.action_budget
            }
        );
    }

    let mut forecast = setup_match(Seed(11), &seats(), &SetupOptions::default()).unwrap();
    let top = forecast.top_undrawn_card().cloned().unwrap();
    let cmd = command(&forecast, "seat_0", vec![ACTION_FORECAST]);
    apply_command(&mut forecast, &cmd).expect("forecast applies");
    assert_eq!(forecast.forecast, Some(top));
    assert_eq!(forecast.undrawn_deck_len(), STANDARD_DECK_SIZE as usize);
    assert_eq!(
        forecast.phase,
        Phase::Action {
            budget_remaining: 2
        }
    );

    let mut role_state = setup_match(Seed(9), &seats(), &SetupOptions::default()).unwrap();
    let cmd = command(&role_state, "seat_0", vec!["bail", "district_old_docks"]);
    apply_command(&mut role_state, &cmd).expect("bail applies");
    assert_eq!(
        role_state
            .district(DistrictId::OldDocks)
            .unwrap()
            .flood_level,
        0
    );
    role_state.active_seat = SeatId("seat_1".to_owned());
    role_state.phase = Phase::Action {
        budget_remaining: STANDARD_ACTION_BUDGET,
    };
    let cmd = command(
        &role_state,
        "seat_1",
        vec![ACTION_REINFORCE, "district_riverside"],
    );
    apply_command(&mut role_state, &cmd).expect("reinforce applies");
    assert_eq!(
        role_state.district(DistrictId::Riverside).unwrap().levees,
        2
    );

    let mut levee = state_with_deck(vec![
        card(
            EventKind::StormSurge {
                district: DistrictId::Riverside,
            },
            1,
        ),
        card(EventKind::Reprieve, 1),
        card(EventKind::Reprieve, 2),
    ]);
    levee
        .district_mut(DistrictId::Riverside)
        .expect("riverside district")
        .levees = 1;
    let cmd = command(&levee, "seat_0", vec![ACTION_END_TURN]);
    let applied = apply_command(&mut levee, &cmd).expect("levee environment applies");
    assert!(applied.effects.iter().any(|effect| {
        effect.payload
            == FloodWatchEffect::LeveeAbsorbed {
                district: DistrictId::Riverside,
                amount: 1,
                remaining_levees: 0,
            }
    }));
    assert_eq!(levee.district(DistrictId::Riverside).unwrap().levees, 0);
    assert_eq!(
        levee.district(DistrictId::Riverside).unwrap().flood_level,
        1
    );

    let mut inundation = state_with_deck(vec![
        card(
            EventKind::StormSurge {
                district: DistrictId::Terraces,
            },
            1,
        ),
        card(
            EventKind::Downpour {
                district: DistrictId::Market,
            },
            1,
        ),
        card(EventKind::Reprieve, 1),
    ]);
    let cmd = command(&inundation, "seat_0", vec![ACTION_END_TURN]);
    apply_command(&mut inundation, &cmd).expect("inundation applies");
    assert_eq!(inundation.drawn.len(), 1);
    assert_eq!(
        inundation.terminal_outcome,
        Some(SharedOutcome::Lost {
            district: DistrictId::Terraces,
        })
    );

    let mut win = state_with_deck(vec![card(EventKind::Reprieve, 1)]);
    let cmd = command(&win, "seat_0", vec![ACTION_END_TURN]);
    apply_command(&mut win, &cmd).expect("win applies");
    assert_eq!(win.terminal_outcome, Some(SharedOutcome::Won));

    DomainEvidenceSummary {
        fixture_count: fixtures.len(),
        start_budget: STANDARD_ACTION_BUDGET,
        forecast_budget_after: match forecast.phase {
            Phase::Action { budget_remaining } => budget_remaining,
            Phase::Terminal => 0,
        },
        levee_absorbed: 1,
        inundated_district: match inundation.terminal_outcome {
            Some(SharedOutcome::Lost { district }) => Some(district),
            Some(SharedOutcome::Won) | None => None,
        },
        win_outcome: win.terminal_outcome,
    }
}
