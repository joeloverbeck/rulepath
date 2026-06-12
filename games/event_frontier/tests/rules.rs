use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};
use event_frontier::{
    actions::{
        choosing_menu, ACTION_EVENT, ACTION_LIMITED_OPERATION, ACTION_OPERATION, ACTION_PASS,
    },
    apply_command,
    cards::{expire_all_edicts, resolve_event_card, EdictKind},
    legal_action_metadata, legal_action_tree,
    rules::advance_to_next_card,
    setup_match, CardPhase, Eligibility, EventFrontierEffect, FactionId, SetupOptions,
};
use event_frontier::{resolve_reckoning, FactionScores, TerminalOutcome, VictoryType};

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
