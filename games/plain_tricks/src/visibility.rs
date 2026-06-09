use engine_core::{EffectEnvelope, FreshnessToken, StableSerialize, Viewer, VisibilityScope};

use crate::{
    effects::PlainTricksEffect,
    ids::{PlainTricksSeat, TrickCardId, TrickSuit, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID},
    state::{CompletedTrick, Phase, PlainTricksState, TerminalOutcome, TrickCounts, TrickPlay},
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
    pub active_seat: Option<PlainTricksSeat>,
    pub round_index: u8,
    pub trick_index: u8,
    pub round_leader: PlainTricksSeat,
    pub current_leader: PlainTricksSeat,
    pub hand_counts: HandCountsView,
    pub current_trick: CurrentTrickView,
    pub trick_history: Vec<CompletedTrickView>,
    pub round_trick_counts: TrickCounts,
    pub total_trick_counts: TrickCounts,
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
pub struct CurrentTrickView {
    pub led_suit: Option<String>,
    pub plays: Vec<PlayedCardView>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayedCardView {
    pub seat: PlainTricksSeat,
    pub card: CardView,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletedTrickView {
    pub round_index: u8,
    pub trick_index: u8,
    pub leader: PlainTricksSeat,
    pub plays: [PlayedCardView; 2],
    pub winner: PlainTricksSeat,
    pub trick_counts_after: TrickCounts,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardView {
    pub card_id: String,
    pub suit: String,
    pub rank: String,
    pub rank_value: u8,
    pub label: String,
    pub accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalView {
    NonTerminal,
    TrickWin {
        winner: PlainTricksSeat,
        totals: TrickCounts,
        rationale: OutcomeRationaleView,
    },
    Split {
        each: u8,
        totals: TrickCounts,
        rationale: OutcomeRationaleView,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutcomeRationaleView {
    pub result_kind: String,
    pub decisive_cause: String,
    pub template_key: String,
    pub decisive_rule_ids: Vec<String>,
    pub per_seat: [SeatOutcomeBreakdownView; 2],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatOutcomeBreakdownView {
    pub seat: PlainTricksSeat,
    pub total_tricks: u8,
    pub result: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrivateView {
    Observer,
    Seat(SeatPrivateView),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatPrivateView {
    pub seat: PlainTricksSeat,
    pub own_hand: Vec<CardView>,
}

pub fn project_view(state: &PlainTricksState, viewer: &Viewer) -> PublicView {
    let viewer_seat = viewer_seat(state, viewer);
    PublicView {
        schema_version: 1,
        rules_version: 1,
        game_id: GAME_ID.to_owned(),
        display_name: "Plain Tricks".to_owned(),
        variant_id: VARIANT_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        phase: state.phase,
        active_seat: state.active_seat,
        round_index: state.round_index,
        trick_index: state.trick_index,
        round_leader: state.round_leader,
        current_leader: state.current_leader,
        hand_counts: HandCountsView {
            seat_0: state.hands[PlainTricksSeat::Seat0.index()].len() as u8,
            seat_1: state.hands[PlainTricksSeat::Seat1.index()].len() as u8,
        },
        current_trick: current_trick_view(state.current_trick.led_suit, &state.current_trick.plays),
        trick_history: state
            .completed_tricks
            .iter()
            .copied()
            .map(completed_trick_view)
            .collect(),
        round_trick_counts: state.round_trick_counts,
        total_trick_counts: state.total_trick_counts,
        terminal: terminal_view(state.terminal_outcome),
        freshness_token: state.freshness_token,
        private_view: private_view(state, viewer_seat),
        ui: ui_metadata(),
    }
}

pub fn filter_effects_for_viewer(
    effects: &[EffectEnvelope<PlainTricksEffect>],
    viewer: &Viewer,
) -> Vec<EffectEnvelope<PlainTricksEffect>> {
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
            "schema={};rules={};game={};variant={};label={};phase={};active={};round={};trick={};round_leader={};current_leader={};hands={},{};current={};history={};round_counts={},{};total_counts={},{};terminal={};freshness={};private={};ui={}",
            self.schema_version,
            self.rules_version,
            self.game_id,
            self.variant_id,
            self.rules_version_label,
            self.phase.as_str(),
            seat_option(self.active_seat),
            self.round_index,
            self.trick_index,
            self.round_leader.as_str(),
            self.current_leader.as_str(),
            self.hand_counts.seat_0,
            self.hand_counts.seat_1,
            encode_current_trick(&self.current_trick),
            self.trick_history
                .iter()
                .map(encode_completed_trick)
                .collect::<Vec<_>>()
                .join("|"),
            self.round_trick_counts.seat_0,
            self.round_trick_counts.seat_1,
            self.total_trick_counts.seat_0,
            self.total_trick_counts.seat_1,
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

fn viewer_seat(state: &PlainTricksState, viewer: &Viewer) -> Option<PlainTricksSeat> {
    viewer
        .seat_id
        .as_ref()
        .and_then(|seat_id| {
            state
                .seats
                .iter()
                .position(|candidate| candidate == seat_id)
        })
        .and_then(PlainTricksSeat::from_index)
}

fn private_view(state: &PlainTricksState, viewer_seat: Option<PlainTricksSeat>) -> PrivateView {
    match viewer_seat {
        Some(seat) => PrivateView::Seat(SeatPrivateView {
            seat,
            own_hand: state.hands[seat.index()]
                .iter()
                .copied()
                .map(card_view)
                .collect(),
        }),
        None => PrivateView::Observer,
    }
}

fn current_trick_view(led_suit: Option<TrickSuit>, plays: &[TrickPlay]) -> CurrentTrickView {
    CurrentTrickView {
        led_suit: led_suit.map(|suit| suit.as_str().to_owned()),
        plays: plays
            .iter()
            .copied()
            .map(|play| PlayedCardView {
                seat: play.seat,
                card: card_view(play.card),
            })
            .collect(),
    }
}

fn completed_trick_view(trick: CompletedTrick) -> CompletedTrickView {
    CompletedTrickView {
        round_index: trick.round_index,
        trick_index: trick.trick_index,
        leader: trick.leader,
        plays: [
            PlayedCardView {
                seat: trick.plays[0].seat,
                card: card_view(trick.plays[0].card),
            },
            PlayedCardView {
                seat: trick.plays[1].seat,
                card: card_view(trick.plays[1].card),
            },
        ],
        winner: trick.winner,
        trick_counts_after: trick.trick_counts_after,
    }
}

pub fn card_view(card: TrickCardId) -> CardView {
    CardView {
        card_id: card.as_str().to_owned(),
        suit: card.suit().as_str().to_owned(),
        rank: card.rank().as_str().to_owned(),
        rank_value: card.rank().value(),
        label: card.label(),
        accessibility_label: card_accessibility_label(card),
    }
}

fn terminal_view(outcome: Option<TerminalOutcome>) -> TerminalView {
    match outcome {
        None => TerminalView::NonTerminal,
        Some(TerminalOutcome::TrickWin { winner, totals }) => TerminalView::TrickWin {
            winner,
            totals,
            rationale: outcome_rationale("trick_win", Some(winner), totals),
        },
        Some(TerminalOutcome::Split { each, totals }) => TerminalView::Split {
            each,
            totals,
            rationale: outcome_rationale("split", None, totals),
        },
    }
}

fn outcome_rationale(
    result_kind: &str,
    winner: Option<PlainTricksSeat>,
    totals: TrickCounts,
) -> OutcomeRationaleView {
    OutcomeRationaleView {
        result_kind: result_kind.to_owned(),
        decisive_cause: decisive_cause(totals),
        template_key: format!("plain_tricks.{result_kind}"),
        decisive_rule_ids: rule_ids(&["PT-SCORE-002", "PT-END-001", "PT-END-002"]),
        per_seat: [
            outcome_breakdown(PlainTricksSeat::Seat0, winner, totals),
            outcome_breakdown(PlainTricksSeat::Seat1, winner, totals),
        ],
    }
}

fn outcome_breakdown(
    seat: PlainTricksSeat,
    winner: Option<PlainTricksSeat>,
    totals: TrickCounts,
) -> SeatOutcomeBreakdownView {
    let result = match winner {
        Some(winner) if winner == seat => "win",
        Some(_) => "loss",
        None => "split",
    };
    SeatOutcomeBreakdownView {
        seat,
        total_tricks: totals.get(seat),
        result: result.to_owned(),
    }
}

fn decisive_cause(totals: TrickCounts) -> String {
    if totals.seat_0 == totals.seat_1 {
        format!("split:{}-{}", totals.seat_0, totals.seat_1)
    } else if totals.seat_0 > totals.seat_1 {
        format!("seat_0_total_tricks:{}-{}", totals.seat_0, totals.seat_1)
    } else {
        format!("seat_1_total_tricks:{}-{}", totals.seat_0, totals.seat_1)
    }
}

fn rule_ids(ids: &[&str]) -> Vec<String> {
    ids.iter().map(|id| (*id).to_owned()).collect()
}

fn seat_option(seat: Option<PlainTricksSeat>) -> &'static str {
    seat.map(PlainTricksSeat::as_str).unwrap_or("none")
}

fn encode_current_trick(trick: &CurrentTrickView) -> String {
    let plays = if trick.plays.is_empty() {
        "none".to_owned()
    } else {
        trick
            .plays
            .iter()
            .map(encode_played_card)
            .collect::<Vec<_>>()
            .join(",")
    };
    format!(
        "led_suit={}:plays={plays}",
        trick.led_suit.as_deref().unwrap_or("none")
    )
}

fn encode_completed_trick(trick: &CompletedTrickView) -> String {
    format!(
        "r{}t{}:leader={}:plays={};{}:winner={}:counts={}-{}",
        trick.round_index,
        trick.trick_index,
        trick.leader.as_str(),
        encode_played_card(&trick.plays[0]),
        encode_played_card(&trick.plays[1]),
        trick.winner.as_str(),
        trick.trick_counts_after.seat_0,
        trick.trick_counts_after.seat_1
    )
}

fn encode_played_card(play: &PlayedCardView) -> String {
    format!("{}:{}", play.seat.as_str(), play.card.card_id)
}

fn encode_card(card: &CardView) -> String {
    format!("{}:{}:{}", card.card_id, card.suit, card.rank)
}

fn encode_terminal(terminal: &TerminalView) -> String {
    match terminal {
        TerminalView::NonTerminal => "none".to_owned(),
        TerminalView::TrickWin {
            winner,
            totals,
            rationale,
        } => format!(
            "trick_win:{}:{}-{}:{}",
            winner.as_str(),
            totals.seat_0,
            totals.seat_1,
            rationale.decisive_cause
        ),
        TerminalView::Split {
            each,
            totals,
            rationale,
        } => format!(
            "split:{each}:{}-{}:{}",
            totals.seat_0, totals.seat_1, rationale.decisive_cause
        ),
    }
}

fn encode_private(private: &PrivateView) -> String {
    match private {
        PrivateView::Observer => "observer".to_owned(),
        PrivateView::Seat(seat) => format!(
            "seat:{}:hand={}",
            seat.seat.as_str(),
            seat.own_hand
                .iter()
                .map(encode_card)
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

fn encode_ui(ui: &UiMetadata) -> String {
    format!(
        "{}:{}:{}:{}",
        ui.game_id, ui.display_name, ui.table_label, ui.play_action_label
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{setup_match, SetupOptions};
    use engine_core::{SeatId, Seed};

    #[test]
    fn observer_view_has_counts_but_no_private_hand_cards() {
        let state = setup_match(
            Seed(2),
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds");
        let view = project_view(&state, &Viewer { seat_id: None });
        let text = format!("{view:?}");

        assert_eq!(view.hand_counts.seat_0, 6);
        assert_eq!(view.hand_counts.seat_1, 6);
        assert!(matches!(view.private_view, PrivateView::Observer));
        for card in TrickCardId::ALL {
            assert!(!text.contains(card.as_str()));
            assert!(!text.contains(&card.label()));
        }
    }

    #[test]
    fn seat_view_contains_only_own_hand() {
        let state = setup_match(
            Seed(2),
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds");
        let view = project_view(
            &state,
            &Viewer {
                seat_id: Some(SeatId("seat_0".to_owned())),
            },
        );

        let PrivateView::Seat(private) = &view.private_view else {
            panic!("seat viewer gets private view");
        };
        assert_eq!(private.own_hand.len(), 6);

        let text = format!("{view:?}");
        let own_ids = private
            .own_hand
            .iter()
            .map(|card| card.card_id.as_str())
            .collect::<Vec<_>>();
        for card in TrickCardId::ALL {
            if own_ids.contains(&card.as_str()) {
                assert!(text.contains(card.as_str()));
            } else {
                assert!(!text.contains(card.as_str()));
            }
        }
    }
}
