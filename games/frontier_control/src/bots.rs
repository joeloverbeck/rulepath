//! Public-view bots for Frontier Control.

use ai_core::RandomLegalBot;
use engine_core::{ActionPath, Actor, CommandEnvelope, Diagnostic, RulesVersion, SeatId, Seed};

use crate::{
    actions::{
        legal_action_tree, validate_command, ACTION_DISMANTLE, ACTION_END_TURN, ACTION_MARCH,
        ACTION_MUSTER, ACTION_PATROL, ACTION_REINFORCE, ACTION_STAKE,
    },
    ids::SiteId,
    state::FrontierControlState,
    visibility::{project_view, PublicView, SiteView},
};

pub const RANDOM_POLICY_ID: &str = "frontier_control_random_legal_v0";
pub const GARRISON_LEVEL1_POLICY_ID: &str = "frontier_control_garrison_level1_v1";
pub const PROSPECTOR_LEVEL1_POLICY_ID: &str = "frontier_control_prospector_level1_v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierBotInput {
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
pub struct FrontierRandomBot {
    pub seed: Seed,
}

impl FrontierRandomBot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_action(
        &self,
        state: &FrontierControlState,
        bot_seat: &SeatId,
    ) -> Result<ActionPath, Diagnostic> {
        let tree = legal_action_tree(state, &actor_for_seat(bot_seat));
        RandomLegalBot::new(self.seed).select_action(&tree)
    }

    pub fn select_decision(
        &self,
        state: &FrontierControlState,
        bot_seat: &SeatId,
    ) -> Result<BotDecision, Diagnostic> {
        let action_path = self.select_action(state, bot_seat)?;
        Ok(decision(
            0,
            RANDOM_POLICY_ID,
            action_path,
            "Selected a seeded random legal Frontier Control action.".to_owned(),
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrontierGarrisonLevel1Bot {
    pub seed: Seed,
}

impl FrontierGarrisonLevel1Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(state: &FrontierControlState, bot_seat: &SeatId) -> FrontierBotInput {
        FrontierBotInput {
            policy_id: GARRISON_LEVEL1_POLICY_ID.to_owned(),
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
        state: &FrontierControlState,
        bot_seat: &SeatId,
    ) -> Result<ActionPath, Diagnostic> {
        self.select_decision(state, bot_seat)
            .map(|decision| decision.action_path)
    }

    pub fn select_decision(
        &self,
        state: &FrontierControlState,
        bot_seat: &SeatId,
    ) -> Result<BotDecision, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        let legal = legal_paths(state, bot_seat);
        if legal.is_empty() {
            return Err(no_legal_actions());
        }

        if let Some(site) = best_dismantle_target(&input.public_view, &legal) {
            return Ok(decision(
                1,
                GARRISON_LEVEL1_POLICY_ID,
                site_action(ACTION_DISMANTLE, site),
                format!(
                    "Garrison dismantled the public stake at {} to deny Prospector scoring.",
                    site.label()
                ),
            ));
        }

        if let Some((from, to)) = best_patrol_target(&input.public_view, &legal) {
            return Ok(decision(
                1,
                GARRISON_LEVEL1_POLICY_ID,
                move_action(ACTION_PATROL, from, to),
                format!(
                    "Garrison patrolled from {} to {} to contest public crew or supply pressure.",
                    from.label(),
                    to.label()
                ),
            ));
        }

        if let Some(site) = first_site_action(&legal, ACTION_REINFORCE) {
            return Ok(decision(
                1,
                GARRISON_LEVEL1_POLICY_ID,
                site_action(ACTION_REINFORCE, site),
                format!(
                    "Garrison reinforced {} to hold a public fort.",
                    site.label()
                ),
            ));
        }

        fallback_end_turn(&legal, GARRISON_LEVEL1_POLICY_ID, "Garrison")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrontierProspectorLevel1Bot {
    pub seed: Seed,
}

impl FrontierProspectorLevel1Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(state: &FrontierControlState, bot_seat: &SeatId) -> FrontierBotInput {
        FrontierBotInput {
            policy_id: PROSPECTOR_LEVEL1_POLICY_ID.to_owned(),
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
        state: &FrontierControlState,
        bot_seat: &SeatId,
    ) -> Result<ActionPath, Diagnostic> {
        self.select_decision(state, bot_seat)
            .map(|decision| decision.action_path)
    }

    pub fn select_decision(
        &self,
        state: &FrontierControlState,
        bot_seat: &SeatId,
    ) -> Result<BotDecision, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        let legal = legal_paths(state, bot_seat);
        if legal.is_empty() {
            return Err(no_legal_actions());
        }

        if let Some(site) = best_stake_target(&input.public_view, &legal) {
            return Ok(decision(
                1,
                PROSPECTOR_LEVEL1_POLICY_ID,
                site_action(ACTION_STAKE, site),
                format!(
                    "Prospectors staked {} because it is the highest-value public legal site.",
                    site.label()
                ),
            ));
        }

        if let Some((from, to)) = best_march_target(&input.public_view, &legal) {
            return Ok(decision(
                1,
                PROSPECTOR_LEVEL1_POLICY_ID,
                move_action(ACTION_MARCH, from, to),
                format!(
                    "Prospectors marched from {} to {} toward public stake value or an opening.",
                    from.label(),
                    to.label()
                ),
            ));
        }

        if legal
            .iter()
            .any(|path| single_segment(path) == ACTION_MUSTER)
        {
            return Ok(decision(
                1,
                PROSPECTOR_LEVEL1_POLICY_ID,
                ActionPath {
                    segments: vec![ACTION_MUSTER.to_owned()],
                },
                "Prospectors mustered because public crews are below the useful expansion threshold."
                    .to_owned(),
            ));
        }

        fallback_end_turn(&legal, PROSPECTOR_LEVEL1_POLICY_ID, "Prospectors")
    }
}

pub fn actor_for_seat(seat: &SeatId) -> Actor {
    Actor {
        seat_id: seat.clone(),
    }
}

pub fn command_for_decision(
    state: &FrontierControlState,
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
    state: &FrontierControlState,
    bot_seat: &SeatId,
    decision: &BotDecision,
) -> Result<(), Diagnostic> {
    validate_command(state, &command_for_decision(state, bot_seat, decision)).map(|_| ())
}

fn legal_paths(state: &FrontierControlState, bot_seat: &SeatId) -> Vec<ActionPath> {
    legal_action_tree(state, &actor_for_seat(bot_seat))
        .root
        .choices
        .iter()
        .map(|choice| ActionPath {
            segments: vec![choice.segment.clone()],
        })
        .collect()
}

fn best_dismantle_target(view: &PublicView, legal: &[ActionPath]) -> Option<SiteId> {
    SiteId::ALL
        .into_iter()
        .filter(|site| legal_has_site(legal, ACTION_DISMANTLE, *site))
        .max_by_key(|site| (stake_value(view, *site), reverse_site_index(*site)))
}

fn best_patrol_target(view: &PublicView, legal: &[ActionPath]) -> Option<(SiteId, SiteId)> {
    legal
        .iter()
        .filter_map(|path| move_parts(path, ACTION_PATROL))
        .max_by_key(|(_, to)| {
            (
                crew_count(view, *to),
                supplied_stake_value(view, *to),
                reverse_site_index(*to),
            )
        })
}

fn best_stake_target(view: &PublicView, legal: &[ActionPath]) -> Option<SiteId> {
    SiteId::ALL
        .into_iter()
        .filter(|site| legal_has_site(legal, ACTION_STAKE, *site))
        .max_by_key(|site| (stake_value(view, *site), reverse_site_index(*site)))
}

fn best_march_target(view: &PublicView, legal: &[ActionPath]) -> Option<(SiteId, SiteId)> {
    legal
        .iter()
        .filter_map(|path| move_parts(path, ACTION_MARCH))
        .max_by_key(|(_, to)| {
            (
                unstaked_value(view, *to),
                u8::from(guard_count(view, *to) > 0),
                reverse_site_index(*to),
            )
        })
}

fn fallback_end_turn(
    legal: &[ActionPath],
    policy_id: &str,
    faction_label: &str,
) -> Result<BotDecision, Diagnostic> {
    if legal
        .iter()
        .any(|path| single_segment(path) == ACTION_END_TURN)
    {
        return Ok(decision(
            1,
            policy_id,
            ActionPath {
                segments: vec![ACTION_END_TURN.to_owned()],
            },
            format!("{faction_label} ended the turn because no public legal action improved the position."),
        ));
    }
    Err(no_legal_actions())
}

fn legal_has_site(legal: &[ActionPath], action: &str, site: SiteId) -> bool {
    let target = format!("{action}/{}", site.as_str());
    legal.iter().any(|path| single_segment(path) == target)
}

fn first_site_action(legal: &[ActionPath], action: &str) -> Option<SiteId> {
    SiteId::ALL
        .into_iter()
        .find(|site| legal_has_site(legal, action, *site))
}

fn move_parts(path: &ActionPath, action: &str) -> Option<(SiteId, SiteId)> {
    let segment = single_segment(path);
    let parts = segment.split('/').collect::<Vec<_>>();
    match parts.as_slice() {
        [family, from, to] if *family == action => Some((SiteId::parse(from)?, SiteId::parse(to)?)),
        _ => None,
    }
}

fn single_segment(path: &ActionPath) -> &str {
    path.segments.first().map(String::as_str).unwrap_or("")
}

fn site_action(action: &str, site: SiteId) -> ActionPath {
    ActionPath {
        segments: vec![format!("{action}/{}", site.as_str())],
    }
}

fn move_action(action: &str, from: SiteId, to: SiteId) -> ActionPath {
    ActionPath {
        segments: vec![format!("{action}/{}/{}", from.as_str(), to.as_str())],
    }
}

fn site_view(view: &PublicView, site: SiteId) -> Option<&SiteView> {
    view.sites.iter().find(|candidate| candidate.site == site)
}

fn stake_value(view: &PublicView, site: SiteId) -> u8 {
    site_view(view, site)
        .map(|candidate| candidate.stake_value)
        .unwrap_or(0)
}

fn unstaked_value(view: &PublicView, site: SiteId) -> u8 {
    site_view(view, site)
        .filter(|candidate| !candidate.stake)
        .map(|candidate| candidate.stake_value)
        .unwrap_or(0)
}

fn supplied_stake_value(view: &PublicView, site: SiteId) -> u8 {
    site_view(view, site)
        .filter(|candidate| candidate.supplied == Some(true))
        .map(|candidate| candidate.stake_value)
        .unwrap_or(0)
}

fn crew_count(view: &PublicView, site: SiteId) -> u8 {
    site_view(view, site)
        .map(|candidate| candidate.crews)
        .unwrap_or(0)
}

fn guard_count(view: &PublicView, site: SiteId) -> u8 {
    site_view(view, site)
        .map(|candidate| candidate.guards)
        .unwrap_or(0)
}

fn reverse_site_index(site: SiteId) -> u8 {
    u8::MAX - site_index(site)
}

fn site_index(site: SiteId) -> u8 {
    SiteId::ALL
        .iter()
        .position(|candidate| *candidate == site)
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

fn no_legal_actions() -> Diagnostic {
    Diagnostic {
        code: "no_legal_actions".to_owned(),
        message: "no legal action is available".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use engine_core::{SeatId, Seed};

    use crate::{
        actions::ACTION_END_TURN,
        ids::FactionId,
        rules::apply_command,
        setup::{setup_match, SetupOptions},
    };

    use super::*;

    fn state() -> FrontierControlState {
        setup_match(
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds")
    }

    fn seat_for(state: &FrontierControlState, faction: FactionId) -> SeatId {
        state
            .seats
            .iter()
            .find(|seat| state.faction_for_seat(seat) == Some(faction))
            .expect("seat exists")
            .clone()
    }

    fn advance_to_garrison(state: &mut FrontierControlState) {
        let seat = seat_for(state, FactionId::Prospectors);
        let decision = BotDecision {
            policy_id: PROSPECTOR_LEVEL1_POLICY_ID.to_owned(),
            policy_version: 1,
            level: 1,
            action_path: ActionPath {
                segments: vec![ACTION_END_TURN.to_owned()],
            },
            rationale: "test end turn".to_owned(),
        };
        let command = command_for_decision(state, &seat, &decision);
        apply_command(state, &command).expect("advance to garrison");
    }

    #[test]
    fn bots_select_legal_paths_for_both_factions() {
        let mut state = state();
        let prospector_seat = seat_for(&state, FactionId::Prospectors);
        let random = FrontierRandomBot::new(Seed(7))
            .select_decision(&state, &prospector_seat)
            .expect("random legal");
        validate_bot_decision(&state, &prospector_seat, &random).expect("random validates");
        let prospector = FrontierProspectorLevel1Bot::new(Seed(1))
            .select_decision(&state, &prospector_seat)
            .expect("prospector legal");
        validate_bot_decision(&state, &prospector_seat, &prospector).expect("prospector validates");

        advance_to_garrison(&mut state);
        let garrison_seat = seat_for(&state, FactionId::Garrison);
        let garrison = FrontierGarrisonLevel1Bot::new(Seed(1))
            .select_decision(&state, &garrison_seat)
            .expect("garrison legal");
        validate_bot_decision(&state, &garrison_seat, &garrison).expect("garrison validates");
        assert!(garrison.rationale.contains("Garrison"));
    }

    #[test]
    fn bots_are_deterministic_under_declared_inputs() {
        let state = state();
        let seat = seat_for(&state, FactionId::Prospectors);
        let left = FrontierProspectorLevel1Bot::new(Seed(42))
            .select_decision(&state, &seat)
            .expect("left decision");
        let right = FrontierProspectorLevel1Bot::new(Seed(42))
            .select_decision(&state, &seat)
            .expect("right decision");

        assert_eq!(left, right);
        assert!(left.rationale.contains("Prospectors"));
    }

    #[test]
    fn bots_finish_level1_game_smoke() {
        let mut state = state();
        for _ in 0..100 {
            if state.terminal_outcome.is_some() {
                assert_eq!(state.phase, crate::state::Phase::Terminal);
                return;
            }
            let active = state.active_faction;
            let seat = state.active_seat().expect("active seat").clone();
            let decision = match active {
                FactionId::Garrison => FrontierGarrisonLevel1Bot::new(Seed(11))
                    .select_decision(&state, &seat)
                    .expect("garrison decision"),
                FactionId::Prospectors => FrontierProspectorLevel1Bot::new(Seed(11))
                    .select_decision(&state, &seat)
                    .expect("prospector decision"),
            };
            validate_bot_decision(&state, &seat, &decision).expect("decision validates");
            let command = command_for_decision(&state, &seat, &decision);
            apply_command(&mut state, &command).expect("decision applies");
        }
        panic!("level1 bot smoke did not finish");
    }
}
