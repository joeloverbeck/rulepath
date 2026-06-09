use engine_core::{EffectEnvelope, FreshnessToken, StableSerialize, Viewer, VisibilityScope};

use crate::{
    effects::PokerLiteEffect,
    ids::{CrestCardId, PokerLiteSeat, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID},
    rules::showdown_strength,
    state::{Phase, PokerLiteState, ShowdownReveal, TerminalOutcome},
    ui::{card_accessibility_label, ui_metadata, UiMetadata},
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
    pub active_seat: Option<PokerLiteSeat>,
    pub shared_pool: u8,
    pub contributions: [u8; 2],
    pub round: RoundView,
    pub private_counts: [u8; 2],
    pub center: CenterView,
    pub showdown: Option<ShowdownView>,
    pub terminal: TerminalView,
    pub freshness_token: FreshnessToken,
    pub private_view: PrivateView,
    pub ui: UiMetadata,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RoundView {
    pub round_index: u8,
    pub round_unit: u8,
    pub outstanding_actor: Option<PokerLiteSeat>,
    pub outstanding_amount: u8,
    pub lift_cap_remaining: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CenterView {
    Hidden { status: String },
    Revealed(CardView),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardView {
    pub card_id: String,
    pub rank: String,
    pub rank_value: u8,
    pub copy: String,
    pub label: String,
    pub accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownView {
    pub seat_0_private: CardView,
    pub seat_1_private: CardView,
    pub center: CardView,
    pub winner: Option<PokerLiteSeat>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalView {
    NonTerminal,
    YieldWin {
        winner: PokerLiteSeat,
        loser: PokerLiteSeat,
        shared_pool: u8,
    },
    ShowdownWin {
        winner: PokerLiteSeat,
        shared_pool: u8,
    },
    Split {
        shared_pool: u8,
        each: u8,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrivateView {
    Observer,
    Seat(SeatPrivateView),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatPrivateView {
    pub seat: PokerLiteSeat,
    pub own_private: Option<CardView>,
    pub own_strength_bucket: Option<String>,
}

pub fn project_view(state: &PokerLiteState, viewer: &Viewer) -> PublicView {
    let viewer_seat = viewer_seat(state, viewer);
    let showdown = showdown_view(state.terminal_outcome);

    PublicView {
        schema_version: 1,
        rules_version: 1,
        game_id: GAME_ID.to_owned(),
        display_name: "Crest Ledger".to_owned(),
        variant_id: VARIANT_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        phase: state.phase,
        active_seat: state.active_seat,
        shared_pool: state.shared_pool,
        contributions: state.contributions,
        round: RoundView {
            round_index: state.round.round_index,
            round_unit: state.round.unit,
            outstanding_actor: state.round.outstanding_actor,
            outstanding_amount: state.round.outstanding_amount,
            lift_cap_remaining: if state.round.lift_used { 0 } else { 1 },
        },
        private_counts: [1, 1],
        center: center_view(state),
        showdown,
        terminal: terminal_view(state.terminal_outcome),
        freshness_token: state.freshness_token,
        private_view: private_view(state, viewer_seat),
        ui: ui_metadata(),
    }
}

pub fn filter_effects_for_viewer(
    effects: &[EffectEnvelope<PokerLiteEffect>],
    viewer: &Viewer,
) -> Vec<EffectEnvelope<PokerLiteEffect>> {
    effects
        .iter()
        .filter(|effect| match &effect.visibility {
            VisibilityScope::Public => true,
            VisibilityScope::PrivateToSeat(seat_id) => viewer.seat_id.as_ref() == Some(seat_id),
        })
        .cloned()
        .collect()
}

impl PublicView {
    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};rules={};game={};variant={};label={};phase={};active={};pool={};contrib={},{};round={}:unit{}:outstanding={}:amount{}:lift{};private_counts={},{};center={};showdown={};terminal={};freshness={};private={};ui={}",
            self.schema_version,
            self.rules_version,
            self.game_id,
            self.variant_id,
            self.rules_version_label,
            self.phase.as_str(),
            seat_option(self.active_seat),
            self.shared_pool,
            self.contributions[0],
            self.contributions[1],
            self.round.round_index,
            self.round.round_unit,
            seat_option(self.round.outstanding_actor),
            self.round.outstanding_amount,
            self.round.lift_cap_remaining,
            self.private_counts[0],
            self.private_counts[1],
            encode_center(&self.center),
            self.showdown.as_ref().map_or_else(|| "none".to_owned(), encode_showdown),
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

fn viewer_seat(state: &PokerLiteState, viewer: &Viewer) -> Option<PokerLiteSeat> {
    viewer
        .seat_id
        .as_ref()
        .and_then(|seat_id| {
            state
                .seats
                .iter()
                .position(|candidate| candidate == seat_id)
        })
        .and_then(PokerLiteSeat::from_index)
}

fn center_view(state: &PokerLiteState) -> CenterView {
    if state.center_visible {
        CenterView::Revealed(card_view(state.center_card_internal()))
    } else {
        CenterView::Hidden {
            status: "hidden".to_owned(),
        }
    }
}

fn private_view(state: &PokerLiteState, viewer_seat: Option<PokerLiteSeat>) -> PrivateView {
    match viewer_seat {
        Some(seat) => {
            let own_card = state.private_card_for_internal(seat);
            PrivateView::Seat(SeatPrivateView {
                seat,
                own_private: Some(card_view(own_card)),
                own_strength_bucket: Some(strength_bucket(state, own_card)),
            })
        }
        None => PrivateView::Observer,
    }
}

fn showdown_view(outcome: Option<TerminalOutcome>) -> Option<ShowdownView> {
    let reveal = match outcome {
        Some(TerminalOutcome::ShowdownWin { reveal, .. })
        | Some(TerminalOutcome::Split { reveal, .. }) => reveal,
        Some(TerminalOutcome::YieldWin { .. }) | None => return None,
    };
    Some(ShowdownView {
        seat_0_private: card_view(reveal.seat_0_private),
        seat_1_private: card_view(reveal.seat_1_private),
        center: card_view(reveal.center),
        winner: winner_from_reveal(reveal),
    })
}

fn terminal_view(outcome: Option<TerminalOutcome>) -> TerminalView {
    match outcome {
        None => TerminalView::NonTerminal,
        Some(TerminalOutcome::YieldWin {
            winner,
            loser,
            shared_pool,
            ..
        }) => TerminalView::YieldWin {
            winner,
            loser,
            shared_pool,
        },
        Some(TerminalOutcome::ShowdownWin {
            winner,
            shared_pool,
            ..
        }) => TerminalView::ShowdownWin {
            winner,
            shared_pool,
        },
        Some(TerminalOutcome::Split {
            shared_pool, each, ..
        }) => TerminalView::Split { shared_pool, each },
    }
}

fn strength_bucket(state: &PokerLiteState, own_card: CrestCardId) -> String {
    if state.center_visible {
        let strength = showdown_strength(own_card, state.center_card_internal());
        let prefix = if strength.pair_flag {
            "paired"
        } else {
            "unpaired"
        };
        format!("{prefix}_{}", own_card.rank().as_str())
    } else {
        format!("{}_private", own_card.rank().as_str())
    }
}

fn winner_from_reveal(reveal: ShowdownReveal) -> Option<PokerLiteSeat> {
    let seat_0 = showdown_strength(reveal.seat_0_private, reveal.center);
    let seat_1 = showdown_strength(reveal.seat_1_private, reveal.center);
    if seat_0 > seat_1 {
        Some(PokerLiteSeat::Seat0)
    } else if seat_1 > seat_0 {
        Some(PokerLiteSeat::Seat1)
    } else {
        None
    }
}

fn card_view(card: CrestCardId) -> CardView {
    CardView {
        card_id: card.as_str().to_owned(),
        rank: card.rank().as_str().to_owned(),
        rank_value: card.rank().value(),
        copy: card.rank_copy().as_str().to_owned(),
        label: card.label(),
        accessibility_label: card_accessibility_label(card),
    }
}

fn seat_option(seat: Option<PokerLiteSeat>) -> &'static str {
    seat.map_or("none", PokerLiteSeat::as_str)
}

fn encode_center(center: &CenterView) -> String {
    match center {
        CenterView::Hidden { status } => status.clone(),
        CenterView::Revealed(card) => encode_card(card),
    }
}

fn encode_showdown(showdown: &ShowdownView) -> String {
    format!(
        "{}:{}:{}:{}",
        encode_card(&showdown.seat_0_private),
        encode_card(&showdown.seat_1_private),
        encode_card(&showdown.center),
        seat_option(showdown.winner)
    )
}

fn encode_card(card: &CardView) -> String {
    format!(
        "{}:{}:{}:{}:{}",
        card.card_id, card.rank, card.rank_value, card.copy, card.label
    )
}

fn encode_terminal(terminal: &TerminalView) -> String {
    match terminal {
        TerminalView::NonTerminal => "non_terminal".to_owned(),
        TerminalView::YieldWin {
            winner,
            loser,
            shared_pool,
        } => format!(
            "yield:{}:{}:{}",
            winner.as_str(),
            loser.as_str(),
            shared_pool
        ),
        TerminalView::ShowdownWin {
            winner,
            shared_pool,
        } => format!("showdown:{}:{}", winner.as_str(), shared_pool),
        TerminalView::Split { shared_pool, each } => format!("split:{shared_pool}:{each}"),
    }
}

fn encode_private(private: &PrivateView) -> String {
    match private {
        PrivateView::Observer => "observer".to_owned(),
        PrivateView::Seat(view) => format!(
            "{}:{}:{}",
            view.seat.as_str(),
            view.own_private
                .as_ref()
                .map_or_else(|| "none".to_owned(), encode_card),
            view.own_strength_bucket.as_deref().unwrap_or("none")
        ),
    }
}

fn encode_ui(ui: &UiMetadata) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        ui.game_id,
        ui.display_name,
        ui.surface_label,
        ui.shared_pool_label,
        ui.hidden_center_label,
        ui.hidden_private_label,
        ui.hold_label,
        ui.press_label,
        ui.lift_label,
        ui.match_label,
        ui.yield_label,
        ui.reduced_motion_note
    )
}
