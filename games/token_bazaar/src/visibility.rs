use engine_core::{Actor, FreshnessToken, StableSerialize, Viewer};

use crate::{
    actions::legal_action_tree,
    effects::TokenBazaarEffect,
    ids::{
        ContractId, TokenBazaarSeat, TokenBazaarSlot, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID,
    },
    state::{contract_spec, ResourceCounts, TerminalOutcome, TokenBazaarState},
    ui::{slot_accessibility_label, ui_metadata, UiMetadata},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicView {
    pub schema_version: u32,
    pub rules_version: u32,
    pub game_id: String,
    pub display_name: String,
    pub variant_id: String,
    pub rules_version_label: String,
    pub supply: ResourceSupplyView,
    pub inventories: [InventoryView; 2],
    pub scores: [u32; 2],
    pub turns_taken: [u8; 2],
    pub turns_per_seat: u8,
    pub active_seat: Option<TokenBazaarSeat>,
    pub market_slots: Vec<MarketSlotView>,
    pub queue_remaining: u8,
    pub fulfilled: [Vec<String>; 2],
    pub legal_actions: Vec<LegalActionView>,
    pub terminal: TerminalView,
    pub freshness_token: FreshnessToken,
    pub recent_effects: Vec<EffectView>,
    pub private_view_status: String,
    pub hidden_fields: Vec<String>,
    pub ui: UiMetadata,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceSupplyView {
    pub amber: u8,
    pub jade: u8,
    pub iron: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InventoryView {
    pub seat: TokenBazaarSeat,
    pub resources: ResourceSupplyView,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MarketSlotView {
    pub slot: TokenBazaarSlot,
    pub slot_id: String,
    pub contract: Option<ContractView>,
    pub is_empty: bool,
    pub accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractView {
    pub contract_id: String,
    pub label: String,
    pub cost: ResourceSupplyView,
    pub points: u8,
    pub accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegalActionView {
    pub action_segment: String,
    pub label: String,
    pub accessibility_label: String,
    pub metadata: Vec<(String, String)>,
    pub freshness_token: FreshnessToken,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalView {
    NonTerminal,
    Win { winning_seat: TokenBazaarSeat },
    Draw,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectView {
    pub kind: String,
    pub summary: String,
}

pub fn project_view(state: &TokenBazaarState, viewer: &Viewer) -> PublicView {
    project_view_with_effects(state, viewer, &[])
}

pub fn project_view_with_effects(
    state: &TokenBazaarState,
    _viewer: &Viewer,
    recent_effects: &[TokenBazaarEffect],
) -> PublicView {
    PublicView {
        schema_version: 1,
        rules_version: 1,
        game_id: GAME_ID.to_owned(),
        display_name: "Token Bazaar".to_owned(),
        variant_id: VARIANT_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        supply: supply_view(state.supply),
        inventories: [
            inventory_view(state, TokenBazaarSeat::Seat0),
            inventory_view(state, TokenBazaarSeat::Seat1),
        ],
        scores: state.scores,
        turns_taken: state.turns_taken,
        turns_per_seat: state.variant.turns_per_seat,
        active_seat: state
            .terminal_outcome
            .is_none()
            .then_some(state.active_seat),
        market_slots: TokenBazaarSlot::ALL
            .into_iter()
            .map(|slot| market_slot_view(state, slot))
            .collect(),
        queue_remaining: state.queue.len() as u8,
        fulfilled: [
            state.fulfilled[0]
                .iter()
                .map(|contract| contract.as_str().to_owned())
                .collect(),
            state.fulfilled[1]
                .iter()
                .map(|contract| contract.as_str().to_owned())
                .collect(),
        ],
        legal_actions: legal_action_views(state),
        terminal: terminal_view(state.terminal_outcome),
        freshness_token: state.freshness_token,
        recent_effects: recent_effects
            .iter()
            .map(|effect| EffectView {
                kind: effect.kind().to_owned(),
                summary: effect.stable_summary(),
            })
            .collect(),
        private_view_status: "not_applicable_all_state_public".to_owned(),
        hidden_fields: Vec::new(),
        ui: ui_metadata(),
    }
}

impl PublicView {
    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};rules={};game={};variant={};label={};supply={};inventories={}|{};scores={}-{};turns={}-{}/{};active={};slots={};queue={};fulfilled={}|{};legal={};terminal={};freshness={};effects={};private={};hidden={};ui={}",
            self.schema_version,
            self.rules_version,
            self.game_id,
            self.variant_id,
            self.rules_version_label,
            encode_supply(self.supply),
            encode_inventory(&self.inventories[0]),
            encode_inventory(&self.inventories[1]),
            self.scores[0],
            self.scores[1],
            self.turns_taken[0],
            self.turns_taken[1],
            self.turns_per_seat,
            self.active_seat.map_or("none", TokenBazaarSeat::as_str),
            self.market_slots.iter().map(encode_slot).collect::<Vec<_>>().join(","),
            self.queue_remaining,
            self.fulfilled[0].join(","),
            self.fulfilled[1].join(","),
            self.legal_actions.iter().map(encode_legal).collect::<Vec<_>>().join(","),
            encode_terminal(&self.terminal),
            self.freshness_token.0,
            self.recent_effects.iter().map(encode_effect).collect::<Vec<_>>().join(","),
            self.private_view_status,
            self.hidden_fields.join(","),
            encode_ui(&self.ui)
        )
    }
}

impl StableSerialize for PublicView {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

fn supply_view(counts: ResourceCounts) -> ResourceSupplyView {
    ResourceSupplyView {
        amber: counts.amber,
        jade: counts.jade,
        iron: counts.iron,
    }
}

fn inventory_view(state: &TokenBazaarState, seat: TokenBazaarSeat) -> InventoryView {
    InventoryView {
        seat,
        resources: supply_view(state.inventory_for(seat)),
    }
}

fn market_slot_view(state: &TokenBazaarState, slot: TokenBazaarSlot) -> MarketSlotView {
    let contract_id = state.slot_contract(slot);
    MarketSlotView {
        slot,
        slot_id: slot.as_str().to_owned(),
        contract: contract_id.map(contract_view),
        is_empty: contract_id.is_none(),
        accessibility_label: slot_accessibility_label(slot, contract_id),
    }
}

fn contract_view(contract: ContractId) -> ContractView {
    let spec = contract_spec(contract);
    ContractView {
        contract_id: spec.id.as_str().to_owned(),
        label: spec.label.to_owned(),
        cost: supply_view(spec.cost),
        points: spec.points,
        accessibility_label: crate::ui::contract_accessibility_label(contract),
    }
}

fn legal_action_views(state: &TokenBazaarState) -> Vec<LegalActionView> {
    if state.terminal_outcome.is_some() {
        return Vec::new();
    }

    let tree = legal_action_tree(
        state,
        &Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        },
    );
    tree.root
        .choices
        .into_iter()
        .map(|choice| LegalActionView {
            action_segment: choice.segment,
            label: choice.label,
            accessibility_label: choice.accessibility_label,
            metadata: choice
                .metadata
                .into_iter()
                .map(|entry| (entry.key, entry.value))
                .collect(),
            freshness_token: state.freshness_token,
        })
        .collect()
}

fn terminal_view(outcome: Option<TerminalOutcome>) -> TerminalView {
    match outcome {
        None => TerminalView::NonTerminal,
        Some(TerminalOutcome::Win { seat }) => TerminalView::Win { winning_seat: seat },
        Some(TerminalOutcome::Draw) => TerminalView::Draw,
    }
}

fn encode_supply(supply: ResourceSupplyView) -> String {
    format!(
        "amber={},jade={},iron={}",
        supply.amber, supply.jade, supply.iron
    )
}

fn encode_inventory(inventory: &InventoryView) -> String {
    format!(
        "{}:{}",
        inventory.seat.as_str(),
        encode_supply(inventory.resources)
    )
}

fn encode_slot(slot: &MarketSlotView) -> String {
    format!(
        "{}:{}:{}",
        slot.slot_id,
        if slot.is_empty { "empty" } else { "occupied" },
        slot.contract
            .as_ref()
            .map_or_else(|| "none".to_owned(), encode_contract)
    )
}

fn encode_contract(contract: &ContractView) -> String {
    format!(
        "{}:{}:{}:{}",
        contract.contract_id,
        contract.label,
        encode_supply(contract.cost),
        contract.points
    )
}

fn encode_legal(action: &LegalActionView) -> String {
    let metadata = action
        .metadata
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join("|");
    format!(
        "{}:{}:{}:{}",
        action.action_segment, action.label, action.freshness_token.0, metadata
    )
}

fn encode_terminal(terminal: &TerminalView) -> String {
    match terminal {
        TerminalView::NonTerminal => "non_terminal".to_owned(),
        TerminalView::Win { winning_seat } => format!("win:{}", winning_seat.as_str()),
        TerminalView::Draw => "draw".to_owned(),
    }
}

fn encode_effect(effect: &EffectView) -> String {
    format!("{}:{}", effect.kind, effect.summary)
}

fn encode_ui(ui: &UiMetadata) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}",
        ui.table_label,
        ui.supply_label,
        ui.inventory_label,
        ui.market_label,
        ui.score_label,
        ui.turn_counter_label,
        ui.reduced_motion_token
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{effects::TokenBazaarEffect, setup::setup_match};
    use engine_core::{SeatId, Seed, StableSerialize};

    fn state() -> TokenBazaarState {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        setup_match(Seed(1), &seats, &Default::default()).expect("setup succeeds")
    }

    #[test]
    fn public_view_exposes_required_board_fields() {
        let state = state();
        let view = project_view(&state, &Viewer { seat_id: None });

        assert_eq!(view.game_id, GAME_ID);
        assert_eq!(
            view.supply,
            ResourceSupplyView {
                amber: 14,
                jade: 14,
                iron: 14
            }
        );
        assert_eq!(
            view.inventories[0].resources,
            ResourceSupplyView {
                amber: 1,
                jade: 1,
                iron: 1
            }
        );
        assert_eq!(view.scores, [0, 0]);
        assert_eq!(view.turns_taken, [0, 0]);
        assert_eq!(view.active_seat, Some(TokenBazaarSeat::Seat0));
        assert_eq!(view.market_slots.len(), 3);
        assert_eq!(
            view.market_slots[0]
                .contract
                .as_ref()
                .map(|contract| contract.contract_id.as_str()),
            Some("balanced-wares")
        );
        assert_eq!(view.market_slots[0].contract.as_ref().unwrap().points, 3);
        assert_eq!(view.queue_remaining, 7);
        assert!(!view.legal_actions.is_empty());
        assert_eq!(view.private_view_status, "not_applicable_all_state_public");
        assert!(view.hidden_fields.is_empty());
        assert_eq!(view.stable_bytes(), view.stable_summary().into_bytes());
    }

    #[test]
    fn observer_and_seat_views_are_identical() {
        let state = state();
        let observer = project_view(&state, &Viewer { seat_id: None });
        let seat = project_view(
            &state,
            &Viewer {
                seat_id: Some(state.seats[0].clone()),
            },
        );

        assert_eq!(observer, seat);
    }

    #[test]
    fn recent_effects_are_public_summaries_without_debug_fields() {
        let state = state();
        let effect = TokenBazaarEffect::ResourceCollected {
            seat: TokenBazaarSeat::Seat0,
            bundle: crate::CollectBundleId::Amber,
            gain: ResourceCounts::new(2, 0, 0),
            inventory_after: ResourceCounts::new(3, 1, 1),
            supply_after: ResourceCounts::new(12, 14, 14),
        };
        let view = project_view_with_effects(&state, &Viewer { seat_id: None }, &[effect]);
        let summary = view.stable_summary();

        assert_eq!(view.recent_effects[0].kind, "tb_resource_collected");
        assert!(!summary.contains("debug"));
        assert!(!summary.contains("candidate"));
        assert!(!summary.contains("valuation"));
        assert!(!summary.contains("internal"));
    }
}
