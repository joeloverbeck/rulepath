use std::collections::BTreeSet;

use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed};
use high_card_duel::{
    apply_action, commit_segment, setup_match, validate_command, HighCardDuelSeat, Phase,
    SetupOptions, STANDARD_DECK_CARD_COUNT,
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
