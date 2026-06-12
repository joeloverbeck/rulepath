//! Public-view bots for Flood Watch.

use ai_core::RandomLegalBot;
use engine_core::{ActionPath, Actor, CommandEnvelope, Diagnostic, RulesVersion, SeatId, Seed};

use crate::{
    actions::{
        legal_action_tree, validate_command, ACTION_BAIL, ACTION_END_TURN, ACTION_FORECAST,
        ACTION_REINFORCE,
    },
    ids::{DistrictId, EventKind},
    state::FloodWatchState,
    visibility::{project_view, CompositionView, PublicView},
};

pub const RANDOM_POLICY_ID: &str = "flood_watch_random_legal_v0";
pub const LEVEL1_POLICY_ID: &str = "flood_watch_level1_public_priority_v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FloodWatchBotInput {
    pub policy_id: String,
    pub bot_seat: SeatId,
    pub public_view: PublicView,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotDecision {
    pub policy_id: String,
    pub policy_version: u32,
    pub level: u8,
    pub action_path: ActionPath,
    pub rationale: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FloodWatchRandomBot {
    pub seed: Seed,
}

impl FloodWatchRandomBot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_action(
        &self,
        state: &FloodWatchState,
        bot_seat: &SeatId,
    ) -> Result<ActionPath, Diagnostic> {
        let tree = legal_action_tree(state, &actor_for_seat(bot_seat));
        RandomLegalBot::new(self.seed).select_action(&tree)
    }

    pub fn select_decision(
        &self,
        state: &FloodWatchState,
        bot_seat: &SeatId,
    ) -> Result<BotDecision, Diagnostic> {
        let action_path = self.select_action(state, bot_seat)?;
        Ok(BotDecision {
            policy_id: RANDOM_POLICY_ID.to_owned(),
            policy_version: 1,
            level: 0,
            action_path,
            rationale: "Selected a seeded random legal Flood Watch action.".to_owned(),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FloodWatchLevel1Bot {
    pub seed: Seed,
}

impl FloodWatchLevel1Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(state: &FloodWatchState, bot_seat: &SeatId) -> FloodWatchBotInput {
        FloodWatchBotInput {
            policy_id: LEVEL1_POLICY_ID.to_owned(),
            bot_seat: bot_seat.clone(),
            public_view: project_view(
                state,
                &engine_core::Viewer {
                    seat_id: Some(bot_seat.clone()),
                },
            ),
        }
    }

    pub fn select_action(
        &self,
        state: &FloodWatchState,
        bot_seat: &SeatId,
    ) -> Result<ActionPath, Diagnostic> {
        self.select_decision(state, bot_seat)
            .map(|decision| decision.action_path)
    }

    pub fn select_decision(
        &self,
        state: &FloodWatchState,
        bot_seat: &SeatId,
    ) -> Result<BotDecision, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        let tree = legal_action_tree(state, &actor_for_seat(bot_seat));
        let legal = legal_paths(&tree);
        if legal.is_empty() {
            return Err(Diagnostic {
                code: "no_legal_actions".to_owned(),
                message: "no legal action is available".to_owned(),
            });
        }

        if let Some(district) = rescue_target(&input.public_view, &legal) {
            return Ok(decision(
                1,
                LEVEL1_POLICY_ID,
                action_path(ACTION_BAIL, district),
                format!(
                    "Bailed {} because it is one step from shared loss.",
                    district.label()
                ),
            ));
        }

        if let Some((action, district, rationale)) = forecast_response(&input.public_view, &legal) {
            return Ok(decision(
                1,
                LEVEL1_POLICY_ID,
                action_path(action, district),
                rationale,
            ));
        }

        if let Some(district) = reinforce_target(&input.public_view, &legal) {
            return Ok(decision(
                1,
                LEVEL1_POLICY_ID,
                action_path(ACTION_REINFORCE, district),
                format!(
                    "Reinforced {} because public remaining composition shows the highest expected pressure.",
                    district.label()
                ),
            ));
        }

        if legal
            .iter()
            .any(|path| single_segment(path) == ACTION_FORECAST)
        {
            return Ok(decision(
                1,
                LEVEL1_POLICY_ID,
                ActionPath {
                    segments: vec![ACTION_FORECAST.to_owned()],
                },
                "Forecasted with spare budget because no public district action improved the position."
                    .to_owned(),
            ));
        }

        if legal
            .iter()
            .any(|path| single_segment(path) == ACTION_END_TURN)
        {
            return Ok(decision(
                1,
                LEVEL1_POLICY_ID,
                ActionPath {
                    segments: vec![ACTION_END_TURN.to_owned()],
                },
                "Ended the turn because no public legal action improved the position.".to_owned(),
            ));
        }

        Err(Diagnostic {
            code: "no_legal_actions".to_owned(),
            message: "no legal action is available".to_owned(),
        })
    }
}

pub fn actor_for_seat(seat: &SeatId) -> Actor {
    Actor {
        seat_id: seat.clone(),
    }
}

pub fn command_for_decision(
    state: &FloodWatchState,
    bot_seat: &SeatId,
    decision: &BotDecision,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_seat(bot_seat),
        action_path: decision.action_path.clone(),
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

pub fn validate_bot_decision(
    state: &FloodWatchState,
    bot_seat: &SeatId,
    decision: &BotDecision,
) -> Result<(), Diagnostic> {
    validate_command(state, &command_for_decision(state, bot_seat, decision)).map(|_| ())
}

fn rescue_target(view: &PublicView, legal: &[ActionPath]) -> Option<DistrictId> {
    DistrictId::ALL
        .into_iter()
        .filter(|district| legal_has(legal, ACTION_BAIL, *district))
        .filter(|district| {
            view.districts
                .iter()
                .any(|candidate| candidate.district == *district && candidate.flood_level >= 2)
        })
        .max_by_key(|district| {
            (
                expected_pressure(&view.remaining_composition, *district),
                5 - district_index(*district),
            )
        })
}

fn forecast_response(
    view: &PublicView,
    legal: &[ActionPath],
) -> Option<(&'static str, DistrictId, String)> {
    let forecast = view
        .forecast
        .as_ref()
        .and_then(|value| EventKind::parse(&value.id))?;
    let (district, rise): (DistrictId, u8) = match forecast {
        EventKind::Downpour { district } => (district, 1),
        EventKind::StormSurge { district } => (district, 2),
        EventKind::Reprieve => return None,
    };
    let district_view = view
        .districts
        .iter()
        .find(|candidate| candidate.district == district)?;
    let unabsorbed = rise.saturating_sub(district_view.levees);
    if district_view.flood_level.saturating_add(unabsorbed) < 3 {
        return None;
    }
    if legal_has(legal, ACTION_REINFORCE, district) {
        return Some((
            ACTION_REINFORCE,
            district,
            format!(
                "Reinforced {} because the public forecast would otherwise inundate it.",
                district.label()
            ),
        ));
    }
    if legal_has(legal, ACTION_BAIL, district) {
        return Some((
            ACTION_BAIL,
            district,
            format!(
                "Bailed {} because the public forecast would otherwise inundate it.",
                district.label()
            ),
        ));
    }
    None
}

fn reinforce_target(view: &PublicView, legal: &[ActionPath]) -> Option<DistrictId> {
    DistrictId::ALL
        .into_iter()
        .filter(|district| legal_has(legal, ACTION_REINFORCE, *district))
        .max_by_key(|district| {
            (
                expected_pressure(&view.remaining_composition, *district),
                exposed_flood_level(view, *district),
                5 - district_index(*district),
            )
        })
        .filter(|district| expected_pressure(&view.remaining_composition, *district) > 0)
}

fn expected_pressure(composition: &CompositionView, district: DistrictId) -> u8 {
    let downpours = composition
        .downpours_per_district
        .iter()
        .find_map(|(candidate, count)| (*candidate == district).then_some(*count))
        .unwrap_or(0);
    let surges = composition
        .surges_per_district
        .iter()
        .find_map(|(candidate, count)| (*candidate == district).then_some(*count))
        .unwrap_or(0);
    downpours.saturating_add(surges.saturating_mul(2))
}

fn exposed_flood_level(view: &PublicView, district: DistrictId) -> u8 {
    view.districts
        .iter()
        .find_map(|candidate| (candidate.district == district).then_some(candidate.flood_level))
        .unwrap_or(0)
}

fn legal_has(legal: &[ActionPath], action: &str, district: DistrictId) -> bool {
    let target = format!("{action}/{}", district.as_str());
    legal.iter().any(|path| single_segment(path) == target)
}

fn legal_paths(tree: &engine_core::ActionTree) -> Vec<ActionPath> {
    tree.root
        .choices
        .iter()
        .map(|choice| ActionPath {
            segments: vec![choice.segment.clone()],
        })
        .collect()
}

fn single_segment(path: &ActionPath) -> &str {
    path.segments.first().map(String::as_str).unwrap_or("")
}

fn action_path(action: &str, district: DistrictId) -> ActionPath {
    ActionPath {
        segments: vec![format!("{action}/{}", district.as_str())],
    }
}

fn district_index(district: DistrictId) -> u8 {
    DistrictId::ALL
        .iter()
        .position(|candidate| *candidate == district)
        .unwrap_or(0) as u8
}

fn decision(level: u8, policy_id: &str, action_path: ActionPath, rationale: String) -> BotDecision {
    BotDecision {
        policy_id: policy_id.to_owned(),
        policy_version: 1,
        level,
        action_path,
        rationale,
    }
}
