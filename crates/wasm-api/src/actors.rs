//! Seat → actor / viewer resolution and viewer authorization for every game.
//!
//! Bridges a browser-supplied seat string to the engine `Actor` that may act
//! (`*_actor_for_seat`), to the `Viewer` scope used when projecting views and
//! filtering effects (`*_viewer_for_seat`), and decides whether a given viewer
//! is allowed to see a specific actor's legal-action tree
//! (`*_viewer_authorizes_actor`). Glob-imported at the crate root.

use engine_core::{Actor, SeatId, Viewer};

use crate::json::escape_json;
use crate::seats::*;

use column_four::{ColumnFourSeat, ColumnFourState};
use directional_flip::{DirectionalFlipSeat, DirectionalFlipState};
use draughts_lite::{DraughtsLiteSeat, DraughtsLiteState};
use event_frontier::EventFrontierState;
use flood_watch::FloodWatchState;
use frontier_control::FrontierControlState;
use high_card_duel::{HighCardDuelSeat, HighCardDuelState};
use masked_claims::{MaskedClaimsSeat, MaskedClaimsState};
use plain_tricks::{PlainTricksSeat, PlainTricksState};
use poker_lite::{PokerLiteSeat, PokerLiteState};
use race_to_n::{RaceSeat, RaceState};
use river_ledger::{RiverLedgerSeat, RiverLedgerState};
use secret_draft::{SecretDraftSeat, SecretDraftState};
use three_marks::{ThreeMarksSeat, ThreeMarksState};
use token_bazaar::{TokenBazaarSeat, TokenBazaarState};

pub(crate) fn race_actor_for_seat(state: &RaceState, seat: RaceSeat) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

pub(crate) fn three_actor_for_seat(
    state: &ThreeMarksState,
    seat: ThreeMarksSeat,
) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

pub(crate) fn column_actor_for_seat(
    state: &ColumnFourState,
    seat: ColumnFourSeat,
) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

pub(crate) fn directional_actor_for_seat(
    state: &DirectionalFlipState,
    seat: DirectionalFlipSeat,
) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

pub(crate) fn draughts_actor_for_seat(
    state: &DraughtsLiteState,
    seat: DraughtsLiteSeat,
) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

pub(crate) fn high_card_actor_for_seat(
    state: &HighCardDuelState,
    seat: HighCardDuelSeat,
) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

pub(crate) fn masked_actor_for_seat(
    state: &MaskedClaimsState,
    seat: MaskedClaimsSeat,
) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

pub(crate) fn flood_actor_for_seat(
    state: &FloodWatchState,
    seat: &SeatId,
) -> Result<Actor, String> {
    if state.seat_index(seat).is_some() {
        Ok(Actor {
            seat_id: seat.clone(),
        })
    } else {
        Err(format!(
            "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
            escape_json(&seat.0)
        ))
    }
}

pub(crate) fn frontier_actor_for_seat(
    state: &FrontierControlState,
    seat: &SeatId,
) -> Result<Actor, String> {
    if state.seats.iter().any(|candidate| candidate == seat) {
        Ok(Actor {
            seat_id: seat.clone(),
        })
    } else {
        Err(format!(
            "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
            escape_json(&seat.0)
        ))
    }
}

pub(crate) fn event_frontier_actor_for_seat(
    state: &EventFrontierState,
    seat: &SeatId,
) -> Result<Actor, String> {
    if state.seats.iter().any(|candidate| candidate == seat) {
        Ok(Actor {
            seat_id: seat.clone(),
        })
    } else {
        Err(format!(
            "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
            escape_json(&seat.0)
        ))
    }
}

pub(crate) fn token_actor_for_seat(
    state: &TokenBazaarState,
    seat: TokenBazaarSeat,
) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

pub(crate) fn secret_actor_for_seat(
    state: &SecretDraftState,
    seat: SecretDraftSeat,
) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

pub(crate) fn poker_actor_for_seat(
    state: &PokerLiteState,
    seat: PokerLiteSeat,
) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

pub(crate) fn plain_actor_for_seat(
    state: &PlainTricksState,
    seat: PlainTricksSeat,
) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

pub(crate) fn river_actor_for_seat(
    state: &RiverLedgerState,
    seat: RiverLedgerSeat,
) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

pub(crate) fn race_viewer_for_seat(
    state: &RaceState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_race_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

pub(crate) fn three_viewer_for_seat(
    state: &ThreeMarksState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_three_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

pub(crate) fn column_viewer_for_seat(
    state: &ColumnFourState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_column_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

pub(crate) fn directional_viewer_for_seat(
    state: &DirectionalFlipState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_directional_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

pub(crate) fn draughts_viewer_for_seat(
    state: &DraughtsLiteState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_draughts_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

pub(crate) fn high_card_viewer_for_seat(
    state: &HighCardDuelState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_high_card_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

pub(crate) fn masked_viewer_for_seat(
    state: &MaskedClaimsState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_masked_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

pub(crate) fn flood_viewer_for_seat(
    state: &FloodWatchState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat.map(parse_flood_seat).transpose()?;
    if let Some(seat_id) = &seat_id {
        flood_actor_for_seat(state, seat_id)?;
    }
    Ok(Viewer { seat_id })
}

pub(crate) fn frontier_viewer_for_seat(
    state: &FrontierControlState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat.map(parse_frontier_seat).transpose()?;
    if let Some(seat_id) = &seat_id {
        frontier_actor_for_seat(state, seat_id)?;
    }
    Ok(Viewer { seat_id })
}

pub(crate) fn event_frontier_viewer_for_seat(
    state: &EventFrontierState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat.map(parse_event_frontier_seat).transpose()?;
    if let Some(seat_id) = &seat_id {
        event_frontier_actor_for_seat(state, seat_id)?;
    }
    Ok(Viewer { seat_id })
}

pub(crate) fn token_viewer_for_seat(
    state: &TokenBazaarState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_token_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

pub(crate) fn secret_viewer_for_seat(
    state: &SecretDraftState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_secret_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

pub(crate) fn poker_viewer_for_seat(
    state: &PokerLiteState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_poker_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

pub(crate) fn plain_viewer_for_seat(
    state: &PlainTricksState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_plain_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

pub(crate) fn river_viewer_for_seat(
    state: &RiverLedgerState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_river_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

pub(crate) fn race_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: RaceSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_race_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

pub(crate) fn three_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: ThreeMarksSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_three_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

pub(crate) fn column_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: ColumnFourSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_column_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

pub(crate) fn directional_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: DirectionalFlipSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_directional_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

pub(crate) fn draughts_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: DraughtsLiteSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_draughts_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

pub(crate) fn high_card_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: HighCardDuelSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_high_card_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

pub(crate) fn masked_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: MaskedClaimsSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_masked_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

pub(crate) fn flood_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: &SeatId,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_flood_seat)
        .transpose()
        .map(|viewer| viewer.as_ref() == Some(actor))
}

pub(crate) fn frontier_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: &SeatId,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_frontier_seat)
        .transpose()
        .map(|viewer| viewer.as_ref() == Some(actor))
}

pub(crate) fn event_frontier_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: &SeatId,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_event_frontier_seat)
        .transpose()
        .map(|viewer| viewer.as_ref() == Some(actor))
}

pub(crate) fn token_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: TokenBazaarSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_token_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

pub(crate) fn secret_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: SecretDraftSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_secret_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

pub(crate) fn poker_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: PokerLiteSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_poker_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

pub(crate) fn river_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: RiverLedgerSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_river_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

pub(crate) fn plain_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: PlainTricksSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_plain_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}
