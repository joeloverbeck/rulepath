use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, Viewer};
use poker_lite::{
    apply_action, filter_effects_for_viewer, legal_action_tree, project_view, setup_effects,
    setup_match, validate_command, CenterView, CrestCardId, Phase, PokerLiteSeat, PrivateView,
    SetupOptions, ShowdownReveal, TerminalOutcome, TerminalView,
};

fn standard_state() -> poker_lite::PokerLiteState {
    setup_match(
        Seed(11),
        &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn state_with_distinct_private_ranks() -> poker_lite::PokerLiteState {
    for seed in 0..100 {
        let state = setup_match(
            Seed(seed),
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds");
        if state.private_card_for_internal(PokerLiteSeat::Seat0).rank()
            != state.private_card_for_internal(PokerLiteSeat::Seat1).rank()
        {
            return state;
        }
    }
    panic!("expected a seed with distinct private ranks");
}

fn viewer(seat: Option<&str>) -> Viewer {
    Viewer {
        seat_id: seat.map(|value| SeatId(value.to_owned())),
    }
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn command(state: &poker_lite::PokerLiteState, seat: &str, segment: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn apply_segment(state: &mut poker_lite::PokerLiteState, seat: &str, segment: &str) {
    let action = validate_command(state, &command(state, seat, segment)).expect("valid command");
    apply_action(state, action).expect("apply succeeds");
}

fn terminal_state_with(outcome: TerminalOutcome) -> poker_lite::PokerLiteState {
    let mut state = standard_state();
    state.phase = Phase::Terminal;
    state.active_seat = None;
    state.terminal_outcome = Some(outcome);
    state
}

#[test]
fn observer_before_reveal_sees_counts_and_no_hidden_crests() {
    let state = standard_state();
    let view = project_view(&state, &viewer(None));

    assert_eq!(view.phase, Phase::PledgeRound { round_index: 0 });
    assert_eq!(view.private_counts, [1, 1]);
    assert!(matches!(view.center, CenterView::Hidden { .. }));
    assert!(matches!(view.private_view, PrivateView::Observer));
    assert_eq!(view.terminal, TerminalView::NonTerminal);

    let text = format!("{view:?}");
    for card in state.private_cards_internal() {
        assert!(!text.contains(card.as_str()));
        assert!(!text.contains(&card.label()));
    }
    assert!(!text.contains(state.center_card_internal().as_str()));
    for card in state.deck_tail_internal() {
        assert!(!text.contains(card.as_str()));
    }
}

#[test]
fn seat_view_gets_only_own_private_strength_bucket() {
    let state = state_with_distinct_private_ranks();
    let seat_0_view = project_view(&state, &viewer(Some("seat_0")));
    let seat_1_private = state.private_card_for_internal(PokerLiteSeat::Seat1);

    let PrivateView::Seat(private) = &seat_0_view.private_view else {
        panic!("seat private view expected");
    };
    assert_eq!(private.seat, PokerLiteSeat::Seat0);
    assert!(private.own_private.is_some());
    assert!(private
        .own_strength_bucket
        .as_deref()
        .is_some_and(|bucket| bucket.ends_with("_private")));

    let text = format!("{seat_0_view:?}");
    assert!(text.contains(
        state
            .private_card_for_internal(PokerLiteSeat::Seat0)
            .as_str()
    ));
    assert!(!text.contains(seat_1_private.as_str()));
    assert!(!text.contains(state.center_card_internal().as_str()));
    assert!(!format!("{:?}", project_view(&state, &viewer(None)))
        .contains(private.own_strength_bucket.as_deref().expect("bucket")));
    assert!(
        !format!("{:?}", project_view(&state, &viewer(Some("seat_1"))))
            .contains(private.own_strength_bucket.as_deref().expect("bucket"))
    );
}

#[test]
fn center_reveal_does_not_reveal_private_or_tail() {
    let mut state = standard_state();
    apply_segment(&mut state, "seat_0", "hold");
    apply_segment(&mut state, "seat_1", "hold");

    let view = project_view(&state, &viewer(None));
    assert!(matches!(view.center, CenterView::Revealed(_)));
    let text = format!("{view:?}");
    assert!(text.contains(state.center_card_internal().as_str()));
    for card in state.private_cards_internal() {
        assert!(!text.contains(card.as_str()));
    }
    for card in state.deck_tail_internal() {
        assert!(!text.contains(card.as_str()));
    }
}

#[test]
fn showdown_view_reveals_both_private_crests_and_yield_does_not() {
    let mut showdown = standard_state();
    for (seat, segment) in [
        ("seat_0", "hold"),
        ("seat_1", "hold"),
        ("seat_1", "hold"),
        ("seat_0", "hold"),
    ] {
        apply_segment(&mut showdown, seat, segment);
    }
    let view = project_view(&showdown, &viewer(None));
    let text = format!("{view:?}");
    for card in showdown.private_cards_internal() {
        assert!(text.contains(card.as_str()));
    }
    assert!(matches!(
        view.terminal,
        TerminalView::ShowdownWin { .. } | TerminalView::Split { .. }
    ));

    let mut yielded = standard_state();
    apply_segment(&mut yielded, "seat_0", "press");
    apply_segment(&mut yielded, "seat_1", "yield");
    assert!(matches!(
        yielded.terminal_outcome,
        Some(TerminalOutcome::YieldWin { .. })
    ));
    let yield_view = project_view(&yielded, &viewer(None));
    let yield_text = format!("{yield_view:?}");
    assert!(matches!(yield_view.terminal, TerminalView::YieldWin { .. }));
    for card in yielded.private_cards_internal() {
        assert!(!yield_text.contains(card.as_str()));
    }
}

#[test]
fn showdown_rationale_explains_pair_beats_high_card() {
    let reveal = ShowdownReveal {
        seat_0_private: CrestCardId::LowDawn,
        seat_1_private: CrestCardId::HighDawn,
        center: CrestCardId::LowDusk,
    };
    let view = project_view(
        &terminal_state_with(TerminalOutcome::ShowdownWin {
            winner: PokerLiteSeat::Seat0,
            shared_pool: 2,
            contributions: [1, 1],
            reveal,
        }),
        &viewer(None),
    );

    let TerminalView::ShowdownWin { rationale, .. } = &view.terminal else {
        panic!("showdown win expected");
    };
    assert_eq!(rationale.decisive_cause, "pair_beats_high_card");
    assert_eq!(rationale.template_key, "poker_lite.pair_beats_high_card");
    assert!(rationale
        .decisive_rule_ids
        .iter()
        .any(|rule| rule == "CL-SCORE-004"));
    assert_eq!(
        rationale.per_seat[0]
            .strength
            .as_ref()
            .expect("seat 0 strength")
            .pair_bucket,
        "pair"
    );
    assert_eq!(
        rationale.per_seat[1]
            .strength
            .as_ref()
            .expect("seat 1 strength")
            .pair_bucket,
        "high_card"
    );
    assert_eq!(
        view.showdown
            .as_ref()
            .expect("showdown view")
            .rationale
            .decisive_cause,
        rationale.decisive_cause
    );
}

#[test]
fn showdown_rationale_explains_private_rank_tiebreak() {
    let reveal = ShowdownReveal {
        seat_0_private: CrestCardId::MiddleDawn,
        seat_1_private: CrestCardId::HighDawn,
        center: CrestCardId::LowDawn,
    };
    let view = project_view(
        &terminal_state_with(TerminalOutcome::ShowdownWin {
            winner: PokerLiteSeat::Seat1,
            shared_pool: 2,
            contributions: [1, 1],
            reveal,
        }),
        &viewer(None),
    );

    let TerminalView::ShowdownWin { rationale, .. } = &view.terminal else {
        panic!("showdown win expected");
    };
    assert_eq!(rationale.decisive_cause, "higher_private_rank");
    assert_eq!(rationale.template_key, "poker_lite.private_rank_tiebreak");
    assert_eq!(
        rationale.per_seat[0]
            .strength
            .as_ref()
            .expect("seat 0 strength")
            .private_rank_value,
        2
    );
    assert_eq!(
        rationale.per_seat[1]
            .strength
            .as_ref()
            .expect("seat 1 strength")
            .private_rank_value,
        3
    );
}

#[test]
fn split_rationale_explains_equal_strength() {
    let reveal = ShowdownReveal {
        seat_0_private: CrestCardId::MiddleDawn,
        seat_1_private: CrestCardId::MiddleDusk,
        center: CrestCardId::HighDawn,
    };
    let view = project_view(
        &terminal_state_with(TerminalOutcome::Split {
            shared_pool: 2,
            each: 1,
            contributions: [1, 1],
            reveal,
        }),
        &viewer(None),
    );

    let TerminalView::Split {
        rationale, each, ..
    } = &view.terminal
    else {
        panic!("split expected");
    };
    assert_eq!(*each, 1);
    assert_eq!(rationale.decisive_cause, "equal_strength_split");
    assert_eq!(rationale.template_key, "poker_lite.equal_strength_split");
    assert!(rationale
        .decisive_rule_ids
        .iter()
        .any(|rule| rule == "CL-END-003"));
    assert!(rationale
        .per_seat
        .iter()
        .all(|seat| seat.result == "split" && seat.allocation == 1));
}

#[test]
fn yield_rationale_carries_no_private_strength_or_loser_crest() {
    let mut yielded = standard_state();
    apply_segment(&mut yielded, "seat_0", "press");
    apply_segment(&mut yielded, "seat_1", "yield");
    let yielded_loser = yielded.private_card_for_internal(PokerLiteSeat::Seat1);

    for seat in [None, Some("seat_0"), Some("seat_1")] {
        let view = project_view(&yielded, &viewer(seat));
        let TerminalView::YieldWin { rationale, .. } = &view.terminal else {
            panic!("yield win expected");
        };
        assert_eq!(rationale.decisive_cause, "opponent_yielded");
        assert_eq!(rationale.template_key, "poker_lite.yield_win_no_reveal");
        assert!(rationale
            .per_seat
            .iter()
            .all(|seat| seat.strength.is_none()));

        let rationale_text = format!("{rationale:?}");
        assert!(!rationale_text.contains("pair_flag"));
        assert!(!rationale_text.contains("private_rank_value"));
        assert!(!rationale_text.contains(yielded_loser.as_str()));
        assert!(!rationale_text.contains(&yielded_loser.label()));
    }

    let observer_text = format!("{:?}", project_view(&yielded, &viewer(None)));
    assert!(!observer_text.contains(yielded_loser.as_str()));
    assert!(!observer_text.contains(&yielded_loser.label()));
    let winner_text = format!("{:?}", project_view(&yielded, &viewer(Some("seat_0"))));
    assert!(!winner_text.contains(yielded_loser.as_str()));
    assert!(!winner_text.contains(&yielded_loser.label()));
}

#[test]
fn action_effect_and_diagnostic_surfaces_do_not_leak_pre_reveal() {
    let state = standard_state();
    let tree = legal_action_tree(&state, &actor("seat_0"));
    let effects = filter_effects_for_viewer(&setup_effects(&state), &viewer(None));
    let diagnostic = validate_command(&state, &command(&state, "seat_0", "match")).unwrap_err();
    let text = format!("{tree:?}{effects:?}{diagnostic:?}");

    for card in state.private_cards_internal() {
        assert!(!text.contains(card.as_str()));
        assert!(!text.contains(&card.label()));
    }
    assert!(!text.contains(state.center_card_internal().as_str()));
    for card in state.deck_tail_internal() {
        assert!(!text.contains(card.as_str()));
    }
}
