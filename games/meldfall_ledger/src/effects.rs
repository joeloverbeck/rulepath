//! Viewer-safe semantic effects for Meldfall Ledger.
//!
//! Later tickets attach these payloads to full legality, redaction, and export
//! flows. The group names and stable strings are intentionally established here.

use engine_core::{EffectEnvelope, SeatId};

use crate::{
    cards::CardId,
    state::{MatchOutcome, MeldId, SeatIndex, TableCard},
};

pub type MeldfallEffectEnvelope = EffectEnvelope<MeldfallEffect>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MeldfallEffect {
    Draw {
        seat: SeatIndex,
        source: DrawSource,
        cards_moved: usize,
        stock_count_after: usize,
        discard_count_after: usize,
    },
    StockDrawPrivate {
        seat: SeatIndex,
        card: CardId,
        stock_count_after: usize,
    },
    Meld {
        seat: SeatIndex,
        meld_id: MeldId,
        cards: Vec<TableCard>,
    },
    LayOff {
        seat: SeatIndex,
        meld_id: MeldId,
        card: TableCard,
        position: LayoffEffectPosition,
    },
    Discard {
        seat: SeatIndex,
        card: CardId,
        discard_count_after: usize,
    },
    RoundScore {
        round_index: u32,
        deltas: Vec<i32>,
        cumulative_scores: Vec<i32>,
    },
    MatchTerminal {
        outcome: MatchOutcome,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DrawSource {
    Stock,
    Discard { selected_index: usize },
}

impl DrawSource {
    pub fn stable_string(self) -> String {
        match self {
            Self::Stock => "stock".to_owned(),
            Self::Discard { selected_index } => format!("discard:{selected_index}"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum LayoffEffectPosition {
    Prepend,
    Append,
}

impl LayoffEffectPosition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Prepend => "prepend",
            Self::Append => "append",
        }
    }
}

pub fn public_effect(payload: MeldfallEffect) -> MeldfallEffectEnvelope {
    EffectEnvelope::public(payload)
}

pub fn private_stock_draw_effect(
    owner: SeatId,
    seat: SeatIndex,
    card: CardId,
    stock_count_after: usize,
) -> MeldfallEffectEnvelope {
    EffectEnvelope::private_to(
        owner,
        MeldfallEffect::StockDrawPrivate {
            seat,
            card,
            stock_count_after,
        },
    )
}

pub fn effect_stable_string(effect: &MeldfallEffectEnvelope) -> String {
    match &effect.payload {
        MeldfallEffect::Draw {
            seat,
            source,
            cards_moved,
            stock_count_after,
            discard_count_after,
        } => format!(
            "Draw:seat={seat}:source={}:cards={cards_moved}:stock_after={stock_count_after}:discard_after={discard_count_after}",
            source.stable_string()
        ),
        MeldfallEffect::StockDrawPrivate {
            seat,
            card,
            stock_count_after,
        } => format!(
            "StockDrawPrivate:seat={seat}:card={}:stock_after={stock_count_after}",
            card.as_str()
        ),
        MeldfallEffect::Meld {
            seat,
            meld_id,
            cards,
        } => format!(
            "Meld:seat={seat}:meld={}:cards=[{}]",
            meld_id.as_string(),
            table_cards(cards)
        ),
        MeldfallEffect::LayOff {
            seat,
            meld_id,
            card,
            position,
        } => format!(
            "LayOff:seat={seat}:meld={}:card={}:position={}",
            meld_id.as_string(),
            card.stable_string(),
            position.as_str()
        ),
        MeldfallEffect::Discard {
            seat,
            card,
            discard_count_after,
        } => format!(
            "Discard:seat={seat}:card={}:discard_after={discard_count_after}",
            card.as_str()
        ),
        MeldfallEffect::RoundScore {
            round_index,
            deltas,
            cumulative_scores,
        } => format!(
            "RoundScore:round={round_index}:deltas=[{}]:cumulative=[{}]",
            int_list(deltas),
            int_list(cumulative_scores)
        ),
        MeldfallEffect::MatchTerminal { outcome } => {
            format!("MatchTerminal:{}", outcome.stable_string())
        }
    }
}

fn table_cards(cards: &[TableCard]) -> String {
    cards
        .iter()
        .map(TableCard::stable_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn int_list(values: &[i32]) -> String {
    values
        .iter()
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join(",")
}
