use engine_core::{FreshnessToken, StableSerialize, Viewer};

use crate::{
    ids::{CardId, HighCardDuelSeat, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID},
    state::{HighCardDuelState, Phase, Score, TerminalOutcome},
    ui::{
        card_accessibility_label, face_down_commitment_label, revealed_card_accessibility_label,
        ui_metadata, UiMetadata,
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
    pub round_number: u8,
    pub round_limit: u8,
    pub phase: Phase,
    pub active_seat: Option<HighCardDuelSeat>,
    pub lead_seat: Option<HighCardDuelSeat>,
    pub reply_seat: Option<HighCardDuelSeat>,
    pub score: Score,
    pub hand_counts: HandCountsView,
    pub deck_count: u8,
    pub commitments: CommitmentViews,
    pub revealed_cards: Vec<RevealedRoundView>,
    pub terminal: TerminalView,
    pub freshness_token: FreshnessToken,
    pub private_view: PrivateView,
    pub ui: UiMetadata,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HandCountsView {
    pub seat_0: u8,
    pub seat_1: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommitmentViews {
    pub seat_0: CommitmentView,
    pub seat_1: CommitmentView,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommitmentView {
    pub seat: HighCardDuelSeat,
    pub status: String,
    pub card: Option<CardView>,
    pub accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardView {
    pub card_id: String,
    pub rank: u8,
    pub sigil: String,
    pub accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevealedRoundView {
    pub round_number: u8,
    pub seat_0_card: CardView,
    pub seat_1_card: CardView,
    pub winner: Option<HighCardDuelSeat>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalView {
    NonTerminal,
    Win {
        winning_seat: HighCardDuelSeat,
        rationale: OutcomeRationaleView,
    },
    Draw {
        rationale: OutcomeRationaleView,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutcomeRationaleView {
    pub result_kind: String,
    pub decisive_cause: String,
    pub template_key: String,
    pub decisive_rule_ids: Vec<String>,
    pub final_score: Score,
    pub round_breakdowns: Vec<RoundOutcomeBreakdownView>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoundOutcomeBreakdownView {
    pub round_number: u8,
    pub seat_0_rank: u8,
    pub seat_1_rank: u8,
    pub winner: Option<HighCardDuelSeat>,
    pub point_delta_seat_0: u8,
    pub point_delta_seat_1: u8,
    pub cumulative_score: Score,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrivateView {
    Observer,
    Seat {
        seat: HighCardDuelSeat,
        hand: Vec<CardView>,
        own_commitment: Option<CardView>,
    },
}

pub fn project_view(state: &HighCardDuelState, viewer: &Viewer) -> PublicView {
    let viewer_seat = viewer_seat(state, viewer);

    PublicView {
        schema_version: 1,
        rules_version: 1,
        game_id: GAME_ID.to_owned(),
        display_name: "High Card Duel".to_owned(),
        variant_id: VARIANT_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        round_number: state.round_number,
        round_limit: state.variant.round_limit,
        phase: state.phase,
        active_seat: active_seat(state),
        lead_seat: (state.phase != Phase::Terminal).then_some(state.lead_seat),
        reply_seat: (state.phase != Phase::Terminal).then_some(state.lead_seat.other()),
        score: state.score,
        hand_counts: HandCountsView {
            seat_0: state.hands[HighCardDuelSeat::Seat0.index()].len() as u8,
            seat_1: state.hands[HighCardDuelSeat::Seat1.index()].len() as u8,
        },
        deck_count: state.deck.len() as u8,
        commitments: CommitmentViews {
            seat_0: commitment_view(state, HighCardDuelSeat::Seat0, viewer_seat),
            seat_1: commitment_view(state, HighCardDuelSeat::Seat1, viewer_seat),
        },
        revealed_cards: state
            .revealed_history
            .iter()
            .map(|round| RevealedRoundView {
                round_number: round.round_number,
                seat_0_card: revealed_card_view(round.seat_0_card),
                seat_1_card: revealed_card_view(round.seat_1_card),
                winner: round.winner,
            })
            .collect(),
        terminal: terminal_view(state.terminal_outcome, state.score, &state.revealed_history),
        freshness_token: state.freshness_token,
        private_view: private_view(state, viewer_seat),
        ui: ui_metadata(),
    }
}

impl PublicView {
    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};rules={};game={};variant={};label={};round={}/{};phase={};active={};lead={};reply={};score={}-{};hands={}-{};deck_count={};commitments={}|{};revealed={};terminal={};freshness={};private={};ui={}",
            self.schema_version,
            self.rules_version,
            self.game_id,
            self.variant_id,
            self.rules_version_label,
            self.round_number,
            self.round_limit,
            self.phase.as_str(),
            seat_option(self.active_seat),
            seat_option(self.lead_seat),
            seat_option(self.reply_seat),
            self.score.seat_0,
            self.score.seat_1,
            self.hand_counts.seat_0,
            self.hand_counts.seat_1,
            self.deck_count,
            encode_commitment(&self.commitments.seat_0),
            encode_commitment(&self.commitments.seat_1),
            self.revealed_cards.iter().map(encode_revealed).collect::<Vec<_>>().join(","),
            encode_terminal(&self.terminal),
            self.freshness_token.0,
            encode_private(&self.private_view),
            encode_ui(&self.ui),
        )
    }
}

impl StableSerialize for PublicView {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

fn viewer_seat(state: &HighCardDuelState, viewer: &Viewer) -> Option<HighCardDuelSeat> {
    viewer
        .seat_id
        .as_ref()
        .and_then(|seat_id| {
            state
                .seats
                .iter()
                .position(|candidate| candidate == seat_id)
        })
        .and_then(HighCardDuelSeat::from_index)
}

fn active_seat(state: &HighCardDuelState) -> Option<HighCardDuelSeat> {
    match state.phase {
        Phase::LeadCommit => Some(state.lead_seat),
        Phase::ReplyCommit => Some(state.lead_seat.other()),
        Phase::Revealed | Phase::Terminal => None,
    }
}

fn commitment_view(
    state: &HighCardDuelState,
    seat: HighCardDuelSeat,
    viewer_seat: Option<HighCardDuelSeat>,
) -> CommitmentView {
    match state.commitment_for(seat) {
        Some(card) if viewer_seat == Some(seat) => CommitmentView {
            seat,
            status: "own_card".to_owned(),
            card: Some(private_card_view(card)),
            accessibility_label: format!("{} for {}", face_down_commitment_label(), seat.as_str()),
        },
        Some(_) => CommitmentView {
            seat,
            status: "face_down".to_owned(),
            card: None,
            accessibility_label: format!("{} for {}", face_down_commitment_label(), seat.as_str()),
        },
        None => CommitmentView {
            seat,
            status: "empty".to_owned(),
            card: None,
            accessibility_label: format!("No commitment for {}", seat.as_str()),
        },
    }
}

fn private_view(state: &HighCardDuelState, viewer_seat: Option<HighCardDuelSeat>) -> PrivateView {
    match viewer_seat {
        Some(seat) => PrivateView::Seat {
            seat,
            hand: state
                .hand_for(seat)
                .iter()
                .copied()
                .map(private_card_view)
                .collect(),
            own_commitment: state.commitment_for(seat).map(private_card_view),
        },
        None => PrivateView::Observer,
    }
}

fn private_card_view(card: CardId) -> CardView {
    CardView {
        card_id: card.stable_id(),
        rank: card.rank(),
        sigil: card.sigil().as_str().to_owned(),
        accessibility_label: card_accessibility_label(card.rank()),
    }
}

fn revealed_card_view(card: CardId) -> CardView {
    CardView {
        card_id: card.stable_id(),
        rank: card.rank(),
        sigil: card.sigil().as_str().to_owned(),
        accessibility_label: revealed_card_accessibility_label(card.rank()),
    }
}

fn terminal_view(
    outcome: Option<TerminalOutcome>,
    final_score: Score,
    revealed_history: &[crate::state::RevealedRound],
) -> TerminalView {
    match outcome {
        None => TerminalView::NonTerminal,
        Some(TerminalOutcome::Win { seat }) => TerminalView::Win {
            winning_seat: seat,
            rationale: outcome_rationale("win", final_score, revealed_history),
        },
        Some(TerminalOutcome::Draw) => TerminalView::Draw {
            rationale: outcome_rationale("draw", final_score, revealed_history),
        },
    }
}

fn outcome_rationale(
    result_kind: &str,
    final_score: Score,
    revealed_history: &[crate::state::RevealedRound],
) -> OutcomeRationaleView {
    OutcomeRationaleView {
        result_kind: result_kind.to_owned(),
        decisive_cause: "final_score_after_round_limit".to_owned(),
        template_key: match result_kind {
            "win" => "high_card_duel.final_score_win",
            "draw" => "high_card_duel.final_score_draw",
            _ => unreachable!("terminal result kind is win or draw"),
        }
        .to_owned(),
        decisive_rule_ids: match result_kind {
            "win" => vec![
                "HCD-ROUND-005".to_owned(),
                "HCD-END-001".to_owned(),
                "HCD-END-002".to_owned(),
            ],
            "draw" => vec![
                "HCD-ROUND-006".to_owned(),
                "HCD-END-001".to_owned(),
                "HCD-END-003".to_owned(),
            ],
            _ => unreachable!("terminal result kind is win or draw"),
        },
        final_score,
        round_breakdowns: round_breakdowns(revealed_history),
    }
}

fn round_breakdowns(
    revealed_history: &[crate::state::RevealedRound],
) -> Vec<RoundOutcomeBreakdownView> {
    let mut cumulative_score = Score {
        seat_0: 0,
        seat_1: 0,
    };
    revealed_history
        .iter()
        .map(|round| {
            let (point_delta_seat_0, point_delta_seat_1) = match round.winner {
                Some(HighCardDuelSeat::Seat0) => (1, 0),
                Some(HighCardDuelSeat::Seat1) => (0, 1),
                None => (0, 0),
            };
            cumulative_score.seat_0 += point_delta_seat_0;
            cumulative_score.seat_1 += point_delta_seat_1;
            RoundOutcomeBreakdownView {
                round_number: round.round_number,
                seat_0_rank: round.seat_0_card.rank(),
                seat_1_rank: round.seat_1_card.rank(),
                winner: round.winner,
                point_delta_seat_0,
                point_delta_seat_1,
                cumulative_score,
            }
        })
        .collect()
}

fn seat_option(seat: Option<HighCardDuelSeat>) -> &'static str {
    seat.map_or("none", HighCardDuelSeat::as_str)
}

fn encode_commitment(commitment: &CommitmentView) -> String {
    format!(
        "{}:{}:{}",
        commitment.seat.as_str(),
        commitment.status,
        commitment
            .card
            .as_ref()
            .map_or_else(|| "redacted".to_owned(), encode_card)
    )
}

fn encode_revealed(round: &RevealedRoundView) -> String {
    format!(
        "{}:{}:{}:{}",
        round.round_number,
        encode_card(&round.seat_0_card),
        encode_card(&round.seat_1_card),
        seat_option(round.winner)
    )
}

fn encode_card(card: &CardView) -> String {
    format!(
        "{}:rank{}:{}:{}",
        card.card_id, card.rank, card.sigil, card.accessibility_label
    )
}

fn encode_terminal(terminal: &TerminalView) -> String {
    match terminal {
        TerminalView::NonTerminal => "non_terminal".to_owned(),
        TerminalView::Win {
            winning_seat,
            rationale,
        } => format!(
            "win:{}:{}",
            winning_seat.as_str(),
            encode_rationale(rationale)
        ),
        TerminalView::Draw { rationale } => format!("draw:{}", encode_rationale(rationale)),
    }
}

fn encode_rationale(rationale: &OutcomeRationaleView) -> String {
    format!(
        "{}|{}|{}|{}|final={}-{}|rounds={}",
        rationale.result_kind,
        rationale.decisive_cause,
        rationale.template_key,
        rationale.decisive_rule_ids.join("+"),
        rationale.final_score.seat_0,
        rationale.final_score.seat_1,
        rationale
            .round_breakdowns
            .iter()
            .map(encode_round_breakdown)
            .collect::<Vec<_>>()
            .join("/")
    )
}

fn encode_round_breakdown(round: &RoundOutcomeBreakdownView) -> String {
    format!(
        "{}:{}-{}:{}:{}-{}:{}-{}",
        round.round_number,
        round.seat_0_rank,
        round.seat_1_rank,
        seat_option(round.winner),
        round.point_delta_seat_0,
        round.point_delta_seat_1,
        round.cumulative_score.seat_0,
        round.cumulative_score.seat_1
    )
}

fn encode_private(private: &PrivateView) -> String {
    match private {
        PrivateView::Observer => "observer".to_owned(),
        PrivateView::Seat {
            seat,
            hand,
            own_commitment,
        } => format!(
            "{}:hand={}:commitment={}",
            seat.as_str(),
            hand.iter().map(encode_card).collect::<Vec<_>>().join("|"),
            own_commitment
                .as_ref()
                .map_or_else(|| "none".to_owned(), encode_card)
        ),
    }
}

fn encode_ui(ui: &UiMetadata) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}",
        ui.table_label,
        ui.card_back_token,
        ui.own_card_token,
        ui.revealed_card_token,
        ui.empty_commitment_token,
        ui.face_down_commitment_token,
        ui.commit_action_label,
        ui.observer_disabled_reason
    )
}
