//! Public projection and no-leak helpers for Flood Watch.

use engine_core::{
    ActionTree, Diagnostic, EffectEnvelope, HashValue, StableSerialize, Viewer, VisibilityScope,
};

use crate::{
    effects::{FloodWatchEffect, FloodWatchEffectEnvelope, TerminalSummary},
    ids::{DistrictId, EventKind, FloodWatchRole, GAME_ID, RULES_VERSION_LABEL},
    state::{DistrictState, FloodWatchState, Phase, SharedOutcome, StableComposition},
    ui::{card_face, ui_metadata, CardFaceView, UiMetadata},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicView {
    pub schema_version: u32,
    pub rules_version: u32,
    pub game_id: String,
    pub display_name: String,
    pub variant_id: String,
    pub rules_version_label: String,
    pub seats: Vec<String>,
    pub roles: Vec<RoleView>,
    pub turn_number: u16,
    pub active_seat: String,
    pub phase: PhaseView,
    pub districts: Vec<DistrictView>,
    pub drawn_cards: Vec<CardFaceView>,
    pub forecast: Option<CardFaceView>,
    pub remaining_composition: CompositionView,
    pub undrawn_count: u8,
    pub terminal: TerminalView,
    pub freshness_token: u64,
    pub ui: UiMetadata,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoleView {
    pub seat: String,
    pub role: FloodWatchRole,
    pub label: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhaseView {
    Action { budget_remaining: u8 },
    Terminal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DistrictView {
    pub district: DistrictId,
    pub label: &'static str,
    pub flood_level: u8,
    pub levees: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionView {
    pub downpours_per_district: Vec<(DistrictId, u8)>,
    pub surges_per_district: Vec<(DistrictId, u8)>,
    pub reprieves: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalView {
    NonTerminal,
    Complete {
        outcome: String,
        summary: TerminalSummary,
    },
}

pub fn project_view(state: &FloodWatchState, _viewer: &Viewer) -> PublicView {
    PublicView {
        schema_version: 1,
        rules_version: 1,
        game_id: GAME_ID.to_owned(),
        display_name: "Flood Watch".to_owned(),
        variant_id: state.variant.id.clone(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        seats: state.seats.iter().map(|seat| seat.0.clone()).collect(),
        roles: state
            .seats
            .iter()
            .zip(state.roles.iter().copied())
            .map(|(seat, role)| RoleView {
                seat: seat.0.clone(),
                role,
                label: role.label().to_owned(),
            })
            .collect(),
        turn_number: state.turn_number,
        active_seat: state.active_seat.0.clone(),
        phase: phase_view(state.phase),
        districts: state.districts.iter().map(district_view).collect(),
        drawn_cards: state
            .drawn
            .iter()
            .map(|card| card_face(card.kind))
            .collect(),
        forecast: state.forecast.as_ref().map(|card| card_face(card.kind)),
        remaining_composition: composition_view(state.remaining_composition()),
        undrawn_count: state.undrawn_deck_len() as u8,
        terminal: terminal_view(state),
        freshness_token: state.freshness_token.0,
        ui: ui_metadata(),
    }
}

pub fn filter_effects_for_viewer(
    effects: &[FloodWatchEffectEnvelope],
    viewer: &Viewer,
) -> Vec<FloodWatchEffectEnvelope> {
    effects
        .iter()
        .filter(|effect| match &effect.visibility {
            VisibilityScope::Public => true,
            VisibilityScope::PrivateToSeat(seat_id) => viewer.seat_id.as_ref() == Some(seat_id),
        })
        .cloned()
        .collect()
}

pub fn public_effect_text(effect: &FloodWatchEffect) -> String {
    match effect {
        FloodWatchEffect::DistrictBailed { district, amount } => {
            format!("{} bailed by {amount}", district.label())
        }
        FloodWatchEffect::LeveePlaced { district, amount } => {
            format!("{} reinforced by {amount}", district.label())
        }
        FloodWatchEffect::ForecastRevealed { card } => {
            format!("Forecast revealed {}", card_face(*card).label)
        }
        FloodWatchEffect::EnvironmentPhaseBegan { turn, draws } => {
            format!("Environment phase began on turn {turn} for {draws} draws")
        }
        FloodWatchEffect::EventDrawn { index, card } => {
            format!("Event {index} drawn: {}", card_face(*card).label)
        }
        FloodWatchEffect::LeveeAbsorbed {
            district,
            amount,
            remaining_levees,
        } => format!(
            "{} levee absorbed {amount}; {remaining_levees} levees remain",
            district.label()
        ),
        FloodWatchEffect::FloodLevelRose {
            district,
            amount,
            new_level,
        } => format!(
            "{} rose by {amount} to flood level {new_level}",
            district.label()
        ),
        FloodWatchEffect::DistrictInundated { district } => {
            format!("{} inundated", district.label())
        }
        FloodWatchEffect::DeckExhausted => "Event deck exhausted".to_owned(),
        FloodWatchEffect::Terminal { summary, .. } => summary.public_summary.clone(),
    }
}

pub fn contains_hidden_event_identity<T: std::fmt::Debug>(
    state: &FloodWatchState,
    value: &T,
) -> bool {
    let rendered = format!("{value:?}");
    let visible_kind_ids = state
        .drawn
        .iter()
        .map(|card| card.kind.id())
        .chain(state.forecast.as_ref().map(|card| card.kind.id()))
        .collect::<Vec<_>>();
    state
        .event_deck_internal()
        .iter()
        .filter(|card| state.forecast.as_ref() != Some(*card))
        .any(|card| {
            rendered.contains(&card.stable_id())
                || match card.kind {
                    EventKind::Downpour { .. } | EventKind::StormSurge { .. } => {
                        let kind_id = card.kind.id();
                        !visible_kind_ids.contains(&kind_id) && rendered.contains(&kind_id)
                    }
                    EventKind::Reprieve => false,
                }
        })
}

pub fn action_tree_hash(tree: &ActionTree) -> HashValue {
    HashValue::from_stable_bytes(format!("{tree:?}").as_bytes())
}

pub fn diagnostic_hash(diagnostic: &Diagnostic) -> HashValue {
    HashValue::from_stable_bytes(format!("{}:{}", diagnostic.code, diagnostic.message).as_bytes())
}

pub fn effect_hash(effect: &EffectEnvelope<FloodWatchEffect>) -> HashValue {
    HashValue::from_stable_bytes(effect_summary(effect).as_bytes())
}

pub fn view_hash(view: &PublicView) -> HashValue {
    view.stable_hash()
}

impl PublicView {
    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};rules={};game={};variant={};label={};seats={};roles={};turn={};active={};phase={};districts={};drawn={};forecast={};remaining={};undrawn={};terminal={};freshness={};ui={}",
            self.schema_version,
            self.rules_version,
            self.game_id,
            self.variant_id,
            self.rules_version_label,
            self.seats.join("|"),
            self.roles.iter().map(encode_role).collect::<Vec<_>>().join(","),
            self.turn_number,
            self.active_seat,
            encode_phase(self.phase),
            self.districts.iter().map(encode_district).collect::<Vec<_>>().join(","),
            self.drawn_cards.iter().map(encode_card_face).collect::<Vec<_>>().join(","),
            self.forecast
                .as_ref()
                .map(encode_card_face)
                .unwrap_or_else(|| "none".to_owned()),
            encode_composition(&self.remaining_composition),
            self.undrawn_count,
            encode_terminal(&self.terminal),
            self.freshness_token,
            encode_ui(&self.ui),
        )
    }
}

fn encode_card_face(card: &CardFaceView) -> String {
    format!(
        "{}:{}:{}:{}:{}:{}",
        card.id,
        card.label,
        card.summary,
        card.details.as_deref().unwrap_or("none"),
        card.family,
        card.accessibility_label
    )
}

fn encode_ui(ui: &UiMetadata) -> String {
    format!(
        "{}:{}:{}:{}:{}:{}:{}",
        ui.display_name,
        ui.event_deck_label,
        ui.forecast_label,
        ui.drawn_label,
        ui.face_down_label,
        ui.face_down_summary,
        ui.reduced_motion_token
    )
}

impl StableSerialize for PublicView {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

fn phase_view(phase: Phase) -> PhaseView {
    match phase {
        Phase::Action { budget_remaining } => PhaseView::Action { budget_remaining },
        Phase::Terminal => PhaseView::Terminal,
    }
}

fn district_view(district: &DistrictState) -> DistrictView {
    DistrictView {
        district: district.district,
        label: district.district.label(),
        flood_level: district.flood_level,
        levees: district.levees,
    }
}

fn composition_view(composition: StableComposition) -> CompositionView {
    CompositionView {
        downpours_per_district: composition.downpours_per_district,
        surges_per_district: composition.surges_per_district,
        reprieves: composition.reprieves,
    }
}

fn terminal_view(state: &FloodWatchState) -> TerminalView {
    match &state.terminal_outcome {
        Some(outcome) => TerminalView::Complete {
            outcome: outcome.stable_summary(),
            summary: terminal_summary_from_state(state, outcome),
        },
        None => TerminalView::NonTerminal,
    }
}

fn terminal_summary_from_state(
    state: &FloodWatchState,
    outcome: &SharedOutcome,
) -> TerminalSummary {
    let surviving_levels = state
        .districts
        .iter()
        .map(|district| (district.district, district.flood_level))
        .collect::<Vec<_>>();
    let drawn_card_count = state.drawn.len() as u8;

    match outcome {
        SharedOutcome::Won => TerminalSummary {
            rule_id: "FW-END-002".to_owned(),
            public_summary: format!(
                "Shared win: the final event resolved with all districts below inundation after {drawn_card_count} drawn cards."
            ),
            drawn_card_count,
            surviving_levels,
        },
        SharedOutcome::Lost { district } => TerminalSummary {
            rule_id: "FW-END-001".to_owned(),
            public_summary: format!(
                "Shared loss: {} reached inundation on turn {} after {drawn_card_count} drawn cards.",
                district.label(),
                state.turn_number
            ),
            drawn_card_count,
            surviving_levels,
        },
    }
}

fn encode_role(role: &RoleView) -> String {
    format!("{}:{}", role.seat, role.role.as_str())
}

fn encode_phase(phase: PhaseView) -> String {
    match phase {
        PhaseView::Action { budget_remaining } => format!("action:{budget_remaining}"),
        PhaseView::Terminal => "terminal".to_owned(),
    }
}

fn encode_district(district: &DistrictView) -> String {
    format!(
        "{}:{}:{}",
        district.district.as_str(),
        district.flood_level,
        district.levees
    )
}

fn encode_composition(composition: &CompositionView) -> String {
    format!(
        "downpours=[{}];surges=[{}];reprieves={}",
        composition
            .downpours_per_district
            .iter()
            .map(|(district, count)| format!("{}:{count}", district.as_str()))
            .collect::<Vec<_>>()
            .join(","),
        composition
            .surges_per_district
            .iter()
            .map(|(district, count)| format!("{}:{count}", district.as_str()))
            .collect::<Vec<_>>()
            .join(","),
        composition.reprieves
    )
}

fn encode_terminal(terminal: &TerminalView) -> String {
    match terminal {
        TerminalView::NonTerminal => "none".to_owned(),
        TerminalView::Complete { outcome, summary } => format!(
            "{}:{}:{}:{}",
            outcome,
            summary.rule_id,
            summary.drawn_card_count,
            summary
                .surviving_levels
                .iter()
                .map(|(district, level)| format!("{}:{level}", district.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

fn effect_summary(effect: &EffectEnvelope<FloodWatchEffect>) -> String {
    let visibility = match &effect.visibility {
        VisibilityScope::Public => "public".to_owned(),
        VisibilityScope::PrivateToSeat(seat) => format!("private:{}", seat.0),
    };
    format!("{visibility}:{}", encode_effect(&effect.payload))
}

fn encode_effect(effect: &FloodWatchEffect) -> String {
    match effect {
        FloodWatchEffect::DistrictBailed { district, amount } => {
            format!("district_bailed:{}:{amount}", district.as_str())
        }
        FloodWatchEffect::LeveePlaced { district, amount } => {
            format!("levee_placed:{}:{amount}", district.as_str())
        }
        FloodWatchEffect::ForecastRevealed { card } => format!("forecast:{}", card.id()),
        FloodWatchEffect::EnvironmentPhaseBegan { turn, draws } => {
            format!("environment_began:{turn}:{draws}")
        }
        FloodWatchEffect::EventDrawn { index, card } => {
            format!("event_drawn:{index}:{}", card.id())
        }
        FloodWatchEffect::LeveeAbsorbed {
            district,
            amount,
            remaining_levees,
        } => format!(
            "levee_absorbed:{}:{amount}:{remaining_levees}",
            district.as_str()
        ),
        FloodWatchEffect::FloodLevelRose {
            district,
            amount,
            new_level,
        } => format!("flood_rose:{}:{amount}:{new_level}", district.as_str()),
        FloodWatchEffect::DistrictInundated { district } => {
            format!("district_inundated:{}", district.as_str())
        }
        FloodWatchEffect::DeckExhausted => "deck_exhausted".to_owned(),
        FloodWatchEffect::Terminal { outcome, summary } => format!(
            "terminal:{}:{}:{}:{}",
            outcome,
            summary.rule_id,
            summary.drawn_card_count,
            summary
                .surviving_levels
                .iter()
                .map(|(district, level)| format!("{}:{level}", district.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}
