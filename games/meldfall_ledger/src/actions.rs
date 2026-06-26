//! Action-tree and command payloads for Meldfall Ledger.
//!
//! This module owns typed Five Hundred Rummy action names while emitting the
//! generic engine action-tree contract.

use engine_core::{
    ActionChoice, ActionMetadata, ActionNode, ActionPath, ActionPreview, ActionTree, FreshnessToken,
};

use crate::{
    cards::CardId,
    state::{MeldId, RoundState, TurnPhase},
};

pub const DRAW_STOCK_SEGMENT: &str = "draw-stock";
pub const DRAW_DISCARD_SEGMENT_PREFIX: &str = "draw-discard";
pub const MELD_NEW_SEGMENT_PREFIX: &str = "meld-new";
pub const LAY_OFF_SEGMENT_PREFIX: &str = "lay-off";
pub const DISCARD_SEGMENT_PREFIX: &str = "discard";
pub const GO_OUT_WITHOUT_DISCARD_SEGMENT: &str = "go-out-without-discard";
pub const FINISH_TURN_SEGMENT: &str = "finish-turn";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MeldfallAction {
    DrawFromStock,
    DrawFromDiscard {
        discard_index: usize,
    },
    MeldNew {
        cards: Vec<CardId>,
    },
    LayOff {
        card: CardId,
        target_meld: MeldId,
        position: LayoffPosition,
    },
    Discard {
        card: CardId,
    },
    GoOutWithoutDiscard,
    FinishTurn,
}

impl MeldfallAction {
    pub fn action_path(&self) -> ActionPath {
        ActionPath {
            segments: vec![self.segment()],
        }
    }

    pub fn segment(&self) -> String {
        match self {
            Self::DrawFromStock => DRAW_STOCK_SEGMENT.to_owned(),
            Self::DrawFromDiscard { discard_index } => {
                format!("{DRAW_DISCARD_SEGMENT_PREFIX}-{discard_index}")
            }
            Self::MeldNew { cards } => {
                format!("{MELD_NEW_SEGMENT_PREFIX}-{}", card_segment(cards))
            }
            Self::LayOff {
                card,
                target_meld,
                position,
            } => format!(
                "{LAY_OFF_SEGMENT_PREFIX}-{}-{}-{}",
                card.as_str(),
                target_meld.as_string(),
                position.as_str()
            ),
            Self::Discard { card } => format!("{DISCARD_SEGMENT_PREFIX}-{}", card.as_str()),
            Self::GoOutWithoutDiscard => GO_OUT_WITHOUT_DISCARD_SEGMENT.to_owned(),
            Self::FinishTurn => FINISH_TURN_SEGMENT.to_owned(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum LayoffPosition {
    Prepend,
    Append,
}

impl LayoffPosition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Prepend => "prepend",
            Self::Append => "append",
        }
    }
}

pub fn draw_action_tree(
    freshness_token: FreshnessToken,
    can_draw_stock: bool,
    discard_indices: &[usize],
) -> ActionTree {
    let mut choices = Vec::new();
    if can_draw_stock {
        choices.push(choice(
            DRAW_STOCK_SEGMENT,
            "Draw stock",
            "Draw one hidden stock card",
            vec![metadata("kind", "draw"), metadata("source", "stock")],
        ));
    }
    choices.extend(discard_indices.iter().copied().map(draw_discard_choice));
    ActionTree::flat(freshness_token, choices)
}

pub fn draw_source_action_tree(freshness_token: FreshnessToken, round: &RoundState) -> ActionTree {
    if round.phase != TurnPhase::Draw || round.pending_pickup.is_some() {
        return ActionTree::flat(freshness_token, Vec::new());
    }
    let discard_indices = (0..round.discard.len()).collect::<Vec<_>>();
    draw_action_tree(freshness_token, !round.stock.is_empty(), &discard_indices)
}

pub fn table_action_tree(
    freshness_token: FreshnessToken,
    melds: Vec<MeldfallAction>,
    lay_offs: Vec<MeldfallAction>,
    discards: Vec<MeldfallAction>,
    can_go_out_without_discard: bool,
) -> ActionTree {
    let mut table_choices = Vec::new();
    table_choices.extend(melds.into_iter().map(action_choice));
    table_choices.extend(lay_offs.into_iter().map(action_choice));
    table_choices.extend(discards.into_iter().map(action_choice));
    if can_go_out_without_discard {
        table_choices.push(action_choice(MeldfallAction::GoOutWithoutDiscard));
    }
    table_choices.push(action_choice(MeldfallAction::FinishTurn));
    ActionTree::flat(freshness_token, table_choices)
}

pub fn progressive_turn_tree(
    freshness_token: FreshnessToken,
    draw_choices: Vec<MeldfallAction>,
    after_draw_choices: Vec<MeldfallAction>,
) -> ActionTree {
    let after_draw = ActionNode {
        choices: after_draw_choices.into_iter().map(action_choice).collect(),
    };
    let choices = draw_choices
        .into_iter()
        .map(|action| {
            let mut choice = action_choice(action);
            choice.next = Some(Box::new(after_draw.clone()));
            choice
        })
        .collect();
    ActionTree::flat(freshness_token, choices)
}

pub fn action_choice(action: MeldfallAction) -> ActionChoice {
    match action {
        MeldfallAction::DrawFromStock => choice(
            DRAW_STOCK_SEGMENT,
            "Draw stock",
            "Draw one hidden stock card",
            vec![metadata("kind", "draw"), metadata("source", "stock")],
        ),
        MeldfallAction::DrawFromDiscard { discard_index } => draw_discard_choice(discard_index),
        MeldfallAction::MeldNew { cards } => choice(
            MeldfallAction::MeldNew {
                cards: cards.clone(),
            }
            .segment(),
            format!("Meld new {}", cards_short(&cards)),
            format!("Create a new meld with {}", cards_words(&cards)),
            vec![
                metadata("kind", "meld_new"),
                metadata("cards", card_segment(&cards)),
            ],
        ),
        MeldfallAction::LayOff {
            card,
            target_meld,
            position,
        } => choice(
            (MeldfallAction::LayOff {
                card,
                target_meld,
                position,
            })
            .segment(),
            format!("Lay off {}", card_short(card)),
            format!(
                "Lay off {} onto {} ({} side)",
                card_words(card),
                target_meld.as_string(),
                position.as_str()
            ),
            vec![
                metadata("kind", "lay_off"),
                metadata("card", card.as_str()),
                metadata("target_meld", target_meld.as_string()),
                metadata("position", position.as_str()),
            ],
        ),
        MeldfallAction::Discard { card } => choice(
            MeldfallAction::Discard { card }.segment(),
            format!("Discard {}", card_short(card)),
            format!("Discard {}", card_words(card)),
            vec![metadata("kind", "discard"), metadata("card", card.as_str())],
        ),
        MeldfallAction::GoOutWithoutDiscard => choice(
            GO_OUT_WITHOUT_DISCARD_SEGMENT,
            "Go out",
            "Go out without discarding",
            vec![metadata("kind", "go_out_without_discard")],
        ),
        MeldfallAction::FinishTurn => choice(
            FINISH_TURN_SEGMENT,
            "Finish turn",
            "Finish the current turn",
            vec![metadata("kind", "finish_turn")],
        ),
    }
}

fn draw_discard_choice(discard_index: usize) -> ActionChoice {
    choice(
        format!("{DRAW_DISCARD_SEGMENT_PREFIX}-{discard_index}"),
        format!("Draw discard {discard_index}"),
        format!("Draw from discard pile index {discard_index}"),
        vec![
            metadata("kind", "draw"),
            metadata("source", "discard"),
            metadata("discard_index", discard_index.to_string()),
        ],
    )
}

fn choice(
    segment: impl Into<String>,
    label: impl Into<String>,
    accessibility_label: impl Into<String>,
    metadata: Vec<ActionMetadata>,
) -> ActionChoice {
    let mut choice = ActionChoice::leaf(segment, label, accessibility_label);
    choice.metadata = metadata;
    choice.preview = ActionPreview::Available;
    choice
}

fn metadata(key: impl Into<String>, value: impl Into<String>) -> ActionMetadata {
    ActionMetadata {
        key: key.into(),
        value: value.into(),
    }
}

fn card_segment(cards: &[CardId]) -> String {
    cards
        .iter()
        .map(|card| card.as_str())
        .collect::<Vec<_>>()
        .join("_")
}

/// Compact human label for a single card, e.g. `10S`, `AH`.
fn card_short(card: CardId) -> String {
    card.card().public_label()
}

/// Spoken-friendly card name, e.g. `Ten of Spades`.
fn card_words(card: CardId) -> String {
    let card = card.card();
    format!(
        "{} of {}",
        capitalize(card.rank.as_str()),
        capitalize(card.suit.as_str())
    )
}

/// Space-joined compact labels for a card list, e.g. `4D 4C 4H`.
fn cards_short(cards: &[CardId]) -> String {
    cards
        .iter()
        .map(|card| card_short(*card))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Comma-joined spoken card names for a card list.
fn cards_words(cards: &[CardId]) -> String {
    cards
        .iter()
        .map(|card| card_words(*card))
        .collect::<Vec<_>>()
        .join(", ")
}

fn capitalize(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
