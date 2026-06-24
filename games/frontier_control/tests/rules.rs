use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId};
use frontier_control::{
    apply_command, legal_action_metadata, legal_action_tree, load_highlands_fixture,
    load_standard_fixture, setup_match, FactionId, FrontierControlEffect, Phase, SetupOptions,
    SiteId, TerminalOutcome, VariantMap, ACTION_END_TURN, ACTION_MARCH, GAME_ID,
    RULES_VERSION_LABEL,
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
    state: &frontier_control::FrontierControlState,
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

#[derive(Debug, Eq, PartialEq)]
struct DomainEvidenceSummary {
    fixture_count: usize,
    standard_edge_count: usize,
    highlands_edge_count: usize,
    clash_site: SiteId,
    disconnected_stake_site: SiteId,
    terminal_winner: Option<FactionId>,
}

#[test]
fn setup_and_action_tree_are_faction_separated() {
    let state = setup_match(&seats(), &SetupOptions::default()).unwrap();
    assert_eq!(state.active_faction, FactionId::Prospectors);
    assert_eq!(state.round_number, 1);

    let tree = legal_action_tree(&state, &actor("seat_1"));
    let segments = tree
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.as_str())
        .collect::<Vec<_>>();
    assert!(segments.iter().any(|segment| segment.starts_with("march/")));
    assert!(segments.contains(&ACTION_END_TURN));
    assert!(!segments
        .iter()
        .any(|segment| segment.starts_with("patrol/")));

    assert!(legal_action_tree(&state, &actor("seat_0"))
        .root
        .choices
        .is_empty());
    assert!(legal_action_metadata(&state, &actor("seat_0"))
        .iter()
        .any(|entry| entry.key == "action_status" && entry.value == "waiting"));
}

#[test]
fn domain_evidence_v1_profile_driver_wraps_frontier_control_domain_validator() {
    let driver = DomainEvidenceV1Driver::new("frontier_control");
    let artifact = domain_evidence_profile_artifact(
        DOMAIN_EVIDENCE_V1,
        PROFILE_VERSION_V1,
        Some("public"),
        "frontier_control",
        &["domain_schema_version", "domain_input", "expected_domain"],
    );

    let report = driver
        .validate(&artifact)
        .expect("domain metadata validates");
    assert_eq!(report.profile_id, DOMAIN_EVIDENCE_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "public");
    assert_eq!(report.validator_owner, "frontier_control");
    assert_eq!(artifact.metadata.canonical_byte_authority, "none");
    assert!(!artifact.canonical_byte_claim);

    let summary = driver
        .validate_with(&artifact, |_| validate_frontier_control_domain_evidence())
        .expect("profile delegates to Frontier Control domain validator");
    assert_eq!(summary.fixture_count, 2);
    assert_eq!(summary.standard_edge_count, 10);
    assert_eq!(summary.highlands_edge_count, 10);
    assert_eq!(summary.clash_site, SiteId::Gatehouse);
    assert_eq!(summary.disconnected_stake_site, SiteId::Goldfield);
    assert_eq!(summary.terminal_winner, Some(FactionId::Garrison));

    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            SETUP_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("public"),
            "frontier_control",
            &["expected_domain"],
        ),
        ProfileValidationErrorKind::WrongProfileId,
    );
    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            DOMAIN_EVIDENCE_V1,
            "v2",
            Some("public"),
            "frontier_control",
            &["expected_domain"],
        ),
        ProfileValidationErrorKind::WrongProfileVersion,
    );
    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            DOMAIN_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("unsupported"),
            "frontier_control",
            &["expected_domain"],
        ),
        ProfileValidationErrorKind::InvalidVisibility,
    );
    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            DOMAIN_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("public"),
            "fixture-check",
            &["expected_domain"],
        ),
        ProfileValidationErrorKind::WrongValidatorOwner,
    );
    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            DOMAIN_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("public"),
            "frontier_control",
            &["expected_domain", "setup_options"],
        ),
        ProfileValidationErrorKind::UnknownField,
    );
}

#[test]
fn clash_scoring_and_terminal_are_rust_owned() {
    let mut state = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let first = command(
        &state,
        "seat_1",
        vec![ACTION_MARCH, "site_base_camp", "site_ford"],
    );
    apply_command(&mut state, &first).unwrap();
    let second = command(
        &state,
        "seat_1",
        vec![ACTION_MARCH, "site_ford", "site_gatehouse"],
    );
    let applied = apply_command(&mut state, &second).unwrap();

    assert!(applied.turn_ended);
    assert_eq!(state.site(SiteId::Gatehouse).unwrap().guards, 1);
    assert!(applied.effects.iter().any(|effect| {
        matches!(
            effect.payload,
            FrontierControlEffect::ClashResolved {
                entering_faction: FactionId::Prospectors,
                ..
            }
        )
    }));

    state.round_number = state.variant.round_count;
    state.active_faction = FactionId::Garrison;
    state.phase = Phase::Action {
        budget_remaining: state.variant.action_budget,
    };
    let end = command(&state, "seat_0", vec![ACTION_END_TURN]);
    let applied = apply_command(&mut state, &end).unwrap();
    assert!(applied.turn_ended);
    assert_eq!(state.phase, Phase::Terminal);
    assert!(state.terminal_outcome.is_some());
}

#[test]
fn diagnostics_are_viewer_safe() {
    let state = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let err = frontier_control::validate_command(
        &state,
        &command(
            &state,
            "seat_1",
            vec![ACTION_MARCH, "site_base_camp", "site_gatehouse"],
        ),
    )
    .unwrap_err();

    assert_eq!(err.code, "non_adjacent_sites");
    assert!(!err.message.contains("hidden"));
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
    let driver = DomainEvidenceV1Driver::new("frontier_control");
    assert_eq!(
        driver
            .validate(&artifact)
            .expect_err("invalid domain-evidence-v1 metadata rejects")
            .kind,
        expected
    );
}

fn validate_frontier_control_domain_evidence() -> DomainEvidenceSummary {
    let standard_fixture = load_standard_fixture().expect("standard fixture parses");
    let highlands_fixture = load_highlands_fixture().expect("highlands fixture parses");
    let fixtures = [
        (
            &standard_fixture,
            setup_match(&seats(), &SetupOptions::default()).expect("standard setup"),
        ),
        (
            &highlands_fixture,
            setup_match(
                &seats(),
                &SetupOptions {
                    variant: VariantMap::highlands(),
                },
            )
            .expect("highlands setup"),
        ),
    ];

    for (fixture, state) in &fixtures {
        assert_eq!(fixture.game_id, GAME_ID);
        assert_eq!(fixture.rules_version, RULES_VERSION_LABEL);
        assert_eq!(fixture.action_budget, state.variant.action_budget);
        assert_eq!(fixture.round_count, state.variant.round_count);
        assert_eq!(fixture.unit_cap_per_site, state.variant.unit_cap_per_site);
        assert_eq!(fixture.edges, state.variant.edges);
        assert_eq!(fixture.fort_sites, state.variant.fort_sites);
        assert_eq!(fixture.base_camp, state.variant.base_camp);
        assert_eq!(fixture.stake_values, state.variant.stake_values);
        assert_eq!(fixture.start_units, state.variant.start_units);
        assert_eq!(fixture.terminal_outcome, "score_compare_garrison_tiebreak");
        assert_eq!(state.active_faction, FactionId::Prospectors);
        assert_eq!(
            state.phase,
            Phase::Action {
                budget_remaining: state.variant.action_budget,
            }
        );
        assert_eq!(
            state
                .neighbors(SiteId::Gatehouse)
                .expect("gatehouse neighbors")
                .contains(&SiteId::Ford),
            true
        );
    }

    let mut clash = setup_match(&seats(), &SetupOptions::default()).expect("setup");
    let first = command(
        &clash,
        "seat_1",
        vec![ACTION_MARCH, "site_base_camp", "site_ford"],
    );
    apply_command(&mut clash, &first).expect("first march applies");
    let second = command(
        &clash,
        "seat_1",
        vec![ACTION_MARCH, "site_ford", "site_gatehouse"],
    );
    let applied = apply_command(&mut clash, &second).expect("clash march applies");
    assert!(applied.effects.iter().any(|effect| {
        effect.payload
            == FrontierControlEffect::ClashResolved {
                site: SiteId::Gatehouse,
                guard_removed: true,
                crew_removed: true,
                entering_faction: FactionId::Prospectors,
            }
    }));
    assert_eq!(clash.site(SiteId::Gatehouse).expect("gatehouse").guards, 1);
    assert_eq!(clash.site(SiteId::Ford).expect("ford").crews, 0);

    let invalid = frontier_control::validate_command(
        &setup_match(&seats(), &SetupOptions::default()).expect("setup"),
        &command(
            &setup_match(&seats(), &SetupOptions::default()).expect("setup"),
            "seat_1",
            vec![ACTION_MARCH, "site_base_camp", "site_gatehouse"],
        ),
    )
    .expect_err("non-adjacent movement rejects");
    assert_eq!(invalid.code, "non_adjacent_sites");

    let mut scoring = setup_match(&seats(), &SetupOptions::default()).expect("setup");
    scoring
        .site_mut(SiteId::Goldfield)
        .expect("goldfield")
        .stake = true;
    scoring.site_mut(SiteId::Ford).expect("ford").stake = true;
    scoring.site_mut(SiteId::Ford).expect("ford").guards = 0;
    scoring
        .site_mut(SiteId::Timberline)
        .expect("timberline")
        .guards = 1;
    scoring.round_number = scoring.variant.round_count;
    scoring.active_faction = FactionId::Garrison;
    scoring.phase = Phase::Action {
        budget_remaining: scoring.variant.action_budget,
    };
    let end = command(&scoring, "seat_0", vec![ACTION_END_TURN]);
    let applied = apply_command(&mut scoring, &end).expect("terminal scoring applies");
    assert!(applied.effects.iter().any(|effect| {
        matches!(
            &effect.payload,
            FrontierControlEffect::RoundScored {
                garrison_points,
                prospector_points,
                ..
            } if *garrison_points > 0 && *prospector_points > 0
        )
    }));
    assert_eq!(
        scoring
            .last_stake_supply
            .iter()
            .find(|entry| entry.site == SiteId::Goldfield)
            .map(|entry| entry.supplied),
        Some(false)
    );
    assert_eq!(scoring.phase, Phase::Terminal);
    assert!(matches!(
        scoring.terminal_outcome,
        Some(TerminalOutcome::Winner {
            faction: FactionId::Garrison,
            ..
        })
    ));

    DomainEvidenceSummary {
        fixture_count: fixtures.len(),
        standard_edge_count: standard_fixture.edges.len(),
        highlands_edge_count: highlands_fixture.edges.len(),
        clash_site: SiteId::Gatehouse,
        disconnected_stake_site: SiteId::Goldfield,
        terminal_winner: match scoring.terminal_outcome {
            Some(TerminalOutcome::Winner { faction, .. }) => Some(faction),
            None => None,
        },
    }
}
