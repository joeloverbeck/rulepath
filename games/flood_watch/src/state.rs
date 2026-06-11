//! State model for Flood Watch.

use engine_core::{FreshnessToken, SeatId, StableSerialize};

use crate::{
    ids::{DistrictId, EventKind, FloodWatchRole},
    variants::{EventComposition, ScenarioVariant},
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
pub struct DistrictState {
    pub district: DistrictId,
    pub flood_level: u8,
    pub levees: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventCard {
    pub kind: EventKind,
    pub copy_index: u8,
}

impl EventCard {
    pub fn stable_id(&self) -> String {
        format!("{}#{}", self.kind.id(), self.copy_index)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SharedOutcome {
    Won,
    Lost { district: DistrictId },
}

impl SharedOutcome {
    pub fn stable_summary(&self) -> String {
        match self {
            Self::Won => "won".to_owned(),
            Self::Lost { district } => format!("lost:{}", district.as_str()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StableComposition {
    pub downpours_per_district: Vec<(DistrictId, u8)>,
    pub surges_per_district: Vec<(DistrictId, u8)>,
    pub reprieves: u8,
}

impl StableComposition {
    pub fn from_scenario(composition: &EventComposition) -> Self {
        Self {
            downpours_per_district: DistrictId::ALL
                .into_iter()
                .map(|district| (district, composition.downpours_per_district))
                .collect(),
            surges_per_district: DistrictId::ALL
                .into_iter()
                .map(|district| (district, composition.surges_per_district))
                .collect(),
            reprieves: composition.reprieves,
        }
    }

    pub fn subtract_drawn(&mut self, drawn: &[EventCard]) {
        for card in drawn {
            match card.kind {
                EventKind::Downpour { district } => {
                    decrement_district_count(&mut self.downpours_per_district, district);
                }
                EventKind::StormSurge { district } => {
                    decrement_district_count(&mut self.surges_per_district, district);
                }
                EventKind::Reprieve => {
                    self.reprieves = self.reprieves.saturating_sub(1);
                }
            }
        }
    }

    pub fn total_cards(&self) -> u8 {
        self.downpours_per_district
            .iter()
            .chain(self.surges_per_district.iter())
            .map(|(_, count)| *count)
            .sum::<u8>()
            + self.reprieves
    }

    pub fn stable_summary(&self) -> String {
        let downpours = self
            .downpours_per_district
            .iter()
            .map(|(district, count)| format!("{}:{count}", district.as_str()))
            .collect::<Vec<_>>()
            .join(",");
        let surges = self
            .surges_per_district
            .iter()
            .map(|(district, count)| format!("{}:{count}", district.as_str()))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "downpours=[{downpours}];surges=[{surges}];reprieves={}",
            self.reprieves
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FloodWatchState {
    pub variant: ScenarioVariant,
    pub seats: [SeatId; 2],
    pub roles: [FloodWatchRole; 2],
    pub turn_number: u16,
    pub active_seat: SeatId,
    pub phase: Phase,
    pub districts: Vec<DistrictState>,
    event_deck: Vec<EventCard>,
    pub drawn: Vec<EventCard>,
    pub forecast: Option<EventCard>,
    pub terminal_outcome: Option<SharedOutcome>,
    pub freshness_token: FreshnessToken,
}

impl FloodWatchState {
    pub fn new_after_setup(
        variant: ScenarioVariant,
        seats: [SeatId; 2],
        event_deck: Vec<EventCard>,
    ) -> Self {
        let districts = DistrictId::ALL
            .into_iter()
            .zip(variant.starting_levels)
            .map(|(district, flood_level)| DistrictState {
                district,
                flood_level,
                levees: 0,
            })
            .collect();
        let active_seat = seats[0].clone();

        Self {
            roles: variant.role_order,
            phase: Phase::Action {
                budget_remaining: variant.action_budget,
            },
            variant,
            seats,
            turn_number: 1,
            active_seat,
            districts,
            event_deck,
            drawn: Vec::new(),
            forecast: None,
            terminal_outcome: None,
            freshness_token: FreshnessToken(0),
        }
    }

    pub fn event_deck_internal(&self) -> &[EventCard] {
        &self.event_deck
    }

    pub fn top_undrawn_card(&self) -> Option<&EventCard> {
        self.event_deck.first()
    }

    pub fn undrawn_deck_len(&self) -> usize {
        self.event_deck.len()
    }

    pub fn district(&self, district: DistrictId) -> Option<&DistrictState> {
        self.districts
            .iter()
            .find(|candidate| candidate.district == district)
    }

    pub fn district_mut(&mut self, district: DistrictId) -> Option<&mut DistrictState> {
        self.districts
            .iter_mut()
            .find(|candidate| candidate.district == district)
    }

    pub fn seat_index(&self, seat: &SeatId) -> Option<usize> {
        self.seats.iter().position(|candidate| candidate == seat)
    }

    pub fn active_role(&self) -> Option<FloodWatchRole> {
        self.seat_index(&self.active_seat)
            .map(|index| self.roles[index])
    }

    pub fn remaining_composition(&self) -> StableComposition {
        let mut composition = StableComposition::from_scenario(&self.variant.event_composition);
        composition.subtract_drawn(&self.drawn);
        composition
    }

    pub fn stable_summary(&self) -> String {
        format!(
            "variant={};seats={}|{};roles={}|{};turn={};active={};phase={};districts={};deck={};drawn={};forecast={};terminal={};freshness={}",
            self.variant.id,
            self.seats[0].0,
            self.seats[1].0,
            self.roles[0].as_str(),
            self.roles[1].as_str(),
            self.turn_number,
            self.active_seat.0,
            self.phase.stable_summary(),
            district_summary(&self.districts),
            event_list_summary(&self.event_deck),
            event_list_summary(&self.drawn),
            self.forecast
                .as_ref()
                .map(EventCard::stable_id)
                .unwrap_or_else(|| "none".to_owned()),
            self.terminal_outcome
                .as_ref()
                .map(SharedOutcome::stable_summary)
                .unwrap_or_else(|| "none".to_owned()),
            self.freshness_token.0
        )
    }
}

impl StableSerialize for FloodWatchState {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

fn decrement_district_count(counts: &mut [(DistrictId, u8)], district: DistrictId) {
    if let Some((_, count)) = counts
        .iter_mut()
        .find(|(candidate, _)| *candidate == district)
    {
        *count = count.saturating_sub(1);
    }
}

fn district_summary(districts: &[DistrictState]) -> String {
    districts
        .iter()
        .map(|district| {
            format!(
                "{}:{}:{}",
                district.district.as_str(),
                district.flood_level,
                district.levees
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn event_list_summary(events: &[EventCard]) -> String {
    events
        .iter()
        .map(EventCard::stable_id)
        .collect::<Vec<_>>()
        .join(",")
}
