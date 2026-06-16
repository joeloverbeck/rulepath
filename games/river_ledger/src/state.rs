use engine_core::{FreshnessToken, SeatId};

use crate::{
    cards::Card,
    ids::{
        RiverLedgerSeat, MAX_RAISES_PER_STREET, STANDARD_BIG_BET_UNIT, STANDARD_BIG_BLIND,
        STANDARD_SMALL_BET_UNIT, STANDARD_SMALL_BLIND,
    },
    variants::Variant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}

impl Street {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preflop => "preflop",
            Self::Flop => "flop",
            Self::Turn => "turn",
            Self::River => "river",
        }
    }

    pub const fn unit(self) -> u8 {
        match self {
            Self::Preflop | Self::Flop => STANDARD_SMALL_BET_UNIT,
            Self::Turn | Self::River => STANDARD_BIG_BET_UNIT,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Phase {
    Setup,
    Betting { street: Street },
    Showdown,
    Terminal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SeatStatus {
    Live,
    Folded,
    ShowdownEligible,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatLedger {
    pub seat: RiverLedgerSeat,
    pub status: SeatStatus,
    pub street_contribution: u16,
    pub total_contribution: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContributionLedger {
    pub seats: Vec<SeatLedger>,
    pub pot_total: u16,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct BettingRoundState {
    pub street: Street,
    pub current_to_call: u16,
    pub raises_this_street: u8,
    pub last_aggressor: Option<RiverLedgerSeat>,
    pub actors_to_respond: Vec<RiverLedgerSeat>,
}

impl BettingRoundState {
    pub fn for_street(street: Street, actors_to_respond: Vec<RiverLedgerSeat>) -> Self {
        Self {
            street,
            current_to_call: 0,
            raises_this_street: 0,
            last_aggressor: None,
            actors_to_respond,
        }
    }

    pub const fn raise_cap_reached(&self) -> bool {
        self.raises_this_street >= MAX_RAISES_PER_STREET
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownReveal {
    pub seat: RiverLedgerSeat,
    pub hole_cards: [Card; 2],
    pub best_five: [Card; 5],
    pub category: String,
    pub tie_break_vector: Vec<u8>,
    pub category_ladder_position: CategoryLadderPosition,
    pub result_label: String,
    pub hand_name: String,
    pub rank_explanation: String,
    pub comparison_note: String,
    pub best_five_accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CategoryLadderPosition {
    pub position: u8,
    pub total: u8,
    pub description: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownSeatExplanation {
    pub seat: RiverLedgerSeat,
    pub status: SeatStatus,
    pub revealed: Option<ShowdownReveal>,
    pub summary: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RiverLedgerShowdownPresentationV2 {
    pub result_banner: ShowdownResultBanner,
    pub decisive_reason: ShowdownDecisiveReason,
    pub board_cards: Vec<ShowdownBoardCardPresentation>,
    pub standings: Vec<ShowdownStandingPresentation>,
    pub folded_rows: Vec<ShowdownFoldedRowPresentation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownResultBanner {
    pub headline: String,
    pub subheadline: String,
    pub accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownDecisiveReason {
    pub short_text: String,
    pub contrast_seat: Option<RiverLedgerSeat>,
    pub contrast_seat_label: Option<String>,
    pub rule_refs: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownBoardCardPresentation {
    pub slot: String,
    pub card: Card,
    pub public_label: String,
    pub used_by_selected: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownStandingPresentation {
    pub seat: RiverLedgerSeat,
    pub seat_label: String,
    pub rank: u8,
    pub result_label: String,
    pub allocation_label: String,
    pub hand_name: String,
    pub short_comparison_note: String,
    pub rank_ladder_label: String,
    pub hole_cards: Vec<ShowdownCardUsageMark>,
    pub board_cards: Vec<ShowdownCardUsageMark>,
    pub best_five: Vec<Card>,
    pub best_five_accessibility_label: String,
    pub detail_rows: Vec<ShowdownDetailRow>,
    pub default_expanded: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownCardUsageMark {
    pub card: Card,
    pub public_label: String,
    pub used_in_best_five: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownDetailRow {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownFoldedRowPresentation {
    pub seat: RiverLedgerSeat,
    pub seat_label: String,
    pub redaction_label: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PotShare {
    pub seat: RiverLedgerSeat,
    pub amount: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalOutcome {
    LastLiveHand {
        winner: RiverLedgerSeat,
        pot_total: u16,
    },
    Showdown {
        winners: Vec<RiverLedgerSeat>,
        pot_total: u16,
        allocations: Vec<PotShare>,
        headline: String,
        decisive_comparison: String,
        comparison_basis: String,
        explanations: Vec<ShowdownSeatExplanation>,
        presentation_v2: Box<RiverLedgerShowdownPresentationV2>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RiverLedgerState {
    pub variant: Variant,
    pub seats: Vec<SeatId>,
    pub phase: Phase,
    pub button: RiverLedgerSeat,
    pub small_blind: RiverLedgerSeat,
    pub big_blind: RiverLedgerSeat,
    pub active_seat: Option<RiverLedgerSeat>,
    pub board: Vec<Card>,
    private_hands: Vec<[Card; 2]>,
    community_deck: [Card; 5],
    deck_tail: Vec<Card>,
    pub ledger: ContributionLedger,
    pub betting: BettingRoundState,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub freshness_token: FreshnessToken,
}

/// Seat role assignments fixed at setup time (button and the seats derived from
/// it). Grouped so the setup constructor stays within a readable arity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SeatRoles {
    pub button: RiverLedgerSeat,
    pub small_blind: RiverLedgerSeat,
    pub big_blind: RiverLedgerSeat,
    pub active_seat: RiverLedgerSeat,
}

impl RiverLedgerState {
    pub fn new_after_setup(
        variant: Variant,
        seats: Vec<SeatId>,
        roles: SeatRoles,
        private_hands: Vec<[Card; 2]>,
        community_deck: [Card; 5],
        deck_tail: Vec<Card>,
    ) -> Self {
        let SeatRoles {
            button,
            small_blind,
            big_blind,
            active_seat,
        } = roles;
        let seat_count = seats.len() as u8;
        let mut ledgers = Vec::with_capacity(seats.len());
        for index in 0..seats.len() {
            let seat = RiverLedgerSeat::from_index(index).expect("setup creates valid seats");
            let total_contribution = if seat == small_blind {
                u16::from(STANDARD_SMALL_BLIND)
            } else if seat == big_blind {
                u16::from(STANDARD_BIG_BLIND)
            } else {
                0
            };
            ledgers.push(SeatLedger {
                seat,
                status: SeatStatus::Live,
                street_contribution: total_contribution,
                total_contribution,
            });
        }

        let pot_total = u16::from(STANDARD_SMALL_BLIND + STANDARD_BIG_BLIND);

        Self {
            variant,
            seats,
            phase: Phase::Betting {
                street: Street::Preflop,
            },
            button,
            small_blind,
            big_blind,
            active_seat: Some(active_seat),
            board: Vec::new(),
            private_hands,
            community_deck,
            deck_tail,
            ledger: ContributionLedger {
                seats: ledgers,
                pot_total,
            },
            betting: BettingRoundState {
                street: Street::Preflop,
                current_to_call: u16::from(STANDARD_BIG_BLIND),
                raises_this_street: 0,
                last_aggressor: Some(big_blind),
                actors_to_respond: response_order_after(big_blind, seat_count),
            },
            terminal_outcome: None,
            freshness_token: FreshnessToken(0),
        }
    }

    pub fn private_hand_for_internal(&self, seat: RiverLedgerSeat) -> Option<[Card; 2]> {
        self.private_hands.get(seat.index()).copied()
    }

    pub fn private_hands_internal(&self) -> &[[Card; 2]] {
        &self.private_hands
    }

    pub fn community_deck_internal(&self) -> &[Card; 5] {
        &self.community_deck
    }

    pub(crate) fn reveal_next_board_cards(&mut self, count: usize) {
        let start = self.board.len();
        let end = start.saturating_add(count).min(self.community_deck.len());
        self.board
            .extend_from_slice(&self.community_deck[start..end]);
    }

    pub fn deck_tail_internal(&self) -> &[Card] {
        &self.deck_tail
    }

    pub fn stable_internal_summary(&self) -> String {
        format!(
            "variant={};seats={};phase={};button={};sb={};bb={};active={};private={};community={};tail={};pot={};contributions={};freshness={}",
            self.variant.id,
            self.seats.len(),
            stable_phase(self.phase),
            self.button.as_str(),
            self.small_blind.as_str(),
            self.big_blind.as_str(),
            self.active_seat
                .map(RiverLedgerSeat::as_str)
                .unwrap_or_else(|| "none".to_owned()),
            stable_private_hands(&self.private_hands),
            stable_cards(&self.community_deck),
            stable_cards(&self.deck_tail),
            self.ledger.pot_total,
            stable_contributions(&self.ledger.seats),
            self.freshness_token.0,
        )
    }

    pub fn setup_public_summary(&self) -> String {
        let hidden_counts = self
            .private_hands
            .iter()
            .enumerate()
            .map(|(index, hand)| format!("seat_{index}:{} hidden", hand.len()))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "variant={};seats={};phase={};button={};sb={};bb={};active={};board_visible={};hole_counts={};reserved_community_count={};deck_tail_count={};pot={};contributions={}",
            self.variant.id,
            self.seats.len(),
            stable_phase(self.phase),
            self.button.as_str(),
            self.small_blind.as_str(),
            self.big_blind.as_str(),
            self.active_seat
                .map(RiverLedgerSeat::as_str)
                .unwrap_or_else(|| "none".to_owned()),
            self.board.len(),
            hidden_counts,
            self.community_deck.len(),
            self.deck_tail.len(),
            self.ledger.pot_total,
            stable_contributions(&self.ledger.seats),
        )
    }
}

pub(crate) fn response_order_after(start: RiverLedgerSeat, count: u8) -> Vec<RiverLedgerSeat> {
    let mut order = Vec::with_capacity(count as usize);
    let mut current = start;
    for _ in 0..count {
        current = current
            .next_in_count(count)
            .expect("response order uses valid count");
        order.push(current);
    }
    order
}

fn stable_phase(phase: Phase) -> &'static str {
    match phase {
        Phase::Setup => "setup",
        Phase::Betting { street } => street.as_str(),
        Phase::Showdown => "showdown",
        Phase::Terminal => "terminal",
    }
}

fn stable_private_hands(hands: &[[Card; 2]]) -> String {
    hands
        .iter()
        .map(|hand| stable_cards(hand))
        .collect::<Vec<_>>()
        .join("|")
}

fn stable_cards(cards: &[Card]) -> String {
    cards
        .iter()
        .map(|card| card.id())
        .collect::<Vec<_>>()
        .join(",")
}

fn stable_contributions(seats: &[SeatLedger]) -> String {
    seats
        .iter()
        .map(|seat| {
            format!(
                "{}:{}:{}",
                seat.seat.as_str(),
                seat.street_contribution,
                seat.total_contribution
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}
