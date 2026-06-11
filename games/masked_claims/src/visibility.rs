use engine_core::{EffectEnvelope, FreshnessToken, StableSerialize, Viewer, VisibilityScope};

use crate::{
    effects::MaskedClaimsEffect,
    ids::{Grade, MaskTileId, MaskedClaimsSeat, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID},
    state::{ChallengeCounters, ExposedMask, MaskedClaimsState, Phase, TerminalOutcome},
    ui::{grade_accessibility_label, ui_metadata, UiMetadata},
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
    pub active_seat: Option<MaskedClaimsSeat>,
    pub turn_index: u8,
    pub claimant: MaskedClaimsSeat,
    pub hand_counts: HandCountsView,
    pub pedestal: Option<PedestalView>,
    pub veiled_gallery: [Vec<VeiledClaimView>; 2],
    pub exposed_rows: [Vec<ExposedMaskView>; 2],
    pub scores: [u8; 2],
    pub counters: [CounterView; 2],
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PedestalView {
    pub claimant: MaskedClaimsSeat,
    pub declared_grade: Grade,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VeiledClaimView {
    pub declared_grade: Grade,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExposedMaskView {
    pub tile_id: String,
    pub actual_grade: Grade,
    pub declared_grade: Grade,
    pub claimant: MaskedClaimsSeat,
    pub challenger: MaskedClaimsSeat,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CounterView {
    pub exposed_lies: u8,
    pub successful_challenges: u8,
    pub challenges_declared: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalView {
    NonTerminal,
    Complete {
        outcome: TerminalOutcome,
        rationale: OutcomeRationaleView,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutcomeRationaleView {
    pub result_kind: String,
    pub decisive_cause: String,
    pub decisive_rule_ids: Vec<String>,
    pub final_scores: [u8; 2],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrivateView {
    Observer,
    Seat(SeatPrivateView),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatPrivateView {
    pub seat: MaskedClaimsSeat,
    pub own_hand: Vec<MaskView>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskView {
    pub tile_id: String,
    pub grade: Grade,
    pub label: String,
    pub accessibility_label: String,
}

pub fn project_view(state: &MaskedClaimsState, viewer: &Viewer) -> PublicView {
    let viewer_seat = viewer_seat(state, viewer);
    PublicView {
        schema_version: 1,
        rules_version: 1,
        game_id: GAME_ID.to_owned(),
        display_name: "Masked Claims".to_owned(),
        variant_id: VARIANT_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        phase: state.phase,
        active_seat: state.active_seat,
        turn_index: state.turn_index,
        claimant: state.claimant,
        hand_counts: HandCountsView {
            seat_0: state.hands[MaskedClaimsSeat::Seat0.index()].len() as u8,
            seat_1: state.hands[MaskedClaimsSeat::Seat1.index()].len() as u8,
        },
        pedestal: state.pedestal.map(|claim| PedestalView {
            claimant: claim.claimant,
            declared_grade: claim.declared,
        }),
        veiled_gallery: [
            state.veiled_gallery[0]
                .iter()
                .map(|claim| VeiledClaimView {
                    declared_grade: claim.declared,
                })
                .collect(),
            state.veiled_gallery[1]
                .iter()
                .map(|claim| VeiledClaimView {
                    declared_grade: claim.declared,
                })
                .collect(),
        ],
        exposed_rows: [
            state.exposed_row[0]
                .iter()
                .copied()
                .map(exposed_view)
                .collect(),
            state.exposed_row[1]
                .iter()
                .copied()
                .map(exposed_view)
                .collect(),
        ],
        scores: state.scores,
        counters: [
            counter_view(state.counters[0]),
            counter_view(state.counters[1]),
        ],
        terminal: terminal_view(state.terminal_outcome),
        freshness_token: state.freshness_token,
        private_view: private_view(state, viewer_seat),
        ui: ui_metadata(),
    }
}

pub fn filter_effects_for_viewer(
    effects: &[EffectEnvelope<MaskedClaimsEffect>],
    viewer: &Viewer,
) -> Vec<EffectEnvelope<MaskedClaimsEffect>> {
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
            "schema={};rules={};game={};variant={};label={};phase={};active={};turn={};claimant={};hands={},{};pedestal={};veiled={}|{};exposed={}|{};scores={},{};counters={}|{};terminal={};freshness={};private={};ui={}",
            self.schema_version,
            self.rules_version,
            self.game_id,
            self.variant_id,
            self.rules_version_label,
            self.phase.as_str(),
            seat_option(self.active_seat),
            self.turn_index,
            self.claimant.as_str(),
            self.hand_counts.seat_0,
            self.hand_counts.seat_1,
            encode_pedestal(self.pedestal),
            encode_veiled(&self.veiled_gallery[0]),
            encode_veiled(&self.veiled_gallery[1]),
            encode_exposed(&self.exposed_rows[0]),
            encode_exposed(&self.exposed_rows[1]),
            self.scores[0],
            self.scores[1],
            encode_counter(self.counters[0]),
            encode_counter(self.counters[1]),
            encode_terminal(&self.terminal),
            self.freshness_token.0,
            encode_private(&self.private_view),
            self.ui.grade_labels.join("|"),
        )
    }
}

impl StableSerialize for PublicView {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

pub fn mask_view(tile: MaskTileId) -> MaskView {
    MaskView {
        tile_id: tile.as_str().to_owned(),
        grade: tile.grade(),
        label: tile.label(),
        accessibility_label: grade_accessibility_label(tile.grade()),
    }
}

fn viewer_seat(state: &MaskedClaimsState, viewer: &Viewer) -> Option<MaskedClaimsSeat> {
    viewer
        .seat_id
        .as_ref()
        .and_then(|seat_id| {
            state
                .seats
                .iter()
                .position(|candidate| candidate == seat_id)
        })
        .and_then(MaskedClaimsSeat::from_index)
}

fn private_view(state: &MaskedClaimsState, viewer_seat: Option<MaskedClaimsSeat>) -> PrivateView {
    match viewer_seat {
        Some(seat) => PrivateView::Seat(SeatPrivateView {
            seat,
            own_hand: state.hands[seat.index()]
                .iter()
                .copied()
                .map(mask_view)
                .collect(),
        }),
        None => PrivateView::Observer,
    }
}

fn exposed_view(mask: ExposedMask) -> ExposedMaskView {
    ExposedMaskView {
        tile_id: mask.tile.as_str().to_owned(),
        actual_grade: mask.tile.grade(),
        declared_grade: mask.declared,
        claimant: mask.claimant,
        challenger: mask.challenger,
    }
}

fn counter_view(counters: ChallengeCounters) -> CounterView {
    CounterView {
        exposed_lies: counters.exposed_lies,
        successful_challenges: counters.successful_challenges,
        challenges_declared: counters.challenges_declared,
    }
}

fn terminal_view(outcome: Option<TerminalOutcome>) -> TerminalView {
    match outcome {
        None => TerminalView::NonTerminal,
        Some(outcome) => TerminalView::Complete {
            outcome,
            rationale: outcome_rationale(outcome),
        },
    }
}

fn outcome_rationale(outcome: TerminalOutcome) -> OutcomeRationaleView {
    let (result_kind, decisive_cause, final_scores) = match outcome {
        TerminalOutcome::ScoreWin { winner, scores } => (
            "score_win".to_owned(),
            format!("{} wins on final score", winner.as_str()),
            scores,
        ),
        TerminalOutcome::TiebreakWin {
            winner,
            scores,
            tiebreak,
        } => (
            "tiebreak_win".to_owned(),
            format!("{} wins on {}", winner.as_str(), tiebreak),
            scores,
        ),
        TerminalOutcome::Draw { scores } => (
            "draw".to_owned(),
            "all tiebreakers are equal".to_owned(),
            scores,
        ),
    };
    OutcomeRationaleView {
        result_kind,
        decisive_cause,
        decisive_rule_ids: vec![
            "MC-SCORE-001".to_owned(),
            "MC-SCORE-002".to_owned(),
            "MC-SCORE-003".to_owned(),
            "MC-END-001".to_owned(),
            "MC-END-002".to_owned(),
            "MC-END-003".to_owned(),
            "MC-END-004".to_owned(),
            "MC-END-005".to_owned(),
        ],
        final_scores,
    }
}

fn seat_option(seat: Option<MaskedClaimsSeat>) -> &'static str {
    seat.map(MaskedClaimsSeat::as_str).unwrap_or("none")
}

fn encode_pedestal(pedestal: Option<PedestalView>) -> String {
    match pedestal {
        Some(view) => format!(
            "{}:{}",
            view.claimant.as_str(),
            view.declared_grade.as_str()
        ),
        None => "none".to_owned(),
    }
}

fn encode_veiled(gallery: &[VeiledClaimView]) -> String {
    if gallery.is_empty() {
        return "none".to_owned();
    }
    gallery
        .iter()
        .map(|claim| claim.declared_grade.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn encode_exposed(row: &[ExposedMaskView]) -> String {
    if row.is_empty() {
        return "none".to_owned();
    }
    row.iter()
        .map(|mask| {
            format!(
                "{}:{}:{}:{}",
                mask.tile_id,
                mask.declared_grade.as_str(),
                mask.claimant.as_str(),
                mask.challenger.as_str()
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn encode_counter(counter: CounterView) -> String {
    format!(
        "lies={},successes={},declared={}",
        counter.exposed_lies, counter.successful_challenges, counter.challenges_declared
    )
}

fn encode_terminal(terminal: &TerminalView) -> String {
    match terminal {
        TerminalView::NonTerminal => "none".to_owned(),
        TerminalView::Complete { rationale, .. } => {
            format!("{}:{}", rationale.result_kind, rationale.decisive_cause)
        }
    }
}

fn encode_private(private: &PrivateView) -> String {
    match private {
        PrivateView::Observer => "observer".to_owned(),
        PrivateView::Seat(view) => format!(
            "{}:{}",
            view.seat.as_str(),
            view.own_hand
                .iter()
                .map(|mask| mask.tile_id.as_str())
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        actions::{validate_command, ValidatedAction},
        apply_action,
        setup::{setup_match, SetupOptions},
    };
    use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed};

    fn viewer(seat: Option<&str>) -> Viewer {
        Viewer {
            seat_id: seat.map(|seat| SeatId(seat.to_owned())),
        }
    }

    fn command(state: &MaskedClaimsState, seat: &str, segments: Vec<&str>) -> CommandEnvelope {
        CommandEnvelope {
            actor: Actor {
                seat_id: SeatId(seat.to_owned()),
            },
            action_path: ActionPath {
                segments: segments.into_iter().map(str::to_owned).collect(),
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    #[test]
    fn public_view_hides_hands_reserve_and_pending_pedestal_tile() {
        let mut state = setup_match(
            Seed(5),
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds");
        let hidden_tile = state.hands[0][0];
        let ValidatedAction::Claim(claim) = validate_command(
            &state,
            &command(&state, "seat_0", vec!["claim", hidden_tile.as_str(), "5"]),
        )
        .expect("claim validates") else {
            panic!("expected claim");
        };
        apply_action(&mut state, ValidatedAction::Claim(claim)).expect("claim applies");

        let public = project_view(&state, &viewer(None));
        let rendered = public.stable_summary();
        assert!(public.pedestal.is_some());
        assert!(!rendered.contains(hidden_tile.as_str()));
        for tile in state.hands[1].iter().chain(state.reserve.iter()) {
            assert!(!rendered.contains(tile.as_str()));
        }
    }

    #[test]
    fn seat_view_contains_only_that_seats_own_hand() {
        let state = setup_match(
            Seed(6),
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds");
        let seat_0 = project_view(&state, &viewer(Some("seat_0")));
        let rendered = seat_0.stable_summary();

        for tile in &state.hands[0] {
            assert!(rendered.contains(tile.as_str()));
        }
        for tile in state.hands[1].iter().chain(state.reserve.iter()) {
            assert!(!rendered.contains(tile.as_str()));
        }
    }
}
