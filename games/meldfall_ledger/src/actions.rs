//! Action-tree and command payloads for Meldfall Ledger.
//!
//! This module owns typed Five Hundred Rummy action names while emitting the
//! generic engine action-tree contract.

use engine_core::{
    ActionChoice, ActionMetadata, ActionNode, ActionPath, ActionPreview, ActionTree, FreshnessToken,
};

use crate::{cards::CardId, state::MeldId};

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
            "Meld new",
            "Create a new meld with selected cards",
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
            "Lay off",
            "Lay off a card onto an existing meld",
            vec![
                metadata("kind", "lay_off"),
                metadata("card", card.as_str()),
                metadata("target_meld", target_meld.as_string()),
                metadata("position", position.as_str()),
            ],
        ),
        MeldfallAction::Discard { card } => choice(
            MeldfallAction::Discard { card }.segment(),
            "Discard",
            "Discard one card",
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
