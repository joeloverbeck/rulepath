use engine_core::{EffectEnvelope, SeatId};

use crate::{
    ids::{PlainTricksSeat, TrickCardId, ACTION_PLAY},
    state::{CompletedTrick, PlainTricksState, TerminalOutcome, TrickCounts, TrickPlay},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlainTricksEffect {
    DealStarted {
        round_index: u8,
        cards_per_seat: u8,
        tail_count: u8,
    },
    HandDealt {
        owner: PlainTricksSeat,
        cards: Vec<TrickCardId>,
    },
    DealCompleted {
        round_index: u8,
        cards_per_seat: u8,
        tail_count: u8,
        leader: PlainTricksSeat,
    },
    CardPlayed {
        seat: PlainTricksSeat,
        card: TrickCardId,
        round_index: u8,
        trick_index: u8,
        led: bool,
    },
    TrickResolved {
        round_index: u8,
        trick_index: u8,
        plays: [TrickPlay; 2],
        winner: PlainTricksSeat,
        trick_counts: TrickCounts,
    },
    RoundScored {
        round_index: u8,
        round_counts: TrickCounts,
        total_counts: TrickCounts,
    },
    DealRotated {
        round_index: u8,
        leader: PlainTricksSeat,
    },
    MatchResolved {
        totals: TrickCounts,
        decisive_cause: String,
    },
    Terminal {
        outcome: TerminalOutcome,
        decisive_cause: String,
    },
    BotChoseActionPublic {
        policy_id: String,
        action_family: String,
    },
}

pub fn public_effect(payload: PlainTricksEffect) -> EffectEnvelope<PlainTricksEffect> {
    EffectEnvelope::public(payload)
}

pub fn private_effect(
    owner_seat_id: SeatId,
    payload: PlainTricksEffect,
) -> EffectEnvelope<PlainTricksEffect> {
    EffectEnvelope::private_to(owner_seat_id, payload)
}

pub fn setup_effects(state: &PlainTricksState) -> Vec<EffectEnvelope<PlainTricksEffect>> {
    let cards_per_seat = state.hands[PlainTricksSeat::Seat0.index()].len() as u8;
    vec![
        deal_started_effect(state.round_index, cards_per_seat, state.tail.len() as u8),
        hand_dealt_effect(
            PlainTricksSeat::Seat0,
            state.seats[PlainTricksSeat::Seat0.index()].clone(),
            state.hands[PlainTricksSeat::Seat0.index()].clone(),
        ),
        hand_dealt_effect(
            PlainTricksSeat::Seat1,
            state.seats[PlainTricksSeat::Seat1.index()].clone(),
            state.hands[PlainTricksSeat::Seat1.index()].clone(),
        ),
        deal_completed_effect(
            state.round_index,
            cards_per_seat,
            state.tail.len() as u8,
            state.round_leader,
        ),
    ]
}

pub fn deal_started_effect(
    round_index: u8,
    cards_per_seat: u8,
    tail_count: u8,
) -> EffectEnvelope<PlainTricksEffect> {
    public_effect(PlainTricksEffect::DealStarted {
        round_index,
        cards_per_seat,
        tail_count,
    })
}

pub fn hand_dealt_effect(
    owner: PlainTricksSeat,
    owner_seat_id: SeatId,
    cards: Vec<TrickCardId>,
) -> EffectEnvelope<PlainTricksEffect> {
    private_effect(owner_seat_id, PlainTricksEffect::HandDealt { owner, cards })
}

pub fn deal_completed_effect(
    round_index: u8,
    cards_per_seat: u8,
    tail_count: u8,
    leader: PlainTricksSeat,
) -> EffectEnvelope<PlainTricksEffect> {
    public_effect(PlainTricksEffect::DealCompleted {
        round_index,
        cards_per_seat,
        tail_count,
        leader,
    })
}

pub fn card_played_effect(
    seat: PlainTricksSeat,
    card: TrickCardId,
    round_index: u8,
    trick_index: u8,
    led: bool,
) -> EffectEnvelope<PlainTricksEffect> {
    public_effect(PlainTricksEffect::CardPlayed {
        seat,
        card,
        round_index,
        trick_index,
        led,
    })
}

pub fn trick_resolved_effect(completed: CompletedTrick) -> EffectEnvelope<PlainTricksEffect> {
    public_effect(PlainTricksEffect::TrickResolved {
        round_index: completed.round_index,
        trick_index: completed.trick_index,
        plays: completed.plays,
        winner: completed.winner,
        trick_counts: completed.trick_counts_after,
    })
}

pub fn round_scored_effect(
    round_index: u8,
    round_counts: TrickCounts,
    total_counts: TrickCounts,
) -> EffectEnvelope<PlainTricksEffect> {
    public_effect(PlainTricksEffect::RoundScored {
        round_index,
        round_counts,
        total_counts,
    })
}

pub fn deal_rotated_effect(
    round_index: u8,
    leader: PlainTricksSeat,
) -> EffectEnvelope<PlainTricksEffect> {
    public_effect(PlainTricksEffect::DealRotated {
        round_index,
        leader,
    })
}

pub fn match_resolved_effect(totals: TrickCounts) -> EffectEnvelope<PlainTricksEffect> {
    public_effect(PlainTricksEffect::MatchResolved {
        totals,
        decisive_cause: decisive_cause(totals),
    })
}

pub fn terminal_effect(outcome: TerminalOutcome) -> EffectEnvelope<PlainTricksEffect> {
    public_effect(PlainTricksEffect::Terminal {
        outcome,
        decisive_cause: decisive_cause(match outcome {
            TerminalOutcome::TrickWin { totals, .. } | TerminalOutcome::Split { totals, .. } => {
                totals
            }
        }),
    })
}

pub fn bot_chose_action_public_effect(
    policy_id: impl Into<String>,
) -> EffectEnvelope<PlainTricksEffect> {
    public_effect(PlainTricksEffect::BotChoseActionPublic {
        policy_id: policy_id.into(),
        action_family: ACTION_PLAY.to_owned(),
    })
}

pub fn decisive_cause(totals: TrickCounts) -> String {
    if totals.seat_0 == totals.seat_1 {
        format!("split:{}-{}", totals.seat_0, totals.seat_1)
    } else if totals.seat_0 > totals.seat_1 {
        format!("seat_0_total_tricks:{}-{}", totals.seat_0, totals.seat_1)
    } else {
        format!("seat_1_total_tricks:{}-{}", totals.seat_0, totals.seat_1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{setup_match, SetupOptions};
    use engine_core::{SeatId, Seed, VisibilityScope};

    #[test]
    fn private_deal_effects_are_scoped_to_owner_and_public_setup_has_counts_only() {
        let state = setup_match(
            Seed(5),
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds");
        let effects = setup_effects(&state);

        assert_eq!(effects[0].visibility, VisibilityScope::Public);
        assert_eq!(
            effects[1].visibility,
            VisibilityScope::PrivateToSeat(SeatId("seat_0".to_owned()))
        );
        assert_eq!(
            effects[2].visibility,
            VisibilityScope::PrivateToSeat(SeatId("seat_1".to_owned()))
        );
        assert_eq!(effects[3].visibility, VisibilityScope::Public);

        let public_text = format!("{:?}{:?}", effects[0], effects[3]);
        for card in TrickCardId::ALL {
            assert!(!public_text.contains(card.as_str()));
            assert!(!public_text.contains(&card.label()));
        }
    }

    #[test]
    fn public_play_and_trick_effects_contain_only_played_cards() {
        let leader = TrickPlay {
            seat: PlainTricksSeat::Seat0,
            card: TrickCardId::Gale6,
        };
        let follower = TrickPlay {
            seat: PlainTricksSeat::Seat1,
            card: TrickCardId::Gale1,
        };
        let completed = CompletedTrick {
            round_index: 0,
            trick_index: 0,
            leader: PlainTricksSeat::Seat0,
            plays: [leader, follower],
            winner: PlainTricksSeat::Seat0,
            trick_counts_after: TrickCounts {
                seat_0: 1,
                seat_1: 0,
            },
        };

        assert_eq!(
            card_played_effect(PlainTricksSeat::Seat0, TrickCardId::Gale6, 0, 0, true).visibility,
            VisibilityScope::Public
        );
        let trick = trick_resolved_effect(completed);
        assert_eq!(trick.visibility, VisibilityScope::Public);
        assert!(format!("{trick:?}").contains("Gale6"));
        assert!(!format!("{trick:?}").contains("Ember6"));
    }

    #[test]
    fn terminal_effects_carry_decisive_cause() {
        let totals = TrickCounts {
            seat_0: 7,
            seat_1: 5,
        };
        let match_effect = match_resolved_effect(totals);
        let terminal = terminal_effect(TerminalOutcome::TrickWin {
            winner: PlainTricksSeat::Seat0,
            totals,
        });

        assert!(format!("{match_effect:?}").contains("seat_0_total_tricks:7-5"));
        assert!(format!("{terminal:?}").contains("seat_0_total_tricks:7-5"));
    }

    #[test]
    fn bot_public_effect_names_policy_and_action_family_only() {
        let effect = bot_chose_action_public_effect("plain-tricks-level2-v1");

        assert_eq!(effect.visibility, VisibilityScope::Public);
        assert!(format!("{effect:?}").contains("plain-tricks-level2-v1"));
        assert!(format!("{effect:?}").contains("play"));
        assert!(!format!("{effect:?}").contains("gale_"));
        assert!(!format!("{effect:?}").contains("river_"));
        assert!(!format!("{effect:?}").contains("ember_"));
    }
}
