//! Public-view bots for Event Frontier.

use ai_core::RandomLegalBot;
use engine_core::{
    ActionChoice, ActionPath, ActionTree, Actor, CommandEnvelope, Diagnostic, RulesVersion, SeatId,
    Seed, Viewer,
};

use crate::{
    actions::{
        legal_action_tree, validate_command, ACTION_EVENT, ACTION_LIMITED_OPERATION,
        ACTION_OPERATION, ACTION_PASS,
    },
    ids::{FactionId, SiteId},
    state::EventFrontierState,
    visibility::{project_view, PublicView, SiteView},
};

pub const RANDOM_POLICY_ID: &str = "event_frontier_random_legal_v0";
pub const CHARTER_LEVEL1_POLICY_ID: &str = "event_frontier_charter_level1_v1";
pub const FREEHOLDERS_LEVEL1_POLICY_ID: &str = "event_frontier_freeholders_level1_v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventFrontierBotInput {
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
pub struct EventFrontierRandomBot {
    pub seed: Seed,
}

impl EventFrontierRandomBot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_action(
        &self,
        state: &EventFrontierState,
        bot_seat: &SeatId,
    ) -> Result<ActionPath, Diagnostic> {
        let legal = legal_paths(state, bot_seat);
        let tree = flat_tree_from_paths(state, &legal);
        RandomLegalBot::new(self.seed).select_action(&tree)
    }

    pub fn select_decision(
        &self,
        state: &EventFrontierState,
        bot_seat: &SeatId,
    ) -> Result<BotDecision, Diagnostic> {
        Ok(decision(
            0,
            RANDOM_POLICY_ID,
            self.select_action(state, bot_seat)?,
            "Selected a seeded random legal Event Frontier action.".to_owned(),
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EventCharterLevel1Bot {
    pub seed: Seed,
}

impl EventCharterLevel1Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(state: &EventFrontierState, bot_seat: &SeatId) -> EventFrontierBotInput {
        input_for(state, bot_seat, CHARTER_LEVEL1_POLICY_ID)
    }

    pub fn select_action(
        &self,
        state: &EventFrontierState,
        bot_seat: &SeatId,
    ) -> Result<ActionPath, Diagnostic> {
        self.select_decision(state, bot_seat)
            .map(|decision| decision.action_path)
    }

    pub fn select_decision(
        &self,
        state: &EventFrontierState,
        bot_seat: &SeatId,
    ) -> Result<BotDecision, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        let legal = legal_paths(state, bot_seat);
        if legal.is_empty() {
            return Err(no_legal_actions());
        }

        match choose_family(&input.public_view, FactionId::Charter, &legal) {
            ChoiceFamily::Event => Ok(decision(
                1,
                CHARTER_LEVEL1_POLICY_ID,
                simple_action(ACTION_EVENT),
                "Charter resolved the public event because the current card favors Charter."
                    .to_owned(),
            )),
            ChoiceFamily::Operation => charter_operation_decision(&input.public_view, &legal),
            ChoiceFamily::Pass => Ok(decision(
                1,
                CHARTER_LEVEL1_POLICY_ID,
                simple_action(ACTION_PASS),
                "Charter passed to save funds for later public operations.".to_owned(),
            )),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EventFreeholdersLevel1Bot {
    pub seed: Seed,
}

impl EventFreeholdersLevel1Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(state: &EventFrontierState, bot_seat: &SeatId) -> EventFrontierBotInput {
        input_for(state, bot_seat, FREEHOLDERS_LEVEL1_POLICY_ID)
    }

    pub fn select_action(
        &self,
        state: &EventFrontierState,
        bot_seat: &SeatId,
    ) -> Result<ActionPath, Diagnostic> {
        self.select_decision(state, bot_seat)
            .map(|decision| decision.action_path)
    }

    pub fn select_decision(
        &self,
        state: &EventFrontierState,
        bot_seat: &SeatId,
    ) -> Result<BotDecision, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        let legal = legal_paths(state, bot_seat);
        if legal.is_empty() {
            return Err(no_legal_actions());
        }

        match choose_family(&input.public_view, FactionId::Freeholders, &legal) {
            ChoiceFamily::Event => Ok(decision(
                1,
                FREEHOLDERS_LEVEL1_POLICY_ID,
                simple_action(ACTION_EVENT),
                "Freeholders resolved the public event because the current card favors Freeholders."
                    .to_owned(),
            )),
            ChoiceFamily::Operation => freeholder_operation_decision(&input.public_view, &legal),
            ChoiceFamily::Pass => Ok(decision(
                1,
                FREEHOLDERS_LEVEL1_POLICY_ID,
                simple_action(ACTION_PASS),
                "Freeholders passed to save provisions for later public operations.".to_owned(),
            )),
        }
    }
}

pub fn actor_for_seat(seat: &SeatId) -> Actor {
    Actor {
        seat_id: seat.clone(),
    }
}

pub fn command_for_decision(
    state: &EventFrontierState,
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
    state: &EventFrontierState,
    bot_seat: &SeatId,
    decision: &BotDecision,
) -> Result<(), Diagnostic> {
    validate_command(state, &command_for_decision(state, bot_seat, decision)).map(|_| ())
}

fn input_for(
    state: &EventFrontierState,
    bot_seat: &SeatId,
    policy_id: &str,
) -> EventFrontierBotInput {
    EventFrontierBotInput {
        policy_id: policy_id.to_owned(),
        bot_seat: bot_seat.clone(),
        public_view: project_view(
            state,
            &Viewer {
                seat_id: Some(bot_seat.clone()),
            },
        ),
    }
}

fn legal_paths(state: &EventFrontierState, bot_seat: &SeatId) -> Vec<ActionPath> {
    let tree = legal_action_tree(state, &actor_for_seat(bot_seat));
    let mut paths = Vec::new();
    for choice in &tree.root.choices {
        collect_legal_choice(choice, &mut paths);
    }
    paths.sort_by(|left, right| left.segments.cmp(&right.segments));
    paths
}

fn collect_legal_choice(choice: &ActionChoice, out: &mut Vec<ActionPath>) {
    if let Some(next) = &choice.next {
        for child in &next.choices {
            collect_legal_choice(child, out);
        }
        return;
    }
    if choice.segment == ACTION_EVENT
        || choice.segment == ACTION_PASS
        || choice.segment.starts_with(ACTION_OPERATION)
        || choice.segment.starts_with(ACTION_LIMITED_OPERATION)
    {
        out.push(simple_action(&choice.segment));
    }
}

fn flat_tree_from_paths(state: &EventFrontierState, legal: &[ActionPath]) -> ActionTree {
    ActionTree::flat(
        state.freshness_token,
        legal
            .iter()
            .map(|path| {
                let segment = single_segment(path).to_owned();
                ActionChoice::leaf(segment.clone(), segment.clone(), segment)
            })
            .collect(),
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ChoiceFamily {
    Event,
    Operation,
    Pass,
}

fn choose_family(view: &PublicView, faction: FactionId, legal: &[ActionPath]) -> ChoiceFamily {
    let event_legal = has_simple(legal, ACTION_EVENT);
    let pass_legal = has_simple(legal, ACTION_PASS);
    let op_legal = legal.iter().any(|path| operation_candidate(path).is_some());
    let own_resource = resource_for(view, faction);
    let own_distance = victory_distance_for(view, faction);
    let opponent_distance = victory_distance_for(view, opposing_faction(faction));

    if event_legal && current_event_favors(view, faction) {
        return ChoiceFamily::Event;
    }
    if op_legal && (own_distance <= 2 || opponent_distance <= 2 || own_resource > 1) {
        return ChoiceFamily::Operation;
    }
    if pass_legal && own_resource <= 1 {
        return ChoiceFamily::Pass;
    }
    if op_legal {
        return ChoiceFamily::Operation;
    }
    if event_legal {
        return ChoiceFamily::Event;
    }
    ChoiceFamily::Pass
}

fn charter_operation_decision(
    view: &PublicView,
    legal: &[ActionPath],
) -> Result<BotDecision, Diagnostic> {
    if let Some(candidate) = best_charter_writ(view, legal) {
        return Ok(decision(
            1,
            CHARTER_LEVEL1_POLICY_ID,
            candidate.path,
            format!(
                "Charter wrote down caches at {} to deny a public Freeholder cache threat.",
                candidate.primary_site.label()
            ),
        ));
    }
    if let Some(candidate) = best_charter_survey(view, legal) {
        return Ok(decision(
            1,
            CHARTER_LEVEL1_POLICY_ID,
            candidate.path,
            format!(
                "Charter surveyed {} to extend public site majority pressure.",
                candidate.primary_site.label()
            ),
        ));
    }
    if let Some(candidate) = best_charter_fortify(view, legal) {
        return Ok(decision(
            1,
            CHARTER_LEVEL1_POLICY_ID,
            candidate.path,
            format!(
                "Charter fortified {} because it is a public contested held site.",
                candidate.primary_site.label()
            ),
        ));
    }
    fallback_pass(legal, CHARTER_LEVEL1_POLICY_ID, "Charter", "funds")
}

fn freeholder_operation_decision(
    view: &PublicView,
    legal: &[ActionPath],
) -> Result<BotDecision, Diagnostic> {
    if let Some(candidate) = best_freeholder_cache(view, legal) {
        return Ok(decision(
            1,
            FREEHOLDERS_LEVEL1_POLICY_ID,
            candidate.path,
            format!(
                "Freeholders laid a cache at {} to move toward the public cache threshold.",
                candidate.primary_site.label()
            ),
        ));
    }
    if let Some(candidate) = best_freeholder_trek_to_cache(view, legal) {
        return Ok(decision(
            1,
            FREEHOLDERS_LEVEL1_POLICY_ID,
            candidate.path,
            format!(
                "Freeholders trekked toward {} to escort a public exposed cache.",
                candidate.primary_site.label()
            ),
        ));
    }
    if let Some(candidate) = best_freeholder_spread(view, legal) {
        return Ok(decision(
            1,
            FREEHOLDERS_LEVEL1_POLICY_ID,
            candidate.path,
            format!(
                "Freeholders spread presence at {} to break public Charter majorities.",
                candidate.primary_site.label()
            ),
        ));
    }
    fallback_pass(
        legal,
        FREEHOLDERS_LEVEL1_POLICY_ID,
        "Freeholders",
        "provisions",
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RankedCandidate {
    path: ActionPath,
    primary_site: SiteId,
    rank: (u8, u8, u8, String),
}

fn best_charter_writ(view: &PublicView, legal: &[ActionPath]) -> Option<RankedCandidate> {
    legal
        .iter()
        .filter_map(operation_candidate)
        .filter(|candidate| candidate.kind == "writ")
        .filter_map(|candidate| {
            let site = candidate.primary_site()?;
            let site_view = site_view(view, site)?;
            Some(RankedCandidate {
                path: candidate.path,
                primary_site: site,
                rank: (
                    site_view.cache_count,
                    total_selected_caches(view, &candidate.sites),
                    reverse_site_index(site),
                    candidate.raw_path,
                ),
            })
        })
        .max_by(|left, right| left.rank.cmp(&right.rank))
}

fn best_charter_survey(view: &PublicView, legal: &[ActionPath]) -> Option<RankedCandidate> {
    legal
        .iter()
        .filter_map(operation_candidate)
        .filter(|candidate| candidate.kind == "survey")
        .filter_map(|candidate| {
            let site = candidate.primary_site()?;
            let site_view = site_view(view, site)?;
            Some(RankedCandidate {
                path: candidate.path,
                primary_site: site,
                rank: (
                    u8::from(
                        site_view.agents.saturating_add(u8::from(site_view.depot))
                            <= site_view.settlers,
                    ),
                    u8::from(site_view.settlers > 0),
                    reverse_site_index(site),
                    candidate.raw_path,
                ),
            })
        })
        .max_by(|left, right| left.rank.cmp(&right.rank))
}

fn best_charter_fortify(view: &PublicView, legal: &[ActionPath]) -> Option<RankedCandidate> {
    legal
        .iter()
        .filter_map(operation_candidate)
        .filter(|candidate| candidate.kind == "fortify")
        .filter_map(|candidate| {
            let site = candidate.primary_site()?;
            let site_view = site_view(view, site)?;
            Some(RankedCandidate {
                path: candidate.path,
                primary_site: site,
                rank: (
                    site_view.settlers,
                    site_view.agents,
                    reverse_site_index(site),
                    candidate.raw_path,
                ),
            })
        })
        .max_by(|left, right| left.rank.cmp(&right.rank))
}

fn best_freeholder_cache(view: &PublicView, legal: &[ActionPath]) -> Option<RankedCandidate> {
    legal
        .iter()
        .filter_map(operation_candidate)
        .filter(|candidate| candidate.kind == "cache")
        .filter_map(|candidate| {
            let site = candidate.primary_site()?;
            let site_view = site_view(view, site)?;
            Some(RankedCandidate {
                path: candidate.path,
                primary_site: site,
                rank: (
                    site_view.cache_count.saturating_add(1),
                    site_view.settlers.saturating_sub(site_view.agents),
                    reverse_site_index(site),
                    candidate.raw_path,
                ),
            })
        })
        .max_by(|left, right| left.rank.cmp(&right.rank))
}

fn best_freeholder_trek_to_cache(
    view: &PublicView,
    legal: &[ActionPath],
) -> Option<RankedCandidate> {
    legal
        .iter()
        .filter_map(operation_candidate)
        .filter(|candidate| candidate.kind == "trek")
        .filter_map(|candidate| {
            let target = candidate.primary_destination()?;
            let site_view = site_view(view, target)?;
            Some(RankedCandidate {
                path: candidate.path,
                primary_site: target,
                rank: (
                    u8::from(site_view.cache_count > 0 && site_view.agents > 0),
                    site_view.cache_count,
                    reverse_site_index(target),
                    candidate.raw_path,
                ),
            })
        })
        .max_by(|left, right| left.rank.cmp(&right.rank))
}

fn best_freeholder_spread(view: &PublicView, legal: &[ActionPath]) -> Option<RankedCandidate> {
    legal
        .iter()
        .filter_map(operation_candidate)
        .filter(|candidate| candidate.kind == "rally" || candidate.kind == "trek")
        .filter_map(|candidate| {
            let site = candidate
                .primary_destination()
                .or_else(|| candidate.primary_site())?;
            let site_view = site_view(view, site)?;
            Some(RankedCandidate {
                path: candidate.path,
                primary_site: site,
                rank: (
                    u8::from(site_view.agents > site_view.settlers),
                    u8::MAX - site_view.settlers,
                    reverse_site_index(site),
                    candidate.raw_path,
                ),
            })
        })
        .max_by(|left, right| left.rank.cmp(&right.rank))
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OperationCandidate {
    path: ActionPath,
    raw_path: String,
    kind: String,
    sites: Vec<SiteId>,
    destinations: Vec<SiteId>,
}

impl OperationCandidate {
    fn primary_site(&self) -> Option<SiteId> {
        self.sites.first().copied()
    }

    fn primary_destination(&self) -> Option<SiteId> {
        self.destinations.first().copied()
    }
}

fn operation_candidate(path: &ActionPath) -> Option<OperationCandidate> {
    let raw_path = single_segment(path);
    let parts = raw_path.split('/').collect::<Vec<_>>();
    let [family, kind, payload] = parts.as_slice() else {
        return None;
    };
    if *family != ACTION_OPERATION && *family != ACTION_LIMITED_OPERATION {
        return None;
    }
    let mut sites = Vec::new();
    let mut destinations = Vec::new();
    for part in payload.split(',') {
        if let Some((from, to)) = part.split_once('>') {
            sites.push(SiteId::parse(from)?);
            destinations.push(SiteId::parse(to)?);
        } else {
            sites.push(SiteId::parse(part)?);
        }
    }
    Some(OperationCandidate {
        path: simple_action(raw_path),
        raw_path: raw_path.to_owned(),
        kind: (*kind).to_owned(),
        sites,
        destinations,
    })
}

fn fallback_pass(
    legal: &[ActionPath],
    policy_id: &str,
    faction_label: &str,
    resource_label: &str,
) -> Result<BotDecision, Diagnostic> {
    if has_simple(legal, ACTION_PASS) {
        return Ok(decision(
            1,
            policy_id,
            simple_action(ACTION_PASS),
            format!(
                "{faction_label} passed because no public legal operation improved the position, saving {resource_label}."
            ),
        ));
    }
    Err(no_legal_actions())
}

fn current_event_favors(view: &PublicView, faction: FactionId) -> bool {
    view.current_card
        .as_deref()
        .is_some_and(|card| event_favors(card, faction))
}

fn event_favors(card: &str, faction: FactionId) -> bool {
    match faction {
        FactionId::Charter => matches!(
            card,
            "ef_border_survey"
                | "ef_depot_grants"
                | "ef_charter_audit"
                | "ef_agents_recall"
                | "ef_granite_pass_snows"
        ),
        FactionId::Freeholders => matches!(
            card,
            "ef_river_mists"
                | "ef_high_meadow_fair"
                | "ef_freeholder_moot"
                | "ef_old_mill_strike"
                | "ef_cache_boom"
        ),
    }
}

fn resource_for(view: &PublicView, faction: FactionId) -> u8 {
    match faction {
        FactionId::Charter => view.resources.funds,
        FactionId::Freeholders => view.resources.provisions,
    }
}

fn victory_distance_for(view: &PublicView, faction: FactionId) -> u8 {
    match faction {
        FactionId::Charter => view.victory_distance.charter_sites_needed,
        FactionId::Freeholders => view.victory_distance.freeholder_caches_needed,
    }
}

fn opposing_faction(faction: FactionId) -> FactionId {
    match faction {
        FactionId::Charter => FactionId::Freeholders,
        FactionId::Freeholders => FactionId::Charter,
    }
}

fn total_selected_caches(view: &PublicView, sites: &[SiteId]) -> u8 {
    sites
        .iter()
        .filter_map(|site| site_view(view, *site))
        .map(|site| site.cache_count)
        .fold(0u8, u8::saturating_add)
}

fn site_view(view: &PublicView, site: SiteId) -> Option<&SiteView> {
    view.sites.iter().find(|candidate| candidate.site == site)
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

fn has_simple(legal: &[ActionPath], segment: &str) -> bool {
    legal.iter().any(|path| single_segment(path) == segment)
}

fn simple_action(segment: &str) -> ActionPath {
    ActionPath {
        segments: vec![segment.to_owned()],
    }
}

fn single_segment(path: &ActionPath) -> &str {
    path.segments.first().map(String::as_str).unwrap_or("")
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
