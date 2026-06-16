use engine_core::{FreshnessToken, HashValue, StableSerialize, Viewer};

use crate::{
    bots::{BotDecisionPublicExplanation, BotDecisionPublicFact},
    cards::Card,
    ids::{RiverLedgerSeat, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID},
    state::{CategoryLadderPosition, Phase, RiverLedgerState, SeatStatus, TerminalOutcome},
    ui::{
        seat_ledger_display, ui_metadata, HoleCardSummary, RiverLedgerSeatLedgerDisplay,
        SeatLedgerRoles, UiMetadata,
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicView {
    pub schema_version: u32,
    pub rules_version: u32,
    pub game_id: String,
    pub display_name: String,
    pub variant_id: String,
    pub rules_version_label: String,
    pub phase: Phase,
    pub active_seat: Option<RiverLedgerSeat>,
    pub button: RiverLedgerSeat,
    pub small_blind: RiverLedgerSeat,
    pub big_blind: RiverLedgerSeat,
    pub pot_total: u16,
    pub seats: Vec<SeatView>,
    pub board: Vec<CardView>,
    pub board_slots: Vec<BoardSlotView>,
    pub reserved_community_count: u8,
    pub deck_tail_count: u8,
    pub terminal: TerminalView,
    pub terminal_rationale: Option<OutcomeRationaleView>,
    pub freshness_token: FreshnessToken,
    pub private_view: PrivateView,
    pub ui: UiMetadata,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatView {
    pub seat: RiverLedgerSeat,
    pub status: SeatStatus,
    pub street_contribution: u16,
    pub total_contribution: u16,
    pub hidden_hole_count: u8,
    pub ledger_display: RiverLedgerSeatLedgerDisplay,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardView {
    pub card_id: String,
    pub rank: String,
    pub rank_value: u8,
    pub suit: String,
    pub label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoardSlotView {
    pub slot: String,
    pub reveal_state: String,
    pub street_label: String,
    pub visual_placeholder_label: String,
    pub accessibility_label: String,
    pub card: Option<CardView>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrivateView {
    Observer,
    Seat(Box<SeatPrivateView>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatPrivateView {
    pub seat: RiverLedgerSeat,
    pub hole_cards: [CardView; 2],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalView {
    NonTerminal,
    LastLiveHand {
        winner: RiverLedgerSeat,
        pot_total: u16,
    },
    Showdown {
        winners: Vec<RiverLedgerSeat>,
        pot_total: u16,
        allocations: Vec<(RiverLedgerSeat, u16)>,
        explanations: Vec<String>,
        presentation_v2: Box<ShowdownPresentationV2View>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownPresentationV2View {
    pub result_banner: ShowdownResultBannerView,
    pub decisive_reason: ShowdownDecisiveReasonView,
    pub board_cards: Vec<ShowdownBoardCardPresentationView>,
    pub standings: Vec<ShowdownStandingPresentationView>,
    pub folded_rows: Vec<ShowdownFoldedRowPresentationView>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownResultBannerView {
    pub headline: String,
    pub subheadline: String,
    pub accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownDecisiveReasonView {
    pub short_text: String,
    pub contrast_seat: Option<RiverLedgerSeat>,
    pub contrast_seat_label: Option<String>,
    pub rule_refs: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownBoardCardPresentationView {
    pub slot: String,
    pub card: CardView,
    pub public_label: String,
    pub used_by_selected: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownStandingPresentationView {
    pub seat: RiverLedgerSeat,
    pub seat_label: String,
    pub rank: u8,
    pub result_label: String,
    pub allocation_label: String,
    pub hand_name: String,
    pub short_comparison_note: String,
    pub rank_ladder_label: String,
    pub hole_cards: Vec<ShowdownCardUsageMarkView>,
    pub board_cards: Vec<ShowdownCardUsageMarkView>,
    pub best_five: Vec<CardView>,
    pub best_five_accessibility_label: String,
    pub detail_rows: Vec<ShowdownDetailRowView>,
    pub default_expanded: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownCardUsageMarkView {
    pub card: CardView,
    pub public_label: String,
    pub used_in_best_five: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownDetailRowView {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownFoldedRowPresentationView {
    pub seat: RiverLedgerSeat,
    pub seat_label: String,
    pub redaction_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutcomeRationaleView {
    pub result_kind: String,
    pub decisive_cause: String,
    pub template_key: String,
    pub headline: Option<String>,
    pub decisive_comparison: Option<String>,
    pub comparison_basis: Option<String>,
    pub decisive_rule_ids: Vec<String>,
    pub per_seat: Vec<SeatOutcomeBreakdownView>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatOutcomeBreakdownView {
    pub seat: RiverLedgerSeat,
    pub result: String,
    pub allocation: u16,
    pub contribution: u16,
    pub strength: Option<ShowdownStrengthView>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownStrengthView {
    pub category: String,
    pub tie_break_vector: Vec<u8>,
    pub best_five: Vec<CardView>,
    pub category_ladder_position: CategoryLadderPosition,
    pub result_label: String,
    pub hand_name: String,
    pub rank_explanation: String,
    pub comparison_note: String,
    pub best_five_accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotDecisionPublicExplanationView {
    pub seat: RiverLedgerSeat,
    pub seat_label: String,
    pub action_label: String,
    pub short_reason: String,
    pub public_facts: Vec<BotDecisionPublicFactView>,
    pub hidden_information_notice: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotDecisionPublicFactView {
    pub label: String,
    pub value: String,
}

pub fn project_bot_decision_public_explanation(
    explanation: &BotDecisionPublicExplanation,
) -> BotDecisionPublicExplanationView {
    BotDecisionPublicExplanationView {
        seat: explanation.seat,
        seat_label: explanation.seat_label.clone(),
        action_label: explanation.action_label.clone(),
        short_reason: explanation.short_reason.clone(),
        public_facts: explanation
            .public_facts
            .iter()
            .map(project_public_fact)
            .collect(),
        hidden_information_notice: explanation.hidden_information_notice.clone(),
    }
}

fn project_public_fact(fact: &BotDecisionPublicFact) -> BotDecisionPublicFactView {
    BotDecisionPublicFactView {
        label: fact.label.clone(),
        value: fact.value.clone(),
    }
}

pub fn project_view(state: &RiverLedgerState, viewer: &Viewer) -> PublicView {
    let viewer_seat = viewer_seat(state, viewer);
    PublicView {
        schema_version: 1,
        rules_version: 1,
        game_id: GAME_ID.to_owned(),
        display_name: "River Ledger".to_owned(),
        variant_id: VARIANT_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        phase: state.phase,
        active_seat: state.active_seat,
        button: state.button,
        small_blind: state.small_blind,
        big_blind: state.big_blind,
        pot_total: state.ledger.pot_total,
        seats: state
            .ledger
            .seats
            .iter()
            .map(|entry| {
                let hidden_hole_count = 2;
                SeatView {
                    seat: entry.seat,
                    status: entry.status,
                    street_contribution: entry.street_contribution,
                    total_contribution: entry.total_contribution,
                    hidden_hole_count,
                    ledger_display: seat_ledger_display(
                        entry.seat,
                        entry.status,
                        entry.street_contribution,
                        entry.total_contribution,
                        hole_card_summary(state, entry.seat, hidden_hole_count),
                        SeatLedgerRoles {
                            active: state.active_seat == Some(entry.seat),
                            button: state.button == entry.seat,
                            small_blind: state.small_blind == entry.seat,
                            big_blind: state.big_blind == entry.seat,
                        },
                    ),
                }
            })
            .collect(),
        board: state.board.iter().copied().map(card_view).collect(),
        board_slots: board_slots(state),
        reserved_community_count: state.community_deck_internal().len() as u8,
        deck_tail_count: state.deck_tail_internal().len() as u8,
        terminal: terminal_view(state.terminal_outcome.as_ref()),
        terminal_rationale: outcome_rationale(state),
        freshness_token: state.freshness_token,
        private_view: private_view(state, viewer_seat),
        ui: ui_metadata(),
    }
}

pub fn view_hash(view: &PublicView) -> HashValue {
    view.stable_hash()
}

impl PublicView {
    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};rules={};game={};variant={};label={};phase={};active={};button={};sb={};bb={};pot={};seats={};board={};reserved={};tail={};terminal={};rationale={};freshness={};private={};ui={}",
            self.schema_version,
            self.rules_version,
            self.game_id,
            self.variant_id,
            self.rules_version_label,
            phase(self.phase),
            seat_option(self.active_seat),
            self.button.as_str(),
            self.small_blind.as_str(),
            self.big_blind.as_str(),
            self.pot_total,
            encode_seats(&self.seats),
            encode_board_slots(&self.board_slots),
            self.reserved_community_count,
            self.deck_tail_count,
            encode_terminal(&self.terminal),
            encode_rationale(self.terminal_rationale.as_ref()),
            self.freshness_token.0,
            encode_private(&self.private_view),
            self.ui.display_name,
        )
    }
}

impl StableSerialize for PublicView {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

fn viewer_seat(state: &RiverLedgerState, viewer: &Viewer) -> Option<RiverLedgerSeat> {
    viewer
        .seat_id
        .as_ref()
        .and_then(|seat_id| {
            state
                .seats
                .iter()
                .position(|candidate| candidate == seat_id)
        })
        .and_then(RiverLedgerSeat::from_index)
}

fn private_view(state: &RiverLedgerState, viewer_seat: Option<RiverLedgerSeat>) -> PrivateView {
    match viewer_seat {
        Some(seat) => {
            let hand = state
                .private_hand_for_internal(seat)
                .expect("viewer seat has a private hand");
            PrivateView::Seat(Box::new(SeatPrivateView {
                seat,
                hole_cards: [card_view(hand[0]), card_view(hand[1])],
            }))
        }
        None => PrivateView::Observer,
    }
}

fn hole_card_summary(
    state: &RiverLedgerState,
    seat: RiverLedgerSeat,
    hidden_hole_count: u8,
) -> HoleCardSummary {
    if let Some(TerminalOutcome::Showdown { explanations, .. }) = state.terminal_outcome.as_ref() {
        if explanations
            .iter()
            .any(|explanation| explanation.seat == seat && explanation.revealed.is_some())
        {
            return HoleCardSummary::Revealed(2);
        }
    }

    HoleCardSummary::Hidden(hidden_hole_count)
}

fn terminal_view(outcome: Option<&TerminalOutcome>) -> TerminalView {
    match outcome {
        None => TerminalView::NonTerminal,
        Some(TerminalOutcome::LastLiveHand { winner, pot_total }) => TerminalView::LastLiveHand {
            winner: *winner,
            pot_total: *pot_total,
        },
        Some(TerminalOutcome::Showdown {
            winners,
            pot_total,
            allocations,
            explanations,
            presentation_v2,
            ..
        }) => TerminalView::Showdown {
            winners: winners.clone(),
            pot_total: *pot_total,
            allocations: allocations
                .iter()
                .map(|share| (share.seat, share.amount))
                .collect(),
            explanations: explanations
                .iter()
                .map(|explanation| explanation.summary.clone())
                .collect(),
            presentation_v2: Box::new(showdown_presentation_v2_view(presentation_v2)),
        },
    }
}

fn showdown_presentation_v2_view(
    presentation: &crate::state::RiverLedgerShowdownPresentationV2,
) -> ShowdownPresentationV2View {
    ShowdownPresentationV2View {
        result_banner: ShowdownResultBannerView {
            headline: presentation.result_banner.headline.clone(),
            subheadline: presentation.result_banner.subheadline.clone(),
            accessibility_label: presentation.result_banner.accessibility_label.clone(),
        },
        decisive_reason: ShowdownDecisiveReasonView {
            short_text: presentation.decisive_reason.short_text.clone(),
            contrast_seat: presentation.decisive_reason.contrast_seat,
            contrast_seat_label: presentation.decisive_reason.contrast_seat_label.clone(),
            rule_refs: presentation.decisive_reason.rule_refs.clone(),
        },
        board_cards: presentation
            .board_cards
            .iter()
            .map(|entry| ShowdownBoardCardPresentationView {
                slot: entry.slot.clone(),
                card: card_view(entry.card),
                public_label: entry.public_label.clone(),
                used_by_selected: entry.used_by_selected.clone(),
            })
            .collect(),
        standings: presentation
            .standings
            .iter()
            .map(|entry| ShowdownStandingPresentationView {
                seat: entry.seat,
                seat_label: entry.seat_label.clone(),
                rank: entry.rank,
                result_label: entry.result_label.clone(),
                allocation_label: entry.allocation_label.clone(),
                hand_name: entry.hand_name.clone(),
                short_comparison_note: entry.short_comparison_note.clone(),
                rank_ladder_label: entry.rank_ladder_label.clone(),
                hole_cards: entry.hole_cards.iter().map(card_usage_mark_view).collect(),
                board_cards: entry.board_cards.iter().map(card_usage_mark_view).collect(),
                best_five: entry.best_five.iter().copied().map(card_view).collect(),
                best_five_accessibility_label: entry.best_five_accessibility_label.clone(),
                detail_rows: entry
                    .detail_rows
                    .iter()
                    .map(|row| ShowdownDetailRowView {
                        label: row.label.clone(),
                        value: row.value.clone(),
                    })
                    .collect(),
                default_expanded: entry.default_expanded,
            })
            .collect(),
        folded_rows: presentation
            .folded_rows
            .iter()
            .map(|entry| ShowdownFoldedRowPresentationView {
                seat: entry.seat,
                seat_label: entry.seat_label.clone(),
                redaction_label: entry.redaction_label.clone(),
            })
            .collect(),
    }
}

fn card_usage_mark_view(mark: &crate::state::ShowdownCardUsageMark) -> ShowdownCardUsageMarkView {
    ShowdownCardUsageMarkView {
        card: card_view(mark.card),
        public_label: mark.public_label.clone(),
        used_in_best_five: mark.used_in_best_five,
    }
}

fn outcome_rationale(state: &RiverLedgerState) -> Option<OutcomeRationaleView> {
    match state.terminal_outcome.as_ref()? {
        TerminalOutcome::LastLiveHand { winner, pot_total } => Some(OutcomeRationaleView {
            result_kind: "last_live_hand".to_owned(),
            decisive_cause: "last_live_after_folds".to_owned(),
            template_key: "river_ledger.last_live_fold_win".to_owned(),
            headline: None,
            decisive_comparison: None,
            comparison_basis: None,
            decisive_rule_ids: rule_ids(&["RL-END-LAST-LIVE", "RL-SCORE-POT-AWARD"]),
            per_seat: state
                .ledger
                .seats
                .iter()
                .map(|ledger| SeatOutcomeBreakdownView {
                    seat: ledger.seat,
                    result: if ledger.seat == *winner {
                        "win".to_owned()
                    } else {
                        "fold_loss".to_owned()
                    },
                    allocation: if ledger.seat == *winner {
                        *pot_total
                    } else {
                        0
                    },
                    contribution: ledger.total_contribution,
                    strength: None,
                })
                .collect(),
        }),
        TerminalOutcome::Showdown {
            winners,
            allocations,
            explanations,
            headline,
            decisive_comparison,
            comparison_basis,
            ..
        } => {
            let split = winners.len() > 1;
            Some(OutcomeRationaleView {
                result_kind: if split {
                    "showdown_split".to_owned()
                } else {
                    "showdown_win".to_owned()
                },
                decisive_cause: if split {
                    "equal_best_hand_split".to_owned()
                } else {
                    "best_showdown_hand".to_owned()
                },
                template_key: if split {
                    "river_ledger.showdown_split_pot".to_owned()
                } else {
                    "river_ledger.showdown_best_hand_win".to_owned()
                },
                headline: Some(headline.clone()),
                decisive_comparison: Some(decisive_comparison.clone()),
                comparison_basis: Some(comparison_basis.clone()),
                decisive_rule_ids: if split {
                    rule_ids(&["RL-SCORE-SHOWDOWN", "RL-SCORE-SPLIT", "RL-END-SHOWDOWN"])
                } else {
                    rule_ids(&["RL-SCORE-SHOWDOWN", "RL-END-SHOWDOWN"])
                },
                per_seat: state
                    .ledger
                    .seats
                    .iter()
                    .map(|ledger| {
                        let allocation = allocations
                            .iter()
                            .find(|share| share.seat == ledger.seat)
                            .map(|share| share.amount)
                            .unwrap_or(0);
                        let revealed = explanations
                            .iter()
                            .find(|explanation| explanation.seat == ledger.seat)
                            .and_then(|explanation| explanation.revealed.as_ref());
                        SeatOutcomeBreakdownView {
                            seat: ledger.seat,
                            result: if winners.contains(&ledger.seat) {
                                if split {
                                    "split".to_owned()
                                } else {
                                    "win".to_owned()
                                }
                            } else if revealed.is_some() {
                                "showdown_loss".to_owned()
                            } else {
                                "folded".to_owned()
                            },
                            allocation,
                            contribution: ledger.total_contribution,
                            strength: revealed.map(|reveal| ShowdownStrengthView {
                                category: reveal.category.clone(),
                                tie_break_vector: reveal.tie_break_vector.clone(),
                                best_five: reveal
                                    .best_five
                                    .iter()
                                    .copied()
                                    .map(card_view)
                                    .collect(),
                                category_ladder_position: reveal.category_ladder_position.clone(),
                                result_label: reveal.result_label.clone(),
                                hand_name: reveal.hand_name.clone(),
                                rank_explanation: reveal.rank_explanation.clone(),
                                comparison_note: reveal.comparison_note.clone(),
                                best_five_accessibility_label: reveal
                                    .best_five_accessibility_label
                                    .clone(),
                            }),
                        }
                    })
                    .collect(),
            })
        }
    }
}

fn rule_ids(ids: &[&str]) -> Vec<String> {
    ids.iter().map(|id| (*id).to_owned()).collect()
}

fn card_view(card: Card) -> CardView {
    CardView {
        card_id: card.id(),
        rank: card.rank.as_str().to_owned(),
        rank_value: card.rank.value(),
        suit: card.suit.as_str().to_owned(),
        label: card.public_label(),
    }
}

fn board_slots(state: &RiverLedgerState) -> Vec<BoardSlotView> {
    board_slot_labels()
        .into_iter()
        .enumerate()
        .map(|(index, (slot, street_label))| {
            let card = state.board.get(index).copied().map(card_view);
            match card {
                Some(card) => BoardSlotView {
                    slot: slot.to_owned(),
                    reveal_state: "revealed".to_owned(),
                    street_label: street_label.to_owned(),
                    visual_placeholder_label: card.label.clone(),
                    accessibility_label: format!("{street_label} revealed: {}.", card.label),
                    card: Some(card),
                },
                None => BoardSlotView {
                    slot: slot.to_owned(),
                    reveal_state: "pending".to_owned(),
                    street_label: street_label.to_owned(),
                    visual_placeholder_label: format!("{street_label} pending"),
                    accessibility_label: format!(
                        "Unrevealed {street_label} card. It is not available yet."
                    ),
                    card: None,
                },
            }
        })
        .collect()
}

fn board_slot_labels() -> [(&'static str, &'static str); 5] {
    [
        ("flop_1", "Flop 1"),
        ("flop_2", "Flop 2"),
        ("flop_3", "Flop 3"),
        ("turn", "Turn"),
        ("river", "River"),
    ]
}

fn encode_seats(seats: &[SeatView]) -> String {
    seats
        .iter()
        .map(|seat| {
            format!(
                "{}:{:?}:{}:{}:{}:{}:{}:{}:{}:{}",
                seat.seat.as_str(),
                seat.status,
                seat.street_contribution,
                seat.total_contribution,
                seat.hidden_hole_count,
                seat.ledger_display.round_contribution.label,
                seat.ledger_display.hand_contribution.label,
                seat.ledger_display.hole_card_summary.value,
                seat.ledger_display.role_badges.join("+"),
                seat.ledger_display.status_label
            )
        })
        .collect::<Vec<_>>()
        .join("|")
}

fn encode_cards(cards: &[CardView]) -> String {
    cards
        .iter()
        .map(|card| {
            format!(
                "{}:{}:{}:{}",
                card.card_id, card.rank, card.suit, card.label
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn encode_board_slots(slots: &[BoardSlotView]) -> String {
    slots
        .iter()
        .map(|slot| {
            format!(
                "{}:{}:{}:{}:{}",
                slot.slot,
                slot.reveal_state,
                slot.street_label,
                slot.visual_placeholder_label,
                slot.card
                    .as_ref()
                    .map_or_else(|| "none".to_owned(), |card| card.card_id.clone())
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn encode_private(private: &PrivateView) -> String {
    match private {
        PrivateView::Observer => "observer".to_owned(),
        PrivateView::Seat(view) => {
            format!("{}:{}", view.seat.as_str(), encode_cards(&view.hole_cards))
        }
    }
}

fn encode_terminal(terminal: &TerminalView) -> String {
    match terminal {
        TerminalView::NonTerminal => "non_terminal".to_owned(),
        TerminalView::LastLiveHand { winner, pot_total } => {
            format!("last_live:{}:{}", winner.as_str(), pot_total)
        }
        TerminalView::Showdown {
            winners,
            pot_total,
            allocations,
            explanations,
            presentation_v2,
        } => format!(
            "showdown:{}:{}:{}:{}:{}:{}:{}",
            winners
                .iter()
                .map(|seat| seat.as_str())
                .collect::<Vec<_>>()
                .join(","),
            pot_total,
            allocations
                .iter()
                .map(|(seat, amount)| format!("{}={amount}", seat.as_str()))
                .collect::<Vec<_>>()
                .join(","),
            explanations.join("|"),
            presentation_v2.result_banner.headline,
            presentation_v2.standings.len(),
            presentation_v2.folded_rows.len(),
        ),
    }
}

fn encode_rationale(rationale: Option<&OutcomeRationaleView>) -> String {
    let Some(rationale) = rationale else {
        return "none".to_owned();
    };
    format!(
        "{}:{}:{}:{}:{}:{}:{}:{}",
        rationale.result_kind,
        rationale.decisive_cause,
        rationale.template_key,
        encode_optional(rationale.headline.as_deref()),
        encode_optional(rationale.decisive_comparison.as_deref()),
        encode_optional(rationale.comparison_basis.as_deref()),
        rationale.decisive_rule_ids.join(","),
        rationale
            .per_seat
            .iter()
            .map(encode_rationale_seat)
            .collect::<Vec<_>>()
            .join("|")
    )
}

fn encode_rationale_seat(seat: &SeatOutcomeBreakdownView) -> String {
    format!(
        "{}:{}:{}:{}:{}",
        seat.seat.as_str(),
        seat.result,
        seat.allocation,
        seat.contribution,
        seat.strength
            .as_ref()
            .map_or_else(|| "none".to_owned(), encode_strength)
    )
}

fn encode_strength(strength: &ShowdownStrengthView) -> String {
    format!(
        "{}:{}:{}:{}:{}:{}:{}:{}:{}",
        strength.category,
        strength
            .tie_break_vector
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(","),
        encode_cards(&strength.best_five),
        strength.category_ladder_position.description,
        strength.result_label,
        strength.hand_name,
        strength.rank_explanation,
        strength.comparison_note,
        strength.best_five_accessibility_label
    )
}

fn encode_optional(value: Option<&str>) -> &str {
    value.unwrap_or("none")
}

fn seat_option(seat: Option<RiverLedgerSeat>) -> String {
    seat.map(RiverLedgerSeat::as_str)
        .unwrap_or_else(|| "none".to_owned())
}

fn phase(phase: Phase) -> &'static str {
    match phase {
        Phase::Setup => "setup",
        Phase::Betting { street } => street.as_str(),
        Phase::Showdown => "showdown",
        Phase::Terminal => "terminal",
    }
}

#[cfg(test)]
mod tests {
    use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, Viewer};

    use super::*;
    use crate::{
        apply_action, canonical_deck, setup_match, validate_command, Rank, SetupOptions, Suit,
    };

    fn seats(count: usize) -> Vec<SeatId> {
        (0..count)
            .map(|index| SeatId(format!("seat_{index}")))
            .collect()
    }

    fn actor(seat: &str) -> Actor {
        Actor {
            seat_id: SeatId(seat.to_owned()),
        }
    }

    fn command(state: &RiverLedgerState, seat: &str, segment: &str) -> CommandEnvelope {
        CommandEnvelope {
            actor: actor(seat),
            action_path: ActionPath {
                segments: vec![segment.to_owned()],
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    fn apply_segment(state: &mut RiverLedgerState, seat: &str, segment: &str) {
        let action =
            validate_command(state, &command(state, seat, segment)).expect("valid command");
        apply_action(state, action).expect("apply succeeds");
    }

    fn check_down_four_player_hand(seed: u64) -> RiverLedgerState {
        let mut state =
            setup_match(Seed(seed), &seats(4), &SetupOptions::default()).expect("setup");
        for (seat, segment) in [
            ("seat_3", "call"),
            ("seat_0", "call"),
            ("seat_1", "call"),
            ("seat_2", "check"),
            ("seat_1", "check"),
            ("seat_2", "check"),
            ("seat_3", "check"),
            ("seat_0", "check"),
            ("seat_1", "check"),
            ("seat_2", "check"),
            ("seat_3", "check"),
            ("seat_0", "check"),
            ("seat_1", "check"),
            ("seat_2", "check"),
            ("seat_3", "check"),
            ("seat_0", "check"),
        ] {
            apply_segment(&mut state, seat, segment);
        }
        state
    }

    fn royal_board() -> Vec<Card> {
        vec![
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Hearts),
        ]
    }

    #[test]
    fn observer_projection_exposes_counts_but_no_hole_or_deck_cards() {
        let state = setup_match(Seed(13), &seats(4), &SetupOptions::default()).expect("setup");
        let view = project_view(&state, &Viewer { seat_id: None });

        assert!(matches!(view.private_view, PrivateView::Observer));
        assert_eq!(view.board.len(), 0);
        assert_eq!(view.board_slots.len(), 5);
        assert!(view.board_slots.iter().all(|slot| {
            slot.reveal_state == "pending"
                && slot.card.is_none()
                && slot.visual_placeholder_label.contains("pending")
                && slot.accessibility_label.contains("not available yet")
        }));
        assert_eq!(view.board_slots[0].street_label, "Flop 1");
        assert_eq!(view.board_slots[3].street_label, "Turn");
        assert_eq!(view.board_slots[4].street_label, "River");
        assert_eq!(view.ui.seat_labels.len(), 6);
        assert_eq!(view.ui.seat_labels[0].seat, "seat_0");
        assert_eq!(view.ui.seat_labels[0].label, "Seat 0");
        assert_eq!(view.ui.seat_labels[5].seat, "seat_5");
        assert_eq!(view.ui.seat_labels[5].label, "Seat 5");
        assert_eq!(view.reserved_community_count, 5);
        assert!(view.seats.iter().all(|seat| seat.hidden_hole_count == 2));
        assert!(view.seats.iter().all(|seat| {
            seat.ledger_display.round_contribution.label == "This round"
                && seat.ledger_display.hand_contribution.label == "Hand total"
                && seat.ledger_display.hole_card_summary.label == "Hole cards"
                && seat.ledger_display.hole_card_summary.value == "2 hidden"
                && seat
                    .ledger_display
                    .hole_card_summary
                    .accessibility_label
                    .contains("2 hidden")
        }));
        assert!(view.seats.iter().any(|seat| {
            seat.seat == view.button
                && seat
                    .ledger_display
                    .role_badges
                    .contains(&"Button".to_owned())
        }));
        assert!(view.seats.iter().any(|seat| {
            Some(seat.seat) == view.active_seat
                && seat
                    .ledger_display
                    .role_badges
                    .contains(&"Active".to_owned())
        }));

        let summary = view.stable_summary();
        for card in state.private_hands_internal().iter().flatten() {
            assert!(!summary.contains(&card.id()));
        }
        for card in canonical_deck() {
            if !state.board.contains(&card) {
                assert!(!summary.contains(&card.id()));
            }
        }
    }

    #[test]
    fn seat_projection_exposes_only_own_hole_cards() {
        let state = setup_match(Seed(14), &seats(4), &SetupOptions::default()).expect("setup");
        let view = project_view(
            &state,
            &Viewer {
                seat_id: Some(SeatId("seat_1".to_owned())),
            },
        );

        let PrivateView::Seat(private) = &view.private_view else {
            panic!("seat private view expected");
        };
        assert_eq!(private.seat, RiverLedgerSeat::from_index(1).unwrap());
        let summary = view.stable_summary();
        for card in state.private_hand_for_internal(private.seat).unwrap() {
            assert!(summary.contains(&card.id()));
        }
        for (index, hand) in state.private_hands_internal().iter().enumerate() {
            if index != private.seat.index() {
                for card in hand {
                    assert!(!summary.contains(&card.id()));
                }
            }
        }
    }

    #[test]
    fn view_hashes_are_stable_and_viewer_distinct() {
        let state = setup_match(Seed(15), &seats(4), &SetupOptions::default()).expect("setup");
        let observer = project_view(&state, &Viewer { seat_id: None });
        let observer_again = project_view(&state, &Viewer { seat_id: None });
        let seat_0 = project_view(
            &state,
            &Viewer {
                seat_id: Some(SeatId("seat_0".to_owned())),
            },
        );
        let seat_1 = project_view(
            &state,
            &Viewer {
                seat_id: Some(SeatId("seat_1".to_owned())),
            },
        );

        assert_eq!(view_hash(&observer), view_hash(&observer_again));
        assert_ne!(view_hash(&observer), view_hash(&seat_0));
        assert_ne!(view_hash(&seat_0), view_hash(&seat_1));
    }

    #[test]
    fn foldout_terminal_rationale_reveals_no_private_strength() {
        let mut state = setup_match(Seed(21), &seats(3), &SetupOptions::default()).expect("setup");
        apply_segment(&mut state, "seat_0", "fold");
        apply_segment(&mut state, "seat_1", "fold");

        let view = project_view(&state, &Viewer { seat_id: None });
        let rationale = view
            .terminal_rationale
            .as_ref()
            .expect("terminal rationale");

        assert_eq!(rationale.result_kind, "last_live_hand");
        assert_eq!(rationale.decisive_cause, "last_live_after_folds");
        assert_eq!(rationale.template_key, "river_ledger.last_live_fold_win");
        assert!(rationale
            .per_seat
            .iter()
            .all(|seat| seat.strength.is_none()));

        let summary = view.stable_summary();
        for card in state.private_hands_internal().iter().flatten() {
            assert!(
                !summary.contains(&card.id()),
                "foldout rationale leaked {}",
                card.id()
            );
        }
    }

    #[test]
    fn showdown_win_terminal_rationale_carries_rust_revealed_strength() {
        let state = (0..200)
            .map(check_down_four_player_hand)
            .find(|state| {
                matches!(
                    state.terminal_outcome.as_ref(),
                    Some(TerminalOutcome::Showdown { winners, .. }) if winners.len() == 1
                )
            })
            .expect("seed with one showdown winner");

        let view = project_view(&state, &Viewer { seat_id: None });
        let rationale = view
            .terminal_rationale
            .as_ref()
            .expect("terminal rationale");

        assert_eq!(rationale.result_kind, "showdown_win");
        assert_eq!(rationale.decisive_cause, "best_showdown_hand");
        assert_eq!(
            rationale.template_key,
            "river_ledger.showdown_best_hand_win"
        );
        assert!(rationale.per_seat.iter().any(|seat| seat.result == "win"));
        assert!(rationale
            .per_seat
            .iter()
            .filter(|seat| seat.result != "folded")
            .all(|seat| seat.strength.is_some()));
    }

    #[test]
    fn showdown_split_terminal_rationale_marks_split_allocations() {
        let mut state = setup_match(Seed(21), &seats(4), &SetupOptions::default()).expect("setup");
        state.board = royal_board();
        state.ledger.pot_total = 12;
        for entry in &mut state.ledger.seats {
            entry.status = SeatStatus::ShowdownEligible;
        }
        state.terminal_outcome = Some(crate::showdown::resolve_showdown(&state));

        let view = project_view(&state, &Viewer { seat_id: None });
        let rationale = view
            .terminal_rationale
            .as_ref()
            .expect("terminal rationale");

        assert_eq!(rationale.result_kind, "showdown_split");
        assert_eq!(rationale.decisive_cause, "equal_best_hand_split");
        assert_eq!(rationale.template_key, "river_ledger.showdown_split_pot");
        assert!(rationale.per_seat.iter().any(|seat| seat.result == "split"));
    }
}
