use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};
use event_frontier::{
    actions::{
        choosing_menu, ACTION_EVENT, ACTION_LIMITED_OPERATION, ACTION_OPERATION, ACTION_PASS,
    },
    apply_command,
    cards::{expire_all_edicts, resolve_event_card, EdictKind},
    legal_action_metadata, legal_action_tree,
    rules::advance_to_next_card,
    setup_match, validate_command, CardId, CardPhase, Eligibility, EventFrontierEffect,
    EventFrontierState, FactionId, FirstChoice, ScenarioVariant, SetupOptions, SiteId, GAME_ID,
    RULES_VERSION_LABEL,
};
use event_frontier::{resolve_reckoning, FactionScores, TerminalOutcome, VictoryType};
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

fn command(seat: &str, segment: &str, freshness_token: FreshnessToken) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn segments_for(seat: &str, state: &event_frontier::EventFrontierState) -> Vec<String> {
    legal_action_tree(state, &actor(seat))
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.clone())
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
struct DomainEvidenceSummary {
    fixture_count: usize,
    fixture_edge_count: usize,
    ordinary_event_count: usize,
    edict_count: usize,
    survey_funds_after: u8,
    reckoning_income: (u8, u8),
    terminal_winner: Option<FactionId>,
}

#[test]
fn first_event_constrains_second_to_operation_or_pass() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    let freshness = state.freshness_token;

    apply_command(&mut state, &command("seat_1", ACTION_EVENT, freshness)).expect("event");

    assert_eq!(
        segments_for("seat_0", &state),
        vec![ACTION_OPERATION.to_owned(), ACTION_PASS.to_owned()]
    );
    assert!(segments_for("seat_1", &state).is_empty());
}

#[test]
fn first_operation_constrains_second_to_event_limited_operation_or_pass() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    let freshness = state.freshness_token;

    apply_command(
        &mut state,
        &command("seat_1", "operation/cache/site_landing", freshness),
    )
    .expect("operation");

    assert_eq!(
        segments_for("seat_0", &state),
        vec![
            ACTION_EVENT.to_owned(),
            ACTION_LIMITED_OPERATION.to_owned(),
            ACTION_PASS.to_owned()
        ]
    );
}

#[test]
fn domain_evidence_v1_profile_driver_wraps_event_frontier_domain_validator() {
    let driver = DomainEvidenceV1Driver::new("event_frontier");
    let artifact = domain_evidence_profile_artifact(
        DOMAIN_EVIDENCE_V1,
        PROFILE_VERSION_V1,
        Some("internal-dev"),
        "event_frontier",
        &["domain_schema_version", "domain_input", "expected_domain"],
    );

    let report = driver
        .validate(&artifact)
        .expect("domain metadata validates");
    assert_eq!(report.profile_id, DOMAIN_EVIDENCE_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "internal-dev");
    assert_eq!(report.validator_owner, "event_frontier");
    assert_eq!(artifact.metadata.canonical_byte_authority, "none");
    assert!(!artifact.canonical_byte_claim);

    let summary = driver
        .validate_with(&artifact, |_| validate_event_frontier_domain_evidence())
        .expect("profile delegates to Event Frontier domain validator");
    assert_eq!(
        summary,
        DomainEvidenceSummary {
            fixture_count: 3,
            fixture_edge_count: 8,
            ordinary_event_count: 14,
            edict_count: 4,
            survey_funds_after: 1,
            reckoning_income: (4, 6),
            terminal_winner: Some(FactionId::Freeholders),
        }
    );

    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            SETUP_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("internal-dev"),
            "event_frontier",
            &["expected_domain"],
        ),
        ProfileValidationErrorKind::WrongProfileId,
    );
    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            DOMAIN_EVIDENCE_V1,
            "v2",
            Some("internal-dev"),
            "event_frontier",
            &["expected_domain"],
        ),
        ProfileValidationErrorKind::WrongProfileVersion,
    );
    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            DOMAIN_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("unsupported"),
            "event_frontier",
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
            "event_frontier",
            &["expected_domain", "setup_options"],
        ),
        ProfileValidationErrorKind::UnknownField,
    );
}

#[test]
fn limited_operation_without_legal_target_is_not_offered() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    state.card_phase = CardPhase::AwaitingSecondChoice {
        first_faction: FactionId::Freeholders,
        second_faction: FactionId::Charter,
        first_choice: FirstChoice::Operation,
    };
    for site in &mut state.sites {
        site.agents = 3;
        site.depot = true;
        site.cache_count = 0;
    }

    let tree = legal_action_tree(&state, &actor("seat_0"));

    assert_eq!(
        tree.root
            .choices
            .iter()
            .map(|choice| choice.segment.as_str())
            .collect::<Vec<_>>(),
        vec![ACTION_EVENT, ACTION_PASS]
    );
    assert!(tree.dead_branch_paths().is_empty());
    assert!(segments_for("seat_1", &state).is_empty());
}

#[test]
fn bare_limited_operation_path_still_rejects_as_malformed() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    state.card_phase = CardPhase::AwaitingSecondChoice {
        first_faction: FactionId::Freeholders,
        second_faction: FactionId::Charter,
        first_choice: FirstChoice::Operation,
    };

    let diagnostic = validate_command(
        &state,
        &command("seat_0", ACTION_LIMITED_OPERATION, state.freshness_token),
    )
    .expect_err("bare limited operation stays malformed");

    assert_eq!(diagnostic.code, "malformed_action");
}

#[test]
fn first_pass_offers_second_full_menu_and_preserves_eligibility() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    let freshness = state.freshness_token;

    apply_command(&mut state, &command("seat_1", ACTION_PASS, freshness)).expect("pass");

    assert_eq!(
        state.card_phase,
        CardPhase::AwaitingSecondChoice {
            first_faction: FactionId::Freeholders,
            second_faction: FactionId::Charter,
            first_choice: event_frontier::FirstChoice::Pass,
        }
    );
    assert_eq!(
        state.eligibility_for(FactionId::Freeholders),
        Eligibility::Eligible
    );
    assert_eq!(state.resources.provisions, 5);
    assert_eq!(
        segments_for("seat_0", &state),
        vec![
            ACTION_EVENT.to_owned(),
            ACTION_OPERATION.to_owned(),
            ACTION_PASS.to_owned()
        ]
    );
}

#[test]
fn double_pass_discards_card_and_keeps_both_factions_eligible() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    let first_freshness = state.freshness_token;
    apply_command(&mut state, &command("seat_1", ACTION_PASS, first_freshness))
        .expect("first pass");
    let second_freshness = state.freshness_token;
    let result = apply_command(
        &mut state,
        &command("seat_0", ACTION_PASS, second_freshness),
    )
    .expect("second pass");

    assert_eq!(
        state.deck.discard[0],
        event_frontier::CardId::HighMeadowFair
    );
    assert_eq!(
        state.deck.current,
        Some(event_frontier::CardId::ReckoningOne)
    );
    assert_eq!(state.card_phase, CardPhase::Reckoning);
    assert_eq!(
        state.eligibility_for(FactionId::Charter),
        Eligibility::Eligible
    );
    assert_eq!(
        state.eligibility_for(FactionId::Freeholders),
        Eligibility::Eligible
    );
    assert!(result.effects.iter().any(|effect| matches!(
        &effect.payload,
        EventFrontierEffect::CardDiscarded {
            reason,
            ..
        } if reason == "double_pass"
    )));
}

#[test]
fn no_eligible_faction_discards_unresolved_card_and_restores_eligibility() {
    let mut state = setup_match(Seed(42), &seats(), &SetupOptions::default()).expect("setup");
    state.set_eligibility(FactionId::Charter, Eligibility::Ineligible);
    state.set_eligibility(FactionId::Freeholders, Eligibility::Ineligible);
    let mut effects = Vec::new();

    advance_to_next_card(&mut state, "test_advance", &mut effects);

    assert_eq!(
        state.deck.current,
        Some(event_frontier::CardId::ReckoningOne)
    );
    assert_eq!(state.card_phase, CardPhase::Reckoning);
    assert_eq!(
        state.eligibility_for(FactionId::Charter),
        Eligibility::Eligible
    );
    assert_eq!(
        state.eligibility_for(FactionId::Freeholders),
        Eligibility::Eligible
    );
    assert!(effects.iter().any(|effect| matches!(
        &effect.payload,
        EventFrontierEffect::CardDiscarded {
            reason,
            ..
        } if reason == "no_eligible_faction"
    )));
}

#[test]
fn waiting_tree_is_empty_and_metadata_does_not_expose_undrawn_order() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");

    assert!(segments_for("seat_0", &state).is_empty());
    let metadata = legal_action_metadata(&state, &actor("seat_0"));
    assert!(metadata
        .iter()
        .any(|entry| entry.key == "action_status" && entry.value == "waiting"));
    let rendered = format!("{metadata:?}");
    assert!(!rendered.contains("ef_survey_ban"));
    assert!(!rendered.contains("undrawn"));
}

#[test]
fn choice_effects_do_not_expose_undrawn_deck_order() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    let freshness = state.freshness_token;
    let result = apply_command(&mut state, &command("seat_1", ACTION_PASS, freshness))
        .expect("pass applies");

    let rendered = format!("{:?}", result.effects);
    assert!(!rendered.contains("ef_survey_ban"));
    assert!(!rendered.contains("undrawn"));
}

#[test]
fn choosing_menu_matches_current_card_phase() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    let (faction, choices) = choosing_menu(&state).expect("choice menu");

    assert_eq!(faction, FactionId::Freeholders);
    assert_eq!(choices.len(), 3);
}

#[test]
fn charter_survey_is_one_compound_command_that_spends_and_places_agent() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    let freshness = state.freshness_token;
    apply_command(&mut state, &command("seat_1", ACTION_PASS, freshness)).expect("pass");
    let freshness = state.freshness_token;

    let result = apply_command(
        &mut state,
        &command("seat_0", "operation/survey/site_granite_pass", freshness),
    )
    .expect("survey");

    assert_eq!(state.resources.funds, 1);
    assert_eq!(
        state
            .site(event_frontier::SiteId::GranitePass)
            .expect("site")
            .agents,
        1
    );
    assert!(result.effects.iter().any(|effect| matches!(
        effect.payload,
        EventFrontierEffect::OpResolved {
            faction: FactionId::Charter,
            ..
        }
    )));
}

#[test]
fn charter_fortify_and_writ_apply_public_site_changes() {
    let mut fortify =
        setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup fortify");
    fortify
        .site_mut(event_frontier::SiteId::Charterhouse)
        .expect("site")
        .depot = false;
    let freshness = fortify.freshness_token;
    apply_command(&mut fortify, &command("seat_1", ACTION_PASS, freshness)).expect("pass");
    let freshness = fortify.freshness_token;
    apply_command(
        &mut fortify,
        &command("seat_0", "operation/fortify/site_charterhouse", freshness),
    )
    .expect("fortify");
    assert!(
        fortify
            .site(event_frontier::SiteId::Charterhouse)
            .expect("site")
            .depot
    );

    let mut writ = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup writ");
    writ.site_mut(event_frontier::SiteId::Charterhouse)
        .expect("site")
        .cache_count = 1;
    let freshness = writ.freshness_token;
    apply_command(&mut writ, &command("seat_1", ACTION_PASS, freshness)).expect("pass");
    let freshness = writ.freshness_token;
    apply_command(
        &mut writ,
        &command("seat_0", "operation/writ/site_charterhouse", freshness),
    )
    .expect("writ");
    assert_eq!(
        writ.site(event_frontier::SiteId::Charterhouse)
            .expect("site")
            .cache_count,
        0
    );
    assert_eq!(writ.resources.funds, 2);
}

#[test]
fn freeholder_trek_cache_and_rally_apply_public_site_changes() {
    let mut trek = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup trek");
    let freshness = trek.freshness_token;
    apply_command(
        &mut trek,
        &command(
            "seat_1",
            "operation/trek/site_landing>site_crossing",
            freshness,
        ),
    )
    .expect("trek");
    assert_eq!(
        trek.site(event_frontier::SiteId::Landing)
            .expect("landing")
            .settlers,
        2
    );
    assert_eq!(
        trek.site(event_frontier::SiteId::Crossing)
            .expect("crossing")
            .settlers,
        1
    );

    let mut cache = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup cache");
    let freshness = cache.freshness_token;
    apply_command(
        &mut cache,
        &command("seat_1", "operation/cache/site_landing", freshness),
    )
    .expect("cache");
    assert_eq!(
        cache
            .site(event_frontier::SiteId::Landing)
            .expect("landing")
            .cache_count,
        2
    );

    let mut rally =
        setup_match(Seed(1), &seats(), &SetupOptions::hard_winter()).expect("setup rally");
    let freshness = rally.freshness_token;
    apply_command(
        &mut rally,
        &command("seat_1", "operation/rally/site_landing", freshness),
    )
    .expect("rally");
    assert_eq!(
        rally
            .site(event_frontier::SiteId::Landing)
            .expect("landing")
            .settlers,
        3
    );
}

#[test]
fn operation_diagnostics_are_viewer_safe_for_bounds_costs_and_preconditions() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    let freshness = state.freshness_token;
    apply_command(&mut state, &command("seat_1", ACTION_PASS, freshness)).expect("pass");
    let freshness = state.freshness_token;
    let over = apply_command(
        &mut state,
        &command(
            "seat_0",
            "operation/survey/site_charterhouse,site_crossing,site_granite_pass",
            freshness,
        ),
    )
    .expect_err("over budget");
    assert_eq!(over.code, "operation_site_bound_exceeded");
    assert!(!format!("{over:?}").contains("ef_survey_ban"));

    let mut poor = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup poor");
    poor.resources.provisions = 0;
    let freshness = poor.freshness_token;
    let unaffordable = apply_command(
        &mut poor,
        &command("seat_1", "operation/cache/site_landing", freshness),
    )
    .expect_err("unaffordable");
    assert_eq!(unaffordable.code, "unaffordable_operation");

    let mut invalid =
        setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup invalid");
    let freshness = invalid.freshness_token;
    let precondition = apply_command(
        &mut invalid,
        &command("seat_1", "operation/cache/site_charterhouse", freshness),
    )
    .expect_err("precondition");
    assert_eq!(
        precondition.code,
        "cache_requires_settler_no_depot_under_cap"
    );
}

#[test]
fn limited_operation_rejects_more_than_one_site() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    let freshness = state.freshness_token;
    apply_command(
        &mut state,
        &command("seat_1", "operation/cache/site_landing", freshness),
    )
    .expect("first op");
    let freshness = state.freshness_token;

    let diagnostic = apply_command(
        &mut state,
        &command(
            "seat_0",
            "limited_operation/survey/site_crossing,site_granite_pass",
            freshness,
        ),
    )
    .expect_err("limited op over one site");

    assert_eq!(diagnostic.code, "operation_site_bound_exceeded");
}

#[test]
fn every_ordinary_event_card_has_typed_rust_effects() {
    let ordinary = [
        event_frontier::CardId::BorderSurvey,
        event_frontier::CardId::RiverMists,
        event_frontier::CardId::StorehouseFire,
        event_frontier::CardId::HighMeadowFair,
        event_frontier::CardId::DepotGrants,
        event_frontier::CardId::TrailWashout,
        event_frontier::CardId::CharterAudit,
        event_frontier::CardId::FreeholderMoot,
        event_frontier::CardId::OldMillStrike,
        event_frontier::CardId::CrossingMarket,
        event_frontier::CardId::GranitePassSnows,
        event_frontier::CardId::CacheBoom,
        event_frontier::CardId::AgentsRecall,
        event_frontier::CardId::LastLight,
    ];

    for card in ordinary {
        let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
        let effects = resolve_event_card(&mut state, card);
        assert!(
            effects.iter().any(|effect| matches!(
                effect.payload,
                EventFrontierEffect::EventResolved {
                    card: resolved,
                    ..
                } if resolved == card
            )),
            "{card:?} did not emit EventResolved"
        );
    }
}

#[test]
fn edicts_activate_as_typed_modifiers() {
    let edicts = [
        (event_frontier::CardId::TollRoads, EdictKind::TollRoads),
        (event_frontier::CardId::SurveyBan, EdictKind::SurveyBan),
        (event_frontier::CardId::LongSeason, EdictKind::LongSeason),
        (event_frontier::CardId::Requisition, EdictKind::Requisition),
    ];

    for (card, kind) in edicts {
        let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
        let effects = resolve_event_card(&mut state, card);

        assert_eq!(state.active_edicts.len(), 1);
        assert_eq!(state.active_edicts[0].kind, kind);
        assert!(effects.iter().any(|effect| matches!(
            &effect.payload,
            EventFrontierEffect::EdictActivated {
                edict,
                ..
            } if edict == kind.as_str()
        )));
    }
}

#[test]
fn toll_roads_and_requisition_modify_operation_cost_without_patching_base_rules() {
    let mut toll = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup toll");
    resolve_event_card(&mut toll, event_frontier::CardId::TollRoads);
    let freshness = toll.freshness_token;
    apply_command(
        &mut toll,
        &command("seat_1", "operation/cache/site_landing", freshness),
    )
    .expect("toll cache");
    assert_eq!(toll.resources.provisions, 2);

    let mut requisition =
        setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup requisition");
    resolve_event_card(&mut requisition, event_frontier::CardId::Requisition);
    let freshness = requisition.freshness_token;
    apply_command(&mut requisition, &command("seat_1", ACTION_PASS, freshness)).expect("pass");
    let freshness = requisition.freshness_token;
    apply_command(
        &mut requisition,
        &command("seat_0", "operation/survey/site_charterhouse", freshness),
    )
    .expect("free depot survey");
    assert_eq!(requisition.resources.funds, 2);
}

#[test]
fn survey_ban_and_long_season_modify_legality_and_bounds() {
    let mut banned =
        setup_match(Seed(1), &seats(), &SetupOptions::hard_winter()).expect("setup banned");
    banned
        .site_mut(event_frontier::SiteId::Landing)
        .expect("landing")
        .agents = 1;
    resolve_event_card(&mut banned, event_frontier::CardId::SurveyBan);
    let freshness = banned.freshness_token;
    let diagnostic = apply_command(
        &mut banned,
        &command("seat_1", "operation/rally/site_landing", freshness),
    )
    .expect_err("survey ban blocks contested rally");
    assert_eq!(diagnostic.code, "survey_ban_contested_site");

    let mut long = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup long");
    long.resources.funds = 3;
    resolve_event_card(&mut long, event_frontier::CardId::LongSeason);
    long.deck.current = Some(event_frontier::CardId::TollRoads);
    long.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };
    let freshness = long.freshness_token;
    apply_command(
        &mut long,
        &command(
            "seat_0",
            "operation/survey/site_charterhouse,site_crossing,site_granite_pass",
            freshness,
        ),
    )
    .expect("long season extra site");
}

#[test]
fn edict_expiry_is_a_deterministic_list_clear() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    resolve_event_card(&mut state, event_frontier::CardId::Requisition);
    resolve_event_card(&mut state, event_frontier::CardId::TollRoads);

    let effects = expire_all_edicts(&mut state);

    assert!(state.active_edicts.is_empty());
    let expired = effects
        .iter()
        .filter_map(|effect| match &effect.payload {
            EventFrontierEffect::EdictExpired { edict } => Some(edict.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(expired, vec!["toll_roads", "requisition"]);
}

#[test]
fn reckoning_pipeline_scores_income_then_reset_in_order() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    resolve_event_card(&mut state, event_frontier::CardId::TollRoads);
    state.set_eligibility(FactionId::Charter, Eligibility::Ineligible);
    state.set_eligibility(FactionId::Freeholders, Eligibility::Ineligible);
    state.deck.current = Some(event_frontier::CardId::ReckoningOne);
    state.card_phase = CardPhase::Reckoning;

    let result = resolve_reckoning(&mut state).expect("reckoning");

    assert_eq!(state.scores.charter, 1);
    assert_eq!(state.scores.freeholders, 2);
    assert_eq!(state.resources.funds, 4);
    assert_eq!(state.resources.provisions, 6);
    assert!(state.active_edicts.is_empty());
    assert_eq!(
        state.eligibility_for(FactionId::Charter),
        Eligibility::Eligible
    );
    let rendered = result
        .effects
        .iter()
        .map(|effect| format!("{:?}", effect.payload))
        .collect::<Vec<_>>();
    let income = rendered
        .iter()
        .position(|entry| entry.contains("reckoning_income"))
        .expect("income");
    let expiry = rendered
        .iter()
        .position(|entry| entry.contains("EdictExpired"))
        .expect("expiry");
    let reset = rendered
        .iter()
        .position(|entry| entry.contains("reckoning_reset"))
        .expect("reset");
    let resolved = rendered
        .iter()
        .position(|entry| entry.contains("ReckoningResolved"))
        .expect("resolved");
    assert!(income < expiry && expiry < reset && reset < resolved);
}

#[test]
fn charter_freeholder_and_both_met_instant_victories_are_deterministic() {
    let mut charter = reckoning_state();
    for site in [
        event_frontier::SiteId::Charterhouse,
        event_frontier::SiteId::Crossing,
        event_frontier::SiteId::GranitePass,
        event_frontier::SiteId::OldMill,
    ] {
        charter.site_mut(site).expect("site").agents = 1;
    }
    resolve_reckoning(&mut charter).expect("charter reckoning");
    assert_terminal(
        &charter,
        FactionId::Charter,
        VictoryType::CharterInstant,
        "EF-END-001",
    );

    let mut freeholder = reckoning_state();
    set_caches(&mut freeholder, 8);
    resolve_reckoning(&mut freeholder).expect("freeholder reckoning");
    assert_terminal(
        &freeholder,
        FactionId::Freeholders,
        VictoryType::FreeholderInstant,
        "EF-END-002",
    );

    let mut both = reckoning_state();
    for site in [
        event_frontier::SiteId::Charterhouse,
        event_frontier::SiteId::Crossing,
        event_frontier::SiteId::GranitePass,
        event_frontier::SiteId::OldMill,
    ] {
        both.site_mut(site).expect("site").agents = 1;
    }
    set_caches(&mut both, 8);
    resolve_reckoning(&mut both).expect("both reckoning");
    assert_terminal(
        &both,
        FactionId::Freeholders,
        VictoryType::FreeholderInstant,
        "EF-END-003",
    );
}

#[test]
fn third_reckoning_final_fallback_and_tiebreak_are_stable() {
    let mut charter = empty_reckoning_state();
    charter.reckoning_count = 2;
    charter.scores = FactionScores {
        charter: 5,
        freeholders: 4,
    };
    resolve_reckoning(&mut charter).expect("fallback");
    assert_terminal(
        &charter,
        FactionId::Charter,
        VictoryType::FinalFallback,
        "EF-END-004",
    );

    let mut tied = empty_reckoning_state();
    tied.reckoning_count = 2;
    tied.scores = FactionScores {
        charter: 4,
        freeholders: 4,
    };
    resolve_reckoning(&mut tied).expect("tiebreak");
    assert_terminal(
        &tied,
        FactionId::Freeholders,
        VictoryType::FinalFallback,
        "EF-END-004",
    );
}

#[test]
fn post_terminal_commands_are_rejected() {
    let mut state = reckoning_state();
    set_caches(&mut state, 8);
    resolve_reckoning(&mut state).expect("terminal");
    let freshness = state.freshness_token;
    let diagnostic = apply_command(&mut state, &command("seat_1", ACTION_PASS, freshness))
        .expect_err("terminal rejects command");
    assert_eq!(diagnostic.code, "terminal");
}

fn reckoning_state() -> event_frontier::EventFrontierState {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    state.deck.current = Some(event_frontier::CardId::ReckoningOne);
    state.card_phase = CardPhase::Reckoning;
    state
}

fn empty_reckoning_state() -> event_frontier::EventFrontierState {
    let mut state = reckoning_state();
    for site in &mut state.sites {
        site.agents = 0;
        site.settlers = 0;
        site.depot = false;
        site.cache_count = 0;
    }
    state
}

fn set_caches(state: &mut event_frontier::EventFrontierState, count: u8) {
    let mut remaining = count;
    for site in &mut state.sites {
        let placed = remaining.min(2);
        site.cache_count = placed;
        remaining -= placed;
    }
}

fn assert_terminal(
    state: &event_frontier::EventFrontierState,
    faction: FactionId,
    victory_type: VictoryType,
    decisive_rule: &'static str,
) {
    assert!(matches!(
        state.terminal_outcome,
        Some(TerminalOutcome::Winner {
            faction: winner,
            victory_type: actual_type,
            decisive_rule: actual_rule,
            ..
        }) if winner == faction && actual_type == victory_type && actual_rule == decisive_rule
    ));
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
    let driver = DomainEvidenceV1Driver::new("event_frontier");
    assert_eq!(
        driver
            .validate(&artifact)
            .expect_err("invalid domain-evidence-v1 metadata rejects")
            .kind,
        expected
    );
}

fn validate_event_frontier_domain_evidence() -> DomainEvidenceSummary {
    let fixtures = [
        parse_event_domain_fixture(include_str!(
            "../data/fixtures/event_frontier_standard.fixture.json"
        )),
        parse_event_domain_fixture(include_str!(
            "../data/fixtures/event_frontier_hard_winter.fixture.json"
        )),
        parse_event_domain_fixture(include_str!(
            "../data/fixtures/event_frontier_land_rush.fixture.json"
        )),
    ];

    let mut fixture_edge_count = None;
    for fixture in &fixtures {
        let variant = ScenarioVariant::resolve(&fixture.variant).expect("variant resolves");
        let state = setup_match(Seed(fixture.seed), &seats(), &SetupOptions { variant })
            .expect("fixture setup");

        assert_eq!(fixture.game_id, GAME_ID);
        assert_eq!(fixture.rules_version, RULES_VERSION_LABEL);
        assert_eq!(fixture.phase, state.card_phase.stable_summary());
        assert_eq!(fixture.eligibility, eligibility_summary(&state));
        assert_eq!(fixture.edges, edge_summary(&state.variant.edges));
        assert_eq!(fixture.current_card, optional_card_id(state.deck.current));
        assert_eq!(
            fixture.next_public_card,
            optional_card_id(state.deck.next_public)
        );
        assert_eq!(fixture.terminal_outcome, "none");
        assert!(state.resources.funds <= state.variant.resource_cap);
        assert!(state.resources.provisions <= state.variant.resource_cap);
        assert!(fixture.resources.contains("funds:"));
        assert!(fixture.thresholds.contains("charter_sites:"));
        assert!(fixture.site_states.contains(SiteId::Landing.as_str()));

        let edge_count = state.variant.edges.len();
        assert_eq!(*fixture_edge_count.get_or_insert(edge_count), edge_count);
    }

    let ordinary_event_count = validate_event_cards();
    let (edict_count, survey_funds_after) = validate_edicts_and_operations();
    let reckoning_income = validate_reckoning_income();
    let terminal_winner = validate_terminal_scoring();

    DomainEvidenceSummary {
        fixture_count: fixtures.len(),
        fixture_edge_count: fixture_edge_count.expect("fixture edge count"),
        ordinary_event_count,
        edict_count,
        survey_funds_after,
        reckoning_income,
        terminal_winner,
    }
}

fn validate_event_cards() -> usize {
    let ordinary = [
        CardId::BorderSurvey,
        CardId::RiverMists,
        CardId::StorehouseFire,
        CardId::HighMeadowFair,
        CardId::DepotGrants,
        CardId::TrailWashout,
        CardId::CharterAudit,
        CardId::FreeholderMoot,
        CardId::OldMillStrike,
        CardId::CrossingMarket,
        CardId::GranitePassSnows,
        CardId::CacheBoom,
        CardId::AgentsRecall,
        CardId::LastLight,
    ];

    for card in ordinary {
        let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
        let effects = resolve_event_card(&mut state, card);
        assert!(effects.iter().any(|effect| matches!(
            effect.payload,
            EventFrontierEffect::EventResolved {
                card: resolved,
                ..
            } if resolved == card
        )));
    }

    ordinary.len()
}

fn validate_edicts_and_operations() -> (usize, u8) {
    let edicts = [
        (CardId::TollRoads, EdictKind::TollRoads),
        (CardId::SurveyBan, EdictKind::SurveyBan),
        (CardId::LongSeason, EdictKind::LongSeason),
        (CardId::Requisition, EdictKind::Requisition),
    ];

    for (card, kind) in edicts {
        let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
        resolve_event_card(&mut state, card);
        assert_eq!(state.active_edicts[0].kind, kind);
    }

    let mut survey = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    let freshness = survey.freshness_token;
    apply_command(&mut survey, &command("seat_1", ACTION_PASS, freshness)).expect("pass");
    let freshness = survey.freshness_token;
    apply_command(
        &mut survey,
        &command("seat_0", "operation/survey/site_granite_pass", freshness),
    )
    .expect("survey");
    assert_eq!(survey.site(SiteId::GranitePass).expect("site").agents, 1);

    let mut cache = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    let freshness = cache.freshness_token;
    apply_command(
        &mut cache,
        &command("seat_1", "operation/cache/site_landing", freshness),
    )
    .expect("cache");
    assert_eq!(cache.site(SiteId::Landing).expect("landing").cache_count, 2);

    let mut limited = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    let freshness = limited.freshness_token;
    apply_command(
        &mut limited,
        &command("seat_1", "operation/cache/site_landing", freshness),
    )
    .expect("first op");
    let freshness = limited.freshness_token;
    let diagnostic = apply_command(
        &mut limited,
        &command(
            "seat_0",
            "limited_operation/survey/site_crossing,site_granite_pass",
            freshness,
        ),
    )
    .expect_err("limited op bound");
    assert_eq!(diagnostic.code, "operation_site_bound_exceeded");

    (edicts.len(), survey.resources.funds)
}

fn validate_reckoning_income() -> (u8, u8) {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    resolve_event_card(&mut state, CardId::TollRoads);
    state.set_eligibility(FactionId::Charter, Eligibility::Ineligible);
    state.set_eligibility(FactionId::Freeholders, Eligibility::Ineligible);
    state.deck.current = Some(CardId::ReckoningOne);
    state.card_phase = CardPhase::Reckoning;

    resolve_reckoning(&mut state).expect("reckoning");

    assert_eq!(state.scores.charter, 1);
    assert_eq!(state.scores.freeholders, 2);
    assert!(state.active_edicts.is_empty());
    assert_eq!(
        state.eligibility_for(FactionId::Charter),
        Eligibility::Eligible
    );
    (state.resources.funds, state.resources.provisions)
}

fn validate_terminal_scoring() -> Option<FactionId> {
    let mut state = reckoning_state();
    set_caches(&mut state, 8);
    resolve_reckoning(&mut state).expect("terminal");

    match state.terminal_outcome {
        Some(TerminalOutcome::Winner {
            faction,
            victory_type: VictoryType::FreeholderInstant,
            decisive_rule: "EF-END-002",
            ..
        }) => Some(faction),
        _ => panic!("expected Freeholder instant terminal"),
    }
}

#[derive(Debug)]
struct EventDomainFixture {
    game_id: String,
    variant: String,
    rules_version: String,
    seed: u64,
    phase: String,
    resources: String,
    thresholds: String,
    eligibility: String,
    current_card: String,
    next_public_card: String,
    edges: String,
    site_states: String,
    terminal_outcome: String,
}

fn parse_event_domain_fixture(input: &str) -> EventDomainFixture {
    EventDomainFixture {
        game_id: string_field(input, "game_id"),
        variant: string_field(input, "variant"),
        rules_version: string_field(input, "rules_version"),
        seed: number_field(input, "seed"),
        phase: string_field(input, "phase"),
        resources: string_field(input, "resources"),
        thresholds: string_field(input, "thresholds"),
        eligibility: string_field(input, "eligibility"),
        current_card: string_field(input, "current_card"),
        next_public_card: string_field(input, "next_public_card"),
        edges: string_field(input, "edges"),
        site_states: string_field(input, "site_states"),
        terminal_outcome: string_field(input, "terminal_outcome"),
    }
}

fn string_field(input: &str, key: &str) -> String {
    let needle = format!("\"{key}\": \"");
    let start = input.find(&needle).expect("field") + needle.len();
    let tail = &input[start..];
    let end = tail.find('"').expect("field terminator");
    tail[..end].to_owned()
}

fn number_field(input: &str, key: &str) -> u64 {
    let needle = format!("\"{key}\": ");
    let start = input.find(&needle).expect("number field") + needle.len();
    let tail = &input[start..];
    let end = tail
        .find(|ch: char| !ch.is_ascii_digit())
        .expect("number terminator");
    tail[..end].parse().expect("number parses")
}

fn optional_card_id(card: Option<CardId>) -> String {
    card.map(CardId::as_str).unwrap_or_default().to_owned()
}

fn eligibility_summary(state: &EventFrontierState) -> String {
    state
        .factions
        .iter()
        .map(|faction| {
            format!(
                "{}:{}",
                faction.as_str(),
                state.eligibility_for(*faction).as_str()
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn edge_summary(edges: &[(SiteId, SiteId)]) -> String {
    edges
        .iter()
        .map(|(left, right)| format!("{}-{}", left.as_str(), right.as_str()))
        .collect::<Vec<_>>()
        .join(",")
}
