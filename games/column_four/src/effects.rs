use engine_core::EffectEnvelope;

use crate::{
    ids::{CellId, ColumnFourSeat, ColumnId, RowId},
    state::{TerminalOutcome, WinningLine},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ColumnFourEffect {
    DropAccepted {
        seat: ColumnFourSeat,
        column: ColumnId,
        ply: u8,
    },
    PieceLanded {
        seat: ColumnFourSeat,
        column: ColumnId,
        row: RowId,
        cell: CellId,
        display_from_anchor: String,
        display_to_anchor: String,
    },
    ActivePlayerChanged {
        previous_seat: ColumnFourSeat,
        active_seat: ColumnFourSeat,
        ply: u8,
    },
    WinDetected {
        winning_seat: ColumnFourSeat,
        line: WinningLine,
    },
    DrawDetected {
        final_ply: u8,
        full_board: bool,
    },
    GameEnded {
        outcome: TerminalOutcome,
        final_ply: u8,
        terminal_hash_ref: String,
    },
    BotChoseAction {
        level: u8,
        policy_id: String,
        action_id: String,
        column: ColumnId,
        rationale: String,
    },
}

pub fn public_effect(payload: ColumnFourEffect) -> EffectEnvelope<ColumnFourEffect> {
    EffectEnvelope::public(payload)
}

pub fn bot_chose_action_effect(
    level: u8,
    policy_id: impl Into<String>,
    action_id: impl Into<String>,
    column: ColumnId,
    rationale: impl Into<String>,
) -> EffectEnvelope<ColumnFourEffect> {
    public_effect(ColumnFourEffect::BotChoseAction {
        level,
        policy_id: policy_id.into(),
        action_id: action_id.into(),
        column,
        rationale: rationale.into(),
    })
}

pub fn display_from_anchor(column: ColumnId) -> String {
    format!("column:{}:top", column.as_str())
}

pub fn display_to_anchor(cell: CellId) -> String {
    format!("cell:{}", cell.as_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rules::{apply_action, ValidatedAction},
        setup::setup_match,
        ColumnFourState,
    };
    use engine_core::{SeatId, Seed, VisibilityScope};

    fn state() -> ColumnFourState {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        setup_match(Seed(1), &seats, &Default::default()).expect("setup succeeds")
    }

    #[test]
    fn public_effect_uses_public_visibility_and_preserves_payload() {
        let payload = ColumnFourEffect::DropAccepted {
            seat: ColumnFourSeat::Seat0,
            column: ColumnId::C4,
            ply: 1,
        };

        let effect = public_effect(payload.clone());

        assert_eq!(effect.visibility, VisibilityScope::Public);
        assert_eq!(effect.payload, payload);
    }

    #[test]
    fn non_terminal_drop_effects_are_ordered_and_include_rust_landing_anchor() {
        let mut state = state();

        let effects = apply_action(
            &mut state,
            ValidatedAction {
                actor: ColumnFourSeat::Seat0,
                column: ColumnId::C4,
            },
        );

        assert_eq!(effects.len(), 3);
        assert_eq!(effects[0].visibility, VisibilityScope::Public);
        assert_eq!(
            effects[0].payload,
            ColumnFourEffect::DropAccepted {
                seat: ColumnFourSeat::Seat0,
                column: ColumnId::C4,
                ply: 1,
            }
        );
        assert_eq!(
            effects[1].payload,
            ColumnFourEffect::PieceLanded {
                seat: ColumnFourSeat::Seat0,
                column: ColumnId::C4,
                row: RowId::R1,
                cell: CellId::new(RowId::R1, ColumnId::C4),
                display_from_anchor: "column:c4:top".to_owned(),
                display_to_anchor: "cell:r1c4".to_owned(),
            }
        );
        assert_eq!(
            effects[2].payload,
            ColumnFourEffect::ActivePlayerChanged {
                previous_seat: ColumnFourSeat::Seat0,
                active_seat: ColumnFourSeat::Seat1,
                ply: 1,
            }
        );
    }

    #[test]
    fn terminal_effects_report_win_and_draw_without_turn_advance() {
        let mut win_state = state();
        for column in [ColumnId::C1, ColumnId::C2, ColumnId::C3] {
            win_state.set_occupancy(
                CellId::new(RowId::R1, column),
                crate::CellOccupancy::Occupied(ColumnFourSeat::Seat0),
            );
        }

        let win_effects = apply_action(
            &mut win_state,
            ValidatedAction {
                actor: ColumnFourSeat::Seat0,
                column: ColumnId::C4,
            },
        );

        assert!(matches!(
            win_effects[2].payload,
            ColumnFourEffect::WinDetected {
                winning_seat: ColumnFourSeat::Seat0,
                ..
            }
        ));
        assert!(matches!(
            win_effects[3].payload,
            ColumnFourEffect::GameEnded {
                outcome: TerminalOutcome::Win { .. },
                final_ply: 1,
                ..
            }
        ));
        assert!(!win_effects
            .iter()
            .any(|effect| matches!(effect.payload, ColumnFourEffect::ActivePlayerChanged { .. })));

        let mut draw_state = state();
        for cell in CellId::ALL {
            draw_state.set_occupancy(cell, crate::CellOccupancy::Occupied(ColumnFourSeat::Seat0));
        }
        draw_state.set_occupancy(
            CellId::new(RowId::R6, ColumnId::C7),
            crate::CellOccupancy::Empty,
        );
        draw_state.ply_count = 41;
        draw_state.terminal_outcome = None;

        let draw_effects = apply_action(
            &mut draw_state,
            ValidatedAction {
                actor: ColumnFourSeat::Seat1,
                column: ColumnId::C7,
            },
        );

        assert!(matches!(
            draw_effects[2].payload,
            ColumnFourEffect::DrawDetected {
                final_ply: 42,
                full_board: true,
            }
        ));
        assert!(matches!(
            draw_effects[3].payload,
            ColumnFourEffect::GameEnded {
                outcome: TerminalOutcome::Draw,
                final_ply: 42,
                ..
            }
        ));
    }

    #[test]
    fn bot_chose_action_effect_is_public_prose_without_rankings() {
        let effect = bot_chose_action_effect(
            2,
            "column_four_tactical_v1",
            "drop/c4",
            ColumnId::C4,
            "I can build central pressure with this legal column.",
        );

        assert_eq!(effect.visibility, VisibilityScope::Public);
        let ColumnFourEffect::BotChoseAction { rationale, .. } = effect.payload else {
            panic!("expected bot effect");
        };
        assert!(rationale.contains("legal column"));
        assert!(!rationale.contains("candidate"));
        assert!(!rationale.contains("score"));
        assert!(!rationale.contains('['));
        assert!(!rationale.contains('{'));
    }
}
