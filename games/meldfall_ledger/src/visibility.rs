//! Public-observer and seat-private projection for Meldfall Ledger.
//!
//! Replay exports are redacted by the replay-export ticket.

use engine_core::{
    ActionChoice, ActionMetadata, ActionNode, ActionTree, Diagnostic, EffectEnvelope, Viewer,
    VisibilityScope,
};

use crate::{
    actions::{DISCARD_SEGMENT_PREFIX, LAY_OFF_SEGMENT_PREFIX, MELD_NEW_SEGMENT_PREFIX},
    cards::CardId,
    effects::MeldfallEffect,
    ids::{GAME_ID, RULES_VERSION_LABEL, VARIANT_ID},
    state::{
        LastSettlementSeatSnapshot, LastSettlementSnapshot, MatchOutcome, MatchState, MeldGroup,
        MeldTableau, SeatIndex, TableCard,
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeldfallView {
    pub game_id: String,
    pub variant_id: String,
    pub rules_version_label: String,
    pub active_seat_index: SeatIndex,
    pub dealer_index: SeatIndex,
    pub phase: String,
    pub stock_count: usize,
    pub discard: Vec<String>,
    pub hand_counts: Vec<usize>,
    pub cumulative_scores: Vec<i32>,
    pub round_played_scores: Vec<i32>,
    pub tableau: PublicTableauView,
    pub round_end: Option<String>,
    pub last_settlement: Option<MeldfallSettlementView>,
    pub terminal: Option<PublicMatchOutcomeView>,
    pub private: PrivateView,
}

impl MeldfallView {
    pub fn stable_string(&self) -> String {
        format!(
            "game={};variant={};rules={};active={};dealer={};phase={};stock_count={};discard=[{}];hand_counts=[{}];scores=[{}];round_scores=[{}];tableau=[{}];round_end={};last_settlement={};terminal={};private={}",
            self.game_id,
            self.variant_id,
            self.rules_version_label,
            self.active_seat_index,
            self.dealer_index,
            self.phase,
            self.stock_count,
            self.discard.join(","),
            usize_list(&self.hand_counts),
            int_list(&self.cumulative_scores),
            int_list(&self.round_played_scores),
            self.tableau.stable_string(),
            self.round_end.as_deref().unwrap_or("none"),
            self.last_settlement
                .as_ref()
                .map(MeldfallSettlementView::stable_string)
                .unwrap_or_else(|| "none".to_owned()),
            self.terminal
                .as_ref()
                .map(PublicMatchOutcomeView::stable_string)
                .unwrap_or_else(|| "none".to_owned()),
            self.private.stable_string()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeldfallSettlementView {
    pub round_index: u32,
    pub round_end_reason: String,
    pub seats: Vec<MeldfallSettlementSeatView>,
}

impl MeldfallSettlementView {
    pub fn stable_string(&self) -> String {
        let seats = self
            .seats
            .iter()
            .map(MeldfallSettlementSeatView::stable_string)
            .collect::<Vec<_>>()
            .join(";");
        format!(
            "round={}:reason={}:seats=[{}]",
            self.round_index, self.round_end_reason, seats
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeldfallSettlementSeatView {
    pub seat_index: SeatIndex,
    pub tabled_positive: i32,
    pub in_hand_penalty: i32,
    pub remaining_hand_count: usize,
    pub delta: i32,
    pub cumulative_score: i32,
    pub rank: usize,
    pub winner: bool,
}

impl MeldfallSettlementSeatView {
    pub fn stable_string(&self) -> String {
        format!(
            "{}:tabled={}:penalty={}:remaining={}:delta={}:cumulative={}:rank={}:winner={}",
            self.seat_index,
            self.tabled_positive,
            self.in_hand_penalty,
            self.remaining_hand_count,
            self.delta,
            self.cumulative_score,
            self.rank,
            self.winner
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrivateView {
    Observer,
    Seat(SeatPrivateView),
}

impl PrivateView {
    pub fn stable_string(&self) -> String {
        match self {
            Self::Observer => "observer".to_owned(),
            Self::Seat(view) => view.stable_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatPrivateView {
    pub seat_index: SeatIndex,
    pub hand: Vec<String>,
}

impl SeatPrivateView {
    pub fn stable_string(&self) -> String {
        format!("seat={}:hand=[{}]", self.seat_index, self.hand.join(","))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicMatchOutcomeView {
    pub standings: Vec<PublicSeatStandingView>,
}

impl PublicMatchOutcomeView {
    pub fn stable_string(&self) -> String {
        self.standings
            .iter()
            .map(PublicSeatStandingView::stable_string)
            .collect::<Vec<_>>()
            .join(";")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicSeatStandingView {
    pub seat_index: SeatIndex,
    pub rank: usize,
    pub cumulative_score: i32,
    pub latest_round_delta: i32,
    pub winner: bool,
}

impl PublicSeatStandingView {
    pub fn stable_string(&self) -> String {
        format!(
            "seat={}:rank={}:score={}:delta={}:winner={}",
            self.seat_index, self.rank, self.cumulative_score, self.latest_round_delta, self.winner
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicTableauView {
    pub groups: Vec<PublicMeldGroupView>,
}

impl PublicTableauView {
    pub fn stable_string(&self) -> String {
        self.groups
            .iter()
            .map(PublicMeldGroupView::stable_string)
            .collect::<Vec<_>>()
            .join(";")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicMeldGroupView {
    pub id: String,
    pub kind: String,
    pub origin_seat: usize,
    pub cards: Vec<PublicTableCardView>,
}

impl PublicMeldGroupView {
    pub fn stable_string(&self) -> String {
        let cards = self
            .cards
            .iter()
            .map(PublicTableCardView::stable_string)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{}:{}:origin={}:cards=[{}]",
            self.id, self.kind, self.origin_seat, cards
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicTableCardView {
    pub card: String,
    pub played_by: usize,
    pub score_credit_owner: usize,
    pub play_turn: u32,
}

impl PublicTableCardView {
    pub fn stable_string(&self) -> String {
        format!(
            "{}:played_by={}:credit={}:turn={}",
            self.card, self.played_by, self.score_credit_owner, self.play_turn
        )
    }
}

pub fn project_public_tableau(tableau: &MeldTableau) -> PublicTableauView {
    PublicTableauView {
        groups: tableau.groups.iter().map(project_meld_group).collect(),
    }
}

pub fn project_tableau_for_viewer(tableau: &MeldTableau, _viewer: &Viewer) -> PublicTableauView {
    project_public_tableau(tableau)
}

pub fn project_view(state: &MatchState, viewer: &Viewer) -> MeldfallView {
    let viewer_seat_index = viewer_seat_index(state, viewer);
    MeldfallView {
        game_id: GAME_ID.to_owned(),
        variant_id: VARIANT_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        active_seat_index: state.round.active_seat_index,
        dealer_index: state.dealer_index,
        phase: state.round.phase.as_str().to_owned(),
        stock_count: state.round.stock.len(),
        discard: card_ids(&state.round.discard),
        hand_counts: state
            .round
            .seats
            .iter()
            .map(|seat| seat.hand.len())
            .collect(),
        cumulative_scores: state.cumulative_scores.clone(),
        round_played_scores: state.round.round_played_scores.clone(),
        tableau: project_tableau_for_viewer(&state.round.tableau, viewer),
        round_end: state
            .round
            .round_end
            .as_ref()
            .map(|round| round.stable_string()),
        last_settlement: state.last_settlement.as_ref().map(project_last_settlement),
        terminal: state.terminal.as_ref().map(project_match_outcome),
        private: private_view(state, viewer_seat_index),
    }
}

pub fn project_action_tree_for_viewer(
    tree: &ActionTree,
    state: &MatchState,
    viewer: &Viewer,
) -> ActionTree {
    let viewer_seat_index = viewer_seat_index(state, viewer);
    let may_see_active_hand = viewer_seat_index == Some(state.round.active_seat_index);
    ActionTree {
        root: project_action_node_for_viewer(&tree.root, may_see_active_hand),
        freshness_token: tree.freshness_token,
    }
}

pub fn project_effects_for_viewer(
    effects: &[EffectEnvelope<MeldfallEffect>],
    viewer: &Viewer,
) -> Vec<EffectEnvelope<MeldfallEffect>> {
    effects
        .iter()
        .filter(|effect| match &effect.visibility {
            VisibilityScope::Public => true,
            VisibilityScope::PrivateToSeat(seat_id) => viewer.seat_id.as_ref() == Some(seat_id),
        })
        .cloned()
        .collect()
}

pub fn redact_diagnostic_for_viewer(
    diagnostic: &Diagnostic,
    viewer_may_see_cards: bool,
) -> Diagnostic {
    if viewer_may_see_cards {
        return diagnostic.clone();
    }
    Diagnostic {
        code: diagnostic.code.clone(),
        message: redact_card_ids(&diagnostic.message),
    }
}

pub fn contains_card_id_in_debug<T: std::fmt::Debug>(value: &T, card: CardId) -> bool {
    format!("{value:?}").contains(&card.as_str())
}

fn project_meld_group(group: &MeldGroup) -> PublicMeldGroupView {
    PublicMeldGroupView {
        id: group.id.as_string(),
        kind: group.kind.stable_string(),
        origin_seat: group.origin_seat,
        cards: group.cards.iter().map(project_table_card).collect(),
    }
}

fn project_table_card(card: &TableCard) -> PublicTableCardView {
    PublicTableCardView {
        card: card.card.as_str(),
        played_by: card.played_by,
        score_credit_owner: card.score_credit_owner,
        play_turn: card.play_turn.0,
    }
}

fn project_last_settlement(snapshot: &LastSettlementSnapshot) -> MeldfallSettlementView {
    MeldfallSettlementView {
        round_index: snapshot.round_index,
        round_end_reason: snapshot.round_end_reason.clone(),
        seats: snapshot
            .seats
            .iter()
            .map(project_last_settlement_seat)
            .collect(),
    }
}

fn project_last_settlement_seat(
    snapshot: &LastSettlementSeatSnapshot,
) -> MeldfallSettlementSeatView {
    MeldfallSettlementSeatView {
        seat_index: snapshot.seat_index,
        tabled_positive: snapshot.tabled_positive,
        in_hand_penalty: snapshot.in_hand_penalty,
        remaining_hand_count: snapshot.remaining_hand_count,
        delta: snapshot.round_delta,
        cumulative_score: snapshot.cumulative_score,
        rank: snapshot.rank,
        winner: snapshot.winner,
    }
}

fn private_view(state: &MatchState, viewer_seat_index: Option<SeatIndex>) -> PrivateView {
    match viewer_seat_index {
        Some(seat_index) => PrivateView::Seat(SeatPrivateView {
            seat_index,
            hand: card_ids(&state.round.seats[seat_index].hand),
        }),
        None => PrivateView::Observer,
    }
}

fn project_match_outcome(outcome: &MatchOutcome) -> PublicMatchOutcomeView {
    PublicMatchOutcomeView {
        standings: outcome
            .standings
            .iter()
            .map(|standing| PublicSeatStandingView {
                seat_index: standing.seat_index,
                rank: standing.rank,
                cumulative_score: standing.cumulative_score,
                latest_round_delta: standing.latest_round_delta,
                winner: standing.winner,
            })
            .collect(),
    }
}

fn project_action_node_for_viewer(node: &ActionNode, may_see_active_hand: bool) -> ActionNode {
    ActionNode {
        choices: node
            .choices
            .iter()
            .filter_map(|choice| project_action_choice_for_viewer(choice, may_see_active_hand))
            .collect(),
    }
}

fn project_action_choice_for_viewer(
    choice: &ActionChoice,
    may_see_active_hand: bool,
) -> Option<ActionChoice> {
    if !may_see_active_hand && choice_exposes_active_hand(choice) {
        return None;
    }
    let mut projected = choice.clone();
    if !may_see_active_hand {
        projected.metadata = projected
            .metadata
            .into_iter()
            .map(redact_metadata)
            .collect();
    }
    projected.next = projected
        .next
        .map(|next| Box::new(project_action_node_for_viewer(&next, may_see_active_hand)));
    Some(projected)
}

fn choice_exposes_active_hand(choice: &ActionChoice) -> bool {
    choice.segment.starts_with(MELD_NEW_SEGMENT_PREFIX)
        || choice.segment.starts_with(LAY_OFF_SEGMENT_PREFIX)
        || choice.segment.starts_with(DISCARD_SEGMENT_PREFIX)
        || choice
            .metadata
            .iter()
            .any(|metadata| matches!(metadata.key.as_str(), "cards" | "card"))
}

fn redact_metadata(metadata: ActionMetadata) -> ActionMetadata {
    if matches!(metadata.key.as_str(), "cards" | "card") {
        ActionMetadata {
            key: metadata.key,
            value: "viewer_authorized_only".to_owned(),
        }
    } else {
        metadata
    }
}

fn viewer_seat_index(state: &MatchState, viewer: &Viewer) -> Option<SeatIndex> {
    viewer.seat_id.as_ref().and_then(|seat_id| {
        state
            .seats
            .iter()
            .position(|candidate| candidate == seat_id)
    })
}

fn card_ids(cards: &[CardId]) -> Vec<String> {
    cards.iter().map(|card| card.as_str()).collect()
}

fn redact_card_ids(message: &str) -> String {
    let mut redacted = message.to_owned();
    for suit in ["clubs", "diamonds", "hearts", "spades"] {
        for rank in [
            "ace", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack",
            "queen", "king",
        ] {
            redacted = redacted.replace(&format!("{rank}_{suit}"), "hidden_card");
        }
    }
    redacted
}

fn usize_list(values: &[usize]) -> String {
    values
        .iter()
        .map(usize::to_string)
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
