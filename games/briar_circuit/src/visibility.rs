use engine_core::{ActionPath, EffectEnvelope, SeatId, Viewer, VisibilityScope};

use crate::{
    cards::CardId,
    effects::BriarCircuitEffect,
    ids::BriarCircuitSeat,
    rules::legal_play_cards,
    state::{BriarCircuitState, CapturedTrick, PassState, Phase, TrickPlay},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PassView {
    pub direction: String,
    pub committed_count: usize,
    pub pending_count: usize,
    pub own_selection: Vec<CardId>,
    pub own_committed: bool,
}

/// Public scoring summary of the most recently completed hand, shown between
/// hands in the browser. Carries only public scoring facts (no card identities).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HandSummaryView {
    pub raw_points: [u8; 4],
    pub hand_additions: [u8; 4],
    pub cumulative_after: [u16; 4],
    pub moon_shooter: Option<BriarCircuitSeat>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BriarCircuitView {
    pub viewer_seat: Option<BriarCircuitSeat>,
    pub phase: String,
    pub dealer: BriarCircuitSeat,
    pub hand_index: u32,
    pub cumulative_scores: [u16; 4],
    pub hand_counts: Vec<(BriarCircuitSeat, usize)>,
    pub own_hand: Vec<CardId>,
    pub pass: Option<PassView>,
    pub active_seat: Option<BriarCircuitSeat>,
    pub hearts_broken: Option<bool>,
    pub current_trick: Vec<TrickPlay>,
    pub captured_tricks: Vec<CapturedTrick>,
    pub last_hand_summary: Option<HandSummaryView>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionPreview {
    pub path: ActionPath,
    pub card: CardId,
}

pub fn project_view(state: &BriarCircuitState, viewer: &Viewer) -> BriarCircuitView {
    let viewer_seat = viewer_seat(viewer);
    let (phase, active_seat, hearts_broken, current_trick) = match &state.phase {
        // The simultaneous pass is resolved one commitment at a time. Reporting the
        // next uncommitted seat as the active seat is public-safe (commit order is
        // public per BC-PASS-003; card identities stay owner-private) and lets the
        // generic turn machinery drive each seat's pass selection.
        Phase::Passing(pass) => (
            "passing".to_owned(),
            next_uncommitted_seat(pass),
            None,
            Vec::new(),
        ),
        Phase::PlayingTrick(play) => (
            "playing".to_owned(),
            Some(play.active_seat),
            Some(play.hearts_broken),
            play.current_trick.plays.clone(),
        ),
        Phase::ScoringHand(_) => ("scoring".to_owned(), None, None, Vec::new()),
        Phase::Terminal(_) => ("terminal".to_owned(), None, None, Vec::new()),
    };

    BriarCircuitView {
        viewer_seat,
        phase,
        dealer: state.dealer,
        hand_index: state.hand_index,
        cumulative_scores: state.cumulative_scores,
        hand_counts: BriarCircuitSeat::ALL
            .into_iter()
            .map(|seat| (seat, state.hand_for_internal(seat).len()))
            .collect(),
        own_hand: viewer_seat
            .map(|seat| state.hand_for_internal(seat).to_vec())
            .unwrap_or_default(),
        pass: project_pass_view(state, viewer),
        active_seat,
        hearts_broken,
        current_trick,
        captured_tricks: state.captured_tricks.clone(),
        last_hand_summary: state
            .last_hand_summary
            .as_ref()
            .map(|breakdown| HandSummaryView {
                raw_points: breakdown.raw_points,
                hand_additions: breakdown.hand_additions,
                cumulative_after: breakdown.cumulative_after,
                moon_shooter: breakdown.moon_shooter,
            }),
    }
}

pub fn project_pass_view(state: &BriarCircuitState, viewer: &Viewer) -> Option<PassView> {
    let pass = state.pass_state()?;
    let viewer_seat = viewer_seat(viewer);
    let own_selection = viewer_seat
        .map(|seat| pass.selection_for(seat).to_vec())
        .unwrap_or_default();
    let own_committed = viewer_seat
        .map(|seat| pass.is_committed(seat))
        .unwrap_or(false);

    Some(PassView {
        direction: pass.direction.as_str().to_owned(),
        committed_count: pass.committed_count(),
        pending_count: pass.pending_count(),
        own_selection,
        own_committed,
    })
}

pub fn project_action_previews(state: &BriarCircuitState, viewer: &Viewer) -> Vec<ActionPreview> {
    let Some(seat) = viewer_seat(viewer) else {
        return Vec::new();
    };
    let Ok(legal_cards) = legal_play_cards(state, seat) else {
        return Vec::new();
    };

    legal_cards
        .into_iter()
        .map(|card| ActionPreview {
            path: ActionPath {
                segments: vec!["play".to_owned(), card.as_str()],
            },
            card,
        })
        .collect()
}

pub fn effect_envelopes(effect: BriarCircuitEffect) -> Vec<EffectEnvelope<BriarCircuitEffect>> {
    match effect {
        BriarCircuitEffect::PassSelectionUpdated {
            seat,
            selected_count,
            selected_cards,
        } => private_effect(
            seat,
            BriarCircuitEffect::PassSelectionUpdated {
                seat,
                selected_count,
                selected_cards,
            },
        ),
        BriarCircuitEffect::PassExchangePrivate {
            seat,
            sent_cards,
            received_cards,
        } => private_effect(
            seat,
            BriarCircuitEffect::PassExchangePrivate {
                seat,
                sent_cards,
                received_cards,
            },
        ),
        public => vec![EffectEnvelope::public(public)],
    }
}

pub fn filter_effects_for_viewer(
    effects: &[EffectEnvelope<BriarCircuitEffect>],
    viewer: &Viewer,
) -> Vec<BriarCircuitEffect> {
    effects
        .iter()
        .filter_map(|effect| match &effect.visibility {
            VisibilityScope::Public => Some(effect.payload.clone()),
            VisibilityScope::PrivateToSeat(seat_id)
                if viewer
                    .seat_id
                    .as_ref()
                    .is_some_and(|viewer_seat| viewer_seat == seat_id) =>
            {
                Some(effect.payload.clone())
            }
            _ => None,
        })
        .collect()
}

fn next_uncommitted_seat(pass: &PassState) -> Option<BriarCircuitSeat> {
    BriarCircuitSeat::ALL
        .into_iter()
        .find(|seat| !pass.is_committed(*seat))
}

fn viewer_seat(viewer: &Viewer) -> Option<BriarCircuitSeat> {
    viewer
        .seat_id
        .as_ref()
        .and_then(|seat_id| BriarCircuitSeat::parse(&seat_id.0))
}

fn private_effect(
    seat: BriarCircuitSeat,
    payload: BriarCircuitEffect,
) -> Vec<EffectEnvelope<BriarCircuitEffect>> {
    vec![EffectEnvelope::private_to(
        SeatId(seat.as_str().to_owned()),
        payload,
    )]
}
