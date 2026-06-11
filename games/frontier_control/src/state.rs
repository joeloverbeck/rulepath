//! State model for Frontier Control.

use engine_core::{FreshnessToken, SeatId, StableSerialize};

use crate::{
    ids::{FactionId, SiteId},
    variants::VariantMap,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase {
    Action { budget_remaining: u8 },
    Terminal,
}

impl Phase {
    pub fn stable_summary(self) -> String {
        match self {
            Self::Action { budget_remaining } => format!("action:{budget_remaining}"),
            Self::Terminal => "terminal".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SiteState {
    pub site: SiteId,
    pub guards: u8,
    pub crews: u8,
    pub stake: bool,
    pub fort: bool,
    pub stake_value: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdjacencyEntry {
    pub site: SiteId,
    pub neighbors: Vec<SiteId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FactionScores {
    pub garrison: u16,
    pub prospectors: u16,
}

impl FactionScores {
    pub const fn zero() -> Self {
        Self {
            garrison: 0,
            prospectors: 0,
        }
    }

    pub const fn score_for(self, faction: FactionId) -> u16 {
        match faction {
            FactionId::Garrison => self.garrison,
            FactionId::Prospectors => self.prospectors,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StakeSupplyStatus {
    pub site: SiteId,
    pub supplied: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalOutcome {
    Winner {
        faction: FactionId,
        scores: FactionScores,
        garrison_tiebreak: bool,
    },
}

impl TerminalOutcome {
    pub fn stable_summary(&self) -> String {
        match self {
            Self::Winner {
                faction,
                scores,
                garrison_tiebreak,
            } => format!(
                "winner:{}:{}:{}:tiebreak={}",
                faction.as_str(),
                scores.garrison,
                scores.prospectors,
                garrison_tiebreak
            ),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierControlState {
    pub variant: VariantMap,
    pub seats: [SeatId; 2],
    pub factions: [FactionId; 2],
    pub round_number: u8,
    pub active_faction: FactionId,
    pub phase: Phase,
    pub sites: Vec<SiteState>,
    pub adjacency: Vec<AdjacencyEntry>,
    pub scores: FactionScores,
    pub last_stake_supply: Vec<StakeSupplyStatus>,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub freshness_token: FreshnessToken,
}

impl FrontierControlState {
    pub fn new_after_setup(
        variant: VariantMap,
        seats: [SeatId; 2],
        adjacency: Vec<AdjacencyEntry>,
    ) -> Self {
        let factions = variant.faction_order;
        let sites = SiteId::ALL
            .into_iter()
            .map(|site| SiteState {
                site,
                guards: count_for_site(&variant.start_units.guards, site),
                crews: count_for_site(&variant.start_units.crews, site),
                stake: false,
                fort: variant.fort_sites.contains(&site),
                stake_value: stake_value_for_site(&variant.stake_values, site),
            })
            .collect();

        Self {
            phase: Phase::Action {
                budget_remaining: variant.action_budget,
            },
            variant,
            seats,
            factions,
            round_number: 1,
            active_faction: FactionId::Prospectors,
            sites,
            adjacency,
            scores: FactionScores::zero(),
            last_stake_supply: Vec::new(),
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

    pub fn active_seat(&self) -> Option<&SeatId> {
        self.factions
            .iter()
            .position(|faction| *faction == self.active_faction)
            .map(|index| &self.seats[index])
    }

    pub fn sites_are_adjacent(&self, left: SiteId, right: SiteId) -> bool {
        self.neighbors(left)
            .map(|neighbors| neighbors.contains(&right))
            .unwrap_or(false)
    }

    pub fn stable_summary(&self) -> String {
        format!(
            "variant={};seats={}|{};factions={}|{};round={};active={};phase={};sites={};adjacency={};scores={}:{};supply={};terminal={};freshness={}",
            self.variant.id,
            self.seats[0].0,
            self.seats[1].0,
            self.factions[0].as_str(),
            self.factions[1].as_str(),
            self.round_number,
            self.active_faction.as_str(),
            self.phase.stable_summary(),
            site_summary(&self.sites),
            adjacency_summary(&self.adjacency),
            self.scores.garrison,
            self.scores.prospectors,
            supply_summary(&self.last_stake_supply),
            self.terminal_outcome
                .as_ref()
                .map(TerminalOutcome::stable_summary)
                .unwrap_or_else(|| "none".to_owned()),
            self.freshness_token.0
        )
    }
}

impl StableSerialize for FrontierControlState {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

fn count_for_site(counts: &[(SiteId, u8)], site: SiteId) -> u8 {
    counts
        .iter()
        .find_map(|(candidate, count)| (*candidate == site).then_some(*count))
        .unwrap_or(0)
}

fn stake_value_for_site(values: &[(SiteId, u8)], site: SiteId) -> u8 {
    count_for_site(values, site)
}

fn site_summary(sites: &[SiteState]) -> String {
    sites
        .iter()
        .map(|site| {
            format!(
                "{}:g{}:c{}:stake{}:fort{}:value{}",
                site.site.as_str(),
                site.guards,
                site.crews,
                u8::from(site.stake),
                u8::from(site.fort),
                site.stake_value
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

fn supply_summary(supply: &[StakeSupplyStatus]) -> String {
    supply
        .iter()
        .map(|entry| format!("{}:{}", entry.site.as_str(), u8::from(entry.supplied)))
        .collect::<Vec<_>>()
        .join(",")
}
