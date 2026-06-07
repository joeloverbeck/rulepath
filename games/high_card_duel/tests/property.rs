use std::collections::BTreeSet;

use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, Viewer};
use high_card_duel::{
    active_commit_seat, apply_action, commit_segment, export_public_observer_replay,
    generate_internal_full_trace, project_view, replay_internal_full_trace, setup_match,
    validate_command, HighCardDuelSeat, HighCardDuelState, Phase, SetupOptions,
    STANDARD_DECK_CARD_COUNT,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn actor(seat: HighCardDuelSeat) -> Actor {
    Actor {
        seat_id: seats()[seat.index()].clone(),
    }
}

#[test]
fn card_conservation_holds_across_commit_reveal_refill_transitions() {
    let mut state =
        setup_match(Seed(2026), &seats(), &SetupOptions::default()).expect("setup succeeds");

    assert_card_conservation(&state);

    for _ in 0..3 {
        let lead = state.lead_seat;
        let reply = lead.other();
        apply_first_card(&mut state, lead);
        assert_card_conservation(&state);
        apply_first_card(&mut state, reply);
        assert_card_conservation(&state);
        if state.phase == Phase::Terminal {
            break;
        }
    }
}

#[test]
fn public_projection_never_grows_hidden_fields_across_seeds() {
    for seed in 0..32 {
        let mut state =
            setup_match(Seed(seed), &seats(), &SetupOptions::default()).expect("setup succeeds");
        let mut previous_round = state.round_number;

        assert_observer_projection_has_no_hidden_ids(&state);

        while state.phase != Phase::Terminal {
            let actor = active_commit_seat(&state).expect("non-terminal state has active actor");
            apply_first_card(&mut state, actor);
            assert!(state.round_number >= previous_round);
            previous_round = state.round_number;
            assert_card_conservation(&state);
            assert_observer_projection_has_no_hidden_ids(&state);
        }
    }
}

#[test]
fn committed_cards_reveal_exactly_once() {
    let mut state =
        setup_match(Seed(2027), &seats(), &SetupOptions::default()).expect("setup succeeds");

    while state.phase != Phase::Terminal {
        let actor = active_commit_seat(&state).expect("non-terminal state has active actor");
        apply_first_card(&mut state, actor);
    }

    let revealed_cards = state
        .revealed_history
        .iter()
        .flat_map(|round| [round.seat_0_card.stable_id(), round.seat_1_card.stable_id()])
        .collect::<Vec<_>>();
    let unique_revealed = revealed_cards.iter().collect::<BTreeSet<_>>();

    assert_eq!(
        revealed_cards.len(),
        usize::from(state.variant.round_limit) * 2
    );
    assert_eq!(revealed_cards.len(), unique_revealed.len());
}

#[test]
fn public_replay_export_never_contains_unrevealed_internal_card_identities() {
    for seed in 0..16 {
        let trace = generate_internal_full_trace(seed);
        let replay = replay_internal_full_trace(&trace);
        let export = export_public_observer_replay(&trace);
        let export_json = export.to_json();

        assert!(!export_json.contains("\"seed\""));
        assert!(!export_json.contains("commit/hcd:r"));
        for hidden_id in hidden_card_ids(&replay.final_state) {
            assert!(
                !export_json.contains(&hidden_id),
                "public export for seed {seed} leaked {hidden_id}"
            );
        }
    }
}

fn apply_first_card(state: &mut high_card_duel::HighCardDuelState, seat: HighCardDuelSeat) {
    let card = state.hand_for(seat)[0];
    let command = CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec![commit_segment(card)],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    };
    let action = validate_command(state, &command).expect("command validates");
    apply_action(state, action);
}

fn assert_card_conservation(state: &high_card_duel::HighCardDuelState) {
    let all_cards = state.internal_card_order();
    let unique_cards = all_cards.iter().collect::<BTreeSet<_>>();

    assert_eq!(all_cards.len(), STANDARD_DECK_CARD_COUNT as usize);
    assert_eq!(unique_cards.len(), STANDARD_DECK_CARD_COUNT as usize);
}

fn assert_observer_projection_has_no_hidden_ids(state: &HighCardDuelState) {
    let view = project_view(state, &Viewer { seat_id: None });
    let summary = view.stable_summary();

    for hidden_id in hidden_card_ids(state) {
        assert!(
            !summary.contains(&hidden_id),
            "observer projection leaked {hidden_id}: {summary}"
        );
    }
}

fn hidden_card_ids(state: &HighCardDuelState) -> Vec<String> {
    let mut hidden = Vec::new();

    for hand in &state.hands {
        hidden.extend(hand.iter().map(|card| card.stable_id()));
    }
    hidden.extend(
        state
            .commitments
            .iter()
            .flatten()
            .map(|card| card.stable_id()),
    );
    hidden.extend(state.deck.iter().map(|card| card.stable_id()));

    hidden
}
