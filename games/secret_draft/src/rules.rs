use engine_core::{Diagnostic, EffectEnvelope};

use crate::{
    actions::{self, SecretDraftAction},
    effects::{public_effect, ConflictSummary, SecretDraftEffect, TieBreakSummary},
    ids::{DraftItemId, DraftThread, SecretDraftSeat},
    state::{Phase, RevealedRound, SecretDraftState, TerminalOutcome},
};

pub type ValidatedAction = actions::ValidatedAction;

pub fn legal_actions(state: &SecretDraftState) -> Vec<SecretDraftAction> {
    let mut actions = Vec::new();
    if state.phase == Phase::Terminal || state.terminal_outcome.is_some() {
        return actions;
    }

    for actor in SecretDraftSeat::ALL {
        if state.seat_committed(actor) {
            continue;
        }
        for item in &state.visible_pool {
            actions.push(SecretDraftAction { actor, item: *item });
        }
    }
    actions
}

pub fn validate_action(
    state: &SecretDraftState,
    action: SecretDraftAction,
) -> Result<ValidatedAction, Diagnostic> {
    if state.phase == Phase::Terminal || state.terminal_outcome.is_some() {
        return Err(actions::terminal_diagnostic());
    }
    if state.seat_committed(action.actor) {
        return Err(actions::already_committed_diagnostic());
    }
    if !state.visible_pool.contains(&action.item) {
        return Err(actions::unavailable_item_diagnostic());
    }
    Ok(ValidatedAction {
        actor: action.actor,
        item: action.item,
    })
}

pub fn apply_action(
    state: &mut SecretDraftState,
    action: ValidatedAction,
) -> Result<Vec<EffectEnvelope<SecretDraftEffect>>, Diagnostic> {
    if state.phase == Phase::Terminal || state.terminal_outcome.is_some() {
        return Err(actions::terminal_diagnostic());
    }
    if state.seat_committed(action.actor) {
        return Err(actions::already_committed_diagnostic());
    }
    if !state.visible_pool.contains(&action.item) {
        return Err(actions::unavailable_item_diagnostic());
    }

    state.set_commitment_for_internal(action.actor, action.item);

    let mut effects = vec![
        public_effect(SecretDraftEffect::CommitmentPlaced {
            seat: action.actor,
            round: state.round_number,
        }),
        public_effect(SecretDraftEffect::OwnCommitAccepted {
            seat: action.actor,
            round: state.round_number,
        }),
        public_effect(SecretDraftEffect::PendingSeatsChanged {
            round: state.round_number,
            seat_0_committed: state.seat_committed(SecretDraftSeat::Seat0),
            seat_1_committed: state.seat_committed(SecretDraftSeat::Seat1),
        }),
    ];

    if state.seat_committed(SecretDraftSeat::Seat0) && state.seat_committed(SecretDraftSeat::Seat1)
    {
        resolve_reveal_batch(state, &mut effects);
    } else {
        state.freshness_token = state.freshness_token.next();
    }

    Ok(effects)
}

fn resolve_reveal_batch(
    state: &mut SecretDraftState,
    effects: &mut Vec<EffectEnvelope<SecretDraftEffect>>,
) {
    let round = state.round_number;
    let seat_0_choice = state
        .commitment_for_internal(SecretDraftSeat::Seat0)
        .expect("reveal resolution requires seat_0 commitment");
    let seat_1_choice = state
        .commitment_for_internal(SecretDraftSeat::Seat1)
        .expect("reveal resolution requires seat_1 commitment");

    effects.push(public_effect(SecretDraftEffect::RevealBatchStarted {
        round,
        group_id: format!("secret_draft_round_{round}_reveal"),
    }));
    effects.push(public_effect(SecretDraftEffect::ChoicesRevealed {
        round,
        seat_0_item: seat_0_choice,
        seat_1_item: seat_1_choice,
    }));

    let resolution = resolve_awards(state, seat_0_choice, seat_1_choice);
    state.drafted[SecretDraftSeat::Seat0.index()].push(resolution.seat_0_award);
    state.drafted[SecretDraftSeat::Seat1.index()].push(resolution.seat_1_award);
    remove_awarded_items(state, resolution.removed_items);

    if let Some(conflict) = resolution.conflict {
        state.priority_conflict_wins[conflict.priority_seat.index()] =
            state.priority_conflict_wins[conflict.priority_seat.index()].saturating_add(1);
        let fallback_seat = conflict.priority_seat.other();
        state.fallback_awards[fallback_seat.index()] =
            state.fallback_awards[fallback_seat.index()].saturating_add(1);
    }

    state.revealed_history.push(RevealedRound {
        round_number: round,
        seat_0_choice,
        seat_1_choice,
        seat_0_award: resolution.seat_0_award,
        seat_1_award: resolution.seat_1_award,
        priority_seat: state.priority_seat,
        contested: resolution.conflict.is_some(),
    });
    state.clear_commitments_internal();

    effects.push(public_effect(SecretDraftEffect::DraftResolved {
        round,
        seat_0_award: resolution.seat_0_award,
        seat_1_award: resolution.seat_1_award,
        removed_items: resolution.removed_items,
        conflict: resolution.conflict,
    }));
    effects.push(public_effect(SecretDraftEffect::PoolChanged {
        remaining_count: state.visible_pool.len() as u8,
    }));

    let terminal = round >= state.variant.round_count;
    let score_context = if terminal {
        ScoreContext::Terminal
    } else {
        ScoreContext::InProgress
    };
    let score_0 = score_for(SecretDraftSeat::Seat0, state, score_context);
    let score_1 = score_for(SecretDraftSeat::Seat1, state, score_context);
    state.scores = [score_0.total, score_1.total];
    let tie_break_summary = tie_break_summary(state, [score_0, score_1]);
    effects.push(public_effect(SecretDraftEffect::ScoreChanged {
        scores: state.scores,
        tie_break_summary,
    }));

    if terminal {
        state.phase = Phase::Terminal;
        let outcome = determine_terminal_outcome_from_summary(tie_break_summary);
        state.terminal_outcome = Some(outcome);
        effects.push(public_effect(SecretDraftEffect::Terminal {
            outcome,
            final_scores: state.scores,
            tie_break_summary,
        }));
    } else {
        state.round_number = state.round_number.saturating_add(1);
        state.priority_seat = state.priority_seat.other();
        effects.push(public_effect(SecretDraftEffect::RoundAdvanced {
            next_round: state.round_number,
            priority_seat: state.priority_seat,
        }));
    }

    state.freshness_token = state.freshness_token.next();
}

struct AwardResolution {
    seat_0_award: DraftItemId,
    seat_1_award: DraftItemId,
    removed_items: [DraftItemId; 2],
    conflict: Option<ConflictSummary>,
}

fn resolve_awards(
    state: &SecretDraftState,
    seat_0_choice: DraftItemId,
    seat_1_choice: DraftItemId,
) -> AwardResolution {
    if seat_0_choice != seat_1_choice {
        return AwardResolution {
            seat_0_award: seat_0_choice,
            seat_1_award: seat_1_choice,
            removed_items: [seat_0_choice, seat_1_choice],
            conflict: None,
        };
    }

    let contested_item = seat_0_choice;
    let fallback_item = state
        .visible_pool
        .iter()
        .copied()
        .find(|item| *item != contested_item)
        .expect("contested reveal requires a fallback item");
    let (seat_0_award, seat_1_award) = match state.priority_seat {
        SecretDraftSeat::Seat0 => (contested_item, fallback_item),
        SecretDraftSeat::Seat1 => (fallback_item, contested_item),
    };

    AwardResolution {
        seat_0_award,
        seat_1_award,
        removed_items: [contested_item, fallback_item],
        conflict: Some(ConflictSummary {
            contested_item,
            priority_seat: state.priority_seat,
            fallback_item,
        }),
    }
}

fn remove_awarded_items(state: &mut SecretDraftState, removed_items: [DraftItemId; 2]) {
    state
        .visible_pool
        .retain(|item| !removed_items.contains(item));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScoreContext {
    InProgress,
    Terminal,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ScoreBreakdown {
    pub base_value: u32,
    pub complete_sets: u8,
    pub high_thread_bonus_count: u8,
    pub conflict_discipline_bonus: u8,
    pub highest_single_value: u8,
    pub distinct_threads: u8,
    pub total: u32,
}

fn score_for(
    seat: SecretDraftSeat,
    state: &SecretDraftState,
    context: ScoreContext,
) -> ScoreBreakdown {
    score_items(
        &state.drafted[seat.index()],
        state.fallback_awards[seat.index()],
        context,
    )
}

fn score_items(
    items: &[DraftItemId],
    fallback_awards: u8,
    context: ScoreContext,
) -> ScoreBreakdown {
    let base_value = items.iter().map(|item| u32::from(item.value())).sum();
    let ember = thread_count(items, DraftThread::Ember);
    let tide = thread_count(items, DraftThread::Tide);
    let grove = thread_count(items, DraftThread::Grove);
    let complete_sets = ember.min(tide).min(grove);
    let high_thread_bonus_count = [ember, tide, grove]
        .iter()
        .filter(|count| **count >= 3)
        .count() as u8;
    let conflict_discipline_bonus = match context {
        ScoreContext::Terminal if fallback_awards == 0 => 1,
        _ => 0,
    };
    let highest_single_value = items.iter().map(|item| item.value()).max().unwrap_or(0);
    let distinct_threads = [ember, tide, grove]
        .iter()
        .filter(|count| **count > 0)
        .count() as u8;
    let total = base_value
        + u32::from(complete_sets) * 3
        + u32::from(high_thread_bonus_count) * 2
        + u32::from(conflict_discipline_bonus);

    ScoreBreakdown {
        base_value,
        complete_sets,
        high_thread_bonus_count,
        conflict_discipline_bonus,
        highest_single_value,
        distinct_threads,
        total,
    }
}

fn thread_count(items: &[DraftItemId], thread: DraftThread) -> u8 {
    items.iter().filter(|item| item.thread() == thread).count() as u8
}

fn tie_break_summary(state: &SecretDraftState, scores: [ScoreBreakdown; 2]) -> TieBreakSummary {
    TieBreakSummary {
        scores: [scores[0].total, scores[1].total],
        complete_sets: [scores[0].complete_sets, scores[1].complete_sets],
        highest_single_values: [
            scores[0].highest_single_value,
            scores[1].highest_single_value,
        ],
        distinct_threads: [scores[0].distinct_threads, scores[1].distinct_threads],
        priority_conflict_wins: state.priority_conflict_wins,
    }
}

pub fn terminal_tie_break_summary(state: &SecretDraftState) -> TieBreakSummary {
    let scores = [
        score_for(SecretDraftSeat::Seat0, state, ScoreContext::Terminal),
        score_for(SecretDraftSeat::Seat1, state, ScoreContext::Terminal),
    ];
    tie_break_summary(state, scores)
}

pub fn determine_terminal_outcome(state: &SecretDraftState) -> TerminalOutcome {
    determine_terminal_outcome_from_summary(terminal_tie_break_summary(state))
}

pub fn determine_terminal_outcome_from_summary(summary: TieBreakSummary) -> TerminalOutcome {
    if let Some(winner) = compare_higher(summary.scores) {
        return TerminalOutcome::Win { seat: winner };
    }
    if let Some(winner) = compare_higher(summary.complete_sets) {
        return TerminalOutcome::Win { seat: winner };
    }
    if let Some(winner) = compare_higher(summary.highest_single_values) {
        return TerminalOutcome::Win { seat: winner };
    }
    if let Some(winner) = compare_higher(summary.distinct_threads) {
        return TerminalOutcome::Win { seat: winner };
    }
    if summary.priority_conflict_wins[0] < summary.priority_conflict_wins[1] {
        return TerminalOutcome::Win {
            seat: SecretDraftSeat::Seat0,
        };
    }
    if summary.priority_conflict_wins[1] < summary.priority_conflict_wins[0] {
        return TerminalOutcome::Win {
            seat: SecretDraftSeat::Seat1,
        };
    }
    TerminalOutcome::Draw
}

fn compare_higher<T>(values: [T; 2]) -> Option<SecretDraftSeat>
where
    T: Ord,
{
    if values[0] > values[1] {
        Some(SecretDraftSeat::Seat0)
    } else if values[1] > values[0] {
        Some(SecretDraftSeat::Seat1)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{setup::SetupOptions, setup_match};
    use engine_core::SeatId;

    fn standard_state() -> SecretDraftState {
        setup_match(
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds")
    }

    fn validated(actor: SecretDraftSeat, item: DraftItemId) -> ValidatedAction {
        ValidatedAction { actor, item }
    }

    fn payloads(effects: &[EffectEnvelope<SecretDraftEffect>]) -> Vec<SecretDraftEffect> {
        effects
            .iter()
            .map(|effect| effect.payload.clone())
            .collect()
    }

    #[test]
    fn first_commit_emits_pending_only_effects_without_item_id() {
        let mut state = standard_state();
        let effects = apply_action(
            &mut state,
            validated(SecretDraftSeat::Seat0, DraftItemId::Ember4),
        )
        .expect("first commit applies");

        assert_eq!(effects.len(), 3);
        assert_eq!(
            payloads(&effects),
            vec![
                SecretDraftEffect::CommitmentPlaced {
                    seat: SecretDraftSeat::Seat0,
                    round: 1
                },
                SecretDraftEffect::OwnCommitAccepted {
                    seat: SecretDraftSeat::Seat0,
                    round: 1
                },
                SecretDraftEffect::PendingSeatsChanged {
                    round: 1,
                    seat_0_committed: true,
                    seat_1_committed: false
                }
            ]
        );
        assert_eq!(
            state.commitment_for_internal(SecretDraftSeat::Seat0),
            Some(DraftItemId::Ember4)
        );
        assert!(!format!("{effects:?}").contains("Ember4"));
        assert!(!format!("{effects:?}").contains("ember_4"));
    }

    #[test]
    fn second_commit_emits_reveal_batch_in_fixed_order() {
        let mut state = standard_state();
        apply_action(
            &mut state,
            validated(SecretDraftSeat::Seat0, DraftItemId::Ember1),
        )
        .expect("first commit applies");
        let effects = apply_action(
            &mut state,
            validated(SecretDraftSeat::Seat1, DraftItemId::Tide1),
        )
        .expect("second commit applies");
        let payloads = payloads(&effects);

        assert!(matches!(
            payloads[0],
            SecretDraftEffect::CommitmentPlaced { .. }
        ));
        assert!(matches!(
            payloads[1],
            SecretDraftEffect::OwnCommitAccepted { .. }
        ));
        assert!(matches!(
            payloads[2],
            SecretDraftEffect::PendingSeatsChanged { .. }
        ));
        assert!(matches!(
            payloads[3],
            SecretDraftEffect::RevealBatchStarted { .. }
        ));
        assert_eq!(
            payloads[4],
            SecretDraftEffect::ChoicesRevealed {
                round: 1,
                seat_0_item: DraftItemId::Ember1,
                seat_1_item: DraftItemId::Tide1
            }
        );
        assert!(matches!(
            payloads[5],
            SecretDraftEffect::DraftResolved { .. }
        ));
        assert!(matches!(payloads[6], SecretDraftEffect::PoolChanged { .. }));
        assert!(matches!(
            payloads[7],
            SecretDraftEffect::ScoreChanged { .. }
        ));
        assert!(matches!(
            payloads[8],
            SecretDraftEffect::RoundAdvanced { .. }
        ));
        assert_eq!(state.visible_pool.len(), 10);
        assert!(state.empty_commitments());
    }

    #[test]
    fn conflict_fallback_is_priority_then_lowest_remaining_item() {
        let mut state = standard_state();
        apply_action(
            &mut state,
            validated(SecretDraftSeat::Seat0, DraftItemId::Ember4),
        )
        .expect("first commit applies");
        let effects = apply_action(
            &mut state,
            validated(SecretDraftSeat::Seat1, DraftItemId::Ember4),
        )
        .expect("second commit applies");

        assert_eq!(state.drafted[0], vec![DraftItemId::Ember4]);
        assert_eq!(state.drafted[1], vec![DraftItemId::Ember1]);
        assert!(!state.visible_pool.contains(&DraftItemId::Ember4));
        assert!(!state.visible_pool.contains(&DraftItemId::Ember1));
        assert_eq!(state.priority_conflict_wins, [1, 0]);
        assert_eq!(state.fallback_awards, [0, 1]);
        assert!(payloads(&effects).iter().any(|effect| matches!(
            effect,
            SecretDraftEffect::DraftResolved {
                conflict: Some(ConflictSummary {
                    contested_item: DraftItemId::Ember4,
                    priority_seat: SecretDraftSeat::Seat0,
                    fallback_item: DraftItemId::Ember1
                }),
                ..
            }
        )));
    }

    #[test]
    fn scoring_components_match_rules() {
        let items = [
            DraftItemId::Ember4,
            DraftItemId::Ember3,
            DraftItemId::Ember2,
            DraftItemId::Tide1,
            DraftItemId::Grove1,
        ];
        let score = score_items(&items, 0, ScoreContext::Terminal);

        assert_eq!(score.base_value, 11);
        assert_eq!(score.complete_sets, 1);
        assert_eq!(score.high_thread_bonus_count, 1);
        assert_eq!(score.conflict_discipline_bonus, 1);
        assert_eq!(score.total, 17);
    }

    #[test]
    fn terminal_tie_break_ladder_reaches_each_rung_and_draw() {
        let mut state = standard_state();
        state.drafted = [vec![DraftItemId::Ember4], vec![DraftItemId::Ember3]];
        assert_eq!(
            determine_terminal_outcome(&state),
            TerminalOutcome::Win {
                seat: SecretDraftSeat::Seat0
            }
        );

        state.drafted = [
            vec![DraftItemId::Ember1, DraftItemId::Tide1, DraftItemId::Grove1],
            vec![DraftItemId::Ember3],
        ];
        assert_eq!(
            determine_terminal_outcome(&state),
            TerminalOutcome::Win {
                seat: SecretDraftSeat::Seat0
            }
        );

        state.drafted = [
            vec![DraftItemId::Ember4],
            vec![DraftItemId::Ember2, DraftItemId::Tide2],
        ];
        assert_eq!(
            determine_terminal_outcome(&state),
            TerminalOutcome::Win {
                seat: SecretDraftSeat::Seat0
            }
        );

        state.drafted = [
            vec![DraftItemId::Ember2, DraftItemId::Ember1],
            vec![DraftItemId::Ember2, DraftItemId::Tide1],
        ];
        assert_eq!(
            determine_terminal_outcome(&state),
            TerminalOutcome::Win {
                seat: SecretDraftSeat::Seat1
            }
        );

        state.drafted = [
            vec![DraftItemId::Ember2, DraftItemId::Tide2],
            vec![DraftItemId::Ember2, DraftItemId::Tide2],
        ];
        state.priority_conflict_wins = [1, 0];
        assert_eq!(
            determine_terminal_outcome(&state),
            TerminalOutcome::Win {
                seat: SecretDraftSeat::Seat1
            }
        );

        state.priority_conflict_wins = [0, 0];
        assert_eq!(determine_terminal_outcome(&state), TerminalOutcome::Draw);
    }

    #[test]
    fn terminal_occurs_after_six_resolved_rounds_with_two_items_removed_each_round() {
        let mut state = standard_state();
        for round in 1..=6 {
            let first = state.visible_pool[0];
            let second = state.visible_pool[1];
            let before_count = state.visible_pool.len();
            apply_action(&mut state, validated(SecretDraftSeat::Seat0, first))
                .expect("first commit applies");
            apply_action(&mut state, validated(SecretDraftSeat::Seat1, second))
                .expect("second commit applies");

            assert_eq!(state.visible_pool.len(), before_count - 2);
            assert_eq!(state.revealed_history.len(), round as usize);
        }

        assert_eq!(state.phase, Phase::Terminal);
        assert!(state.terminal_outcome.is_some());
        assert_eq!(state.visible_pool.len(), 0);
    }
}
