use crate::{
    actions::{ActionFamily, TokenBazaarAction},
    ids::{CollectBundleId, ContractId, ResourceId, TokenBazaarSlot},
    state::contract_spec,
};

pub const SEAT_LABEL_AUDIT: &str =
    "Token Bazaar is factionless; keep existing player/seat naming for inventories.";
pub const TURN_REPORT_AUDIT: &str =
    "Not adopted by ACTCONMAT-006; Token Bazaar accounting narration is outside this ticket.";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMetadata {
    pub table_label: String,
    pub supply_label: String,
    pub inventory_label: String,
    pub market_label: String,
    pub score_label: String,
    pub turn_counter_label: String,
    pub reduced_motion_token: String,
}

pub fn ui_metadata() -> UiMetadata {
    UiMetadata {
        table_label: "Token Bazaar market".to_owned(),
        supply_label: "Public supply".to_owned(),
        inventory_label: "Player inventory".to_owned(),
        market_label: "Visible contracts".to_owned(),
        score_label: "Score".to_owned(),
        turn_counter_label: "Turns taken".to_owned(),
        reduced_motion_token: "resource-accounting-reduced-motion".to_owned(),
    }
}

pub fn resource_accessibility_label(resource: ResourceId, amount: u8) -> String {
    format!("{amount} {}", resource.as_str())
}

pub fn contract_accessibility_label(contract: ContractId) -> String {
    let spec = contract_spec(contract);
    format!(
        "{} contract, worth {} points, costs {}",
        spec.label,
        spec.points,
        resource_cost_label(spec.cost)
    )
}

pub fn slot_accessibility_label(slot: TokenBazaarSlot, contract: Option<ContractId>) -> String {
    match contract {
        Some(contract) => format!(
            "{} contains {}",
            slot.as_str(),
            contract_accessibility_label(contract)
        ),
        None => format!("{} is empty", slot.as_str()),
    }
}

pub fn action_preview_copy(action: TokenBazaarAction) -> String {
    match action {
        TokenBazaarAction::Collect { bundle } => {
            format!("Collect {}", collect_bundle_label(bundle))
        }
        TokenBazaarAction::Exchange { pay, take } => {
            format!("Pay 2 {} to take 1 {}", pay.as_str(), take.as_str())
        }
        TokenBazaarAction::Fulfill { slot } => format!("Fulfill the contract in {}", slot.as_str()),
        TokenBazaarAction::Pass => "Pass because no other action is legal".to_owned(),
    }
}

pub fn action_family_label(family: ActionFamily) -> &'static str {
    match family {
        ActionFamily::Collect => "Collect",
        ActionFamily::Exchange => "Exchange",
        ActionFamily::Fulfill => "Fulfill",
        ActionFamily::Pass => "Pass",
    }
}

fn collect_bundle_label(bundle: CollectBundleId) -> &'static str {
    match bundle {
        CollectBundleId::Amber => "2 amber",
        CollectBundleId::Jade => "2 jade",
        CollectBundleId::Iron => "2 iron",
        CollectBundleId::AmberJade => "1 amber and 1 jade",
        CollectBundleId::JadeIron => "1 jade and 1 iron",
        CollectBundleId::IronAmber => "1 iron and 1 amber",
    }
}

fn resource_cost_label(cost: crate::ResourceCounts) -> String {
    ResourceId::ALL
        .iter()
        .filter_map(|resource| {
            let amount = cost.get(*resource);
            (amount > 0).then(|| format!("{amount} {}", resource.as_str()))
        })
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_metadata_has_labels_without_debug_or_candidate_data() {
        let metadata = ui_metadata();
        assert_eq!(metadata.table_label, "Token Bazaar market");
        assert_eq!(
            contract_accessibility_label(ContractId::BalancedWares),
            "Balanced Wares contract, worth 3 points, costs 1 amber, 1 jade, 1 iron"
        );
        assert_eq!(
            action_preview_copy(TokenBazaarAction::Exchange {
                pay: ResourceId::Amber,
                take: ResourceId::Iron
            }),
            "Pay 2 amber to take 1 iron"
        );
        let combined = format!("{metadata:?}");
        assert!(!combined.contains("debug"));
        assert!(!combined.contains("candidate"));
        assert!(!combined.contains("valuation"));
        assert!(!combined.contains("internal"));
    }
}
