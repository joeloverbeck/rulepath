use ai_core::RandomLegalBot;
use engine_core::{ActionPath, Actor, Diagnostic, Seed};

use crate::{actions::legal_action_tree, ids::RaceSeat, state::RaceState, visibility::PublicView};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RaceRandomBot {
    pub seed: Seed,
}

impl RaceRandomBot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_action(
        &self,
        state: &RaceState,
        bot_seat: RaceSeat,
    ) -> Result<ActionPath, Diagnostic> {
        let actor = Actor {
            seat_id: state.seats[bot_seat.index()].clone(),
        };
        let tree = legal_action_tree(state, &actor);
        RandomLegalBot::new(self.seed).select_action(&tree)
    }

    pub fn select_action_from_view(
        &self,
        _view: &PublicView,
        tree: &engine_core::ActionTree,
    ) -> Result<ActionPath, Diagnostic> {
        RandomLegalBot::new(self.seed).select_action(tree)
    }
}
