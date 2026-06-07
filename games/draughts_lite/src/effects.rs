use engine_core::{Diagnostic, EffectEnvelope, VisibilityScope};
use game_stdlib::board_space::Coord;

use crate::{
    rules::{segments_for_move, LegalMove, MoveKind, MoveStep},
    state::PieceKind,
    DraughtsLiteSeat, PieceId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DraughtsLiteEffect {
    MoveCommitted {
        action_path: Vec<String>,
        seat: DraughtsLiteSeat,
        piece_id: PieceId,
        start_cell: Coord,
        final_cell: Coord,
        move_kind: MoveKind,
        path_length: u8,
    },
    QuietStep {
        piece_id: PieceId,
        origin: Coord,
        landing: Coord,
        piece_kind_before: PieceKind,
        piece_kind_after: PieceKind,
    },
    CaptureStep {
        piece_id: PieceId,
        origin: Coord,
        landing: Coord,
        captured_cell: Coord,
        captured_piece_id: PieceId,
        captured_owner: DraughtsLiteSeat,
    },
    Promotion {
        piece_id: PieceId,
        seat: DraughtsLiteSeat,
        cell: Coord,
        from: PieceKind,
        to: PieceKind,
        during_capture: bool,
    },
    ForcedCaptureAvailable {
        active_seat: DraughtsLiteSeat,
        capture_origin_count: u8,
        explanation: String,
    },
    ForcedContinuationRequired {
        piece_id: PieceId,
        current_landing: Coord,
        continuation_destination_count: u8,
        explanation: String,
    },
    InvalidCommand {
        code: String,
        public_message: String,
        rejected_action_path: Vec<String>,
    },
    TerminalWin {
        winner: DraughtsLiteSeat,
        loser: DraughtsLiteSeat,
        reason: TerminalWinReason,
    },
    BotChoseAction {
        level: u8,
        policy_id: String,
        action_path: Vec<String>,
        rationale: String,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TerminalWinReason {
    OpponentNoPieces,
    OpponentNoLegalMove,
}

pub fn public_effect(payload: DraughtsLiteEffect) -> EffectEnvelope<DraughtsLiteEffect> {
    EffectEnvelope {
        visibility: VisibilityScope::Public,
        payload,
    }
}

pub fn move_effects(legal_move: &LegalMove) -> Vec<EffectEnvelope<DraughtsLiteEffect>> {
    let mut effects = vec![public_effect(DraughtsLiteEffect::MoveCommitted {
        action_path: segments_for_move(legal_move),
        seat: legal_move.actor,
        piece_id: legal_move.piece_id,
        start_cell: legal_move.origin,
        final_cell: legal_move.final_cell(),
        move_kind: legal_move.kind,
        path_length: legal_move.steps.len() as u8,
    })];

    for (index, step) in legal_move.steps.iter().enumerate() {
        effects.push(step_effect(legal_move, *step));
        if step.promotes {
            effects.push(public_effect(DraughtsLiteEffect::Promotion {
                piece_id: legal_move.piece_id,
                seat: legal_move.actor,
                cell: step.to,
                from: step.piece_kind_before,
                to: step.piece_kind_after,
                during_capture: step.capture.is_some(),
            }));
        }
        if step.capture.is_some() && index + 1 < legal_move.steps.len() {
            effects.push(public_effect(
                DraughtsLiteEffect::ForcedContinuationRequired {
                    piece_id: legal_move.piece_id,
                    current_landing: step.to,
                    continuation_destination_count: 1,
                    explanation:
                        "This piece has another legal capture, so the same move must continue."
                            .to_owned(),
                },
            ));
        }
    }

    effects
}

pub fn forced_capture_available_effect(
    active_seat: DraughtsLiteSeat,
    capture_origin_count: u8,
) -> EffectEnvelope<DraughtsLiteEffect> {
    public_effect(DraughtsLiteEffect::ForcedCaptureAvailable {
        active_seat,
        capture_origin_count,
        explanation: "At least one capture is available, so a capture is mandatory.".to_owned(),
    })
}

pub fn terminal_win_effect(
    winner: DraughtsLiteSeat,
    loser: DraughtsLiteSeat,
    reason: TerminalWinReason,
) -> EffectEnvelope<DraughtsLiteEffect> {
    public_effect(DraughtsLiteEffect::TerminalWin {
        winner,
        loser,
        reason,
    })
}

pub fn invalid_command_effect(
    diagnostic: &Diagnostic,
    rejected_action_path: &[String],
) -> EffectEnvelope<DraughtsLiteEffect> {
    public_effect(DraughtsLiteEffect::InvalidCommand {
        code: diagnostic.code.clone(),
        public_message: diagnostic.message.clone(),
        rejected_action_path: rejected_action_path.to_vec(),
    })
}

pub fn bot_chose_action_effect(
    level: u8,
    policy_id: impl Into<String>,
    action_path: Vec<String>,
    rationale: impl Into<String>,
) -> EffectEnvelope<DraughtsLiteEffect> {
    public_effect(DraughtsLiteEffect::BotChoseAction {
        level,
        policy_id: policy_id.into(),
        action_path,
        rationale: rationale.into(),
    })
}

pub fn display_anchor(cell: Coord) -> String {
    format!("cell:{}", cell.id())
}

fn step_effect(legal_move: &LegalMove, step: MoveStep) -> EffectEnvelope<DraughtsLiteEffect> {
    match step.capture {
        Some(capture) => public_effect(DraughtsLiteEffect::CaptureStep {
            piece_id: legal_move.piece_id,
            origin: step.from,
            landing: step.to,
            captured_cell: capture.cell,
            captured_piece_id: capture.piece_id,
            captured_owner: capture.owner,
        }),
        None => public_effect(DraughtsLiteEffect::QuietStep {
            piece_id: legal_move.piece_id,
            origin: step.from,
            landing: step.to,
            piece_kind_before: step.piece_kind_before,
            piece_kind_after: step.piece_kind_after,
        }),
    }
}

#[cfg(test)]
mod tests {
    use engine_core::VisibilityScope;

    use super::*;

    #[test]
    fn bot_chose_action_effect_is_public_safe_prose() {
        let effect = bot_chose_action_effect(
            1,
            "draughts_lite_level1_v1",
            vec!["from/r3c2".to_owned(), "jump/r5c4".to_owned()],
            "I can make this legal capture and keep the path complete.",
        );

        assert_eq!(effect.visibility, VisibilityScope::Public);
        let DraughtsLiteEffect::BotChoseAction { rationale, .. } = effect.payload else {
            panic!("expected bot effect");
        };
        assert!(rationale.contains("legal capture"));
        assert!(!rationale.contains("candidate"));
        assert!(!rationale.contains("score"));
        assert!(!rationale.contains('{'));
    }

    #[test]
    fn invalid_command_effect_is_public_and_path_only() {
        let diagnostic = Diagnostic {
            code: "stale_action".to_owned(),
            message: "the action was submitted for an older decision point".to_owned(),
        };

        let effect =
            invalid_command_effect(&diagnostic, &["from/r3c2".to_owned(), "to/r4c3".to_owned()]);

        assert_eq!(effect.visibility, VisibilityScope::Public);
        assert_eq!(
            effect.payload,
            DraughtsLiteEffect::InvalidCommand {
                code: "stale_action".to_owned(),
                public_message: "the action was submitted for an older decision point".to_owned(),
                rejected_action_path: vec!["from/r3c2".to_owned(), "to/r4c3".to_owned()],
            }
        );
    }
}
