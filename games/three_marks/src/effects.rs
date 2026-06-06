use engine_core::{EffectEnvelope, VisibilityScope};

use crate::{
    ids::{CellId, ThreeMarksSeat},
    state::{TerminalOutcome, WinningLine},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ThreeMarksEffect {
    SetupComplete {
        game_id: &'static str,
        variant_id: String,
        rules_version: String,
        seats: [String; 2],
    },
    MarkPlaced {
        seat: ThreeMarksSeat,
        cell: CellId,
        ply: u8,
        occupancy_summary: String,
    },
    ActivePlayerChanged {
        previous_seat: ThreeMarksSeat,
        active_seat: ThreeMarksSeat,
        ply: u8,
    },
    PlacementRejected {
        reason: RejectionReason,
        label: String,
    },
    LineCompleted {
        winning_seat: ThreeMarksSeat,
        line: WinningLine,
    },
    DrawReached {
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
        cell: CellId,
        explanation: String,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RejectionReason {
    Occupied,
    Stale,
    InvalidCell,
    WrongActor,
    Terminal,
    UnknownActor,
    InvalidPath,
    InvalidAction,
}

pub fn public_effect(payload: ThreeMarksEffect) -> EffectEnvelope<ThreeMarksEffect> {
    EffectEnvelope {
        visibility: VisibilityScope::Public,
        payload,
    }
}
