use engine_core::{ActionChoice, ActionMetadata, ActionPreview, ActionTree, Actor};

use crate::{
    ids::{CollectBundleId, ResourceId, TokenBazaarSeat, TokenBazaarSlot},
    rules::legal_actions,
    state::{contract_spec, ResourceCounts, TokenBazaarState},
};

pub const COLLECT_SEGMENT_PREFIX: &str = "collect/";
pub const EXCHANGE_SEGMENT_PREFIX: &str = "exchange/";
pub const FULFILL_SEGMENT_PREFIX: &str = "fulfill/";
pub const PASS_SEGMENT: &str = "pass";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ActionFamily {
    Collect,
    Exchange,
    Fulfill,
    Pass,
}

impl ActionFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Collect => "collect",
            Self::Exchange => "exchange",
            Self::Fulfill => "fulfill",
            Self::Pass => "pass",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TokenBazaarAction {
    Collect { bundle: CollectBundleId },
    Exchange { pay: ResourceId, take: ResourceId },
    Fulfill { slot: TokenBazaarSlot },
    Pass,
}

impl TokenBazaarAction {
    pub const fn family(self) -> ActionFamily {
        match self {
            Self::Collect { .. } => ActionFamily::Collect,
            Self::Exchange { .. } => ActionFamily::Exchange,
            Self::Fulfill { .. } => ActionFamily::Fulfill,
            Self::Pass => ActionFamily::Pass,
        }
    }

    pub fn segment(self) -> String {
        match self {
            Self::Collect { bundle } => format!("{COLLECT_SEGMENT_PREFIX}{}", bundle.as_str()),
            Self::Exchange { pay, take } => {
                format!(
                    "{EXCHANGE_SEGMENT_PREFIX}{}/{}",
                    pay.as_str(),
                    take.as_str()
                )
            }
            Self::Fulfill { slot } => format!("{FULFILL_SEGMENT_PREFIX}{}", slot.as_str()),
            Self::Pass => PASS_SEGMENT.to_owned(),
        }
    }
}

pub fn legal_action_tree(state: &TokenBazaarState, actor: &Actor) -> ActionTree {
    if state.terminal_outcome.is_some() || actor_seat(state, actor) != Some(state.active_seat) {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    ActionTree::flat(
        state.freshness_token,
        legal_actions(state)
            .into_iter()
            .map(|action| action_choice(state, action))
            .collect(),
    )
}

pub fn actor_seat(state: &TokenBazaarState, actor: &Actor) -> Option<TokenBazaarSeat> {
    state
        .seats
        .iter()
        .position(|seat_id| seat_id == &actor.seat_id)
        .and_then(TokenBazaarSeat::from_index)
}

pub fn parse_action_segment(segment: &str) -> Option<TokenBazaarAction> {
    if segment == PASS_SEGMENT {
        return Some(TokenBazaarAction::Pass);
    }

    if let Some(bundle) = segment.strip_prefix(COLLECT_SEGMENT_PREFIX) {
        return Some(TokenBazaarAction::Collect {
            bundle: CollectBundleId::parse(bundle)?,
        });
    }

    if let Some(exchange) = segment.strip_prefix(EXCHANGE_SEGMENT_PREFIX) {
        let mut parts = exchange.split('/');
        let pay = ResourceId::parse(parts.next()?)?;
        let take = ResourceId::parse(parts.next()?)?;
        if parts.next().is_some() {
            return None;
        }
        return Some(TokenBazaarAction::Exchange { pay, take });
    }

    if let Some(slot) = segment.strip_prefix(FULFILL_SEGMENT_PREFIX) {
        return Some(TokenBazaarAction::Fulfill {
            slot: TokenBazaarSlot::parse(slot)?,
        });
    }

    None
}

fn action_choice(state: &TokenBazaarState, action: TokenBazaarAction) -> ActionChoice {
    match action {
        TokenBazaarAction::Collect { bundle } => collect_choice(bundle),
        TokenBazaarAction::Exchange { pay, take } => exchange_choice(pay, take),
        TokenBazaarAction::Fulfill { slot } => fulfill_choice(state, slot),
        TokenBazaarAction::Pass => pass_choice(),
    }
}

fn collect_choice(bundle: CollectBundleId) -> ActionChoice {
    let gain = collect_gain(bundle);
    let mut choice = ActionChoice::leaf(
        TokenBazaarAction::Collect { bundle }.segment(),
        format!("Collect {}", bundle.as_str()),
        format!("Collect {}", resource_counts_label(gain)),
    );
    choice.metadata = vec![
        metadata("family", ActionFamily::Collect.as_str()),
        metadata("gain", resource_counts_stable(gain)),
        metadata("bundle_id", bundle.as_str()),
    ];
    choice.tags = vec!["economy".to_owned(), "collect".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}

fn exchange_choice(pay: ResourceId, take: ResourceId) -> ActionChoice {
    let mut cost = ResourceCounts::default();
    cost.set(pay, 2);
    let mut gain = ResourceCounts::default();
    gain.set(take, 1);
    let mut choice = ActionChoice::leaf(
        TokenBazaarAction::Exchange { pay, take }.segment(),
        format!("Exchange {} for {}", pay.as_str(), take.as_str()),
        format!("Pay 2 {} to take 1 {}", pay.as_str(), take.as_str()),
    );
    choice.metadata = vec![
        metadata("family", ActionFamily::Exchange.as_str()),
        metadata("cost", resource_counts_stable(cost)),
        metadata("gain", resource_counts_stable(gain)),
        metadata("pay_resource", pay.as_str()),
        metadata("take_resource", take.as_str()),
    ];
    choice.tags = vec!["economy".to_owned(), "exchange".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}

fn fulfill_choice(state: &TokenBazaarState, slot: TokenBazaarSlot) -> ActionChoice {
    let contract_id = state
        .slot_contract(slot)
        .expect("fulfill action must target occupied slot");
    let contract = contract_spec(contract_id);
    let mut choice = ActionChoice::leaf(
        TokenBazaarAction::Fulfill { slot }.segment(),
        format!("Fulfill {}", contract.label),
        format!(
            "Fulfill {} for {} points by paying {}",
            contract.label,
            contract.points,
            resource_counts_label(contract.cost)
        ),
    );
    choice.metadata = vec![
        metadata("family", ActionFamily::Fulfill.as_str()),
        metadata("cost", resource_counts_stable(contract.cost)),
        metadata("slot_id", slot.as_str()),
        metadata("contract_id", contract.id.as_str()),
        metadata("points", contract.points.to_string()),
    ];
    choice.tags = vec!["economy".to_owned(), "fulfill".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}

fn pass_choice() -> ActionChoice {
    let mut choice = ActionChoice::leaf(
        PASS_SEGMENT,
        "Pass",
        "Pass because no other action is legal",
    );
    choice.metadata = vec![metadata("family", ActionFamily::Pass.as_str())];
    choice.tags = vec!["forced".to_owned(), "pass".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}

pub fn collect_gain(bundle: CollectBundleId) -> ResourceCounts {
    match bundle {
        CollectBundleId::Amber => ResourceCounts::new(2, 0, 0),
        CollectBundleId::Jade => ResourceCounts::new(0, 2, 0),
        CollectBundleId::Iron => ResourceCounts::new(0, 0, 2),
        CollectBundleId::AmberJade => ResourceCounts::new(1, 1, 0),
        CollectBundleId::JadeIron => ResourceCounts::new(0, 1, 1),
        CollectBundleId::IronAmber => ResourceCounts::new(1, 0, 1),
    }
}

pub fn resource_counts_stable(counts: ResourceCounts) -> String {
    format!(
        "amber:{},jade:{},iron:{}",
        counts.amber, counts.jade, counts.iron
    )
}

fn resource_counts_label(counts: ResourceCounts) -> String {
    ResourceId::ALL
        .iter()
        .filter_map(|resource| {
            let amount = counts.get(*resource);
            (amount > 0).then(|| format!("{amount} {}", resource.as_str()))
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn metadata(key: impl Into<String>, value: impl Into<String>) -> ActionMetadata {
    ActionMetadata {
        key: key.into(),
        value: value.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup::setup_match;
    use engine_core::{SeatId, Seed};

    fn state() -> TokenBazaarState {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        setup_match(Seed(1), &seats, &Default::default()).expect("setup succeeds")
    }

    #[test]
    fn action_tree_is_stably_ordered_with_metadata() {
        let state = state();
        let tree = legal_action_tree(
            &state,
            &Actor {
                seat_id: state.seats[0].clone(),
            },
        );
        let segments = tree
            .root
            .choices
            .iter()
            .map(|choice| choice.segment.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            segments,
            vec![
                "collect/amber",
                "collect/jade",
                "collect/iron",
                "collect/amber-jade",
                "collect/jade-iron",
                "collect/iron-amber",
                "fulfill/slot_0",
            ]
        );
        let fulfill = tree
            .root
            .choices
            .iter()
            .find(|choice| choice.segment == "fulfill/slot_0")
            .expect("fulfill choice exists");
        assert_eq!(metadata_value(fulfill, "family"), Some("fulfill"));
        assert_eq!(
            metadata_value(fulfill, "contract_id"),
            Some("balanced-wares")
        );
        assert_eq!(metadata_value(fulfill, "points"), Some("3"));
        assert_eq!(
            metadata_value(fulfill, "cost"),
            Some("amber:1,jade:1,iron:1")
        );
        assert!(fulfill
            .metadata
            .iter()
            .all(|entry| entry.key != "debug" && entry.key != "valuation"));
    }

    #[test]
    fn action_tree_is_empty_for_non_active_actor() {
        let state = state();
        let tree = legal_action_tree(
            &state,
            &Actor {
                seat_id: state.seats[1].clone(),
            },
        );

        assert!(tree.root.choices.is_empty());
    }

    #[test]
    fn parse_action_segments() {
        assert_eq!(
            parse_action_segment("collect/amber-jade"),
            Some(TokenBazaarAction::Collect {
                bundle: CollectBundleId::AmberJade
            })
        );
        assert_eq!(
            parse_action_segment("exchange/amber/iron"),
            Some(TokenBazaarAction::Exchange {
                pay: ResourceId::Amber,
                take: ResourceId::Iron
            })
        );
        assert_eq!(
            parse_action_segment("fulfill/slot_2"),
            Some(TokenBazaarAction::Fulfill {
                slot: TokenBazaarSlot::Slot2
            })
        );
        assert_eq!(parse_action_segment("pass"), Some(TokenBazaarAction::Pass));
        assert_eq!(parse_action_segment("exchange/amber/iron/extra"), None);
    }

    fn metadata_value<'a>(choice: &'a ActionChoice, key: &str) -> Option<&'a str> {
        choice
            .metadata
            .iter()
            .find(|entry| entry.key == key)
            .map(|entry| entry.value.as_str())
    }
}
