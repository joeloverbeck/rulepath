use engine_core::{SeatId, Seed, Viewer};
use token_bazaar::{
    determine_terminal_outcome, export_public_replay, project_view, setup_match, ContractId,
    OutcomeRationaleView, ResourceCounts, TerminalOutcome, TerminalTrigger, TerminalView,
    TiebreakLadderRungView, TokenBazaarLevel1Bot, TokenBazaarSeat,
};

fn state() -> token_bazaar::TokenBazaarState {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    setup_match(Seed(1), &seats, &Default::default()).expect("setup succeeds")
}

#[test]
fn observer_and_seat_views_match_for_public_game() {
    let state = state();
    let observer = project_view(&state, &Viewer { seat_id: None });
    let seat = project_view(
        &state,
        &Viewer {
            seat_id: Some(state.seats[0].clone()),
        },
    );

    assert_eq!(observer, seat);
}

#[test]
fn public_surfaces_do_not_expose_internal_or_candidate_fields() {
    let state = state();
    let view = project_view(&state, &Viewer { seat_id: None });
    let decision = TokenBazaarLevel1Bot::new(Seed(3))
        .select_decision(&state, TokenBazaarSeat::Seat0)
        .expect("bot chooses");
    let export = export_public_replay(
        1,
        &[
            vec!["collect/amber".to_owned()],
            vec!["collect/amber".to_owned()],
        ],
    );
    let combined = format!(
        "{}\n{}\n{}",
        view.stable_summary(),
        decision.rationale,
        export.to_json()
    );

    for forbidden in ["debug", "candidate", "valuation", "internal", "omniscient"] {
        assert!(
            !combined.contains(forbidden),
            "leaked forbidden token {forbidden}"
        );
    }
    assert!(view.hidden_fields.is_empty());
}

#[test]
fn terminal_rationale_marks_score_rung() {
    let mut state = terminal_state(TerminalTrigger::TurnCap);
    state.scores = [5, 3];
    state.terminal_outcome = Some(determine_terminal_outcome(&state));

    let rationale = win_rationale(&state, TokenBazaarSeat::Seat0);

    assert_eq!(rationale.decisive_cause, "score");
    assert_eq!(rationale.template_key, "token_bazaar.score_win");
    assert_eq!(rationale.terminal_trigger, "turn_cap");
    assert_eq!(
        rationale.decisive_rule_ids,
        vec!["TB-END-001", "TB-END-003", "TB-SCORE-001"]
    );
    assert_decisive_rung(&rationale, "score", Some(TokenBazaarSeat::Seat0));
}

#[test]
fn terminal_rationale_marks_fulfilled_count_rung() {
    let mut state = terminal_state(TerminalTrigger::TurnCap);
    state.scores = [3, 3];
    state.fulfilled = [
        vec![ContractId::BalancedWares, ContractId::AmberGuild],
        vec![ContractId::IronGuild],
    ];
    state.terminal_outcome = Some(determine_terminal_outcome(&state));

    let rationale = win_rationale(&state, TokenBazaarSeat::Seat0);

    assert_eq!(rationale.decisive_cause, "fulfilled_contracts");
    assert_eq!(
        rationale.template_key,
        "token_bazaar.fulfilled_tiebreak_win"
    );
    assert_eq!(
        rationale.decisive_rule_ids,
        vec!["TB-END-001", "TB-END-003", "TB-SCORE-001", "TB-SCORE-004"]
    );
    assert_decisive_rung(
        &rationale,
        "fulfilled_contracts",
        Some(TokenBazaarSeat::Seat0),
    );
    assert_eq!(rationale.final_standing[0].fulfilled_count, 2);
    assert_eq!(rationale.final_standing[1].fulfilled_count, 1);
}

#[test]
fn terminal_rationale_marks_inventory_total_rung() {
    let mut state = terminal_state(TerminalTrigger::MarketExhaustion);
    state.scores = [3, 3];
    state.fulfilled = [vec![ContractId::BalancedWares], vec![ContractId::IronGuild]];
    state.inventories = [ResourceCounts::new(1, 1, 1), ResourceCounts::new(2, 1, 1)];
    state.terminal_outcome = Some(determine_terminal_outcome(&state));

    let rationale = win_rationale(&state, TokenBazaarSeat::Seat1);

    assert_eq!(rationale.decisive_cause, "inventory_total");
    assert_eq!(
        rationale.template_key,
        "token_bazaar.inventory_tiebreak_win"
    );
    assert_eq!(rationale.terminal_trigger, "market_exhaustion");
    assert_eq!(
        rationale.decisive_rule_ids,
        vec![
            "TB-END-002",
            "TB-END-003",
            "TB-SCORE-001",
            "TB-SCORE-004",
            "TB-SCORE-005"
        ]
    );
    assert_decisive_rung(&rationale, "inventory_total", Some(TokenBazaarSeat::Seat1));
    assert_eq!(rationale.final_standing[0].inventory_total, 3);
    assert_eq!(rationale.final_standing[1].inventory_total, 4);
}

#[test]
fn terminal_rationale_marks_all_tied_draw_rung() {
    let mut state = terminal_state(TerminalTrigger::TurnCap);
    state.scores = [3, 3];
    state.fulfilled = [vec![ContractId::BalancedWares], vec![ContractId::IronGuild]];
    state.inventories = [ResourceCounts::new(1, 1, 1), ResourceCounts::new(1, 1, 1)];
    state.terminal_outcome = Some(determine_terminal_outcome(&state));

    assert_eq!(state.terminal_outcome, Some(TerminalOutcome::Draw));
    let view = project_view(&state, &Viewer { seat_id: None });
    let TerminalView::Draw { rationale } = view.terminal else {
        panic!("expected draw rationale");
    };

    assert_eq!(rationale.decisive_cause, "all_tied_draw");
    assert_eq!(rationale.template_key, "token_bazaar.all_tied_draw");
    assert_eq!(
        rationale.decisive_rule_ids,
        vec![
            "TB-END-001",
            "TB-END-003",
            "TB-SCORE-001",
            "TB-SCORE-004",
            "TB-SCORE-005"
        ]
    );
    assert_decisive_rung(&rationale, "all_tied_draw", None);
}

fn terminal_state(trigger: TerminalTrigger) -> token_bazaar::TokenBazaarState {
    let mut state = state();
    state.terminal_trigger = Some(trigger);
    state.active_seat = TokenBazaarSeat::Seat0;
    state
}

fn win_rationale(
    state: &token_bazaar::TokenBazaarState,
    expected_winner: TokenBazaarSeat,
) -> OutcomeRationaleView {
    assert_eq!(
        state.terminal_outcome,
        Some(TerminalOutcome::Win {
            seat: expected_winner
        })
    );
    let view = project_view(state, &Viewer { seat_id: None });
    let TerminalView::Win {
        winning_seat,
        rationale,
    } = view.terminal
    else {
        panic!("expected win rationale");
    };
    assert_eq!(winning_seat, expected_winner);
    rationale
}

fn assert_decisive_rung(
    rationale: &OutcomeRationaleView,
    expected_rung: &str,
    expected_winner: Option<TokenBazaarSeat>,
) {
    let decisive = rationale
        .ladder
        .iter()
        .filter(|rung| rung.decisive)
        .collect::<Vec<&TiebreakLadderRungView>>();
    assert_eq!(decisive.len(), 1);
    assert_eq!(decisive[0].rung, expected_rung);
    assert_eq!(decisive[0].winner, expected_winner);
}
