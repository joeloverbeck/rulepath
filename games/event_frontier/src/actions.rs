//! Legal action tree and action-path parsing for Event Frontier.

use engine_core::{
    ActionChoice, ActionMetadata, ActionNode, ActionPreview, ActionTree, Actor, CommandEnvelope,
    Diagnostic,
};

use crate::{
    cards::{CardCatalog, CardId},
    ids::{FactionId, SiteId},
    state::{CardPhase, EventFrontierState, FirstChoice},
};

pub const ACTION_EVENT: &str = "event";
pub const ACTION_OPERATION: &str = "operation";
pub const ACTION_LIMITED_OPERATION: &str = "limited_operation";
pub const ACTION_PASS: &str = "pass";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OperationKind {
    Survey,
    Fortify,
    Writ,
    Trek,
    Cache,
    Rally,
}

impl OperationKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Survey => "survey",
            Self::Fortify => "fortify",
            Self::Writ => "writ",
            Self::Trek => "trek",
            Self::Cache => "cache",
            Self::Rally => "rally",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Survey => "Survey",
            Self::Fortify => "Fortify",
            Self::Writ => "Writ",
            Self::Trek => "Trek",
            Self::Cache => "Cache",
            Self::Rally => "Rally",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "survey" => Some(Self::Survey),
            "fortify" => Some(Self::Fortify),
            "writ" => Some(Self::Writ),
            "trek" => Some(Self::Trek),
            "cache" => Some(Self::Cache),
            "rally" => Some(Self::Rally),
            _ => None,
        }
    }

    pub const fn faction(self) -> FactionId {
        match self {
            Self::Survey | Self::Fortify | Self::Writ => FactionId::Charter,
            Self::Trek | Self::Cache | Self::Rally => FactionId::Freeholders,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OperationSelection {
    pub site: SiteId,
    pub destination: Option<SiteId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventFrontierAction {
    Event,
    Operation {
        kind: OperationKind,
        selections: Vec<OperationSelection>,
        limited: bool,
    },
    Pass,
}

impl EventFrontierAction {
    pub fn as_choice_label(&self) -> String {
        match self {
            Self::Event => "event".to_owned(),
            Self::Operation { kind, limited, .. } => {
                let prefix = if *limited {
                    ACTION_LIMITED_OPERATION
                } else {
                    ACTION_OPERATION
                };
                format!("{prefix}:{}", kind.as_str())
            }
            Self::Pass => "pass".to_owned(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChoicePosition {
    First,
    Second { first_choice: FirstChoice },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedAction {
    pub action: EventFrontierAction,
    pub actor_faction: FactionId,
    pub position: ChoicePosition,
}

pub fn legal_action_tree(state: &EventFrontierState, actor: &Actor) -> ActionTree {
    let Some(actor_faction) = state.faction_for_seat(&actor.seat_id) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    let Some((choosing_faction, menu)) = choosing_menu(state) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    if actor_faction != choosing_faction {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    let choices = menu
        .into_iter()
        .map(|entry| menu_choice(state, choosing_faction, entry))
        .collect();
    ActionTree::flat(state.freshness_token, choices)
}

pub fn legal_action_metadata(state: &EventFrontierState, actor: &Actor) -> Vec<ActionMetadata> {
    let Some(actor_faction) = state.faction_for_seat(&actor.seat_id) else {
        return vec![metadata("action_status", "not_seated")];
    };
    if state.terminal_outcome.is_some() || state.card_phase == CardPhase::Terminal {
        return vec![metadata("action_status", "terminal")];
    }
    let Some((choosing_faction, _)) = choosing_menu(state) else {
        return vec![
            metadata("action_status", "waiting"),
            metadata("actor_seat", actor.seat_id.0.clone()),
            metadata("actor_faction", actor_faction.as_str()),
            metadata("phase", state.card_phase.stable_summary()),
            metadata("reason", "card flow is resolving automatically"),
        ];
    };
    if actor_faction == choosing_faction {
        return vec![
            metadata("action_status", "available"),
            metadata("actor_seat", actor.seat_id.0.clone()),
            metadata("actor_faction", actor_faction.as_str()),
            metadata("phase", state.card_phase.stable_summary()),
            metadata("current_card", current_card_metadata(state)),
            metadata("next_public_card", next_public_card_metadata(state)),
        ];
    }
    vec![
        metadata("action_status", "waiting"),
        metadata("actor_seat", actor.seat_id.0.clone()),
        metadata("actor_faction", actor_faction.as_str()),
        metadata("waiting_for", choosing_faction.as_str()),
        metadata("phase", state.card_phase.stable_summary()),
        metadata("reason", "waiting for the eligible faction to choose"),
    ]
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuEntry {
    Event,
    Operation { limited: bool },
    Pass,
}

pub fn choosing_menu(state: &EventFrontierState) -> Option<(FactionId, Vec<MenuEntry>)> {
    match &state.card_phase {
        CardPhase::AwaitingFirstChoice { faction } => Some((
            *faction,
            vec![
                MenuEntry::Event,
                MenuEntry::Operation { limited: false },
                MenuEntry::Pass,
            ],
        )),
        CardPhase::AwaitingSecondChoice {
            second_faction,
            first_choice,
            ..
        } => Some((*second_faction, second_choice_menu(*first_choice))),
        CardPhase::Reckoning | CardPhase::Terminal => None,
    }
}

pub fn second_choice_menu(first_choice: FirstChoice) -> Vec<MenuEntry> {
    match first_choice {
        FirstChoice::Event => vec![MenuEntry::Operation { limited: false }, MenuEntry::Pass],
        FirstChoice::Operation => vec![
            MenuEntry::Event,
            MenuEntry::Operation { limited: true },
            MenuEntry::Pass,
        ],
        FirstChoice::Pass => vec![
            MenuEntry::Event,
            MenuEntry::Operation { limited: false },
            MenuEntry::Pass,
        ],
    }
}

pub fn parse_action_path(segments: &[String]) -> Option<EventFrontierAction> {
    if segments.len() == 1 {
        if let Some(parsed) = parse_single_segment(&segments[0]) {
            return Some(parsed);
        }
        let split = segments[0]
            .split('/')
            .map(str::to_owned)
            .collect::<Vec<_>>();
        if split.len() == 1 {
            return None;
        }
        return parse_action_path(&split);
    }

    match segments {
        [action] if action == ACTION_EVENT => Some(EventFrontierAction::Event),
        [action] if action == ACTION_PASS => Some(EventFrontierAction::Pass),
        [family, kind, payload] if family == ACTION_OPERATION => {
            parse_operation(kind, payload, false)
        }
        [family, kind, payload] if family == ACTION_LIMITED_OPERATION => {
            parse_operation(kind, payload, true)
        }
        _ => None,
    }
}

pub fn validate_command(
    state: &EventFrontierState,
    command: &CommandEnvelope,
) -> Result<ValidatedAction, Diagnostic> {
    if command.freshness_token != state.freshness_token {
        return Err(stale_action_diagnostic());
    }
    if state.terminal_outcome.is_some() || state.card_phase == CardPhase::Terminal {
        return Err(terminal_diagnostic());
    }
    let actor_faction = state
        .faction_for_seat(&command.actor.seat_id)
        .ok_or_else(wrong_actor_diagnostic)?;
    let action =
        parse_action_path(&command.action_path.segments).ok_or_else(malformed_action_diagnostic)?;
    let (choosing_faction, menu) = choosing_menu(state).ok_or_else(wrong_phase_diagnostic)?;
    if actor_faction != choosing_faction {
        return Err(wrong_faction_diagnostic());
    }
    validate_menu_allows(&menu, &action)?;
    if let EventFrontierAction::Operation {
        kind,
        selections,
        limited,
    } = &action
    {
        validate_operation(state, actor_faction, *kind, selections, *limited)?;
    }

    Ok(ValidatedAction {
        action,
        actor_faction,
        position: choice_position(&state.card_phase).ok_or_else(wrong_phase_diagnostic)?,
    })
}

pub fn validate_operation(
    state: &EventFrontierState,
    faction: FactionId,
    kind: OperationKind,
    selections: &[OperationSelection],
    limited: bool,
) -> Result<(), Diagnostic> {
    if kind.faction() != faction {
        return Err(wrong_operation_faction_diagnostic());
    }
    if selections.is_empty() {
        return Err(empty_operation_diagnostic());
    }
    let max_sites = if limited {
        1
    } else {
        current_ops_value(state).ok_or_else(wrong_phase_diagnostic)?
    };
    if selections.len() > max_sites as usize {
        return Err(over_budget_diagnostic());
    }
    if operation_cost(selections) > resource_pool(state, faction) {
        return Err(unaffordable_diagnostic());
    }
    ensure_unique_operation_sites(selections)?;

    for selection in selections {
        validate_operation_selection(state, kind, *selection)?;
    }
    Ok(())
}

pub fn operation_cost(selections: &[OperationSelection]) -> u8 {
    selections.len() as u8
}

pub fn resource_pool(state: &EventFrontierState, faction: FactionId) -> u8 {
    match faction {
        FactionId::Charter => state.resources.funds,
        FactionId::Freeholders => state.resources.provisions,
    }
}

fn menu_choice(state: &EventFrontierState, faction: FactionId, entry: MenuEntry) -> ActionChoice {
    match entry {
        MenuEntry::Event => simple_choice(
            ACTION_EVENT,
            "Event",
            "Resolve the current event card",
            "event",
            state,
            faction,
        ),
        MenuEntry::Pass => simple_choice(
            ACTION_PASS,
            "Pass",
            "Pass and gain one resource",
            "pass",
            state,
            faction,
        ),
        MenuEntry::Operation { limited } => operation_root_choice(state, faction, limited),
    }
}

fn operation_root_choice(
    state: &EventFrontierState,
    faction: FactionId,
    limited: bool,
) -> ActionChoice {
    let segment = if limited {
        ACTION_LIMITED_OPERATION
    } else {
        ACTION_OPERATION
    };
    let mut choice = simple_choice(
        segment,
        if limited {
            "Limited operation"
        } else {
            "Operation"
        },
        if limited {
            "Choose a one-site operation"
        } else {
            "Choose an operation"
        },
        if limited {
            "limited-operation"
        } else {
            "operation"
        },
        state,
        faction,
    );
    choice.next = Some(Box::new(ActionNode {
        choices: operation_kind_choices(state, faction, limited),
    }));
    choice
}

fn operation_kind_choices(
    state: &EventFrontierState,
    faction: FactionId,
    limited: bool,
) -> Vec<ActionChoice> {
    operation_kinds_for(faction)
        .into_iter()
        .filter_map(|kind| {
            let leaf_choices = operation_leaf_choices(state, faction, kind, limited);
            if leaf_choices.is_empty() {
                return None;
            }
            let mut choice = ActionChoice::leaf(
                kind.as_str(),
                kind.label(),
                format!("Choose {} operation targets", kind.label()),
            );
            choice.preview = ActionPreview::Available;
            choice.tags.push("operation-kind".to_owned());
            choice.metadata = vec![
                metadata("op", kind.as_str()),
                metadata("limited", limited.to_string()),
                metadata("cost_rule", "base_one_resource_per_site"),
                metadata("edict_modifier_point", "pending_ticket_007"),
            ];
            choice.next = Some(Box::new(ActionNode {
                choices: leaf_choices,
            }));
            Some(choice)
        })
        .collect()
}

fn operation_leaf_choices(
    state: &EventFrontierState,
    faction: FactionId,
    kind: OperationKind,
    limited: bool,
) -> Vec<ActionChoice> {
    let bound = if limited {
        1
    } else {
        current_ops_value(state).unwrap_or(0)
    };
    let max_sites = bound.min(resource_pool(state, faction));
    if max_sites == 0 {
        return Vec::new();
    }

    let singles = legal_single_selections(state, kind);
    let mut choices = Vec::new();
    for size in 1..=max_sites as usize {
        for selections in selection_combinations(&singles, size) {
            let payload = encode_selections(&selections);
            let segment = format!(
                "{}/{}/{}",
                if limited {
                    ACTION_LIMITED_OPERATION
                } else {
                    ACTION_OPERATION
                },
                kind.as_str(),
                payload
            );
            let mut choice = ActionChoice::leaf(
                segment,
                selection_label(kind, &selections),
                format!("Apply {} to {}", kind.label(), payload),
            );
            choice.preview = ActionPreview::Available;
            choice.tags.push("operation-leaf".to_owned());
            choice.metadata = vec![
                metadata("op", kind.as_str()),
                metadata("site_count", selections.len().to_string()),
                metadata("cost", operation_cost(&selections).to_string()),
                metadata("ops_bound", bound.to_string()),
                metadata("eligibility_consequence", "acting_forfeits_next_card"),
            ];
            choices.push(choice);
        }
    }
    choices
}

fn legal_single_selections(
    state: &EventFrontierState,
    kind: OperationKind,
) -> Vec<OperationSelection> {
    let mut selections = Vec::new();
    for site in SiteId::ALL {
        match kind {
            OperationKind::Trek => {
                if let Some(site_state) = state.site(site) {
                    if site_state.settlers == 0 {
                        continue;
                    }
                }
                for destination in state.neighbors(site).unwrap_or(&[]) {
                    if state
                        .site(*destination)
                        .is_some_and(|target| target.settlers < 3)
                    {
                        selections.push(OperationSelection {
                            site,
                            destination: Some(*destination),
                        });
                    }
                }
            }
            _ => {
                let selection = OperationSelection {
                    site,
                    destination: None,
                };
                if validate_operation_selection(state, kind, selection).is_ok() {
                    selections.push(selection);
                }
            }
        }
    }
    selections.sort();
    selections
}

fn validate_operation_selection(
    state: &EventFrontierState,
    kind: OperationKind,
    selection: OperationSelection,
) -> Result<(), Diagnostic> {
    let site = state
        .site(selection.site)
        .ok_or_else(unavailable_site_diagnostic)?;
    match kind {
        OperationKind::Survey => {
            if selection.destination.is_some() {
                return Err(malformed_action_diagnostic());
            }
            if site.agents >= 3 {
                return Err(site_cap_diagnostic("agent_cap"));
            }
            if selection.site == SiteId::Charterhouse
                || adjacent_to_charter_presence(state, selection.site)
            {
                Ok(())
            } else {
                Err(precondition_diagnostic("survey_requires_adjacency"))
            }
        }
        OperationKind::Fortify => {
            if selection.destination.is_some() {
                return Err(malformed_action_diagnostic());
            }
            if site.agents >= 2 && !site.depot {
                Ok(())
            } else {
                Err(precondition_diagnostic(
                    "fortify_requires_two_agents_no_depot",
                ))
            }
        }
        OperationKind::Writ => {
            if selection.destination.is_some() {
                return Err(malformed_action_diagnostic());
            }
            if site.agents > 0 && site.cache_count > 0 {
                Ok(())
            } else {
                Err(precondition_diagnostic("writ_requires_agent_and_cache"))
            }
        }
        OperationKind::Trek => {
            let destination = selection
                .destination
                .ok_or_else(malformed_action_diagnostic)?;
            let target = state
                .site(destination)
                .ok_or_else(unavailable_site_diagnostic)?;
            if site.settlers == 0 {
                return Err(precondition_diagnostic("trek_requires_settler"));
            }
            if target.settlers >= 3 {
                return Err(site_cap_diagnostic("settler_cap"));
            }
            if state
                .neighbors(selection.site)
                .is_some_and(|neighbors| neighbors.contains(&destination))
            {
                Ok(())
            } else {
                Err(precondition_diagnostic("trek_requires_trail"))
            }
        }
        OperationKind::Cache => {
            if selection.destination.is_some() {
                return Err(malformed_action_diagnostic());
            }
            if site.settlers > 0 && !site.depot && site.cache_count < 2 {
                Ok(())
            } else {
                Err(precondition_diagnostic(
                    "cache_requires_settler_no_depot_under_cap",
                ))
            }
        }
        OperationKind::Rally => {
            if selection.destination.is_some() {
                return Err(malformed_action_diagnostic());
            }
            if site.settlers >= 3 {
                return Err(site_cap_diagnostic("settler_cap"));
            }
            if selection.site == SiteId::Landing || site.cache_count > 0 {
                Ok(())
            } else {
                Err(precondition_diagnostic("rally_requires_landing_or_cache"))
            }
        }
    }
}

fn adjacent_to_charter_presence(state: &EventFrontierState, site: SiteId) -> bool {
    state.neighbors(site).is_some_and(|neighbors| {
        neighbors.iter().any(|neighbor| {
            state
                .site(*neighbor)
                .is_some_and(|candidate| candidate.agents > 0 || candidate.depot)
        })
    })
}

fn ensure_unique_operation_sites(selections: &[OperationSelection]) -> Result<(), Diagnostic> {
    for (index, left) in selections.iter().enumerate() {
        for right in selections.iter().skip(index + 1) {
            if left.site == right.site {
                return Err(duplicate_site_diagnostic());
            }
        }
    }
    Ok(())
}

fn operation_kinds_for(faction: FactionId) -> [OperationKind; 3] {
    match faction {
        FactionId::Charter => [
            OperationKind::Survey,
            OperationKind::Fortify,
            OperationKind::Writ,
        ],
        FactionId::Freeholders => [
            OperationKind::Trek,
            OperationKind::Cache,
            OperationKind::Rally,
        ],
    }
}

fn selection_combinations(
    selections: &[OperationSelection],
    size: usize,
) -> Vec<Vec<OperationSelection>> {
    fn walk(
        selections: &[OperationSelection],
        size: usize,
        start: usize,
        current: &mut Vec<OperationSelection>,
        out: &mut Vec<Vec<OperationSelection>>,
    ) {
        if current.len() == size {
            out.push(current.clone());
            return;
        }
        for index in start..selections.len() {
            if current
                .iter()
                .any(|selection| selection.site == selections[index].site)
            {
                continue;
            }
            current.push(selections[index]);
            walk(selections, size, index + 1, current, out);
            current.pop();
        }
    }

    let mut out = Vec::new();
    walk(selections, size, 0, &mut Vec::new(), &mut out);
    out
}

fn parse_operation(kind: &str, payload: &str, limited: bool) -> Option<EventFrontierAction> {
    let kind = OperationKind::parse(kind)?;
    let selections = parse_selections(kind, payload)?;
    Some(EventFrontierAction::Operation {
        kind,
        selections,
        limited,
    })
}

fn parse_selections(kind: OperationKind, payload: &str) -> Option<Vec<OperationSelection>> {
    if payload.is_empty() {
        return None;
    }
    payload
        .split(',')
        .map(|part| {
            if kind == OperationKind::Trek {
                let (from, to) = part.split_once('>')?;
                Some(OperationSelection {
                    site: SiteId::parse(from)?,
                    destination: Some(SiteId::parse(to)?),
                })
            } else {
                Some(OperationSelection {
                    site: SiteId::parse(part)?,
                    destination: None,
                })
            }
        })
        .collect()
}

fn encode_selections(selections: &[OperationSelection]) -> String {
    selections
        .iter()
        .map(|selection| match selection.destination {
            Some(destination) => format!("{}>{}", selection.site.as_str(), destination.as_str()),
            None => selection.site.as_str().to_owned(),
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn selection_label(kind: OperationKind, selections: &[OperationSelection]) -> String {
    format!("{} {}", kind.label(), encode_selections(selections))
}

fn parse_single_segment(segment: &str) -> Option<EventFrontierAction> {
    match segment {
        ACTION_EVENT => Some(EventFrontierAction::Event),
        ACTION_PASS => Some(EventFrontierAction::Pass),
        _ => None,
    }
}

fn simple_choice(
    segment: &str,
    label: &str,
    accessibility_label: &str,
    tag: &str,
    state: &EventFrontierState,
    faction: FactionId,
) -> ActionChoice {
    let mut choice = ActionChoice::leaf(segment, label, accessibility_label);
    choice.preview = ActionPreview::Available;
    choice.tags.push(tag.to_owned());
    choice.metadata = vec![
        metadata("faction", faction.as_str()),
        metadata("phase", state.card_phase.stable_summary()),
        metadata("current_card", current_card_metadata(state)),
        metadata("next_public_card", next_public_card_metadata(state)),
    ];
    choice
}

fn choice_position(phase: &CardPhase) -> Option<ChoicePosition> {
    match phase {
        CardPhase::AwaitingFirstChoice { .. } => Some(ChoicePosition::First),
        CardPhase::AwaitingSecondChoice { first_choice, .. } => Some(ChoicePosition::Second {
            first_choice: *first_choice,
        }),
        CardPhase::Reckoning | CardPhase::Terminal => None,
    }
}

fn validate_menu_allows(
    menu: &[MenuEntry],
    action: &EventFrontierAction,
) -> Result<(), Diagnostic> {
    let allowed = match action {
        EventFrontierAction::Event => menu.contains(&MenuEntry::Event),
        EventFrontierAction::Operation { limited, .. } => {
            menu.contains(&MenuEntry::Operation { limited: *limited })
        }
        EventFrontierAction::Pass => menu.contains(&MenuEntry::Pass),
    };
    allowed
        .then_some(())
        .ok_or_else(unavailable_action_diagnostic)
}

fn current_ops_value(state: &EventFrontierState) -> Option<u8> {
    let current = state.deck.current?;
    card_ops_value(current)
}

fn card_ops_value(card: CardId) -> Option<u8> {
    let catalog = CardCatalog::parse(include_str!("../data/cards.toml")).ok()?;
    catalog
        .cards
        .iter()
        .find_map(|data| (data.id == card).then_some(data.ops_value))
}

fn current_card_metadata(state: &EventFrontierState) -> String {
    state
        .deck
        .current
        .map(|card| card.as_str().to_owned())
        .unwrap_or_else(|| "none".to_owned())
}

fn next_public_card_metadata(state: &EventFrontierState) -> String {
    state
        .deck
        .next_public
        .map(|card| card.as_str().to_owned())
        .unwrap_or_else(|| "none".to_owned())
}

fn metadata(key: &str, value: impl Into<String>) -> ActionMetadata {
    ActionMetadata {
        key: key.to_owned(),
        value: value.into(),
    }
}

fn stale_action_diagnostic() -> Diagnostic {
    diagnostic(
        "stale_action",
        "that Event Frontier action was built for an older state",
    )
}

fn terminal_diagnostic() -> Diagnostic {
    diagnostic("terminal", "Event Frontier is already terminal")
}

fn wrong_actor_diagnostic() -> Diagnostic {
    diagnostic(
        "wrong_actor",
        "that seat is not seated in this Event Frontier match",
    )
}

fn wrong_phase_diagnostic() -> Diagnostic {
    diagnostic(
        "wrong_phase",
        "that Event Frontier action is not available in the current phase",
    )
}

fn wrong_faction_diagnostic() -> Diagnostic {
    diagnostic(
        "wrong_faction",
        "that faction is waiting for the eligible faction to choose",
    )
}

fn wrong_operation_faction_diagnostic() -> Diagnostic {
    diagnostic(
        "wrong_operation_faction",
        "that operation is not available to this faction",
    )
}

fn malformed_action_diagnostic() -> Diagnostic {
    diagnostic(
        "malformed_action",
        "that Event Frontier action path is malformed",
    )
}

fn unavailable_action_diagnostic() -> Diagnostic {
    diagnostic(
        "action_unavailable",
        "that Event Frontier choice is not available now",
    )
}

fn empty_operation_diagnostic() -> Diagnostic {
    diagnostic(
        "empty_operation",
        "an Event Frontier operation must select at least one site",
    )
}

fn over_budget_diagnostic() -> Diagnostic {
    diagnostic(
        "operation_site_bound_exceeded",
        "that operation selects more sites than the current ops value allows",
    )
}

fn unaffordable_diagnostic() -> Diagnostic {
    diagnostic(
        "unaffordable_operation",
        "that operation costs more public resources than the faction has",
    )
}

fn unavailable_site_diagnostic() -> Diagnostic {
    diagnostic("unknown_site", "that Event Frontier site is not available")
}

fn duplicate_site_diagnostic() -> Diagnostic {
    diagnostic(
        "duplicate_operation_site",
        "an operation may select each site at most once",
    )
}

fn precondition_diagnostic(code: &str) -> Diagnostic {
    diagnostic(code, "that operation precondition is not satisfied")
}

fn site_cap_diagnostic(code: &str) -> Diagnostic {
    diagnostic(code, "that site is at its component cap")
}

fn diagnostic(code: &str, message: &str) -> Diagnostic {
    Diagnostic {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}
