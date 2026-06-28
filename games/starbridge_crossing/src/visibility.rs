//! Public view projection for Starbridge Crossing.

use engine_core::{EffectEnvelope, SeatId, StableSerialize, Viewer};

use crate::{
    effects::StarbridgeEffect,
    ids::{DATA_VERSION_LABEL, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID},
    rules::progress_score,
    state::{FinishRank, StarPegId, StarbridgeState, TerminalStatus},
    topology::spaces_by_stable_order,
    ui::{space_ui_metadata, SpaceUiMetadata},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StarbridgePublicView {
    pub game_id: String,
    pub variant_id: String,
    pub rules_version_label: String,
    pub data_version_label: String,
    pub spaces: Vec<SpaceView>,
    pub seats: Vec<SeatView>,
    pub active_seat: Option<String>,
    pub finish_ranks: Vec<FinishRank>,
    pub terminal: Option<String>,
    pub terminal_rationale: Option<StarbridgeOutcomeRationaleView>,
    pub ply_count: u32,
    pub command_count: u32,
    pub audit: AllPublicAudit,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpaceView {
    pub space: String,
    pub coord: (i8, i8, i8),
    pub zone: String,
    pub occupant: Option<PegView>,
    pub ui: SpaceUiMetadata,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PegView {
    pub peg: String,
    pub owner_seat_index: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatView {
    pub seat_id: String,
    pub seat_index: u8,
    pub home: String,
    pub target: String,
    pub finish_rank: Option<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StarbridgeOutcomeRationaleView {
    pub result_kind: String,
    pub decisive_cause: String,
    pub template_key: String,
    pub decisive_rule_ids: Vec<String>,
    pub final_standing: Vec<StarbridgeOutcomeStandingView>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StarbridgeOutcomeStandingView {
    pub seat: SeatId,
    pub seat_index: u8,
    pub finish_rank: Option<u8>,
    pub winner: bool,
    pub finished: bool,
    pub progress: Option<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllPublicAudit {
    pub redaction_class: String,
    pub private_fields: Vec<String>,
    pub rationale: String,
}

pub fn project_view(state: &StarbridgeState, _viewer: &Viewer) -> StarbridgePublicView {
    StarbridgePublicView {
        game_id: GAME_ID.to_owned(),
        variant_id: VARIANT_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        data_version_label: DATA_VERSION_LABEL.to_owned(),
        spaces: spaces_by_stable_order()
            .map(|space| SpaceView {
                space: space.id.to_string(),
                coord: (space.coord.q, space.coord.r, space.coord.s),
                zone: crate::ui::zone_label(space.zone),
                occupant: state.occupancy(space.id).map(|peg| PegView {
                    peg: peg.stable_id(),
                    owner_seat_index: peg.seat_index,
                }),
                ui: space_ui_metadata(space.id, space.zone),
            })
            .collect(),
        seats: state
            .seats
            .iter()
            .map(|seat| SeatView {
                seat_id: seat.seat_id.0.clone(),
                seat_index: seat.seat_index,
                home: seat.home.label().to_owned(),
                target: seat.target.label().to_owned(),
                finish_rank: state
                    .finish_ranks
                    .iter()
                    .find(|rank| rank.seat_index == seat.seat_index)
                    .map(|rank| rank.rank),
            })
            .collect(),
        active_seat: state
            .seats
            .get(usize::from(state.active_seat_index))
            .filter(|_| state.terminal_status.is_none())
            .map(|seat| seat.seat_id.0.clone()),
        finish_ranks: state.finish_ranks.clone(),
        terminal: state.terminal_status.map(terminal_label),
        terminal_rationale: outcome_rationale(state),
        ply_count: state.ply_count,
        command_count: state.command_count,
        audit: AllPublicAudit {
            redaction_class: "none".to_owned(),
            private_fields: Vec::new(),
            rationale: "Starbridge Crossing has no private game facts".to_owned(),
        },
    }
}

pub fn filter_effects_for_viewer(
    effects: &[EffectEnvelope<StarbridgeEffect>],
    _viewer: &Viewer,
) -> Vec<EffectEnvelope<StarbridgeEffect>> {
    effects.to_vec()
}

impl StarbridgePublicView {
    pub fn stable_summary(&self) -> String {
        let spaces = self
            .spaces
            .iter()
            .map(|space| {
                format!(
                    "{}:{}:{}:{}",
                    space.space,
                    space.coord.0,
                    space.coord.1,
                    space
                        .occupant
                        .as_ref()
                        .map_or_else(|| "empty".to_owned(), |peg| peg.peg.clone())
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let seats = self
            .seats
            .iter()
            .map(|seat| {
                format!(
                    "{}:{}:{}:{}:{}",
                    seat.seat_index,
                    seat.seat_id,
                    seat.home,
                    seat.target,
                    seat.finish_rank
                        .map_or_else(|| "none".to_owned(), |rank| rank.to_string())
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let ranks = self
            .finish_ranks
            .iter()
            .map(|rank| format!("{}:{}", rank.seat_index, rank.rank))
            .collect::<Vec<_>>()
            .join(",");

        format!(
            "game={};variant={};rules={};data={};active={};terminal={};ply={};commands={};seats={};ranks={};spaces={};audit={}:{}",
            self.game_id,
            self.variant_id,
            self.rules_version_label,
            self.data_version_label,
            self.active_seat.as_deref().unwrap_or("none"),
            self.terminal.as_deref().unwrap_or("none"),
            self.ply_count,
            self.command_count,
            seats,
            ranks,
            spaces,
            self.audit.redaction_class,
            self.audit.private_fields.join(",")
        )
    }
}

impl StableSerialize for StarbridgePublicView {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

fn terminal_label(status: TerminalStatus) -> String {
    match status {
        TerminalStatus::Complete => "complete".to_owned(),
        TerminalStatus::TurnLimit { max_plies } => format!("turn_limit:{max_plies}"),
    }
}

fn outcome_rationale(state: &StarbridgeState) -> Option<StarbridgeOutcomeRationaleView> {
    let status = state.terminal_status?;
    let (result_kind, decisive_cause, template_key, decisive_rule_ids, include_progress) =
        match status {
            TerminalStatus::Complete => (
                "finish_order",
                "finish_order_complete",
                "starbridge_crossing.finish_order_complete",
                vec![
                    "SC-FINISH-001".to_owned(),
                    "SC-FINISH-002".to_owned(),
                    "SC-FINISH-003".to_owned(),
                    "SC-FINISH-004".to_owned(),
                    "SC-END-001".to_owned(),
                ],
                false,
            ),
            TerminalStatus::TurnLimit { .. } => (
                "turn_limit",
                "turn_limit_progress_vector",
                "starbridge_crossing.turn_limit_progress_vector",
                vec![
                    "SC-FINISH-005".to_owned(),
                    "SC-FINISH-006".to_owned(),
                    "SC-END-002".to_owned(),
                ],
                true,
            ),
        };

    Some(StarbridgeOutcomeRationaleView {
        result_kind: result_kind.to_owned(),
        decisive_cause: decisive_cause.to_owned(),
        template_key: template_key.to_owned(),
        decisive_rule_ids,
        final_standing: state
            .seats
            .iter()
            .map(|seat| {
                let finish_rank = state
                    .finish_ranks
                    .iter()
                    .find(|rank| rank.seat_index == seat.seat_index)
                    .map(|rank| rank.rank);
                let progress_score = progress_score(state, seat.seat_index);

                StarbridgeOutcomeStandingView {
                    seat: seat.seat_id.clone(),
                    seat_index: seat.seat_index,
                    finish_rank,
                    winner: finish_rank == Some(1),
                    finished: progress_score >= state.variant.pegs_per_seat,
                    progress: include_progress.then_some(progress_score),
                }
            })
            .collect(),
    })
}

#[allow(dead_code)]
fn _peg_id_is_public(_: StarPegId) {}
