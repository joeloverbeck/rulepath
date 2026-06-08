use engine_core::{EffectEnvelope, FreshnessToken, StableSerialize, Viewer, VisibilityScope};

use crate::{
    effects::SecretDraftEffect,
    ids::{DraftItemId, SecretDraftSeat, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID},
    state::{Phase, RevealedRound, SecretDraftState, TerminalOutcome},
    ui::{item_accessibility_label, pending_copy, ui_metadata, UiMetadata},
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
    pub priority_seat: SecretDraftSeat,
    pub visible_pool: Vec<DraftItemView>,
    pub drafted: DraftedCollectionsView,
    pub commitments: CommitmentViews,
    pub scores: [u32; 2],
    pub revealed_history: Vec<RevealedRoundView>,
    pub terminal: TerminalView,
    pub freshness_token: FreshnessToken,
    pub private_view: PrivateView,
    pub ui: UiMetadata,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DraftItemView {
    pub item_id: String,
    pub label: String,
    pub thread: String,
    pub value: u8,
    pub accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DraftedCollectionsView {
    pub seat_0: Vec<DraftItemView>,
    pub seat_1: Vec<DraftItemView>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommitmentViews {
    pub seat_0: CommitmentView,
    pub seat_1: CommitmentView,
    pub copy: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommitmentView {
    pub seat: SecretDraftSeat,
    pub committed: bool,
    pub status: String,
    pub accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevealedRoundView {
    pub round_number: u8,
    pub seat_0_choice: DraftItemView,
    pub seat_1_choice: DraftItemView,
    pub seat_0_award: DraftItemView,
    pub seat_1_award: DraftItemView,
    pub priority_seat: SecretDraftSeat,
    pub contested: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalView {
    NonTerminal,
    Win { winning_seat: SecretDraftSeat },
    Draw,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrivateView {
    Observer,
    Seat {
        seat: SecretDraftSeat,
        own_committed: bool,
        waiting_copy: String,
    },
}

pub fn project_view(state: &SecretDraftState, viewer: &Viewer) -> PublicView {
    let viewer_seat = viewer_seat(state, viewer);
    let seat_0_committed = state.seat_committed(SecretDraftSeat::Seat0);
    let seat_1_committed = state.seat_committed(SecretDraftSeat::Seat1);

    PublicView {
        schema_version: 1,
        rules_version: 1,
        game_id: GAME_ID.to_owned(),
        display_name: "Veiled Draft".to_owned(),
        variant_id: VARIANT_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        round_number: state.round_number,
        round_limit: state.variant.round_count,
        phase: state.phase,
        priority_seat: state.priority_seat,
        visible_pool: state
            .visible_pool
            .iter()
            .copied()
            .map(draft_item_view)
            .collect(),
        drafted: DraftedCollectionsView {
            seat_0: state.drafted[SecretDraftSeat::Seat0.index()]
                .iter()
                .copied()
                .map(draft_item_view)
                .collect(),
            seat_1: state.drafted[SecretDraftSeat::Seat1.index()]
                .iter()
                .copied()
                .map(draft_item_view)
                .collect(),
        },
        commitments: CommitmentViews {
            seat_0: commitment_view(SecretDraftSeat::Seat0, seat_0_committed),
            seat_1: commitment_view(SecretDraftSeat::Seat1, seat_1_committed),
            copy: pending_copy(seat_0_committed, seat_1_committed),
        },
        scores: state.scores,
        revealed_history: state
            .revealed_history
            .iter()
            .map(revealed_round_view)
            .collect(),
        terminal: terminal_view(state.terminal_outcome),
        freshness_token: state.freshness_token,
        private_view: private_view(state, viewer_seat),
        ui: ui_metadata(),
    }
}

pub fn filter_effects_for_viewer(
    effects: &[EffectEnvelope<SecretDraftEffect>],
    viewer: &Viewer,
) -> Vec<EffectEnvelope<SecretDraftEffect>> {
    effects
        .iter()
        .filter(|effect| match &effect.visibility {
            VisibilityScope::Public => true,
            VisibilityScope::PrivateToSeat(seat_id) => viewer.seat_id.as_ref() == Some(seat_id),
        })
        .cloned()
        .collect()
}

pub fn contains_item_id_in_debug<T: std::fmt::Debug>(value: &T, item: DraftItemId) -> bool {
    format!("{value:?}").contains(item.as_str())
}

impl PublicView {
    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};rules={};game={};variant={};label={};round={}/{};phase={};priority={};pool={};drafted={}|{};commitments={}|{};scores={},{};revealed={};terminal={};freshness={};private={};ui={}",
            self.schema_version,
            self.rules_version,
            self.game_id,
            self.variant_id,
            self.rules_version_label,
            self.round_number,
            self.round_limit,
            self.phase.as_str(),
            self.priority_seat.as_str(),
            self.visible_pool.iter().map(encode_item).collect::<Vec<_>>().join(","),
            self.drafted.seat_0.iter().map(encode_item).collect::<Vec<_>>().join(","),
            self.drafted.seat_1.iter().map(encode_item).collect::<Vec<_>>().join(","),
            encode_commitment(&self.commitments.seat_0),
            encode_commitment(&self.commitments.seat_1),
            self.scores[0],
            self.scores[1],
            self.revealed_history.iter().map(encode_revealed_round).collect::<Vec<_>>().join(","),
            encode_terminal(self.terminal),
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

fn viewer_seat(state: &SecretDraftState, viewer: &Viewer) -> Option<SecretDraftSeat> {
    viewer
        .seat_id
        .as_ref()
        .and_then(|seat_id| {
            state
                .seats
                .iter()
                .position(|candidate| candidate == seat_id)
        })
        .and_then(SecretDraftSeat::from_index)
}

fn commitment_view(seat: SecretDraftSeat, committed: bool) -> CommitmentView {
    CommitmentView {
        seat,
        committed,
        status: if committed { "committed" } else { "waiting" }.to_owned(),
        accessibility_label: if committed {
            format!("{} has committed", seat.as_str())
        } else {
            format!("{} is waiting", seat.as_str())
        },
    }
}

fn private_view(state: &SecretDraftState, viewer_seat: Option<SecretDraftSeat>) -> PrivateView {
    match viewer_seat {
        Some(seat) => PrivateView::Seat {
            seat,
            own_committed: state.seat_committed(seat),
            waiting_copy: if state.seat_committed(seat) {
                "You have committed. Waiting for reveal.".to_owned()
            } else {
                "Choose from the visible pool.".to_owned()
            },
        },
        None => PrivateView::Observer,
    }
}

fn revealed_round_view(round: &RevealedRound) -> RevealedRoundView {
    RevealedRoundView {
        round_number: round.round_number,
        seat_0_choice: draft_item_view(round.seat_0_choice),
        seat_1_choice: draft_item_view(round.seat_1_choice),
        seat_0_award: draft_item_view(round.seat_0_award),
        seat_1_award: draft_item_view(round.seat_1_award),
        priority_seat: round.priority_seat,
        contested: round.contested,
    }
}

fn draft_item_view(item: DraftItemId) -> DraftItemView {
    DraftItemView {
        item_id: item.as_str().to_owned(),
        label: item.label().to_owned(),
        thread: item.thread().as_str().to_owned(),
        value: item.value(),
        accessibility_label: item_accessibility_label(item),
    }
}

fn terminal_view(outcome: Option<TerminalOutcome>) -> TerminalView {
    match outcome {
        None => TerminalView::NonTerminal,
        Some(TerminalOutcome::Win { seat }) => TerminalView::Win { winning_seat: seat },
        Some(TerminalOutcome::Draw) => TerminalView::Draw,
    }
}

fn encode_item(item: &DraftItemView) -> String {
    format!(
        "{}:{}:{}:{}",
        item.item_id, item.label, item.thread, item.value
    )
}

fn encode_commitment(commitment: &CommitmentView) -> String {
    format!(
        "{}:{}:{}",
        commitment.seat.as_str(),
        commitment.status,
        commitment.committed
    )
}

fn encode_revealed_round(round: &RevealedRoundView) -> String {
    format!(
        "{}:{}:{}:{}:{}:{}:{}",
        round.round_number,
        encode_item(&round.seat_0_choice),
        encode_item(&round.seat_1_choice),
        encode_item(&round.seat_0_award),
        encode_item(&round.seat_1_award),
        round.priority_seat.as_str(),
        round.contested
    )
}

fn encode_terminal(terminal: TerminalView) -> String {
    match terminal {
        TerminalView::NonTerminal => "non_terminal".to_owned(),
        TerminalView::Win { winning_seat } => format!("win:{}", winning_seat.as_str()),
        TerminalView::Draw => "draw".to_owned(),
    }
}

fn encode_private(private: &PrivateView) -> String {
    match private {
        PrivateView::Observer => "observer".to_owned(),
        PrivateView::Seat {
            seat,
            own_committed,
            waiting_copy,
        } => format!("{}:{}:{}", seat.as_str(), own_committed, waiting_copy),
    }
}

fn encode_ui(ui: &UiMetadata) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}",
        ui.game_id,
        ui.display_name,
        ui.table_label,
        ui.visible_pool_label,
        ui.drafted_label,
        ui.pending_label,
        ui.reveal_group_token,
        ui.reduced_motion_token
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rules::{apply_action, ValidatedAction},
        setup::SetupOptions,
        setup_match,
        ui::priority_copy,
    };
    use engine_core::{SeatId, VisibilityScope};

    fn standard_state() -> SecretDraftState {
        setup_match(
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds")
    }

    fn viewer(seat: Option<&str>) -> Viewer {
        Viewer {
            seat_id: seat.map(|seat| SeatId(seat.to_owned())),
        }
    }

    fn validated(actor: SecretDraftSeat, item: DraftItemId) -> ValidatedAction {
        ValidatedAction { actor, item }
    }

    #[test]
    fn public_and_committing_seat_views_redact_pre_reveal_commitment() {
        let mut state = standard_state();
        apply_action(
            &mut state,
            validated(SecretDraftSeat::Seat0, DraftItemId::Ember4),
        )
        .expect("first commit applies");

        let public_view = project_view(&state, &viewer(None));
        let seat_view = project_view(&state, &viewer(Some("seat_0")));

        assert_eq!(public_view.commitments.seat_0.status, "committed");
        assert_eq!(seat_view.commitments.seat_0.status, "committed");
        assert_eq!(
            seat_view.private_view,
            PrivateView::Seat {
                seat: SecretDraftSeat::Seat0,
                own_committed: true,
                waiting_copy: "You have committed. Waiting for reveal.".to_owned()
            }
        );
        assert!(!contains_item_id_in_debug(
            &public_view.commitments,
            DraftItemId::Ember4
        ));
        assert!(!contains_item_id_in_debug(
            &seat_view.commitments,
            DraftItemId::Ember4
        ));
        assert!(!contains_item_id_in_debug(
            &seat_view.private_view,
            DraftItemId::Ember4
        ));
    }

    #[test]
    fn viewer_scoped_pre_reveal_effects_have_pending_booleans_only() {
        let mut state = standard_state();
        let effects = apply_action(
            &mut state,
            validated(SecretDraftSeat::Seat0, DraftItemId::Grove4),
        )
        .expect("first commit applies");
        let filtered = filter_effects_for_viewer(&effects, &viewer(Some("seat_0")));

        assert_eq!(filtered.len(), 3);
        assert!(filtered
            .iter()
            .all(|effect| effect.visibility == VisibilityScope::Public));
        assert!(!contains_item_id_in_debug(&filtered, DraftItemId::Grove4));
    }

    #[test]
    fn post_reveal_view_exposes_revealed_and_awarded_items() {
        let mut state = standard_state();
        apply_action(
            &mut state,
            validated(SecretDraftSeat::Seat0, DraftItemId::Ember2),
        )
        .expect("first commit applies");
        apply_action(
            &mut state,
            validated(SecretDraftSeat::Seat1, DraftItemId::Tide2),
        )
        .expect("second commit applies");

        let view = project_view(&state, &viewer(None));
        assert_eq!(view.revealed_history.len(), 1);
        assert_eq!(view.revealed_history[0].seat_0_choice.item_id, "ember_2");
        assert_eq!(view.revealed_history[0].seat_1_choice.item_id, "tide_2");
        assert_eq!(view.drafted.seat_0[0].item_id, "ember_2");
        assert_eq!(view.drafted.seat_1[0].item_id, "tide_2");
        assert!(!view.commitments.seat_0.committed);
        assert!(!view.commitments.seat_1.committed);
    }

    #[test]
    fn projection_is_deterministic_and_stably_ordered() {
        let state = standard_state();
        let first = project_view(&state, &viewer(None));
        let second = project_view(&state, &viewer(None));

        assert_eq!(first, second);
        assert_eq!(first.stable_summary(), second.stable_summary());
        assert_eq!(first.visible_pool[0].item_id, "ember_1");
        assert_eq!(first.visible_pool[11].item_id, "grove_4");
        assert_eq!(
            priority_copy(first.priority_seat),
            "seat_0 has conflict priority"
        );
    }
}
