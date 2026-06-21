use engine_core::{StableSerialize, Viewer};

use crate::{
    cards::CardId,
    effects::VowTideEffect,
    ids::{VowTideSeat, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID},
    state::{CapturedTrick, Phase, TrickPlay, VowTideState},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicView {
    pub schema_version: u32,
    pub rules_version: u32,
    pub game_id: String,
    pub variant_id: String,
    pub rules_version_label: String,
    pub phase: String,
    pub active_seat: Option<VowTideSeat>,
    pub seats: Vec<String>,
    pub seat_labels: Vec<String>,
    pub dealer: VowTideSeat,
    pub hand_index: u32,
    pub hand_size: u8,
    pub hand_schedule: Vec<u8>,
    pub trump_indicator: CardView,
    pub hand_counts: Vec<(VowTideSeat, u8)>,
    pub hidden_stock_count: u8,
    pub public_bids: Vec<(VowTideSeat, Option<u8>)>,
    pub trick_counts: Vec<(VowTideSeat, u8)>,
    pub current_trick: Vec<PlayedCardView>,
    pub captured_tricks: Vec<CapturedTrickView>,
    pub cumulative_scores: Vec<(VowTideSeat, i16)>,
    pub completed_hand_count: usize,
    pub terminal: Option<TerminalView>,
    pub private_view: PrivateView,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrivateView {
    Observer,
    Seat(SeatPrivateView),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatPrivateView {
    pub seat: VowTideSeat,
    pub own_hand: Vec<CardView>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardView {
    pub card_id: String,
    pub suit: String,
    pub rank: String,
    pub label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayedCardView {
    pub seat: VowTideSeat,
    pub card: CardView,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapturedTrickView {
    pub hand_index: u32,
    pub trick_index: u8,
    pub winner: VowTideSeat,
    pub plays: Vec<PlayedCardView>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalView {
    pub winners: Vec<VowTideSeat>,
    pub standings: Vec<(VowTideSeat, i16, u8, bool)>,
    pub hands_played: u32,
}

pub fn project_view(state: &VowTideState, viewer: &Viewer) -> PublicView {
    let viewer_seat = viewer_seat(state, viewer);
    PublicView {
        schema_version: 1,
        rules_version: 1,
        game_id: GAME_ID.to_owned(),
        variant_id: VARIANT_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        phase: phase_label(&state.phase).to_owned(),
        active_seat: state.active_seat(),
        seats: state.seats.iter().map(|seat| seat.0.clone()).collect(),
        seat_labels: state.seat_labels.clone(),
        dealer: state.dealer,
        hand_index: state.hand_index,
        hand_size: state.current_hand_size().unwrap_or_default(),
        hand_schedule: state.hand_schedule.clone(),
        trump_indicator: card_view(state.trump_indicator),
        hand_counts: state
            .private_hands
            .iter()
            .map(|(seat, hand)| (*seat, hand.len() as u8))
            .collect(),
        hidden_stock_count: state.hidden_stock.len() as u8,
        public_bids: state.public_bids.clone(),
        trick_counts: state.trick_counts.clone(),
        current_trick: current_trick(state),
        captured_tricks: state
            .captured_tricks
            .iter()
            .map(captured_trick_view)
            .collect(),
        cumulative_scores: state.cumulative_scores.clone(),
        completed_hand_count: state.completed_hands.len(),
        terminal: state
            .terminal_outcome
            .as_ref()
            .map(|terminal| TerminalView {
                winners: terminal.winners.clone(),
                standings: terminal
                    .standings
                    .iter()
                    .map(|standing| {
                        (
                            standing.seat,
                            standing.cumulative_score,
                            standing.rank,
                            standing.is_winner,
                        )
                    })
                    .collect(),
                hands_played: terminal.hands_played,
            }),
        private_view: private_view(state, viewer_seat),
    }
}

pub fn filter_effects_for_viewer(
    effects: &[VowTideEffect],
    _viewer: &Viewer,
) -> Vec<VowTideEffect> {
    effects.to_vec()
}

impl PublicView {
    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};rules={};game={};variant={};phase={};active={};seats={};labels={};dealer={};hand={};size={};schedule={:?};trump={};hand_counts={};stock_count={};bids={};tricks={};current={};captured={};scores={};completed={};terminal={};private={}",
            self.schema_version,
            self.rules_version,
            self.game_id,
            self.variant_id,
            self.phase,
            seat_option(self.active_seat),
            self.seats.join("/"),
            self.seat_labels.join("/"),
            self.dealer.as_str(),
            self.hand_index,
            self.hand_size,
            self.hand_schedule,
            self.trump_indicator.card_id,
            encode_seat_u8(&self.hand_counts),
            self.hidden_stock_count,
            encode_bids(&self.public_bids),
            encode_seat_u8(&self.trick_counts),
            self.current_trick
                .iter()
                .map(encode_played)
                .collect::<Vec<_>>()
                .join(","),
            self.captured_tricks
                .iter()
                .map(encode_captured)
                .collect::<Vec<_>>()
                .join("|"),
            encode_scores(&self.cumulative_scores),
            self.completed_hand_count,
            encode_terminal(self.terminal.as_ref()),
            encode_private(&self.private_view),
        )
    }
}

impl StableSerialize for PublicView {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

fn viewer_seat(state: &VowTideState, viewer: &Viewer) -> Option<VowTideSeat> {
    viewer
        .seat_id
        .as_ref()
        .and_then(|seat_id| {
            state
                .seats
                .iter()
                .position(|candidate| candidate == seat_id)
        })
        .and_then(VowTideSeat::from_index)
}

fn private_view(state: &VowTideState, viewer_seat: Option<VowTideSeat>) -> PrivateView {
    match viewer_seat {
        Some(seat) => PrivateView::Seat(SeatPrivateView {
            seat,
            own_hand: state
                .hand_for_internal(seat)
                .iter()
                .copied()
                .map(card_view)
                .collect(),
        }),
        None => PrivateView::Observer,
    }
}

fn current_trick(state: &VowTideState) -> Vec<PlayedCardView> {
    state
        .playing_state()
        .map(|playing| {
            playing
                .current_trick
                .plays
                .iter()
                .copied()
                .map(played_card_view)
                .collect()
        })
        .unwrap_or_default()
}

fn captured_trick_view(trick: &CapturedTrick) -> CapturedTrickView {
    CapturedTrickView {
        hand_index: trick.hand_index,
        trick_index: trick.trick_index,
        winner: trick.winner,
        plays: trick.plays.iter().copied().map(played_card_view).collect(),
    }
}

fn played_card_view(play: TrickPlay) -> PlayedCardView {
    PlayedCardView {
        seat: play.seat,
        card: card_view(play.card),
    }
}

fn card_view(card: CardId) -> CardView {
    let card_value = card.card();
    CardView {
        card_id: card.as_str(),
        suit: card_value.suit.as_str().to_owned(),
        rank: card_value.rank.as_str().to_owned(),
        label: card_value.public_label(),
    }
}

fn phase_label(phase: &Phase) -> &'static str {
    match phase {
        Phase::Bidding(_) => "bidding",
        Phase::PlayingTrick(_) => "playing_trick",
        Phase::Terminal(_) => "terminal",
    }
}

fn seat_option(seat: Option<VowTideSeat>) -> String {
    seat.map(|seat| seat.as_str().to_owned())
        .unwrap_or_else(|| "none".to_owned())
}

fn encode_private(private: &PrivateView) -> String {
    match private {
        PrivateView::Observer => "observer".to_owned(),
        PrivateView::Seat(view) => format!(
            "{}:{}",
            view.seat.as_str(),
            view.own_hand
                .iter()
                .map(|card| card.card_id.clone())
                .collect::<Vec<_>>()
                .join("/")
        ),
    }
}

fn encode_played(play: &PlayedCardView) -> String {
    format!("{}:{}", play.seat.as_str(), play.card.card_id)
}

fn encode_captured(trick: &CapturedTrickView) -> String {
    format!(
        "{}:{}:{}:{}",
        trick.hand_index,
        trick.trick_index,
        trick.winner.as_str(),
        trick
            .plays
            .iter()
            .map(encode_played)
            .collect::<Vec<_>>()
            .join("/")
    )
}

fn encode_bids(values: &[(VowTideSeat, Option<u8>)]) -> String {
    values
        .iter()
        .map(|(seat, bid)| {
            format!(
                "{}:{}",
                seat.as_str(),
                bid.map(|value| value.to_string())
                    .unwrap_or_else(|| "none".to_owned())
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn encode_seat_u8(values: &[(VowTideSeat, u8)]) -> String {
    values
        .iter()
        .map(|(seat, value)| format!("{}:{value}", seat.as_str()))
        .collect::<Vec<_>>()
        .join(",")
}

fn encode_scores(values: &[(VowTideSeat, i16)]) -> String {
    values
        .iter()
        .map(|(seat, value)| format!("{}:{value}", seat.as_str()))
        .collect::<Vec<_>>()
        .join(",")
}

fn encode_terminal(terminal: Option<&TerminalView>) -> String {
    terminal
        .map(|terminal| {
            format!(
                "winners={};standings={};hands={}",
                terminal
                    .winners
                    .iter()
                    .map(|seat| seat.as_str())
                    .collect::<Vec<_>>()
                    .join("/"),
                terminal
                    .standings
                    .iter()
                    .map(|(seat, score, rank, winner)| {
                        format!("{}:{}:{}:{}", seat.as_str(), score, rank, winner)
                    })
                    .collect::<Vec<_>>()
                    .join(","),
                terminal.hands_played
            )
        })
        .unwrap_or_else(|| "none".to_owned())
}
