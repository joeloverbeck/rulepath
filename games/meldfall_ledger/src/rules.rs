//! Rust-owned legality and validation for Meldfall Ledger.
//!
//! Meld legality is the first local rummy primitive for this crate. Later
//! tickets fill draw, lay-off, discard, go-out, stock exhaustion, and
//! stale/wrong-seat diagnostics.

use std::collections::BTreeSet;

use engine_core::Diagnostic;

use crate::{
    actions::LayoffPosition,
    cards::{ranks_are_consecutive_low_or_high, CardId},
    effects::{public_effect, LayoffEffectPosition, MeldfallEffect, MeldfallEffectEnvelope},
    state::{MeldGroup, MeldId, MeldKind, RoundState, SeatIndex, TableCard, TurnOrdinal},
};

pub fn validate_new_meld(cards: &[CardId]) -> Result<MeldKind, Diagnostic> {
    if cards.len() < 3 {
        return Err(meld_diagnostic(
            "ML_MELD_TOO_SMALL",
            "meldfall_ledger melds require at least three cards",
        ));
    }
    if has_duplicate_cards(cards) {
        return Err(meld_diagnostic(
            "ML_MELD_DUPLICATE_CARD",
            "meldfall_ledger melds cannot reuse the same card",
        ));
    }

    if let Some(kind) = validate_set(cards) {
        return Ok(kind);
    }
    if let Some(kind) = validate_run(cards) {
        return Ok(kind);
    }

    Err(meld_diagnostic(
        "ML_INVALID_MELD_SHAPE",
        "meldfall_ledger melds must be a same-rank set or same-suit consecutive run",
    ))
}

pub fn validate_owned_meld(hand: &[CardId], cards: &[CardId]) -> Result<MeldKind, Diagnostic> {
    if has_duplicate_cards(cards) {
        return Err(meld_diagnostic(
            "ML_MELD_DUPLICATE_CARD",
            "meldfall_ledger melds cannot reuse the same card",
        ));
    }
    for card in cards {
        if !hand.contains(card) {
            return Err(meld_diagnostic(
                "ML_MELD_CARD_NOT_OWNED",
                format!(
                    "meldfall_ledger cannot meld card {} because it is not in the active hand",
                    card.as_str()
                ),
            ));
        }
    }
    validate_new_meld(cards)
}

pub fn take_new_meld_from_hand(
    hand: &mut Vec<CardId>,
    cards: &[CardId],
    meld_id: MeldId,
    origin_seat: SeatIndex,
    play_turn: TurnOrdinal,
) -> Result<MeldGroup, Diagnostic> {
    let kind = validate_owned_meld(hand, cards)?;
    let mut remaining = hand.clone();
    for card in cards {
        let index = remaining
            .iter()
            .position(|held| held == card)
            .expect("validated ownership ensures selected card exists");
        remaining.remove(index);
    }
    *hand = remaining;
    Ok(MeldGroup {
        id: meld_id,
        kind,
        origin_seat,
        cards: cards
            .iter()
            .copied()
            .map(|card| TableCard {
                card,
                played_by: origin_seat,
                score_credit_owner: origin_seat,
                play_turn,
            })
            .collect(),
    })
}

pub fn table_new_meld(
    round: &mut RoundState,
    seat_index: SeatIndex,
    cards: &[CardId],
    play_turn: TurnOrdinal,
) -> Result<MeldGroup, Diagnostic> {
    if seat_index >= round.seats.len() {
        return Err(meld_diagnostic(
            "ML_INVALID_SEAT_INDEX",
            format!(
                "meldfall_ledger cannot table meld for seat index {seat_index}; only {} seats exist",
                round.seats.len()
            ),
        ));
    }

    let meld_id = round.tableau.next_meld_id();
    let group = take_new_meld_from_hand(
        &mut round.seats[seat_index].hand,
        cards,
        meld_id,
        seat_index,
        play_turn,
    )?;
    round.round_played_scores[seat_index] += tabled_score_for(&group);
    round.tableau.groups.push(group.clone());
    Ok(group)
}

pub fn lay_off_card(
    round: &mut RoundState,
    seat_index: SeatIndex,
    card: CardId,
    target_meld: MeldId,
    position: LayoffPosition,
    play_turn: TurnOrdinal,
) -> Result<MeldfallEffectEnvelope, Diagnostic> {
    if seat_index >= round.seats.len() {
        return Err(meld_diagnostic(
            "ML_INVALID_SEAT_INDEX",
            format!(
                "meldfall_ledger cannot lay off for seat index {seat_index}; only {} seats exist",
                round.seats.len()
            ),
        ));
    }
    if !round.seats[seat_index].hand.contains(&card) {
        return Err(meld_diagnostic(
            "ML_LAYOFF_CARD_NOT_OWNED",
            "meldfall_ledger cannot lay off a card outside the active hand",
        ));
    }

    let group_index = round
        .tableau
        .groups
        .iter()
        .position(|group| group.id == target_meld)
        .ok_or_else(|| {
            meld_diagnostic(
                "ML_UNKNOWN_MELD",
                format!(
                    "meldfall_ledger cannot lay off onto unknown meld {}",
                    target_meld.as_string()
                ),
            )
        })?;
    validate_lay_off_candidate(&round.tableau.groups[group_index], card, position)?;

    let hand_index = round.seats[seat_index]
        .hand
        .iter()
        .position(|held| *held == card)
        .expect("owned card was checked before mutation");
    round.seats[seat_index].hand.remove(hand_index);

    let table_card = TableCard {
        card,
        played_by: seat_index,
        score_credit_owner: seat_index,
        play_turn,
    };
    match position {
        LayoffPosition::Prepend => round.tableau.groups[group_index]
            .cards
            .insert(0, table_card.clone()),
        LayoffPosition::Append => round.tableau.groups[group_index]
            .cards
            .push(table_card.clone()),
    }
    round.round_played_scores[seat_index] += i32::from(card.card().rank.score_value());

    Ok(public_effect(MeldfallEffect::LayOff {
        seat: seat_index,
        meld_id: target_meld,
        card: table_card,
        position: layoff_effect_position(position),
    }))
}

fn validate_set(cards: &[CardId]) -> Option<MeldKind> {
    if cards.len() > 4 {
        return None;
    }
    let rank = cards.first()?.card().rank;
    if cards.iter().all(|card| card.card().rank == rank) {
        let distinct_suits = cards
            .iter()
            .map(|card| card.card().suit)
            .collect::<BTreeSet<_>>();
        if distinct_suits.len() == cards.len() {
            return Some(MeldKind::Set { rank });
        }
    }
    None
}

fn validate_run(cards: &[CardId]) -> Option<MeldKind> {
    let suit = cards.first()?.card().suit;
    if !cards.iter().all(|card| card.card().suit == suit) {
        return None;
    }
    let ranks = cards
        .iter()
        .map(|card| card.card().rank)
        .collect::<Vec<_>>();
    if ranks_are_consecutive_low_or_high(&ranks) {
        Some(MeldKind::Run { suit })
    } else {
        None
    }
}

fn validate_lay_off_candidate(
    group: &MeldGroup,
    card: CardId,
    position: LayoffPosition,
) -> Result<(), Diagnostic> {
    if group.cards.iter().any(|table_card| table_card.card == card) {
        return Err(invalid_layoff(group.id));
    }

    let mut candidate = group
        .cards
        .iter()
        .map(|table_card| table_card.card)
        .collect::<Vec<_>>();
    match position {
        LayoffPosition::Prepend => candidate.insert(0, card),
        LayoffPosition::Append => candidate.push(card),
    }
    let candidate_kind = validate_new_meld(&candidate).map_err(|_| invalid_layoff(group.id))?;
    if !same_meld_kind(&group.kind, &candidate_kind) {
        return Err(invalid_layoff(group.id));
    }
    if matches!(group.kind, MeldKind::Run { .. }) && !ordered_run_is_consecutive(&candidate) {
        return Err(invalid_layoff(group.id));
    }
    Ok(())
}

fn same_meld_kind(left: &MeldKind, right: &MeldKind) -> bool {
    match (left, right) {
        (MeldKind::Set { rank: left }, MeldKind::Set { rank: right }) => left == right,
        (MeldKind::Run { suit: left }, MeldKind::Run { suit: right }) => left == right,
        (MeldKind::Unknown, MeldKind::Unknown) => true,
        _ => false,
    }
}

fn ordered_run_is_consecutive(cards: &[CardId]) -> bool {
    ordered_run_is_consecutive_by(cards, |rank| rank.low_run_value())
        || ordered_run_is_consecutive_by(cards, |rank| rank.high_run_value())
}

fn ordered_run_is_consecutive_by(
    cards: &[CardId],
    value: impl Fn(crate::cards::Rank) -> u8,
) -> bool {
    cards
        .windows(2)
        .all(|pair| value(pair[1].card().rank) == value(pair[0].card().rank) + 1)
}

fn has_duplicate_cards(cards: &[CardId]) -> bool {
    cards.iter().copied().collect::<BTreeSet<_>>().len() != cards.len()
}

fn tabled_score_for(group: &MeldGroup) -> i32 {
    group
        .cards
        .iter()
        .map(|card| i32::from(card.card.card().rank.score_value()))
        .sum()
}

fn meld_diagnostic(code: &str, message: impl Into<String>) -> Diagnostic {
    Diagnostic {
        code: code.to_owned(),
        message: message.into(),
    }
}

fn invalid_layoff(target_meld: MeldId) -> Diagnostic {
    meld_diagnostic(
        "ML_INVALID_LAYOFF",
        format!(
            "meldfall_ledger selected card cannot extend target meld {}",
            target_meld.as_string()
        ),
    )
}

const fn layoff_effect_position(position: LayoffPosition) -> LayoffEffectPosition {
    match position {
        LayoffPosition::Prepend => LayoffEffectPosition::Prepend,
        LayoffPosition::Append => LayoffEffectPosition::Append,
    }
}
