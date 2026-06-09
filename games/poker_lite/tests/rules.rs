use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed};
use poker_lite::{
    apply_action, compare_showdown, legal_action_tree, setup_match, validate_command, CrestCardId,
    Phase, PokerLiteEffect, PokerLiteSeat, SetupOptions, ShowdownReveal, TerminalOutcome,
};

fn standard_state() -> poker_lite::PokerLiteState {
    setup_match(
        Seed(3),
        &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
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

#[test]
fn legal_action_generation_follows_pledge_state() {
    let mut state = standard_state();

    let opening_segments = legal_action_tree(&state, &actor("seat_0"))
        .root
        .choices
        .into_iter()
        .map(|choice| choice.segment)
        .collect::<Vec<_>>();
    assert_eq!(opening_segments, vec!["hold", "press"]);

    state.active_seat = Some(PokerLiteSeat::Seat1);
    state.round.outstanding_actor = Some(PokerLiteSeat::Seat1);
    state.round.outstanding_amount = 1;
    let facing_segments = legal_action_tree(&state, &actor("seat_1"))
        .root
        .choices
        .into_iter()
        .map(|choice| choice.segment)
        .collect::<Vec<_>>();
    assert_eq!(facing_segments, vec!["lift", "match", "yield"]);

    state.round.lift_used = true;
    let capped_segments = legal_action_tree(&state, &actor("seat_1"))
        .root
        .choices
        .into_iter()
        .map(|choice| choice.segment)
        .collect::<Vec<_>>();
    assert_eq!(capped_segments, vec!["match", "yield"]);
}

#[test]
fn metadata_is_public_allow_list_only() {
    let state = standard_state();
    let tree = legal_action_tree(&state, &actor("seat_0"));
    let allowed = [
        "action_family",
        "round_index",
        "round_unit",
        "actor_seat",
        "required_to_match",
        "adds_to_pool",
        "shared_pool_after",
        "lift_cap_remaining",
        "center_visible",
        "accessibility_copy",
    ];

    for choice in &tree.root.choices {
        assert!(choice
            .metadata
            .iter()
            .all(|entry| allowed.contains(&entry.key.as_str())));
        let serialized = format!("{choice:?}");
        for forbidden in ["card", "rank", "deck", "hidden", "strength", "private"] {
            assert!(!serialized.contains(forbidden), "{serialized}");
        }
    }
}

#[test]
fn validation_reports_fail_closed_diagnostics() {
    let state = standard_state();
    assert_eq!(
        validate_command(&state, &command(&state, "seat_1", "hold"))
            .unwrap_err()
            .code,
        "wrong_seat"
    );
    assert_eq!(
        validate_command(&state, &command(&state, "seat_0", "match"))
            .unwrap_err()
            .code,
        "action_unavailable"
    );

    let mut terminal = standard_state();
    terminal.phase = Phase::Terminal;
    assert_eq!(
        validate_command(&terminal, &command(&terminal, "seat_0", "hold"))
            .unwrap_err()
            .code,
        "terminal_state"
    );

    let malformed = CommandEnvelope {
        actor: actor("seat_0"),
        action_path: ActionPath {
            segments: vec!["hold".to_owned(), "extra".to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    };
    assert_eq!(
        validate_command(&state, &malformed).unwrap_err().code,
        "malformed_action"
    );

    let mut capped = standard_state();
    capped.active_seat = Some(PokerLiteSeat::Seat1);
    capped.round.outstanding_actor = Some(PokerLiteSeat::Seat1);
    capped.round.outstanding_amount = 1;
    capped.round.lift_used = true;
    let diagnostic = validate_command(&capped, &command(&capped, "seat_1", "lift")).unwrap_err();
    assert_eq!(diagnostic.code, "lift_cap_exceeded");
    assert!(!format!("{diagnostic:?}").contains("low_"));
}

#[test]
fn hold_hold_closes_round_one_and_reveals_center_only() {
    let mut state = standard_state();

    apply_segment(&mut state, "seat_0", "hold");
    assert_eq!(state.phase, Phase::PledgeRound { round_index: 0 });
    assert_eq!(state.active_seat, Some(PokerLiteSeat::Seat1));
    assert!(!state.center_visible);

    apply_segment(&mut state, "seat_1", "hold");
    assert_eq!(state.phase, Phase::PledgeRound { round_index: 1 });
    assert_eq!(state.active_seat, Some(PokerLiteSeat::Seat1));
    assert!(state.center_visible);
    assert_eq!(state.round.round_index, 1);
    assert_eq!(state.round.unit, 2);
    assert_eq!(state.contributions, [1, 1]);
    assert_eq!(state.shared_pool, 2);
    assert!(state.terminal_outcome.is_none());
}

#[test]
fn press_lift_match_accounting_is_exact_and_bounded() {
    let mut state = standard_state();

    apply_segment(&mut state, "seat_0", "press");
    assert_eq!(state.contributions, [2, 1]);
    assert_eq!(state.shared_pool, 3);
    assert_eq!(state.active_seat, Some(PokerLiteSeat::Seat1));
    assert_eq!(state.round.outstanding_actor, Some(PokerLiteSeat::Seat1));
    assert_eq!(state.round.outstanding_amount, 1);

    apply_segment(&mut state, "seat_1", "lift");
    assert_eq!(state.contributions, [2, 3]);
    assert_eq!(state.shared_pool, 5);
    assert_eq!(state.active_seat, Some(PokerLiteSeat::Seat0));
    assert!(state.round.lift_used);
    assert_eq!(state.round.outstanding_amount, 1);

    apply_segment(&mut state, "seat_0", "match");
    assert_eq!(state.contributions, [3, 3]);
    assert_eq!(state.shared_pool, 6);
    assert!(state.center_visible);
    assert_eq!(state.phase, Phase::PledgeRound { round_index: 1 });

    apply_segment(&mut state, "seat_1", "press");
    apply_segment(&mut state, "seat_0", "lift");
    apply_segment(&mut state, "seat_1", "match");

    assert_eq!(state.contributions, [7, 7]);
    assert_eq!(state.shared_pool, 14);
    assert!(state.contributions.iter().all(|amount| *amount <= 7));
    assert_eq!(state.phase, Phase::Terminal);
    assert!(state.terminal_outcome.is_some());
}

#[test]
fn yield_terminal_awards_pool_without_showdown_reveal() {
    let mut state = standard_state();

    apply_segment(&mut state, "seat_0", "press");
    apply_segment(&mut state, "seat_1", "yield");

    assert_eq!(state.phase, Phase::Terminal);
    assert_eq!(state.active_seat, None);
    assert!(!state.center_visible);
    assert_eq!(
        state.terminal_outcome,
        Some(TerminalOutcome::YieldWin {
            winner: PokerLiteSeat::Seat0,
            loser: PokerLiteSeat::Seat1,
            shared_pool: 3,
            contributions: [2, 1],
        })
    );
}

#[test]
fn comparator_covers_pair_high_card_and_split() {
    assert_eq!(
        compare_showdown(ShowdownReveal {
            seat_0_private: CrestCardId::LowDawn,
            seat_1_private: CrestCardId::HighDawn,
            center: CrestCardId::LowDusk,
        }),
        Some(PokerLiteSeat::Seat0)
    );
    assert_eq!(
        compare_showdown(ShowdownReveal {
            seat_0_private: CrestCardId::MiddleDawn,
            seat_1_private: CrestCardId::HighDawn,
            center: CrestCardId::LowDawn,
        }),
        Some(PokerLiteSeat::Seat1)
    );
    assert_eq!(
        compare_showdown(ShowdownReveal {
            seat_0_private: CrestCardId::MiddleDawn,
            seat_1_private: CrestCardId::MiddleDusk,
            center: CrestCardId::HighDawn,
        }),
        None
    );
}

#[test]
fn showdown_terminal_allocates_win_or_split_from_rust_comparator() {
    for seed in 0..100 {
        let mut state = setup_match(
            Seed(seed),
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds");
        apply_segment(&mut state, "seat_0", "hold");
        apply_segment(&mut state, "seat_1", "hold");
        apply_segment(&mut state, "seat_1", "hold");
        apply_segment(&mut state, "seat_0", "hold");

        let Some(outcome) = state.terminal_outcome else {
            panic!("terminal outcome expected");
        };
        match outcome {
            TerminalOutcome::ShowdownWin {
                winner,
                shared_pool,
                contributions,
                reveal,
            } => {
                assert_eq!(Some(winner), compare_showdown(reveal));
                assert_eq!(shared_pool, 2);
                assert_eq!(contributions, [1, 1]);
            }
            TerminalOutcome::Split {
                shared_pool,
                each,
                contributions,
                reveal,
            } => {
                assert_eq!(compare_showdown(reveal), None);
                assert_eq!(shared_pool, 2);
                assert_eq!(each, 1);
                assert_eq!(contributions, [1, 1]);
            }
            TerminalOutcome::YieldWin { .. } => panic!("hold stream cannot yield"),
        }
    }
}

#[test]
fn showdown_transition_emits_one_grouped_showdown_reveal() {
    let mut state = standard_state();
    apply_segment(&mut state, "seat_0", "hold");
    apply_segment(&mut state, "seat_1", "hold");
    apply_segment(&mut state, "seat_1", "hold");

    let action = validate_command(&state, &command(&state, "seat_0", "hold")).expect("valid hold");
    let effects = apply_action(&mut state, action).expect("apply succeeds");
    let reveal_effects = effects
        .iter()
        .filter(|effect| matches!(effect.payload, PokerLiteEffect::ShowdownRevealed { .. }))
        .collect::<Vec<_>>();

    assert_eq!(reveal_effects.len(), 1);
    assert!(matches!(
        reveal_effects[0].payload,
        PokerLiteEffect::ShowdownRevealed { .. }
    ));
}

#[test]
fn identical_command_stream_is_deterministic() {
    fn run() -> poker_lite::PokerLiteState {
        let mut state = standard_state();
        for (seat, segment) in [
            ("seat_0", "press"),
            ("seat_1", "lift"),
            ("seat_0", "match"),
            ("seat_1", "hold"),
            ("seat_0", "hold"),
        ] {
            apply_segment(&mut state, seat, segment);
        }
        state
    }

    let first = run();
    let second = run();

    assert_eq!(first.terminal_outcome, second.terminal_outcome);
    assert_eq!(first.contributions, second.contributions);
    assert_eq!(first.shared_pool, second.shared_pool);
    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
}
