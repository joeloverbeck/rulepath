use engine_core::{FreshnessToken, SeatId, SeededRng};

use crate::{
    ids::{PlainTricksSeat, TrickCardId, TrickSuit},
    variants::Variant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Phase {
    Playing { round_index: u8, trick_index: u8 },
    Terminal,
}

impl Phase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Playing {
                round_index: 0,
                trick_index: 0,
            } => "round_1_trick_1",
            Self::Playing {
                round_index: 0,
                trick_index: 1,
            } => "round_1_trick_2",
            Self::Playing {
                round_index: 0,
                trick_index: 2,
            } => "round_1_trick_3",
            Self::Playing {
                round_index: 0,
                trick_index: 3,
            } => "round_1_trick_4",
            Self::Playing {
                round_index: 0,
                trick_index: 4,
            } => "round_1_trick_5",
            Self::Playing {
                round_index: 0,
                trick_index: 5,
            } => "round_1_trick_6",
            Self::Playing {
                round_index: 1,
                trick_index: 0,
            } => "round_2_trick_1",
            Self::Playing {
                round_index: 1,
                trick_index: 1,
            } => "round_2_trick_2",
            Self::Playing {
                round_index: 1,
                trick_index: 2,
            } => "round_2_trick_3",
            Self::Playing {
                round_index: 1,
                trick_index: 3,
            } => "round_2_trick_4",
            Self::Playing {
                round_index: 1,
                trick_index: 4,
            } => "round_2_trick_5",
            Self::Playing {
                round_index: 1,
                trick_index: 5,
            } => "round_2_trick_6",
            Self::Playing { .. } => "round_unknown_trick_unknown",
            Self::Terminal => "terminal",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TrickPlay {
    pub seat: PlainTricksSeat,
    pub card: TrickCardId,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CurrentTrick {
    pub led_suit: Option<TrickSuit>,
    pub plays: Vec<TrickPlay>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct TrickCounts {
    pub seat_0: u8,
    pub seat_1: u8,
}

impl TrickCounts {
    pub const fn get(self, seat: PlainTricksSeat) -> u8 {
        match seat {
            PlainTricksSeat::Seat0 => self.seat_0,
            PlainTricksSeat::Seat1 => self.seat_1,
        }
    }

    pub fn increment(&mut self, seat: PlainTricksSeat) {
        match seat {
            PlainTricksSeat::Seat0 => self.seat_0 += 1,
            PlainTricksSeat::Seat1 => self.seat_1 += 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CompletedTrick {
    pub round_index: u8,
    pub trick_index: u8,
    pub leader: PlainTricksSeat,
    pub plays: [TrickPlay; 2],
    pub winner: PlainTricksSeat,
    pub trick_counts_after: TrickCounts,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TerminalOutcome {
    TrickWin {
        winner: PlainTricksSeat,
        totals: TrickCounts,
    },
    Split {
        each: u8,
        totals: TrickCounts,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlainTricksState {
    pub variant: Variant,
    pub seats: [SeatId; 2],
    pub phase: Phase,
    pub active_seat: Option<PlainTricksSeat>,
    pub round_index: u8,
    pub trick_index: u8,
    pub round_leader: PlainTricksSeat,
    pub current_leader: PlainTricksSeat,
    pub(crate) hands: [Vec<TrickCardId>; 2],
    pub(crate) tail: Vec<TrickCardId>,
    pub(crate) rng: SeededRng,
    pub current_trick: CurrentTrick,
    pub round_trick_counts: TrickCounts,
    pub total_trick_counts: TrickCounts,
    pub completed_tricks: Vec<CompletedTrick>,
    pub(crate) effect_history: Vec<String>,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub freshness_token: FreshnessToken,
}

impl PlainTricksState {
    #[allow(dead_code)]
    pub(crate) fn hand_for_internal(&self, seat: PlainTricksSeat) -> &[TrickCardId] {
        &self.hands[seat.index()]
    }

    #[allow(dead_code)]
    pub(crate) fn hands_internal(&self) -> &[Vec<TrickCardId>; 2] {
        &self.hands
    }

    #[allow(dead_code)]
    pub(crate) fn tail_internal(&self) -> &[TrickCardId] {
        &self.tail
    }

    #[allow(dead_code)]
    pub(crate) fn effect_history_internal(&self) -> &[String] {
        &self.effect_history
    }

    pub fn stable_internal_summary(&self) -> String {
        format!(
            "variant={};phase={};active={};round_index={};trick_index={};round_leader={};current_leader={};hands={}|{};tail={};current_trick={};round_counts={},{};total_counts={},{};completed={};effects={};terminal={};freshness={}",
            self.variant.id,
            self.phase.as_str(),
            self.active_seat
                .map(PlainTricksSeat::as_str)
                .unwrap_or("none"),
            self.round_index,
            self.trick_index,
            self.round_leader.as_str(),
            self.current_leader.as_str(),
            stable_cards(&self.hands[0]),
            stable_cards(&self.hands[1]),
            stable_cards(&self.tail),
            stable_current_trick(&self.current_trick),
            self.round_trick_counts.seat_0,
            self.round_trick_counts.seat_1,
            self.total_trick_counts.seat_0,
            self.total_trick_counts.seat_1,
            stable_completed_tricks(&self.completed_tricks),
            stable_effect_history(&self.effect_history),
            stable_terminal(self.terminal_outcome),
            self.freshness_token.0,
        )
    }

    pub(crate) fn new_after_deal(
        variant: Variant,
        seats: [SeatId; 2],
        round_index: u8,
        leader: PlainTricksSeat,
        hands: [Vec<TrickCardId>; 2],
        tail: Vec<TrickCardId>,
        rng: SeededRng,
    ) -> Self {
        Self {
            variant,
            seats,
            phase: Phase::Playing {
                round_index,
                trick_index: 0,
            },
            active_seat: Some(leader),
            round_index,
            trick_index: 0,
            round_leader: leader,
            current_leader: leader,
            hands,
            tail,
            rng,
            current_trick: CurrentTrick::default(),
            round_trick_counts: TrickCounts::default(),
            total_trick_counts: TrickCounts::default(),
            completed_tricks: Vec::new(),
            effect_history: Vec::new(),
            terminal_outcome: None,
            freshness_token: FreshnessToken(0),
        }
    }
}

fn stable_cards(cards: &[TrickCardId]) -> String {
    if cards.is_empty() {
        return "none".to_owned();
    }
    cards
        .iter()
        .map(|card| card.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn stable_current_trick(trick: &CurrentTrick) -> String {
    let led_suit = trick.led_suit.map(TrickSuit::as_str).unwrap_or("none");
    let plays = if trick.plays.is_empty() {
        "none".to_owned()
    } else {
        trick
            .plays
            .iter()
            .map(|play| format!("{}:{}", play.seat.as_str(), play.card.as_str()))
            .collect::<Vec<_>>()
            .join(",")
    };
    format!("led_suit={led_suit}:plays={plays}")
}

fn stable_completed_tricks(tricks: &[CompletedTrick]) -> String {
    if tricks.is_empty() {
        return "none".to_owned();
    }
    tricks
        .iter()
        .map(|trick| {
            format!(
                "r{}t{}:{}:{}-{}:{}-{}",
                trick.round_index,
                trick.trick_index,
                trick.leader.as_str(),
                trick.plays[0].seat.as_str(),
                trick.plays[0].card.as_str(),
                trick.plays[1].seat.as_str(),
                trick.plays[1].card.as_str()
            )
        })
        .collect::<Vec<_>>()
        .join("|")
}

fn stable_effect_history(effects: &[String]) -> String {
    if effects.is_empty() {
        return "none".to_owned();
    }
    effects.join("|")
}

fn stable_terminal(terminal: Option<TerminalOutcome>) -> String {
    match terminal {
        Some(TerminalOutcome::TrickWin { winner, totals }) => format!(
            "trick_win:{}:{}-{}",
            winner.as_str(),
            totals.seat_0,
            totals.seat_1
        ),
        Some(TerminalOutcome::Split { each, totals }) => {
            format!("split:{each}:{}-{}", totals.seat_0, totals.seat_1)
        }
        None => "none".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_summary_keeps_hidden_fields_internal() {
        let state = PlainTricksState::new_after_deal(
            Variant::plain_tricks_standard(),
            [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            0,
            PlainTricksSeat::Seat0,
            [
                vec![TrickCardId::Gale1, TrickCardId::River2],
                vec![TrickCardId::Ember3],
            ],
            vec![TrickCardId::Gale6],
            SeededRng::from_seed(engine_core::Seed(0)),
        );

        assert_eq!(
            state.hand_for_internal(PlainTricksSeat::Seat0),
            &[TrickCardId::Gale1, TrickCardId::River2]
        );
        assert_eq!(state.tail_internal(), &[TrickCardId::Gale6]);
        assert!(state.effect_history_internal().is_empty());
        assert_eq!(
            state.stable_internal_summary(),
            "variant=plain_tricks_standard;phase=round_1_trick_1;active=seat_0;round_index=0;trick_index=0;round_leader=seat_0;current_leader=seat_0;hands=gale_1,river_2|ember_3;tail=gale_6;current_trick=led_suit=none:plays=none;round_counts=0,0;total_counts=0,0;completed=none;effects=none;terminal=none;freshness=0"
        );
    }
}
