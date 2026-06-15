use engine_core::{FreshnessToken, HashValue, StableSerialize, Viewer};

use crate::{
    cards::Card,
    ids::{RiverLedgerSeat, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID},
    state::{Phase, RiverLedgerState, SeatStatus, TerminalOutcome},
    ui::{ui_metadata, UiMetadata},
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
    pub reserved_community_count: u8,
    pub deck_tail_count: u8,
    pub terminal: TerminalView,
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
    },
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
            .map(|entry| SeatView {
                seat: entry.seat,
                status: entry.status,
                street_contribution: entry.street_contribution,
                total_contribution: entry.total_contribution,
                hidden_hole_count: 2,
            })
            .collect(),
        board: state.board.iter().copied().map(card_view).collect(),
        reserved_community_count: state.community_deck_internal().len() as u8,
        deck_tail_count: state.deck_tail_internal().len() as u8,
        terminal: terminal_view(state.terminal_outcome.as_ref()),
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
            "schema={};rules={};game={};variant={};label={};phase={};active={};button={};sb={};bb={};pot={};seats={};board={};reserved={};tail={};terminal={};freshness={};private={};ui={}",
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
            encode_cards(&self.board),
            self.reserved_community_count,
            self.deck_tail_count,
            encode_terminal(&self.terminal),
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
        },
    }
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

fn encode_seats(seats: &[SeatView]) -> String {
    seats
        .iter()
        .map(|seat| {
            format!(
                "{}:{:?}:{}:{}:{}",
                seat.seat.as_str(),
                seat.status,
                seat.street_contribution,
                seat.total_contribution,
                seat.hidden_hole_count
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
        } => format!(
            "showdown:{}:{}:{}:{}",
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
            explanations.join("|")
        ),
    }
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
    use engine_core::{SeatId, Seed, Viewer};

    use super::*;
    use crate::{canonical_deck, setup_match, SetupOptions};

    fn seats(count: usize) -> Vec<SeatId> {
        (0..count)
            .map(|index| SeatId(format!("seat_{index}")))
            .collect()
    }

    #[test]
    fn observer_projection_exposes_counts_but_no_hole_or_deck_cards() {
        let state = setup_match(Seed(13), &seats(4), &SetupOptions::default()).expect("setup");
        let view = project_view(&state, &Viewer { seat_id: None });

        assert!(matches!(view.private_view, PrivateView::Observer));
        assert_eq!(view.board.len(), 0);
        assert_eq!(view.reserved_community_count, 5);
        assert!(view.seats.iter().all(|seat| seat.hidden_hole_count == 2));

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
}
