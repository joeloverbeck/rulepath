//! State model for Event Frontier.

use engine_core::{FreshnessToken, SeatId, StableSerialize};

use crate::{
    cards::{CardId, EdictKind},
    ids::{FactionId, SiteId},
    variants::ScenarioVariant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Eligibility {
    Eligible,
    Ineligible,
}

impl Eligibility {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Eligible => "eligible",
            Self::Ineligible => "ineligible",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CardPhase {
    AwaitingFirstChoice {
        faction: FactionId,
    },
    AwaitingSecondChoice {
        first_faction: FactionId,
        second_faction: FactionId,
        first_choice: FirstChoice,
    },
    Reckoning,
    Terminal,
}

impl CardPhase {
    pub fn stable_summary(&self) -> String {
        match self {
            Self::AwaitingFirstChoice { faction } => {
                format!("awaiting_first_choice:{}", faction.as_str())
            }
            Self::AwaitingSecondChoice {
                first_faction,
                second_faction,
                first_choice,
            } => format!(
                "awaiting_second_choice:{}:{}:{}",
                first_faction.as_str(),
                second_faction.as_str(),
                first_choice.as_str()
            ),
            Self::Reckoning => "reckoning".to_owned(),
            Self::Terminal => "terminal".to_owned(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FirstChoice {
    Event,
    Operation,
    Pass,
}

impl FirstChoice {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Event => "event",
            Self::Operation => "operation",
            Self::Pass => "pass",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeckState {
    pub undrawn: Vec<CardId>,
    pub current: Option<CardId>,
    pub next_public: Option<CardId>,
    pub discard: Vec<CardId>,
    pub epoch: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SiteState {
    pub site: SiteId,
    pub agents: u8,
    pub settlers: u8,
    pub depot: bool,
    pub cache_count: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdjacencyEntry {
    pub site: SiteId,
    pub neighbors: Vec<SiteId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourcePools {
    pub funds: u8,
    pub provisions: u8,
}

impl ResourcePools {
    pub fn stable_summary(self) -> String {
        format!("funds={},provisions={}", self.funds, self.provisions)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FactionScores {
    pub charter: u16,
    pub freeholders: u16,
}

impl FactionScores {
    pub const fn zero() -> Self {
        Self {
            charter: 0,
            freeholders: 0,
        }
    }

    pub fn stable_summary(self) -> String {
        format!("charter={},freeholders={}", self.charter, self.freeholders)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveEdict {
    pub kind: EdictKind,
    pub card: CardId,
    pub activation_index: u8,
    pub expires_at_reckoning: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VictoryType {
    CharterInstant,
    FreeholderInstant,
    FinalFallback,
}

impl VictoryType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::CharterInstant => "charter_instant",
            Self::FreeholderInstant => "freeholder_instant",
            Self::FinalFallback => "final_fallback",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalOutcome {
    Winner {
        faction: FactionId,
        victory_type: VictoryType,
        decisive_rule: &'static str,
    },
}

impl TerminalOutcome {
    pub fn stable_summary(&self) -> String {
        match self {
            Self::Winner {
                faction,
                victory_type,
                decisive_rule,
            } => format!(
                "winner:{}:{}:{}",
                faction.as_str(),
                victory_type.as_str(),
                decisive_rule
            ),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventFrontierState {
    pub variant: ScenarioVariant,
    pub seats: [SeatId; 2],
    pub factions: [FactionId; 2],
    pub adjacency: Vec<AdjacencyEntry>,
    pub sites: Vec<SiteState>,
    pub resources: ResourcePools,
    pub deck: DeckState,
    pub eligibility: [Eligibility; 2],
    pub card_phase: CardPhase,
    pub active_edicts: Vec<ActiveEdict>,
    pub scores: FactionScores,
    pub reckoning_count: u8,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub freshness_token: FreshnessToken,
}

impl EventFrontierState {
    pub fn new_after_setup(
        variant: ScenarioVariant,
        seats: [SeatId; 2],
        adjacency: Vec<AdjacencyEntry>,
        deck_order: Vec<CardId>,
    ) -> Self {
        let factions = variant.faction_order;
        let current = deck_order.first().copied();
        let next_public = deck_order.get(1).copied();
        let undrawn = deck_order.into_iter().skip(2).collect::<Vec<_>>();

        Self {
            sites: SiteId::ALL
                .into_iter()
                .map(|site| SiteState {
                    site,
                    agents: count_for_site(&variant.start_agents, site),
                    settlers: count_for_site(&variant.start_settlers, site),
                    depot: variant.start_depots.contains(&site),
                    cache_count: count_for_site(&variant.start_caches, site),
                })
                .collect(),
            resources: ResourcePools {
                funds: variant.starting_resources.0,
                provisions: variant.starting_resources.1,
            },
            deck: DeckState {
                undrawn,
                current,
                next_public,
                discard: Vec::new(),
                epoch: current.map(epoch_for_card).unwrap_or(0),
            },
            variant,
            seats,
            factions,
            adjacency,
            eligibility: [Eligibility::Eligible, Eligibility::Eligible],
            card_phase: match current {
                Some(card) if is_reckoning(card) => CardPhase::Reckoning,
                Some(_) => CardPhase::AwaitingFirstChoice {
                    faction: FactionId::Charter,
                },
                None => CardPhase::Terminal,
            },
            active_edicts: Vec::new(),
            scores: FactionScores::zero(),
            reckoning_count: 0,
            terminal_outcome: None,
            freshness_token: FreshnessToken(0),
        }
    }

    pub fn site(&self, site: SiteId) -> Option<&SiteState> {
        self.sites.iter().find(|candidate| candidate.site == site)
    }

    pub fn site_mut(&mut self, site: SiteId) -> Option<&mut SiteState> {
        self.sites
            .iter_mut()
            .find(|candidate| candidate.site == site)
    }

    pub fn neighbors(&self, site: SiteId) -> Option<&[SiteId]> {
        self.adjacency
            .iter()
            .find(|entry| entry.site == site)
            .map(|entry| entry.neighbors.as_slice())
    }

    pub fn faction_for_seat(&self, seat: &SeatId) -> Option<FactionId> {
        self.seats
            .iter()
            .position(|candidate| candidate == seat)
            .map(|index| self.factions[index])
    }

    pub fn eligibility_for(&self, faction: FactionId) -> Eligibility {
        let index = faction_index(faction);
        self.eligibility[index]
    }

    pub fn set_eligibility(&mut self, faction: FactionId, eligibility: Eligibility) {
        let index = faction_index(faction);
        self.eligibility[index] = eligibility;
    }

    pub fn stable_summary(&self) -> String {
        EventFrontierSnapshot::from_state(self).stable_summary()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventFrontierSnapshot {
    pub schema_version: u32,
    pub rules_version: u32,
    pub rules_version_label: String,
    pub variant: ScenarioVariant,
    pub seats: [SeatId; 2],
    pub factions: [FactionId; 2],
    pub adjacency: Vec<AdjacencyEntry>,
    pub sites: Vec<SiteState>,
    pub resources: ResourcePools,
    pub deck: DeckState,
    pub eligibility: [Eligibility; 2],
    pub card_phase: CardPhase,
    pub active_edicts: Vec<ActiveEdict>,
    pub scores: FactionScores,
    pub reckoning_count: u8,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub freshness_token: FreshnessToken,
}

impl EventFrontierSnapshot {
    pub fn from_state(state: &EventFrontierState) -> Self {
        Self {
            schema_version: 1,
            rules_version: 1,
            rules_version_label: state.variant.rules_version_label.clone(),
            variant: state.variant.clone(),
            seats: state.seats.clone(),
            factions: state.factions,
            adjacency: state.adjacency.clone(),
            sites: state.sites.clone(),
            resources: state.resources,
            deck: state.deck.clone(),
            eligibility: state.eligibility,
            card_phase: state.card_phase.clone(),
            active_edicts: state.active_edicts.clone(),
            scores: state.scores,
            reckoning_count: state.reckoning_count,
            terminal_outcome: state.terminal_outcome.clone(),
            freshness_token: state.freshness_token,
        }
    }

    pub fn into_state(self) -> EventFrontierState {
        EventFrontierState {
            variant: self.variant,
            seats: self.seats,
            factions: self.factions,
            adjacency: self.adjacency,
            sites: self.sites,
            resources: self.resources,
            deck: self.deck,
            eligibility: self.eligibility,
            card_phase: self.card_phase,
            active_edicts: self.active_edicts,
            scores: self.scores,
            reckoning_count: self.reckoning_count,
            terminal_outcome: self.terminal_outcome,
            freshness_token: self.freshness_token,
        }
    }

    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};rules={};rules_label={};variant={};seat_count={};resource_cap={};thresholds=charter_sites:{},freeholder_caches:{};seats={}|{};factions={}|{};adjacency={};sites={};resources={};deck={};eligibility={};phase={};edicts={};scores={};reckonings={};terminal={};freshness={}",
            self.schema_version,
            self.rules_version,
            self.rules_version_label,
            self.variant.id,
            self.variant.seat_count,
            self.variant.resource_cap,
            self.variant.charter_site_threshold,
            self.variant.freeholder_cache_threshold,
            self.seats[0].0,
            self.seats[1].0,
            self.factions[0].as_str(),
            self.factions[1].as_str(),
            adjacency_summary(&self.adjacency),
            site_summary(&self.sites),
            self.resources.stable_summary(),
            deck_summary(&self.deck),
            eligibility_summary(&self.eligibility),
            self.card_phase.stable_summary(),
            edict_summary(&self.active_edicts),
            self.scores.stable_summary(),
            self.reckoning_count,
            self.terminal_outcome
                .as_ref()
                .map(TerminalOutcome::stable_summary)
                .unwrap_or_else(|| "none".to_owned()),
            self.freshness_token.0
        )
    }
}

impl StableSerialize for EventFrontierSnapshot {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

impl StableSerialize for EventFrontierState {
    fn stable_bytes(&self) -> Vec<u8> {
        EventFrontierSnapshot::from_state(self).stable_bytes()
    }
}

pub fn is_reckoning(card: CardId) -> bool {
    matches!(
        card,
        CardId::ReckoningOne | CardId::ReckoningTwo | CardId::ReckoningThree
    )
}

pub const fn epoch_for_card(card: CardId) -> u8 {
    match card {
        CardId::BorderSurvey
        | CardId::TollRoads
        | CardId::RiverMists
        | CardId::StorehouseFire
        | CardId::SurveyBan
        | CardId::HighMeadowFair
        | CardId::ReckoningOne => 1,
        CardId::DepotGrants
        | CardId::LongSeason
        | CardId::TrailWashout
        | CardId::CharterAudit
        | CardId::FreeholderMoot
        | CardId::Requisition
        | CardId::ReckoningTwo => 2,
        CardId::OldMillStrike
        | CardId::CrossingMarket
        | CardId::GranitePassSnows
        | CardId::CacheBoom
        | CardId::AgentsRecall
        | CardId::LastLight
        | CardId::ReckoningThree => 3,
    }
}

fn count_for_site(counts: &[(SiteId, u8)], site: SiteId) -> u8 {
    counts
        .iter()
        .find_map(|(candidate, count)| (*candidate == site).then_some(*count))
        .unwrap_or(0)
}

fn faction_index(faction: FactionId) -> usize {
    match faction {
        FactionId::Charter => 0,
        FactionId::Freeholders => 1,
    }
}

fn card_name(card: Option<CardId>) -> &'static str {
    card.map(CardId::as_str).unwrap_or("none")
}

fn card_list_summary(cards: &[CardId]) -> String {
    cards
        .iter()
        .map(|card| card.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn deck_summary(deck: &DeckState) -> String {
    format!(
        "epoch={};current={};next={};undrawn=[{}];discard=[{}]",
        deck.epoch,
        card_name(deck.current),
        card_name(deck.next_public),
        card_list_summary(&deck.undrawn),
        card_list_summary(&deck.discard)
    )
}

fn site_summary(sites: &[SiteState]) -> String {
    sites
        .iter()
        .map(|site| {
            format!(
                "{}:agents{}:settlers{}:depot{}:caches{}",
                site.site.as_str(),
                site.agents,
                site.settlers,
                u8::from(site.depot),
                site.cache_count
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn adjacency_summary(adjacency: &[AdjacencyEntry]) -> String {
    adjacency
        .iter()
        .map(|entry| {
            let neighbors = entry
                .neighbors
                .iter()
                .map(|site| site.as_str())
                .collect::<Vec<_>>()
                .join("|");
            format!("{}=[{}]", entry.site.as_str(), neighbors)
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn eligibility_summary(eligibility: &[Eligibility; 2]) -> String {
    format!(
        "{}={},{}={}",
        FactionId::Charter.as_str(),
        eligibility[0].as_str(),
        FactionId::Freeholders.as_str(),
        eligibility[1].as_str()
    )
}

fn edict_summary(edicts: &[ActiveEdict]) -> String {
    edicts
        .iter()
        .map(|edict| {
            format!(
                "{}:{}:{}:{}",
                edict.kind.as_str(),
                edict.card.as_str(),
                edict.activation_index,
                edict.expires_at_reckoning,
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup::{setup_match, SetupOptions};
    use engine_core::Seed;

    fn seats() -> [SeatId; 2] {
        [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
    }

    #[test]
    fn snapshot_round_trips_and_serializes_in_stable_order() {
        let state = setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup");
        let snapshot = EventFrontierSnapshot::from_state(&state);

        assert_eq!(snapshot.clone().into_state(), state);
        assert_eq!(
            snapshot.stable_bytes(),
            snapshot.stable_summary().into_bytes()
        );
        assert!(snapshot
            .stable_summary()
            .contains("eligibility=faction_charter=eligible,faction_freeholders=eligible"));
        assert!(snapshot.stable_summary().contains("undrawn=["));
    }
}
