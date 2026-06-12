//! Legal action tree and action-path parsing for Frontier Control.

use engine_core::{
    ActionChoice, ActionMetadata, ActionPreview, ActionTree, Actor, CommandEnvelope, Diagnostic,
};

use crate::{
    ids::{FactionId, SiteId},
    state::{FrontierControlState, Phase},
};

pub const ACTION_MARCH: &str = "march";
pub const ACTION_STAKE: &str = "stake";
pub const ACTION_MUSTER: &str = "muster";
pub const ACTION_PATROL: &str = "patrol";
pub const ACTION_REINFORCE: &str = "reinforce";
pub const ACTION_DISMANTLE: &str = "dismantle";
pub const ACTION_END_TURN: &str = "end_turn";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FrontierControlAction {
    March { from: SiteId, to: SiteId },
    Stake { site: SiteId },
    Muster,
    Patrol { from: SiteId, to: SiteId },
    Reinforce { site: SiteId },
    Dismantle { site: SiteId },
    EndTurn,
}

impl FrontierControlAction {
    pub fn faction(&self) -> Option<FactionId> {
        match self {
            Self::March { .. } | Self::Stake { .. } | Self::Muster => Some(FactionId::Prospectors),
            Self::Patrol { .. } | Self::Reinforce { .. } | Self::Dismantle { .. } => {
                Some(FactionId::Garrison)
            }
            Self::EndTurn => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedAction {
    pub action: FrontierControlAction,
    pub actor_faction: FactionId,
    pub budget_remaining: u8,
}

pub fn legal_action_tree(state: &FrontierControlState, actor: &Actor) -> ActionTree {
    if state.terminal_outcome.is_some() || state.phase == Phase::Terminal {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }
    let Some(actor_faction) = state.faction_for_seat(&actor.seat_id) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    if actor_faction != state.active_faction {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }
    let Phase::Action { budget_remaining } = state.phase else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };

    ActionTree::flat(
        state.freshness_token,
        legal_action_choices(state, actor_faction, budget_remaining),
    )
}

pub fn legal_action_metadata(state: &FrontierControlState, actor: &Actor) -> Vec<ActionMetadata> {
    let Some(actor_faction) = state.faction_for_seat(&actor.seat_id) else {
        return vec![metadata("action_status", "not_seated")];
    };
    if state.terminal_outcome.is_some() || state.phase == Phase::Terminal {
        return vec![metadata("action_status", "terminal")];
    }
    if actor_faction == state.active_faction {
        return vec![
            metadata("action_status", "available"),
            metadata("actor_seat", actor.seat_id.0.clone()),
            metadata("active_faction", actor_faction.as_str()),
            metadata("phase", "action"),
        ];
    }
    vec![
        metadata("action_status", "waiting"),
        metadata("actor_seat", actor.seat_id.0.clone()),
        metadata("actor_faction", actor_faction.as_str()),
        metadata("waiting_for", state.active_faction.as_str()),
        metadata(
            "reason",
            "waiting for the active faction to spend its action budget",
        ),
    ]
}

pub fn parse_action_path(segments: &[String]) -> Option<FrontierControlAction> {
    match segments {
        [single] => parse_single_segment(single),
        [family, from, to] if family == ACTION_MARCH => Some(FrontierControlAction::March {
            from: SiteId::parse(from)?,
            to: SiteId::parse(to)?,
        }),
        [family, from, to] if family == ACTION_PATROL => Some(FrontierControlAction::Patrol {
            from: SiteId::parse(from)?,
            to: SiteId::parse(to)?,
        }),
        [family, site] if family == ACTION_STAKE => Some(FrontierControlAction::Stake {
            site: SiteId::parse(site)?,
        }),
        [family, site] if family == ACTION_REINFORCE => Some(FrontierControlAction::Reinforce {
            site: SiteId::parse(site)?,
        }),
        [family, site] if family == ACTION_DISMANTLE => Some(FrontierControlAction::Dismantle {
            site: SiteId::parse(site)?,
        }),
        _ => None,
    }
}

pub fn validate_command(
    state: &FrontierControlState,
    command: &CommandEnvelope,
) -> Result<ValidatedAction, Diagnostic> {
    if command.freshness_token != state.freshness_token {
        return Err(stale_action_diagnostic());
    }
    if state.terminal_outcome.is_some() || state.phase == Phase::Terminal {
        return Err(terminal_diagnostic());
    }
    let actor_faction = state
        .faction_for_seat(&command.actor.seat_id)
        .ok_or_else(wrong_actor_diagnostic)?;
    if actor_faction != state.active_faction {
        return Err(wrong_faction_diagnostic());
    }
    let Phase::Action { budget_remaining } = state.phase else {
        return Err(wrong_phase_diagnostic());
    };
    if budget_remaining == 0 {
        return Err(out_of_budget_diagnostic());
    }
    let action =
        parse_action_path(&command.action_path.segments).ok_or_else(malformed_action_diagnostic)?;
    validate_action_available(state, actor_faction, &action)?;

    Ok(ValidatedAction {
        action,
        actor_faction,
        budget_remaining,
    })
}

pub fn action_segment(action: &FrontierControlAction) -> String {
    match action {
        FrontierControlAction::March { from, to } => {
            format!("{ACTION_MARCH}/{}/{}", from.as_str(), to.as_str())
        }
        FrontierControlAction::Stake { site } => format!("{ACTION_STAKE}/{}", site.as_str()),
        FrontierControlAction::Muster => ACTION_MUSTER.to_owned(),
        FrontierControlAction::Patrol { from, to } => {
            format!("{ACTION_PATROL}/{}/{}", from.as_str(), to.as_str())
        }
        FrontierControlAction::Reinforce { site } => {
            format!("{ACTION_REINFORCE}/{}", site.as_str())
        }
        FrontierControlAction::Dismantle { site } => {
            format!("{ACTION_DISMANTLE}/{}", site.as_str())
        }
        FrontierControlAction::EndTurn => ACTION_END_TURN.to_owned(),
    }
}

fn legal_action_choices(
    state: &FrontierControlState,
    actor_faction: FactionId,
    budget_remaining: u8,
) -> Vec<ActionChoice> {
    let mut choices = Vec::new();
    match actor_faction {
        FactionId::Prospectors => {
            for site in &state.sites {
                if site.crews == 0 {
                    continue;
                }
                if let Some(neighbors) = state.neighbors(site.site) {
                    for to in neighbors {
                        let action = FrontierControlAction::March {
                            from: site.site,
                            to: *to,
                        };
                        if validate_action_available(state, actor_faction, &action).is_ok() {
                            choices.push(move_choice(state, action, budget_remaining));
                        }
                    }
                }
            }
            for site in &state.sites {
                let action = FrontierControlAction::Stake { site: site.site };
                if validate_action_available(state, actor_faction, &action).is_ok() {
                    choices.push(site_choice(
                        action,
                        budget_remaining,
                        "Stake",
                        site.site,
                        "stake",
                        Some(site.stake_value),
                    ));
                }
            }
            let muster = FrontierControlAction::Muster;
            if validate_action_available(state, actor_faction, &muster).is_ok() {
                choices.push(simple_choice(
                    muster,
                    "Muster",
                    "Muster one crew at Base Camp",
                    budget_remaining,
                    "muster",
                ));
            }
        }
        FactionId::Garrison => {
            for site in &state.sites {
                if site.guards == 0 {
                    continue;
                }
                if let Some(neighbors) = state.neighbors(site.site) {
                    for to in neighbors {
                        let action = FrontierControlAction::Patrol {
                            from: site.site,
                            to: *to,
                        };
                        if validate_action_available(state, actor_faction, &action).is_ok() {
                            choices.push(move_choice(state, action, budget_remaining));
                        }
                    }
                }
            }
            for site in &state.sites {
                let reinforce = FrontierControlAction::Reinforce { site: site.site };
                if validate_action_available(state, actor_faction, &reinforce).is_ok() {
                    choices.push(site_choice(
                        reinforce,
                        budget_remaining,
                        "Reinforce",
                        site.site,
                        "reinforce",
                        None,
                    ));
                }
                let dismantle = FrontierControlAction::Dismantle { site: site.site };
                if validate_action_available(state, actor_faction, &dismantle).is_ok() {
                    choices.push(site_choice(
                        dismantle,
                        budget_remaining,
                        "Dismantle",
                        site.site,
                        "dismantle",
                        None,
                    ));
                }
            }
        }
    }

    choices.push(simple_choice(
        FrontierControlAction::EndTurn,
        "End turn",
        "End this faction turn",
        budget_remaining,
        "end-turn",
    ));
    choices
}

fn validate_action_available(
    state: &FrontierControlState,
    actor_faction: FactionId,
    action: &FrontierControlAction,
) -> Result<(), Diagnostic> {
    if let Some(action_faction) = action.faction() {
        if action_faction != actor_faction {
            return Err(wrong_faction_diagnostic());
        }
    }

    match action {
        FrontierControlAction::March { from, to } => {
            validate_move_sites(state, *from, *to)?;
            let from_site = state
                .site(*from)
                .ok_or_else(unavailable_action_diagnostic)?;
            let to_site = state.site(*to).ok_or_else(unavailable_action_diagnostic)?;
            if from_site.crews == 0 {
                return Err(missing_unit_diagnostic());
            }
            if to_site.guards == 0 && to_site.crews >= state.variant.unit_cap_per_site {
                return Err(site_cap_diagnostic());
            }
        }
        FrontierControlAction::Patrol { from, to } => {
            validate_move_sites(state, *from, *to)?;
            let from_site = state
                .site(*from)
                .ok_or_else(unavailable_action_diagnostic)?;
            let to_site = state.site(*to).ok_or_else(unavailable_action_diagnostic)?;
            if from_site.guards == 0 {
                return Err(missing_unit_diagnostic());
            }
            if to_site.crews == 0 && to_site.guards >= state.variant.unit_cap_per_site {
                return Err(site_cap_diagnostic());
            }
        }
        FrontierControlAction::Stake { site } => {
            let site = state
                .site(*site)
                .ok_or_else(unavailable_action_diagnostic)?;
            if site.crews == 0 || site.guards > 0 || site.stake || site.stake_value == 0 {
                return Err(stake_unavailable_diagnostic());
            }
        }
        FrontierControlAction::Muster => {
            let camp = state
                .site(state.variant.base_camp)
                .ok_or_else(unavailable_action_diagnostic)?;
            if camp.guards > 0 || camp.crews >= state.variant.unit_cap_per_site {
                return Err(muster_unavailable_diagnostic());
            }
        }
        FrontierControlAction::Reinforce { site } => {
            let site = state
                .site(*site)
                .ok_or_else(unavailable_action_diagnostic)?;
            if !site.fort
                || site.guards == 0
                || site.crews > 0
                || site.guards >= state.variant.unit_cap_per_site
            {
                return Err(reinforce_unavailable_diagnostic());
            }
        }
        FrontierControlAction::Dismantle { site } => {
            let site = state
                .site(*site)
                .ok_or_else(unavailable_action_diagnostic)?;
            if site.guards == 0 || !site.stake {
                return Err(dismantle_unavailable_diagnostic());
            }
        }
        FrontierControlAction::EndTurn => {}
    }
    Ok(())
}

fn validate_move_sites(
    state: &FrontierControlState,
    from: SiteId,
    to: SiteId,
) -> Result<(), Diagnostic> {
    if !state.sites_are_adjacent(from, to) {
        return Err(non_adjacent_diagnostic());
    }
    Ok(())
}

fn parse_single_segment(segment: &str) -> Option<FrontierControlAction> {
    if segment == ACTION_MUSTER {
        return Some(FrontierControlAction::Muster);
    }
    if segment == ACTION_END_TURN {
        return Some(FrontierControlAction::EndTurn);
    }
    let parts = segment.split('/').collect::<Vec<_>>();
    parse_action_path(
        &parts
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<String>>(),
    )
}

fn move_choice(
    state: &FrontierControlState,
    action: FrontierControlAction,
    budget_remaining: u8,
) -> ActionChoice {
    let (family, from, to, label_verb) = match action {
        FrontierControlAction::March { from, to } => (ACTION_MARCH, from, to, "March"),
        FrontierControlAction::Patrol { from, to } => (ACTION_PATROL, from, to, "Patrol"),
        _ => unreachable!("move_choice requires a move action"),
    };
    let to_site = state.site(to).expect("legal choice target exists");
    let clash = match family {
        ACTION_MARCH => to_site.guards > 0,
        ACTION_PATROL => to_site.crews > 0,
        _ => false,
    };
    let mut choice = ActionChoice::leaf(
        action_segment(&action),
        format!("{label_verb} {} to {}", from.label(), to.label()),
        format!(
            "{label_verb} from {} to {}; remaining budget {budget_remaining}",
            from.label(),
            to.label()
        ),
    );
    choice.metadata = vec![
        metadata("action_family", family),
        metadata("remaining_budget", budget_remaining.to_string()),
        metadata("from_site", from.as_str()),
        metadata("to_site", to.as_str()),
        metadata("from_label", from.label()),
        metadata("to_label", to.label()),
        metadata("clash_preview", bool_value(clash)),
    ];
    choice.tags = vec![family.to_owned(), "budgeted".to_owned(), "move".to_owned()];
    if clash {
        choice.tags.push("clash".to_owned());
    }
    choice.preview = ActionPreview::Available;
    choice
}

fn site_choice(
    action: FrontierControlAction,
    budget_remaining: u8,
    verb: &str,
    site: SiteId,
    family: &str,
    stake_value: Option<u8>,
) -> ActionChoice {
    let mut choice = ActionChoice::leaf(
        action_segment(&action),
        format!("{verb} {}", site.label()),
        format!(
            "{verb} {}; remaining budget {budget_remaining}",
            site.label()
        ),
    );
    choice.metadata = vec![
        metadata("action_family", family),
        metadata("remaining_budget", budget_remaining.to_string()),
        metadata("site_id", site.as_str()),
        metadata("site_label", site.label()),
    ];
    if let Some(value) = stake_value {
        choice
            .metadata
            .push(metadata("stake_value", value.to_string()));
    }
    choice.tags = vec![family.to_owned(), "budgeted".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}

fn simple_choice(
    action: FrontierControlAction,
    label: &str,
    accessibility_label: &str,
    budget_remaining: u8,
    tag: &str,
) -> ActionChoice {
    let mut choice = ActionChoice::leaf(action_segment(&action), label, accessibility_label);
    choice.metadata = vec![
        metadata("action_family", tag),
        metadata("remaining_budget", budget_remaining.to_string()),
    ];
    choice.tags = vec![tag.to_owned(), "budgeted".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}

pub fn metadata(key: impl Into<String>, value: impl Into<String>) -> ActionMetadata {
    ActionMetadata {
        key: key.into(),
        value: value.into(),
    }
}

fn bool_value(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

pub fn wrong_actor_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_actor".to_owned(),
        message: "only a seated Frontier Control actor may submit an action".to_owned(),
    }
}

pub fn wrong_faction_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_faction".to_owned(),
        message: "only the active Frontier Control faction may act right now".to_owned(),
    }
}

pub fn wrong_phase_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_phase".to_owned(),
        message: "that action is not available in the current phase".to_owned(),
    }
}

pub fn terminal_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "terminal_state".to_owned(),
        message: "actions cannot be submitted after the match is complete".to_owned(),
    }
}

pub fn stale_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "stale_action".to_owned(),
        message: "the action was submitted for an older decision point".to_owned(),
    }
}

pub fn out_of_budget_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "out_of_budget".to_owned(),
        message: "that faction has no action budget remaining".to_owned(),
    }
}

pub fn malformed_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "malformed_action".to_owned(),
        message: "Frontier Control actions must use a Rust-supplied legal action path".to_owned(),
    }
}

pub fn unavailable_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "action_unavailable".to_owned(),
        message: "that Frontier Control action is not available now".to_owned(),
    }
}

pub fn non_adjacent_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "non_adjacent_sites".to_owned(),
        message: "those Frontier Control sites are not connected by a trail".to_owned(),
    }
}

pub fn missing_unit_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "missing_unit".to_owned(),
        message: "the selected site does not hold a movable unit for that faction".to_owned(),
    }
}

pub fn site_cap_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "site_cap_reached".to_owned(),
        message: "that site is already at the unit cap for the moving faction".to_owned(),
    }
}

pub fn stake_unavailable_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "stake_unavailable".to_owned(),
        message: "a stake can be placed only on a crew-held, guard-free, valuable unstaked site"
            .to_owned(),
    }
}

pub fn muster_unavailable_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "muster_unavailable".to_owned(),
        message: "muster requires an unguarded Base Camp below the crew cap".to_owned(),
    }
}

pub fn reinforce_unavailable_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "reinforce_unavailable".to_owned(),
        message: "reinforce requires a held fort below the guard cap".to_owned(),
    }
}

pub fn dismantle_unavailable_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "dismantle_unavailable".to_owned(),
        message: "dismantle requires a guard at a staked site".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId};

    use crate::{setup::setup_match, setup::SetupOptions};

    use super::*;

    fn state() -> FrontierControlState {
        setup_match(
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds")
    }

    fn actor_for(state: &FrontierControlState, faction: FactionId) -> Actor {
        let seat = state
            .seats
            .iter()
            .find(|seat| state.faction_for_seat(seat) == Some(faction))
            .expect("faction seat exists")
            .clone();
        Actor { seat_id: seat }
    }

    fn command(
        state: &FrontierControlState,
        faction: FactionId,
        segments: Vec<&str>,
    ) -> CommandEnvelope {
        CommandEnvelope {
            actor: actor_for(state, faction),
            action_path: ActionPath {
                segments: segments.into_iter().map(str::to_owned).collect(),
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    fn choice_segments(tree: &ActionTree) -> Vec<&str> {
        tree.root
            .choices
            .iter()
            .map(|choice| choice.segment.as_str())
            .collect()
    }

    #[test]
    fn prospectors_tree_has_only_prospector_actions_and_end_turn() {
        let mut state = state();
        state.site_mut(SiteId::BaseCamp).expect("base camp").crews = 2;
        state.site_mut(SiteId::Ford).expect("ford").crews = 1;

        let tree = legal_action_tree(&state, &actor_for(&state, FactionId::Prospectors));
        let segments = choice_segments(&tree);

        assert!(segments.iter().any(|segment| segment.starts_with("march/")));
        assert!(segments.iter().any(|segment| segment.starts_with("stake/")));
        assert!(segments.contains(&ACTION_MUSTER));
        assert!(segments.contains(&ACTION_END_TURN));
        assert!(!segments
            .iter()
            .any(|segment| segment.starts_with("patrol/")));
        assert!(!segments
            .iter()
            .any(|segment| segment.starts_with("reinforce/")));
        assert!(!segments
            .iter()
            .any(|segment| segment.starts_with("dismantle/")));
    }

    #[test]
    fn waiting_faction_tree_is_empty_with_safe_metadata() {
        let state = state();
        let actor = actor_for(&state, FactionId::Garrison);
        let tree = legal_action_tree(&state, &actor);
        let metadata = legal_action_metadata(&state, &actor);

        assert!(tree.root.choices.is_empty());
        assert!(metadata
            .iter()
            .any(|entry| entry.key == "action_status" && entry.value == "waiting"));
    }

    #[test]
    fn validation_rejects_wrong_faction_stale_and_non_adjacent_actions() {
        let state = state();
        let wrong = command(&state, FactionId::Garrison, vec![ACTION_END_TURN]);
        assert_eq!(
            validate_command(&state, &wrong)
                .expect_err("wrong faction")
                .code,
            "wrong_faction"
        );

        let mut stale = command(
            &state,
            FactionId::Prospectors,
            vec![ACTION_MARCH, "site_base_camp", "site_ford"],
        );
        stale.freshness_token = FreshnessToken(99);
        assert_eq!(
            validate_command(&state, &stale).expect_err("stale").code,
            "stale_action"
        );

        let non_adjacent = command(
            &state,
            FactionId::Prospectors,
            vec![ACTION_MARCH, "site_base_camp", "site_gatehouse"],
        );
        assert_eq!(
            validate_command(&state, &non_adjacent)
                .expect_err("non-adjacent")
                .code,
            "non_adjacent_sites"
        );
    }
}
