//! Public projection and no-leak helpers for Event Frontier.

use engine_core::{
    ActionTree, Diagnostic, EffectEnvelope, HashValue, StableSerialize, Viewer, VisibilityScope,
};

use crate::{
    actions::OperationKind,
    cards::CardId,
    effects::{EventFrontierEffect, EventFrontierEffectEnvelope},
    ids::{FactionId, SiteId, GAME_ID, RULES_VERSION_LABEL},
    state::{CardPhase, Eligibility, EventFrontierState, TerminalOutcome},
    ui::{card_face, site_label, ui_metadata, CardFaceView, UiMetadata},
};

pub const HIDDEN_SURFACE: &str = "undrawn_deck_order";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicView {
    pub schema_version: u32,
    pub rules_version: u32,
    pub game_id: String,
    pub variant_id: String,
    pub rules_version_label: String,
    pub seats: Vec<String>,
    pub factions: Vec<String>,
    pub active_seat: Option<String>,
    pub sites: Vec<SiteView>,
    pub adjacency: Vec<(SiteId, Vec<SiteId>)>,
    pub resources: ResourceView,
    pub scores: ScoreView,
    pub eligibility: Vec<(FactionId, Eligibility)>,
    pub current_card: Option<CardFaceView>,
    pub next_public_card: Option<CardFaceView>,
    pub discard: Vec<CardFaceView>,
    pub active_edicts: Vec<String>,
    pub epoch: u8,
    pub reckoning_count: u8,
    pub victory_distance: VictoryDistanceView,
    pub terminal: TerminalView,
    pub ui: UiMetadata,
    pub freshness_token: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SiteView {
    pub site: SiteId,
    pub label: String,
    pub agents: u8,
    pub settlers: u8,
    pub depot: bool,
    pub cache_count: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceView {
    pub funds: u8,
    pub provisions: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScoreView {
    pub charter: u16,
    pub freeholders: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VictoryDistanceView {
    pub charter_sites_needed: u8,
    pub freeholder_caches_needed: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalView {
    NonTerminal,
    Complete {
        winner: FactionId,
        victory_type: String,
        scores: ScoreView,
        decisive_rule: String,
    },
}

pub fn project_view(state: &EventFrontierState, _viewer: &Viewer) -> PublicView {
    PublicView {
        schema_version: 1,
        rules_version: 1,
        game_id: GAME_ID.to_owned(),
        variant_id: state.variant.id.clone(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        seats: state.seats.iter().map(|seat| seat.0.clone()).collect(),
        factions: state
            .factions
            .iter()
            .map(|faction| faction.as_str().to_owned())
            .collect(),
        active_seat: active_seat(state),
        sites: state
            .sites
            .iter()
            .map(|site| SiteView {
                site: site.site,
                label: site_label(site.site),
                agents: site.agents,
                settlers: site.settlers,
                depot: site.depot,
                cache_count: site.cache_count,
            })
            .collect(),
        adjacency: state
            .adjacency
            .iter()
            .map(|entry| (entry.site, entry.neighbors.clone()))
            .collect(),
        resources: ResourceView {
            funds: state.resources.funds,
            provisions: state.resources.provisions,
        },
        scores: ScoreView {
            charter: state.scores.charter,
            freeholders: state.scores.freeholders,
        },
        eligibility: FactionId::ALL
            .into_iter()
            .map(|faction| (faction, state.eligibility_for(faction)))
            .collect(),
        current_card: state.deck.current.map(card_face),
        next_public_card: state.deck.next_public.map(card_face),
        discard: state.deck.discard.iter().copied().map(card_face).collect(),
        active_edicts: state
            .active_edicts
            .iter()
            .map(|edict| {
                format!(
                    "{}:{}:{}",
                    edict.kind.as_str(),
                    edict.card.as_str(),
                    edict.activation_index
                )
            })
            .collect(),
        epoch: state.deck.epoch,
        reckoning_count: state.reckoning_count,
        victory_distance: victory_distance(state),
        terminal: terminal_view(state),
        ui: ui_metadata(),
        freshness_token: state.freshness_token.0,
    }
}

pub fn filter_effects_for_viewer(
    effects: &[EventFrontierEffectEnvelope],
    viewer: &Viewer,
) -> Vec<EventFrontierEffectEnvelope> {
    effects
        .iter()
        .filter(|effect| match &effect.visibility {
            VisibilityScope::Public => true,
            VisibilityScope::PrivateToSeat(seat_id) => viewer.seat_id.as_ref() == Some(seat_id),
        })
        .cloned()
        .collect()
}

pub fn public_effect_text(effect: &EventFrontierEffect) -> String {
    match effect {
        EventFrontierEffect::EventResolved { card, summary } => {
            format!("{} resolved: {summary}", card_label(*card))
        }
        EventFrontierEffect::EdictActivated { edict, .. } => {
            format!("edict activated: {}", edict_label(edict))
        }
        EventFrontierEffect::EdictExpired { edict } => {
            format!("edict expired: {}", edict_label(edict))
        }
        EventFrontierEffect::CardRevealed { card, next_public } => format!(
            "card revealed: {}; next={}",
            card_label(*card),
            next_public.map(card_label).unwrap_or("none")
        ),
        EventFrontierEffect::ChoiceTaken { faction, choice } => {
            format!("{} chose {}", faction.label(), choice_label(choice))
        }
        EventFrontierEffect::CardDiscarded { card, reason } => {
            format!("{} discarded: {}", card_label(*card), reason_label(reason))
        }
        EventFrontierEffect::EligibilityChanged {
            faction,
            eligible,
            reason,
        } => format!(
            "{} eligibility={eligible}: {}",
            faction.label(),
            reason_label(reason)
        ),
        EventFrontierEffect::ResourcesChanged {
            faction,
            new,
            reason,
            ..
        } => format!(
            "{} resource now {new}: {}",
            faction.label(),
            reason_label(reason)
        ),
        EventFrontierEffect::OpResolved { faction, op, sites } => format!(
            "{} operation {op} on {}",
            faction.label(),
            sites
                .iter()
                .map(|site| site.label())
                .collect::<Vec<_>>()
                .join(", "),
            op = choice_label(op),
        ),
        EventFrontierEffect::AgentPlaced { site, new_count } => {
            format!("agent placed at {} count {new_count}", site.label())
        }
        EventFrontierEffect::AgentRemoved { site, new_count } => {
            format!("agent removed at {} count {new_count}", site.label())
        }
        EventFrontierEffect::DepotBuilt { site } => format!("depot built at {}", site.label()),
        EventFrontierEffect::CacheRemoved { site, new_count } => {
            format!("cache removed at {} count {new_count}", site.label())
        }
        EventFrontierEffect::SettlerMoved {
            from,
            to,
            from_count,
            to_count,
        } => format!(
            "settler moved {}>{} counts {from_count}>{to_count}",
            from.label(),
            to.label()
        ),
        EventFrontierEffect::CacheLaid { site, new_count } => {
            format!("cache laid at {} count {new_count}", site.label())
        }
        EventFrontierEffect::SettlerRallied { site, new_count } => {
            format!("settler rallied at {} count {new_count}", site.label())
        }
        EventFrontierEffect::ReckoningResolved {
            round,
            victory_check,
            ..
        } => format!(
            "{} resolved: {victory_check}",
            reckoning_round_label(*round)
        ),
        EventFrontierEffect::Terminal {
            winner,
            victory_type,
            summary,
            ..
        } => format!(
            "terminal {} {}: {summary}",
            winner.label(),
            victory_type_label(victory_type)
        ),
    }
}

pub fn reason_label(reason: &str) -> &'static str {
    match reason {
        "event_choice" => "resolved the event",
        "operation_choice" => "used an operation",
        "reckoning_resolved" => "Reckoning resolved",
        "resolved_after_second_choice" => "second choice resolved",
        "double_pass" => "both factions passed",
        "resolved_after_second_pass" => "second faction passed",
        "no_eligible_faction" => "no eligible faction remained",
        "no_eligible_card_discard" => "no eligible faction could choose this card",
        "reckoning_income" => "Reckoning income",
        "operation_cost" => "operation cost",
        "pass_income" => "pass income",
        "storehouse_fire" => "Storehouse Fire",
        "high_meadow_fair" => "High Meadow Fair",
        "depot_grants" => "Depot Grants",
        "charter_audit" => "Charter Audit",
        "freeholder_moot" => "Freeholder Moot",
        "crossing_market" => "Crossing Market",
        "granite_pass_snows" => "Granite Pass Snows",
        "last_light" => "Last Light",
        _ => "state update",
    }
}

fn card_label(card: CardId) -> &'static str {
    match card {
        CardId::BorderSurvey => "Border Survey",
        CardId::TollRoads => "Toll Roads",
        CardId::RiverMists => "River Mists",
        CardId::StorehouseFire => "Storehouse Fire",
        CardId::SurveyBan => "Survey Ban",
        CardId::HighMeadowFair => "High Meadow Fair",
        CardId::ReckoningOne => "First Reckoning",
        CardId::DepotGrants => "Depot Grants",
        CardId::LongSeason => "Long Season",
        CardId::TrailWashout => "Trail Washout",
        CardId::CharterAudit => "Charter Audit",
        CardId::FreeholderMoot => "Freeholder Moot",
        CardId::Requisition => "Requisition",
        CardId::ReckoningTwo => "Second Reckoning",
        CardId::OldMillStrike => "Old Mill Strike",
        CardId::CrossingMarket => "Crossing Market",
        CardId::GranitePassSnows => "Granite Pass Snows",
        CardId::CacheBoom => "Cache Boom",
        CardId::AgentsRecall => "Agents Recall",
        CardId::LastLight => "Last Light",
        CardId::ReckoningThree => "Final Reckoning",
    }
}

fn reckoning_round_label(round: u8) -> &'static str {
    match round {
        1 => "First Reckoning",
        2 => "Second Reckoning",
        3 => "Final Reckoning",
        _ => "Reckoning",
    }
}

fn edict_label(edict: &str) -> &'static str {
    match edict {
        "toll_roads" => "Toll Roads",
        "survey_ban" => "Survey Ban",
        "long_season" => "Long Season",
        "requisition" => "Requisition",
        _ => "edict",
    }
}

fn choice_label(choice: &str) -> &'static str {
    if let Some(kind) = OperationKind::parse(choice) {
        return kind.label();
    }
    match choice {
        "event" => "Event",
        "operation" => "Operation",
        "limited_operation" => "Limited operation",
        "pass" => "Pass",
        _ => "choice",
    }
}

fn victory_type_label(victory_type: &str) -> &'static str {
    match victory_type {
        "charter_instant" => "Charter instant victory",
        "freeholder_instant" => "Freeholder instant victory",
        "both_met_freeholder" => "Freeholder both-met victory",
        "final_fallback" => "final fallback",
        _ => "terminal result",
    }
}

pub fn contains_hidden_deck_order<T: std::fmt::Debug>(
    state: &EventFrontierState,
    value: &T,
) -> bool {
    let rendered = format!("{value:?}");
    state
        .deck
        .undrawn
        .iter()
        .any(|card| rendered.contains(card.as_str()))
}

pub fn action_tree_hash(tree: &ActionTree) -> HashValue {
    HashValue::from_stable_bytes(format!("{tree:?}").as_bytes())
}

pub fn diagnostic_hash(diagnostic: &Diagnostic) -> HashValue {
    HashValue::from_stable_bytes(format!("{}:{}", diagnostic.code, diagnostic.message).as_bytes())
}

pub fn effect_hash(effect: &EffectEnvelope<EventFrontierEffect>) -> HashValue {
    HashValue::from_stable_bytes(public_effect_text(&effect.payload).as_bytes())
}

pub fn view_hash(view: &PublicView) -> HashValue {
    view.stable_hash()
}

impl PublicView {
    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};rules={};game={};variant={};rules_label={};seats={};factions={};active={};sites={};adjacency={};resources={}:{};scores={}:{};eligibility={};current={};next={};discard={};edicts={};epoch={};reckonings={};distance={}:{};terminal={};ui={};freshness={}",
            self.schema_version,
            self.rules_version,
            self.game_id,
            self.variant_id,
            self.rules_version_label,
            self.seats.join("|"),
            self.factions.join("|"),
            self.active_seat.as_deref().unwrap_or("none"),
            self.sites.iter().map(encode_site).collect::<Vec<_>>().join(","),
            self.adjacency.iter().map(encode_adjacency).collect::<Vec<_>>().join(","),
            self.resources.funds,
            self.resources.provisions,
            self.scores.charter,
            self.scores.freeholders,
            self.eligibility.iter().map(encode_eligibility).collect::<Vec<_>>().join(","),
            self.current_card
                .as_ref()
                .map(encode_card_face)
                .unwrap_or_else(|| "none".to_owned()),
            self.next_public_card
                .as_ref()
                .map(encode_card_face)
                .unwrap_or_else(|| "none".to_owned()),
            self.discard.iter().map(encode_card_face).collect::<Vec<_>>().join(","),
            self.active_edicts.join(","),
            self.epoch,
            self.reckoning_count,
            self.victory_distance.charter_sites_needed,
            self.victory_distance.freeholder_caches_needed,
            encode_terminal(&self.terminal),
            encode_ui(&self.ui),
            self.freshness_token
        )
    }
}

fn encode_card_face(card: &CardFaceView) -> String {
    format!(
        "{}:{}:{}:{}:{}",
        card.id, card.label, card.summary, card.family, card.accessibility_label
    )
}

fn encode_ui(ui: &UiMetadata) -> String {
    format!(
        "{}:{}:{}:{}:{}:{}:{}:{}:{}",
        ui.table_label,
        ui.event_deck_label,
        ui.current_card_label,
        ui.next_card_label,
        ui.discard_label,
        ui.face_down_label,
        ui.face_down_summary,
        ui.reduced_motion_token,
        ui.action_affordance_templates
            .iter()
            .map(|template| format!("{}={}", template.id, template.text))
            .collect::<Vec<_>>()
            .join("|")
    )
}

fn active_seat(state: &EventFrontierState) -> Option<String> {
    let faction = match &state.card_phase {
        CardPhase::AwaitingFirstChoice { faction } => *faction,
        CardPhase::AwaitingSecondChoice { second_faction, .. } => *second_faction,
        CardPhase::Reckoning | CardPhase::Terminal => return None,
    };
    state
        .seats
        .iter()
        .find(|seat| state.faction_for_seat(seat) == Some(faction))
        .map(|seat| seat.0.clone())
}

impl StableSerialize for PublicView {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

fn victory_distance(state: &EventFrontierState) -> VictoryDistanceView {
    let charter_majorities = state
        .sites
        .iter()
        .filter(|site| site.agents.saturating_add(u8::from(site.depot)) > site.settlers)
        .count() as u8;
    let caches = state
        .sites
        .iter()
        .map(|site| site.cache_count)
        .fold(0u8, u8::saturating_add);
    VictoryDistanceView {
        charter_sites_needed: state
            .variant
            .charter_site_threshold
            .saturating_sub(charter_majorities),
        freeholder_caches_needed: state
            .variant
            .freeholder_cache_threshold
            .saturating_sub(caches),
    }
}

fn terminal_view(state: &EventFrontierState) -> TerminalView {
    match &state.terminal_outcome {
        Some(TerminalOutcome::Winner {
            faction,
            victory_type,
            scores,
            decisive_rule,
        }) => TerminalView::Complete {
            winner: *faction,
            victory_type: victory_type.as_str().to_owned(),
            scores: ScoreView {
                charter: scores.charter,
                freeholders: scores.freeholders,
            },
            decisive_rule: decisive_rule.to_string(),
        },
        None => TerminalView::NonTerminal,
    }
}

fn encode_site(site: &SiteView) -> String {
    format!(
        "{}:{}:a{}:s{}:d{}:c{}",
        site.site.as_str(),
        site.label,
        site.agents,
        site.settlers,
        u8::from(site.depot),
        site.cache_count
    )
}

fn encode_adjacency((site, neighbors): &(SiteId, Vec<SiteId>)) -> String {
    format!(
        "{}=[{}]",
        site.as_str(),
        neighbors
            .iter()
            .map(|neighbor| neighbor.as_str())
            .collect::<Vec<_>>()
            .join("|")
    )
}

fn encode_eligibility((faction, eligibility): &(FactionId, Eligibility)) -> String {
    format!("{}={}", faction.as_str(), eligibility.as_str())
}

fn encode_terminal(terminal: &TerminalView) -> String {
    match terminal {
        TerminalView::NonTerminal => "none".to_owned(),
        TerminalView::Complete {
            winner,
            victory_type,
            scores,
            decisive_rule,
        } => format!(
            "{}:{}:{}:{}:{}",
            winner.as_str(),
            victory_type,
            scores.charter,
            scores.freeholders,
            decisive_rule
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effects::SiteScoreBreakdown;

    #[test]
    fn public_effect_text_does_not_emit_raw_snake_case_tokens() {
        let samples = vec![
            EventFrontierEffect::EventResolved {
                card: CardId::HighMeadowFair,
                summary: "fair raised provisions and a settler".to_owned(),
            },
            EventFrontierEffect::EdictActivated {
                card: CardId::TollRoads,
                edict: "toll_roads".to_owned(),
            },
            EventFrontierEffect::EdictExpired {
                edict: "survey_ban".to_owned(),
            },
            EventFrontierEffect::CardRevealed {
                card: CardId::BorderSurvey,
                next_public: Some(CardId::ReckoningOne),
            },
            EventFrontierEffect::ChoiceTaken {
                faction: FactionId::Charter,
                choice: "limited_operation".to_owned(),
            },
            EventFrontierEffect::CardDiscarded {
                card: CardId::LastLight,
                reason: "double_pass".to_owned(),
            },
            EventFrontierEffect::EligibilityChanged {
                faction: FactionId::Freeholders,
                eligible: false,
                reason: "event_choice".to_owned(),
            },
            EventFrontierEffect::ResourcesChanged {
                faction: FactionId::Charter,
                previous: 2,
                new: 1,
                reason: "operation_cost".to_owned(),
            },
            EventFrontierEffect::OpResolved {
                faction: FactionId::Charter,
                op: "survey".to_owned(),
                sites: vec![SiteId::GranitePass, SiteId::HighMeadow],
            },
            EventFrontierEffect::AgentPlaced {
                site: SiteId::Crossing,
                new_count: 1,
            },
            EventFrontierEffect::AgentRemoved {
                site: SiteId::Charterhouse,
                new_count: 0,
            },
            EventFrontierEffect::DepotBuilt {
                site: SiteId::Landing,
            },
            EventFrontierEffect::CacheRemoved {
                site: SiteId::OldMill,
                new_count: 0,
            },
            EventFrontierEffect::SettlerMoved {
                from: SiteId::Landing,
                to: SiteId::HighMeadow,
                from_count: 0,
                to_count: 1,
            },
            EventFrontierEffect::CacheLaid {
                site: SiteId::HighMeadow,
                new_count: 1,
            },
            EventFrontierEffect::SettlerRallied {
                site: SiteId::HighMeadow,
                new_count: 2,
            },
            EventFrontierEffect::ReckoningResolved {
                round: 1,
                victory_check: "no instant victory".to_owned(),
                site_breakdown: vec![SiteScoreBreakdown {
                    site: SiteId::Crossing,
                    charter_presence: 1,
                    freeholder_presence: 0,
                    awarded_to: Some(FactionId::Charter),
                }],
                income: (1, 1),
                expired_edicts: vec!["toll_roads".to_owned()],
            },
            EventFrontierEffect::Terminal {
                winner: FactionId::Freeholders,
                victory_type: "final_fallback".to_owned(),
                totals: (5, 5),
                summary: "Freeholders win tied final fallback scores".to_owned(),
            },
        ];

        for effect in samples {
            let text = public_effect_text(&effect);
            assert!(
                !contains_snake_case_token(&text),
                "raw token leaked in `{text}`"
            );
        }
    }

    fn contains_snake_case_token(text: &str) -> bool {
        text.split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
            .any(|token| {
                let mut parts = token.split('_');
                let Some(first) = parts.next() else {
                    return false;
                };
                !first.is_empty()
                    && first.chars().all(|ch| ch.is_ascii_lowercase())
                    && parts.clone().next().is_some()
                    && parts.all(|part| {
                        !part.is_empty() && part.chars().all(|ch| ch.is_ascii_lowercase())
                    })
            })
    }
}
