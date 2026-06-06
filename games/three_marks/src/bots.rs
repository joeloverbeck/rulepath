use ai_core::RandomLegalBot;
use engine_core::{ActionPath, Actor, Diagnostic, EffectEnvelope, Seed};

use crate::{
    actions::{legal_action_tree, legal_cells},
    effects::{public_effect, ThreeMarksEffect},
    ids::{CellId, ThreeMarksSeat},
    state::{CellOccupancy, ThreeMarksState},
};

pub const RANDOM_POLICY_ID: &str = "three_marks-random-legal-v1";
pub const LEVEL1_POLICY_ID: &str = "three_marks-priority-v1";
const WINNING_LINES: [[CellId; 3]; 8] = [
    [CellId::R1C1, CellId::R1C2, CellId::R1C3],
    [CellId::R2C1, CellId::R2C2, CellId::R2C3],
    [CellId::R3C1, CellId::R3C2, CellId::R3C3],
    [CellId::R1C1, CellId::R2C1, CellId::R3C1],
    [CellId::R1C2, CellId::R2C2, CellId::R3C2],
    [CellId::R1C3, CellId::R2C3, CellId::R3C3],
    [CellId::R1C1, CellId::R2C2, CellId::R3C3],
    [CellId::R1C3, CellId::R2C2, CellId::R3C1],
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotDecision {
    pub action_path: ActionPath,
    pub explanation: String,
    pub effects: Vec<EffectEnvelope<ThreeMarksEffect>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ThreeMarksRandomBot {
    pub seed: Seed,
}

impl ThreeMarksRandomBot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_action(
        &self,
        state: &ThreeMarksState,
        bot_seat: ThreeMarksSeat,
    ) -> Result<ActionPath, Diagnostic> {
        let actor = Actor {
            seat_id: state.seats[bot_seat.index()].clone(),
        };
        let tree = legal_action_tree(state, &actor);
        RandomLegalBot::new(self.seed).select_action(&tree)
    }

    pub fn select_decision(
        &self,
        state: &ThreeMarksState,
        bot_seat: ThreeMarksSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let action_path = self.select_action(state, bot_seat)?;
        let cell = cell_from_path(&action_path).ok_or_else(|| Diagnostic {
            code: "invalid_bot_action".to_owned(),
            message: "bot selected an invalid action path".to_owned(),
        })?;
        let explanation = "selected a seeded random legal placement".to_owned();
        Ok(decision(
            0,
            RANDOM_POLICY_ID,
            action_path,
            cell,
            explanation,
        ))
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ThreeMarksLevel1Bot;

impl ThreeMarksLevel1Bot {
    pub fn new() -> Self {
        Self
    }

    pub fn select_decision(
        &self,
        state: &ThreeMarksState,
        bot_seat: ThreeMarksSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let legal = legal_cells(state);
        if legal.is_empty() {
            return Err(Diagnostic {
                code: "no_legal_actions".to_owned(),
                message: "no legal action is available".to_owned(),
            });
        }

        let (cell, explanation) = choose_level1_cell(state, bot_seat, &legal);
        Ok(decision(
            1,
            LEVEL1_POLICY_ID,
            ActionPath {
                segments: vec![format!("place/{}", cell.as_str())],
            },
            cell,
            explanation,
        ))
    }

    pub fn select_action(
        &self,
        state: &ThreeMarksState,
        bot_seat: ThreeMarksSeat,
    ) -> Result<ActionPath, Diagnostic> {
        self.select_decision(state, bot_seat)
            .map(|decision| decision.action_path)
    }
}

fn choose_level1_cell(
    state: &ThreeMarksState,
    bot_seat: ThreeMarksSeat,
    legal: &[CellId],
) -> (CellId, String) {
    if let Some(cell) = first_matching(legal, |cell| immediate_win(state, bot_seat, cell)) {
        return (cell, "completed a line".to_owned());
    }

    let opponent = bot_seat.other();
    if let Some(cell) = first_matching(legal, |cell| immediate_win(state, opponent, cell)) {
        return (cell, "blocked a line".to_owned());
    }

    if let Some(cell) = first_matching(legal, |cell| creates_fork(state, bot_seat, cell)) {
        return (cell, "created two line threats".to_owned());
    }

    if let Some(cell) = first_matching(legal, |cell| creates_fork(state, opponent, cell)) {
        return (cell, "blocked a fork threat".to_owned());
    }

    if legal.contains(&CellId::R2C2) {
        return (CellId::R2C2, "took center".to_owned());
    }

    if let Some(cell) = opposite_corner(state, bot_seat, legal) {
        return (cell, "took the opposite corner".to_owned());
    }

    if let Some(cell) = first_matching(legal, is_corner) {
        return (cell, "chose first stable corner".to_owned());
    }

    if let Some(cell) = first_matching(legal, is_side) {
        return (cell, "chose first stable side".to_owned());
    }

    (legal[0], "chose first stable legal placement".to_owned())
}

fn decision(
    level: u8,
    policy_id: &str,
    action_path: ActionPath,
    cell: CellId,
    explanation: String,
) -> BotDecision {
    let action_id = action_path.segments.join("/");
    BotDecision {
        action_path,
        explanation: explanation.clone(),
        effects: vec![public_effect(ThreeMarksEffect::BotChoseAction {
            level,
            policy_id: policy_id.to_owned(),
            action_id,
            cell,
            explanation,
        })],
    }
}

fn first_matching(legal: &[CellId], predicate: impl Fn(CellId) -> bool) -> Option<CellId> {
    legal.iter().copied().find(|cell| predicate(*cell))
}

fn immediate_win(state: &ThreeMarksState, seat: ThreeMarksSeat, cell: CellId) -> bool {
    if !state.occupancy(cell).is_empty() {
        return false;
    }
    let mut candidate = state.clone();
    candidate.cells[cell.index()] = CellOccupancy::Occupied(seat);
    WINNING_LINES.into_iter().any(|line| {
        line.contains(&cell)
            && line
                .iter()
                .all(|line_cell| candidate.occupancy(*line_cell) == CellOccupancy::Occupied(seat))
    })
}

fn creates_fork(state: &ThreeMarksState, seat: ThreeMarksSeat, cell: CellId) -> bool {
    if !state.occupancy(cell).is_empty() {
        return false;
    }
    let mut candidate = state.clone();
    candidate.cells[cell.index()] = CellOccupancy::Occupied(seat);
    CellId::ALL
        .into_iter()
        .filter(|next| candidate.occupancy(*next).is_empty())
        .filter(|next| immediate_win(&candidate, seat, *next))
        .take(2)
        .count()
        >= 2
}

fn opposite_corner(
    state: &ThreeMarksState,
    bot_seat: ThreeMarksSeat,
    legal: &[CellId],
) -> Option<CellId> {
    let opponent = bot_seat.other();
    [
        (CellId::R1C1, CellId::R3C3),
        (CellId::R1C3, CellId::R3C1),
        (CellId::R3C1, CellId::R1C3),
        (CellId::R3C3, CellId::R1C1),
    ]
    .into_iter()
    .find_map(|(opponent_corner, opposite)| {
        if state.occupancy(opponent_corner) == CellOccupancy::Occupied(opponent)
            && legal.contains(&opposite)
        {
            Some(opposite)
        } else {
            None
        }
    })
}

fn is_corner(cell: CellId) -> bool {
    matches!(
        cell,
        CellId::R1C1 | CellId::R1C3 | CellId::R3C1 | CellId::R3C3
    )
}

fn is_side(cell: CellId) -> bool {
    matches!(
        cell,
        CellId::R1C2 | CellId::R2C1 | CellId::R2C3 | CellId::R3C2
    )
}

fn cell_from_path(path: &ActionPath) -> Option<CellId> {
    if path.segments.len() != 1 {
        return None;
    }
    CellId::parse(path.segments[0].strip_prefix("place/")?)
}
