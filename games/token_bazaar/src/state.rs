use engine_core::{FreshnessToken, SeatId, StableSerialize};

use crate::{
    ids::{ContractId, ResourceId, TokenBazaarSeat, TokenBazaarSlot},
    variants::Variant,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct ResourceCounts {
    pub amber: u8,
    pub jade: u8,
    pub iron: u8,
}

impl ResourceCounts {
    pub const fn new(amber: u8, jade: u8, iron: u8) -> Self {
        Self { amber, jade, iron }
    }

    pub const fn get(self, resource: ResourceId) -> u8 {
        match resource {
            ResourceId::Amber => self.amber,
            ResourceId::Jade => self.jade,
            ResourceId::Iron => self.iron,
        }
    }

    pub fn set(&mut self, resource: ResourceId, value: u8) {
        match resource {
            ResourceId::Amber => self.amber = value,
            ResourceId::Jade => self.jade = value,
            ResourceId::Iron => self.iron = value,
        }
    }

    pub const fn total(self) -> u16 {
        self.amber as u16 + self.jade as u16 + self.iron as u16
    }

    pub fn stable_summary(self) -> String {
        format!("amber={},jade={},iron={}", self.amber, self.jade, self.iron)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ContractSpec {
    pub id: ContractId,
    pub label: &'static str,
    pub cost: ResourceCounts,
    pub points: u8,
}

pub const STANDARD_CONTRACTS: [ContractSpec; 10] = [
    ContractSpec {
        id: ContractId::BalancedWares,
        label: "Balanced Wares",
        cost: ResourceCounts::new(1, 1, 1),
        points: 3,
    },
    ContractSpec {
        id: ContractId::AmberGuild,
        label: "Amber Guild",
        cost: ResourceCounts::new(2, 1, 0),
        points: 3,
    },
    ContractSpec {
        id: ContractId::IronGuild,
        label: "Iron Guild",
        cost: ResourceCounts::new(1, 0, 2),
        points: 3,
    },
    ContractSpec {
        id: ContractId::JadeGuild,
        label: "Jade Guild",
        cost: ResourceCounts::new(0, 2, 1),
        points: 3,
    },
    ContractSpec {
        id: ContractId::AmberFocus,
        label: "Amber Focus",
        cost: ResourceCounts::new(3, 0, 0),
        points: 4,
    },
    ContractSpec {
        id: ContractId::JadeFocus,
        label: "Jade Focus",
        cost: ResourceCounts::new(0, 3, 0),
        points: 4,
    },
    ContractSpec {
        id: ContractId::IronFocus,
        label: "Iron Focus",
        cost: ResourceCounts::new(0, 0, 3),
        points: 4,
    },
    ContractSpec {
        id: ContractId::SunRoute,
        label: "Sun Route",
        cost: ResourceCounts::new(2, 2, 0),
        points: 5,
    },
    ContractSpec {
        id: ContractId::StoneRoute,
        label: "Stone Route",
        cost: ResourceCounts::new(0, 2, 2),
        points: 5,
    },
    ContractSpec {
        id: ContractId::CrownRoute,
        label: "Crown Route",
        cost: ResourceCounts::new(2, 0, 2),
        points: 5,
    },
];

pub fn contract_spec(id: ContractId) -> &'static ContractSpec {
    STANDARD_CONTRACTS
        .iter()
        .find(|contract| contract.id == id)
        .expect("all contract ids have specs")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TerminalOutcome {
    Win { seat: TokenBazaarSeat },
    Draw,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TerminalTrigger {
    TurnCap,
    MarketExhaustion,
}

impl TerminalTrigger {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TurnCap => "turn_cap",
            Self::MarketExhaustion => "market_exhaustion",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TiebreakRung {
    Score,
    FulfilledContracts,
    InventoryTotal,
    AllTiedDraw,
}

impl TiebreakRung {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Score => "score",
            Self::FulfilledContracts => "fulfilled_contracts",
            Self::InventoryTotal => "inventory_total",
            Self::AllTiedDraw => "all_tied_draw",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenBazaarState {
    pub variant: Variant,
    pub seats: [SeatId; 2],
    pub supply: ResourceCounts,
    pub inventories: [ResourceCounts; 2],
    pub scores: [u32; 2],
    pub slots: [Option<ContractId>; 3],
    pub queue: Vec<ContractId>,
    pub fulfilled: [Vec<ContractId>; 2],
    pub turns_taken: [u8; 2],
    pub active_seat: TokenBazaarSeat,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub terminal_trigger: Option<TerminalTrigger>,
    pub freshness_token: FreshnessToken,
}

impl TokenBazaarState {
    pub fn inventory_for(&self, seat: TokenBazaarSeat) -> ResourceCounts {
        self.inventories[seat.index()]
    }

    pub fn score_for(&self, seat: TokenBazaarSeat) -> u32 {
        self.scores[seat.index()]
    }

    pub fn slot_contract(&self, slot: TokenBazaarSlot) -> Option<ContractId> {
        self.slots[slot.index()]
    }

    pub fn fulfilled_for(&self, seat: TokenBazaarSeat) -> &[ContractId] {
        &self.fulfilled[seat.index()]
    }

    pub fn fulfilled_counts(&self) -> [u8; 2] {
        [self.fulfilled[0].len() as u8, self.fulfilled[1].len() as u8]
    }

    pub fn inventory_totals(&self) -> [u16; 2] {
        [self.inventories[0].total(), self.inventories[1].total()]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenBazaarSnapshot {
    pub schema_version: u32,
    pub rules_version: u32,
    pub rules_version_label: String,
    pub variant: Variant,
    pub seats: [SeatId; 2],
    pub supply: ResourceCounts,
    pub inventories: [ResourceCounts; 2],
    pub scores: [u32; 2],
    pub slots: [Option<ContractId>; 3],
    pub queue: Vec<ContractId>,
    pub fulfilled: [Vec<ContractId>; 2],
    pub turns_taken: [u8; 2],
    pub active_seat: TokenBazaarSeat,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub terminal_trigger: Option<TerminalTrigger>,
    pub freshness_token: FreshnessToken,
}

impl TokenBazaarSnapshot {
    pub fn from_state(state: &TokenBazaarState) -> Self {
        Self {
            schema_version: 1,
            rules_version: 1,
            rules_version_label: state.variant.rules_version_label.clone(),
            variant: state.variant.clone(),
            seats: state.seats.clone(),
            supply: state.supply,
            inventories: state.inventories,
            scores: state.scores,
            slots: state.slots,
            queue: state.queue.clone(),
            fulfilled: state.fulfilled.clone(),
            turns_taken: state.turns_taken,
            active_seat: state.active_seat,
            terminal_outcome: state.terminal_outcome,
            terminal_trigger: state.terminal_trigger,
            freshness_token: state.freshness_token,
        }
    }

    pub fn into_state(self) -> TokenBazaarState {
        TokenBazaarState {
            variant: self.variant,
            seats: self.seats,
            supply: self.supply,
            inventories: self.inventories,
            scores: self.scores,
            slots: self.slots,
            queue: self.queue,
            fulfilled: self.fulfilled,
            turns_taken: self.turns_taken,
            active_seat: self.active_seat,
            terminal_outcome: self.terminal_outcome,
            terminal_trigger: self.terminal_trigger,
            freshness_token: self.freshness_token,
        }
    }

    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};rules={};rules_label={};variant={};seat_count={};first_active={};resource_supply={};starting_resource_count={};market_slot_count={};contract_count={};turns_per_seat={};contract_order={};terminal_scoring={};seat_0={};seat_1={};supply={};inv_0={};inv_1={};score_0={};score_1={};slots={};queue={};fulfilled_0={};fulfilled_1={};turns_0={};turns_1={};active={};terminal={};terminal_trigger={};freshness={}",
            self.schema_version,
            self.rules_version,
            self.rules_version_label,
            self.variant.id,
            self.variant.seat_count,
            self.variant.first_active_seat,
            self.variant.resource_supply,
            self.variant.starting_resource_count,
            self.variant.market_slot_count,
            self.variant.contract_count,
            self.variant.turns_per_seat,
            self.variant.contract_order,
            self.variant.terminal_scoring,
            self.seats[0].0,
            self.seats[1].0,
            self.supply.stable_summary(),
            self.inventories[0].stable_summary(),
            self.inventories[1].stable_summary(),
            self.scores[0],
            self.scores[1],
            slot_summary(&self.slots),
            contract_list_summary(&self.queue),
            contract_list_summary(&self.fulfilled[0]),
            contract_list_summary(&self.fulfilled[1]),
            self.turns_taken[0],
            self.turns_taken[1],
            self.active_seat.as_str(),
            terminal_summary(self.terminal_outcome),
            self.terminal_trigger.map_or("none", TerminalTrigger::as_str),
            self.freshness_token.0
        )
    }
}

impl StableSerialize for TokenBazaarSnapshot {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

fn slot_summary(slots: &[Option<ContractId>; 3]) -> String {
    TokenBazaarSlot::ALL
        .iter()
        .map(|slot| {
            let value = slots[slot.index()]
                .map(ContractId::as_str)
                .unwrap_or("empty");
            format!("{}:{value}", slot.as_str())
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn contract_list_summary(contracts: &[ContractId]) -> String {
    contracts
        .iter()
        .map(|contract| contract.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn terminal_summary(outcome: Option<TerminalOutcome>) -> String {
    match outcome {
        None => "none".to_owned(),
        Some(TerminalOutcome::Draw) => "draw".to_owned(),
        Some(TerminalOutcome::Win { seat }) => format!("win:{}", seat.as_str()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variants::Variant;

    #[test]
    fn contract_specs_are_in_standard_queue_order() {
        assert_eq!(STANDARD_CONTRACTS.len(), 10);
        assert_eq!(STANDARD_CONTRACTS[0].id, ContractId::BalancedWares);
        assert_eq!(STANDARD_CONTRACTS[0].cost, ResourceCounts::new(1, 1, 1));
        assert_eq!(STANDARD_CONTRACTS[9].id, ContractId::CrownRoute);
        assert_eq!(STANDARD_CONTRACTS[9].cost, ResourceCounts::new(2, 0, 2));
    }

    #[test]
    fn snapshot_round_trips_and_serializes_in_stable_order() {
        let state = TokenBazaarState {
            variant: Variant::token_bazaar_standard(),
            seats: [SeatId("a".to_owned()), SeatId("b".to_owned())],
            supply: ResourceCounts::new(14, 14, 14),
            inventories: [ResourceCounts::new(1, 1, 1), ResourceCounts::new(1, 1, 1)],
            scores: [0, 0],
            slots: [
                Some(ContractId::BalancedWares),
                Some(ContractId::AmberGuild),
                Some(ContractId::IronGuild),
            ],
            queue: vec![ContractId::JadeGuild, ContractId::AmberFocus],
            fulfilled: [Vec::new(), vec![ContractId::BalancedWares]],
            turns_taken: [0, 1],
            active_seat: TokenBazaarSeat::Seat1,
            terminal_outcome: None,
            terminal_trigger: None,
            freshness_token: FreshnessToken(0),
        };

        let snapshot = TokenBazaarSnapshot::from_state(&state);

        assert_eq!(snapshot.clone().into_state(), state);
        assert_eq!(
            snapshot.stable_bytes(),
            snapshot.stable_summary().into_bytes()
        );
        assert!(snapshot
            .stable_summary()
            .contains("slots=slot_0:balanced-wares,slot_1:amber-guild,slot_2:iron-guild"));
        assert!(snapshot
            .stable_summary()
            .contains("queue=jade-guild,amber-focus"));
    }
}
