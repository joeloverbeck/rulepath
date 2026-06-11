//! Public projection and no-leak helpers for Frontier Control.

use engine_core::{
    ActionTree, Diagnostic, EffectEnvelope, HashValue, StableSerialize, Viewer, VisibilityScope,
};

use crate::{
    effects::{
        FortScoreBreakdown, FrontierControlEffect, FrontierControlEffectEnvelope,
        StakeScoreBreakdown,
    },
    ids::{FactionId, SiteId, GAME_ID, RULES_VERSION_LABEL},
    state::{FactionScores, FrontierControlState, Phase, SiteState, TerminalOutcome},
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
    pub factions: Vec<FactionView>,
    pub round_number: u8,
    pub active_faction: FactionId,
    pub active_seat: Option<String>,
    pub phase: PhaseView,
    pub sites: Vec<SiteView>,
    pub scores: ScoreView,
    pub terminal: TerminalView,
    pub freshness_token: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FactionView {
    pub seat: String,
    pub faction: FactionId,
    pub label: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhaseView {
    Action { budget_remaining: u8 },
    Terminal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SiteView {
    pub site: SiteId,
    pub label: &'static str,
    pub guards: u8,
    pub crews: u8,
    pub stake: bool,
    pub fort: bool,
    pub stake_value: u8,
    pub supplied: Option<bool>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScoreView {
    pub garrison: u16,
    pub prospectors: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalView {
    NonTerminal,
    Winner {
        faction: FactionId,
        scores: ScoreView,
        garrison_tiebreak: bool,
        summary: String,
    },
}

pub fn project_view(state: &FrontierControlState, _viewer: &Viewer) -> PublicView {
    PublicView {
        schema_version: 1,
        rules_version: 1,
        game_id: GAME_ID.to_owned(),
        display_name: state.variant.display_name.clone(),
        variant_id: state.variant.id.clone(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        seats: state.seats.iter().map(|seat| seat.0.clone()).collect(),
        factions: state
            .seats
            .iter()
            .zip(state.factions.iter().copied())
            .map(|(seat, faction)| FactionView {
                seat: seat.0.clone(),
                faction,
                label: faction.label().to_owned(),
            })
            .collect(),
        round_number: state.round_number,
        active_faction: state.active_faction,
        active_seat: state.active_seat().map(|seat| seat.0.clone()),
        phase: phase_view(state.phase),
        sites: state
            .sites
            .iter()
            .map(|site| site_view(state, site))
            .collect(),
        scores: score_view(state.scores),
        terminal: terminal_view(state),
        freshness_token: state.freshness_token.0,
    }
}

pub fn filter_effects_for_viewer(
    effects: &[FrontierControlEffectEnvelope],
    _viewer: &Viewer,
) -> Vec<FrontierControlEffectEnvelope> {
    effects.to_vec()
}

pub fn public_effect_text(effect: &FrontierControlEffect) -> String {
    match effect {
        FrontierControlEffect::CrewMarched { from, to } => {
            format!("Crew marched from {} to {}", from.label(), to.label())
        }
        FrontierControlEffect::GuardPatrolled { from, to } => {
            format!("Guard patrolled from {} to {}", from.label(), to.label())
        }
        FrontierControlEffect::ClashResolved {
            site,
            guard_removed,
            crew_removed,
            entering_faction,
        } => format!(
            "Clash at {} resolved for {}; guard_removed={}; crew_removed={}",
            site.label(),
            entering_faction.label(),
            guard_removed,
            crew_removed
        ),
        FrontierControlEffect::StakePlaced { site } => format!("Stake placed at {}", site.label()),
        FrontierControlEffect::StakeDismantled { site } => {
            format!("Stake dismantled at {}", site.label())
        }
        FrontierControlEffect::CrewMustered { site, crews } => {
            format!("Crew mustered at {}; crews={crews}", site.label())
        }
        FrontierControlEffect::GuardReinforced { site, guards } => {
            format!("Guard reinforced at {}; guards={guards}", site.label())
        }
        FrontierControlEffect::TurnEnded { faction, round } => {
            format!("{} ended turn in round {round}", faction.label())
        }
        FrontierControlEffect::RoundScored {
            round,
            garrison_points,
            prospector_points,
            ..
        } => format!(
            "Round {round} scored: Garrison +{garrison_points}, Prospectors +{prospector_points}"
        ),
        FrontierControlEffect::Terminal { summary, .. } => summary.clone(),
    }
}

pub fn action_tree_hash(tree: &ActionTree) -> HashValue {
    HashValue::from_stable_bytes(format!("{tree:?}").as_bytes())
}

pub fn diagnostic_hash(diagnostic: &Diagnostic) -> HashValue {
    HashValue::from_stable_bytes(format!("{}:{}", diagnostic.code, diagnostic.message).as_bytes())
}

pub fn effect_hash(effect: &EffectEnvelope<FrontierControlEffect>) -> HashValue {
    HashValue::from_stable_bytes(effect_summary(effect).as_bytes())
}

pub fn view_hash(view: &PublicView) -> HashValue {
    view.stable_hash()
}

impl PublicView {
    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};rules={};game={};display={};variant={};label={};seats={};factions={};round={};active={};active_seat={};phase={};sites={};scores={}:{};terminal={};freshness={}",
            self.schema_version,
            self.rules_version,
            self.game_id,
            self.display_name,
            self.variant_id,
            self.rules_version_label,
            self.seats.join("|"),
            self.factions.iter().map(encode_faction).collect::<Vec<_>>().join(","),
            self.round_number,
            self.active_faction.as_str(),
            self.active_seat.as_deref().unwrap_or("observer"),
            encode_phase(self.phase),
            self.sites.iter().map(encode_site).collect::<Vec<_>>().join(","),
            self.scores.garrison,
            self.scores.prospectors,
            encode_terminal(&self.terminal),
            self.freshness_token,
        )
    }
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

fn site_view(state: &FrontierControlState, site: &SiteState) -> SiteView {
    SiteView {
        site: site.site,
        label: site.site.label(),
        guards: site.guards,
        crews: site.crews,
        stake: site.stake,
        fort: site.fort,
        stake_value: site.stake_value,
        supplied: state
            .last_stake_supply
            .iter()
            .find(|entry| entry.site == site.site)
            .map(|entry| entry.supplied),
    }
}

fn score_view(scores: FactionScores) -> ScoreView {
    ScoreView {
        garrison: scores.garrison,
        prospectors: scores.prospectors,
    }
}

fn terminal_view(state: &FrontierControlState) -> TerminalView {
    match &state.terminal_outcome {
        Some(TerminalOutcome::Winner {
            faction,
            scores,
            garrison_tiebreak,
        }) => TerminalView::Winner {
            faction: *faction,
            scores: score_view(*scores),
            garrison_tiebreak: *garrison_tiebreak,
            summary: terminal_summary(*faction, *scores, *garrison_tiebreak),
        },
        None => TerminalView::NonTerminal,
    }
}

fn terminal_summary(winner: FactionId, scores: FactionScores, garrison_tiebreak: bool) -> String {
    if garrison_tiebreak {
        return format!(
            "{} wins the frontier on the Garrison tiebreak, {}-{}",
            winner.label(),
            scores.garrison,
            scores.prospectors
        );
    }
    format!(
        "{} wins the frontier, {}-{}",
        winner.label(),
        scores.garrison,
        scores.prospectors
    )
}

fn encode_faction(faction: &FactionView) -> String {
    format!("{}:{}", faction.seat, faction.faction.as_str())
}

fn encode_phase(phase: PhaseView) -> String {
    match phase {
        PhaseView::Action { budget_remaining } => format!("action:{budget_remaining}"),
        PhaseView::Terminal => "terminal".to_owned(),
    }
}

fn encode_site(site: &SiteView) -> String {
    format!(
        "{}:g{}:c{}:stake{}:fort{}:value{}:supplied{}",
        site.site.as_str(),
        site.guards,
        site.crews,
        u8::from(site.stake),
        u8::from(site.fort),
        site.stake_value,
        site.supplied
            .map(|value| u8::from(value).to_string())
            .unwrap_or_else(|| "none".to_owned())
    )
}

fn encode_terminal(terminal: &TerminalView) -> String {
    match terminal {
        TerminalView::NonTerminal => "none".to_owned(),
        TerminalView::Winner {
            faction,
            scores,
            garrison_tiebreak,
            summary,
        } => format!(
            "winner:{}:{}:{}:tiebreak={}:{}",
            faction.as_str(),
            scores.garrison,
            scores.prospectors,
            garrison_tiebreak,
            summary
        ),
    }
}

fn effect_summary(effect: &EffectEnvelope<FrontierControlEffect>) -> String {
    let visibility = match &effect.visibility {
        VisibilityScope::Public => "public".to_owned(),
        VisibilityScope::PrivateToSeat(seat) => format!("private:{}", seat.0),
    };
    format!("{visibility}:{}", encode_effect(&effect.payload))
}

fn encode_effect(effect: &FrontierControlEffect) -> String {
    match effect {
        FrontierControlEffect::CrewMarched { from, to } => {
            format!("crew_marched:{}:{}", from.as_str(), to.as_str())
        }
        FrontierControlEffect::GuardPatrolled { from, to } => {
            format!("guard_patrolled:{}:{}", from.as_str(), to.as_str())
        }
        FrontierControlEffect::ClashResolved {
            site,
            guard_removed,
            crew_removed,
            entering_faction,
        } => format!(
            "clash:{}:{}:{}:{}",
            site.as_str(),
            guard_removed,
            crew_removed,
            entering_faction.as_str()
        ),
        FrontierControlEffect::StakePlaced { site } => format!("stake_placed:{}", site.as_str()),
        FrontierControlEffect::StakeDismantled { site } => {
            format!("stake_dismantled:{}", site.as_str())
        }
        FrontierControlEffect::CrewMustered { site, crews } => {
            format!("crew_mustered:{}:{crews}", site.as_str())
        }
        FrontierControlEffect::GuardReinforced { site, guards } => {
            format!("guard_reinforced:{}:{guards}", site.as_str())
        }
        FrontierControlEffect::TurnEnded { faction, round } => {
            format!("turn_ended:{}:{round}", faction.as_str())
        }
        FrontierControlEffect::RoundScored {
            round,
            garrison_points,
            prospector_points,
            fort_breakdown,
            stake_breakdown,
        } => format!(
            "round_scored:{round}:{garrison_points}:{prospector_points}:forts={}:stakes={}",
            fort_breakdown
                .iter()
                .map(encode_fort_breakdown)
                .collect::<Vec<_>>()
                .join(","),
            stake_breakdown
                .iter()
                .map(encode_stake_breakdown)
                .collect::<Vec<_>>()
                .join(",")
        ),
        FrontierControlEffect::Terminal {
            winner,
            garrison_total,
            prospector_total,
            tiebreak_applied,
            summary,
        } => format!(
            "terminal:{}:{garrison_total}:{prospector_total}:{tiebreak_applied}:{summary}",
            winner.as_str()
        ),
    }
}

fn encode_fort_breakdown(entry: &FortScoreBreakdown) -> String {
    format!("{}:{}:{}", entry.site.as_str(), entry.held, entry.points)
}

fn encode_stake_breakdown(entry: &StakeScoreBreakdown) -> String {
    format!(
        "{}:{}:{}:{}",
        entry.site.as_str(),
        entry.value,
        entry.supplied,
        entry.points
    )
}

#[cfg(test)]
mod tests {
    use engine_core::{SeatId, Viewer};

    use crate::{
        effects::{public_effect, FrontierControlEffect},
        setup::{setup_match, SetupOptions},
        state::StakeSupplyStatus,
    };

    use super::*;

    fn state() -> FrontierControlState {
        setup_match(
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds")
    }

    #[test]
    fn visibility_all_viewers_receive_identical_public_view() {
        let state = state();
        let observer = project_view(&state, &Viewer { seat_id: None });
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

        assert_eq!(observer, seat_0);
        assert_eq!(observer, seat_1);
        assert_eq!(view_hash(&observer), view_hash(&seat_0));
    }

    #[test]
    fn visibility_effect_filter_is_identity_projection() {
        let effects = vec![public_effect(FrontierControlEffect::StakePlaced {
            site: SiteId::Ford,
        })];
        let filtered = filter_effects_for_viewer(&effects, &Viewer { seat_id: None });

        assert_eq!(filtered, effects);
    }

    #[test]
    fn visibility_projects_recorded_supply_status() {
        let mut state = state();
        state.site_mut(SiteId::Ford).expect("ford").stake = true;
        state.last_stake_supply = vec![StakeSupplyStatus {
            site: SiteId::Ford,
            supplied: false,
        }];

        let view = project_view(&state, &Viewer { seat_id: None });
        let ford = view
            .sites
            .iter()
            .find(|site| site.site == SiteId::Ford)
            .expect("ford is projected");

        assert_eq!(ford.supplied, Some(false));
    }
}
